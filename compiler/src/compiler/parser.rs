use crate::compiler::lexer::{Token, TokenType, Lexer}; // Updated import to include Lexer

#[derive(Debug)]
pub enum Expr {
    Number(i32), 
    String(String), 
    Variable(String),
    ArrayLiteral(Vec<Expr>), 
    IndexAccess(String, Box<Expr>), 
    Call(String, Vec<Expr>), // func(arg1, arg2)
    Binary(Box<Expr>, TokenType, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    VarDecl(String, Expr), 
    Assignment(String, Expr), 
    Print(Expr),
    IfStmt(Expr, Vec<Stmt>, Option<Vec<Stmt>>), 
    WhileStmt(Expr, Vec<Stmt>),
    BlockStmt(Vec<Stmt>), 
    FuncDecl(String, Vec<String>, Vec<Stmt>), // name, args, body
    ReturnStmt(Option<Expr>),
    ExprStmt(Expr), // Side-effect için kullanılan ifadeler (func call gibi)
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
    
    // Primary expressions: Number, String, Variable, ArrayLiteral, Paren
    fn parse_primary(&mut self) -> Expr {
        let t = self.peek().clone();
        
        match t.kind {
            TokenType::Number(n) => { self.advance(); Expr::Number(n) },
            TokenType::String(s) => { self.advance(); Expr::String(s) },
            TokenType::Id(n) => {
                self.advance();
                // Değişken mi, çağrı mı, dizi mi?
                if self.peek().kind == TokenType::LBracket {
                    // Array: arr[0]
                    self.advance(); 
                    let index = self.parse_expr();
                    self.consume(TokenType::RBracket, "Expected ']'"); 
                    Expr::IndexAccess(n, Box::new(index))
                } else if self.peek().kind == TokenType::LParen {
                    // Call: func(args)
                    self.advance(); // (
                    let mut args = Vec::new();
                    if self.peek().kind != TokenType::RParen {
                        args.push(self.parse_expr());
                        while self.peek().kind == TokenType::Comma {
                            self.advance(); // ,
                            args.push(self.parse_expr());
                        }
                    }
                    self.consume(TokenType::RParen, "Expected ')'");
                    Expr::Call(n, args)
                } else {
                    // Normal değişken
                    Expr::Variable(n)
                }
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
            _ => panic!("Beklenmeyen token: {:?}", t),
        }
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
            TokenType::Import => {
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
                } else {
                    panic!("Import statement must be followed by a string literal.");
                }
            }
            TokenType::Func => {
                // func name(arg1, arg2) { body }
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
                self.advance(); // return
                let expr = if self.peek().kind != TokenType::Semicolon {
                     Some(self.parse_expr())
                } else {
                     None
                };
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
                self.advance();
                self.consume(TokenType::LParen, "Expected '('");
                let condition = self.parse_expr();
                self.consume(TokenType::RParen, "Expected ')'");
                let block = self.parse_block();
                Stmt::WhileStmt(condition, block)
            }
            TokenType::For => {
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
            TokenType::Id(name) => {
                self.advance(); 
                if self.peek().kind == TokenType::LParen {
                    // Function Call (as Stmt)
                    self.advance(); // (
                    let mut args = Vec::new();
                    if self.peek().kind != TokenType::RParen {
                        args.push(self.parse_expr());
                        while self.peek().kind == TokenType::Comma { self.advance(); args.push(self.parse_expr()); }
                    }
                    self.consume(TokenType::RParen, "Expected ')'");
                    self.consume(TokenType::Semicolon, "Expected ';'");
                    Stmt::ExprStmt(Expr::Call(name, args)) 
                } else {
                    self.consume(TokenType::Assign, "Expected '='");
                    let expr = self.parse_expr();
                    self.consume(TokenType::Semicolon, "Expected ';'");
                    Stmt::Assignment(name, expr)
                }
            }
            _ => panic!("Unknown expression or token: {:?}", t),
        }
    }
    
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut s = Vec::new();
        while self.peek().kind != TokenType::EOF { s.push(self.parse_stmt()); }
        s
    }
}
