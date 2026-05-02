# doc_xml_schema.md — Stage 1.1 (Формализация: primary.md → XML)

## Назначение
Этот документ задаёт правила генерации XML формализованных документов для импорта в Альту.

## 1. Базовые принципы преобразования

1. **Маппинг 1:1:** Для документов раздела `formalized` в `primary.md` (в блоке `fields`) имена полей полностью 
   совпадают с именами XML-тегов.
2. **Кодировка:** Все файлы генерируются строго в `windows-1251`. Декларация: `<?xml version="1.0" encoding="windows-1251"?>`.
3. **Экранирование:** Все текстовые значения должны быть XML-экранированы (`&amp;`, `&lt;`, `&gt;` и т.д.).
4. **Даты:** Приведение к формату `YYYY-MM-DD`.
5. **Числа:** Числовые поля писать как строковое представление числа **без принудительного округления**:
   - если в первичке есть десятичная часть — сохранять;
   - если десятичной части нет — писать целым (без `.00`), если нет отдельного требования.
6. **Русский вариант текста:** если обозначения, названия или просто текст представлены на нескольких языках,
   использовать русский вариант.


### 1.1. Запрет на генерацию новых фактов
1. Если поле в `primary.md` имеет `status: pending` — значение в XML не генерировать.
2. Этап 1.1 не исправляет `primary.md`; при блокерах — остановиться и сформировать `doc_xml_review.md`.

## 2. Работа с объемными данными (линки)
- Помещаемый в XML-тег текст **НЕ ДОЛЖЕН СОДЕРЖАТЬ СИМВОЛОВ `&`, `<`, `>`!!!**
- если текст представлен на нескольких языках, использовать русский вариант.

Если в `primary.md` поле содержит `value: link:<имя_файла>`, AI обязан:
1. Прочесть документ (путь указан в заголовке документа в `primary.md`), команда Хобота 
   `read_file <путь к файлу> <utf-t> <xml_escape>` - **С XML ЭКРАНИРОВКОЙ!**
2. Если документа нет или он содержит неполную информацию:
  - перетащить указанный первичный документ, предпочитая форматы в следующем порядке: docx → png → pdf,
  - на его основе сформировать `md` документ **БЕЗ СОКРАЩЕНИЙ И ПОТЕРЬ**,
  - записать его в `alta\prompt\stage_0\doc_conversion_schema.md`
  - исполнять, начиная с пункта 1.
3. При подстановке в XML-тег тексты брать полностью. **ЗАПРЕЩЕНО СОКРАЩАТЬ / ДЕЛАТЬ ВЫЖИМКИ ТЕКСТОВ**.

**Типовые поля для линков:**
- `ContractTerms_ContractText`
- `DocumentBody_TextSection` (TextPara)
- `Subject` (доверенность)
- `DocumentBody_FileData` (для FreeBinaryDoc — конвертация файла в base64).

## 3. Структурные правила

- **Скаляры:** `FieldName` → `<FieldName>value</FieldName>`
- **Объекты:** `ObjName` (таблица полей) → `<ObjName><Field>...</Field></ObjName>`
- **Массивы:** `TagName_[n]` → Повторяющиеся узлы `<TagName>...</TagName>`. Суффикс `_[n]` в XML не пишется.

## 4. Корневые типы (Stage 1)
Генератор должен поддерживать XML по `xml_target_root` из `primary.md`:

- `AltaE2CONT` (Contract 03011)
- `AltaSupplementaryContract` (Supplementary Contract 03012)
- `AltaE2I` (Invoice 04021) + повторяющийся блок `InvoiceGoods`
- `AltaE2PACK` (Packing List 04131) + повторяющиеся блоки `Goods`, `TransportMeans`
- `AltaE3CMR` (CMR 02015) + повторяющийся блок `CMRGoods` (внутри `GoodsPackingInfo`)
- `AltaPaymentOrder` (Payment Order 04023) + вложенный блок `PayerSign`
- `AltaServiceInvoice` (Service Invoice 04031) + повторяющийся блок `ServiceDescription` + вложенные блоки реквизитов
- `AltaFreeDoc` (текстовые документы: 04011/04033/04111/05999/09999) + вложенный блок `DocumentBody_TextSection`
    + повторяющийся `TextPara`
- `AltaPassport` (Passport 11001)
- `AltaLetterOfAttorney` (LetterOfAttorney 11004)
- `AltaFreeBinaryDoc` (если используется) — правила см. раздел 3.2

