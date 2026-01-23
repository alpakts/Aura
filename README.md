# Aura Compiler (.aur Language) 🦀

![Status](https://img.shields.io/badge/Status-Development-blue)
![Language](https://img.shields.io/badge/Written%20in-Rust-orange)
![Output](https://img.shields.io/badge/Output-LLVM%20IR-green)

**Aura** is a modern compiler developed in Rust for a custom programming language (`.aur`). It encompasses Lexical Analysis (Lexer), Parsing (Parser), and LLVM IR generation (Compiler). The output is compiled into executable `.exe` files on Windows using Clang.

---

## ✨ Features

*   **Variables & Types**: Automatic Type Inference with `int` and `string` support.
*   **Arrays**: Array definition and index access (`arr[0]`).
*   **Control Structures**: `if`, `else if`, `else`, `while`, `for` loops.
*   **Functions**: Functions that can accept parameters and return values.
*   **Built-in Functions**: `print` (numerical) and `print_str` (string) printing functions.
*   **Automation**: Compilation and linking process with a single command (`cargo run`).

---

## 📚 Documentation

Detailed installation, usage, and syntax guides can be found in the following directories:

### 🇬🇧 English
*   **[Syntax Guide](documentation-EN/SYNTAX.md)**: Language rules and examples.
*   **[Build & Installation](documentation-EN/BUILD_GUIDE.md)**: Setup guide for Windows, Clang, and VS Build Tools.

*(Turkish documentation is also available in the `documentation-TR` folder)*

---

## 🏗 Architecture

The compiler consists of 3 main modules:

1.  **Lexer (`src/lexer.rs`)**: Breaks down the source code (`.aur`) into meaningful parts (tokens).
2.  **Parser (`src/parser.rs`)**: Processes tokens to create an Abstract Syntax Tree (AST).
3.  **Compiler (`src/compiler.rs`)**: Traverses the AST to generate optimized **LLVM IR** code.

---

## 🚀 Quick Start

**Requirements:** Rust, LLVM (Clang), Visual Studio Build Tools.

```powershell
# Clone the project
git clone https://github.com/username/kernel-base.git
cd kernel-base

# Build and Run (in Developer PowerShell)
cargo run
```

This command will read the `example.aur` file and produce a `test.exe` output (or equivalent).

---
*Developed with ❤️ using Rust & LLVM*
