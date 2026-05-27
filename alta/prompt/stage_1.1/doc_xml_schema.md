# doc_xml_schema.md — Stage 1.1 (Формализация: primary.md → XML)

## Назначение
Этот документ задаёт правила генерации XML формализованных документов для импорта в Альту.

Вход этапа 1.1:
- `alta\stage_1.0_result\<ИмяКейса>\primary.md`
- `alta\source\<ИмяКейса>\...\<папка первички\md\*.md>` - md-файлы, используются для постановки link в текстовых блоках
- `alta\stable_source\*.xml` - стабильные XML файлы, копируются в выходную папку 

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

  1) Проверь состояние `primary.md` скриптом
     `alta\service\script\gen_result_full_audit.bat alta\stage_1.0_result\<ИмяКейса>\primary.md`. Если проверка выдает ошибки,
     сообщи оператору о необходимости возврата на стадию 1.0

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

  5) В обоих вариантах, скопируй стабильные XML файлы 
     из `alta\stable_source\*.xml` в `alta\stage_1.1_result\<ИмяКейса>\formalized_docs\`.

  6) Сформируй `dt_xml_review.md`.

### 1.3 Верификация (обязательна после семантической генерации)
После генерации `dt.xml` AI обязан проверить, что 
  - линки подставлены во все файлы,
  - для вставки взят русский текст,
  - файлы выведены в кодировке `windows-1251`.

При семантической генерации, проверяй при генерации каждого документа:
  - Соответствие корневого тега значению `xml_target_root` из `primary.md` (для данного документа).
  - Полноту переноса данных: все ли поля из `primary.md/fields` (для данного документа) попали в XML (с учетом
    правил скаляров/объектов/массивов).
  - XML well-formed: корректно закрыты все теги.
  - в документ помещены все элементы массивов из `primary.md`.

### 1.4. Отчетность
- Сгенерировать отчет `doc_xml_review.md` в папке `alta\stage_1.1_result\<ИмяКейса>\`.
- Указать список созданных файлов и возникшие трудности при разрешении линков (если были).

## 3. Структурные правила

- **Скаляры:** `FieldName` → `<FieldName>value</FieldName>`
- **Объекты:** `ObjName` (таблица полей) → `<ObjName><Field>...</Field></ObjName>`
- **Массивы:** `TagName_[n]` → Повторяющиеся узлы `<TagName>...</TagName>`. Суффикс `_[n]` в XML не пишется.

## 4. Корневые типы
Генератор должен поддерживать XML по `xml_target_root` из `primary.md`:

- `AltaE2CONT` (Contract 03011)
- `AltaSupplementaryContract` (Supplementary Contract 03012)
- `AltaE2I` (Invoice 04021) + повторяющийся блок `InvoiceGoods`
- `AltaE2PACK` (Packing List 04131) + повторяющиеся блоки `Goods`, `TransportMeans`
- `AltaE3CMR` (CMR 02015) + повторяющийся блок `CMRGoods` (внутри `GoodsPackingInfo`)
- `AltaPaymentOrder` (Payment Order 04023) + вложенный блок `PayerSign`
- `AltaServiceInvoice` (Service Invoice 04031) + повторяющийся блок `ServiceDescription` + вложенные блоки реквизитов

Имена файлов выводятся на основе их типов.

---

### 1) Contract (03011) — AltaE2CONT

| XML тег                                 | UQI                                                           | Комментарий                                                                                |
|-----------------------------------------|---------------------------------------------------------------|--------------------------------------------------------------------------------------------|
| `DocumentCode`                          | (константа)                                                   | `03011`                                                                                    |
| `ContractRegistration_PrDocumentNumber` | `formalized.contract_1.ContractRegistration_PrDocumentNumber` | № контракта                                                                                |
| `ContractRegistration_PrDocumentDate`   | `formalized.contract_1.ContractRegistration_PrDocumentDate`   | дата `YYYY-MM-DD`                                                                          |
| `ContractTerms_Amount`                  | `formalized.contract_1.ContractTerms_Amount`                  | сумма                                                                                      |
| `ContractTerms_CurrencyCode`            | `formalized.contract_1.ContractTerms_CurrencyCode`            | ISO 4217 numeric (пример: CNY=156)                                                         |
| `ContractTerms_LastDate`                | `formalized.contract_1.ContractTerms_LastDate`                | дата `YYYY-MM-DD`                                                                          |
| `ContractTerms_OtherTerms`              | `formalized.contract_1.ContractTerms_OtherTerms`              | условия поставки / Incoterms                                                               |
| `ContractTerms_ContractText`            | `formalized.contract_1.ContractTerms_ContractText`            | если в `primary.md` хранится `link` на файл — прочитать файл и вставить текст (XML-escape) |
| `ContractTerms_DealSign`                | `formalized.contract_1.ContractTerms_DealSign`                | системный признак (обычно `1`)                                                             |
| `ForeignPerson_OrganizationName`        | `formalized.contract_1.ForeignPerson_OrganizationName`        | продавец                                                                                   |
| `ForeignPerson_Address_CountryCode`     | `formalized.contract_1.ForeignPerson_Address_CountryCode`     | alpha-2, напр. `CN`                                                                        |
| `ForeignPerson_Address_CounryName`      | `formalized.contract_1.ForeignPerson_Address_CounryName`      | опечатка в теге: `CounryName`                                                              |
| `ForeignPerson_Address_Region`          | `formalized.contract_1.ForeignPerson_Address_Region`          |                                                                                            |
| `ForeignPerson_Address_City`            | `formalized.contract_1.ForeignPerson_Address_City`            |                                                                                            |
| `ForeignPerson_Address_StreetHouse`     | `formalized.contract_1.ForeignPerson_Address_StreetHouse`     |                                                                                            |
| `RussianPerson_OrganizationName`        | `formalized.contract_1.RussianPerson_OrganizationName`        | покупатель                                                                                 |
| `RussianPerson_OGRN`                    | `formalized.contract_1.RussianPerson_OGRN`                    |                                                                                            |
| `RussianPerson_INN`                     | `formalized.contract_1.RussianPerson_INN`                     |                                                                                            |
| `RussianPerson_KPP`                     | `formalized.contract_1.RussianPerson_KPP`                     |                                                                                            |
| `RussianPerson_Address_PostalCode`      | `formalized.contract_1.RussianPerson_Address_PostalCode`      |                                                                                            |
| `RussianPerson_Address_CountryCode`     | `formalized.contract_1.RussianPerson_Address_CountryCode`     | alpha-2, напр. `RU`                                                                        |
| `RussianPerson_Address_CounryName`      | `formalized.contract_1.RussianPerson_Address_CounryName`      | опечатка в теге: `CounryName`                                                              |
| `RussianPerson_Address_Region`          | `formalized.contract_1.RussianPerson_Address_Region`          |                                                                                            |
| `RussianPerson_Address_City`            | `formalized.contract_1.RussianPerson_Address_City`            |                                                                                            |
| `RussianPerson_Address_StreetHouse`     | `formalized.contract_1.RussianPerson_Address_StreetHouse`     |                                                                                            |
---

### 2) Supplementary Contract (03012) — AltaSupplementaryContract

| XML тег                                   | UQI                                                                           | Комментарий                                                                        |
|-------------------------------------------|-------------------------------------------------------------------------------|------------------------------------------------------------------------------------|
| `DocumentNumber`                          | `formalized.supplementary_contract_1.DocumentNumber`                          | № доп. соглашения                                                                  |
| `IssueDate`                               | `formalized.supplementary_contract_1.IssueDate`                               | дата `YYYY-MM-DD`                                                                  |
| `ContractDescription_Amount`              | `formalized.supplementary_contract_1.ContractDescription_Amount`              | сумма                                                                              |
| `ContractDescription_CurrencyCode`        | `formalized.supplementary_contract_1.ContractDescription_CurrencyCode`        | ISO 4217 numeric                                                                   |
| `ContractDescription_LastDate`            | `formalized.supplementary_contract_1.ContractDescription_LastDate`            | дата `YYYY-MM-DD`                                                                  |
| `ContractDescription_ContractText`        | `formalized.supplementary_contract_1.ContractDescription_ContractText`        | если в `primary.md` хранится `link` — прочитать файл и вставить текст (XML-escape) |
| `ContractDescription_DealSign`            | `formalized.supplementary_contract_1.ContractDescription_DealSign`            | системный признак                                                                  |
| `ContractDescription_StockCategorySign`   | `formalized.supplementary_contract_1.ContractDescription_StockCategorySign`   | системный признак                                                                  |
| `ContractDescription_BuyerLimitationSign` | `formalized.supplementary_contract_1.ContractDescription_BuyerLimitationSign` | системный признак                                                                  |
| `ContractDescription_InsuranceSign`       | `formalized.supplementary_contract_1.ContractDescription_InsuranceSign`       | системный признак                                                                  |
| `RussianPerson_OrganizationName`          | `formalized.supplementary_contract_1.RussianPerson_OrganizationName`          |                                                                                    |
| `RussianPerson_ShortName`                 | `formalized.supplementary_contract_1.RussianPerson_ShortName`                 |                                                                                    |
| `RussianPerson_OGRN`                      | `formalized.supplementary_contract_1.RussianPerson_OGRN`                      |                                                                                    |
| `RussianPerson_INN`                       | `formalized.supplementary_contract_1.RussianPerson_INN`                       |                                                                                    |
| `RussianPerson_KPP`                       | `formalized.supplementary_contract_1.RussianPerson_KPP`                       |                                                                                    |
| `RussianPerson_Address_PostalCode`        | `formalized.supplementary_contract_1.RussianPerson_Address_PostalCode`        |                                                                                    |
| `RussianPerson_Address_CountryCode`       | `formalized.supplementary_contract_1.RussianPerson_Address_CountryCode`       | alpha-2                                                                            |
| `RussianPerson_Address_CounryName`        | `formalized.supplementary_contract_1.RussianPerson_Address_CounryName`        | опечатка в теге: `CounryName`                                                      |
| `RussianPerson_Address_Region`            | `formalized.supplementary_contract_1.RussianPerson_Address_Region`            |                                                                                    |
| `RussianPerson_Address_City`              | `formalized.supplementary_contract_1.RussianPerson_Address_City`              |                                                                                    |
| `RussianPerson_Address_StreetHouse`       | `formalized.supplementary_contract_1.RussianPerson_Address_StreetHouse`       |                                                                                    |
| `ForeignPerson_OrganizationName`          | `formalized.supplementary_contract_1.ForeignPerson_OrganizationName`          |                                                                                    |
| `ForeignPerson_ShortName`                 | `formalized.supplementary_contract_1.ForeignPerson_ShortName`                 |                                                                                    |
| `ForeignPerson_Address_CountryCode`       | `formalized.supplementary_contract_1.ForeignPerson_Address_CountryCode`       | alpha-2                                                                            |
| `ForeignPerson_Address_CounryName`        | `formalized.supplementary_contract_1.ForeignPerson_Address_CounryName`        | опечатка в теге: `CounryName`                                                      |
| `ForeignPerson_Address_Region`            | `formalized.supplementary_contract_1.ForeignPerson_Address_Region`            |                                                                                    |
| `ForeignPerson_Address_City`              | `formalized.supplementary_contract_1.ForeignPerson_Address_City`              |                                                                                    |
| `ForeignPerson_Address_StreetHouse`       | `formalized.supplementary_contract_1.ForeignPerson_Address_StreetHouse`       |                                                                                    |

#### 2.1) ContractSignedPerson (вложенный блок)

| XML тег                                 | UQI                                                                         | Комментарий |
|-----------------------------------------|-----------------------------------------------------------------------------|-------------|
| `ContractSignedPerson/PersonSurname`    | `formalized.supplementary_contract_1.ContractSignedPerson.PersonSurname`    |             |
| `ContractSignedPerson/PersonName`       | `formalized.supplementary_contract_1.ContractSignedPerson.PersonName`       |             |
| `ContractSignedPerson/PersonMiddleName` | `formalized.supplementary_contract_1.ContractSignedPerson.PersonMiddleName` |             |

---

### 3) Invoice (04021) — AltaE2I

#### 3.1) Заголовок / реквизиты

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

### 3.2) InvoiceGoods (повторяющийся блок)

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

### 4) Packing List (04131) — AltaE2PACK

#### 4.1) Заголовок / реквизиты

| XML тег                                       | UQI                                                                     | Комментарий            |
|-----------------------------------------------|-------------------------------------------------------------------------|------------------------|
| `GrossWeightQuantity`                         | `formalized.packing_list_1.GrossWeightQuantity`                         | общий брутто           |
| `NetWeightQuantity`                           | `formalized.packing_list_1.NetWeightQuantity`                           | общий нетто            |
| `Consignor_OrganizationName`                  | `formalized.packing_list_1.Consignor_OrganizationName`                  |                        |
| `Consignor_ShortName`                         | `formalized.packing_list_1.Consignor_ShortName`                         |                        |
| `Consignor_Address_CountryCode`               | `formalized.packing_list_1.Consignor_Address_CountryCode`               | alpha-2                |
| `Consignor_Address_CounryName`                | `formalized.packing_list_1.Consignor_Address_CounryName`                | опечатка: `CounryName` |
| `Consignor_Address_Region`                    | `formalized.packing_list_1.Consignor_Address_Region`                    |                        |
| `Consignor_Address_City`                      | `formalized.packing_list_1.Consignor_Address_City`                      |                        |
| `Consignor_Address_StreetHouse`               | `formalized.packing_list_1.Consignor_Address_StreetHouse`               |                        |
| `Consignee_OrganizationName`                  | `formalized.packing_list_1.Consignee_OrganizationName`                  |                        |
| `Consignee_ShortName`                         | `formalized.packing_list_1.Consignee_ShortName`                         |                        |
| `Consignee_OGRN`                              | `formalized.packing_list_1.Consignee_OGRN`                              |                        |
| `Consignee_INN`                               | `formalized.packing_list_1.Consignee_INN`                               |                        |
| `Consignee_KPP`                               | `formalized.packing_list_1.Consignee_KPP`                               |                        |
| `Consignee_Address_PostalCode`                | `formalized.packing_list_1.Consignee_Address_PostalCode`                |                        |
| `Consignee_Address_CountryCode`               | `formalized.packing_list_1.Consignee_Address_CountryCode`               | alpha-2                |
| `Consignee_Address_CounryName`                | `formalized.packing_list_1.Consignee_Address_CounryName`                | опечатка: `CounryName` |
| `Consignee_Address_Region`                    | `formalized.packing_list_1.Consignee_Address_Region`                    |                        |
| `Consignee_Address_City`                      | `formalized.packing_list_1.Consignee_Address_City`                      |                        |
| `Consignee_Address_StreetHouse`               | `formalized.packing_list_1.Consignee_Address_StreetHouse`               |                        |
| `DeliveryTerms_DeliveryPlace`                 | `formalized.packing_list_1.DeliveryTerms_DeliveryPlace`                 |                        |
| `DeliveryTerms_DeliveryTermsNumericCode`      | `formalized.packing_list_1.DeliveryTerms_DeliveryTermsNumericCode`      |                        |
| `DeliveryTerms_DeliveryTermsStringCode`       | `formalized.packing_list_1.DeliveryTerms_DeliveryTermsStringCode`       |                        |
| `DeliveryTerms_Contract_PrDocumentName`       | `formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentName`       |                        |
| `DeliveryTerms_Contract_PrDocumentNumber`     | `formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentNumber`     |                        |
| `DeliveryTerms_Contract_PrDocumentDate`       | `formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentDate`       | `YYYY-MM-DD`           |
| `DeliveryTerms_Invoice_PrDocumentName`        | `formalized.packing_list_1.DeliveryTerms_Invoice_PrDocumentName`        |                        |
| `DeliveryTerms_Invoice_PrDocumentNumber`      | `formalized.packing_list_1.DeliveryTerms_Invoice_PrDocumentNumber`      |                        |
| `DeliveryTerms_Invoice_PrDocumentDate`        | `formalized.packing_list_1.DeliveryTerms_Invoice_PrDocumentDate`        | `YYYY-MM-DD`           |
| `DeliveryTerms_Registration_PrDocumentName`   | `formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentName`   |                        |
| `DeliveryTerms_Registration_PrDocumentNumber` | `formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentNumber` |                        |
| `DeliveryTerms_Registration_PrDocumentDate`   | `formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentDate`   | `YYYY-MM-DD`           |

#### 4.2) Goods (повторяющийся блок)

Правило: каждый `formalized.packing_list_1.Goods_[n]` → отдельный `<Goods>...</Goods>`.

| XML тег внутри `Goods`       | UQI                                                              | Комментарий                                  |
|------------------------------|------------------------------------------------------------------|----------------------------------------------|
| `GoodsDescription`           | `formalized.packing_list_1.Goods_[n].GoodsDescription`           | описание “грузовой строки”                   |
| `GoodsQuantity`              | `formalized.packing_list_1.Goods_[n].GoodsQuantity`              | кол-во мест/груз.единиц (не “кол-во товара”) |
| `GrossWeightQuantity`        | `formalized.packing_list_1.Goods_[n].GrossWeightQuantity`        |                                              |
| `NetWeightQuantity`          | `formalized.packing_list_1.Goods_[n].NetWeightQuantity`          |                                              |
| `PackingInfo/PakingQuantity` | `formalized.packing_list_1.Goods_[n].PackingInfo.PakingQuantity` | опечатка: `PakingQuantity`                   |

#### 4.3) TransportMeans (повторяющийся блок)

Правило: каждый `formalized.packing_list_1.TransportMeans_[n]` → отдельный `<TransportMeans>...</TransportMeans>`.

| XML тег внутри `TransportMeans` | UQI                                                            | Комментарий                   |
|---------------------------------|----------------------------------------------------------------|-------------------------------|
| `Number`                        | `formalized.packing_list_1.TransportMeans_[n].Number`          | номер ТС                      |
| `ModeCode`                      | `formalized.packing_list_1.TransportMeans_[n].ModeCode`        | код вида транспорта           |
| `NationalityCode`               | `formalized.packing_list_1.TransportMeans_[n].NationalityCode` |                               |
| `MoverIndicator`                | `formalized.packing_list_1.TransportMeans_[n].MoverIndicator`  | `true` тягач / `false` прицеп |

---

### 5) CMR (02015) — AltaE3CMR

#### 5.1) Заголовок / реквизиты

| XML тег                                     | UQI                                                          | Комментарий                |
|---------------------------------------------|--------------------------------------------------------------|----------------------------|
| `LanguageCode`                              | `formalized.cmr_1.LanguageCode`                              |                            |
| `CMR_Choice`                                | `formalized.cmr_1.CMR_Choice`                                | системный признак          |
| `RegistrationDocument_RegID`                | `formalized.cmr_1.RegistrationDocument_RegID`                | номер                      |
| `RegistrationDocument_DateInf`              | `formalized.cmr_1.RegistrationDocument_DateInf`              | `YYYY-MM-DD`               |
| `RegistrationDocument_Place`                | `formalized.cmr_1.RegistrationDocument_Place`                |                            |
| `TrakingCargo_TakingCargoDate`              | `formalized.cmr_1.TrakingCargo_TakingCargoDate`              | `YYYY-MM-DD`               |
| `TrakingCargo_TakingCargoPlace_CountryCode` | `formalized.cmr_1.TrakingCargo_TakingCargoPlace_CountryCode` | alpha-2                    |
| `TrakingCargo_TakingCargoPlace_CounryName`  | `formalized.cmr_1.TrakingCargo_TakingCargoPlace_CounryName`  | опечатка: `CounryName`     |
| `DeliveryPlace_CountryCode`                 | `formalized.cmr_1.DeliveryPlace_CountryCode`                 | alpha-2                    |
| `DeliveryPlace_CounryName`                  | `formalized.cmr_1.DeliveryPlace_CounryName`                  | опечатка: `CounryName`     |
| `DeliveryTerms_DeliveryPlace`               | `formalized.cmr_1.DeliveryTerms_DeliveryPlace`               |                            |
| `DeliveryTerms_DeliveryTermsStringCode`     | `formalized.cmr_1.DeliveryTerms_DeliveryTermsStringCode`     |                            |
| `GoodsQuantity`                             | `formalized.cmr_1.GoodsQuantity`                             | общее кол-во мест/упаковок |
| `CMRGoodsWeight_GrossWeightQuantity`        | `formalized.cmr_1.CMRGoodsWeight_GrossWeightQuantity`        | общий брутто               |
| `CMRTransport_PrimeMoverStateSignID`        | `formalized.cmr_1.CMRTransport_PrimeMoverStateSignID`        | тягач                      |
| `CMRTransport_TrailerStateSignID`           | `formalized.cmr_1.CMRTransport_TrailerStateSignID`           | прицеп                     |
| `Consignor_NameInf`                         | `formalized.cmr_1.Consignor_NameInf`                         |                            |
| `Consignor_ShortName`                       | `formalized.cmr_1.Consignor_ShortName`                       |                            |
| `Consignor_PostalAddress_CountryCode`       | `formalized.cmr_1.Consignor_PostalAddress_CountryCode`       | alpha-2                    |
| `Consignor_Address_CounryName`              | `formalized.cmr_1.Consignor_Address_CounryName`              | опечатка: `CounryName`     |
| `Consignor_Address_Region`                  | `formalized.cmr_1.Consignor_Address_Region`                  |                            |
| `Consignor_Address_City`                    | `formalized.cmr_1.Consignor_Address_City`                    |                            |
| `Consignor_Address_StreetHouse`             | `formalized.cmr_1.Consignor_Address_StreetHouse`             |                            |
| `Consignor_Guarantee_OrganizationName`      | `formalized.cmr_1.Consignor_Guarantee_OrganizationName`      |                            |
| `Consignor_Guarantee_ShortName`             | `formalized.cmr_1.Consignor_Guarantee_ShortName`             |                            |
| `Consignor_Guarantee_Address_CountryCode`   | `formalized.cmr_1.Consignor_Guarantee_Address_CountryCode`   | alpha-2                    |
| `Consignor_Guarantee_Address_CounryName`    | `formalized.cmr_1.Consignor_Guarantee_Address_CounryName`    | опечатка: `CounryName`     |
| `Consignor_Guarantee_Address_Region`        | `formalized.cmr_1.Consignor_Guarantee_Address_Region`        |                            |
| `Consignor_Guarantee_Address_City`          | `formalized.cmr_1.Consignor_Guarantee_Address_City`          |                            |
| `Consignor_Guarantee_Address_StreetHouse`   | `formalized.cmr_1.Consignor_Guarantee_Address_StreetHouse`   |                            |
| `Consignee_NameInf`                         | `formalized.cmr_1.Consignee_NameInf`                         |                            |
| `Consignee_ShortName`                       | `formalized.cmr_1.Consignee_ShortName`                       |                            |
| `Consignee_OGRNID`                          | `formalized.cmr_1.Consignee_OGRNID`                          |                            |
| `Consignee_INNID`                           | `formalized.cmr_1.Consignee_INNID`                           |                            |
| `Consignee_KPPCode`                         | `formalized.cmr_1.Consignee_KPPCode`                         |                            |
| `Consignee_PostalAddress_PostalCode`        | `formalized.cmr_1.Consignee_PostalAddress_PostalCode`        |                            |
| `Consignee_PostalAddress_CountryCode`       | `formalized.cmr_1.Consignee_PostalAddress_CountryCode`       | alpha-2                    |
| `Consignee_Address_CounryName`              | `formalized.cmr_1.Consignee_Address_CounryName`              | опечатка: `CounryName`     |
| `Consignee_Address_Region`                  | `formalized.cmr_1.Consignee_Address_Region`                  |                            |
| `Consignee_Address_City`                    | `formalized.cmr_1.Consignee_Address_City`                    |                            |
| `Consignee_Address_StreetHouse`             | `formalized.cmr_1.Consignee_Address_StreetHouse`             |                            |

#### 5.2) CMRGoods (повторяющийся блок)

Правило: каждый `formalized.cmr_1.CMRGoods_[n]` → отдельный `<CMRGoods>...</CMRGoods>`.

| XML тег внутри `CMRGoods` | UQI                                                   | Комментарий          |
|---------------------------|-------------------------------------------------------|----------------------|
| `GoodsDescription`        | `formalized.cmr_1.CMRGoods_[n].GoodsDescription`      |                      |
| `GoodsNumeric`            | `formalized.cmr_1.CMRGoods_[n].GoodsNumeric`          | номер строки         |
| `GoodsNomenclatureCode`   | `formalized.cmr_1.CMRGoods_[n].GoodsNomenclatureCode` | по смыслу ТН ВЭД     |
| `GoodsQuantity`           | `formalized.cmr_1.CMRGoods_[n].GoodsQuantity`         | кол-во мест/упаковок |
| `GrossWeightQuantity`     | `formalized.cmr_1.CMRGoods_[n].GrossWeightQuantity`   | брутто по строке     |

#### 5.3) GoodsPackingInfo (вложенный блок внутри CMRGoods)

| XML тег                               | UQI                                                                 | Комментарий                |
|---------------------------------------|---------------------------------------------------------------------|----------------------------|
| `GoodsPackingInfo/PackingCode`        | `formalized.cmr_1.CMRGoods_[n].GoodsPackingInfo.PackingCode`        |                            |
| `GoodsPackingInfo/PakingQuantity`     | `formalized.cmr_1.CMRGoods_[n].GoodsPackingInfo.PakingQuantity`     | опечатка: `PakingQuantity` |
| `GoodsPackingInfo/PackingDescription` | `formalized.cmr_1.CMRGoods_[n].GoodsPackingInfo.PackingDescription` |                            |

---

### 6) Payment Order (04023) — AltaPaymentOrder

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

#### 6.1) PayerSign (вложенный блок)

| XML тег                   | UQI                                                  | Комментарий |
|---------------------------|------------------------------------------------------|-------------|
| `PayerSign/PersonSurname` | `formalized.payment_order_1.PayerSign.PersonSurname` |             |
| `PayerSign/PersonName`    | `formalized.payment_order_1.PayerSign.PersonName`    |             |

---

### 7) Service Invoice (04031) — AltaServiceInvoice

#### 7.1) Заголовок / реквизиты

| XML тег                                                          | UQI                                                                                           | Комментарий      |
|------------------------------------------------------------------|-----------------------------------------------------------------------------------------------|------------------|
| `DocumentSign`                                                   | `formalized.service_invoice_1.DocumentSign`                                                   |                  |
| `TotalServiceCost`                                               | `formalized.service_invoice_1.TotalServiceCost`                                               |                  |
| `Currency`                                                       | `formalized.service_invoice_1.Currency`                                                       | ISO 4217 alpha-3 |
| `ServiceProvider_Name`                                           | `formalized.service_invoice_1.ServiceProvider_Name`                                           |                  |
| `ContractDetails_PrDocumentNumber`                               | `formalized.service_invoice_1.ContractDetails_PrDocumentNumber`                               |                  |
| `ContractDetails_PrDocumentDate`                                 | `formalized.service_invoice_1.ContractDetails_PrDocumentDate`                                 | `YYYY-MM-DD`     |
| `Registration_PrDocumentName`                                    | `formalized.service_invoice_1.Registration_PrDocumentName`                                    |                  |
| `Registration_PrDocumentNumber`                                  | `formalized.service_invoice_1.Registration_PrDocumentNumber`                                  |                  |
| `Registration_PrDocumentDate`                                    | `formalized.service_invoice_1.Registration_PrDocumentDate`                                    | `YYYY-MM-DD`     |
| `Consignor_OrganizationName`                                     | `formalized.service_invoice_1.Consignor_OrganizationName`                                     |                  |
| `Consignee_OrganizationName`                                     | `formalized.service_invoice_1.Consignee_OrganizationName`                                     |                  |
| `Consignee_RFOrganizationFeatures_OGRN`                          | `formalized.service_invoice_1.Consignee_RFOrganizationFeatures_OGRN`                          |                  |
| `Consignee_RFOrganizationFeatures_INN`                           | `formalized.service_invoice_1.Consignee_RFOrganizationFeatures_INN`                           |                  |
| `Consignee_RFOrganizationFeatures_KPP`                           | `formalized.service_invoice_1.Consignee_RFOrganizationFeatures_KPP`                           |                  |
| `Signature_Choice`                                               | `formalized.service_invoice_1.Signature_Choice`                                               |                  |
| `SignatureDirectorChiefAccountant_Director_PersonSurname`        | `formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonSurname`        |                  |
| `SignatureDirectorChiefAccountant_Director_PersonName`           | `formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonName`           |                  |
| `SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname` | `formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname` |                  |
| `SignatureDirectorChiefAccountant_ChiefAccountant_PersonName`    | `formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonName`    |                  |

#### 7.2) ServiceProvider_PaymentRequisitions (вложенный блок)

| XML тег                                        | UQI                                                                         | Комментарий |
|------------------------------------------------|-----------------------------------------------------------------------------|-------------|
| `ServiceProvider_PaymentRequisitions/BankName` | `formalized.service_invoice_1.ServiceProvider_PaymentRequisitions.BankName` |             |

#### 7.3) PaymentDocument (вложенный блок)

| XML тег                            | UQI                                                             | Комментарий  |
|------------------------------------|-----------------------------------------------------------------|--------------|
| `PaymentDocument/PrDocumentNumber` | `formalized.service_invoice_1.PaymentDocument.PrDocumentNumber` |              |
| `PaymentDocument/PrDocumentDate`   | `formalized.service_invoice_1.PaymentDocument.PrDocumentDate`   | `YYYY-MM-DD` |

#### 7.4) Consignor_SubjectAddressDetails (вложенный блок)

| XML тег                                       | UQI                                                                        | Комментарий            |
|-----------------------------------------------|----------------------------------------------------------------------------|------------------------|
| `Consignor_SubjectAddressDetails/PostalCode`  | `formalized.service_invoice_1.Consignor_SubjectAddressDetails.PostalCode`  |                        |
| `Consignor_SubjectAddressDetails/CountryCode` | `formalized.service_invoice_1.Consignor_SubjectAddressDetails.CountryCode` | alpha-2                |
| `Consignor_SubjectAddressDetails/CounryName`  | `formalized.service_invoice_1.Consignor_SubjectAddressDetails.CounryName`  | опечатка: `CounryName` |
| `Consignor_SubjectAddressDetails/Region`      | `formalized.service_invoice_1.Consignor_SubjectAddressDetails.Region`      |                        |
| `Consignor_SubjectAddressDetails/Town`        | `formalized.service_invoice_1.Consignor_SubjectAddressDetails.Town`        |                        |
| `Consignor_SubjectAddressDetails/StreetHouse` | `formalized.service_invoice_1.Consignor_SubjectAddressDetails.StreetHouse` |                        |

#### 7.5) Consignee_SubjectAddressDetails (вложенный блок)

| XML тег                                       | UQI                                                                        | Комментарий            |
|-----------------------------------------------|----------------------------------------------------------------------------|------------------------|
| `Consignee_SubjectAddressDetails/PostalCode`  | `formalized.service_invoice_1.Consignee_SubjectAddressDetails.PostalCode`  |                        |
| `Consignee_SubjectAddressDetails/CountryCode` | `formalized.service_invoice_1.Consignee_SubjectAddressDetails.CountryCode` | alpha-2                |
| `Consignee_SubjectAddressDetails/CounryName`  | `formalized.service_invoice_1.Consignee_SubjectAddressDetails.CounryName`  | опечатка: `CounryName` |
| `Consignee_SubjectAddressDetails/Region`      | `formalized.service_invoice_1.Consignee_SubjectAddressDetails.Region`      |                        |
| `Consignee_SubjectAddressDetails/Town`        | `formalized.service_invoice_1.Consignee_SubjectAddressDetails.Town`        |                        |
| `Consignee_SubjectAddressDetails/StreetHouse` | `formalized.service_invoice_1.Consignee_SubjectAddressDetails.StreetHouse` |                        |
| `Consignee_SubjectAddressDetails/House`       | `formalized.service_invoice_1.Consignee_SubjectAddressDetails.House`       |                        |
| `Consignee_SubjectAddressDetails/Room`        | `formalized.service_invoice_1.Consignee_SubjectAddressDetails.Room`        |                        |

#### 7.6) ServiceDescription (повторяющийся блок)

Правило: каждый `formalized.service_invoice_1.ServiceDescription_[n]` → отдельный `<ServiceDescription>...</ServiceDescription>`.

| XML тег внутри `ServiceDescription` | UQI                                                                        | Комментарий      |
|-------------------------------------|----------------------------------------------------------------------------|------------------|
| `GoodsDescription`                  | `formalized.service_invoice_1.ServiceDescription_[n].GoodsDescription`     | текст            |
| `CurrencyCode`                      | `formalized.service_invoice_1.ServiceDescription_[n].CurrencyCode`         | ISO 4217 alpha-3 |
| `ServiceName`                       | `formalized.service_invoice_1.ServiceDescription_[n].ServiceName`          |                  |
| `TaxRate`                           | `formalized.service_invoice_1.ServiceDescription_[n].TaxRate`              |                  |
| `TaxSum`                            | `formalized.service_invoice_1.ServiceDescription_[n].TaxSum`               |                  |
| `ServiceCost_Amount`                | `formalized.service_invoice_1.ServiceDescription_[n].ServiceCost_Amount`   |                  |
| `ServiceCost_Currency`              | `formalized.service_invoice_1.ServiceDescription_[n].ServiceCost_Currency` | ISO 4217 alpha-3 |

