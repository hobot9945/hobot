# gen_dt_xml.ps1
#
# Алгоритм (в общих чертах)
# -------------------------
# 1) Вход: читаем stage 2.0 артефакт `dt_fields.md` (UTF-8).
# 2) Парсинг: извлекаем строки Markdown-таблиц формата
#      | NN | field | value | ... |
#    и строим словарь `map[field] = value`.
#    Примеры ключей:
#      - sender.name
#      - goods[1].tnved_code
#      - goods[1].tovg[3].description
#      - goods[2].g44_docs[14].doc_date
# 3) Генерация: создаём `System.Xml.XmlWriter` с кодировкой windows-1251 и
#    последовательно пишем:
#      - верхний уровень `<AltaGTD>` + whitelist-теги из dt_xml_schema.md
#      - блоки NAME для граф 8/9/14 и G_54P/N2
#      - массив товаров: каждый goods[i] -> `<BLOCK>...</BLOCK>`
#        внутри BLOCK: G_31 + TXT + TOVG + G44 (по данным из map)
# 4) Проверка: читаем получившийся dt.xml и парсим как XML, считаем количества
#    `<BLOCK>`, `<TOVG>`, `<TXT>`, `<G44>`.
#
# Ключевые допущения/ограничения
# ------------------------------
# * Парсер строк таблицы делает split по '|'. Предполагается, что значение ячейки
#   `value` не содержит символ '|'.
# * TXT/TEXT сейчас пишется как обычный текст. Если потребуется вставлять именно
#   сущности `&#13;&#10;`, это делается отдельной доработкой (WriteRaw/постобработка).
#
#
# Параметры
# ---------
# -CaseName: имя кейса (имя папки в stage_2.0_result / stage_2.1_result)
# -HobotRoot: корень проекта hobot

param(
  [Parameter(Mandatory=$true)][string]$CaseName,
  [Parameter(Mandatory=$true)][string]$HobotRoot
)

$ErrorActionPreference = 'Stop'

$inPath  = Join-Path (Join-Path (Join-Path $HobotRoot 'alta\stage_2.0_result') $CaseName) 'dt_fields.md'
$outDir  = Join-Path (Join-Path (Join-Path $HobotRoot 'alta\stage_2.1_result') $CaseName) ''
$outPath = Join-Path $outDir 'dt.xml'

