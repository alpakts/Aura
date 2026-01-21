# 🗺️ .aa Dili Derleyici Yol Haritası (Roadmap)

Bu dosya, kendi programlama dilimizi (.aa) geliştirirken izleyeceğimiz adımları takip etmemiz için oluşturulmuştur.

## ✅ Aşama 1: Dil Tasarımı ve Örnek Kod
- [x] Dilin sözdizimini (syntax) belirle (English keywords).
- [x] `test.aa` dosyasını ilk örnek kod ile oluştur.

## ✅ Aşama 2: Sözcüksel Analiz (Lexer)
- [x] Kaynak kodu okuma.
- [x] Karakterleri anlamlı parçalara (Token) ayırma.
- [x] Desteklenen kelimeler: `var`, `print`, `=`, `+`, `-`, `*`, `/`, `(`, `)`, sayılar ve isimler. (Rust ile uygulandı)

## ⏳ Aşama 3: Sözdizim Analizi (Parser) & AST
- [ ] Token listesini alıp mantıksal bir ağaç (Abstract Syntax Tree) yapısına dönüştürme.
- [ ] İşlem önceliği (çarpma/bölme önce gelir) kurallarını belirleme.

## ⏳ Aşama 4: LLVM Ara Kod Üretimi (IR Generation)
- [ ] AST ağacını gezerek LLVM IR (.ll) kodlarını üretme.
- [ ] Değişkenleri belleğe (stack) yerleştirme.
- [ ] Matematiksel fonksiyonları LLVM komutlarına çevirme.

## ⏳ Aşama 5: Makine Koduna Dönüştürme (Compilation)
- [ ] Üretilen `.ll` dosyasını Clang kullanarak `.exe` dosyasına çevirme.
- [ ] İlk `.aa` programımızı çalıştırma!

---
*Not: Her aşama bittiğinde buraya işaret koyacağız.*