## 5. Выходные файлы
Генерируемые XML-файлы сохраняются в:
`alta\stage_1.1_result\<case>\formalized_docs\`

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

---

### 8) AltaFreeDoc (04111/05999/09999/04033/04011) — общая структура

Для документов с `xml_target_root = AltaFreeDoc` структура одинаковая:
`DocumentCode`, `DocumentHead_*`, `DocumentBody_TextSection/TextPara`.

#### 8.1) Insurance document (04111)

| XML тег                       | UQI                                                           | Комментарий  |
|-------------------------------|---------------------------------------------------------------|--------------|
| `DocumentCode`                | (константа)                                                   | `04111`      |
| `DocumentHead_DocumentName`   | `formalized.insurance_document_1.DocumentHead_DocumentName`   |              |
| `DocumentHead_DocumentDate`   | `formalized.insurance_document_1.DocumentHead_DocumentDate`   | `YYYY-MM-DD` |
| `DocumentHead_DocumentNumber` | `formalized.insurance_document_1.DocumentHead_DocumentNumber` |              |

TextPara: см. 8.5.

#### 8.2) TechDescription (05999)

| XML тег                       | UQI                                                         | Комментарий  |
|-------------------------------|-------------------------------------------------------------|--------------|
| `DocumentCode`                | (константа)                                                 | `05999`      |
| `DocumentHead_DocumentName`   | `formalized.tech_description_1.DocumentHead_DocumentName`   |              |
| `DocumentHead_DocumentDate`   | `formalized.tech_description_1.DocumentHead_DocumentDate`   | `YYYY-MM-DD` |
| `DocumentHead_DocumentNumber` | `formalized.tech_description_1.DocumentHead_DocumentNumber` |              |

TextPara: см. 8.5.

#### 8.3) FreeDoc (09999)

| XML тег                       | UQI                                                 | Комментарий  |
|-------------------------------|-----------------------------------------------------|--------------|
| `DocumentCode`                | (константа)                                         | `09999`      |
| `DocumentHead_DocumentName`   | `formalized.free_doc_1.DocumentHead_DocumentName`   |              |
| `DocumentHead_DocumentDate`   | `formalized.free_doc_1.DocumentHead_DocumentDate`   | `YYYY-MM-DD` |
| `DocumentHead_DocumentNumber` | `formalized.free_doc_1.DocumentHead_DocumentNumber` |              |

TextPara: см. 8.5.

#### 8.4) Transport Contract (04033)

| XML тег                       | UQI                                                           | Комментарий  |
|-------------------------------|---------------------------------------------------------------|--------------|
| `DocumentCode`                | (константа)                                                   | `04033`      |
| `DocumentHead_DocumentName`   | `formalized.transport_contract_1.DocumentHead_DocumentName`   |              |
| `DocumentHead_DocumentDate`   | `formalized.transport_contract_1.DocumentHead_DocumentDate`   | `YYYY-MM-DD` |
| `DocumentHead_DocumentNumber` | `formalized.transport_contract_1.DocumentHead_DocumentNumber` |              |

TextPara: см. 8.5.

#### 8.5) EGRUL (04011)

| XML тег                       | UQI                                              | Комментарий  |
|-------------------------------|--------------------------------------------------|--------------|
| `DocumentCode`                | (константа)                                      | `04011`      |
| `DocumentHead_DocumentName`   | `formalized.egrul_1.DocumentHead_DocumentName`   |              |
| `DocumentHead_DocumentDate`   | `formalized.egrul_1.DocumentHead_DocumentDate`   | `YYYY-MM-DD` |
| `DocumentHead_DocumentNumber` | `formalized.egrul_1.DocumentHead_DocumentNumber` |              |

#### 8.6) DocumentBody_TextSection/TextPara (повторяющийся блок)

Правило: каждый `formalized.<free_doc_type>_1.DocumentBody_TextSection.TextPara_[n]`
→ отдельный `<TextPara>...</TextPara>` внутри `<DocumentBody_TextSection>`.

| XML тег                             | UQI                                                        | Комментарий                                                                        |
|-------------------------------------|------------------------------------------------------------|------------------------------------------------------------------------------------|
| `DocumentBody_TextSection/TextPara` | `formalized.<...>_1.DocumentBody_TextSection.TextPara_[n]` | если в `primary.md` хранится `link` — прочитать файл и вставить текст (XML-escape) |

---

### 9) FreeBinaryDoc — AltaFreeBinaryDoc

| XML тег                         | UQI                                                          | Комментарий                                                                       |
|---------------------------------|--------------------------------------------------------------|-----------------------------------------------------------------------------------|
| `DocumentCode`                  | `formalized.free_binary_doc_1.DocumentCode`                  |                                                                                   |
| `DocumentInfo_PrDocumentName`   | `formalized.free_binary_doc_1.DocumentInfo_PrDocumentName`   |                                                                                   |
| `DocumentInfo_PrDocumentNumber` | `formalized.free_binary_doc_1.DocumentInfo_PrDocumentNumber` |                                                                                   |
| `DocumentInfo_PrDocumentDate`   | `formalized.free_binary_doc_1.DocumentInfo_PrDocumentDate`   | `YYYY-MM-DD`                                                                      |
| `DocumentBody_FileName`         | `formalized.free_binary_doc_1.DocumentBody_FileName`         | имя файла                                                                         |
| `DocumentBody_FileData`         | `formalized.free_binary_doc_1.DocumentBody_FileData`         | в `primary.md` хранить `link`; при генерации XML прочитать файл и вставить base64 |
| `Thumbnail`                     | `formalized.free_binary_doc_1.Thumbnail`                     | если используется — обычно тоже `link`→base64                                     |

---

### 10) Passport (11001) — AltaPassport

| XML тег                       | UQI                                                 | Комментарий                               |
|-------------------------------|-----------------------------------------------------|-------------------------------------------|
| `CardSeries`                  | `formalized.passport_1.CardSeries`                  |                                           |
| `CardNumber`                  | `formalized.passport_1.CardNumber`                  |                                           |
| `OrganizationName`            | `formalized.passport_1.OrganizationName`            | кем выдан                                 |
| `CardDate`                    | `formalized.passport_1.CardDate`                    | `YYYY-MM-DD`                              |
| `PersonInfo_PersonSurname`    | `formalized.passport_1.PersonInfo_PersonSurname`    |                                           |
| `PersonInfo_PersonName`       | `formalized.passport_1.PersonInfo_PersonName`       |                                           |
| `PersonInfo_PersonMiddleName` | `formalized.passport_1.PersonInfo_PersonMiddleName` |                                           |
| `PersonInfo_Sex`              | `formalized.passport_1.PersonInfo_Sex`              | 1/2                                       |
| `PersonInfo_Birthday`         | `formalized.passport_1.PersonInfo_Birthday`         | `YYYY-MM-DD`                              |
| `PersonInfo_Birthplace`       | `formalized.passport_1.PersonInfo_Birthplace`       |                                           |
| `ResidencePlace_PostalCode`   | `formalized.passport_1.ResidencePlace_PostalCode`   |                                           |
| `ResidencePlace_CountryCode`  | `formalized.passport_1.ResidencePlace_CountryCode`  | alpha-2                                   |
| `ResidencePlace_CounryName`   | `formalized.passport_1.ResidencePlace_CounryName`   | если используется; опечатка: `CounryName` |
| `ResidencePlace_Region`       | `formalized.passport_1.ResidencePlace_Region`       |                                           |
| `ResidencePlace_City`         | `formalized.passport_1.ResidencePlace_City`         |                                           |
| `ResidencePlace_StreetHouse`  | `formalized.passport_1.ResidencePlace_StreetHouse`  |                                           |

---

### 11) Letter of Attorney (11004) — AltaLetterOfAttorney

| XML тег                                            | UQI                                                                                | Комментарий                                                                        |
|----------------------------------------------------|------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|
| `Subject`                                          | `formalized.letter_of_attorney_1.Subject`                                          | если в `primary.md` хранится `link` — прочитать файл и вставить текст (XML-escape) |
| `EndDate`                                          | `formalized.letter_of_attorney_1.EndDate`                                          | `YYYY-MM-DD`                                                                       |
| `DocumentReference_PrDocumentName`                 | `formalized.letter_of_attorney_1.DocumentReference_PrDocumentName`                 |                                                                                    |
| `DocumentReference_PrDocumentNumber`               | `formalized.letter_of_attorney_1.DocumentReference_PrDocumentNumber`               |                                                                                    |
| `DocumentReference_PrDocumentDate`                 | `formalized.letter_of_attorney_1.DocumentReference_PrDocumentDate`                 | `YYYY-MM-DD`                                                                       |
| `Organization_OrganizationName`                    | `formalized.letter_of_attorney_1.Organization_OrganizationName`                    |                                                                                    |
| `Organization_ShortName`                           | `formalized.letter_of_attorney_1.Organization_ShortName`                           |                                                                                    |
| `Organization_OGRN`                                | `formalized.letter_of_attorney_1.Organization_OGRN`                                |                                                                                    |
| `Organization_INN`                                 | `formalized.letter_of_attorney_1.Organization_INN`                                 |                                                                                    |
| `Organization_KPP`                                 | `formalized.letter_of_attorney_1.Organization_KPP`                                 |                                                                                    |
| `Organization_Address_PostalCode`                  | `formalized.letter_of_attorney_1.Organization_Address_PostalCode`                  |                                                                                    |
| `Organization_Address_CountryCode`                 | `formalized.letter_of_attorney_1.Organization_Address_CountryCode`                 | alpha-2                                                                            |
| `Organization_Address_CounryName`                  | `formalized.letter_of_attorney_1.Organization_Address_CounryName`                  | если используется; опечатка: `CounryName`                                          |
| `Organization_Address_Region`                      | `formalized.letter_of_attorney_1.Organization_Address_Region`                      |                                                                                    |
| `Organization_Address_City`                        | `formalized.letter_of_attorney_1.Organization_Address_City`                        |                                                                                    |
| `Organization_Address_StreetHouse`                 | `formalized.letter_of_attorney_1.Organization_Address_StreetHouse`                 |                                                                                    |
| `Organization_OrganizationPerson_PersonSurname`    | `formalized.letter_of_attorney_1.Organization_OrganizationPerson_PersonSurname`    |                                                                                    |
| `Organization_OrganizationPerson_PersonName`       | `formalized.letter_of_attorney_1.Organization_OrganizationPerson_PersonName`       |                                                                                    |
| `Organization_OrganizationPerson_PersonMiddleName` | `formalized.letter_of_attorney_1.Organization_OrganizationPerson_PersonMiddleName` |                                                                                    |
| `Organization_OrganizationPerson_PersonPost`       | `formalized.letter_of_attorney_1.Organization_OrganizationPerson_PersonPost`       |                                                                                    |
| `EmpoweredPerson_PersonSurname`                    | `formalized.letter_of_attorney_1.EmpoweredPerson_PersonSurname`                    |                                                                                    |
| `EmpoweredPerson_PersonName`                       | `formalized.letter_of_attorney_1.EmpoweredPerson_PersonName`                       |                                                                                    |
| `EmpoweredPerson_PersonMiddleName`                 | `formalized.letter_of_attorney_1.EmpoweredPerson_PersonMiddleName`                 |                                                                                    |
| `EmpoweredPerson_PersonPost`                       | `formalized.letter_of_attorney_1.EmpoweredPerson_PersonPost`                       |                                                                                    |
| `EmpoweredPerson_Passport_IdentityCardCode`        | `formalized.letter_of_attorney_1.EmpoweredPerson_Passport_IdentityCardCode`        |                                                                                    |
| `EmpoweredPerson_Passport_IdentityCardName`        | `formalized.letter_of_attorney_1.EmpoweredPerson_Passport_IdentityCardName`        |                                                                                    |
| `EmpoweredPerson_Passport_IdentityCardSeries`      | `formalized.letter_of_attorney_1.EmpoweredPerson_Passport_IdentityCardSeries`      |                                                                                    |
| `EmpoweredPerson_Passport_IdentityCardNumber`      | `formalized.letter_of_attorney_1.EmpoweredPerson_Passport_IdentityCardNumber`      |                                                                                    |
| `EmpoweredPerson_Passport_IdentityCardDate`        | `formalized.letter_of_attorney_1.EmpoweredPerson_Passport_IdentityCardDate`        | `YYYY-MM-DD`                                                                       |
| `EmpoweredPerson_Passport_OrganizationName`        | `formalized.letter_of_attorney_1.EmpoweredPerson_Passport_OrganizationName`        |                                                                                    |

## 6. РАЗДЕЛ IV: Порядок работы

### 6.1. Подготовка
- Прочитать `alta\stage_1.0_result\<кейс>\primary.md` (источник истины этапа 1.1).
- Убедиться, что в `primary.md` для документов formalized нет полей со статусом pending, необходимых для генерации 
  соответствующего XML. Если есть — остановить этап и сообщить оператору (использовать issues из `primary.md`).

### 6.2. Генерация XML
- Для каждого документа из `primary.md/formalized` создать отдельный XML-файл.
- Если в поле встретился `link`, выполнить действия из Раздела 2 данного промпта.
- Использовать команду Хобота `write_file` с параметром кодировки `windows-1251` (3-й параметр команды).

### 6.3. Верификация (семантическая)
После создания файлов AI **обязан проверить**:
- Соответствие корневого тега значению `xml_target_root` из `primary.md` (для данного документа).
- Полноту переноса данных: все ли поля из `primary.md/fields` (для данного документа) попали в XML (с учетом 
  правил скаляров/объектов/массивов).
- Корректность кодировки (отсутствие "кракозябр" при чтении записанного файла).

### 6.4. Отчетность
- Сгенерировать отчет `doc_xml_review.md` в папке `alta\stage_1.1_result\<кейс>\`.
- Указать список созданных файлов и возникшие трудности при разрешении линков (если были).

