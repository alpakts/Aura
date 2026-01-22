use crate::lexer::{Token, TokenType};

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
                    // Dizi: arr[0]
                    self.advance(); 
                    let index = self.parse_expr();
                    self.consume(TokenType::RBracket, "']' bekleniyor"); 
                    Expr::IndexAccess(n, Box::new(index))
                } else if self.peek().kind == TokenType::LParen {
                    // Çağrı: func(args)
                    self.advance(); // (
                    let mut args = Vec::new();
                    if self.peek().kind != TokenType::RParen {
                        args.push(self.parse_expr());
                        while self.peek().kind == TokenType::Comma {
                            self.advance(); // ,
                            args.push(self.parse_expr());
                        }
                    }
                    self.consume(TokenType::RParen, "')' bekleniyor");
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
                self.consume(TokenType::RBracket, "']' bekleniyor"); // ]
                Expr::ArrayLiteral(elements)
            },
            TokenType::LParen => {
                self.advance(); // (
                let e = self.parse_expr(); 
                self.consume(TokenType::RParen, "')' eksik"); 
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
            TokenType::Func => {
                // func name(arg1, arg2) { body }
                self.advance();
                let name = if let TokenType::Id(n) = self.advance().kind { n } else { panic!("Fonksiyon adı eksik") };
                
                self.consume(TokenType::LParen, "'(' bekleniyor");
                let mut args = Vec::new();
                if self.peek().kind != TokenType::RParen {
                    if let TokenType::Id(arg) = self.advance().kind { args.push(arg); }
                    while self.peek().kind == TokenType::Comma {
                        self.advance();
                        if let TokenType::Id(arg) = self.advance().kind { args.push(arg); }
                    }
                }
                self.consume(TokenType::RParen, "')' bekleniyor");
                let body = self.parse_block();
                Stmt::FuncDecl(name, args, body)
            }
            TokenType::Return => {
                self.advance(); // return
                let expr = if self.peek().kind != TokenType::Semicolon && self.peek().kind != TokenType::RBrace {
                     Some(self.parse_expr())
                } else {
                     None
                };
                if self.peek().kind == TokenType::Semicolon { self.advance(); }
                Stmt::ReturnStmt(expr)
            }
            TokenType::Var => {
                self.advance();
                if let TokenType::Id(name) = self.advance().kind {
                    self.consume(TokenType::Assign, "'=' bekleniyor");
                    let expr = self.parse_expr();
                    if self.peek().kind == TokenType::Semicolon { self.advance(); }
                    Stmt::VarDecl(name, expr)
                } else { panic!("Değişken adı bekleniyor"); }
            }
            TokenType::Print => {
                self.advance(); self.consume(TokenType::LParen, "'(' bekleniyor");
                let e = self.parse_expr();
                self.consume(TokenType::RParen, "')' bekleniyor");
                if self.peek().kind == TokenType::Semicolon { self.advance(); }
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
                self.consume(TokenType::LParen, "'(' bekleniyor");
                let condition = self.parse_expr();
                self.consume(TokenType::RParen, "')' bekleniyor");
                let block = self.parse_block();
                Stmt::WhileStmt(condition, block)
            }
            TokenType::For => {
                self.advance(); // for
                self.consume(TokenType::LParen, "'(' bekleniyor");
                let mut init_stmts = Vec::new();
                if self.peek().kind != TokenType::Semicolon {
                    init_stmts.push(self.parse_stmt()); 
                } else { self.advance(); }
                let condition = if self.peek().kind != TokenType::Semicolon {
                    self.parse_expr()
                } else { Expr::Number(1) };
                self.consume(TokenType::Semicolon, "Koşuldan sonra ';' bekleniyor");
                let mut step_stmts = Vec::new();
                if self.peek().kind != TokenType::RParen {
                    if let TokenType::Id(name) = self.peek().kind.clone() {
                        self.advance(); 
                        self.consume(TokenType::Assign, "'=' bekleniyor");
                        let expr = self.parse_expr();
                        step_stmts.push(Stmt::Assignment(name, expr));
                    }
                }
                self.consume(TokenType::RParen, "')' bekleniyor");
                let mut body = self.parse_block();
                body.extend(step_stmts);
                let while_loop = Stmt::WhileStmt(condition, body);
                init_stmts.push(while_loop);
                Stmt::BlockStmt(init_stmts)
            }
            TokenType::Id(name) => {
                self.advance(); 
                if self.peek().kind == TokenType::LParen {
                    // Fonksiyon Çağrısı (Stmt olarak)
                    self.advance(); // (
                    let mut args = Vec::new();
                    if self.peek().kind != TokenType::RParen {
                        args.push(self.parse_expr());
                        while self.peek().kind == TokenType::Comma { self.advance(); args.push(self.parse_expr()); }
                    }
                    self.consume(TokenType::RParen, "')' bekleniyor");
                    if self.peek().kind == TokenType::Semicolon { self.advance(); }
                    Stmt::ExprStmt(Expr::Call(name, args)) 
                } else {
                    self.consume(TokenType::Assign, "'=' bekleniyor");
                    let expr = self.parse_expr();
                    if self.peek().kind == TokenType::Semicolon { self.advance(); }
                    Stmt::Assignment(name, expr)
                }
            }
            _ => panic!("Bilinmeyen ifade: {:?}", t),
        }
    }
    
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut s = Vec::new();
        while self.peek().kind != TokenType::EOF { s.push(self.parse_stmt()); }
        s
    }
}
