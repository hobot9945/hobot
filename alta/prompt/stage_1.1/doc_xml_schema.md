# doc_xml_schema.md — Stage 1.1 (Формализация: primary.md → XML)

## Назначение
Этот документ задаёт правила генерации XML формализованных документов для импорта в Альту.

Вход этапа 1.1:
- `alta\stage_1.0_result\<ИмяКейса>\primary.md`
- `alta\source\<ИмяКейса>\...\<папка первички\md\*.md>` - md-файлы, используются для постановки link в текстовых блоках

Выход этапа 1.1:
- `alta\stage_1.1_result\<ИмяКейса>\formalized_docs\*.xml` в кодировке `windows-1251`
- `alta\stage_1.1_result\<ИмяКейса>\doc_xml_review.md`

## 1. Исполнение задачи

### 1.1 Правила

  1) **Маппинг 1:1:** Для документов раздела `formalized` в `primary.md` (в блоке `fields`) имена полей полностью 
     совпадают с именами XML-тегов.

  2) **Кодировка:** Все файлы генерируются строго в `windows-1251`. Декларация: `<?xml version="1.0" encoding="windows-1251"?>`.

  3) **Экранирование:** все текстовые значения должны быть XML-экранированы, за исключением управляющим символов xml
     (`&#13;` и `&#10;`):
    - `&` → `&amp;`,
    - `<` → `&lt;`,
    - `>` → `&gt;`,
    - `"` → `&quot;`,
    - `'` → `&apos;`,
    - `&#13;` и `&#10;` - **НЕ** экранируй!
 
  4) **Даты:** Приведение к формату `YYYY-MM-DD`.

  5) **Числа/даты/артикулы/коды** - как в оригинале.

  6) **Пустые XML поля:** если поле в `primary.md` имеет `status: pending` — оставь в XML пустое значение, фиксируй факт 
     в `doc_xml_review.md` и сообщи оператору.

  7) **Подстановка линков:**
    - **!!! ЗАПРЕЩЕНО оставлять `link:...` в сгенерированном XML.**
    - Любое поле в `primary.md`, содержащее `link:<путь>`, **ОБЯЗАНО** быть разрешено в полный текст/данные подставляемого
      значения.
    - **Источник текста** — только раздел `# Текст документа` внутри md-файла.
    - **Никаких сокращений, выжимок, пересказов.** Текст переносится максимально близко к исходному документу.
    - Если текст представлен на нескольких языках — использовать **русский вариант**.
    - Текст исходных файлов читай с помощью `read_file "<path>", "utf-8", "xml_escape>"`.
    - Патч пиши командой `patch_file "<path>", "<строка_замены>", "<строка_замены>", "<текст_подстановки>", "<cp1251>"`.

  8) **Что исключать из подстановки (служебные отметки конвертера):**
    - блок `# META` целиком;
    - строки вида `## Page N`;
    - строки вида `- confidence: ...`;
    - раздел `# Смысловые блоки ...` и всё ниже него.

### 1.2 Генерация `*.xml`
Есть два режима генерации:
  - семантика - AI генерирует `*.xml` своими средствами, опираясь на данную схему;
  - механика - AI запускает скрипт `alta\service\script\gen_doc_xml.bat "<ИмяКейса>"`, где `<ИмяКейса>` -имя каталога
    `alta\stage_1.1_result\<ИмяКейса>`.

  0) Проверь наличие выходного каталога.

  1) Проверь состояние `primary.md` скриптами:
     - `alta\service\script\gen_result_full_audit.bat alta\stage_1.0_result\<ИмяКейса>\primary.md`.
     - `alta\service\script\check_pendings.bat alta\stage_1.0_result\<ИмяКейса>\primary.md`.
     - Если проверка выдает ошибки, сообщи оператору о необходимости возврата на стадию 1.0

  2) Выдай на экран меню и запроси тип генерации.

  3) Выполни генерацию:
     - в механическом варианте просто запусти скрипт.
     - в семантическом варианте, в цикле, подокументно:
       - для каждого документа из `primary.md/formalized` создать отдельный XML-файл,
       - если в поле встретился `link`,
         - проверь, что русский текст не перемешан с английским,
         - выполни подстановку,
       - выполнить верификацию файла (см. п. 1.3).

  4) В механическом варианте, патчами выполни подстановку линков. **ФАЙЛЫ, ТРЕБУЮЩИЕ ПОДСТАНОВКИ**, отмечены в отчете скрипта. 
     Там же **указаны СТРОКА ПОДСТАНОВКИ И ЛИНК НА ФАЙЛ-ИСТОЧНИК**.
     - проверь, что русский текст не перемешан с английским,
     - выполни подстановку,

  5) Сформируй `dt_xml_review.md`.

### 1.3 Верификация (обязательна после семантической генерации)
  1) После генерации `dt.xml` AI обязан проверить, что 
    - линки подставлены во все файлы,
    - для вставки взят русский текст,
    - файлы выведены в кодировке `windows-1251`.

  2) **ТОЛЬКО ПРИ СЕМАНТИЧЕСКОЙ ГЕНЕРАЦИИ**, проверяй для каждого документа:
    - Соответствие корневого тега значению `xml_target_root` из `primary.md` (для данного документа).
    - Полноту переноса данных: все ли поля из `primary.md/fields` (для данного документа) попали в XML (с учетом
      правил скаляров/объектов/массивов).
    - XML well-formed: корректно закрыты все теги.
    - в документ помещены все элементы массивов из `primary.md`.

  3) Запусти `alta\service\script\check_xml.bat <путь_к_каталогу_с_xml>` для проверки формата всех *.xml.

