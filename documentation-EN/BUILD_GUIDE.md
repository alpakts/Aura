# 🛠️ Aura Build and Installation Guide

Aura is a high-performance 64-bit compiler. It translates `.aur` source code into LLVM IR and then links it with a native runtime using Clang.

## 📋 System Requirements

### Windows
1.  **Rust**: To build the compiler itself.
2.  **LLVM (Clang)**: Required for the final native linking stage.
    *   `winget install LLVM`
3.  **Visual Studio Build Tools**: Required for the Windows SDK and C Runtime libraries.

### Linux (Ubuntu/Debian)
1.  **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2.  **Clang & Build Essentials**:
    ```bash
    sudo apt update
    sudo apt install clang build-essential
    ```

### macOS
1.  **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2.  **Xcode Command Line Tools**:
    ```bash
    xcode-select --install
    ```

---

## 🚀 Building the Compiler

Navigate to the `compiler` directory and build the host compiler:

```bash
cd compiler
cargo build --release
```

The resulting binary will be in `target/release/aura` (or `aura.exe` on Windows).

---

## 🏗️ Compiling Aura Programs

Aura makes it easy. You just point to your main source file:

```bash
# General usage
aura build path/to/main.aur

# Direct compilation and execution (Development mode)
cargo run -- ../src/main.aur
```

### What happens under the hood?
1.  **Aura Lexer/Parser**: Scans your code and builds an AST.
2.  **Aura Compiler**: Generates 64-bit **LLVM IR (.ll)**.
3.  **Native Linker (Clang)**: Automatically detects your OS (Windows, Linux, or macOS), finds the appropriate runtime libraries (WinSock, LibC, etc.), and produces a native executable in the `dist/` folder.

---

## 📂 Project Structure

*   `src/`: Your Aura source code (`.aur` files).
*   `src/views/`: HTML templates for the MVC engine.
*   `src/dist/`: Where the final native binaries are stored.
*   `compiler/src/`: The Rust source code for the Aura compiler.
*   `compiler/src/compiler/aura_runtime.c`: The core C runtime for Aura.
*   `compiler/src/compiler/aura_mvc.c`: The MVC and Template engine implementation.

---

## 🔧 Troubleshooting

### "clang not found"
Ensure LLVM is in your system PATH. On Linux/Mac, this is usually automatic. On Windows, you might need to restart your terminal after installation.

### "Link Error: unresolved external symbol" (Windows)
Aura tries to find Visual Studio paths via the registry. If it fails, ensure "Desktop development with C++" is installed in the Visual Studio Installer.

### Running on 32-bit Systems
**Aura is 64-bit native.** It uses `i64` for all integers and pointers. Compiling for 32-bit platforms is currently not supported.
