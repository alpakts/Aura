# 🛠️ Build and Installation Guide

This project is a compiler written in **Rust** that outputs **LLVM IR (.ll)** code. To convert this IR code into an executable **Windows EXE**, **Clang** and **Visual Studio Build Tools** are required.

## 📋 Requirements

1.  **Rust**: To compile the compiler source (`kernel-base`).
    *   [Install Rust](https://www.rust-lang.org/tools/install)
2.  **LLVM (Clang)**: To compile the `.ll` file.
    *   You can install via `winget install LLVM` or use the [LLVM Release Page](https://github.com/llvm/llvm-project/releases).
    *   Make sure to select **"Add LLVM to the system PATH for all users"** during installation!
3.  **Visual Studio 2022 (Build Tools)**: Required for the Linker (`link.exe`) and C Runtime (`msvcrt.lib`).
    *   Install the "Desktop development with C++" workload.

---

## 🚀 Building and Running the Project

### Step 1: Open Developer PowerShell (IMPORTANT!) ⚠️

If you use standard Windows PowerShell or CMD, you will likely encounter library errors such as `printf` or `msvcrt` not found.

Instead:
1.  Open the Windows Start menu.
2.  Search for and run **"Developer PowerShell for VS 2022"** (or 2019).
3.  Navigate to the project directory:
    ```powershell
    cd "Path\To\kernel-base"
    ```

### Step 2: One-Command Run

Everything is ready! When you run the Rust project, our compiler will automatically read your `.aa` code, convert it to `.ll`, and then use `clang` to produce an `.exe`.

```powershell
cargo run
```

This command sequentially performs:
1.  Compiles the compiler itself (`src/main.rs` -> `kernel-base.exe`).
2.  Reads the `test.aa` file.
3.  Generates the `test.ll` file.
4.  Automatically executes:
    ```powershell
    clang test.ll -o test.exe -target i686-pc-windows-msvc -l legacy_stdio_definitions -l msvcrt
    ```
5.  If successful, generates `test.exe`.

### Step 3: Test the Program

Run the generated executable:

```powershell
.\test.exe
```

---

## 🔧 Manual Compilation (If Automation Fails)

If `cargo run` fails but `test.ll` is generated, you can manually build the EXE:

**Inside Developer PowerShell:**
```powershell
clang test.ll -o test.exe -target i686-pc-windows-msvc -l legacy_stdio_definitions -l msvcrt
```

Then run:
```powershell
.\test.exe
```

## ❓ Common Errors

*   **`unable to find a Visual Studio installation`**: You are using standard PowerShell. Use Developer PowerShell.
*   **`unresolved external symbol _printf`**: Ensure you added `-l legacy_stdio_definitions -l msvcrt` to your command.
*   **`inttoptr` / `getelementptr` errors**: You might have mixed up `print_str` and `print`, or the compiler logic for string literals is outdated. (This has been fixed in the current version).

Happy Coding! 💻
