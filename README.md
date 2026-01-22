# Kernel Base Compiler (.aa Language) 🦀

![Status](https://img.shields.io/badge/Status-Development-blue)
![Language](https://img.shields.io/badge/Written%20in-Rust-orange)
![Output](https://img.shields.io/badge/Output-LLVM%20IR-green)

**Kernel Base**, Rust ile geliştirilmiş, özel bir programlama dili (`.aa`) için tasarlanmş modern bir derleyicidir. Sözcüksel analiz (Lexer), sözdizimsel analiz (Parser) ve LLVM IR kod üretimi (Compiler) aşamalarını içerir. Üretilen çıktılar, Clang kullanılarak Windows üzerinde çalıştırılabilir `.exe` dosyalarına dönüştürülür.

**Kernel Base** is a modern compiler developed in Rust for a custom programming language (`.aa`). It encompasses Lexical Analysis, Parsing, and LLVM IR generation. The output is compiled into executable `.exe` files on Windows using Clang.

---

## ✨ Özellikler / Features

*   **Değişkenler & Tipler**: Otomatik tip çıkarımı (Type Inference) ile `int` ve `string` desteği.
*   **Diziler (Arrays)**: Dizi tanımlama ve indeks erişimi (`arr[0]`).
*   **Kontrol Yapıları**: `if`, `else if`, `else`, `while`, `for` döngüleri.
*   **Fonksiyonlar**: Parametre alabilen ve değer döndüren fonksiyonlar.
*   **Built-in Fonksiyonlar**: `print` (sayısal) ve `print_str` (metinsel) yazdırma fonksiyonları.
*   **Otomasyon**: Tek komutla (`cargo run`) derleme ve linking işlemi.

---

## 📚 Dökümantasyon / Documentation

Projenin detaylı kurulum, kullanım ve sözdizimi rehberlerine aşağıdaki klasörlerden ulaşabilirsiniz:

### 🇹🇷 Türkçe
*   **[Sözdizimi Rehberi (Syntax)](documentation-TR/SYNTAX.md)**: Dil kuralları ve örnekler.
*   **[Kurulum ve Derleme (Build Guide)](documentation-TR/BUILD_GUIDE.md)**: Windows üzerinde Clang ve VS Build Tools kurulumu.

### 🇬🇧 English
*   **[Syntax Guide](documentation-EN/SYNTAX.md)**: Language rules and examples.
*   **[Build & Installation](documentation-EN/BUILD_GUIDE.md)**: Setup guide for Windows, Clang, and VS Build Tools.

---

## 🏗 Mimari / Architecture

Derleyici 3 ana modülden oluşur / The compiler consists of 3 main modules:

1.  **Lexer (`src/lexer.rs`)**: Kaynak kodunu (`.aa`) anlamlı parçalara (token) ayırır.
2.  **Parser (`src/parser.rs`)**: Tokenları işleyerek Soyut Sözdizimi Ağacı (AST) oluşturur.
3.  **Compiler (`src/compiler.rs`)**: AST'yi dolaşarak optimize edilmiş **LLVM IR** kodu üretir.

---

## 🚀 Hızlı Başlangıç / Quick Start

**Gereksinimler:** Rust, LLVM (Clang), Visual Studio Build Tools.

```powershell
# Projeyi klonlayın
git clone https://github.com/username/kernel-base.git
cd kernel-base

# Derleyin ve Çalıştırın (Developer PowerShell içinde)
cargo run
```

Bu komut `example.aa` (veya `test.aa`) dosyasını okuyacak ve `test.exe` çıktısını üretecektir.

---
*Developed with ❤️ using Rust & LLVM*
