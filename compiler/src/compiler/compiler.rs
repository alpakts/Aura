use std::collections::HashMap;
use crate::compiler::lexer::TokenType;
use crate::compiler::parser::{Expr, Stmt};

#[derive(Clone, PartialEq, Debug)]
pub enum TargetOs {
    Windows,
    Linux,
    MacOS,
}

#[derive(Clone, PartialEq, Debug)]
enum VarType { 
    Int, 
    Str, 
    Array(Box<VarType>, usize),
    Instance(String) 
}

pub struct Compiler {
    output: String,     
    main_body: String,  
    current_output: String, // Buffer for functions
    
    reg_counter: i32,
    label_counter: i32,
    str_counter: i32,
    string_literals: Vec<(i32, String, usize)>,
    
    var_types: HashMap<String, VarType>, 
    is_in_function: bool, 
    
    classes: HashMap<String, Vec<String>>, // ClassName -> [FieldNames]
    class_methods: HashMap<String, Vec<String>>, // ClassName -> [MethodNames]
    current_class: Option<String>,
    pub target_os: TargetOs,
    scope_stack: Vec<Vec<String>>, // Stack of blocks, each containing variable names (Instances) to cleanup
    block_terminated: bool, // Tracking if 'ret' or 'br' was emitted in current block
}

impl Compiler {
    pub fn new() -> Self {
        #[cfg(target_os = "windows")]
        let target = TargetOs::Windows;
        #[cfg(target_os = "linux")]
        let target = TargetOs::Linux;
        #[cfg(target_os = "macos")]
        let target = TargetOs::MacOS;
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        let target = TargetOs::Windows; // Default fallback

        Self { 
            output: String::new(),
            main_body: String::new(),
            current_output: String::new(),
            reg_counter: 1, 
            label_counter: 0,
            str_counter: 0,
            string_literals: Vec::new(),
            var_types: HashMap::new(),
            is_in_function: false,
            classes: HashMap::new(),
            class_methods: HashMap::new(),
            current_class: None,
            target_os: target,
            scope_stack: Vec::new(),
            block_terminated: false,
        }
    }

    fn get_reg(&mut self) -> String {
        let r = format!("%tmp{}", self.reg_counter);
        self.reg_counter += 1;
        r
    }
    
    fn get_label(&mut self) -> String {
        let l = format!("L{}", self.label_counter);
        self.label_counter += 1;
        l
    }

    fn add_string(&mut self, s: String) -> String {
        // Check if string already exists
        if let Some((id, _, _)) = self.string_literals.iter().find(|(_, content, _)| content == &s) {
            return format!("@str.{}", id);
        }
        
        let id = self.str_counter;
        let len = s.len() + 1; 
        self.string_literals.push((id, s, len));
        self.str_counter += 1;
        format!("@str.{}", id)
    }

