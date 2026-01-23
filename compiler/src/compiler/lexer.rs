
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Var, Print, If, Else,
    While, For, Foreach, In, 
    Func, Return, Import, From,
    Class, New, // Class support
    Id(String), Number(i32), String(String), 
    Assign, Plus, Minus, Mul, Div, 
    LParen, RParen, LBrace, RBrace, // { }
    LBracket, RBracket, Comma, Semicolon, Dot, // [ ] , ; .
    Eq, Neq, Lt, Gt, Lte, Gte,      // == != < > <= >=
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token { 
    pub kind: TokenType, 
    pub line: usize 
}

pub struct Lexer { 
    input: Vec<char>, 
    pos: usize, 
    line: usize 
}

impl Lexer {
    pub fn new(input: String) -> Self { 
        Self { input: input.chars().collect(), pos: 0, line: 1 } 
    }
    
    fn peek(&self) -> Option<char> { 
        self.input.get(self.pos).copied() 
    }
    
    fn create_token(&self, kind: TokenType) -> Token { 
        Token { kind, line: self.line } 
    }
    
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
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = { self.skip_whitespace_and_comments(); self.peek() } {
            let kind = match c {
                '.' => { self.advance(); TokenType::Dot },
                '=' => { 
                    self.advance(); 
                    if self.peek() == Some('=') { self.advance(); TokenType::Eq } else { TokenType::Assign }
                },
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); TokenType::Neq } else { panic!("Line {}: Expected '=' after '!'", self.line) }
                },
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
                '[' => { self.advance(); TokenType::LBracket },
                ']' => { self.advance(); TokenType::RBracket },
                ',' => { self.advance(); TokenType::Comma },
                ';' => { self.advance(); TokenType::Semicolon },
                '"' => {
                    self.advance(); let mut s = String::new();
                    while let Some(ch) = self.peek() { if ch == '"' { break; } s.push(self.advance().unwrap()); }
                    if self.peek() == Some('"') { self.advance(); } else { panic!("Run-away string literal!"); }
                    TokenType::String(s)
                },
                '0'..='9' => {
                    let mut s = String::new();
                    while let Some(ch) = self.peek() { if ch.is_digit(10) { s.push(self.advance().unwrap()); } else { break; } }
                    TokenType::Number(s.parse().unwrap())
                },
                'a'..='z'|'A'..='Z'|'_' => {
                    let mut s = String::new();
                    while let Some(ch) = self.peek() { if ch.is_alphanumeric()||ch=='_' { s.push(self.advance().unwrap()); } else { break; } }
                    match s.as_str() { 
                        "var"=>TokenType::Var, "print"=>TokenType::Print, 
                        "if"=>TokenType::If, "else"=>TokenType::Else, 
                        "while"=>TokenType::While, "for"=>TokenType::For,
                        "foreach"=>TokenType::Foreach, "in"=>TokenType::In,
                        "func"=>TokenType::Func, "return"=>TokenType::Return,
                        "import"=>TokenType::Import, "from"=>TokenType::From,
                        "class"=>TokenType::Class, "new"=>TokenType::New,
                        _=>TokenType::Id(s) 
                    }
                },
                _ => panic!("Unknown character: {}", c),
            };
            tokens.push(self.create_token(kind));
        }
        tokens.push(self.create_token(TokenType::EOF));
        tokens
    }
}
