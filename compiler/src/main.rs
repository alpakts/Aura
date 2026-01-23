mod compiler;

use std::fs;
use std::path::Path;
use compiler::lexer::Lexer;
use compiler::parser::Parser;
use compiler::compiler::Compiler;

fn main() {
    // 1. Get Input Path (File or Directory)
    let args: Vec<String> = std::env::args().collect();
    let arg_path = if args.len() > 1 { &args[1] } else { "." }; // Default to current directory if no args

    let path = Path::new(arg_path);
    
    // Resolve absolute path
    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().unwrap().join(path)
    };

    let abs_path = match fs::canonicalize(&abs_path) {
        Ok(p) => p,
        Err(_) => {
             println!("❌ Error: Path {:?} not found.", abs_path);
             return;
        }
    };

    // Determine Input File
    let input_file = if abs_path.is_dir() {
        abs_path.join("main.aur")
    } else {
        abs_path.clone()
    };

    if !input_file.exists() {
        println!("❌ Error: Input file {:?} not found.", input_file);
        if abs_path.is_dir() {
            println!("   (Looked for 'main.aur' in directory {:?})", abs_path);
        }
        return;
    }

    let source_dir = input_file.parent().unwrap();
    
    // 2. Setup Dist Directory (Inside Source Directory)
    let dist_dir = source_dir.join("dist");

    if !dist_dir.exists() {
        if let Err(e) = fs::create_dir_all(&dist_dir) {
             println!("❌ Error creating dist directory: {}", e);
             return;
        }
    }

    // 3. Change Working Directory to Source Directory
    // This ensures `import "math.aur"` works if math.aur is next to example.aur
    if let Err(e) = std::env::set_current_dir(source_dir) {
        println!("⚠️ Warning: Could not change directory to {:?}: {}", source_dir, e);
    }
    
    println!("📂 Working Directory: {:?}", source_dir);
    println!("🚀 Compiling: {:?}", input_file);
    
    let src = fs::read_to_string(&input_file).unwrap();
    let file_stem = input_file.file_stem().unwrap().to_str().unwrap();

    // 1. Lexer
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    
    // 2. Parser
    let mut parser = Parser::new(tokens, source_dir.to_path_buf());
    let ast = parser.parse(); 
    
    // 3. Compiler
    let mut compiler = Compiler::new();
    let ir = compiler.compile(&ast);
    
    // 4. Write LLVM IR to dist
    let ll_path = dist_dir.join(format!("{}.ll", file_stem));
    fs::write(&ll_path, ir).unwrap();
    println!("✅ LLVM IR Generated: {:?}", ll_path);

    // 5. Compile to Native Executable
    println!("🔨 Compiling to Native Executable with Clang...");
    let exe_path = dist_dir.join(format!("{}.exe", file_stem));
    
    let mut clang_cmd = std::process::Command::new("clang");
    clang_cmd
        .arg(&ll_path)
        .arg("-o")
        .arg(&exe_path)
        .arg("-target")
        .arg("i686-pc-windows-msvc")
        .arg("-l")
        .arg("legacy_stdio_definitions")
        .arg("-l")
        .arg("msvcrt")
        .arg("-Wno-override-module"); 

    println!("Running: {:?}", clang_cmd);

    match clang_cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("🎉 Successfully compiled to {:?}", exe_path);
                
                println!("🚀 Running executable...");
                println!("--------------------------------------------------");
                let _ = std::process::Command::new(&exe_path).status();
                println!("\n--------------------------------------------------");
            } else {
                println!("❌ Clang Compilation Failed:");
                println!("{}", String::from_utf8_lossy(&output.stderr));
                if String::from_utf8_lossy(&output.stderr).contains("visual studio") {
                        println!("⚠️ Hint: It looks like Clang can't find the Visual Studio linker. Try running this from the 'Developer PowerShell for Visual Studio' or ensure your environment variables are set correctly.");
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to execute clang: {}", e);
            println!("Make sure clang is installed and in your PATH.");
        }
    }
}
// Remove old main body as we rewrote entirely
fn _unused() {}
