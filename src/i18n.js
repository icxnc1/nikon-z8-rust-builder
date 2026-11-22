/* Nikon Z8 Rust Builder UI localization. All strings are embedded in the native .exe.
   Stable technical feature IDs and machine-readable API status fields are NOT translated. */
(() => {
"use strict";
const keys = [
"appTitle","subtitle","warning","resourcesTitle","resourcesButton",
"stockTitle","stockLabel","stockButton","featuresTitle",
"featureVideo","featureLog","featureStill","featurePrecapture",
"versionLabel","tagLabel","outputFolderLabel","buildButton","buildInfo",
"verifyTitle","candidateLabel","manifestLabel","verifyButton",
"resultsTitle","footer","busy","loading","languageLabel",
"stockPlaceholder","folderPlaceholder"
];
const translations = {
"ro": [
"Nikon Z8 · Constructor Rust","v0.5.1 · C3.11 → Experimental · Windows / Ubuntu WSL",
"Aplicație offline: nu scrie pe carduri și nu accesează camera. Verificarea fișierului nu certifică siguranța instalării.",
"1. Verificare resurse","Verifică instrumentele","2. Firmware Nikon original",
"Calea fișierului C3.11 (SHA verificat)","Verifică originalul","3. Funcții",
"Înregistrare video / N-RAW 3:2","Culoare Log / RED / Sony",
"Instrumente foto / SDR HEIF / Pixel Shift / Crop","Precaptură RAW",
"Identitate versiune","Etichetă","Folder nou, inexistent","Generează și verifică în Rust",
"Include verificarea integrității Nikon, 20 de unități, 15 rețete, SHA, CRC, reconstruire și recitire de control. Nu suprascrie fișiere existente.",
"4. Verificarea unui fișier existent","Fișier firmware experimental","Manifest JSON",
"Verifică independent pachetul","Rezultate",
"GPL-3.0-only · Sursă: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Nu este produs oficial Nikon.",
"În curs de execuție...","Se încarcă...","Limbă",
"Calea completă către Z_8_0311.bin","Folder de ieșire nou (nu trebuie să existe)"
],
"en": [
"Nikon Z8 · Rust Builder","v0.5.1 · C3.11 → Experimental · Windows / Ubuntu WSL",
"Offline application: does not write to memory cards or access the camera. File verification does not certify installation safety.",
"1. Resource verification","Check tools","2. Original Nikon firmware",
"C3.11 file path (SHA-verified)","Verify original","3. Features",
"Video recording / N-RAW 3:2","Log colour / RED / Sony",
"Photo tools / SDR HEIF / Pixel Shift / Crop","RAW pre-capture",
"Version identity","Tag","New, non-existing output folder","Build and verify with Rust",
"Checks Nikon integrity, 20 units, 15 recipes, SHA, CRC, rebuild and readback. Never overwrites an existing output.",
"4. Verify existing file","Experimental firmware file","JSON manifest",
"Independently verify package","Results",
"GPL-3.0-only · Upstream: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Not an official Nikon product.",
"Processing...","Loading...","Language",
"Full path to Z_8_0311.bin","New output directory (must not exist)"
],
"de": [
"Nikon Z8 · Rust-Patch-Builder","v0.5.1 · C3.11 → Experimentell · Windows / Ubuntu WSL",
"Offline-Anwendung: kein Zugriff auf Kamera oder Speicherkarten. Eine Dateiprüfung bestätigt nicht die Sicherheit einer Installation.",
"1. Ressourcenprüfung","Werkzeuge prüfen","2. Originale Nikon-Firmware",
"Dateipfad C3.11 (SHA-geprüft)","Original prüfen","3. Funktionen",
"Videoaufnahme / N-RAW 3:2","Log-Farbe / RED / Sony",
"Fotowerkzeuge / SDR HEIF / Pixel Shift / Crop","RAW-Vorauslösung",
"Versionskennung","Kennzeichen","Neuer, noch nicht vorhandener Ausgabeordner","Mit Rust erstellen und prüfen",
"Prüft Nikon-Integrität, 20 Einheiten, 15 Rezepte, SHA, CRC, Neuaufbau und erneutes Einlesen. Überschreibt keine vorhandenen Dateien.",
"4. Vorhandene Datei prüfen","Experimentelle Firmwaredatei","JSON-Manifest",
"Paket unabhängig prüfen","Ergebnisse",
"GPL-3.0-only · Quelle: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Kein offizielles Nikon-Produkt.",
"Verarbeitung läuft...","Wird geladen...","Sprache",
"Vollständiger Pfad zu Z_8_0311.bin","Neuer Ausgabeordner (darf noch nicht existieren)"
],
"ja": [
"Nikon Z8 · Rust パッチビルダー","v0.5.1 · C3.11 → 実験版 · Windows / Ubuntu WSL",
"オフラインアプリです。メモリーカードへの書き込みやカメラへのアクセスは行いません。ファイルの検証は導入時の安全性を保証しません。",
"1. リソースの検証","ツールを検証","2. Nikon 純正ファームウェア",
"C3.11 ファイルのパス（SHA 検証）","純正ファイルを検証","3. 機能",
"動画撮影 / N-RAW 3:2","Log カラー / RED / Sony",
"写真ツール / SDR HEIF / ピクセルシフト / クロップ","RAW プリキャプチャー",
"バージョン識別子","タグ","新規出力フォルダー（未作成）","Rust で生成して検証",
"Nikon の整合性、20 ユニット、15 レシピ、SHA、CRC、再構築と再読み込みを確認します。既存ファイルは上書きしません。",
"4. 既存ファイルの検証","実験版ファームウェアファイル","JSON マニフェスト",
"パッケージを独立検証","結果",
"GPL-3.0-only · 原作者: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Nikon 公式製品ではありません。",
"処理中...","読み込み中...","言語",
"Z_8_0311.bin のフルパス","未作成の新しい出力フォルダー"
],
"zh-CN": [
"Nikon Z8 · Rust 固件构建器","v0.5.1 · C3.11 → 实验版 · Windows / Ubuntu WSL",
"离线应用：不会写入存储卡，也不会访问相机。文件验证不代表刷写安全。",
"1. 资源检查","检查工具","2. 尼康原厂固件",
"C3.11 文件路径（SHA 校验）","验证原厂固件","3. 功能",
"视频录制 / N-RAW 3:2","Log 色彩 / RED / Sony",
"照片工具 / SDR HEIF / 像素位移 / 裁切","RAW 预拍摄",
"版本标识","标签","全新且不存在的输出文件夹","使用 Rust 构建并验证",
"检查尼康完整性、20 个单元、15 种配方、SHA、CRC、重建及回读。不覆盖现有文件。",
"4. 验证已有文件","实验固件文件","JSON 清单",
"独立验证固件包","结果",
"GPL-3.0-only · 上游项目：XinmiaoZhao/NikonFirmwarePatch v0.5.1 · 非尼康官方产品。",
"正在处理...","正在加载...","语言",
"Z_8_0311.bin 的完整路径","新的输出文件夹（必须尚不存在）"
],
"zh-TW": [
"Nikon Z8 · Rust 韌體建置工具","v0.5.1 · C3.11 → 實驗版 · Windows / Ubuntu WSL",
"離線應用程式：不會寫入記憶卡，也不會存取相機。檔案驗證不代表更新韌體安全。",
"1. 資源檢查","檢查工具","2. Nikon 原廠韌體",
"C3.11 檔案路徑（SHA 驗證）","驗證原廠韌體","3. 功能",
"影片錄製 / N-RAW 3:2","Log 色彩 / RED / Sony",
"相片工具 / SDR HEIF / 像素位移 / 裁切","RAW 預拍攝",
"版本識別","標籤","全新且尚不存在的輸出資料夾","使用 Rust 建置並驗證",
"驗證 Nikon 完整性、20 個單元、15 種配方、SHA、CRC、重建與回讀。不覆寫現有檔案。",
"4. 驗證現有檔案","實驗性韌體檔案","JSON 清單",
"獨立驗證韌體套件","結果",
"GPL-3.0-only · 上游專案：XinmiaoZhao/NikonFirmwarePatch v0.5.1 · 非 Nikon 官方產品。",
"處理中...","載入中...","語言",
"Z_8_0311.bin 的完整路徑","新的輸出資料夾（必須尚不存在）"
],
"es": [
"Nikon Z8 · Constructor Rust","v0.5.1 · C3.11 → Experimental · Windows / Ubuntu WSL",
"Aplicación sin conexión: no escribe en tarjetas ni accede a la cámara. Verificar el archivo no garantiza que su instalación sea segura.",
"1. Comprobación de recursos","Comprobar herramientas","2. Firmware original de Nikon",
"Ruta del archivo C3.11 (SHA verificado)","Verificar original","3. Funciones",
"Grabación de vídeo / N-RAW 3:2","Color Log / RED / Sony",
"Herramientas de foto / SDR HEIF / Pixel Shift / Recorte","Precaptura RAW",
"Identificador de versión","Etiqueta","Nueva carpeta de salida que aún no exista","Generar y verificar con Rust",
"Comprueba integridad Nikon, 20 unidades, 15 recetas, SHA, CRC, reconstrucción y relectura. No sobrescribe archivos existentes.",
"4. Verificar archivo existente","Archivo de firmware experimental","Manifiesto JSON",
"Verificar el paquete de forma independiente","Resultados",
"GPL-3.0-only · Proyecto original: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · No es un producto oficial de Nikon.",
"Procesando...","Cargando...","Idioma",
"Ruta completa a Z_8_0311.bin","Nueva carpeta de salida (no debe existir)"
],
"it": [
"Nikon Z8 · Generatore Rust","v0.5.1 · C3.11 → Sperimentale · Windows / Ubuntu WSL",
"Applicazione offline: non scrive sulle schede né accede alla fotocamera. La verifica del file non garantisce la sicurezza dell'installazione.",
"1. Verifica delle risorse","Controlla strumenti","2. Firmware Nikon originale",
"Percorso del file C3.11 (SHA verificato)","Verifica originale","3. Funzioni",
"Registrazione video / N-RAW 3:2","Colore Log / RED / Sony",
"Strumenti fotografici / SDR HEIF / Pixel Shift / Ritaglio","Pre-acquisizione RAW",
"Identificativo versione","Etichetta","Nuova cartella di output non ancora esistente","Genera e verifica con Rust",
"Controlla integrità Nikon, 20 unità, 15 ricette, SHA, CRC, ricostruzione e rilettura. Non sovrascrive file esistenti.",
"4. Verifica file esistente","File firmware sperimentale","Manifest JSON",
"Verifica il pacchetto in modo indipendente","Risultati",
"GPL-3.0-only · Progetto originale: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Non è un prodotto ufficiale Nikon.",
"Elaborazione...","Caricamento...","Lingua",
"Percorso completo di Z_8_0311.bin","Nuova cartella di output (non deve esistere)"
],
"fr": [
"Nikon Z8 · Générateur Rust","v0.5.1 · C3.11 → Expérimental · Windows / Ubuntu WSL",
"Application hors ligne : n'écrit pas sur les cartes et n'accède pas à l'appareil photo. La vérification ne garantit pas la sécurité de l'installation.",
"1. Vérification des ressources","Vérifier les outils","2. Micrologiciel Nikon d'origine",
"Chemin du fichier C3.11 (SHA vérifié)","Vérifier l'original","3. Fonctions",
"Enregistrement vidéo / N-RAW 3:2","Couleurs Log / RED / Sony",
"Outils photo / SDR HEIF / Pixel Shift / Recadrage","Précapture RAW",
"Identifiant de version","Étiquette","Nouveau dossier de sortie inexistant","Générer et vérifier avec Rust",
"Vérifie l'intégrité Nikon, 20 unités, 15 recettes, SHA, CRC, reconstruction et relecture. N'écrase aucun fichier existant.",
"4. Vérifier un fichier existant","Fichier de micrologiciel expérimental","Manifeste JSON",
"Vérifier le paquet indépendamment","Résultats",
"GPL-3.0-only · Projet source : XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Produit non officiel Nikon.",
"Traitement en cours...","Chargement...","Langue",
"Chemin complet de Z_8_0311.bin","Nouveau dossier de sortie (ne doit pas exister)"
],
"pl": [
"Nikon Z8 · Konstruktor Rust","v0.5.1 · C3.11 → Eksperymentalny · Windows / Ubuntu WSL",
"Aplikacja offline: nie zapisuje na kartach pamięci ani nie łączy się z aparatem. Weryfikacja pliku nie potwierdza bezpieczeństwa instalacji.",
"1. Kontrola zasobów","Sprawdź narzędzia","2. Oryginalny firmware Nikon",
"Ścieżka pliku C3.11 (zweryfikowany SHA)","Sprawdź oryginał","3. Funkcje",
"Nagrywanie wideo / N-RAW 3:2","Kolor Log / RED / Sony",
"Narzędzia fotograficzne / SDR HEIF / Pixel Shift / Kadrowanie","Wstępny zapis RAW",
"Identyfikator wersji","Znacznik","Nowy, nieistniejący folder wynikowy","Zbuduj i zweryfikuj w Rust",
"Sprawdza integralność Nikon, 20 jednostek, 15 receptur, SHA, CRC, przebudowę i ponowny odczyt. Nie nadpisuje istniejących plików.",
"4. Weryfikacja istniejącego pliku","Eksperymentalny plik firmware","Manifest JSON",
"Niezależnie sprawdź pakiet","Wyniki",
"GPL-3.0-only · Projekt źródłowy: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Produkt nieoficjalny firmy Nikon.",
"Przetwarzanie...","Ładowanie...","Język",
"Pełna ścieżka do Z_8_0311.bin","Nowy folder wynikowy (musi nie istnieć)"
],
"nb": [
"Nikon Z8 · Rust-bygger","v0.5.1 · C3.11 → Eksperimentell · Windows / Ubuntu WSL",
"Frakoblet app: skriver ikke til minnekort eller kommuniserer med kameraet. Filkontroll garanterer ikke at installasjonen er trygg.",
"1. Ressurssjekk","Kontroller verktøy","2. Original Nikon-fastvare",
"Filbane for C3.11 (SHA-kontrollert)","Kontroller originalfil","3. Funksjoner",
"Videoopptak / N-RAW 3:2","Log-farge / RED / Sony",
"Fotoverktøy / SDR HEIF / Pixel Shift / Beskjæring","RAW-forhåndsopptak",
"Versjonsidentitet","Tagg","Ny uopprettet utdatamappe","Bygg og kontroller med Rust",
"Kontrollerer Nikon-integritet, 20 enheter, 15 oppskrifter, SHA, CRC, ombygging og ny gjennomlesing. Overskriver aldri eksisterende filer.",
"4. Kontroller eksisterende fil","Eksperimentell fastvarefil","JSON-manifest",
"Kontroller pakken uavhengig","Resultater",
"GPL-3.0-only · Oppstrøms: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Ikke et offisielt Nikon-produkt.",
"Behandler...","Laster...","Språk",
"Full bane til Z_8_0311.bin","Ny utdatamappe (må ikke finnes)"
],
"sv": [
"Nikon Z8 · Rust-byggare","v0.5.1 · C3.11 → Experimentell · Windows / Ubuntu WSL",
"Offlineprogram: skriver inte till minneskort eller kommunicerar med kameran. Filverifiering garanterar inte säker installation.",
"1. Resurskontroll","Kontrollera verktyg","2. Nikons originalfirmware",
"Filsökväg till C3.11 (SHA-verifierad)","Verifiera originalet","3. Funktioner",
"Videoinspelning / N-RAW 3:2","Log-färg / RED / Sony",
"Fotoverktyg / SDR HEIF / Pixel Shift / Beskärning","RAW-förinspelning",
"Versionsidentitet","Tagg","Ny utdatakatalog som inte finns","Bygg och verifiera med Rust",
"Kontrollerar Nikon-integritet, 20 enheter, 15 recept, SHA, CRC, ombyggnad och återläsning. Skriver aldrig över befintliga filer.",
"4. Verifiera befintlig fil","Experimentell firmwarefil","JSON-manifest",
"Verifiera paketet oberoende","Resultat",
"GPL-3.0-only · Ursprung: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Inte en officiell Nikon-produkt.",
"Bearbetar...","Läser in...","Språk",
"Fullständig sökväg till Z_8_0311.bin","Ny utdatakatalog (får inte finnas)"
],
"pt": [
"Nikon Z8 · Criador Rust","v0.5.1 · C3.11 → Experimental · Windows / Ubuntu WSL",
"Aplicação offline: não escreve em cartões de memória nem acede à câmara. A verificação do ficheiro não garante a segurança da instalação.",
"1. Verificação de recursos","Verificar ferramentas","2. Firmware Nikon original",
"Caminho do ficheiro C3.11 (SHA verificado)","Verificar original","3. Funcionalidades",
"Gravação de vídeo / N-RAW 3:2","Cor Log / RED / Sony",
"Ferramentas de fotografia / SDR HEIF / Pixel Shift / Recorte","Pré-captura RAW",
"Identificador da versão","Etiqueta","Nova pasta de saída, ainda inexistente","Compilar e verificar com Rust",
"Verifica a integridade Nikon, 20 unidades, 15 receitas, SHA, CRC, reconstrução e releitura. Não substitui ficheiros existentes.",
"4. Verificar ficheiro existente","Ficheiro de firmware experimental","Manifesto JSON",
"Verificar pacote independentemente","Resultados",
"GPL-3.0-only · Projeto original: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Não é um produto oficial da Nikon.",
"A processar...","A carregar...","Idioma",
"Caminho completo para Z_8_0311.bin","Nova pasta de saída (não pode existir)"
],
"ko": [
"Nikon Z8 · Rust 빌더","v0.5.1 · C3.11 → 실험용 · Windows / Ubuntu WSL",
"오프라인 앱: 메모리 카드에 쓰거나 카메라에 접근하지 않습니다. 파일 검증은 설치 안전성을 보증하지 않습니다.",
"1. 리소스 검증","도구 확인","2. Nikon 정품 펌웨어",
"C3.11 파일 경로(SHA 검증)","원본 확인","3. 기능",
"동영상 녹화 / N-RAW 3:2","Log 색상 / RED / Sony",
"사진 도구 / SDR HEIF / 픽셀 시프트 / 크롭","RAW 프리캡처",
"버전 식별자","태그","기존에 없는 새 출력 폴더","Rust로 빌드 및 검증",
"Nikon 무결성, 20개 유닛, 15개 레시피, SHA, CRC, 재구성 및 재읽기를 확인합니다. 기존 출력은 덮어쓰지 않습니다.",
"4. 기존 파일 검증","실험용 펌웨어 파일","JSON 매니페스트",
"패키지 독립 검증","결과",
"GPL-3.0-only · 원본 프로젝트: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Nikon 공식 제품이 아닙니다.",
"처리 중...","불러오는 중...","언어",
"Z_8_0311.bin의 전체 경로","새 출력 폴더(존재하지 않아야 함)"
],
"fi": [
"Nikon Z8 · Rust-rakennin","v0.5.1 · C3.11 → Kokeellinen · Windows / Ubuntu WSL",
"Offline-sovellus: ei kirjoita muistikorteille eikä käytä kameraa. Tiedoston tarkistus ei takaa asentamisen turvallisuutta.",
"1. Resurssien tarkistus","Tarkista työkalut","2. Nikonin alkuperäinen laiteohjelmisto",
"C3.11-tiedoston polku (SHA-tarkistettu)","Tarkista alkuperäinen","3. Toiminnot",
"Videotallennus / N-RAW 3:2","Log-värit / RED / Sony",
"Valokuvatyökalut / SDR HEIF / Pixel Shift / Rajaus","RAW-esitallennus",
"Version tunniste","Tunniste","Uusi, vielä olematon tuloskansio","Rakenna ja tarkista Rustilla",
"Tarkistaa Nikon-eheyden, 20 yksikköä, 15 reseptiä, SHA:n, CRC:n, uudelleenrakennuksen ja takaisinkirjoituksen tarkistuksen. Ei korvaa olemassa olevia tiedostoja.",
"4. Tarkista olemassa oleva tiedosto","Kokeellinen laiteohjelmistotiedosto","JSON-manifesti",
"Tarkista paketti riippumattomasti","Tulokset",
"GPL-3.0-only · Alkuperäinen projekti: XinmiaoZhao/NikonFirmwarePatch v0.5.1 · Ei Nikonin virallinen tuote.",
"Käsitellään...","Ladataan...","Kieli",
"Z_8_0311.bin-tiedoston koko polku","Uusi tuloskansio (ei saa olla olemassa)"
]
};
const targetSelectors = {
appTitle:"h1",subtitle:"body > p:not(.warning)",warning:"body > p.warning",
resourcesTitle:"section:nth-of-type(1) h2",
resourcesButton:"section:nth-of-type(1) button",
stockTitle:"section:nth-of-type(2) h2",
stockLabel:"section:nth-of-type(2) label",
stockButton:"section:nth-of-type(2) button",
featuresTitle:"section:nth-of-type(3) h2",
featureVideo:"section:nth-of-type(3) fieldset label:nth-of-type(1)",
featureLog:"section:nth-of-type(3) fieldset label:nth-of-type(2)",
featureStill:"section:nth-of-type(3) fieldset label:nth-of-type(3)",
featurePrecapture:"section:nth-of-type(3) fieldset label:nth-of-type(4)",
versionLabel:"section:nth-of-type(3) > label:nth-of-type(1)",
tagLabel:"section:nth-of-type(3) > label:nth-of-type(2)",
outputFolderLabel:"section:nth-of-type(3) > label:nth-of-type(3)",
buildButton:"#build",buildInfo:"section:nth-of-type(3) p small",
verifyTitle:"section:nth-of-type(4) h2",
candidateLabel:"section:nth-of-type(4) label:nth-of-type(1)",
manifestLabel:"section:nth-of-type(4) label:nth-of-type(2)",
verifyButton:"section:nth-of-type(4) button",
resultsTitle:"section:nth-of-type(5) h2",footer:"footer small",
languageLabel:"#language-label"
};
const supported=Object.keys(translations);
if(supported.some(c=>translations[c].length!==keys.length)){
  throw new Error("Nikon localization table is not complete");
}
const languageNames={
  ro:"Română",en:"English",de:"Deutsch",ja:"日本語",
  "zh-CN":"简体中文","zh-TW":"繁體中文",es:"Español",it:"Italiano",
  fr:"Français",pl:"Polski",nb:"Norsk bokmål",sv:"Svenska",pt:"Português",ko:"한국어",fi:"Suomi"
};
function selectedCode(raw){
  if(!raw)return null;
  const normalized=raw.replace(/_/g,"-").toLowerCase();
  if(normalized==="zh-tw"||normalized==="zh-hk"||normalized==="zh-mo"||normalized.startsWith("zh-hant"))return "zh-TW";
  if(normalized==="zh-cn"||normalized==="zh-sg"||normalized.startsWith("zh-hans")||normalized==="zh")return "zh-CN";
  if(normalized==="no"||normalized.startsWith("no-")||normalized==="nn"||normalized.startsWith("nn-"))return "nb";
  const short=normalized.split("-")[0];
  return supported.includes(short)?short:null;
}
function getPreference(){
  try {const saved=localStorage.getItem("z8-builder-language");if(supported.includes(saved))return saved;}catch(_){}
  return selectedCode(navigator.language)??"ro";
}
function putText(node,value){
  if(!node)return;
  // Input controls embedded in labels must never be recreated or detached.
  const textNode=Array.from(node.childNodes).find(child=>child.nodeType===Node.TEXT_NODE);
  if(textNode)textNode.textContent=value;
  else node.appendChild(document.createTextNode(value));
}
let current=getPreference();
function tr(key){
  const index=keys.indexOf(key);
  if(index<0)return key;
  return translations[current][index];
}
function update(code){
  if(!supported.includes(code))return;
  current=code;
  document.documentElement.lang=code;
  document.title="Nikon Z8 · Rust Builder — "+languageNames[code];
  const selection=document.getElementById("language");
  if(selection)selection.value=code;
  for(const [key,selector] of Object.entries(targetSelectors)){
    putText(document.querySelector(selector),tr(key));
  }
  const stock=document.getElementById("stock");
  const dest=document.getElementById("dest");
  if(stock)stock.placeholder=tr("stockPlaceholder");
  if(dest)dest.placeholder=tr("folderPlaceholder");
  const status=document.getElementById("state");
  if(status && (status.dataset.loading==="1" || status.textContent==="Se încarcă...")){
    status.dataset.loading="1";status.textContent=tr("loading");
  }
  try{localStorage.setItem("z8-builder-language",code);}catch(_){}
}
function init(){
  let area=document.getElementById("language-control");
  if(!area){
    area=document.createElement("div");
    area.id="language-control";
    area.className="language-control";
    const label=document.createElement("label");
    label.id="language-label";
    label.htmlFor="language";
    label.textContent="Limbă";
    const select=document.createElement("select");
    select.id="language";
    select.setAttribute("aria-label","Language");
    select.autocomplete="off";
    for(const code of supported){
      const option=document.createElement("option");
      option.value=code;
      option.textContent=languageNames[code];
      select.appendChild(option);
    }
    select.addEventListener("change",()=>update(select.value));
    area.appendChild(label);
    area.appendChild(select);
    document.body.insertBefore(area,document.body.firstChild);
  }
  update(current);
}
window.z8t=tr;
if(document.readyState==="loading")document.addEventListener("DOMContentLoaded",init,{once:true});
else init();
})();
