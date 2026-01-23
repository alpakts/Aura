mod lexer;
mod parser;
mod compiler;

use std::fs;
use std::path::Path;
use lexer::Lexer;
use parser::Parser;
use compiler::Compiler;

fn main() {
    let file = Path::new("example.aur");
    if file.exists() {
        println!("🚀 Derleniyor: {:?}", file);
        let src = fs::read_to_string(&file).unwrap();
        
        // 1. Lexer (Sözcüksel Analiz)
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize();
        
        // 2. Parser (Sözdizimsel Analiz)
        let mut parser = Parser::new(tokens);
        let ast = parser.parse(); 
        
        // 3. Compiler (Kod Üretimi - LLVM)
        let mut compiler = Compiler::new();
        let ir = compiler.compile(&ast);
        
        // 4. Dosyaya Yazma
        let out = file.with_extension("ll");
        fs::write(&out, ir).unwrap();
        println!("✅ LLVM IR Oluşturuldu: {:?}", out);
    } else {
        println!("❌ Hata: example.aur dosyası bulunamadı.");
    }
}
