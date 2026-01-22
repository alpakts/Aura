use std::fs;
use std::path::Path;
use std::collections::HashMap;

// --- AŞAMA 2: LEXER (SÖZCÜKSEL ANALİZ) ---
#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Var, Print, If, Else,
    Id(String), Number(i32), String(String), 
    Assign, Plus, Minus, Mul, Div, 
    LParen, RParen, LBrace, RBrace, // { }
    Eq, Neq, Lt, Gt, Lte, Gte,      // == != < > <= >=
    EOF,
}

#[derive(Debug, Clone)]
struct Token { kind: TokenType, line: usize }

struct Lexer { input: Vec<char>, pos: usize, line: usize }

impl Lexer {
    fn new(input: String) -> Self { Self { input: input.chars().collect(), pos: 0, line: 1 } }
    fn peek(&self) -> Option<char> { self.input.get(self.pos).copied() }
    fn create_token(&self, kind: TokenType) -> Token { Token { kind, line: self.line } }
    
    fn advance(&mut self) -> Option<char> {
        let c = self.peek(); self.pos += 1;
        if let Some('\n') = c { self.line += 1; } c
    }
    
    fn skip_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() { self.advance(); }
            else if c == '/' {
                if self.input.get(self.pos+1) == Some(&'/') {
                    while let Some(x) = self.peek() { if x == '\n' { break; } self.advance(); }
                } else { break; }
            } else { break; }
        }
    }
    
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = { self.skip_whitespace_and_comments(); self.peek() } {
            let kind = match c {
                '=' => { 
                    self.advance(); 
                    if self.peek() == Some('=') { self.advance(); TokenType::Eq } else { TokenType::Assign }
                },
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); TokenType::Neq } else { panic!("Satır {}: ! yanına = bekleniyor", self.line) }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); TokenType::Lte } else { TokenType::Lt }
                },
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); TokenType::Gte } else { TokenType::Gt }
                },
                '+' => { self.advance(); TokenType::Plus },
                '-' => { self.advance(); TokenType::Minus },
                '*' => { self.advance(); TokenType::Mul },
                '/' => { self.advance(); TokenType::Div },
                '(' => { self.advance(); TokenType::LParen },
                ')' => { self.advance(); TokenType::RParen },
                '{' => { self.advance(); TokenType::LBrace },
                '}' => { self.advance(); TokenType::RBrace },
                '"' => {
                    self.advance(); let mut s = String::new();
                    while let Some(ch) = self.peek() { if ch == '"' { break; } s.push(self.advance().unwrap()); }
                    if self.peek() == Some('"') { self.advance(); } else { panic!("String kapanmadı!"); }
                    TokenType::String(s)
                }
                '0'..='9' => {
                    let mut s = String::new();
                    while let Some(ch) = self.peek() { if ch.is_digit(10) { s.push(self.advance().unwrap()); } else { break; } }
                    TokenType::Number(s.parse().unwrap())
                }
                'a'..='z'|'A'..='Z'|'_' => {
                    let mut s = String::new();
                    while let Some(ch) = self.peek() { if ch.is_alphanumeric()||ch=='_' { s.push(self.advance().unwrap()); } else { break; } }
                    match s.as_str() { 
                        "var"=>TokenType::Var, "print"=>TokenType::Print, 
                        "if"=>TokenType::If, "else"=>TokenType::Else, 
                        _=>TokenType::Id(s) 
                    }
                }
                _ => panic!("Bilinmeyen karakter: {}", c),
            };
            tokens.push(self.create_token(kind));
        }
        tokens.push(self.create_token(TokenType::EOF));
        tokens
    }
}

// --- AŞAMA 3: PARSER ---
#[derive(Debug)]
enum Expr {
    Number(i32), String(String), Variable(String),
    Binary(Box<Expr>, TokenType, Box<Expr>),
}
#[derive(Debug)]
enum Stmt {
    VarDecl(String, Expr), Assignment(String, Expr), Print(Expr),
    IfStmt(Expr, Vec<Stmt>, Option<Vec<Stmt>>), // condition, then_block, else_block
}

