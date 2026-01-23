mod compiler;

use std::fs;
use std::path::Path;
use compiler::lexer::Lexer;
use compiler::parser::Parser;
use compiler::compiler::Compiler;

fn main() {
    // Change directory to source folder so imports work
    if let Err(e) = std::env::set_current_dir("../src") {
        println!("⚠️ Warning: Could not change directory to ../src: {}", e);
    }

    let file = Path::new("example.aur");
    if file.exists() {
        println!("🚀 Compiling: {:?}", file);
        let src = fs::read_to_string(&file).unwrap();
        
        // 1. Lexer (Lexical Analysis)
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize();
        
        // 2. Parser (Syntactic Analysis)
        let mut parser = Parser::new(tokens);
        let ast = parser.parse(); 
        
        // 3. Compiler (Code Generation - LLVM)
        let mut compiler = Compiler::new();
        let ir = compiler.compile(&ast);
        
        // 4. Write to File
        let out = file.with_extension("ll");
        fs::write(&out, ir).unwrap();
        println!("✅ LLVM IR Generated: {:?}", out);
    } else {
        println!("❌ Error: example.aur file not found.");
    }
}
