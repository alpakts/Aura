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
Şu an için derleyici **Tam Sayı (Integer)** veri tipini tam olarak desteklemektedir.

* **Integer**: `0`, `10`, `-5` gibi tam sayılar.
* **String**: `"Merhaba"` (Sözdizimsel olarak tanınır ancak henüz makine koduna derlenmez - Geliştiriliyor 🚧).

## 💡 Tam Örnek Program

```aa
// İki sayıyı toplayıp yazdıran program
var not1 = 50
var not2 = 80
ortalama = (not1 + not2) / 2
print(ortalama)
```
