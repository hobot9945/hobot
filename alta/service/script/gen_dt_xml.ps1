<#
.SYNOPSIS
    Генератор XML-файла декларации (dt.xml) для программы «Альта-ГТД» из Markdown-таблиц.

.DESCRIPTION
    Алгоритм работы:
    1. Чтение: Скрипт загружает файл dt_fields.md (результат этапа 2.0).
    2. Парсинг: Разбирает Markdown-таблицы и создает хеш-таблицу (словарь) `$map`.
       Ключ — это имя поля (UQI), значение — объект, содержащий само значение и его статус.
    3. Валидация и Извлечение:
       Все поля, запрашиваемые скриптом, считаются ОБЯЗАТЕЛЬНЫМИ.
       Если поле отсутствует в словаре или имеет статус 'pending' — фиксируется ошибка,
       но генерация не прерывается (стратегия best-effort).
    4. Генерация: С помощью System.Xml.XmlWriter формируется XML в кодировке windows-1251.
    5. Итог: Если были зафиксированы ошибки валидации, скрипт завершается с кодом 1.
#>

param(
    [Parameter(Mandatory=$true)]
    [string]$CaseName,   # Имя текущего кейса (например, "МоскитнаяСетка")

    [Parameter(Mandatory=$true)]
    [string]$HobotRoot   # Абсолютный путь к корню проекта
)

# Системная настройка: при критических ошибках (нет файла, нет прав) скрипт должен упасть.
$ErrorActionPreference = 'Stop'

# Формируем пути к файлам с помощью Join-Path (надежное склеивание путей со слешами)
$inPath  = Join-Path (Join-Path (Join-Path $HobotRoot 'alta\stage_2.0_result') $CaseName) 'dt_fields.md'
$outDir  = Join-Path (Join-Path (Join-Path $HobotRoot 'alta\stage_2.1_result') $CaseName) ''
$outPath = Join-Path $outDir 'dt.xml'

# Проверяем, существует ли исходный файл
if (-not (Test-Path -LiteralPath $inPath)) {
    throw "ОШИБКА: Входной файл не найден: $inPath"
}

# Создаем папку для результата, если ее нет. Out-Null подавляет лишний вывод в консоль.
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

# --------------------------------------------------------------
# КОЛЛЕКТОР ОШИБОК
# --------------------------------------------------------------
# Список (динамический массив) для накопления ошибок валидации.
$Errors = New-Object System.Collections.Generic.List[string]

function Add-Error([string]$msg) {
    $Errors.Add($msg) | Out-Null
}


# --------------------------------------------------------------
# ФУНКЦИИ ДОСТУПА И ВАЛИДАЦИИ
# --------------------------------------------------------------

<#
.SYNOPSIS
    Проверяет физическое наличие ключа в словаре.
.DESCRIPTION
    Используется ТОЛЬКО для определения границ массивов (циклы while).
    Возвращает $true, если поле есть в файле, даже если его статус 'pending'.
#>
function HasKey([string]$k) {
    return $map.ContainsKey($k)
}

<#
.SYNOPSIS
    Извлекает значение поля из словаря и проводит его валидацию.
.DESCRIPTION
    Логика:
    1. Если ключа нет -> пишет ошибку, возвращает пустую строку.
    2. Если ключ есть, но статус 'pending' -> пишет ошибку, возвращает пустую строку.
    3. Иначе -> возвращает реальное значение.
#>
function V([string]$k, [string]$context='') {

    $ctxStr = $k
    if (-not [string]::IsNullOrWhiteSpace($context)) {
        $ctxStr = "$k (для тега $context)"
    }

    if (-not $map.ContainsKey($k)) {
        Add-Error "Отсутствует обязательное поле: $ctxStr"
        return ''
    }

    $fieldData = $map[$k]

    if ($fieldData.Status -eq 'pending') {
        Add-Error "Поле не заполнено (статус pending): $ctxStr"
        return ''
    }

    return $fieldData.Value
}

