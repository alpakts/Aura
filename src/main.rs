use std::fs;
use std::path::Path;

// --- AŞAMA 2: LEXER (SÖZCÜKSEL ANALİZ) ---
// Kodun metin halini alıp kelimelere (Token) böler.

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Var,        
    Print,      
    Id(String), 
    Number(i32),
    String(String), 
    Assign,     
    Plus,       
    Minus,      
    Mul,        
    Div,        
    LParen,     
    RParen,     
    EOF,        
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenType,
    line: usize,
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        if let Some('\n') = c {
            self.line += 1;
        }
        c
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' {
                if self.input.get(self.pos + 1) == Some(&'/') {
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.advance();
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = { self.skip_whitespace_and_comments(); self.peek() } {
            let start_line = self.line;
            let kind = match c {
                '=' => { self.advance(); TokenType::Assign },
                '+' => { self.advance(); TokenType::Plus },
                '-' => { self.advance(); TokenType::Minus },
                '*' => { self.advance(); TokenType::Mul },
                '/' => { self.advance(); TokenType::Div },
                '(' => { self.advance(); TokenType::LParen },
                ')' => { self.advance(); TokenType::RParen },
                '"' => {
                    self.advance();
                    let mut string_val = String::new();
                    while let Some(c) = self.peek() {
                        if c == '"' { break; }
                        string_val.push(self.advance().unwrap());
                    }
                    if self.peek() == Some('"') {
                        self.advance();
                    } else {
                        panic!("Hata: Kapanmamış string! Satır: {}", self.line);
                    }
                    TokenType::String(string_val)
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_digit(10) {
                            num_str.push(self.advance().unwrap());
                        } else {
                            break;
                        }
                    }
                    TokenType::Number(num_str.parse().unwrap())
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut id_str = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            id_str.push(self.advance().unwrap());
                        } else {
                            break;
                        }
                    }
                    match id_str.as_str() {
                        "var" => TokenType::Var,
                        "print" => TokenType::Print,
                        _ => TokenType::Id(id_str),
                    }
                }
                _ => panic!("Beklenmeyen karakter: {} satır {}", c, self.line),
            };
            tokens.push(Token { kind, line: start_line });
        }
        tokens.push(Token { kind: TokenType::EOF, line: self.line });
        tokens
    }
}

// --- AŞAMA 3: PARSER (SOYUT SÖZDİZİM AĞACI - AST) ---
// Kelimeleri (token) alır, kurallara uygun cümleler (AST) kurar.

#[derive(Debug)]
enum Expr {
    Number(i32),
    String(String),
    Variable(String),
    Binary(Box<Expr>, TokenType, Box<Expr>), 
}