struct Parser { tokens: Vec<Token>, pos: usize }
impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }
    fn peek(&self) -> &Token { &self.tokens[self.pos] }
    fn advance(&mut self) -> Token { let t = self.tokens[self.pos].clone(); if t.kind!=TokenType::EOF { self.pos+=1; } t }
    fn consume(&mut self, expected: TokenType, msg: &str) {
        let t = self.advance(); if std::mem::discriminant(&t.kind)!=std::mem::discriminant(&expected) { panic!("{}", msg); }
    }
    
    fn parse_primary(&mut self) -> Expr {
        let t = self.advance();
        match t.kind {
            TokenType::Number(n) => Expr::Number(n),
            TokenType::String(s) => Expr::String(s),
            TokenType::Id(n) => Expr::Variable(n),
            TokenType::LParen => { let e=self.parse_expr(); self.consume(TokenType::RParen, "')' eksik"); e }
            _ => panic!("Beklenmeyen token: {:?}", t),
        }
    }
    
    // İşlem önceliği hiyerarşisi: 
    // 1. Primary (Sayılar, Parantezler)
    // 2. Mul/Div/Binary
    // 3. Plus/Minus
    // 4. Comparison (==, <, > vb.) -> En düşük öncelik, en dışta olur
    
    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_primary();
        while matches!(self.peek().kind, TokenType::Mul|TokenType::Div) {
            let op = self.advance().kind;
            node = Expr::Binary(Box::new(node), op, Box::new(self.parse_primary()));
        }
        node
    }
    
    fn parse_arithmetic(&mut self) -> Expr {
        let mut node = self.parse_term();
        while matches!(self.peek().kind, TokenType::Plus|TokenType::Minus) {
            let op = self.advance().kind;
            node = Expr::Binary(Box::new(node), op, Box::new(self.parse_term()));
        }
        node
    }

    fn parse_expr(&mut self) -> Expr {
        let mut node = self.parse_arithmetic();
        // Karşılaştırma operatörleri (==, !=, <, >, <=, >=)
        while matches!(self.peek().kind, TokenType::Eq|TokenType::Neq|TokenType::Lt|TokenType::Gt|TokenType::Lte|TokenType::Gte) {
             let op = self.advance().kind;
             node = Expr::Binary(Box::new(node), op, Box::new(self.parse_arithmetic()));
        }
        node
    }
    
    fn parse_block(&mut self) -> Vec<Stmt> {
        self.consume(TokenType::LBrace, "'{' bekleniyor");
        let mut stmts = Vec::new();
        while self.peek().kind != TokenType::RBrace && self.peek().kind != TokenType::EOF {
            stmts.push(self.parse_stmt());
        }
        self.consume(TokenType::RBrace, "'}' bekleniyor");
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        let t = self.peek().clone();
        match t.kind {
            TokenType::Var => {
                self.advance();
                if let TokenType::Id(name) = self.advance().kind {
                    self.consume(TokenType::Assign, "'=' bekleniyor");
                    Stmt::VarDecl(name, self.parse_expr())
                } else { panic!("Değişken adı bekleniyor"); }
            }
            TokenType::Print => {
                self.advance(); self.consume(TokenType::LParen, "'(' bekleniyor");
                let e = self.parse_expr();
                self.consume(TokenType::RParen, "')' bekleniyor");
                Stmt::Print(e)
            }
            TokenType::If => {
                self.advance();
                self.consume(TokenType::LParen, "'(' bekleniyor");
                let condition = self.parse_expr();
                self.consume(TokenType::RParen, "')' bekleniyor");
                
                let then_block = self.parse_block();
                let mut else_block = None;
                
                if matches!(self.peek().kind, TokenType::Else) {
                    self.advance();
                    // Else if desteği (else bloğu içinde yeni if) veya direkt else bloğu
                    if matches!(self.peek().kind, TokenType::If) {
                        // 'else if' -> Bunu 'else { if ... }' olarak parse et
                        let nested_stmt = self.parse_stmt();
                        else_block = Some(vec![nested_stmt]);
                    } else {
                        else_block = Some(self.parse_block());
                    }
                }
                
                Stmt::IfStmt(condition, then_block, else_block)
            }
            TokenType::Id(name) => {
                self.advance(); self.consume(TokenType::Assign, "'=' bekleniyor");
                Stmt::Assignment(name, self.parse_expr())
            }
            _ => panic!("Bilinmeyen ifade: {:?}", t),
        }
    }
    fn parse(&mut self) -> Vec<Stmt> {
        let mut s = Vec::new();
        while self.peek().kind != TokenType::EOF { s.push(self.parse_stmt()); }
        s
    }
}

// --- AŞAMA 4: COMPILER ---

#[derive(Clone, PartialEq, Debug)]
enum VarType { Int, Str } 

