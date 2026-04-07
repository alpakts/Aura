# 📘 Aura (.aur) Programlama Dili Sözdizimi Kılavuzu

Bu belge, **Aura** programlama dilinin mevcut sürümünde desteklenen kuralları ve kullanımı açıklar. Aura, yerel MVC yeteneklerine sahip, yüksek performanslı ve 64-bit derlenen bir dildir.

## 1. Değişken Tanımlama
Değişkenler `var` anahtar kelimesi ile tanımlanır. Aura, ilk atama üzerinden otomatik tip belirleme (Type Inference) yapar.

```aura
var x = 10
var isim = "Alper"
var aktif = 1
```

## 2. Diziler (Arrays)
Diziler, köşeli parantez `[]` ile tanımlanan 64-bit yapılardır.

```aura
var sayilar = [10, 20, 30]
print(sayilar[0]) // 10 yazdırır

var kullanicilar = [u1, u2] // Sınıf örneklerinden oluşan dizi
```

## 3. Nesne Yönelimli Programlama (OOP)
Aura, sınıfları (class), alanları (field) ve metodları destekler. Tüm nesne örnekleri arka planda 64-bit pointer olarak işlenir.

```aura
class Kullanici {
    var id;
    var isim;

    func init(uId, uIsim) {
        this.id = uId;
        this.isim = uIsim;
    }

    func selamVer() {
        print_str("Merhaba, ben ");
        print_str(this.isim);
    }
}

var u = new Kullanici();
u.init(1, "Aura AI");
u.selamVer();
```

## 4. Web & MVC Motoru (Yerleşik)
Aura, web uygulamaları için yerleşik ve yüksek performanslı bir template motoruna sahiptir.

### Dosya Okuma
```aura
var tpl = read_file("views/index.html");
```

### Şablon İşleme (Rendering)
Tek bir nesneyi HTML şablonuna bağlamak için `render` kullanılır. `{model.alan_adi}` etiketlerini otomatik doldurur.

```aura
var html = render(tpl, kullaniciOrnegi);
```

### Liste İşleme (AuraView Engine)
Bir dizi nesneyi (array of objects) rekürsif olarak işlemek için `render_list` kullanılır. Belirli bir etiketi, öğe şablonu (item template) kullanarak doldurur.

```aura
// render_list(anaSablon, hedefEtiket, veriDizisi, ogeSablonu)
var listeHtml = render_list(tpl, "{kullanici_listesi}", kullanicilar, ogeTpl);
```

## 5. Yazdırma Komutları
* `print(deger)`: Sayıları (i64) yazdırır.
* `print_str(metin)`: Metinleri veya pointerları yazdırır.

## 6. Kontrol Akışı
Standart `if`, `else if`, `else`, `while` ve C-stili `for` döngüleri desteklenmektedir.

## 7. Mimari Özellikler
* **64-Bit:** Tüm tam sayılar ve pointerlar 64-bit (`i64`) genişliğindedir.
* **Doğrudan Derleme:** Aura kodu önce LLVM IR'ye, ardından Clang aracılığıyla doğrudan makine koduna dönüştürülür.