#[derive(Debug)]
enum Stmt {
    VarDecl(String, Expr), 
    Assignment(String, Expr), 
    Print(Expr),           
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        if token.kind != TokenType::EOF {
            self.pos += 1;
        }
        token
    }

    fn consume(&mut self, expected: TokenType, msg: &str) {
        let token = self.advance();
        if std::mem::discriminant(&token.kind) != std::mem::discriminant(&expected) {
            panic!("Parser Hatası Satır {}: {}", token.line, msg);
        }
    }

    fn parse_primary(&mut self) -> Expr {
        let token = self.advance();
        match token.kind {
            TokenType::Number(n) => Expr::Number(n),
            TokenType::String(s) => Expr::String(s),
            TokenType::Id(name) => Expr::Variable(name),
            TokenType::LParen => {
                let expr = self.parse_expression();
                self.consume(TokenType::RParen, "')' bekleniyordu.");
                expr
            }
            _ => panic!("Beklenmeyen ifade: {:?} satır {}", token.kind, token.line),
        }
    }

    fn parse_multiplication(&mut self) -> Expr {
        let mut node = self.parse_primary();
        while matches!(self.peek().kind, TokenType::Mul | TokenType::Div) {
            let op = self.advance().kind;
            let right = self.parse_primary();
            node = Expr::Binary(Box::new(node), op, Box::new(right));
        }
        node
    }

    fn parse_expression(&mut self) -> Expr {
        let mut node = self.parse_multiplication();
        while matches!(self.peek().kind, TokenType::Plus | TokenType::Minus) {
            let op = self.advance().kind;
            let right = self.parse_multiplication();
            node = Expr::Binary(Box::new(node), op, Box::new(right));
        }
        node
    }

    fn parse_statement(&mut self) -> Stmt {
        let token = self.peek().clone();
        match token.kind {
            TokenType::Var => {
                self.advance(); 
                if let TokenType::Id(name) = self.advance().kind {
                    self.consume(TokenType::Assign, "'=' bekleniyordu.");
                    let expr = self.parse_expression();
                    Stmt::VarDecl(name, expr)
                } else {
                    panic!("'var'dan sonra değişken ismi bekleniyordu.");
                }
            }
            TokenType::Print => {
                self.advance(); 
                self.consume(TokenType::LParen, "'(' bekleniyordu.");
                let expr = self.parse_expression();
                self.consume(TokenType::RParen, "')' bekleniyordu.");
                Stmt::Print(expr)
            }
            TokenType::Id(name) => {
                self.advance(); 
                self.consume(TokenType::Assign, "'=' bekleniyordu.");
                let expr = self.parse_expression();
                Stmt::Assignment(name, expr)
            }
            _ => panic!("Bilinmeyen komut: {:?} satır {}", token.kind, token.line),
        }
    }

    fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while self.peek().kind != TokenType::EOF {
            statements.push(self.parse_statement());
        }
        statements
    }
}

// --- AŞAMA 4: COMPILER (LLVM IR KOD ÜRETİMİ) ---
// AST'yi alır, LLVM IR formatında metin çıktısı üretir.

struct Compiler {
    output: String,
    reg_counter: i32,
    str_counter: i32,
    strings: Vec<(i32, String)>, 
    declared_vars: std::collections::HashSet<String>, // Tanımlanmış değişkenleri takip et
}

impl Compiler {
    fn new() -> Self {
        Self { 
            output: String::new(), 
            reg_counter: 1,
            str_counter: 0,
            strings: Vec::new(),
            declared_vars: std::collections::HashSet::new(),
        }
    }

    fn get_reg(&mut self) -> String {
        let r = format!("%{}", self.reg_counter);
        self.reg_counter += 1;
        r
    }