    fn emit(&mut self, s: &str) {
        let trimmed = s.trim_start();
        
        // Labels reset termination (new basic block)
        if trimmed.contains(':') && !trimmed.contains('"') && !trimmed.contains('@') {
            self.block_terminated = false;
        }

        if self.block_terminated { return; }

        if trimmed.starts_with("ret ") || trimmed.starts_with("br ") {
            self.block_terminated = true;
        }

        if self.is_in_function {
            self.current_output.push_str(s);
        } else {
            self.main_body.push_str(s);
        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> (String, VarType) {
        match expr {
            Expr::Number(n) => (format!("{}", n), VarType::Int),
            Expr::String(s) => {
                let str_id = self.add_string(s.clone());
                (str_id, VarType::Str)
            }
            Expr::ArrayLiteral(_) => {
                panic!("Array Literal can only be used in variable declaration!");
            }
            Expr::Variable(name) => {
                let vtype_opt = self.var_types.get(name).cloned();
                if let Some(vtype) = vtype_opt {
                    let reg = self.get_reg();
                    match &vtype { 
                        VarType::Int => { self.emit(&format!("  {} = load i32, i32* %{}_ptr\n", reg, name)); }, 
                        VarType::Str => { self.emit(&format!("  {} = load i8*, i8** %{}_ptr\n", reg, name)); },
                        VarType::Instance(cls) => { self.emit(&format!("  {} = load %struct.{}*, %struct.{}** %{}_ptr\n", reg, cls, cls, name)); },
                        VarType::Array(_, _) => panic!("Arrays can only be accessed via index: {}[0]", name),
                    }
                    (reg, vtype)
                } else {
                    panic!("Undefined variable: {}", name);
                }
            }
            Expr::New(class_name) => {
                if let Some(fields) = self.classes.get(class_name) {
                    let field_count = fields.len();
                    // Assuming all fields are i32 (4 bytes). Struct size = 4 * count.
                    // Note: This is simplified. Real structs need alignment etc.
                    let size = field_count * 4; 
                    
                    let malloc_reg = self.get_reg();
                    self.emit(&format!("  {} = call i8* @malloc(i32 {})\n", malloc_reg, size));
                    
                    let cast_reg = self.get_reg();
                    self.emit(&format!("  {} = bitcast i8* {} to %struct.{}*\n", cast_reg, malloc_reg, class_name));
                    
                    (cast_reg, VarType::Instance(class_name.clone()))
                } else {
                    panic!("Unknown class: {}", class_name);
                }
            }
            Expr::Get(obj_expr, field_name) => {
                let (obj_reg, vtype) = self.compile_expr(obj_expr);
                if let VarType::Instance(class_name) = vtype {
                     let fields = self.classes.get(&class_name).unwrap();
                     let index = fields.iter().position(|r| r == field_name)
                        .expect(&format!("Field '{}' not found in class '{}'", field_name, class_name));
                     
                     let gep_reg = self.get_reg();
                     // Access field at index
                     self.emit(&format!("  {} = getelementptr inbounds %struct.{}, %struct.{}* {}, i32 0, i32 {}\n", 
                         gep_reg, class_name, class_name, obj_reg, index));
                     
                     let val_reg = self.get_reg();
                     self.emit(&format!("  {} = load i32, i32* {}\n", val_reg, gep_reg));
                     (val_reg, VarType::Int) // Assuming fields are all Int
                } else { panic!("Property access on non-object"); }
            }
            Expr::Set(obj_expr, field_name, val_expr) => {
                let (obj_reg, vtype) = self.compile_expr(obj_expr);
                if let VarType::Instance(class_name) = vtype {
                     let fields = self.classes.get(&class_name).unwrap();
                     let index = fields.iter().position(|r| r == field_name)
                        .expect(&format!("Field '{}' not found in class '{}'", field_name, class_name));
                     
                     let (val_reg, _) = self.compile_expr(val_expr);
                     
                     let gep_reg = self.get_reg();
                     self.emit(&format!("  {} = getelementptr inbounds %struct.{}, %struct.{}* {}, i32 0, i32 {}\n", 
                         gep_reg, class_name, class_name, obj_reg, index));
                     
                     self.emit(&format!("  store i32 {}, i32* {}\n", val_reg, gep_reg));
                     (val_reg, VarType::Int)
                } else { panic!("Property set on non-object"); }
            }
            Expr::IndexAccess(name, index_expr) => {
                 let vtype = self.var_types.get(name).expect(&format!("Undefined variable: {}", name)).clone();
                 if let VarType::Array(elem_type, len) = vtype {
                     let (idx_val, _) = self.compile_expr(index_expr);
                     let ptr_reg = self.get_reg();
                     let llvm_type = match *elem_type {
                         VarType::Int => "i32",
                         VarType::Str => "i8*",
                         _ => "i32"
                     };
                     self.emit(&format!("  {} = getelementptr inbounds [{} x {}], [{} x {}]* %{}_ptr, i32 0, i32 {}\n", 
                         ptr_reg, len, llvm_type, len, llvm_type, name, idx_val));
                     
                     let val_reg = self.get_reg();
                     self.emit(&format!("  {} = load {}, {}* {}\n", val_reg, llvm_type, llvm_type, ptr_reg));
                     (val_reg, *elem_type)
                 } else { panic!("'{}' is not an array!", name); }
            }
            Expr::MethodCall(obj_expr, method_name, args) => {
                let (obj_val, obj_type) = self.compile_expr(obj_expr);
                if let VarType::Instance(class_name) = obj_type {
                    // Mangled name: Class_Method
                    let func_name = format!("{}_{}", class_name, method_name);
                    
                    let mut arg_vals = Vec::new();
                    // Pass 'this' as first argument
                    arg_vals.push(format!("%struct.{}* {}", class_name, obj_val));

                    for arg in args {
                        let (val, _) = self.compile_expr(arg);
                        arg_vals.push(format!("i32 {}", val)); // Simplify: assume i32 args
                    }
                    
                    let args_str = arg_vals.join(", ");
                    let reg = self.get_reg();
                    // Assume methods return i32 for now
                    self.emit(&format!("  {} = call i32 @{}({})\n", reg, func_name, args_str));
                    (reg, VarType::Int)
                } else {
                    panic!("Method calls only supported on class instances.");
                }
            },
            Expr::Call(name, args) => {
                if name == "print_str" {
                     let (val, vtype) = self.compile_expr(&args[0]);
                     if val.starts_with("@str.") {
                         let str_len = self.string_literals.iter().find(|(id, _, _)| format!("@str.{}", id) == val).unwrap().2;
                         let ptr_reg = self.get_reg();
                         self.emit(&format!("  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i32 0, i32 0\n", ptr_reg, str_len, str_len, val));
                         self.emit(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* {})\n", ptr_reg));
                     } else {
                         let final_ptr = if vtype == VarType::Str {
                             val // Zaten i8* (input_str'den gelmiş olabilir)
                         } else {
                             let ptr_reg = self.get_reg();
                             self.emit(&format!("  {} = inttoptr i32 {} to i8*\n", ptr_reg, val));
                             ptr_reg
                         };
                         self.emit(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* {})\n", final_ptr));
                     }
                     ("0".to_string(), VarType::Int) 
                } else if name == "free" {
                    let (obj_reg, obj_type) = self.compile_expr(&args[0]);
                    if let VarType::Instance(class_name) = obj_type {
                        let cast_reg = self.get_reg();
                        self.emit(&format!("  {} = bitcast %struct.{}* {} to i8*\n", cast_reg, class_name, obj_reg));
                        self.emit(&format!("  call void @free(i8* {})\n", cast_reg));
                        ("0".to_string(), VarType::Int)
                    } else {
                        panic!("free() only supports class instances, got {:?}", obj_type);
                    }
                } else if name == "input" {
                    let ptr_reg = self.get_reg();
                    self.emit(&format!("  {} = alloca i32\n", ptr_reg));
                    self.emit(&format!("  call i32 (i8*, ...) @scanf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @fmt_input_num, i32 0, i32 0), i32* {})\n", ptr_reg));
                    let val_reg = self.get_reg();
                    self.emit(&format!("  {} = load i32, i32* {}\n", val_reg, ptr_reg));
                    (val_reg, VarType::Int)
                } else if name == "input_str" {
                    let malloc_reg = self.get_reg();
                    self.emit(&format!("  {} = call i8* @malloc(i32 256)\n", malloc_reg));
                    self.emit(&format!("  call i32 (i8*, ...) @scanf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @fmt_input_str, i32 0, i32 0), i8* {})\n", malloc_reg));
                    (malloc_reg, VarType::Str)
                } else if name == "api_listen" {
                    // Parametreleri al: api_listen(port, instance)
                    let _ = self.compile_expr(&args[0]); // Port
                    let (obj_val, obj_type) = self.compile_expr(&args[1]);
                    
                    if let VarType::Instance(class_name) = obj_type {
                        // --- Socket Baslat ---
                        if self.target_os == TargetOs::Windows {
                            // WinSock Baslat
                            let wsa_data = self.get_reg();
                            self.emit(&format!("  {} = alloca [512 x i8]\n", wsa_data)); 
                            let wsa_ptr = self.get_reg();
                            self.emit(&format!("  {} = getelementptr inbounds [512 x i8], [512 x i8]* {}, i32 0, i32 0\n", wsa_ptr, wsa_data));
                            self.emit(&format!("  call i32 @WSAStartup(i32 514, i8* {})\n", wsa_ptr));
                        }

                        // Socket & Listen Setup
                        let sock = self.get_reg();
                        self.emit(&format!("  {} = call i32 @socket(i32 2, i32 1, i32 6)\n", sock));
                        let addr = self.get_reg();
                        self.emit(&format!("  {} = alloca [16 x i8]\n", addr)); 
                        let addr_ptr = self.get_reg();
                        self.emit(&format!("  {} = getelementptr inbounds [16 x i8], [16 x i8]* {}, i32 0, i32 0\n", addr_ptr, addr));
                        self.emit(&format!("  call void @llvm.memset.p0i8.i32(i8* {}, i8 0, i32 16, i1 false)\n", addr_ptr));
                        let fam_i16 = self.get_reg();
                        self.emit(&format!("  {} = bitcast i8* {} to i16*\n", fam_i16, addr_ptr));
                        self.emit(&format!("  store i16 2, i16* {}\n", fam_i16));
                        let port_ptr = self.get_reg();
                        self.emit(&format!("  {} = getelementptr inbounds i8, i8* {}, i32 2\n", port_ptr, addr_ptr));
                        let port_ptr_16 = self.get_reg();
                        self.emit(&format!("  {} = bitcast i8* {} to i16*\n", port_ptr_16, port_ptr));
                        self.emit(&format!("  store i16 36895, i16* {}\n", port_ptr_16)); // Fixed 8080 for API

                        self.emit(&format!("  call i32 @bind(i32 {}, i8* {}, i32 16)\n", sock, addr_ptr));
                        self.emit(&format!("  call i32 @listen(i32 {}, i32 5)\n", sock));
                        self.emit(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @fmt_api_start, i32 0, i32 0), i32 8080)\n"));

                        // --- Router Loop ---
                        let start_label = self.get_label();
                        self.emit(&format!("  br label %{}\n", start_label));
                        self.emit(&format!("{}:\n", start_label));

                        // Accept
                        let client_sock = self.get_reg();
                        self.emit(&format!("  {} = call i32 @accept(i32 {}, i8* null, i32* null)\n", client_sock, sock));
                        
                        // Request Oku
                        let buf = self.get_reg();
                        self.emit(&format!("  {} = alloca [1024 x i8]\n", buf));
                        let buf_ptr = self.get_reg();
                        self.emit(&format!("  {} = getelementptr inbounds [1024 x i8], [1024 x i8]* {}, i32 0, i32 0\n", buf_ptr, buf));
                        self.emit(&format!("  call i32 @recv(i32 {}, i8* {}, i32 1024, i32 0)\n", client_sock, buf_ptr));

                        // JSON Header Gonder
                        let h1 = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
                        let h1_val = self.add_string(h1.to_string());
                        let h1_ptr = self.get_reg();
                        self.emit(&format!("  {} = getelementptr inbounds [55 x i8], [55 x i8]* {}, i32 0, i32 0\n", h1_ptr, h1_val));
                        self.emit(&format!("  call i32 @send(i32 {}, i8* {}, i32 54, i32 0)\n", client_sock, h1_ptr));

                        // --- Dynamic Method Routing ---
                        if let Some(methods) = self.class_methods.get(&class_name).cloned() {
                            for m in methods {
                                let m_val = self.add_string(m.to_string());
                                let m_ptr = self.get_reg();
                                self.emit(&format!("  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i32 0, i32 0\n", m_ptr, m.len()+1, m.len()+1, m_val));
                                
                                let str_match = self.get_reg();
                                self.emit(&format!("  {} = call i8* @strstr(i8* {}, i8* {})\n", str_match, buf_ptr, m_ptr));
                                
                                let is_match = self.get_reg();
                                self.emit(&format!("  {} = icmp ne i8* {}, null\n", is_match, str_match));
                                
                                let l_then = self.get_label();
                                let l_next = self.get_label();
                                self.emit(&format!("  br i1 {}, label %{}, label %{}\n", is_match, l_then, l_next));
                                
                                self.emit(&format!("{}:\n", l_then));
                                // --- Parametre Ayirma Logic ---
                                // Buffer'da '?' ara
                                let q_mark = self.add_string("?".to_string());
                                let q_ptr = self.get_reg();
                                self.emit(&format!("  {} = call i8* @strstr(i8* {}, i8* getelementptr inbounds ([2 x i8], [2 x i8]* {}, i32 0, i32 0))\n", q_ptr, buf_ptr, q_mark));
                                
                                let param_val_final = self.get_reg();
                                self.emit(&format!("  {} = alloca i32\n", param_val_final));
                                self.emit(&format!("  store i32 0, i32* {}\n", param_val_final));

                                let has_q = self.get_reg();
                                self.emit(&format!("  {} = icmp ne i8* {}, null\n", has_q, q_ptr));
                                let l_q_then = self.get_label();
                                let l_q_next = self.get_label();
                                self.emit(&format!("  br i1 {}, label %{}, label %{}\n", has_q, l_q_then, l_q_next));
                                
                                self.emit(&format!("{}:\n", l_q_then));
                                // '=' ara
                                let eq_mark = self.add_string("=".to_string());
                                let eq_ptr = self.get_reg();
                                self.emit(&format!("  {} = call i8* @strstr(i8* {}, i8* getelementptr inbounds ([2 x i8], [2 x i8]* {}, i32 0, i32 0))\n", eq_ptr, q_ptr, eq_mark));
                                
                                let has_eq = self.get_reg();
                                self.emit(&format!("  {} = icmp ne i8* {}, null\n", has_eq, eq_ptr));
                                let l_eq_then = self.get_label();
                                self.emit(&format!("  br i1 {}, label %{}, label %{}\n", has_eq, l_eq_then, l_q_next));
                                
                                self.emit(&format!("{}:\n", l_eq_then));
                                let val_start = self.get_reg();
                                self.emit(&format!("  {} = getelementptr inbounds i8, i8* {}, i32 1\n", val_start, eq_ptr));
                                let parsed_int = self.get_reg();
                                self.emit(&format!("  {} = call i32 @atoi(i8* {})\n", parsed_int, val_start));
                                self.emit(&format!("  store i32 {}, i32* {}\n", parsed_int, param_val_final));
                                self.emit(&format!("  br label %{}\n", l_q_next));

                                self.emit(&format!("{}:\n", l_q_next));
                                let final_arg = self.get_reg();
                                self.emit(&format!("  {} = load i32, i32* {}\n", final_arg, param_val_final));

                                // Metodu cagir: Class_Method(this, param)
                                let res_ptr = self.get_reg();
                                self.emit(&format!("  {} = call i8* @{}_{}(%struct.{}* {}, i32 {})\n", res_ptr, class_name, m, class_name, obj_val, final_arg));
                                
                                // Uzunlugu olc (strlen)
                                let res_len = self.get_reg();
                                self.emit(&format!("  {} = call i32 @strlen(i8* {})\n", res_len, res_ptr));
                                
                                self.emit(&format!("  call i32 @send(i32 {}, i8* {}, i32 {}, i32 0)\n", client_sock, res_ptr, res_len));
                                self.emit(&format!("  br label %{}\n", l_next));
                                
                                self.emit(&format!("{}:\n", l_next));
                            }
                        }

                        if self.target_os == TargetOs::Windows {
                            self.emit(&format!("  call i32 @closesocket(i32 {})\n", client_sock));
                        } else {
                            self.emit(&format!("  call i32 @close(i32 {})\n", client_sock));
                        }
                        self.emit(&format!("  br label %{}\n", start_label)); // Infinite Loop!

                        ("0".to_string(), VarType::Int)
                    } else { panic!("api_listen requires a class instance."); }
                } else {
                    let mut arg_vals = Vec::new();
                    for arg in args {
                        let (val, vtype) = self.compile_expr(arg);
                        if let VarType::Str = vtype {
                            // Simplify string passing (assume int for now or ptrtoint)
                            if val.starts_with("@str.") {
                                let str_len = self.string_literals.iter().find(|(id, _, _)| format!("@str.{}", id) == val).unwrap().2;
                                let ptr_reg = self.get_reg();
                                self.emit(&format!("  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i32 0, i32 0\n", ptr_reg, str_len, str_len, val));
                                let int_reg = self.get_reg();
                                self.emit(&format!("  {} = ptrtoint i8* {} to i32\n", int_reg, ptr_reg));
                                arg_vals.push(format!("i32 {}", int_reg));
                            } else {
                                let int_reg = self.get_reg();
                                self.emit(&format!("  {} = ptrtoint i8* {} to i32\n", int_reg, val));
                                arg_vals.push(format!("i32 {}", int_reg));
                            }
                        } else {
                            arg_vals.push(format!("i32 {}", val)); 
                        }
                    }
                    let args_str = arg_vals.join(", ");
                    let reg = self.get_reg();
                    self.emit(&format!("  {} = call i32 @{}({})\n", reg, name, args_str));
                    (reg, VarType::Int)
                }
            }
            Expr::Binary(left, op, right) => {
                let (l_val, _) = self.compile_expr(left);
                let (r_val, _) = self.compile_expr(right);

                if matches!(op, TokenType::Plus|TokenType::Minus|TokenType::Mul|TokenType::Div) {
                    let reg = self.get_reg();
                    let op_str = match op {
                        TokenType::Plus => "add", TokenType::Minus => "sub",
                        TokenType::Mul => "mul", TokenType::Div => "sdiv",
                        _ => unreachable!()
                    };
                    self.emit(&format!("  {} = {} i32 {}, {}\n", reg, op_str, l_val, r_val));
                    (reg, VarType::Int)
                } else {
                    let reg = self.get_reg();
                    let op_str = match op {
                        TokenType::Eq => "eq", TokenType::Neq => "ne",
                        TokenType::Lt => "slt", TokenType::Gt => "sgt",
                        TokenType::Lte => "sle", TokenType::Gte => "sge",
                        _ => unreachable!()
                    };
                    self.emit(&format!("  {} = icmp {} i32 {}, {}\n", reg, op_str, l_val, r_val));
                    (reg, VarType::Int) 
                }
            }
        }
    }

    fn emit_block_cleanup(&mut self, skip_var: Option<&str>) {
        if let Some(scope) = self.scope_stack.last().cloned() {
            for var_name in scope.iter().rev() {
                if let Some(skip) = skip_var { if var_name == skip { continue; } }
                
                // 1. Get Instance type
                if let Some(VarType::Instance(cls_name)) = self.var_types.get(var_name).cloned() {
                    // 2. Destructor Call (ClassName_drop)
                    if let Some(methods) = self.class_methods.get(&cls_name) {
                        if methods.contains(&"drop".to_string()) {
                            let ptr_reg = self.get_reg();
                            self.emit(&format!("  {} = load %struct.{}*, %struct.{}** %{}_ptr\n", ptr_reg, cls_name, cls_name, var_name));
                            // drop(this) - currently returns i8* for all aura funcs
                            self.emit(&format!("  call i8* @{}_drop(%struct.{}* {})\n", cls_name, cls_name, ptr_reg));
                        }
                    }

                    // 3. free(i8*)
                    let ptr_reg = self.get_reg();
                    self.emit(&format!("  {} = load %struct.{}*, %struct.{}** %{}_ptr\n", ptr_reg, cls_name, cls_name, var_name));
                    let cast_reg = self.get_reg();
                    self.emit(&format!("  {} = bitcast %struct.{}* {} to i8*\n", cast_reg, cls_name, ptr_reg));
                    self.emit(&format!("  call void @free(i8* {})\n", cast_reg));
                }
            }
        }
    }

    fn compile_block(&mut self, stmts: &[Stmt]) {
        let old_term = self.block_terminated;
        self.block_terminated = false;
        self.scope_stack.push(Vec::new());
        for stmt in stmts { self.compile_stmt(stmt); }
        self.emit_block_cleanup(None);
        self.scope_stack.pop();
        self.block_terminated = old_term; // Restore status (e.g. if the whole block returned)
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::ClassDecl(name, fields, methods) => {
                // Register class properties & methods
                self.classes.insert(name.clone(), fields.clone());
                self.current_class = Some(name.clone());
                
                let mut method_names = Vec::new();

                // Compile methods
                for method in methods {
                    let method_clone = method.clone();
                    if let Stmt::FuncDecl(method_name, mut args, body) = method_clone {
                        method_names.push(method_name.clone());
                        // 1. Mangle Name: Class_Method
                        let mangled_name = format!("{}_{}", name, method_name);
                        
                        // 2. Inject 'this' argument
                        args.insert(0, "this".to_string());
                        
                        // 3. Compile as standard function
                        self.compile_stmt(&Stmt::FuncDecl(mangled_name, args.clone(), body.to_vec()));
                    }
                }
                
                self.class_methods.insert(name.clone(), method_names);

                // Clear context
                self.current_class = None;
            },
            Stmt::FuncDecl(name, args, body) => {
                let old_in_func = self.is_in_function;
                let old_vars = self.var_types.clone(); 
                self.is_in_function = true;
                self.current_output = String::new(); 
                self.var_types.clear(); 
                self.block_terminated = false;

                let mut arg_defs = Vec::new();
                for (i, arg_name) in args.iter().enumerate() {
                    if arg_name == "this" {
                        if let Some(cls_name) = self.current_class.clone() {
                             arg_defs.push(format!("%struct.{}* %arg{}", cls_name, i));
                        } else {
                             // Should not happen if 'this' is only injected by us
                             panic!("'this' argument found outside of class context");
                        }
                    } else {
                        arg_defs.push(format!("i32 %arg{}", i)); 
                    }
                }
                self.output.push_str(&format!("\ndefine i8* @{}({}) {{\nentry:\n", name, arg_defs.join(", ")));
                
                self.scope_stack.push(Vec::new()); // Function Top-Level Scope
                self.block_terminated = false;
                for (i, arg_name) in args.iter().enumerate() {
                    if arg_name == "this" {
                         if let Some(cls_name) = self.current_class.clone() {
                             self.current_output.push_str(&format!("  %{}_ptr = alloca %struct.{}*\n", arg_name, cls_name));
                             self.current_output.push_str(&format!("  store %struct.{}* %arg{}, %struct.{}** %{}_ptr\n", cls_name, i, cls_name, arg_name));
                             self.var_types.insert(arg_name.clone(), VarType::Instance(cls_name));
                         }
                    } else {
                        self.current_output.push_str(&format!("  %{}_ptr = alloca i32\n", arg_name));
                        self.current_output.push_str(&format!("  store i32 %arg{}, i32* %{}_ptr\n", i, arg_name));
                        self.var_types.insert(arg_name.clone(), VarType::Int);
                    }
                }
                
                self.compile_block(body);
                
                if !self.current_output.contains("ret i8*") {
                    self.current_output.push_str("  ret i8* null\n");
                }
                self.output.push_str(&self.current_output);
                self.output.push_str("}\n");
                self.scope_stack.pop(); // Pop Function Scope
                self.is_in_function = old_in_func;
                self.var_types = old_vars;
            }
            Stmt::ReturnStmt(expr_opt) => {
                if let Some(expr) = expr_opt {
                    let (val, vtype) = self.compile_expr(expr);
                    
                    // Cleanup current block (skipping return value if from variable)
                    let skip_v = if let Expr::Variable(n) = expr { Some(n.as_str()) } else { None };
                    
                    // We need to cleanup ALL parent scopes up to the function entry? 
                    // Aura currently doesn't have deep nested blocks that survive return.
                    // For now, cleanup current scope only.
                    self.emit_block_cleanup(skip_v);

                    if self.is_in_function {
                        if vtype == VarType::Int {
                            let ptr = self.get_reg();
                            self.emit(&format!("  {} = inttoptr i32 {} to i8*\n", ptr, val));
                            self.emit(&format!("  ret i8* {}\n", ptr));
                        } else if val.starts_with("@str.") {
                             let str_len = self.string_literals.iter().find(|(id, _, _)| format!("@str.{}", id) == val).unwrap().2;
                             let ptr_reg = self.get_reg();
                             self.emit(&format!("  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i32 0, i32 0\n", ptr_reg, str_len, str_len, val));
                             self.emit(&format!("  ret i8* {}\n", ptr_reg));
                        } else {
                            self.emit(&format!("  ret i8* {}\n", val));
                        }
                    } else {
                        // In main: use i32
                        self.emit(&format!("  ret i32 {}\n", val));
                    }
                } else {
                    if self.is_in_function { self.emit("  ret i8* null\n"); }
                    else { self.emit("  ret i32 0\n"); }
                }
            }
            Stmt::VarDecl(name, expr) => {
                if let Expr::ArrayLiteral(elements) = expr {
                    let len = elements.len();
                    // Determine element type from the first element
                    let (first_val, elem_vtype) = if len > 0 { 
                        self.compile_expr(&elements[0]) 
                    } else { ("0".to_string(), VarType::Int) };
                    
                    let llvm_type = match elem_vtype {
                        VarType::Int => "i32",
                        VarType::Str => "i8*",
                        _ => "i32"
                    };

                    self.emit(&format!("  %{}_ptr = alloca [{} x {}]\n", name, len, llvm_type));
                    self.var_types.insert(name.clone(), VarType::Array(Box::new(elem_vtype.clone()), len));
                    
                    for (i, el) in elements.iter().enumerate() {
                        let (val, _) = if i == 0 { (first_val.clone(), elem_vtype.clone()) } else { self.compile_expr(el) };
                        let ptr_reg = self.get_reg();
                        self.emit(&format!("  {} = getelementptr inbounds [{} x {}], [{} x {}]* %{}_ptr, i32 0, i32 {}\n", 
                            ptr_reg, len, llvm_type, len, llvm_type, name, i));
                        
                        // String ise literal PTR'sini almamiz gerekebilir (print_str logic gibi)
                        let store_val = if elem_vtype == VarType::Str && val.starts_with("@str.") {
                            let str_len = self.string_literals.iter().find(|(id, _, _)| format!("@str.{}", id) == val).unwrap().2;
                            let reg = self.get_reg();
                            self.emit(&format!("  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i32 0, i32 0\n", reg, str_len, str_len, val));
                            reg
                        } else { val };

                        self.emit(&format!("  store {} {}, {}* {}\n", llvm_type, store_val, llvm_type, ptr_reg));
                    }
                } else {
                    let (val, vtype) = self.compile_expr(expr);
                    match &vtype {
                        VarType::Instance(cls) => {
                             self.emit(&format!("  %{}_ptr = alloca %struct.{}*\n", name, cls));
                             self.emit(&format!("  store %struct.{}* {}, %struct.{}** %{}_ptr\n", cls, val, cls, name));
                             if let Some(scope) = self.scope_stack.last_mut() {
                                 scope.push(name.clone());
                             }
                        },
                        VarType::Int => {
                             self.emit(&format!("  %{}_ptr = alloca i32\n", name));
                             self.emit(&format!("  store i32 {}, i32* %{}_ptr\n", val, name));
                        },
                        VarType::Str => {
                             self.emit(&format!("  %{}_ptr = alloca i8*\n", name));
                             self.emit(&format!("  store i8* {}, i8** %{}_ptr\n", val, name));
                        },
                        _ => panic!("Unsupported var type decl")
                    }
                    self.var_types.insert(name.clone(), vtype);
                }
            }
            Stmt::Assignment(name, expr) => {
                 let (val, vtype) = self.compile_expr(expr);
                 // Assuming var already exists and type matches
                 match vtype {
                     VarType::Int => { self.emit(&format!("  store i32 {}, i32* %{}_ptr\n", val, name)); }
                     VarType::Str => { self.emit(&format!("  store i8* {}, i8** %{}_ptr\n", val, name)); }
                     VarType::Instance(cls) => {
                          self.emit(&format!("  store %struct.{}* {}, %struct.{}** %{}_ptr\n", cls, val, cls, name));
                     }
                     _ => panic!("Assign error")
                 }
            }
            Stmt::ExprStmt(expr) => {
                self.compile_expr(expr);
            }
            Stmt::Print(expr) => {
                let (val, vtype) = self.compile_expr(expr);
                match vtype {
                    VarType::Int => { self.emit(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 {})\n", val)); }
                    VarType::Str => {
                         if val.starts_with("@") {
                             let str_len = self.string_literals.iter().find(|(id, _, _)| format!("@str.{}", id) == val).unwrap().2;
                             self.emit(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i32 0, i32 0))\n", str_len, str_len, val));
                         } else {
                             self.emit(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* {})\n", val));
                         }
                    }
                    _ => panic!("This type cannot be printed directly (try printing a field)"),
                }
            }
            Stmt::IfStmt(cond, then_block, else_block_opt) => {
                let (cond_reg, _) = self.compile_expr(cond);
                let label_then = self.get_label();
                let label_else = self.get_label();
                let label_merge = self.get_label(); 
                let jump_false = if else_block_opt.is_some() { &label_else } else { &label_merge };

                self.emit(&format!("  br i1 {}, label %{}, label %{}\n", cond_reg, label_then, jump_false));
                self.emit(&format!("{}:\n", label_then));
                self.compile_block(then_block);
                self.emit(&format!("  br label %{}\n", label_merge)); 
                if let Some(else_block) = else_block_opt {
                    self.emit(&format!("{}:\n", label_else));
                    self.compile_block(else_block);
                    self.emit(&format!("  br label %{}\n", label_merge));
                }
                self.emit(&format!("{}:\n", label_merge));
            }
            Stmt::WhileStmt(cond, block) => {
                let label_cond = self.get_label();
                let label_body = self.get_label();
                let label_end = self.get_label();
                self.emit(&format!("  br label %{}\n", label_cond));
                self.emit(&format!("{}:\n", label_cond));
                let (cond_reg, _) = self.compile_expr(cond);
                self.emit(&format!("  br i1 {}, label %{}, label %{}\n", cond_reg, label_body, label_end));
                self.emit(&format!("{}:\n", label_body));
                self.compile_block(block);
                self.emit(&format!("  br label %{}\n", label_cond));
                self.emit(&format!("{}:\n", label_end));
            }
            Stmt::BlockStmt(stmts) => { self.compile_block(stmts); }
        }
    }

