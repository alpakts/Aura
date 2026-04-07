# 🛠️ Aura Kurulum ve Derleme Kılavuzu

Aura, yüksek performanslı bir 64-bit derleyicidir. `.aur` kaynak kodunu LLVM IR'ye çevirir ve ardından Clang kullanarak yerel bir çalışma zamanı (runtime) ile bağlar.

## 📋 Sistem Gereksinimleri

### Windows
1.  **Rust**: Derleyicinin kendisini derlemek için gereklidir.
2.  **LLVM (Clang)**: Son aşamada yerel bağlama (linking) için gereklidir.
    *   `winget install LLVM`
3.  **Visual Studio Build Tools**: Windows SDK ve C Çalışma Zamanı (Runtime) kütüphaneleri için gereklidir.

### Linux (Ubuntu/Debian)
1.  **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2.  **Clang & Build Essentials**:
    ```bash
    sudo apt update
    sudo apt install clang build-essential
    ```

### macOS
1.  **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2.  **Xcode Komut Satırı Araçları**:
    ```bash
    xcode-select --install
    ```

---

## 🚀 Derleyiciyi İnşa Etmek

`compiler` dizinine gidin ve ana derleyiciyi derleyin:

```bash
cd compiler
cargo build --release
```

Derlenen dosya `target/release/aura` (Windows'ta `aura.exe`) konumunda oluşacaktır.

---

## 🏗️ Aura Programlarını Derlemek

Aura ile derleme yapmak çok kolaydır. Sadece ana kaynak dosyanızı göstermeniz yeterlidir:

```bash
# Genel kullanım
aura build yol/dosya.aur

# Geliştirme modu (Doğrudan derle ve çalıştır)
cargo run -- ../src/main.aur
```

### Arka Planda Neler Oluyor?
1.  **Aura Lexer/Parser**: Kodunuzu tarar ve bir AST (Soyut Sözdizimi Ağacı) oluşturur.
2.  **Aura Compiler**: 64-bit **LLVM IR (.ll)** üretir.
3.  **Yerel Bağlayıcı (Clang)**: İşletim sisteminizi (Windows, Linux veya macOS) otomatik algılar, gerekli sistem kütüphanelerini bulur ve `dist/` klasöründe yerel bir çalıştırılabilir dosya üretir.

---

## 📂 Proje Yapısı

*   `src/`: Aura kaynak kodlarınız (`.aur` dosyaları).
*   `src/views/`: MVC motoru için HTML şablonları.
*   `src/dist/`: Derlenmiş yerel binary dosyaların bulunduğu klasör.
*   `compiler/src/`: Aura derleyicisinin Rust kaynak kodları.
*   `compiler/src/compiler/aura_runtime.c`: Aura'nın çekirdek C çalışma zamanı.
*   `compiler/src/compiler/aura_mvc.c`: MVC ve Şablon motoru uygulaması.

---

## 🔧 Sorun Giderme

### "clang bulunamadı"
LLVM'in sistem PATH değişkenine eklendiğinden emin olun. Linux/Mac'te bu genellikle otomatiktir. Windows'ta kurulumdan sonra terminali yeniden başlatmanız gerekebilir.

### "Link Error: unresolved external symbol" (Windows)
Aura, Visual Studio yollarını kayıt defteri (registry) üzerinden bulmaya çalışır. Hata alıyorsanız, Visual Studio Installer üzerinden "C++ ile masaüstü geliştirme" yükünün kurulu olduğundan emin olun.

### 32-bit Sistemlerde Çalıştırma
**Aura tamamen 64-bit tabanlıdır.** Tüm sayılar ve pointerlar `i64` kullanır. Şu an için 32-bit sistem desteği bulunmamaktadır.
