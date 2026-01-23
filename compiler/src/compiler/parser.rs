use crate::compiler::lexer::{Token, TokenType, Lexer}; // Updated import to include Lexer

#[derive(Debug)]
pub enum Expr {
    Number(i32), 
    String(String), 
    Variable(String),
    ArrayLiteral(Vec<Expr>), 
    IndexAccess(String, Box<Expr>), 
    Call(String, Vec<Expr>), 
    Binary(Box<Expr>, TokenType, Box<Expr>),
    New(String), // new ClassName()
    Get(Box<Expr>, String), // obj.field
    Set(Box<Expr>, String, Box<Expr>), // obj.field = val
}

#[derive(Debug)]
pub enum Stmt {
    VarDecl(String, Expr), 
    Assignment(String, Expr), 
    Print(Expr),
    IfStmt(Expr, Vec<Stmt>, Option<Vec<Stmt>>), 
    WhileStmt(Expr, Vec<Stmt>),
    BlockStmt(Vec<Stmt>), 
    FuncDecl(String, Vec<String>, Vec<Stmt>), 
    ClassDecl(String, Vec<String>), // class Name { var f1; var f2; }
    ReturnStmt(Option<Expr>),
    ExprStmt(Expr), 
}

pub struct Parser { 
    tokens: Vec<Token>, 
    pos: usize 
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { 
        Self { tokens, pos: 0 } 
    }
    
    fn peek(&self) -> &Token { &self.tokens[self.pos] }
    
    fn advance(&mut self) -> Token { 
        let t = self.tokens[self.pos].clone(); 
        if t.kind != TokenType::EOF { self.pos += 1; } 
        t 
    }
    
    fn consume(&mut self, expected: TokenType, msg: &str) {
        let t = self.advance(); 
        if std::mem::discriminant(&t.kind) != std::mem::discriminant(&expected) { 
            panic!("{}", msg); 
        }
    }
    
