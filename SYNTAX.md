# 📘 .aa Programlama Dili Sözdizimi (Syntax) Kılavuzu

Bu belge, **.aa** programlama dilinin mevcut sürümünde desteklenen kuralları ve kullanımı açıklar.

## 1. Değişken Tanımlama
Değişkenler `var` anahtar kelimesi ile tanımlanır.
Değişken isimleri harf veya alt çizgi ile başlayabilir.

```aa
var x = 10
var y = 20
var sayi_bir = 50
```

## 2. Atama İşlemleri (Assignment)
Daha önce tanımlanmış bir değişkene yeni değer atanabilir.
Eğer değişken tanımlanmadan atama yapılırsa, derleyici otomatik olarak tanılayacaktır (geliştirme aşamasında).

```aa
x = 100
sonuc = 5 + 5
```

## 3. Matematiksel İşlemler
Standart dört işlem desteklenir. İşlem önceliği geçerlidir (Çarpma/Bölme önce yapılır).

* `+` : Toplama
* `-` : Çıkarma
* `*` : Çarpma
* `/` : Bölme

```aa
var a = 10 + 5    // 15
var b = a * 2     // 30
var c = (a + b) / 5
```

## 4. Ekrana Yazdırma
Sonuçları terminalde görmek için `print()` fonksiyonu kullanılır.

```aa
print(123)
print(x)
print(x + y)
```

## 5. Veri Tipleri
Derleyici şu an aşağıdaki veri tiplerini tam olarak desteklemektedir:

* **Integer (Tam Sayı)**: `0`, `10`, `-5` gibi tam sayılar.
* **String (Metin)**: `"Merhaba Dünya"` gibi çift tırnak içindeki metinler.

## 6. Koşullu İfadeler (If / Else)
Karar mekanizmaları için `if`, `else if` ve `else` blokları kullanılır. Bloklar `{` ve `}` süslü parantezleri ile tanımlanır.

**Desteklenen Karşılaştırma Operatörleri:**
* `==` : Eşittir
* `!=` : Eşit Değildir
* `<`  : Küçüktür
* `>`  : Büyüktür
* `<=` : Küçük Eşittir
* `>=` : Büyük Eşittir

```aa
var not = 75

if (not >= 50) {
    print("Gectiniz")
} else {
    print("Kaldiniz")
}
```

## 7. Tam Örnek Program

```aa
// Not Hesaplama ve Karar Verme
var vize = 60
var final = 70
var ortalama = (vize + final) / 2

print("Ortalama:")
print(ortalama)

if (ortalama > 50) {
    print("Basarili")
} else {
    print("Basarisiz")
}
```
