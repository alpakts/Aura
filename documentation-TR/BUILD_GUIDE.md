# 🛠️ Build ve Kurulum Rehberi

Bu proje, **Rust** ile yazılmış bir derleyicidir ve çıktı olarak **LLVM IR (.ll)** üretir. Bu IR kodunun çalıştırılabilir bir **Windows EXE** dosyasına dönüştürülmesi için **Clang** ve **Visual Studio Build Tools** gereklidir.

## 📋 Gereksinimler

1.  **Rust**: Derleyiciyi (`kernel-base`) derlemek için.
    *   [Rust İndir](https://www.rust-lang.org/tools/install)
2.  **LLVM (Clang)**: `.ll` dosyasını derlemek için.
    *   `winget install LLVM` komutuyla veya [LLVM Release Page](https://github.com/llvm/llvm-project/releases)'den Windows Installer ile kurabilirsiniz.
    *   Kurulumda **"Add LLVM to the system PATH for all users"** seçeneğini seçmeyi unutmayın!
3.  **Visual Studio 2022 (Build Tools)**: Linker (`link.exe`) ve C Runtime (`msvcrt.lib`) için.
    *   "Desktop development with C++" iş yükünü seçerek kurun.

---

## 🚀 Projeyi Derleme ve Çalıştırma

### Adım 1: Developer PowerShell'i Açın (ÖNEMLİ!) ⚠️

Standart Windows PowerShell veya CMD kullanırsanız, `printf` veya `msvcrt` gibi kütüphane hataları alırsınız.

Bunun yerine:
1.  Windows Başlat menüsünü açın.
2.  **"Developer PowerShell for VS 2022"** (veya 2019) aratın ve çalıştırın.
3.  Proje klasörüne gidin:
    ```powershell
    cd "Klasör\Yolunuz\kernel-base"
    ```

### Adım 2: Tek Komutla Çalıştır

Artık her şey hazır! Rust projesini çalıştırdığınızda, derleyicimiz otomatik olarak `.aa` kodunuzu okur, `.ll`'ye çevirir ve ardından `clang` ile `.exe` üretir.

```powershell
cargo run
```

Bu komut sırasıyla şunları yapar:
1.  Derleyiciyi derler (`src/main.rs` -> `kernel-base.exe`).
2.  `test.aa` dosyasını okur.
3.  `test.ll` dosyasını oluşturur.
4.  Otomatik olarak şu komutu çalıştırır:
    ```powershell
    clang test.ll -o test.exe -target i686-pc-windows-msvc -l legacy_stdio_definitions -l msvcrt
    ```
5.  Başarılı olursa `test.exe` dosyasını oluşturur.

### Adım 3: Programı Test Et

Oluşan çalıştırılabilir dosyayı çalıştırın:

```powershell
.\test.exe
```

---

## 🔧 Manuel Derleme (Otomasyon Çalışmazsa)

Eğer `cargo run` hata verirse ancak `test.ll` oluşmuşsa, manuel olarak EXE oluşturabilirsiniz:

**Developer PowerShell içinde:**
```powershell
clang test.ll -o test.exe -target i686-pc-windows-msvc -l legacy_stdio_definitions -l msvcrt
```

Ardından çalıştırın:
```powershell
.\test.exe
```

## ❓ Sık Karşılaşılan Hatalar

*   **`unable to find a Visual Studio installation`**: Normal PowerShell kullanıyorsunuzdur. Developer PowerShell kullanın.
*   **`unresolved external symbol _printf`**: Komutunuza `-l legacy_stdio_definitions -l msvcrt` kütüphanelerini eklediğinizden emin olun.
*   **`inttoptr` / `getelementptr` hataları**: `print_str` ile `print` fonksiyonlarını karıştırdınız veya `compiler.rs` içinde string literal işleme mantığı eski kalmış olabilir. (Bu proje kapsamında düzeltildi).

İyi kodlamalar! 💻