### 1.4. Отчетность
- Сгенерировать отчет `doc_xml_review.md` в папке `alta\stage_1.1_result\<ИмяКейса>\`.
- Указать список созданных файлов и возникшие трудности при разрешении линков (если были).

## 3. Структурные правила

- **Скаляры:** `FieldName` → `<FieldName>value</FieldName>`
- **Объекты:** `ObjName` (таблица полей) → `<ObjName><Field>...</Field></ObjName>`
- **Массивы:** `TagName_[n]` → Повторяющиеся узлы `<TagName>...</TagName>`. Суффикс `_[n]` в XML не пишется.

## 4. Корневые типы
Генератор должен поддерживать XML по `xml_target_root` из `primary.md`:

- `AltaE2I` (Invoice 04021) + повторяющийся блок `InvoiceGoods`
- `AltaE2PACK` (Packing List 04131) + повторяющиеся блоки `Goods`, `TransportMeans`
- `AltaE3CMR` (CMR 02015) + повторяющийся блок `CMRGoods` (внутри `GoodsPackingInfo`)
- `AltaPaymentOrder` (Payment Order 04023) + вложенный блок `PayerSign`
- `AltaServiceInvoice` (Service Invoice 04031) + повторяющийся блок `ServiceDescription` + вложенные блоки реквизитов
- `AltaFreeDoc` используется для генерации документов `Insurance Invoice` (04111) и `Tech Description` (05999).
Имена файлов выводятся на основе их типов.

---

## 2. Подокументное маппирование XML тегов

### 2.1. Invoice (04021) — AltaE2I

#### 2.1.1 Заголовок / реквизиты

| XML тег                                  | UQI                                                           | Комментарий                               |
|------------------------------------------|---------------------------------------------------------------|-------------------------------------------|
| `DocumentCode`                           | (константа)                                                   | `04021`                                   |
| `CurrencyRate`                           | `formalized.invoice_1.CurrencyRate`                           | курс                                      |
| `CurrencyCode`                           | `formalized.invoice_1.CurrencyCode`                           | ISO 4217 alpha-3 (например `CNY`)         |
| `PlacesQuantity`                         | `formalized.invoice_1.PlacesQuantity`                         |                                           |
| `PlacesDescription`                      | `formalized.invoice_1.PlacesDescription`                      |                                           |
| `GrossWeightQuantity`                    | `formalized.invoice_1.GrossWeightQuantity`                    | общий брутто                              |
| `NetWeightQuantity`                      | `formalized.invoice_1.NetWeightQuantity`                      | общий нетто                               |
| `GCost`                                  | `formalized.invoice_1.GCost`                                  | системное поле                            |
| `TotalCost`                              | `formalized.invoice_1.TotalCost`                              | итого                                     |
| `DeliveryTerms_DeliveryPlace`            | `formalized.invoice_1.DeliveryTerms_DeliveryPlace`            |                                           |
| `DeliveryTerms_DeliveryTermsNumericCode` | `formalized.invoice_1.DeliveryTerms_DeliveryTermsNumericCode` |                                           |
| `DeliveryTerms_DeliveryTermsStringCode`  | `formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode`  |                                           |
| `DeliveryTerms_DispatchCountryCode`      | `formalized.invoice_1.DeliveryTerms_DispatchCountryCode`      | alpha-2                                   |
| `DeliveryTerms_TradingCountryCode`       | `formalized.invoice_1.DeliveryTerms_TradingCountryCode`       | alpha-2                                   |
| `DeliveryTerms_DestinationCountryCode`   | `formalized.invoice_1.DeliveryTerms_DestinationCountryCode`   | alpha-2                                   |
| `Registration_PrDocumentName`            | `formalized.invoice_1.Registration_PrDocumentName`            |                                           |
| `Registration_PrDocumentNumber`          | `formalized.invoice_1.Registration_PrDocumentNumber`          |                                           |
| `Registration_PrDocumentDate`            | `formalized.invoice_1.Registration_PrDocumentDate`            | `YYYY-MM-DD`                              |
| `Contract_PrDocumentNumber`              | `formalized.invoice_1.Contract_PrDocumentNumber`              |                                           |
| `Contract_PrDocumentDate`                | `formalized.invoice_1.Contract_PrDocumentDate`                | `YYYY-MM-DD`                              |
| `Buyer_CompanyID`                        | `formalized.invoice_1.Buyer_CompanyID`                        | (по смыслу ИНН; тег вводит в заблуждение) |
| `Buyer_KPPCode`                          | `formalized.invoice_1.Buyer_KPPCode`                          |                                           |
| `Buyer_Name`                             | `formalized.invoice_1.Buyer_Name`                             |                                           |
| `Buyer_PostalAddress_PostalCode`         | `formalized.invoice_1.Buyer_PostalAddress_PostalCode`         |                                           |
| `Buyer_PostalAddress_CountryCode`        | `formalized.invoice_1.Buyer_PostalAddress_CountryCode`        | alpha-2                                   |
| `Buyer_PostalAddress_CounryName`         | `formalized.invoice_1.Buyer_PostalAddress_CounryName`         | опечатка: `CounryName`                    |
| `Buyer_PostalAddress_Region`             | `formalized.invoice_1.Buyer_PostalAddress_Region`             |                                           |
| `Buyer_PostalAddress_City`               | `formalized.invoice_1.Buyer_PostalAddress_City`               |                                           |
| `Buyer_PostalAddress_StreetHouse`        | `formalized.invoice_1.Buyer_PostalAddress_StreetHouse`        |                                           |
| `Seler_Name`                             | `formalized.invoice_1.Seler_Name`                             | опечатка: `Seler` (продавец)              |
| `Seler_PostalAddress_CountryCode`        | `formalized.invoice_1.Seler_PostalAddress_CountryCode`        | alpha-2                                   |
| `Seler_PostalAddress_CounryName`         | `formalized.invoice_1.Seler_PostalAddress_CounryName`         | опечатка: `CounryName`                    |
| `Seler_PostalAddress_Region`             | `formalized.invoice_1.Seler_PostalAddress_Region`             |                                           |
| `Seler_PostalAddress_City`               | `formalized.invoice_1.Seler_PostalAddress_City`               |                                           |
| `Seler_PostalAddress_StreetHouse`        | `formalized.invoice_1.Seler_PostalAddress_StreetHouse`        |                                           |
| `Consignor_OrganizationName`             | `formalized.invoice_1.Consignor_OrganizationName`             |                                           |
| `Consignor_Address_CountryCode`          | `formalized.invoice_1.Consignor_Address_CountryCode`          | alpha-2                                   |
| `Consignor_Address_CounryName`           | `formalized.invoice_1.Consignor_Address_CounryName`           | опечатка: `CounryName`                    |
| `Consignor_Address_Region`               | `formalized.invoice_1.Consignor_Address_Region`               |                                           |
| `Consignor_Address_City`                 | `formalized.invoice_1.Consignor_Address_City`                 |                                           |
| `Consignor_Address_StreetHouse`          | `formalized.invoice_1.Consignor_Address_StreetHouse`          |                                           |
| `Consignee_OrganizationName`             | `formalized.invoice_1.Consignee_OrganizationName`             |                                           |
| `Consignee_OGRN`                         | `formalized.invoice_1.Consignee_OGRN`                         |                                           |
| `Consignee_INN`                          | `formalized.invoice_1.Consignee_INN`                          |                                           |
| `Consignee_KPP`                          | `formalized.invoice_1.Consignee_KPP`                          |                                           |
| `Consignee_Address_PostalCode`           | `formalized.invoice_1.Consignee_Address_PostalCode`           |                                           |
| `Consignee_Address_CountryCode`          | `formalized.invoice_1.Consignee_Address_CountryCode`          | alpha-2                                   |
| `Consignee_Address_CounryName`           | `formalized.invoice_1.Consignee_Address_CounryName`           | опечатка: `CounryName`                    |
| `Consignee_Address_Region`               | `formalized.invoice_1.Consignee_Address_Region`               |                                           |
| `Consignee_Address_City`                 | `formalized.invoice_1.Consignee_Address_City`                 |                                           |
| `Consignee_Address_StreetHouse`          | `formalized.invoice_1.Consignee_Address_StreetHouse`          |                                           |

### 2.1.2 InvoiceGoods (повторяющийся блок)

Правило: каждый `formalized.invoice_1.InvoiceGoods_[n]` → отдельный `<InvoiceGoods>...</InvoiceGoods>`.

| XML тег внутри `InvoiceGoods`             | UQI                                                                             | Комментарий                                                                                 |
|-------------------------------------------|---------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------|
| `GoodsCode`                               | `formalized.invoice_1.InvoiceGoods_[n].GoodsCode`                               | ТН ВЭД                                                                                      |
| `GoodsDescription`                        | `formalized.invoice_1.InvoiceGoods_[n].GoodsDescription`                        | текст как в документе                                                                       |
| `GoodsQuantity`                           | `formalized.invoice_1.InvoiceGoods_[n].GoodsQuantity`                           |                                                                                             |
| `MeasureUnitQualifierName`                | `formalized.invoice_1.InvoiceGoods_[n].MeasureUnitQualifierName`                |                                                                                             |
| `GrossWeightQuantity`                     | `formalized.invoice_1.InvoiceGoods_[n].GrossWeightQuantity`                     |                                                                                             |
| `NetWeightQuantity`                       | `formalized.invoice_1.InvoiceGoods_[n].NetWeightQuantity`                       |                                                                                             |
| `Price`                                   | `formalized.invoice_1.InvoiceGoods_[n].Price`                                   |                                                                                             |
| `TotalCost`                               | `formalized.invoice_1.InvoiceGoods_[n].TotalCost`                               |                                                                                             |
| `OriginCountryCode`                       | `formalized.invoice_1.InvoiceGoods_[n].OriginCountryCode`                       | цифровой код страны                                                                         |
| `AdditionalGoodsDescription_Manufacturer` | `formalized.invoice_1.InvoiceGoods_[n].AdditionalGoodsDescription_Manufacturer` |                                                                                             |
| `AdditionalGoodsDescription_TradeMark`    | `formalized.invoice_1.InvoiceGoods_[n].AdditionalGoodsDescription_TradeMark`    | если отсутствует — `"ОТСУТСТВУЕТ"`                                                          |
| `AdditionalGoodsDescription_GoodsMark`    | `formalized.invoice_1.InvoiceGoods_[n].AdditionalGoodsDescription_GoodsMark`    | если отсутствует — `"ОТСУТСТВУЕТ"`                                                          |
| `AdditionalGoodsDescription_GoodsModel`   | `formalized.invoice_1.InvoiceGoods_[n].AdditionalGoodsDescription_GoodsModel`   | тег `GoodsModel` по смыслу вводит в заблуждение (фактически “наименование/вариант позиции”) |

---

### 2.2 Packing List (04131) — AltaE2PACK

#### 2.2.1 Заголовок / реквизиты

| XML тег                                       | UQI                                                                     | Комментарий            |
|-----------------------------------------------|-------------------------------------------------------------------------|------------------------|
| `GrossWeightQuantity`                         | `formalized.packing_list.GrossWeightQuantity`                         | общий брутто           |
| `NetWeightQuantity`                           | `formalized.packing_list.NetWeightQuantity`                           | общий нетто            |
| `Consignor_OrganizationName`                  | `formalized.packing_list.Consignor_OrganizationName`                  |                        |
| `Consignor_ShortName`                         | `formalized.packing_list.Consignor_ShortName`                         |                        |
| `Consignor_Address_CountryCode`               | `formalized.packing_list.Consignor_Address_CountryCode`               | alpha-2                |
| `Consignor_Address_CounryName`                | `formalized.packing_list.Consignor_Address_CounryName`                | опечатка: `CounryName` |
| `Consignor_Address_Region`                    | `formalized.packing_list.Consignor_Address_Region`                    |                        |
| `Consignor_Address_City`                      | `formalized.packing_list.Consignor_Address_City`                      |                        |
| `Consignor_Address_StreetHouse`               | `formalized.packing_list.Consignor_Address_StreetHouse`               |                        |
| `Consignee_OrganizationName`                  | `formalized.packing_list.Consignee_OrganizationName`                  |                        |
| `Consignee_ShortName`                         | `formalized.packing_list.Consignee_ShortName`                         |                        |
| `Consignee_OGRN`                              | `formalized.packing_list.Consignee_OGRN`                              |                        |
| `Consignee_INN`                               | `formalized.packing_list.Consignee_INN`                               |                        |
| `Consignee_KPP`                               | `formalized.packing_list.Consignee_KPP`                               |                        |
| `Consignee_Address_PostalCode`                | `formalized.packing_list.Consignee_Address_PostalCode`                |                        |
| `Consignee_Address_CountryCode`               | `formalized.packing_list.Consignee_Address_CountryCode`               | alpha-2                |
| `Consignee_Address_CounryName`                | `formalized.packing_list.Consignee_Address_CounryName`                | опечатка: `CounryName` |
| `Consignee_Address_Region`                    | `formalized.packing_list.Consignee_Address_Region`                    |                        |
| `Consignee_Address_City`                      | `formalized.packing_list.Consignee_Address_City`                      |                        |
| `Consignee_Address_StreetHouse`               | `formalized.packing_list.Consignee_Address_StreetHouse`               |                        |
| `DeliveryTerms_DeliveryPlace`                 | `formalized.packing_list.DeliveryTerms_DeliveryPlace`                 |                        |
| `DeliveryTerms_DeliveryTermsNumericCode`      | `formalized.packing_list.DeliveryTerms_DeliveryTermsNumericCode`      |                        |
| `DeliveryTerms_DeliveryTermsStringCode`       | `formalized.packing_list.DeliveryTerms_DeliveryTermsStringCode`       |                        |
| `DeliveryTerms_Contract_PrDocumentName`       | `formalized.packing_list.DeliveryTerms_Contract_PrDocumentName`       |                        |
| `DeliveryTerms_Contract_PrDocumentNumber`     | `formalized.packing_list.DeliveryTerms_Contract_PrDocumentNumber`     |                        |
| `DeliveryTerms_Contract_PrDocumentDate`       | `formalized.packing_list.DeliveryTerms_Contract_PrDocumentDate`       | `YYYY-MM-DD`           |
| `DeliveryTerms_Invoice_PrDocumentName`        | `formalized.packing_list.DeliveryTerms_Invoice_PrDocumentName`        |                        |
| `DeliveryTerms_Invoice_PrDocumentNumber`      | `formalized.packing_list.DeliveryTerms_Invoice_PrDocumentNumber`      |                        |
| `DeliveryTerms_Invoice_PrDocumentDate`        | `formalized.packing_list.DeliveryTerms_Invoice_PrDocumentDate`        | `YYYY-MM-DD`           |
| `DeliveryTerms_Registration_PrDocumentName`   | `formalized.packing_list.DeliveryTerms_Registration_PrDocumentName`   |                        |
| `DeliveryTerms_Registration_PrDocumentNumber` | `formalized.packing_list.DeliveryTerms_Registration_PrDocumentNumber` |                        |
| `DeliveryTerms_Registration_PrDocumentDate`   | `formalized.packing_list.DeliveryTerms_Registration_PrDocumentDate`   | `YYYY-MM-DD`           |

#### 2.2.2 Goods (повторяющийся блок)

Правило: каждый `formalized.packing_list.Goods_[n]` → отдельный `<Goods>...</Goods>`.

| XML тег внутри `Goods`       | UQI                                                              | Комментарий                                  |
|------------------------------|------------------------------------------------------------------|----------------------------------------------|
| `GoodsDescription`           | `formalized.packing_list.Goods_[n].GoodsDescription`           | описание “грузовой строки”                   |
| `GoodsQuantity`              | `formalized.packing_list.Goods_[n].GoodsQuantity`              | кол-во мест/груз.единиц (не “кол-во товара”) |
| `GrossWeightQuantity`        | `formalized.packing_list.Goods_[n].GrossWeightQuantity`        |                                              |
| `NetWeightQuantity`          | `formalized.packing_list.Goods_[n].NetWeightQuantity`          |                                              |
| `PackingInfo/PakingQuantity` | `formalized.packing_list.Goods_[n].PackingInfo.PakingQuantity` | опечатка: `PakingQuantity`                   |

#### 2.2.3 TransportMeans (повторяющийся блок)

Правило: каждый `formalized.packing_list.TransportMeans_[n]` → отдельный `<TransportMeans>...</TransportMeans>`.

| XML тег внутри `TransportMeans` | UQI                                                            | Комментарий                   |
|---------------------------------|----------------------------------------------------------------|-------------------------------|
| `Number`                        | `formalized.packing_list.TransportMeans_[n].Number`          | номер ТС                      |
| `ModeCode`                      | `formalized.packing_list.TransportMeans_[n].ModeCode`        | код вида транспорта           |
| `NationalityCode`               | `formalized.packing_list.TransportMeans_[n].NationalityCode` |                               |
| `MoverIndicator`                | `formalized.packing_list.TransportMeans_[n].MoverIndicator`  | `true` тягач / `false` прицеп |

---

### 2.3 CMR (02015) — AltaE3CMR

#### 2.3.1 Заголовок / реквизиты

| XML тег                                     | UQI                                                          | Комментарий                |
|---------------------------------------------|--------------------------------------------------------------|----------------------------|
| `LanguageCode`                              | `formalized.cmr.LanguageCode`                              |                            |
| `CMR_Choice`                                | `formalized.cmr.CMR_Choice`                                | системный признак          |
| `RegistrationDocument_RegID`                | `formalized.cmr.RegistrationDocument_RegID`                | номер                      |
| `RegistrationDocument_DateInf`              | `formalized.cmr.RegistrationDocument_DateInf`              | `YYYY-MM-DD`               |
| `RegistrationDocument_Place`                | `formalized.cmr.RegistrationDocument_Place`                |                            |
| `TrakingCargo_TakingCargoDate`              | `formalized.cmr.TrakingCargo_TakingCargoDate`              | `YYYY-MM-DD`               |
| `TrakingCargo_TakingCargoPlace_CountryCode` | `formalized.cmr.TrakingCargo_TakingCargoPlace_CountryCode` | alpha-2                    |
| `TrakingCargo_TakingCargoPlace_CounryName`  | `formalized.cmr.TrakingCargo_TakingCargoPlace_CounryName`  | опечатка: `CounryName`     |
| `DeliveryPlace_CountryCode`                 | `formalized.cmr.DeliveryPlace_CountryCode`                 | alpha-2                    |
| `DeliveryPlace_CounryName`                  | `formalized.cmr.DeliveryPlace_CounryName`                  | опечатка: `CounryName`     |
| `DeliveryTerms_DeliveryPlace`               | `formalized.cmr.DeliveryTerms_DeliveryPlace`               |                            |
| `DeliveryTerms_DeliveryTermsStringCode`     | `formalized.cmr.DeliveryTerms_DeliveryTermsStringCode`     |                            |
| `GoodsQuantity`                             | `formalized.cmr.GoodsQuantity`                             | общее кол-во мест/упаковок |
| `CMRGoodsWeight_GrossWeightQuantity`        | `formalized.cmr.CMRGoodsWeight_GrossWeightQuantity`        | общий брутто               |
| `CMRTransport_PrimeMoverStateSignID`        | `formalized.cmr.CMRTransport_PrimeMoverStateSignID`        | тягач                      |
| `CMRTransport_TrailerStateSignID`           | `formalized.cmr.CMRTransport_TrailerStateSignID`           | прицеп                     |
| `Consignor_NameInf`                         | `formalized.cmr.Consignor_NameInf`                         |                            |
| `Consignor_ShortName`                       | `formalized.cmr.Consignor_ShortName`                       |                            |
| `Consignor_PostalAddress_CountryCode`       | `formalized.cmr.Consignor_PostalAddress_CountryCode`       | alpha-2                    |
| `Consignor_Address_CounryName`              | `formalized.cmr.Consignor_Address_CounryName`              | опечатка: `CounryName`     |
| `Consignor_Address_Region`                  | `formalized.cmr.Consignor_Address_Region`                  |                            |
| `Consignor_Address_City`                    | `formalized.cmr.Consignor_Address_City`                    |                            |
| `Consignor_Address_StreetHouse`             | `formalized.cmr.Consignor_Address_StreetHouse`             |                            |
| `Consignor_Guarantee_OrganizationName`      | `formalized.cmr.Consignor_Guarantee_OrganizationName`      |                            |
| `Consignor_Guarantee_ShortName`             | `formalized.cmr.Consignor_Guarantee_ShortName`             |                            |
| `Consignor_Guarantee_Address_CountryCode`   | `formalized.cmr.Consignor_Guarantee_Address_CountryCode`   | alpha-2                    |
| `Consignor_Guarantee_Address_CounryName`    | `formalized.cmr.Consignor_Guarantee_Address_CounryName`    | опечатка: `CounryName`     |
| `Consignor_Guarantee_Address_Region`        | `formalized.cmr.Consignor_Guarantee_Address_Region`        |                            |
| `Consignor_Guarantee_Address_City`          | `formalized.cmr.Consignor_Guarantee_Address_City`          |                            |
| `Consignor_Guarantee_Address_StreetHouse`   | `formalized.cmr.Consignor_Guarantee_Address_StreetHouse`   |                            |
| `Consignee_NameInf`                         | `formalized.cmr.Consignee_NameInf`                         |                            |
| `Consignee_ShortName`                       | `formalized.cmr.Consignee_ShortName`                       |                            |
| `Consignee_OGRNID`                          | `formalized.cmr.Consignee_OGRNID`                          |                            |
| `Consignee_INNID`                           | `formalized.cmr.Consignee_INNID`                           |                            |
| `Consignee_KPPCode`                         | `formalized.cmr.Consignee_KPPCode`                         |                            |
| `Consignee_PostalAddress_PostalCode`        | `formalized.cmr.Consignee_PostalAddress_PostalCode`        |                            |
| `Consignee_PostalAddress_CountryCode`       | `formalized.cmr.Consignee_PostalAddress_CountryCode`       | alpha-2                    |
| `Consignee_Address_CounryName`              | `formalized.cmr.Consignee_Address_CounryName`              | опечатка: `CounryName`     |
| `Consignee_Address_Region`                  | `formalized.cmr.Consignee_Address_Region`                  |                            |
| `Consignee_Address_City`                    | `formalized.cmr.Consignee_Address_City`                    |                            |
| `Consignee_Address_StreetHouse`             | `formalized.cmr.Consignee_Address_StreetHouse`             |                            |

#### 2.3.2 CMRGoods (повторяющийся блок)

Правило: каждый `formalized.cmr.CMRGoods_[n]` → отдельный `<CMRGoods>...</CMRGoods>`.

| XML тег внутри `CMRGoods` | UQI                                                   | Комментарий          |
|---------------------------|-------------------------------------------------------|----------------------|
| `GoodsDescription`        | `formalized.cmr.CMRGoods_[n].GoodsDescription`      |                      |
| `GoodsNumeric`            | `formalized.cmr.CMRGoods_[n].GoodsNumeric`          | номер строки         |
| `GoodsNomenclatureCode`   | `formalized.cmr.CMRGoods_[n].GoodsNomenclatureCode` | по смыслу ТН ВЭД     |
| `GoodsQuantity`           | `formalized.cmr.CMRGoods_[n].GoodsQuantity`         | кол-во мест/упаковок |
| `GrossWeightQuantity`     | `formalized.cmr.CMRGoods_[n].GrossWeightQuantity`   | брутто по строке     |

#### 2.3.3 GoodsPackingInfo (вложенный блок внутри CMRGoods)

| XML тег                               | UQI                                                                 | Комментарий                |
|---------------------------------------|---------------------------------------------------------------------|----------------------------|
| `GoodsPackingInfo/PackingCode`        | `formalized.cmr.CMRGoods_[n].GoodsPackingInfo.PackingCode`        |                            |
| `GoodsPackingInfo/PakingQuantity`     | `formalized.cmr.CMRGoods_[n].GoodsPackingInfo.PakingQuantity`     | опечатка: `PakingQuantity` |
| `GoodsPackingInfo/PackingDescription` | `formalized.cmr.CMRGoods_[n].GoodsPackingInfo.PackingDescription` |                            |

---

### 2.4 Payment Order (04023) — AltaPaymentOrder

#### 2.4.1 Заголовок / реквизиты

| XML тег                              | UQI                                                             | Комментарий                      |
|--------------------------------------|-----------------------------------------------------------------|----------------------------------|
| `DocumentCode`                       | (константа)                                                     | `04023`                          |
| `PaymentModeCode`                    | `formalized.payment_order_1.PaymentModeCode`                    | системный код                    |
| `PaymentAmount`                      | `formalized.payment_order_1.PaymentAmount`                      |                                  |
| `TransactionKind`                    | `formalized.payment_order_1.TransactionKind`                    |                                  |
| `Priority`                           | `formalized.payment_order_1.Priority`                           |                                  |
| `Purpose`                            | `formalized.payment_order_1.Purpose`                            | текст                            |
| `ValueSpelledOut`                    | `formalized.payment_order_1.ValueSpelledOut`                    | текст                            |
| `DocumentReference_PrDocumentNumber` | `formalized.payment_order_1.DocumentReference_PrDocumentNumber` |                                  |
| `DocumentReference_PrDocumentDate`   | `formalized.payment_order_1.DocumentReference_PrDocumentDate`   | `YYYY-MM-DD`                     |
| `Payer_OrganizationName`             | `formalized.payment_order_1.Payer_OrganizationName`             |                                  |
| `Payer_INN`                          | `formalized.payment_order_1.Payer_INN`                          |                                  |
| `Payer_KPP`                          | `formalized.payment_order_1.Payer_KPP`                          |                                  |
| `Payer_Bank_BankName`                | `formalized.payment_order_1.Payer_Bank_BankName`                | может быть многострочным текстом |
| `Payee_OrganizationName`             | `formalized.payment_order_1.Payee_OrganizationName`             |                                  |
| `Payee_Bank_BankName`                | `formalized.payment_order_1.Payee_Bank_BankName`                | может быть многострочным текстом |

#### 2.4.1 PayerSign (вложенный блок)

| XML тег                   | UQI                                                  | Комментарий |
|---------------------------|------------------------------------------------------|-------------|
| `PayerSign/PersonSurname` | `formalized.payment_order_1.PayerSign.PersonSurname` |             |
| `PayerSign/PersonName`    | `formalized.payment_order_1.PayerSign.PersonName`    |             |

---

### 2.5 Service Invoice (04031) — AltaServiceInvoice

#### 2.5.1 Заголовок / реквизиты

| XML тег                                                          | UQI                                                                                           | Комментарий      |
|------------------------------------------------------------------|-----------------------------------------------------------------------------------------------|------------------|
| `DocumentSign`                                                   | `formalized.service_invoice.DocumentSign`                                                   |                  |
| `TotalServiceCost`                                               | `formalized.service_invoice.TotalServiceCost`                                               |                  |
| `Currency`                                                       | `formalized.service_invoice.Currency`                                                       | ISO 4217 alpha-3 |
| `ServiceProvider_Name`                                           | `formalized.service_invoice.ServiceProvider_Name`                                           |                  |
| `ContractDetails_PrDocumentNumber`                               | `formalized.service_invoice.ContractDetails_PrDocumentNumber`                               |                  |
| `ContractDetails_PrDocumentDate`                                 | `formalized.service_invoice.ContractDetails_PrDocumentDate`                                 | `YYYY-MM-DD`     |
| `Registration_PrDocumentName`                                    | `formalized.service_invoice.Registration_PrDocumentName`                                    |                  |
| `Registration_PrDocumentNumber`                                  | `formalized.service_invoice.Registration_PrDocumentNumber`                                  |                  |
| `Registration_PrDocumentDate`                                    | `formalized.service_invoice.Registration_PrDocumentDate`                                    | `YYYY-MM-DD`     |
| `Consignor_OrganizationName`                                     | `formalized.service_invoice.Consignor_OrganizationName`                                     |                  |
| `Consignee_OrganizationName`                                     | `formalized.service_invoice.Consignee_OrganizationName`                                     |                  |
| `Consignee_RFOrganizationFeatures_OGRN`                          | `formalized.service_invoice.Consignee_RFOrganizationFeatures_OGRN`                          |                  |
| `Consignee_RFOrganizationFeatures_INN`                           | `formalized.service_invoice.Consignee_RFOrganizationFeatures_INN`                           |                  |
| `Consignee_RFOrganizationFeatures_KPP`                           | `formalized.service_invoice.Consignee_RFOrganizationFeatures_KPP`                           |                  |
| `Signature_Choice`                                               | `formalized.service_invoice.Signature_Choice`                                               |                  |
| `SignatureDirectorChiefAccountant_Director_PersonSurname`        | `formalized.service_invoice.SignatureDirectorChiefAccountant_Director_PersonSurname`        |                  |
| `SignatureDirectorChiefAccountant_Director_PersonName`           | `formalized.service_invoice.SignatureDirectorChiefAccountant_Director_PersonName`           |                  |
| `SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname` | `formalized.service_invoice.SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname` |                  |
| `SignatureDirectorChiefAccountant_ChiefAccountant_PersonName`    | `formalized.service_invoice.SignatureDirectorChiefAccountant_ChiefAccountant_PersonName`    |                  |

#### 2.5.2 ServiceProvider_PaymentRequisitions (вложенный блок)

| XML тег                                        | UQI                                                                         | Комментарий |
|------------------------------------------------|-----------------------------------------------------------------------------|-------------|
| `ServiceProvider_PaymentRequisitions/BankName` | `formalized.service_invoice.ServiceProvider_PaymentRequisitions.BankName` |             |

#### 2.5.3 PaymentDocument (вложенный блок)

| XML тег                            | UQI                                                             | Комментарий  |
|------------------------------------|-----------------------------------------------------------------|--------------|
| `PaymentDocument/PrDocumentNumber` | `formalized.service_invoice.PaymentDocument.PrDocumentNumber` |              |
| `PaymentDocument/PrDocumentDate`   | `formalized.service_invoice.PaymentDocument.PrDocumentDate`   | `YYYY-MM-DD` |

#### 2.5.4 Consignor_SubjectAddressDetails (вложенный блок)

| XML тег                                       | UQI                                                                        | Комментарий            |
|-----------------------------------------------|----------------------------------------------------------------------------|------------------------|
| `Consignor_SubjectAddressDetails/PostalCode`  | `formalized.service_invoice.Consignor_SubjectAddressDetails.PostalCode`  |                        |
| `Consignor_SubjectAddressDetails/CountryCode` | `formalized.service_invoice.Consignor_SubjectAddressDetails.CountryCode` | alpha-2                |
| `Consignor_SubjectAddressDetails/CounryName`  | `formalized.service_invoice.Consignor_SubjectAddressDetails.CounryName`  | опечатка: `CounryName` |
| `Consignor_SubjectAddressDetails/Region`      | `formalized.service_invoice.Consignor_SubjectAddressDetails.Region`      |                        |
| `Consignor_SubjectAddressDetails/Town`        | `formalized.service_invoice.Consignor_SubjectAddressDetails.Town`        |                        |
| `Consignor_SubjectAddressDetails/StreetHouse` | `formalized.service_invoice.Consignor_SubjectAddressDetails.StreetHouse` |                        |

#### 2.5.5 Consignee_SubjectAddressDetails (вложенный блок)

| XML тег                                       | UQI                                                                        | Комментарий            |
|-----------------------------------------------|----------------------------------------------------------------------------|------------------------|
| `Consignee_SubjectAddressDetails/PostalCode`  | `formalized.service_invoice.Consignee_SubjectAddressDetails.PostalCode`  |                        |
| `Consignee_SubjectAddressDetails/CountryCode` | `formalized.service_invoice.Consignee_SubjectAddressDetails.CountryCode` | alpha-2                |
| `Consignee_SubjectAddressDetails/CounryName`  | `formalized.service_invoice.Consignee_SubjectAddressDetails.CounryName`  | опечатка: `CounryName` |
| `Consignee_SubjectAddressDetails/Region`      | `formalized.service_invoice.Consignee_SubjectAddressDetails.Region`      |                        |
| `Consignee_SubjectAddressDetails/Town`        | `formalized.service_invoice.Consignee_SubjectAddressDetails.Town`        |                        |
| `Consignee_SubjectAddressDetails/StreetHouse` | `formalized.service_invoice.Consignee_SubjectAddressDetails.StreetHouse` |                        |
| `Consignee_SubjectAddressDetails/House`       | `formalized.service_invoice.Consignee_SubjectAddressDetails.House`       |                        |
| `Consignee_SubjectAddressDetails/Room`        | `formalized.service_invoice.Consignee_SubjectAddressDetails.Room`        |                        |

#### 2.5.6 ServiceDescription (повторяющийся блок)

Правило: каждый `formalized.service_invoice.ServiceDescription_[n]` → отдельный `<ServiceDescription>...</ServiceDescription>`.

| XML тег внутри `ServiceDescription` | UQI                                                                        | Комментарий      |
|-------------------------------------|----------------------------------------------------------------------------|------------------|
| `GoodsDescription`                  | `formalized.service_invoice.ServiceDescription_[n].GoodsDescription`     | текст            |
| `CurrencyCode`                      | `formalized.service_invoice.ServiceDescription_[n].CurrencyCode`         | ISO 4217 alpha-3 |
| `ServiceName`                       | `formalized.service_invoice.ServiceDescription_[n].ServiceName`          |                  |
| `TaxRate`                           | `formalized.service_invoice.ServiceDescription_[n].TaxRate`              |                  |
| `TaxSum`                            | `formalized.service_invoice.ServiceDescription_[n].TaxSum`               |                  |
| `ServiceCost_Amount`                | `formalized.service_invoice.ServiceDescription_[n].ServiceCost_Amount`   |                  |
| `ServiceCost_Currency`              | `formalized.service_invoice.ServiceDescription_[n].ServiceCost_Currency` | ISO 4217 alpha-3 |

### 2.6 Free Doc / Страховка и техописание (04111 / 05999) — AltaFreeDoc

Используется для генерации документов «Счет за страховые услуги» (04111) и «Техническое описание» (05999).

#### 2.6.1 Заголовок / реквизиты

| XML тег                      | UQI                                                                                                                     | Комментарий                            |
|------------------------------|-------------------------------------------------------------------------------------------------------------------------|----------------------------------------|
| `DocumentCode`               | `formalized.insurance_document.DocumentCode` / `formalized.tech_description.DocumentCode`                               | Константа (`04111` или `05999`)        |
| `DocumentHead_DocumentName`   | `formalized.insurance_document.DocumentHead_DocumentName` / `formalized.tech_description.DocumentHead_DocumentName`     |                                        |
| `DocumentHead_DocumentDate`   | `formalized.insurance_document.DocumentHead_DocumentDate` / `formalized.tech_description.DocumentHead_DocumentDate`     | Формат: `YYYY-MM-DD`                   |
| `DocumentHead_DocumentNumber` | `formalized.insurance_document.DocumentHead_DocumentNumber` / `formalized.tech_description.DocumentHead_DocumentNumber` |                                        |

#### 2.6.2 DocumentBody_TextSection (вложенный блок)

| XML тег                             | UQI                                                                         | Комментарий                           |
|-------------------------------------|-----------------------------------------------------------------------------|---------------------------------------|
| `DocumentBody_TextSection/TextPara` | `formalized.insurance_document.TextPara` / `formalized.tech_description.TextPara` | Полный текст после разрешения линка   |

**Правило фильтрации:** Все неформализуемые поля (начиная с `doc_gr44` и до конца таблицы в `primary.md` для данных типов документов) игнорируются и в XML **не переносятся**.