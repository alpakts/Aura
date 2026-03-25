; 🚀 Aura Programming Language - Inno Setup Script (PROGRAM FILES VERSION)
#define MyAppName "Aura Programming Language"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "Aura Engine"
#define MyAppURL "https://github.com/rust-lang/aura-lang"
#define MyAppExeName "aura.exe"

[Setup]
AppId={{84BB1A5B-5E4B-48BB-B85A-066A83293357}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
; Tekrar Program Files (x86 veya x64) dizinine kuruyoruz
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputDir=.
OutputBaseFilename=Aura-Setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
; Program Files için yönetici izni şarttır
PrivilegesRequired=admin
; KRITIK: Ortam değişkeni değişikliğini Windows'a anında bildirir
ChangesEnvironment=yes

[Languages]
Name: "turkish"; MessagesFile: "compiler:Languages\Turkish.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "Aura-SDK\bin\*"; DestDir: "{app}\bin"; Flags: ignoreversion recursesubdirs
Source: "Aura-SDK\lib\*"; DestDir: "{app}\lib"; Flags: ignoreversion recursesubdirs
; VS Code Eklentisi klasörünü kopyalıyoruz
Source: "Aura-SDK\vscode-extension\*"; DestDir: "{app}\vscode-extension"; Flags: ignoreversion recursesubdirs

[Tasks]
Name: "vsc_ext"; Description: "Aura VS Code Eklentisini Yükle (Syntax Highlighting)"; GroupDescription: "Ek Seçenekler:"

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\bin\{#MyAppExeName}"

[Registry]
; PATH'e otomatik ekleme işlemi (User Path üzerinden devam ediyoruz)
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}\bin"; Check: NeedsAddPath

[Code]
function NeedsAddPath(): Boolean;
var
  OldPath: String;
begin
  if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath) then
  begin
    // Küçük/Büyük harf duyarsız kontrol
    Result := Pos(Uppercase(ExpandConstant('{app}\bin')), Uppercase(OldPath)) = 0;
  end
  else
    Result := True;
end;

[Run]
; Kurulum bitince versiyon basarak test et
Filename: "powershell.exe"; Parameters: "-noprofile -command ""& {{ aura version ; Read-Host 'Kurulum Başarılı! Çıkış için bir tuşa basın...' }}"""; Flags: postinstall skipifsilent
; VS Code eklentisini kur (Eğer task seçiliyse)
Filename: "code.cmd"; Parameters: "--install-extension ""{app}\vscode-extension"""; StatusMsg: "VS Code eklentisi kuruluyor..."; Tasks: vsc_ext; Flags: runhidden
