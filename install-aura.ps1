# 🚀 Aura Language Installer (Windows)
$InstallDir = "C:\Aura"
$BinName = "aura.exe"

Write-Host "`n--- Aura Language için Kurulum Başlıyor ---" -ForegroundColor Cyan

# 1. Klasör Hazırlığı
if (!(Test-Path $InstallDir)) {
    Write-Host "📂 Kurulum klasörü oluşturuluyor: $InstallDir"
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# 2. Derleme (Release Mode)
Write-Host "🔨 Aura derleyicisi en hızlı modda paketleniyor..." -ForegroundColor Yellow
cd "$PSScriptRoot\compiler"

# Rust kurulu mu kontrol et
if (!(Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Hata: Rust (cargo) kurulu değil. Lütfen önce Rust kurun." -ForegroundColor Red
    return
}

# Derleyicinin adını 'aura' olarak derle
& cargo build --release | Out-Null

$ReleasePath = "$PSScriptRoot\compiler\target\release\$BinName"

if (Test-Path $ReleasePath) {
    # 3. Kopyalama
    Write-Host "🚚 Dosyalar kopyalanıyor..." -ForegroundColor Green
    Copy-Item $ReleasePath -Destination "$InstallDir\$BinName" -Force
    Write-Host "✅ Derleyici kopyalandı: $InstallDir\$BinName"
} else {
    Write-Host "❌ Hata: Derleme başarısız oldu. Lütfen compiler klasörünü kontrol edin." -ForegroundColor Red
    return
}

# 4. PATH'e Ekleme (Kalıcı)
Write-Host "🔗 Aura komutu sisteme (PATH) ekleniyor..." -ForegroundColor Yellow
$ExistingPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($ExistingPath -notlike "*$InstallDir*") {
    $NewPath = "$ExistingPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "✅ Başarılı! Aura komutu sistem yoluna eklendi." -ForegroundColor Green
} else {
    Write-Host "ℹ️ Aura zaten sistem yolunda kayıtlı." -ForegroundColor Gray
}

# 5. Gereksinim Kontrolü (Clang)
if (!(Get-Command "clang" -ErrorAction SilentlyContinue)) {
    Write-Host "`n⚠️ UYARI: 'clang' komutu bulunamadı!" -ForegroundColor Red
    Write-Host "Aura kodu derleyip .exe üretebilmek için LLVM (Clang) yüklü olmalıdır."
    Write-Host "İndirmek için: https://llvm.org/builds/"
}

Write-Host "`n--- 🎉 KURULUM TAMAMLANDI! ---" -ForegroundColor Green
Write-Host "Lütfen yeni bir terminal açın ve şunu yazın:" -ForegroundColor Cyan
Write-Host "   aura build main.aur`n" -ForegroundColor White