    pub fn compile(&mut self, stmts: &[Stmt]) -> String {
        self.output = String::new();
        self.main_body = String::new();
        
        // 1. Scan for Class Declarations first to register them (and generate struct defs later)
        // Note: recursion/nested blocks might hide class decls if not top level.
        // For now, only Top Level classes supported.
        for stmt in stmts {
            if let Stmt::ClassDecl(name, fields, _) = stmt {
                self.classes.insert(name.clone(), fields.clone());
            }
        }
        
        // 2. Compile Statements
        for stmt in stmts { self.compile_stmt(stmt); }
        
        let mut header = String::from("; Module: aura_lang\n");
        // Generate Struct Definitions
        for (name, fields) in &self.classes {
             // Generate { i32, i32, ... }
             let types_str = fields.iter().map(|_| "i32").collect::<Vec<_>>().join(", ");
             header.push_str(&format!("%struct.{} = type {{ {} }}\n", name, types_str));
        }

        header.push_str("declare i16 @htons(i16)\n");
        header.push_str("declare i8* @strstr(i8*, i8*)\n");
        header.push_str("declare i32 @strlen(i8*)\n");
        header.push_str("declare i32 @atoi(i8*)\n");
        
        if self.target_os == TargetOs::Windows {
            header.push_str("declare i32 @WSAStartup(i32, i8*)\n");
            header.push_str("declare i32 @closesocket(i32)\n");
        } else {
            header.push_str("declare i32 @close(i32)\n");
        }

        header.push_str("declare i32 @socket(i32, i32, i32)\n");
        header.push_str("declare i32 @bind(i32, i8*, i32)\n");
        header.push_str("declare i32 @listen(i32, i32)\n");
        header.push_str("declare i32 @accept(i32, i8*, i32*)\n");
        header.push_str("declare i32 @recv(i32, i8*, i32, i32)\n");
        header.push_str("declare i32 @send(i32, i8*, i32, i32)\n");
        header.push_str("declare void @llvm.memset.p0i8.i32(i8*, i8, i32, i1)\n");

        header.push_str("declare i32 @printf(i8*, ...)\n");
        header.push_str("declare i32 @scanf(i8*, ...)\n"); // Added scanf
        header.push_str("declare i32 @system(i8*)\n");
        header.push_str("declare i8* @malloc(i32)\n"); // Added malloc
        header.push_str("declare void @free(i8*)\n");
        
        header.push_str("@fmt_num = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\"\n");
        header.push_str("@fmt_str = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\"\n");
        header.push_str("@fmt_input_num = private unnamed_addr constant [3 x i8] c\"%d\\00\"\n");
        header.push_str("@fmt_input_str = private unnamed_addr constant [3 x i8] c\"%s\\00\"\n");
        header.push_str("@fmt_api_start = private unnamed_addr constant [9 x i8] c\"API: %d\\0A\\00\"\n");
        header.push_str("@cmd_chcp = private unnamed_addr constant [17 x i8] c\"chcp 65001 > nul\\00\"\n");
        
        for (id, content, len) in &self.string_literals {
             let mut llvm_str = String::new();
             for byte in content.bytes() {
                 if byte >= 32 && byte <= 126 && byte != 34 && byte != 92 {
                     llvm_str.push(byte as char);
                 } else {
                     llvm_str.push_str(&format!("\\{:02X}", byte));
                 }
             }
             header.push_str(&format!("@str.{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", id, len, llvm_str));
        }
        
        header.push_str("\n");
        header.push_str(&self.output); // Functions
        
        header.push_str("\ndefine i32 @main() {\nentry:\n");
        if self.target_os == TargetOs::Windows {
            header.push_str("  call i32 @system(i8* getelementptr inbounds ([17 x i8], [17 x i8]* @cmd_chcp, i32 0, i32 0))\n");
        }
        
        header.push_str(&self.main_body);
        header.push_str("  ret i32 0\n}\n");
        
        header
    }
}