<#
.SYNOPSIS
    Безопасно экранирует спецсимволы для XML, сохраняя числовые сущности.

.DESCRIPTION
    Стандартные средства .NET (такие как XmlWriter или SecurityElement)
    автоматически экранируют амперсанд ('&' -> '&amp;').
    Если в исходном тексте есть управляющие символы для «Альта-ГТД»
    (например, переносы строк '&#10;' или '&#13;'), они превратятся
    в текст '&amp;#10;', что сломает отображение.

    Данная функция:
    1. Надежно экранирует ВСЕ опасные символы (<, >, &, ", ') системным методом.
    2. С помощью регулярного выражения находит числовые сущности и "откатывает"
       двойное экранирование амперсанда только для них.
#>
function Get-SafeXml([string]$val) {

    if ([string]::IsNullOrEmpty($val)) { return '' }

    # 1. Надежное системное экранирование (<, >, &, ", ')
    $safe = [System.Security.SecurityElement]::Escape($val)

    # 2. Возвращаем амперсанды только для числовых сущностей (&#10; &#13; и т.д.)
    return [regex]::Replace($safe, '&amp;#(\d+);', '&#$1;')
}

<#
.SYNOPSIS
    Записывает XML-элемент (тег) с поддержкой управляющих символов.

.DESCRIPTION
    Использует метод WriteRaw вместо стандартного WriteElementString.
    Это необходимо, чтобы XmlWriter не выполнял автоматическое экранирование
    повторно. Безопасность данных обеспечивается предварительным вызовом
    функции Get-SafeXml, которая корректно обрабатывает как спецсимволы XML,
    так и сущности переноса строк (&#10;, &#13;).
#>
function WriteEl([string]$name, [string]$value) {
    $w.WriteStartElement($name)
    $w.WriteRaw((Get-SafeXml $value))
    $w.WriteEndElement()
}

<#
.SYNOPSIS
    Комбинированная функция для сокращения кода: извлекает значение и сразу пишет тег.
.DESCRIPTION
    $name - Имя тега (используется и как контекст ошибки).
    $key  - Имя поля (UQI) в dt_fields.md.
#>
function WV([string]$name, [string]$key) {
    WriteEl $name (V $key $name)
}

# --------------------------------------------------------------
# ШАГ 1: ПАРСИНГ Markdown -> Словарь
# --------------------------------------------------------------

$lines = Get-Content -LiteralPath $inPath -Encoding UTF8

# Функция для разбивки строки Markdown-таблицы на массив ячеек
function Split-Row([string]$row) {
    $r = $row.Trim()
    # Если строка не начинается с вертикальной черты, это не часть таблицы. Возвращаем пустой массив @()
    if (-not $r.StartsWith('|')) { return @() }

    # Разбиваем строку по символу '|'
    $parts = $r.Split('|')
    $cells = @()

    # Цикл со 2-го элемента до предпоследнего (игнорируем пустоты до первой и после последней '|').
    for ($i = 1; $i -lt $parts.Length - 1; $i++) {
        $cells += $parts[$i].Trim()
    }
    return $cells
}

# Инициализируем пустую хеш-таблицу (словарь)
$map = @{}

foreach ($ln in $lines) {
    # Ищем строки данных: начинается с '|', затем возможны пробелы, затем две цифры и снова '|'
    if ($ln -match '^\|\s*\d{2}\s*\|') {
        $cells = Split-Row $ln

        # Ожидаем минимум 4 колонки: [0]num, [1]field, [2]value, [3]status
        if ($cells.Count -ge 4) {
            $field = $cells[1]

            # Сохраняем в словарь объект (структуру) с двумя свойствами: Value и Status.
            $map[$field] = [PSCustomObject]@{
                Value  = $cells[2]
                Status = $cells[3].ToLower() # Приводим статус к нижнему регистру для надежности
            }
        }
    }
}

# --------------------------------------------------------------
# ШАГ 3: ГЕНЕРАЦИЯ XML (XmlWriter)
# --------------------------------------------------------------

# Для российской таможни жестко требуется кодировка windows-1251
$enc = [System.Text.Encoding]::GetEncoding('windows-1251')

$settings = New-Object System.Xml.XmlWriterSettings
$settings.Encoding = $enc
$settings.Indent = $true
$settings.OmitXmlDeclaration = $false

$w = [System.Xml.XmlWriter]::Create($outPath, $settings)
$w.WriteStartDocument()
$w.WriteStartElement('AltaGTD')

# ==============================================================
# БЛОК: ШАПКА ДЕКЛАРАЦИИ (Общие поля)
# ==============================================================

WV 'G_1_1'  'declaration.direction'             # Графа 1: направление перемещения (ИМ/ЭК)
WV 'G_1_2'  'declaration.procedure'             # Графа 1: код таможенной процедуры (напр., 40)
WV 'G_1_31' 'declaration.form'                  # Графа 1: форма декларирования (ЭД)

# Отправитель (Графа 2)
WV 'G_2_50'  'sender.country_name'              # Графа 2: страна отправителя (наименование)
WV 'G_2_7'   'sender.country_code'              # Графа 2: страна отправителя (код alpha-2)
WV 'G_2_NAM' 'sender.name'                      # Графа 2: наименование отправителя
WV 'G_2_SUB' 'sender.region'                    # Графа 2: регион отправителя
WV 'G_2_CIT' 'sender.city'                      # Графа 2: город отправителя
WV 'G_2_STR' 'sender.street'                    # Графа 2: улица/дом отправителя

# Общие сведения о грузе (Графы 5, 6)
WV 'G_5_1' 'shipment.total_goods_number'        # Графа 5: всего наименований товаров
WV 'G_6_0' 'shipment.packages_flag'             # Графа 6: признак количества мест (обычно 0)
WV 'G_6_1' 'shipment.total_packages'            # Графа 6: всего грузовых мест по декларации

# Получатель (Графа 8)
WV 'G_8_1'     'consignee.ogrn'                 # Графа 8: ОГРН получателя
WV 'G_8_50'    'consignee.country_name'         # Графа 8: страна получателя (наименование)
WV 'G_8_6'     'consignee.inn_kpp'              # Графа 8: ИНН/КПП получателя
WV 'G_8_7'     'consignee.country_code'         # Графа 8: страна получателя (код alpha-2)
WV 'G_8_NAM'   'consignee.name'                 # Графа 8: наименование получателя
WV 'G_8_POS'   'consignee.postcode'             # Графа 8: почтовый индекс получателя
WV 'G_8_SUB'   'consignee.region'               # Графа 8: регион получателя
WV 'G_8_CIT'   'consignee.city'                 # Графа 8: город получателя
WV 'G_8_STR'   'consignee.street'               # Графа 8: улица получателя
WV 'G_8_BLD'   'consignee.building'             # Графа 8: дом/строение получателя
WV 'G_8_ROM'   'consignee.room'                 # Графа 8: помещение/офис получателя
WV 'G_8_SM14'  'consignee.same_as_declarant'    # Графа 8: признак совпадения получателя с декларантом
WV 'G_8_PHONE' 'consignee.phone'                # Графа 8: телефон получателя
WV 'G_8_EMAIL' 'consignee.email'                # Графа 8: email получателя

# Финансовое лицо / Контрактодержатель (Графа 9)
WV 'G_9_1'     'financial.ogrn'                 # Графа 9: ОГРН контрактодержателя
WV 'G_9_4'     'financial.inn_kpp'              # Графа 9: ИНН/КПП контрактодержателя
WV 'G_9_NAM'   'financial.name'                 # Графа 9: наименование контрактодержателя
WV 'G_9_CC'    'financial.country_code'         # Графа 9: страна контрактодержателя (код alpha-2)
WV 'G_9_CN'    'financial.country_name'         # Графа 9: страна контрактодержателя (наименование)
WV 'G_9_POS'   'financial.postcode'             # Графа 9: почтовый индекс контрактодержателя
WV 'G_9_SUB'   'financial.region'               # Графа 9: регион контрактодержателя
WV 'G_9_CIT'   'financial.city'                 # Графа 9: город контрактодержателя
WV 'G_9_STR'   'financial.street'               # Графа 9: улица контрактодержателя
WV 'G_9_BLD'   'financial.building'             # Графа 9: дом/строение контрактодержателя
WV 'G_9_ROM'   'financial.room'                 # Графа 9: помещение/офис контрактодержателя
WV 'G_9_SM14'  'financial.same_as_declarant'    # Графа 9: признак совпадения с декларантом
WV 'G_9_7'     'financial.country_code_alt'     # Графа 9: страна (альтернативный код)
WV 'G_9_PHONE' 'financial.phone'                # Графа 9: телефон контрактодержателя
WV 'G_9_EMAIL' 'financial.email'                # Графа 9: email контрактодержателя

# Торгующая страна (Графа 11)
WV 'G_11_1' 'shipment.trade_country_code'       # Графа 11: торгующая страна (код alpha-2)

# Декларант (Графа 14)
WV 'G_14_1'     'declarant.ogrn'                # Графа 14: ОГРН декларанта
WV 'G_14_4'     'declarant.inn_kpp'             # Графа 14: ИНН/КПП декларанта
WV 'G_14_NAM'   'declarant.name'                # Графа 14: наименование декларанта
WV 'G_14_CC'    'declarant.country_code'        # Графа 14: страна декларанта (код alpha-2)
WV 'G_14_CN'    'declarant.country_name'        # Графа 14: страна декларанта (наименование)
WV 'G_14_POS'   'declarant.postcode'            # Графа 14: почтовый индекс декларанта
WV 'G_14_SUB'   'declarant.region'              # Графа 14: регион декларанта
WV 'G_14_CIT'   'declarant.city'                # Графа 14: город декларанта
WV 'G_14_STR'   'declarant.street'              # Графа 14: улица декларанта
WV 'G_14_BLD'   'declarant.building'            # Графа 14: дом/строение декларанта
WV 'G_14_ROM'   'declarant.room'                # Графа 14: помещение/офис декларанта
WV 'G_14_PHONE' 'declarant.phone'               # Графа 14: телефон декларанта
WV 'G_14_EMAIL' 'declarant.email'               # Графа 14: email декларанта

# Страны (Графы 15-17)
WV 'G_15_1'  'shipment.dispatch_country_name'   # Графа 15: страна отправления (наименование)
WV 'G_15A_1' 'shipment.dispatch_country_code'   # Графа 15A: страна отправления (код alpha-2)
WV 'G_16_1'  'shipment.origin_country_name'     # Графа 16: страна происхождения (наименование)
WV 'G_16_2'  'shipment.origin_country_code'     # Графа 16: страна происхождения (код alpha-2)
WV 'G_17_1'  'shipment.destination_country_name'# Графа 17: страна назначения (наименование)
WV 'G_17A_1' 'shipment.destination_country_code'# Графа 17A: страна назначения (код alpha-2)

# Транспорт (Графы 18, 19, 21, 25, 26)
WV 'G_18_0' 'transport.vehicles_count'          # Графа 18: количество ТС при отправлении
WV 'G_18'   'transport.identification'          # Графа 18: идентификация ТС (гос. номера)
WV 'G_18_2' 'transport.registration_country_code'# Графа 18: код страны регистрации ТС
WV 'G_19_1' 'transport.container_flag'          # Графа 19: признак контейнерной перевозки
WV 'G_21_0' 'transport.border_mode'             # Графа 21: количество ТС на границе (либо признак)
WV 'G_25_1' 'transport.border_transport_code'   # Графа 25: вид транспорта на границе (код)
WV 'G_26_1' 'transport.internal_transport_code' # Графа 26: вид транспорта внутри страны (код)

# Условия поставки (Графа 20)
WV 'G_20_20' 'delivery.terms_code'              # Графа 20: код условий поставки по Инкотермс (напр., EXW)
WV 'G_20_21' 'delivery.place_name'              # Графа 20: географический пункт поставки

# Валюта и стоимость (Графы 22, 23)
WV 'G_22_1' 'shipment.invoice_currency_numeric' # Графа 22: цифровой код валюты счета (ISO)
WV 'G_22_2' 'shipment.invoice_amount'           # Графа 22: общая фактурная стоимость
WV 'G_22_3' 'shipment.invoice_currency_alpha'   # Графа 22: буквенный код валюты счета (ISO)
WV 'G_23_1' 'shipment.currency_rate'            # Графа 23: курс валюты
WV 'G_23_2' 'shipment.currency_rate'            # Графа 23: курс валюты (дубль для структуры Альты)

# Таможенные органы (Графа 29)
WV 'G_29_1' 'customs.border_code'               # Графа 29: код таможенного органа на границе
WV 'G_29_2' 'customs.border_name'               # Графа 29: наименование таможенного органа на границе

# Местоположение товаров (Графа 30)
WV 'G_30_0'    'location.type'                  # Графа 30: код типа места нахождения товаров (напр., 11)
WV 'G_30_10'   'location.document_kind'         # Графа 30: вид документа (дог. на СВХ и т.д.)
WV 'G_30_1'    'location.document_number'       # Графа 30: номер документа (лицензии СВХ или ДО-1)
WV 'G_30_DATE' 'location.document_date'         # Графа 30: дата документа СВХ
WV 'G_30_CC'   'location.address.country_code'  # Графа 30: страна СВХ (код alpha-2)
WV 'G_30_SUB'  'location.address.region'        # Графа 30: регион СВХ
WV 'G_30_CIT'  'location.address.city'          # Графа 30: город СВХ
WV 'G_30_STR'  'location.address.street'        # Графа 30: улица/дом СВХ
WV 'G_30_12'   'location.customs_code'          # Графа 30: код таможенного органа поста СВХ

# Подписант / Представитель (Графа 54)
WV 'G_54_20'    'representative.date'                   # Графа 54: дата заполнения декларации
WV 'G_54_21'    'representative.phone'                  # Графа 54: контактный телефон представителя
WV 'G_54_EMAIL' 'representative.email'                  # Графа 54: email представителя
WV 'G_54_3'     'representative.last_name'              # Графа 54: фамилия представителя
WV 'G_54_3NM'   'representative.first_name'             # Графа 54: имя представителя
WV 'G_54_3MD'   'representative.middle_name'            # Графа 54: отчество представителя
WV 'G_54_4'     'representative.authority_doc_name'     # Графа 54: наименование документа полномочий (Доверенность)
WV 'G_54_5'     'representative.authority_doc_number'   # Графа 54: номер документа полномочий
WV 'G_54_60'    'representative.authority_doc_date_from'# Графа 54: дата выдачи документа полномочий
WV 'G_54_61'    'representative.authority_doc_date_to'  # Графа 54: срок действия документа полномочий
WV 'G_54_7'     'representative.position'               # Графа 54: должность представителя
WV 'G_54_8'     'representative.passport_code'          # Графа 54: код документа, удостоверяющего личность (Паспорт)
WV 'G_54_9'     'representative.passport_name'          # Графа 54: наименование документа, удостоверяющего личность
WV 'G_54_100'   'representative.passport_number'        # Графа 54: номер паспорта
WV 'G_54_101'   'representative.passport_date'          # Графа 54: дата выдачи паспорта
WV 'G_54_12'    'representative.passport_series'        # Графа 54: серия паспорта
WV 'G_54_13'    'representative.passport_issuer'        # Графа 54: кем выдан паспорт

# ==============================================================
# БЛОК: ТОВАРЫ (Графы 31-44)
# ==============================================================

# Определяем общее количество товаров из поля. Fallback: если поля нет, считаем что товара 2.
$goodsCountStr = V 'shipment.total_goods_number'
$goodsCount = 2
if (-not [string]::IsNullOrWhiteSpace($goodsCountStr)) {
    try { $goodsCount = [int]$goodsCountStr } catch { $goodsCount = 2 }
}
if ($goodsCount -lt 1) { $goodsCount = 2 }

# Цикл по каждому товару
for ($gi = 1; $gi -le $goodsCount; $gi++) {

    # Базовый префикс для ключей текущего товара (например, "goods[1].")
    $pref = "goods[$gi]."

    $w.WriteStartElement('BLOCK')

    # Основные атрибуты товара
    WV 'G_32_1' ($pref+'item_no')
    WV 'G_33_1' ($pref+'tnved_code')
    WV 'G_33_4' ($pref+'tnved.flag_1')
    WV 'G_33_5' ($pref+'tnved.flag_2')
    WV 'G_34_1' ($pref+'origin_country_code')
    WV 'G_35_1' ($pref+'gross_weight')
    WV 'G_36_2' ($pref+'preference')
    WV 'G_38_1' ($pref+'net_weight')
    WV 'G_42_1' ($pref+'invoice_cost')
    WriteEl 'G_44' 'СМ.ДОПОЛНЕНИЕ'

    # --- Графа 31 (Описание товара) ---
    $w.WriteStartElement('G_31')

    # Используем WriteAttributeString, чтобы Альта правильно распарсила префиксы 'Pref='.
    # Используем WriteRaw с безопасным эскейпингом, чтобы не было двойного эскейпа.
    $w.WriteStartElement('NAME')
    $w.WriteAttributeString('Pref','1-:')
    $w.WriteRaw( (Get-SafeXml (V ($pref+'g31.name') "BLOCK $gi g31.name")) )
    $w.WriteEndElement()

    $w.WriteStartElement('FIRMA')
    $w.WriteAttributeString('Pref','ПРОИЗВ.:')
    $w.WriteRaw( (Get-SafeXml (V ($pref+'g31.manufacturer') "BLOCK $gi g31.manufacturer")) )
    $w.WriteEndElement()

    $w.WriteStartElement('TM')
    $w.WriteAttributeString('Pref','(ТМ):')
    $w.WriteRaw( (Get-SafeXml (V ($pref+'g31.trade_mark') "BLOCK $gi g31.trade_mark")) )
    $w.WriteEndElement()

    $w.WriteStartElement('PL')
    $w.WriteAttributeString('Pref','2-')
    $w.WriteEndElement()

    $pl = V ($pref+'places') "BLOCK $gi places"
    if (-not [string]::IsNullOrWhiteSpace($pl)) { WriteEl 'PLACE' $pl }

    $w.WriteEndElement() # Закрываем G_31

    # --- Массив TXT (Текстовые дополнения к графе 31) ---
    $tj = 1
    while ($true) {
        $l1_key = $pref + "txt[$tj].line_1"

        # Если ключа нет в файле — значит массив TXT для этого товара закончился
        if (-not (HasKey $l1_key)) { break }

        $l1 = V $l1_key "BLOCK $gi TXT $tj line_1"
        $l2 = V ($pref+"txt[$tj].line_2") "BLOCK $gi TXT $tj line_2"

        # Формат Альты требует пустых тегов <TEXT> для разрывов строк
        foreach ($t in @($l1, '', $l2, '', '', '')) {
            $w.WriteStartElement('TXT')
            WriteEl 'TEXT' $t
            $w.WriteEndElement()
        }
        $tj++
    }

    # --- Массив TOVG (Товары группы, артикулы) ---
    $vj = 1
    while ($true) {
        $ln_key = $pref + "tovg[$vj].line_no"

        # Граница массива
        if (-not (HasKey $ln_key)) { break }

        $w.WriteStartElement('TOVG')
        WV 'G32G'       $ln_key
        WV 'G31_1'      ($pref+"tovg[$vj].description")
        WV 'G31_11'     ($pref+"tovg[$vj].manufacturer")
        WV 'G31_12'     ($pref+"tovg[$vj].trade_mark")
        WV 'G31_14'     ($pref+"tovg[$vj].goods_mark")
        WV 'G31_15_MOD' ($pref+"tovg[$vj].model")
        WV 'KOLVO'      ($pref+"tovg[$vj].quantity")
        WV 'CODE_EDI'   ($pref+"tovg[$vj].unit_code")
        WV 'NAME_EDI'   ($pref+"tovg[$vj].unit_name")
        WV 'G31_35'     ($pref+"tovg[$vj].gross_weight")
        WV 'G31_38'     ($pref+"tovg[$vj].net_weight")
        WV 'G31_42'     ($pref+"tovg[$vj].invoice_cost")
        WV 'INVOICCOST' ($pref+"tovg[$vj].invoice_cost") # Дубликат
        $w.WriteEndElement() # Закрываем TOVG

        $vj++
    }

    # --- Массив G44 (Документы к товару) ---
    $g44Pref = $pref
    $firstDc_key = $pref + "g44_docs[1].doc_code"

    # Fallback-логика: если у текущего товара (например goods[2]) нет своих документов G44,
    # мы автоматически используем (копируем) список документов от первого товара.
    if (-not (HasKey $firstDc_key)) {
        $fallback_key = "goods[1].g44_docs[1].doc_code"
        if (HasKey $fallback_key) {
            $g44Pref = "goods[1]."
        }
    }

    $dk = 1
    while ($true) {
        $dc_key = $g44Pref + "g44_docs[$dk].doc_code"

        # Граница массива
        if (-not (HasKey $dc_key)) { break }

        $w.WriteStartElement('G44')
        WV 'G4403' ($g44Pref+"g44_docs[$dk].kind_code")
        WV 'G441'  $dc_key
        WV 'G442'  ($g44Pref+"g44_docs[$dk].doc_number")
        WV 'G443'  ($g44Pref+"g44_docs[$dk].doc_date")
        WV 'G444'  ($g44Pref+"g44_docs[$dk].doc_name")
        $w.WriteEndElement() # Закрываем G44

        $dk++
    }

    $w.WriteEndElement() # Закрываем BLOCK
}

$w.WriteEndElement() # Закрываем AltaGTD
$w.WriteEndDocument()
$w.Close()

# --------------------------------------------------------------
# ШАГ 4: ИТОГОВАЯ ПРОВЕРКА И ВЫВОД ОТЧЕТА
# --------------------------------------------------------------

# Если в процессе работы накопились ошибки (отсутствие полей или pending),
# мы сообщаем об этом и роняем скрипт с кодом 1,
# хотя сам XML-файл при этом был сохранен (best-effort).
if ($Errors.Count -gt 0) {
    Write-Host ""
    Write-Host "XML сгенерирован, но обнаружены ОШИБКИ ВАЛИДАЦИИ!" -ForegroundColor Red
    Write-Host "--- СПИСОК ОШИБОК ---"
    $Errors | ForEach-Object { Write-Host $_ }
    Write-Host "Всего ошибок: $($Errors.Count)"
    exit 1
}

# Если ошибок нет, делаем быструю диагностику получившегося XML
$txt = [System.IO.File]::ReadAllText($outPath, $enc)
$x = [xml]$txt

# Считаем количество сгенерированных ключевых узлов через регулярные выражения
$cntBlock = ([regex]::Matches($txt, '<BLOCK>').Count)
$cntTovg  = ([regex]::Matches($txt, '<TOVG>').Count)
$cntTxt   = ([regex]::Matches($txt, '<TXT>').Count)
$cntG44   = ([regex]::Matches($txt, '<G44>').Count)

Write-Host "УСПЕШНО: XML файл сгенерирован ($outPath)" -ForegroundColor Green
Write-Host "Корневой тег: $($x.DocumentElement.Name)"
Write-Host "Статистика узлов: BLOCK=$cntBlock | TOVG=$cntTovg | TXT=$cntTxt | G44=$cntG44"
exit 0