struct Compiler {
    output: String,
    reg_counter: i32,
    label_counter: i32, // Label sayacı (if/else için)
    str_counter: i32,
    string_literals: Vec<(i32, String, usize)>,
    var_types: HashMap<String, VarType>,
}

impl Compiler {
    fn new() -> Self {
        Self { 
            output: String::new(), 
            reg_counter: 1, 
            label_counter: 0,
            str_counter: 0,
            string_literals: Vec::new(),
            var_types: HashMap::new(),
        }
    }

    // Registerlara '%tmp' ismi veriyoruz ki LLVM sıra hatası vermesin
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
        let id = self.str_counter;
        let len = s.len() + 1; 
        self.string_literals.push((id, s, len));
        self.str_counter += 1;
        format!("@str.{}", id)
    }

    fn compile_expr(&mut self, expr: &Expr) -> (String, VarType) {
        match expr {
            Expr::Number(n) => (format!("{}", n), VarType::Int),
            Expr::String(s) => {
                let str_id = self.add_string(s.clone());
                (str_id, VarType::Str)
            }
            Expr::Variable(name) => {
                let reg = self.get_reg();
                let vtype = self.var_types.get(name).expect(&format!("Tanımlanmamış değişken: {}", name)).clone();
                let llvm_type = match vtype { VarType::Int => "i32", VarType::Str => "i8*" };
                self.output.push_str(&format!("  {} = load {}, {}* %{}_ptr\n", reg, llvm_type, llvm_type, name));
                (reg, vtype)
            }
            Expr::Binary(left, op, right) => {
                let (l_val, l_type) = self.compile_expr(left);
                let (r_val, r_type) = self.compile_expr(right);

                // Matematiksel işlemler
                if matches!(op, TokenType::Plus|TokenType::Minus|TokenType::Mul|TokenType::Div) {
                    if l_type == VarType::Str || r_type == VarType::Str { panic!("String mat işlem yapılamaz"); }
                    let reg = self.get_reg();
                    let op_str = match op {
                        TokenType::Plus => "add", TokenType::Minus => "sub",
                        TokenType::Mul => "mul", TokenType::Div => "sdiv",
                        _ => unreachable!()
                    };
                    self.output.push_str(&format!("  {} = {} i32 {}, {}\n", reg, op_str, l_val, r_val));
                    (reg, VarType::Int)
                } 
                // Karşılaştırma işlemleri
                else {
                    let reg = self.get_reg();
                    let op_str = match op {
                        TokenType::Eq => "eq", TokenType::Neq => "ne",
                        TokenType::Lt => "slt", TokenType::Gt => "sgt",
                        TokenType::Lte => "sle", TokenType::Gte => "sge",
                        _ => unreachable!()
                    };
                    // icmp sonucu i1 (boolean) döner, ama bizim dilimiz bunu şimdilik desteklemediği için 
                    // i32'ye cast etmemiz gerekebilir IF yapısında direkt i1 kullanacağız.
                    self.output.push_str(&format!("  {} = icmp {} i32 {}, {}\n", reg, op_str, l_val, r_val));
                    // Karşılaştırma sonucu özel bir tiptir (i1), ama biz şimdilik VarType::Int gibi davranalım
                    // IF kontrolünde i1 bekliyor olacağız.
                    (reg, VarType::Int) // Aslında i1 (boolean)
                }
            }
        }
    }

    fn compile_block(&mut self, stmts: &[Stmt]) {
        for stmt in stmts { self.compile_stmt(stmt); }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl(name, expr) => {
                let (val, vtype) = self.compile_expr(expr);
                let llvm_type = match vtype { VarType::Int => "i32", VarType::Str => "i8*" };
                self.output.push_str(&format!("  %{}_ptr = alloca {}\n", name, llvm_type));
                self.var_types.insert(name.clone(), vtype.clone());
                
                if vtype == VarType::Str {
                    let str_len = self.string_literals.iter().find(|s| format!("@str.{}", s.0) == val).unwrap().2;
                    let reg = self.get_reg();
                     self.output.push_str(&format!("  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i32 0, i32 0\n", reg, str_len, str_len, val));
                     self.output.push_str(&format!("  store i8* {}, i8** %{}_ptr\n", reg, name));
                } else {
                    self.output.push_str(&format!("  store {} {}, {}* %{}_ptr\n", llvm_type, val, llvm_type, name));
                }
            }
            Stmt::Assignment(name, expr) => {
                let (val, vtype) = self.compile_expr(expr);
                if !self.var_types.contains_key(name) {
                     let llvm_type = match vtype { VarType::Int => "i32", VarType::Str => "i8*" };
                     self.output.push_str(&format!("  %{}_ptr = alloca {}\n", name, llvm_type));
                     self.var_types.insert(name.clone(), vtype.clone());
                }
                let target_type = self.var_types.get(name).unwrap();
                let llvm_type = match target_type { VarType::Int => "i32", VarType::Str => "i8*" };

                if vtype == VarType::Str {
                    let str_len = self.string_literals.iter().find(|s| format!("@str.{}", s.0) == val).unwrap().2;
                    let reg = self.get_reg();
                    self.output.push_str(&format!("  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i32 0, i32 0\n", reg, str_len, str_len, val));
                    self.output.push_str(&format!("  store i8* {}, i8** %{}_ptr\n", reg, name));
                } else {
                    self.output.push_str(&format!("  store {} {}, {}* %{}_ptr\n", llvm_type, val, llvm_type, name));
                }
            }
            Stmt::Print(expr) => {
                let (val, vtype) = self.compile_expr(expr);
                match vtype {
                    VarType::Int => { self.output.push_str(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 {})\n", val)); }
                    VarType::Str => {
                         if val.starts_with("@") {
                             let str_len = self.string_literals.iter().find(|s| format!("@str.{}", s.0) == val).unwrap().2;
                             self.output.push_str(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* getelementptr inbounds ([{} x i8], [{} x i8]* {}, i32 0, i32 0))\n", str_len, str_len, val));
                         } else {
                             self.output.push_str(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_str, i32 0, i32 0), i8* {})\n", val));
                         }
                    }
                }
            }
            Stmt::IfStmt(cond, then_block, else_block_opt) => {
                let (cond_reg, _) = self.compile_expr(cond);
                // LLVM branch (br) i1 tipi bekler.
                
                // Etiketleri oluştur
                let label_then = self.get_label();
                let label_else = self.get_label();
                let label_merge = self.get_label(); // if/else bittikten sonraki birleşme noktası
                
                // Eğer else bloğu varsa else etiketine git, yoksa merge etiketi else'in yerini tutar
                let jump_false = if else_block_opt.is_some() { &label_else } else { &label_merge };

                self.output.push_str(&format!("  br i1 {}, label %{}, label %{}\n", cond_reg, label_then, jump_false));
                
                // THEN bloğu
                self.output.push_str(&format!("{}:\n", label_then));
                self.compile_block(then_block);
                self.output.push_str(&format!("  br label %{}\n", label_merge)); // İş bitince merge'e atla
                
                // ELSE bloğu (varsa)
                if let Some(else_block) = else_block_opt {
                    self.output.push_str(&format!("{}:\n", label_else));
                    self.compile_block(else_block);
                    self.output.push_str(&format!("  br label %{}\n", label_merge));
                }
                
                // Birleşme noktası
                self.output.push_str(&format!("{}:\n", label_merge));
            }
        }
    }

    fn compile(&mut self, stmts: &[Stmt]) -> String {
        let mut body = String::new(); 
        self.output = String::new(); 
        for stmt in stmts { self.compile_stmt(stmt); }
        body = self.output.clone(); 
        
        // Header
        let mut header = String::from("; Modül: aa_lang\n");
        header.push_str("declare i32 @printf(i8*, ...)\n");
        header.push_str("@fmt_num = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\"\n");
        header.push_str("@fmt_str = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\"\n");
        for (id, content, len) in &self.string_literals {
             header.push_str(&format!("@str.{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", id, len, content));
        }
        header.push_str("\ndefine i32 @main() {\nentry:\n");
        header.push_str(&body);
        header.push_str("  ret i32 0\n}\n");
        header
    }
}

fn main() {
    // Sadece test.aa dosyasını bulalım
    let file = Path::new("test.aa");
    if file.exists() {
        println!("🚀 Derleniyor: {:?}", file);
        let src = fs::read_to_string(&file).unwrap();
        
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse(); // Yeni parse fonksiyonu Vec<Stmt> dönmeli
        
        let mut compiler = Compiler::new();
        let ir = compiler.compile(&ast);
        
        let out = file.with_extension("ll");
        fs::write(&out, ir).unwrap();
        println!("✅ LLVM IR Oluşturuldu: {:?}", out);
    }
}