if (-not (Test-Path -LiteralPath $inPath)) {
  throw "Input not found: $inPath"
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null

# --------------------------------------------------------------
# 1) Parse dt_fields.md -> map[field]=value
# --------------------------------------------------------------
#
# На этом шаге мы превращаем "табличный Markdown" в плоский словарь (Hashtable):
#   map[<field_path>] = <value>
#
# Где <field_path> — это строковый ключ из второй колонки таблицы.
# Включая индексы массивов/подмассивов, например:
#   goods[1].tovg[3].description
#   goods[2].g44_docs[14].doc_date
#
# Такой плоский словарь дальше удобно адресовать конкатенацией строковых префиксов.

$lines = Get-Content -LiteralPath $inPath -Encoding UTF8

# Split-Row(row) -> string[]
# --------------------------
# Назначение:
#   Разобрать одну строку Markdown-таблицы вида:
#     | ... | ... | ... |
#   в массив ячеек (без крайних '|').
#
# Вход:
#   row: строка (одна строка файла dt_fields.md)
#
# Выход:
#   Массив строк-ячеек. Если строка не начинается с '|', возвращает пустой массив.
#
# Важное допущение:
#   split делается по символу '|'. Если в значении ячейки встречается '|',
#   разбор будет некорректным.
function Split-Row([string]$row) {
  $r = $row.Trim()
  if (-not $r.StartsWith('|')) { return @() }
  $parts = $r.Split('|')
  $cells = @()
  for ($i=1; $i -lt $parts.Length-1; $i++) {
    $cells += $parts[$i].Trim()
  }
  return $cells
}

# Собираем словарь map[field]=value по всем строкам таблиц.
# Берём только строки, где первая колонка — двузначный номер "NN".
$map = @{}
foreach ($ln in $lines) {
  if ($ln -match '^\|\s*\d{2}\s*\|') {
    $cells = Split-Row $ln
    # cells[0] = num, cells[1] = field, cells[2] = value
    if ($cells.Count -ge 3) {
      $field = $cells[1]
      $value = $cells[2]
      $map[$field] = $value
    }
  }
}

# V(key) -> string
# ---------------
# Назначение:
#   Безопасно получить значение из map.
#   Возвращает пустую строку, если ключ отсутствует.
function V([string]$k) {
  if ($map.ContainsKey($k)) { return $map[$k] }
  return ''
}

# NormDate(date_string) -> string
# ------------------------------
# Назначение:
#   Нормализовать даты к формату YYYY-MM-DD для XML.
#
# Правила:
#   - Если вход в формате dd.MM.yyyy -> конвертируем.
#   - Иначе возвращаем как есть.
function NormDate([string]$d) {
  if ([string]::IsNullOrWhiteSpace($d)) { return '' }
  if ($d -match '^\d{2}\.\d{2}\.\d{4}$') {
    return ([datetime]::ParseExact($d,'dd.MM.yyyy',$null)).ToString('yyyy-MM-dd')
  }
  return $d
}

# --------------------------------------------------------------
# 2) Generate XML via XmlWriter (windows-1251)
# --------------------------------------------------------------
$enc = [System.Text.Encoding]::GetEncoding('windows-1251')
$settings = New-Object System.Xml.XmlWriterSettings
$settings.Encoding = $enc
$settings.Indent = $true
$settings.OmitXmlDeclaration = $false

$w = [System.Xml.XmlWriter]::Create($outPath, $settings)
$w.WriteStartDocument()
$w.WriteStartElement('AltaGTD')

# WriteEl(tagName, value)
# -----------------------
# Назначение:
#   Утилита для записи простого скалярного тега <tagName>value</tagName>.
#   Пустые/пробельные значения пропускаются (тег не пишется).
#
# Примечание:
#   XmlWriter сам делает XML-экранирование текста (&, <, >, кавычки).
function WriteEl([string]$name, [string]$value) {
  if (-not [string]::IsNullOrWhiteSpace($value)) {
    $w.WriteElementString($name, $value)
  }
}
# --- Top-level whitelist (из dt_xml_schema.md) ---
WriteEl 'G_1_1'  (V 'declaration.direction')
WriteEl 'G_1_2'  (V 'declaration.procedure')
WriteEl 'G_1_31' (V 'declaration.form')

WriteEl 'G_2_50'  (V 'sender.country_name')
WriteEl 'G_2_7'   (V 'sender.country_code')
WriteEl 'G_2_NAM' (V 'sender.name')
WriteEl 'G_2_SUB' (V 'sender.region')
WriteEl 'G_2_CIT' (V 'sender.city')
WriteEl 'G_2_STR' (V 'sender.street')

WriteEl 'G_5_1' (V 'shipment.total_goods_number')
WriteEl 'G_6_0' (V 'shipment.packages_flag')
WriteEl 'G_6_1' (V 'shipment.total_packages')

# G_8
WriteEl 'G_8_1'     (V 'consignee.ogrn')
WriteEl 'G_8_50'    (V 'consignee.country_name')
WriteEl 'G_8_6'     (V 'consignee.inn_kpp')
WriteEl 'G_8_7'     (V 'consignee.country_code')
WriteEl 'G_8_NAM'   (V 'consignee.name')
WriteEl 'G_8_POS'   (V 'consignee.postcode')
WriteEl 'G_8_SUB'   (V 'consignee.region')
WriteEl 'G_8_CIT'   (V 'consignee.city')
WriteEl 'G_8_STR'   (V 'consignee.street')
WriteEl 'G_8_BLD'   (V 'consignee.building')
WriteEl 'G_8_ROM'   (V 'consignee.room')
WriteEl 'G_8_SM14'  (V 'consignee.same_as_declarant')
WriteEl 'G_8_PHONE' (V 'consignee.phone')
WriteEl 'G_8_EMAIL' (V 'consignee.email')
$nd = V 'consignee.name_display'
if (-not [string]::IsNullOrWhiteSpace($nd)) {
  $w.WriteStartElement('G_8'); $w.WriteElementString('NAME',$nd); $w.WriteEndElement()
}

# G_9
WriteEl 'G_9_1'     (V 'financial.ogrn')
WriteEl 'G_9_4'     (V 'financial.inn_kpp')
WriteEl 'G_9_NAM'   (V 'financial.name')
WriteEl 'G_9_CC'    (V 'financial.country_code')
WriteEl 'G_9_CN'    (V 'financial.country_name')
WriteEl 'G_9_POS'   (V 'financial.postcode')
WriteEl 'G_9_SUB'   (V 'financial.region')
WriteEl 'G_9_CIT'   (V 'financial.city')
WriteEl 'G_9_STR'   (V 'financial.street')
WriteEl 'G_9_BLD'   (V 'financial.building')
WriteEl 'G_9_ROM'   (V 'financial.room')
WriteEl 'G_9_SM14'  (V 'financial.same_as_declarant')
WriteEl 'G_9_7'     (V 'financial.country_code_alt')
WriteEl 'G_9_PHONE' (V 'financial.phone')
WriteEl 'G_9_EMAIL' (V 'financial.email')
$nd = V 'financial.name_display'
if (-not [string]::IsNullOrWhiteSpace($nd)) {
  $w.WriteStartElement('G_9'); $w.WriteElementString('NAME',$nd); $w.WriteEndElement()
}

WriteEl 'G_11_1' (V 'shipment.trade_country_code')

# G_14
WriteEl 'G_14_1'     (V 'declarant.ogrn')
WriteEl 'G_14_4'     (V 'declarant.inn_kpp')
WriteEl 'G_14_NAM'   (V 'declarant.name')
WriteEl 'G_14_CC'    (V 'declarant.country_code')
WriteEl 'G_14_CN'    (V 'declarant.country_name')
WriteEl 'G_14_POS'   (V 'declarant.postcode')
WriteEl 'G_14_SUB'   (V 'declarant.region')
WriteEl 'G_14_CIT'   (V 'declarant.city')
WriteEl 'G_14_STR'   (V 'declarant.street')
WriteEl 'G_14_BLD'   (V 'declarant.building')
WriteEl 'G_14_ROM'   (V 'declarant.room')
WriteEl 'G_14_PHONE' (V 'declarant.phone')
WriteEl 'G_14_EMAIL' (V 'declarant.email')
$nd = V 'declarant.name_display'
if (-not [string]::IsNullOrWhiteSpace($nd)) {
  $w.WriteStartElement('G_14'); $w.WriteElementString('NAME',$nd); $w.WriteEndElement()
}

# 15-17
WriteEl 'G_15_1'  (V 'shipment.dispatch_country_name')
WriteEl 'G_15A_1' (V 'shipment.dispatch_country_code')
WriteEl 'G_16_1'  (V 'shipment.origin_country_name')
WriteEl 'G_16_2'  (V 'shipment.origin_country_code')
WriteEl 'G_17_1'  (V 'shipment.destination_country_name')
WriteEl 'G_17A_1' (V 'shipment.destination_country_code')

# 18-21
WriteEl 'G_18_0' (V 'transport.vehicles_count')
WriteEl 'G_18'   (V 'transport.identification')
WriteEl 'G_18_2' (V 'transport.registration_country_code')
WriteEl 'G_19_1' (V 'transport.container_flag')
WriteEl 'G_21_0' (V 'transport.border_mode')

# 20
WriteEl 'G_20_20' (V 'delivery.terms_code')
WriteEl 'G_20_21' (V 'delivery.place_name')

# 22-23
WriteEl 'G_22_1' (V 'shipment.invoice_currency_numeric')
WriteEl 'G_22_2' (V 'shipment.invoice_amount')
WriteEl 'G_22_3' (V 'shipment.invoice_currency_alpha')
WriteEl 'G_23_1' (V 'shipment.currency_rate')
WriteEl 'G_23_2' (V 'shipment.currency_rate')

# 25-26
WriteEl 'G_25_1' (V 'transport.border_transport_code')
WriteEl 'G_26_1' (V 'transport.internal_transport_code')

# 29
WriteEl 'G_29_1' (V 'customs.border_code')
WriteEl 'G_29_2' (V 'customs.border_name')

# 30
WriteEl 'G_30_0'    (V 'location.type')
WriteEl 'G_30_10'   (V 'location.document_kind')
WriteEl 'G_30_1'    (V 'location.document_number')
WriteEl 'G_30_DATE' (V 'location.document_date')
WriteEl 'G_30_CC'   (V 'location.address.country_code')
WriteEl 'G_30_SUB'  (V 'location.address.region')
WriteEl 'G_30_CIT'  (V 'location.address.city')
WriteEl 'G_30_STR'  (V 'location.address.street')
WriteEl 'G_30_12'   (V 'location.customs_code')

# 54
WriteEl 'G_54_20'    (V 'representative.date')
WriteEl 'G_54_21'    (V 'representative.phone')
WriteEl 'G_54_EMAIL' (V 'representative.email')
WriteEl 'G_54_3'     (V 'representative.last_name')
WriteEl 'G_54_3NM'   (V 'representative.first_name')
WriteEl 'G_54_3MD'   (V 'representative.middle_name')
WriteEl 'G_54_4'     (V 'representative.authority_doc_name')
WriteEl 'G_54_5'     (V 'representative.authority_doc_number')
WriteEl 'G_54_60'    (V 'representative.authority_doc_date_from')
WriteEl 'G_54_61'    (V 'representative.authority_doc_date_to')
WriteEl 'G_54_7'     (V 'representative.position')
WriteEl 'G_54_8'     (V 'representative.passport_code')
WriteEl 'G_54_9'     (V 'representative.passport_name')
WriteEl 'G_54_100'   (V 'representative.passport_number')
WriteEl 'G_54_101'   (V 'representative.passport_date')
WriteEl 'G_54_12'    (V 'representative.passport_series')
WriteEl 'G_54_13'    (V 'representative.passport_issuer')

# --- BLOCKs ---
# Каждый goods[i] из dt_fields.md превращается в один <BLOCK>.
# Индексы в dt_fields.md начинаются с 1 (goods[1], goods[2], ...).
#
# goodsCount берём из shipment.total_goods_number. Если поле отсутствует/битое — fallback=2.
$goodsCount = 2
try { $goodsCount = [int](V 'shipment.total_goods_number') } catch { $goodsCount = 2 }
if ($goodsCount -lt 1) { $goodsCount = 2 }

for ($gi=1; $gi -le $goodsCount; $gi++) {
  # Префикс ключей для текущего товара.
  # Пример: "goods[1].tnved_code" -> V($pref+'tnved_code')
  $pref = 'goods['+$gi+'].'

  # --- BLOCK: базовые скалярные поля товара ---
  $w.WriteStartElement('BLOCK')
  WriteEl 'G_32_1' (V ($pref+'item_no'))
  WriteEl 'G_33_1' (V ($pref+'tnved_code'))
  WriteEl 'G_33_4' (V ($pref+'tnved.flag_1'))
  WriteEl 'G_33_5' (V ($pref+'tnved.flag_2'))
  WriteEl 'G_34_1' (V ($pref+'origin_country_code'))
  WriteEl 'G_35_1' (V ($pref+'gross_weight'))
  WriteEl 'G_36_2' (V ($pref+'preference'))
  WriteEl 'G_38_1' (V ($pref+'net_weight'))
  WriteEl 'G_42_1' (V ($pref+'invoice_cost'))
  WriteEl 'G_44' 'СМ.ДОПОЛНЕНИЕ'

  # --- G_31: описание товара + атрибуты Pref ---
  # Pref — это атрибуты Альты. Пишем их через WriteAttributeString,
  # чтобы XmlWriter гарантированно поставил кавычки и экранировал значение.
  $w.WriteStartElement('G_31')
  $w.WriteStartElement('NAME');  $w.WriteAttributeString('Pref','1-:');      $w.WriteString((V ($pref+'g31.name'))); $w.WriteEndElement()
  $w.WriteStartElement('FIRMA'); $w.WriteAttributeString('Pref','ПРОИЗВ.:'); $w.WriteString((V ($pref+'g31.manufacturer'))); $w.WriteEndElement()
  $w.WriteStartElement('TM');    $w.WriteAttributeString('Pref','(ТМ):');    $w.WriteString((V ($pref+'g31.trademark'))); $w.WriteEndElement()
  $w.WriteStartElement('PL');    $w.WriteAttributeString('Pref','2-');       $w.WriteEndElement()
  $pl = V ($pref+'places')
  if (-not [string]::IsNullOrWhiteSpace($pl)) { $w.WriteElementString('PLACE',$pl) }
  $w.WriteEndElement() # </G_31>

  # --- TXT: дополнение к графе 31 ---
  # dt_fields хранит txt[j].line_1 / txt[j].line_2.
  # Итерируемся по j=1..пока обе строки пустые (считаем, что дальше массива нет).
  # На каждый txt[j] по текущей схеме генерим 6 узлов <TXT>:
  #   line_1, пусто, line_2, пусто, пусто, пусто
  $tj=1
  while ($true) {
    $l1 = V ($pref+'txt['+$tj+'].line_1')
    $l2 = V ($pref+'txt['+$tj+'].line_2')
    if ([string]::IsNullOrWhiteSpace($l1) -and [string]::IsNullOrWhiteSpace($l2)) { break }

    foreach ($t in @($l1,'',$l2,'','','')) {
      $w.WriteStartElement('TXT')
      $w.WriteElementString('TEXT',$t)
      $w.WriteEndElement()
    }
    $tj++
  }

  # --- TOVG: табличное описание строк внутри товара ---
  # Итерируемся по j=1..пока нет tovq[j].line_no.
  $vj=1
  while ($true) {
    $ln = V ($pref+'tovg['+$vj+'].line_no')
    if ([string]::IsNullOrWhiteSpace($ln)) { break }

    $w.WriteStartElement('TOVG')
    WriteEl 'G32G' (V ($pref+'tovg['+$vj+'].line_no'))
    WriteEl 'G31_1' (V ($pref+'tovg['+$vj+'].description'))
    WriteEl 'G31_11' (V ($pref+'tovg['+$vj+'].manufacturer'))
    WriteEl 'G31_12' (V ($pref+'tovg['+$vj+'].trade_mark'))
    WriteEl 'G31_14' (V ($pref+'tovg['+$vj+'].goods_mark'))
    WriteEl 'G31_15_MOD' (V ($pref+'tovg['+$vj+'].model'))
    WriteEl 'KOLVO' (V ($pref+'tovg['+$vj+'].quantity'))
    WriteEl 'CODE_EDI' (V ($pref+'tovg['+$vj+'].unit_code'))
    WriteEl 'NAME_EDI' (V ($pref+'tovg['+$vj+'].unit_name'))
    WriteEl 'G31_35' (V ($pref+'tovg['+$vj+'].gross_weight'))
    WriteEl 'G31_38' (V ($pref+'tovg['+$vj+'].net_weight'))
    WriteEl 'G31_42' (V ($pref+'tovg['+$vj+'].invoice_cost'))
    WriteEl 'INVOICCOST' (V ($pref+'tovg['+$vj+'].invoice_cost'))
    $w.WriteEndElement() # </TOVG>

    $vj++
  }

  # --- G44: перечень документов (графа 44) ---
  # Итерируемся по k=1..пока нет doc_code.
  # Если у товара нет g44_docs (например, goods[2] в dt_fields.md),
  # дублируем список документов из goods[1].
  $g44Pref = $pref
  $firstDc = V ($pref+'g44_docs[1].doc_code')
  if ([string]::IsNullOrWhiteSpace($firstDc)) {
    $firstDc1 = V ('goods[1].g44_docs[1].doc_code')
    if (-not [string]::IsNullOrWhiteSpace($firstDc1)) {
      $g44Pref = 'goods[1].'
    }
  }

  $dk=1
  while ($true) {
    $dc = V ($g44Pref+'g44_docs['+$dk+'].doc_code')
    if ([string]::IsNullOrWhiteSpace($dc)) { break }

    $w.WriteStartElement('G44')
    $kc = V ($g44Pref+'g44_docs['+$dk+'].kind_code')
    if (-not [string]::IsNullOrWhiteSpace($kc)) { WriteEl 'G4403' $kc }
    WriteEl 'G441' $dc
    WriteEl 'G442' (V ($g44Pref+'g44_docs['+$dk+'].doc_number'))
    WriteEl 'G443' (V ($g44Pref+'g44_docs['+$dk+'].doc_date'))
    WriteEl 'G444' (V ($g44Pref+'g44_docs['+$dk+'].doc_name'))
    $w.WriteEndElement() # </G44>

    $dk++
  }

  $w.WriteEndElement() # </BLOCK>
}

$w.WriteEndElement()
$w.WriteEndDocument()
$w.Close()

# --------------------------------------------------------------
# 3) Verification (quick)
# --------------------------------------------------------------
$txt = [System.IO.File]::ReadAllText($outPath,$enc)
$x = [xml]$txt
$cntBlock = ([regex]::Matches($txt,'<BLOCK>').Count)
$cntTovg  = ([regex]::Matches($txt,'<TOVG>').Count)
$cntTxt   = ([regex]::Matches($txt,'<TXT>').Count)
$cntG44   = ([regex]::Matches($txt,'<G44>').Count)

Write-Host ('OK: dt.xml generated: ' + $outPath)
Write-Host ('root=' + $x.DocumentElement.Name)
Write-Host ('counts: BLOCK=' + $cntBlock + ' TOVG=' + $cntTovg + ' TXT=' + $cntTxt + ' G44=' + $cntG44)
