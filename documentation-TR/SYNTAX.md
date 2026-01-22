# 📘 .aa Programlama Dili Sözdizimi (Syntax) Kılavuzu

Bu belge, **.aa** programlama dilinin mevcut sürümünde desteklenen kuralları ve kullanımı açıklar.

## 1. Değişken Tanımlama
Değişkenler `var` anahtar kelimesi ile tanımlanır. Tür belirtmeye gerek yoktur (Type Inference).

```aa
var x = 10
var isim = "Ali"
var aktif = 1
```

## 2. Diziler (Arrays)
Köşeli parantez `[]` ile dizi tanımlanabilir ve indeks `[i]` ile erişilebilir.

```aa
var sayilar = [10, 20, 30]

print(sayilar[0]) // 10 yazar
var x = sayilar[1] + 5
```

## 3. Matematiksel İşlemler
Standart işlemler ve işlem önceliği desteklenir.

* `+`, `-`, `*`, `/`

```aa
var a = (10 + 5) * 2
```

## 4. Ekrana Yazdırma
İki farklı yazdırma fonksiyonu bulunur:

* `print(değer)`: Sayıları veya sayısal ifadeleri yazdırır.
* `print_str(metin)`: Metinleri (String Literal veya String Değişkeni) yazdırır.

```aa
print(100)               // Sayı basar
print_str("Merhaba")     // Metin basar

var mesaj = "Selam"
print_str(mesaj)         // Değişken içeriğini basar
```

## 5. Koşullu İfadeler (If / Else If / Else)
Klasik `if` yapısı desteklenir. Zincirleme `else if` yazılabilir.

```aa
var not = 75

if (not > 90) {
    print_str("Harika")
} else if (not > 50) {
    print_str("Gectiniz")
} else {
    print_str("Kaldiniz")
}
```

## 6. Döngüler (Loops)

### While Döngüsü
Koşul doğru olduğu sürece çalışır.

```aa
var i = 0
while (i < 5) {
    print(i)
    i = i + 1
}
```

### For Döngüsü
C tarzı `for` döngüsü desteklenir: `for (başlangıç; koşul; artış)`.

```aa
for (var k = 0; k < 10; k = k + 1) {
    print(k)
}
```

## 7. Fonksiyonlar
Fonksiyonlar `func` ile tanımlanır, parametre alabilir ve `return` ile değer döndürebilir.

```aa
func topla(x, y) {
    return x + y
}

var sonuc = topla(10, 20)
print(sonuc) // 30
```

### Fonksiyon İpuçları:
* Fonksiyon içinde tanımlanan değişkenler yereldir (Local Scope).
* String parametreleri fonksiyonlara iletilebilir (`print_str` ile yazdırılmalıdır).

```aa
func selamla(isim) {
    print_str("Merhaba")
    print_str(isim)
}

selamla("Ahmet")
```

## 8. Yorum Satırları
Tek satırlık yorumlar `//` ile başlar.

```aa
// Bu bir yorum satırıdır
var x = 1 // Yanına da yazılabilir
```
