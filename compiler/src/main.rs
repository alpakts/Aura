mod compiler;

use std::fs;
use std::path::{Path, PathBuf};
use compiler::lexer::Lexer;
use compiler::parser::Parser;
use compiler::compiler::Compiler;

/// Visual Studio yollarını otomatik bulmak için yardımcı fonksiyon
fn find_msvc_paths() -> Option<(Vec<PathBuf>, Vec<PathBuf>)> {
    println!("🔍 Searching for Visual Studio & Windows SDK libraries...");
    
    // Windows Registry üzerinden kütüphane yollarını tara
    #[cfg(windows)]
    {
        use cc::windows_registry;
        if let Some(tool) = windows_registry::find_tool("x86_64-pc-windows-msvc", "cl.exe") {
            let mut lib_paths = Vec::new();
            let mut include_paths = Vec::new();
            
            for path in tool.env() {
                if path.0 == "LIB" {
                    for p in std::env::split_paths(&path.1) {
                        lib_paths.push(p);
                    }
                } else if path.0 == "INCLUDE" {
                    for p in std::env::split_paths(&path.1) {
                        include_paths.push(p);
                    }
                }
            }
            if !lib_paths.is_empty() {
                println!("✅ Found {} library and {} include search paths in system.", lib_paths.len(), include_paths.len());
                return Some((lib_paths, include_paths));
            }
        }
    }
    
    // OFFLINE FALLBACK: Kendi 'lib' klasörümüze bak (Aura-SDK/lib)
    if let Ok(exe_p) = std::env::current_exe() {
        if let Some(parent) = exe_p.parent() {
            let local_lib = parent.parent().unwrap_or(parent).join("lib");
            if local_lib.exists() {
                println!("✅ Found local library path: {:?}", local_lib);
                return Some((vec![local_lib], Vec::new()));
            }
        }
    }

    println!("⚠️ Note: No libraries found. Installation might be corrupted.");
    None
}

fn main() {
    // 1. Dosya Yolunu Al
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("🚀 Aura Programming Language v0.1.0");
        println!("Usage:");
        println!("  aura build <file.aur>   - Compile to EXE");
        println!("  aura <file.aur>         - Compile to EXE (Direct)");
        println!("  aura version            - Show version");
        return;
    }

    let mut arg_path = &args[1];
    
    // Alt komut kontrolü (Subcommands)
    if arg_path == "build" {
        if args.len() > 2 {
             arg_path = &args[2];
        } else {
             println!("❌ Error: Please provide a file to build.");
             return;
        }
    } else if arg_path == "version" {
        println!("Aura Engine v0.1.0 (Experimental)");
        return;
    }

    let path = Path::new(arg_path);
    let abs_path = if path.is_absolute() { path.to_path_buf() } else { std::env::current_dir().unwrap().join(path) };
    let abs_path = match fs::canonicalize(&abs_path) { Ok(p) => p, Err(_) => { println!("❌ Error: Path {:?} not found.", abs_path); return; } };

    let input_file = if abs_path.is_dir() { abs_path.join("main.aur") } else { abs_path.clone() };
    if !input_file.exists() { println!("❌ Error: Input file {:?} not found.", input_file); return; }

    let source_dir = input_file.parent().unwrap();
    let dist_dir = source_dir.join("dist");
    if !dist_dir.exists() { fs::create_dir_all(&dist_dir).unwrap(); }

    if let Err(e) = std::env::set_current_dir(source_dir) { println!("⚠️ Warning: Cwd error: {}", e); }
    
    println!("🚀 Compiling: {:?}", input_file);
    let src = fs::read_to_string(&input_file).unwrap();
    let file_stem = input_file.file_stem().unwrap().to_str().unwrap();

    // -- Derleme Aşamaları --
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens, source_dir.to_path_buf());
    let ast = parser.parse(); 
    let mut compiler = Compiler::new();
    let ir = compiler.compile(&ast);
    
    let ll_path = dist_dir.join(format!("{}.ll", file_stem));
    fs::write(&ll_path, ir).unwrap();
    println!("✅ LLVM IR Generated: {:?}", ll_path);

    // 5. Clang ile EXE Oluşturma (AKILLI MOD)
    println!("🔨 Linking to Native Executable...");
    let exe_path = dist_dir.join(format!("{}.exe", file_stem));
    
    let mut clang_cmd = std::process::Command::new("clang");
    clang_cmd
        .arg(&ll_path)
        .arg("-o")
        .arg(&exe_path)
        .arg("-target")
        .arg("x86_64-pc-windows-msvc")
        .arg("-fuse-ld=lld")
        .arg("-Wno-override-module");

    // EĞER KÜTÜPHANELER BULUNDUYSA LİNKERE EKLE
    if let Some((lib_paths, include_paths)) = find_msvc_paths() {
        for p in lib_paths {
            let mut arg_val = String::from("-L");
            arg_val.push_str(p.to_str().unwrap_or(""));
            clang_cmd.arg(arg_val);
        }
        for p in include_paths {
            let mut arg_val = String::from("-I");
            arg_val.push_str(p.to_str().unwrap_or(""));
            clang_cmd.arg(arg_val);
        }
        // Temel kütüphaneleri bağla (Modern MSVC ve Ağ desteği için tam liste)
        clang_cmd.arg("-lmsvcrt");
        clang_cmd.arg("-lucrt");
        clang_cmd.arg("-lvcruntime");
        clang_cmd.arg("-llegacy_stdio_definitions");
        clang_cmd.arg("-lkernel32");
        clang_cmd.arg("-luser32");
        clang_cmd.arg("-lws2_32"); // Ağ desteği için eklendi!
    } else {
        // Fallback: Belki terminalde zaten vardır
        clang_cmd.arg("-lmsvcrt");
        clang_cmd.arg("-llegacy_stdio_definitions");
        clang_cmd.arg("-lws2_32");
    }

    // --- AURA RUNTIME INTEGRATION ---
    // Try to find aura_runtime.c in the project source or relative to exe
    if let Ok(exe_p) = std::env::current_exe() {
        if let Some(p) = exe_p.parent() {
            // Development: compiler/target/debug/aura.exe -> proj_root is compiler/
            let proj_root = p.parent().unwrap_or(p).parent().unwrap_or(p);
            let runtime_file = proj_root.join("src").join("compiler").join("aura_runtime.c");
            if runtime_file.exists() {
                clang_cmd.arg(runtime_file);
            } else {
                // Try nearby (for standalone release)
                let alt_runtime = p.join("aura_runtime.c");
                if alt_runtime.exists() {
                    clang_cmd.arg(alt_runtime);
                }
            }
        }
    }

    match clang_cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("🎉 Success: Compiled to {:?}", exe_path);
                println!("🚀 Running: \n----------------------------------");
                let _ = std::process::Command::new(&exe_path).status();
            } else {
                println!("❌ Link Error:\n{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => println!("❌ Clang execution failed: {}", e),
    }
}