    fn compile_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => format!("{}", n),
            Expr::Variable(name) => {
                let reg = self.get_reg();
                // Değişkeni bellekten yükle (Load)
                self.output.push_str(&format!("  {} = load i32, i32* %{}_ptr\n", reg, name));
                reg
            }
            Expr::Binary(left, op, right) => {
                let l_val = self.compile_expr(left);
                let r_val = self.compile_expr(right);
                let reg = self.get_reg();
                let op_str = match op {
                    TokenType::Plus => "add",
                    TokenType::Minus => "sub",
                    TokenType::Mul => "mul",
                    TokenType::Div => "sdiv", // Signed division
                    _ => panic!("Desteklenmeyen işlem"),
                };
                self.output.push_str(&format!("  {} = {} i32 {}, {}\n", reg, op_str, l_val, r_val));
                reg
            }
            Expr::String(s) => {
                // Stringleri global olarak tanımlamak gerekir, basitlik için burada pas geçiyoruz
                // İleri seviye string işlemleri için daha karmaşık yapı gerekir
                // Şimdilik sadece sayısal işlemlere odaklanalım
                 format!("\"{}\"", s)
            }
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl(name, expr) => {
                // Değişken için bellekte yer ayır (Alloca)
                self.output.push_str(&format!("  %{}_ptr = alloca i32\n", name));
                self.declared_vars.insert(name.clone()); // Değişkeni kaydet
                let val = self.compile_expr(expr);
                // Değeri belleğe yaz (Store)
                self.output.push_str(&format!("  store i32 {}, i32* %{}_ptr\n", val, name));
            }
            Stmt::Assignment(name, expr) => {
                // Eğer değişken daha önce tanımlanmadıysa, önce alloca yap
                if !self.declared_vars.contains(name) {
                    self.output.push_str(&format!("  %{}_ptr = alloca i32\n", name));
                    self.declared_vars.insert(name.clone());
                }
                let val = self.compile_expr(expr);
                self.output.push_str(&format!("  store i32 {}, i32* %{}_ptr\n", val, name));
            }
            Stmt::Print(expr) => {
                let val = self.compile_expr(expr);
                // printf çağrısı
                // Basitlik için sadece sayı yazdırıyoruz şimdilik (Format string: "%d\n")
                self.output.push_str(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @fmt_num, i32 0, i32 0), i32 {})\n", val));
            }
        }
    }

    fn compile(&mut self, stmts: &[Stmt]) -> String {
        // LLVM Başlıkları
        let mut final_code = String::from("; Modül: aa_lang\n");
        final_code.push_str("declare i32 @printf(i8*, ...)\n");
        final_code.push_str("@fmt_num = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\"\n\n");
        
        final_code.push_str("define i32 @main() {\nentry:\n");
        
        // Şimdi statement'ları derle - bunlar self.output'a yazılacak
        for stmt in stmts {
            self.compile_stmt(stmt);
        }

        // self.output'taki kodları main fonksiyonunun içine ekle
        final_code.push_str(&self.output);
        
        // Fonksiyonu kapat
        final_code.push_str("  ret i32 0\n}\n");
        
        final_code
    }
}

// Dosya tarama fonksiyonu
fn find_aa_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find_aa_files(&path, files)?;
            } else {
                if let Some(ext) = path.extension() {
                    if ext == "aa" {
                        files.push(path);
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    println!("📂 Tarama başlatılıyor: {:?}", current_dir);

    let mut aa_files = Vec::new();
    find_aa_files(&current_dir, &mut aa_files).expect("Dosya tarama hatası");

    if aa_files.is_empty() {
        println!("❌ Hiç .aa dosyası bulunamadı.");
        return;
    }

    println!("🔎 Bulunan dosyalar: {:?}", aa_files);

    for file_path in aa_files {
        println!("\n🚀 Derleniyor: {:?}", file_path);
        let source = fs::read_to_string(&file_path).expect("Dosya okunamadı!");
        
        // 1. Lexing
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        // 2. Parsing
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program();

        // 3. Compiling only if numeric (basitleştirilmiş)
        // String desteği string pointer mantığı gerektirdiği için 
        // şu anlık derleyici sadece sayısal çıktı verecek şekilde ayarlı.
        let mut compiler = Compiler::new();
        // Sadece test amaçlı string içeren satırları filtreliyorum hata vermesin diye
        // (Gerçek bir derleyicide string tablosu oluşturulur)
        let filtered_ast: Vec<Stmt> = ast.into_iter().filter(|stmt| {
            match stmt {
                Stmt::Print(Expr::String(_)) => false,
                Stmt::VarDecl(_, Expr::String(_)) => false,
                _ => true
            }
        }).collect();

        if filtered_ast.len() < 1 {
            println!("⚠️ Bu dosya sadece string içeriyor, şimdilik sadece sayısal işlemler derleniyor.");
            continue;
        }

        let llvm_ir = compiler.compile(&filtered_ast);

        // .ll dosyasını kaydet
        let mut output_path = file_path.clone();
        output_path.set_extension("ll"); // test.aa -> test.ll
        fs::write(&output_path, &llvm_ir).expect("LLVM IR kaydedilemedi");
        
        println!("✅ BAŞARILI! LLVM IR kodu oluşturuldu: {:?}", output_path);
        println!("📝 İçerik Önizleme:\n{}", llvm_ir);
    }
}