    // Primary expressions: Number, String, Variable, ArrayLiteral, Paren, New
    fn parse_primary(&mut self) -> Expr {
        let t = self.peek().clone();
        
        // 1. Atom Parsing
        let mut expr = match t.kind {
            TokenType::Number(n) => { self.advance(); Expr::Number(n) },
            TokenType::String(s) => { self.advance(); Expr::String(s) },
            TokenType::Id(n) => { self.advance(); Expr::Variable(n) },
            TokenType::New => {
                 self.advance(); // new
                 if let TokenType::Id(class_name) = self.advance().kind {
                     self.consume(TokenType::LParen, "Expected '('");
                     self.consume(TokenType::RParen, "Expected ')'");
                     Expr::New(class_name)
                 } else { panic!("Expected class name after 'new'"); }
            },
            TokenType::LBracket => { // Array Literal [1, 2, 3]
                self.advance(); // [
                let mut elements = Vec::new();
                if self.peek().kind != TokenType::RBracket {
                    elements.push(self.parse_expr());
                    while self.peek().kind == TokenType::Comma {
                        self.advance(); // ,
                        elements.push(self.parse_expr());
                    }
                }
                self.consume(TokenType::RBracket, "Expected ']'"); // ]
                Expr::ArrayLiteral(elements)
            },
            TokenType::LParen => {
                self.advance(); // (
                let e = self.parse_expr(); 
                self.consume(TokenType::RParen, "Missing ')'"); 
                e 
            },
            _ => panic!("Unexpected token: {:?}", t),
        };

        // 2. Postfix Loop (Call (), Index [], Member Access .)
        loop {
            match self.peek().kind {
                TokenType::LParen => {
                    // Call
                    self.advance(); 
                    let mut args = Vec::new();
                    if self.peek().kind != TokenType::RParen {
                        args.push(self.parse_expr());
                        while self.peek().kind == TokenType::Comma {
                             self.advance();
                             args.push(self.parse_expr());
                        }
                    }
                    self.consume(TokenType::RParen, "Expected ')'");
                    
                    if let Expr::Variable(name) = expr {
                        expr = Expr::Call(name, args);
                    } else {
                        // For now we only support calling named functions directly, 
                        // but technically `obj.method()` could be supported via this or similar.
                        // Since we don't have first-class func pointers yet, let's restrict or adapt.
                        // Actually, if we add methods later, `Expr::Get` might need to be handled here.
                        // For this iteration: just wrap it. 
                        // Note: Compiler needs to handle Call on non-Variable if we want `(get_func())()`
                        panic!("Function call only supported on identifiers for now");
                    }
                },
                TokenType::LBracket => {
                    // Index
                    self.advance();
                    let index = self.parse_expr();
                    self.consume(TokenType::RBracket, "Expected ']'");
                    if let Expr::Variable(name) = expr {
                         // Simplify: IndexAccess logic in compiler assumes variable name.
                         // But `arr[0][1]` should work. 
                         // Current Compiler `IndexAccess` takes `String` name.
                         // We need to upgrade IndexAccess to take `Box<Expr>` target to support `getArr()[0]`.
                         // For now, let's keep the limitation: `IndexAccess(String, Box<Expr>)`.
                         expr = Expr::IndexAccess(name, Box::new(index));
                    } else {
                         // If we want to support `expression[index]`, we need to change Expr definition.
                         // For this request, let's stick to simple `obj.field`.
                         panic!("Indexing only supported on variables for now");
                    }
                },
                TokenType::Dot => {
                    // Member Access
                    self.advance();
                    if let TokenType::Id(field) = self.advance().kind {
                        expr = Expr::Get(Box::new(expr), field);
                    } else {
                        panic!("Expected field name after '.'");
                    }
                },
                _ => break,
            }
        }
        
        expr
    }
    
    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_primary();
        while matches!(self.peek().kind, TokenType::Mul|TokenType::Div) {
            let op = self.advance().kind.clone();
            node = Expr::Binary(Box::new(node), op, Box::new(self.parse_primary()));
        }
        node
    }
    
    fn parse_arithmetic(&mut self) -> Expr {
        let mut node = self.parse_term();
        while matches!(self.peek().kind, TokenType::Plus|TokenType::Minus) {
            let op = self.advance().kind.clone();
            node = Expr::Binary(Box::new(node), op, Box::new(self.parse_term()));
        }
        node
    }

    fn parse_expr(&mut self) -> Expr {
        let mut node = self.parse_arithmetic();
        while matches!(self.peek().kind, TokenType::Eq|TokenType::Neq|TokenType::Lt|TokenType::Gt|TokenType::Lte|TokenType::Gte) {
             let op = self.advance().kind.clone();
             node = Expr::Binary(Box::new(node), op, Box::new(self.parse_arithmetic()));
        }
        node
    }
    
    fn parse_block(&mut self) -> Vec<Stmt> {
        self.consume(TokenType::LBrace, "Expected '{'");
        let mut stmts = Vec::new();
        while self.peek().kind != TokenType::RBrace && self.peek().kind != TokenType::EOF {
            stmts.push(self.parse_stmt());
        }
        self.consume(TokenType::RBrace, "Expected '}'");
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        let t = self.peek().clone();
        match t.kind {
            TokenType::Class => {
                self.advance(); // class
                let name = if let TokenType::Id(n) = self.advance().kind { n } else { panic!("Expected class name") };
                self.consume(TokenType::LBrace, "Expected '{'");
                let mut fields = Vec::new();
                while self.peek().kind != TokenType::RBrace && self.peek().kind != TokenType::EOF {
                    // Parse "var fieldName;"
                    if let TokenType::Var = self.peek().kind {
                        self.advance();
                        if let TokenType::Id(f_name) = self.advance().kind {
                            self.consume(TokenType::Semicolon, "Expected ';'");
                            fields.push(f_name);
                        } else { panic!("Expected field name"); }
                    } else {
                        panic!("Only variables allowed in class definition for now");
                    }
                }
                self.consume(TokenType::RBrace, "Expected '}'");
                Stmt::ClassDecl(name, fields)
            }
            TokenType::Import => { 
                // ... (Existing Import Logic)
                self.advance(); // import
                let path_token = self.advance();
                if let TokenType::String(path) = path_token.kind {
                    self.consume(TokenType::Semicolon, "Expected ';'");
                    let content = std::fs::read_to_string(&path)
                        .expect(&format!("Could not read imported file: {}", path));
                    let mut lexer = Lexer::new(content);
                    let mut parser = Parser::new(lexer.tokenize());
                    let imported_stmts = parser.parse();
                    Stmt::BlockStmt(imported_stmts)
                } else { panic!("Import panic"); } 
            }
            // ... (Existing Func, Return, Var, Print, If, While, For)
            TokenType::Func => {
                 self.advance();
                 let name = if let TokenType::Id(n) = self.advance().kind { n } else { panic!("Function name missing") };
                 self.consume(TokenType::LParen, "Expected '('");
                 let mut args = Vec::new();
                 if self.peek().kind != TokenType::RParen {
                     if let TokenType::Id(arg) = self.advance().kind { args.push(arg); }
                     while self.peek().kind == TokenType::Comma {
                         self.advance();
                         if let TokenType::Id(arg) = self.advance().kind { args.push(arg); }
                     }
                 }
                 self.consume(TokenType::RParen, "Expected ')'");
                 let body = self.parse_block();
                 Stmt::FuncDecl(name, args, body)
            }
            TokenType::Return => {
                self.advance(); 
                let expr = if self.peek().kind != TokenType::Semicolon { Some(self.parse_expr()) } else { None };
                self.consume(TokenType::Semicolon, "Expected ';'");
                Stmt::ReturnStmt(expr)
            }
            TokenType::Var => {
                self.advance();
                if let TokenType::Id(name) = self.advance().kind {
                    self.consume(TokenType::Assign, "Expected '='");
                    let expr = self.parse_expr();
                    self.consume(TokenType::Semicolon, "Expected ';'");
                    Stmt::VarDecl(name, expr)
                } else { panic!("Expected variable name"); }
            }
            TokenType::Print => {
                self.advance(); self.consume(TokenType::LParen, "Expected '('");
                let e = self.parse_expr();
                self.consume(TokenType::RParen, "Expected ')'");
                self.consume(TokenType::Semicolon, "Expected ';'");
                Stmt::Print(e)
            }
            TokenType::If => {
                 // ... (Keep existing If logic)
                self.advance();
                self.consume(TokenType::LParen, "Expected '('");
                let condition = self.parse_expr();
                self.consume(TokenType::RParen, "Expected ')'");
                let then_block = self.parse_block();
                let mut else_block = None;
                if matches!(self.peek().kind, TokenType::Else) {
                    self.advance();
                    if matches!(self.peek().kind, TokenType::If) {
                        let nested_stmt = self.parse_stmt();
                        else_block = Some(vec![nested_stmt]);
                    } else {
                        else_block = Some(self.parse_block());
                    }
                }
                Stmt::IfStmt(condition, then_block, else_block)
            }
            TokenType::While => {
                 // ... (Keep existing While logic)
                self.advance();
                self.consume(TokenType::LParen, "Expected '('");
                let condition = self.parse_expr();
                self.consume(TokenType::RParen, "Expected ')'");
                let block = self.parse_block();
                Stmt::WhileStmt(condition, block)
            }
            TokenType::For => {
                 // ... (Keep existing For logic)
                self.advance(); // for
                self.consume(TokenType::LParen, "Expected '('");
                let mut init_stmts = Vec::new();
                if self.peek().kind != TokenType::Semicolon {
                    init_stmts.push(self.parse_stmt()); 
                } else { self.advance(); }
                let condition = if self.peek().kind != TokenType::Semicolon {
                    self.parse_expr()
                } else { Expr::Number(1) };
                self.consume(TokenType::Semicolon, "Expected ';' after condition");
                let mut step_stmts = Vec::new();
                if self.peek().kind != TokenType::RParen {
                    if let TokenType::Id(name) = self.peek().kind.clone() {
                        self.advance(); 
                        self.consume(TokenType::Assign, "Expected '='");
                        let expr = self.parse_expr();
                        step_stmts.push(Stmt::Assignment(name, expr));
                    }
                }
                self.consume(TokenType::RParen, "Expected ')'");
                let mut body = self.parse_block();
                body.extend(step_stmts);
                let while_loop = Stmt::WhileStmt(condition, body);
                init_stmts.push(while_loop);
                Stmt::BlockStmt(init_stmts)
            }
            _ => {
                // Determine if it's Assignment or Expression Statement
                // Unlike before, we don't assume `Id` always means assignment.
                // We parse an expression first.
                let expr = self.parse_expr();
                
                if self.peek().kind == TokenType::Assign {
                     // Assignment: l-value = r-value
                     self.advance(); // =
                     let r_val = self.parse_expr();
                     self.consume(TokenType::Semicolon, "Expected ';'");
                     
                     match expr {
                         Expr::Variable(name) => Stmt::Assignment(name, r_val),
                         Expr::Get(obj, field) => Stmt::ExprStmt(Expr::Set(obj, field, Box::new(r_val))),
                         _ => panic!("Invalid assignment target"),
                     }
                } else {
                     // Expression Statement (e.g. Call)
                     self.consume(TokenType::Semicolon, "Expected ';'");
                     Stmt::ExprStmt(expr)
                }
            }
        }
    }
    
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut s = Vec::new();
        while self.peek().kind != TokenType::EOF { s.push(self.parse_stmt()); }
        s
    }
}
