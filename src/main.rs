use std::fs;

// Token türlerimizi tanımlayalım (Enum kullanarak)
#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Var,        // var
    Print,      // print
    Id(String), // x, y, result
    Number(i32),// 5, 10
    Assign,     // =
    Plus,       // +
    Minus,      // -
    Mul,        // *
    Div,        // /
    LParen,     // (
    RParen,     // )
    EOF,        // Dosya sonu
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

    // Sıradaki karakteri oku
    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    // Bir karakter ilerle
    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        if let Some('\n') = c {
            self.line += 1;
        }
        c
    }

    // Boşlukları ve yorumları atla
    fn skip_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' {
                // Yorum satırı kontrolü (//)
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

    // Tüm token'ları üret
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

            tokens.append(&mut vec![Token { kind, line: start_line }]);
        }

        tokens.push(Token { kind: TokenType::EOF, line: self.line });
        tokens
    }
}

fn main() {
    // test.aa dosyasını oku
    let source = fs::read_to_string("test.aa").expect("test.aa dosyası okunamadı!");
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    println!("--- RUST LEXER SONUCU ---");
    for token in tokens {
        println!("{:?}", token);
    }
}
