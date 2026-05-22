# Инструкция по этапу 1.0: Сбор и нормализация фактов

## 0. Разделы файла `primary_schema.md`

1. Раздел 1. Workflow

2. Раздел 2. Общие правила шаблонов

3. Раздел 3. `formalized` (Шаблоны формализуемых документов)

4. Раздел 4. `non_formalized` (Шаблоны неформализуемых документов)

5. Раздел 5. Формат `primary.md`

6. Раздел 6. Правила работы

7. Раздел 7. Порядок работы (задание)

8. ПРИЛОЖЕНИЕ. Вырезки из справочников


## Раздел 1. Workflow

### 1.1 Вход этапа:
- `alta\source\<кейс>\...\<папка первички>\md\*.md` - факты поставки (приведенная к `md` первичка),
- `alta\master_data\*` (declarant, representative, letter_of_attorney, transport_contract),
- `alta\stable_source\*` (формализованные xml-документы кодировка cp1251, не меняющиеся между поставками; нужны только
   для генерации линков на объемные тексты),
- `alta\source\<кейс>\...\<папка первички>\operator\` (Опционально. Данные, предоставленные оператором).
- `alta\prompt\codebook.md` - полные справочники.

### 1.2 Выход этапа:
- `primary.md` (источник истины для этапов 1.1, 2.0),
- `primary_review.md` — краткий отчет.

### 1.3 Обработка:
- `primary.md` служит входом для двух разных обработок - генерации формализованных документов (этап 1.1) и генерации
  данных для ДТ (этап 2.0). Поэтому, данная схема содержит две группы шаблонов, формализованные и не-формализованные 
  документы:
  - Формализованные служат входом как для обоих 1.1 и 2.0.
  - Не формализаванные - только для 2.0.
- Согласно шаблонам и формату `primary.md` генерирует `primary.md`. 
- `primary.md` - это НЕ рекомендации, а формальная база данных, построенная по жестким правилам.
- По результату генерации `primary.md` строит отчет `primary_review.md`.

## Раздел 2. Общие правила шаблонов

- Идентификаторы документов, например, `formalized.invoice_1`, `non_formalized.svh_1` (`[раздел].[тип_документа]_[n]`),
  используются как `uqi_prefix` для ссылок на поля: `<uqi_prefix>.<имя_поля>`, `<uqi_prefix>.<имя_массива>_[n].<имя_поля>`.

- **Формат поля документа в схеме:** занимает одну строку. Первым идет идентификатор, далее, в скобках, пояснения.

- **Нумерация (жесткие индексы полей):** 
  - все поля в шаблонах пронумерованы в формате `NN: field_name`, начиная с 1 без 
    пропусков. Список полей замыкает строка`_audit: n`, где n - число полей документа.
  - Индексы полей совпадают 1:1 в схеме и `primary.md`.
  - В *.md используются индексы смысловых блоков, но они НЕЗАВИСИМЫ от индексов полей в схеме/`primary.md`.

- **Массивы:** в шаблоне идут ПОСЛЕ полей. Их поля имеют свою нумерацию и аудит полей элементов и самого массива: 
  - Каждый элемент завершает строка `_item_audit: n`, где n - число полей в элементе массива.

- **Объемные тексты:** Если текст слишком большой (техописание, контракт), в `primary.md` укажи
  `value: link:<путь к md-версии документа в папке md>`. Полный текст будет подставлен на этапе 1.1.
  Если md-версия недоступна, можно указывать другие форматы документов. Приоритет: `md → docx / xlsx → png → pdf`.

- **Реквизиты:** Максимально разбивать адреса на компоненты. Если надежно разбить нельзя — сохранять исходную строку
  в соответствующее поле и назначать статус `pending`.

-  **`для сверок`** - означает, что поля могут быть дублированы в разных документах и должны быть идентичными.

## Раздел 3. `formalized` (Шаблоны формализуемых документов)

### 3.0 Правила формализованных шаблонов
На их базе будут строиться xml-файлы для импорта в Альту. 

Типы полей:
- Формализуемые поля. Их имена совпадают с целевыми XML тегами и записаны в SnakeCase. Используются для генерации XML. 
- Неформализуемые поля. Имена записаны в snake_case. Используются только как вход в этап 2.0.
- В конце каждого документа добавлены четыре неформализуемых поля. Они нужны на этапе 2.0 для заполнения 44 графы.
  - nn: `doc_code` (<код документа> — константа; G44/G441)
  - nn: `doc_name` (<НАИМЕНОВАНИЕ ДОКУМЕНТА> — константа; G44/G444)
  - nn: `doc_number` (= <копируемое поле>; G44/G442)
  - nn: `doc_date` (= <копируемое поле>; G44/G443)

#### 3.1 Contract / Контракт (03011)

- **xml_target_root:** `AltaE2CONT`
- **uqi_prefix:** `formalized.contract_[n]`

- **Поля:**
  - 01: `DocumentCode` (03011 — код вида документа для графы 44: G44/G441; константа; derived)
  - 02: `ContractRegistration_PrDocumentNumber` (№ контракта; графа 44: G44/G442)
  - 03: `ContractRegistration_PrDocumentDate` (дата контракта; графа 44: G44/G443)

  - 04: `ContractTerms_Amount` (общая сумма контракта; для контроля/сверки; в dt.xml обычно напрямую не печатается)
  - 05: `ContractTerms_CurrencyCode` (цифровой код валюты ISO 4217 numeric; для контроля/сверки; напр. CNY=156)
  - 06: `ContractTerms_LastDate` (срок действия/исполнения; для контроля/сверки)
  - 07: `ContractTerms_OtherTerms` (условия поставки / Incoterms, напр. `EXW ...`; источник для графы 20: G_20_1)
  - 08: `ContractTerms_ContractText` (текст контракта; в primary.md хранить `link` на файл-источник)
  - 09: `ContractTerms_DealSign` (`1` - системный признак Альты; для импорта; derived)

  - 10: `ForeignPerson_OrganizationName` (продавец/сторона контракта; обычно совпадает с отправителем; может использоваться
    для сверок)
  - 11: `ForeignPerson_Address_CountryCode` (страна продавца alpha-2 из `cb:country`; derived)
  - 12: `ForeignPerson_Address_CounryName` (страна продавца, текст; **опечатка тега CounryName**;)
  - 13: `ForeignPerson_Address_Region` (регион/область продавца; для сверок)
  - 14: `ForeignPerson_Address_City` (город/район продавца; для сверок)
  - 15: `ForeignPerson_Address_StreetHouse` (улица/дом продавца одной строкой; для сверок)

  - 16: `RussianPerson_OrganizationName` (покупатель/сторона контракта; обычно совпадает с декларантом/получателем; для сверок)
  - 17: `RussianPerson_OGRN` (ОГРН покупателя; для сверок/мастер-данных)
  - 18: `RussianPerson_INN` (ИНН покупателя; для сверок/мастер-данных)
  - 19: `RussianPerson_KPP` (КПП покупателя; для сверок/мастер-данных)
  - 20: `RussianPerson_Address_PostalCode` (индекс покупателя; для сверок/мастер-данных)
  - 21: `RussianPerson_Address_CountryCode` (страна покупателя alpha-2; для сверок/мастер-данных)
  - 22: `RussianPerson_Address_CounryName` (страна покупателя, текст; **опечатка тега CounryName**; для сверок/мастер-данных)
  - 23: `RussianPerson_Address_Region` (регион покупателя; для сверок/мастер-данных)
  - 24: `RussianPerson_Address_City` (город покупателя; для сверок/мастер-данных)
  - 25: `RussianPerson_Address_StreetHouse` (улица/дом/офис одной строкой; для сверок/мастер-данных)

- **Неформализуемые поля:**
  - 26: `doc_code` (03011 — константа)
  - 27: `doc_name` (`КОНТРАКТ` — константа)
  - 28: `doc_number` (= `ContractRegistration_PrDocumentNumber`)
  - 29: `doc_date` (= `ContractRegistration_PrDocumentDate`)

- _audit: 29

**Примечание:**
`ContractTerms_ContractText` в `primary.md` не копировать полный текст контракта, сохранять только `link`
на файл-источник. Полный текст подставлять только при генерации XML. `link` не приводит к сокращениям/потере данных,
поэтому допустим.

#### 3.2. Supplementary Contract / Дополнительное соглашение к контракту (03012)

- **xml_target_root:** `AltaSupplementaryContract`
- **uqi_prefix:** `formalized.supplementary_contract_[n]`

- **Поля:**
  - 01: `DocumentNumber` (№ доп. соглашения; графа 44: G44/G442)
  - 02: `IssueDate` (дата доп. соглашения; графа 44: G44/G443)

  - 03: `ContractDescription_Amount` (новая/уточненная сумма контракта; для контроля/сверки)
  - 04: `ContractDescription_CurrencyCode` (цифровой код валюты ISO 4217 numeric; для контроля/сверки)
  - 05: `ContractDescription_LastDate` (новый срок действия/исполнения; для контроля/сверки)
  - 06: `ContractDescription_ContractText` (текст доп. соглашения; в primary.md хранить `link` на файл-источник; в dt.xml
    напрямую не переносится)
  - 07: `ContractDescription_DealSign` (`1` - системный признак Альты; для импорта; константа; derived)
  - 08: `ContractDescription_StockCategorySign` (`0` - системный признак Альты; для импорта; константа; derived)
  - 09: `ContractDescription_BuyerLimitationSign` (`0` - системный признак Альты; для импорта; константа; derived)
  - 10: `ContractDescription_InsuranceSign` (`0` - системный признак Альты; для импорта; константа; derived)

  - 11: `RussianPerson_OrganizationName` (российская сторона; покупатель; для сверок/мастер-данных)
  - 12: `RussianPerson_ShortName` (краткое наименование; для сверок/мастер-данных)
  - 13: `RussianPerson_OGRN` (ОГРН; для сверок/мастер-данных)
  - 14: `RussianPerson_INN` (ИНН; для сверок/мастер-данных)
  - 15: `RussianPerson_KPP` (КПП; для сверок/мастер-данных)
  - 16: `RussianPerson_Address_PostalCode` (индекс; для сверок/мастер-данных)
  - 17: `RussianPerson_Address_CountryCode` (страна alpha-2; для сверок/мастер-данных)
  - 18: `RussianPerson_Address_CounryName` (страна, текст; **опечатка тега CounryName**; для сверок/мастер-данных)
  - 19: `RussianPerson_Address_Region` (регион; для сверок/мастер-данных)
  - 20: `RussianPerson_Address_City` (город; для сверок/мастер-данных)
  - 21: `RussianPerson_Address_StreetHouse` (улица/дом одной строкой; для сверок/мастер-данных)

  - 22: `ForeignPerson_OrganizationName` (иностранная сторона; продавец; для сверок)
  - 23: `ForeignPerson_ShortName` (краткое наименование; для сверок)
  - 24: `ForeignPerson_Address_CountryCode` (страна alpha-2 из `cb:country`; для сверок)
  - 25: `ForeignPerson_Address_CounryName` (страна, текст; **опечатка тега CounryName**; для сверок)
  - 26: `ForeignPerson_Address_Region` (регион; для сверок)
  - 27: `ForeignPerson_Address_City` (город/район; для сверок)
  - 28: `ForeignPerson_Address_StreetHouse` (улица/дом одной строкой; для сверок)

  - `ContractSignedPerson` (подписант доп. соглашения; группирующий тег)
    - 29: `PersonSurname` (фамилия подписанта)
    - 30: `PersonName` (имя подписанта)
    - 31: `PersonMiddleName` (отчество подписанта)

- **Неформализуемые поля:**
  - 32: `doc_code` (03012 — константа)
  - 33: `doc_name` (`ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ` — константа)
  - 34: `doc_number` (= `DocumentNumber`)
  - 35: `doc_date` (= `IssueDate`)

- _audit: 35

#### 3.3. Invoice (04021)

- **xml_target_root:** `AltaE2I`
- **uqi_prefix:** `formalized.invoice_[n]`

- **Поля шапки / реквизиты документа:**
  - 01: `CurrencyRate` (курс валюты; источник для графы 23: G_23_1, G_23_2)
  - 02: `CurrencyCode` (валюта инвойса ISO 4217 alpha-3, напр. `CNY`, `USD`; источник для графы 22: G_22_3)
  - 03: `DocumentCode` (04021 — код вида документа для графы 44: G44/G441; константа; derived)
  - 04: `PlacesQuantity` (кол-во грузовых мест по инвойсу; приоритет #3 для графы 6: G_6_1)
  - 05: `PlacesDescription` (описание мест, напр. "Поддон"; для сверок/контекста, обычно не в dt.xml напрямую)
  - 06: `GrossWeightQuantity` (общий вес брутто по инвойсу; для сверок с CMR/PL/СВХ)
  - 07: `NetWeightQuantity` (общий вес нетто по инвойсу; для сверок)
  - 08: `GCost` (системное поле Альты; дубль `TotalCost`; для импорта/совместимости; derived)
  - 09: `TotalCost` (итого по инвойсу; источник для графы 22: G_22_2)

  - 10: `DeliveryTerms_DeliveryPlace` (место поставки по Incoterms; источник для графы 20: G_20_2)
  - 11: `DeliveryTerms_DeliveryTermsNumericCode` (числовой код условий поставки; источник для графы 20: G_20_1_1 / внутренний
    код Альты)
  - 12: `DeliveryTerms_DeliveryTermsStringCode` (строковый код условий, напр. `EXW`; источник для графы 20: G_20_1)
  - 13: `DeliveryTerms_DispatchCountryCode` (страна отправления alpha-2; источник для графы 15A: G_15A_1)
  - 14: `DeliveryTerms_TradingCountryCode` (торгующая страна alpha-2; источник для графы 11: G_11_1)
  - 15: `DeliveryTerms_DestinationCountryCode` (страна назначения alpha-2; источник для графы 17A: G_17A_1)

  - 16: `Registration_PrDocumentName` (наименование документа для печати/сверок; может использоваться в графе 44: G44/G444)
  - 17: `Registration_PrDocumentNumber` (номер инвойса; графа 44: G44/G442)
  - 18: `Registration_PrDocumentDate` (дата инвойса; графа 44: G44/G443)

  - 19: `Contract_PrDocumentNumber` (№ контракта-ссылки; для связи документов; графа 44: G44/G442 (для контракта))
  - 20: `Contract_PrDocumentDate` (дата контракта-ссылки; для связи документов; графа 44: G44/G443 (для контракта))

- **Стороны (местами теги “кривые” — это часть структуры Альты):**
  - 21: `Buyer_CompanyID` (ИНН покупателя; источник для мастер-данных; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - 22: `Buyer_KPPCode` (КПП покупателя; источник для мастер-данных; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - 23: `Buyer_Name` (наименование покупателя; графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM)
  - 24: `Buyer_PostalAddress_PostalCode` (индекс покупателя; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS)
  - 25: `Buyer_PostalAddress_CountryCode` (страна покупателя alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC)
  - 26: `Buyer_PostalAddress_CounryName` (страна покупателя, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN,
    G_14_CN)
  - 27: `Buyer_PostalAddress_Region` (регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB)
  - 28: `Buyer_PostalAddress_City` (город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT)
  - 29: `Buyer_PostalAddress_StreetHouse` (улица/дом/офис одной строкой; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR)

  - 30: `Seler_Name` (продавец; **опечатка Seler**; источник для графы 2: G_2_NAM)
  - 31: `Seler_PostalAddress_CountryCode` (страна продавца alpha-2 из `cb:country`; графа 2: G_2_7)
  - 32: `Seler_PostalAddress_CounryName` (страна продавца, текст; **опечатка CounryName**; графа 2: G_2_50)
  - 33: `Seler_PostalAddress_Region` (регион продавца; графа 2: G_2_SUB)
  - 34: `Seler_PostalAddress_City` (город/район продавца; графа 2: G_2_CIT)
  - 35: `Seler_PostalAddress_StreetHouse` (улица/дом одной строкой; графа 2: G_2_STR)

  - 36: `Consignor_OrganizationName` (грузоотправитель; если отличается от продавца — для сверок/графы 2)
  - 37: `Consignor_Address_CountryCode` (страна грузоотправителя alpha-2 из `cb:country`; для сверок)
  - 38: `Consignor_Address_CounryName` (страна грузоотправителя, текст; **опечатка CounryName**; для сверок)
  - 39: `Consignor_Address_Region` (регион; для сверок)
  - 40: `Consignor_Address_City` (город/район; для сверок)
  - 41: `Consignor_Address_StreetHouse` (улица/дом одной строкой; для сверок)

  - 42: `Consignee_OrganizationName` (грузополучатель; обычно получатель/декларант; графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM)
  - 43: `Consignee_OGRN` (ОГРН; графы 8/9/14: G_8_1, G_9_1, G_14_1)
  - 44: `Consignee_INN` (ИНН; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - 45: `Consignee_KPP` (КПП; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - 46: `Consignee_Address_PostalCode` (индекс; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS)
  - 47: `Consignee_Address_CountryCode` (страна alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC)
  - 48: `Consignee_Address_CounryName` (страна, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, G_14_CN)
  - 49: `Consignee_Address_Region` (регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB)
  - 50: `Consignee_Address_City` (город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT)
  - 51: `Consignee_Address_StreetHouse` (улица/дом/офис одной строкой; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR)

- **Неформализуемые поля:**
  - 52: `doc_code` (04021 — константа)
  - 53: `doc_name` (`ИНВОЙС` — константа)
  - 54: `doc_number` (= `Registration_PrDocumentNumber`)
  - 55: `doc_date` (= `Registration_PrDocumentDate`)

- _audit: 55

- **Товарные позиции** (каждый элемент соответствует узлу `<InvoiceGoods>...</InvoiceGoods>`; источник для блока товаров
  dt.xml: `BLOCK/TOVG/TXT`:
  - `InvoiceGoods[n]`
    - 01: `GoodsCode` (код ТН ВЭД; источник для графы 33: G_33_1)
    - 02: `GoodsDescription` (описание товара как в инвойсе; источник для графы 31: G_31/NAME и для строк дополнения TXT/TEXT)
    - 03: `GoodsQuantity` (кол-во по строке инвойса в “основной” единице строки; для сверок; не использовать
      как TOVG/KOLVO, если в инвойсе есть отдельная колонка доп.кол-ва)
    - 04: `goods_supplementary_quantity` (количество в доп.ед.изм для ДТ; например, `Quantity in M2`; неформализуемое поле)
    - 05: `goods_supplementary_uom_name` (наименование доп.ед.изм из `cb:unit`; неформализуемое поле)
    - 06: `MeasureUnitQualifierName` (единица измерения доп.количества для ДТ, наименование из `cb:unit`; цель: TOVG/NAME_EDI)
    - 07: `GrossWeightQuantity` (брутто по строке; источник для веса: G_35_1 (агрегация) и TOVG/G31_35)
    - 08: `NetWeightQuantity` (нетто по строке; источник для веса: G_38_1 (агрегация) и TOVG/G31_38; derived)
    - 09: `Price` (цена за единицу; для сверок/контроля; обычно не переносится в dt.xml напрямую)
    - 10: `TotalCost` (стоимость по строке; источник для графы 42 (агрегация) и TOVG/INVOICCOST)
    - 11: `OriginCountryCode` (цифровой код страны происхождения; источник для графы 34 после нормализации в alpha-2: G_34_1)

    - 12: `AdditionalGoodsDescription_Manufacturer` (производитель; источник для графы 31: G_31/FIRMA и TOVG/G31_11)
    - 13: `AdditionalGoodsDescription_TradeMark` (товарная марка/ТМ; источник для графы 31: G_31/TM и TOVG/G31_12; если
      отсутствует в первичке — "ОТСУТСТВУЕТ")
    - 14: `AdditionalGoodsDescription_GoodsMark` (товарный знак/маркировка; источник для графы 31 и TOVG/G31_14; если
      отсутствует — "ОТСУТСТВУЕТ")
    - 15: `AdditionalGoodsDescription_GoodsModel` (модель/модификация; источник для графы 31 и TOVG/G31_15_MOD);
      если нельзя извлечь точное значение, заполняется названием товара.

  - _item_audit: 15

#### 3.4. Packing List / Упаковочный лист (04131)

- **xml_target_root:** `AltaE2PACK`
- **uqi_prefix:** `formalized.packing_list_[n]`

- **Поля:**
  - 01: `GrossWeightQuantity` (общий вес брутто по упаковочному; для сверок; может участвовать  
    по графам 35/38 при необходимости; derived)
  - 02: `NetWeightQuantity` (общий вес нетто по упаковочному; для сверок; derived)

  - 03: `Consignor_OrganizationName` (грузоотправитель; для сверок с инвойсом/CMR)
  - 04: `Consignor_ShortName` (краткое наименование; для сверок)
  - 05: `Consignor_Address_CountryCode` (страна грузоотправителя alpha-2, используй `cb:country`; для сверок)
  - 06: `Consignor_Address_CounryName` (страна грузоотправителя, текст; **опечатка CounryName**; для сверок)
  - 07: `Consignor_Address_Region` (регион; для сверок)
  - 08: `Consignor_Address_City` (город/район; для сверок)
  - 09: `Consignor_Address_StreetHouse` (улица/дом одной строкой; для сверок)

  - 10: `Consignee_OrganizationName` (грузополучатель; для сверок/мастер-данных)
  - 11: `Consignee_ShortName` (краткое наименование; для сверок/мастер-данных)
  - 12: `Consignee_OGRN` (ОГРН; для сверок/мастер-данных → графы 8/9/14: G_8_1, G_9_1, G_14_1)
  - 13: `Consignee_INN` (ИНН; для сверок/мастер-данных → графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - 14: `Consignee_KPP` (КПП; для сверок/мастер-данных → графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - 15: `Consignee_Address_PostalCode` (индекс; для сверок/мастер-данных → G_8_POS, G_9_POS, G_14_POS)
  - 16: `Consignee_Address_CountryCode` (страна alpha-2; для сверок/мастер-данных → G_8_7, G_9_CC, G_14_CC)
  - 17: `Consignee_Address_CounryName` (страна, текст; **опечатка CounryName**; для сверок/мастер-данных → G_8_50, G_9_CN,
    G_14_CN)
  - 18: `Consignee_Address_Region` (регион; для сверок/мастер-данных → G_8_SUB, G_9_SUB, G_14_SUB)
  - 19: `Consignee_Address_City` (город; для сверок/мастер-данных → G_8_CIT, G_9_CIT, G_14_CIT)
  - 20: `Consignee_Address_StreetHouse` (улица/дом/офис одной строкой; для сверок/мастер-данных → G_8_STR, G_9_STR, G_14_STR)

  - 21: `DeliveryTerms_DeliveryPlace` (место поставки по Incoterms; источник для графы 20: G_20_2)
  - 22: `DeliveryTerms_DeliveryTermsNumericCode` (внутренний числовой код условий; источник для графы 20: G_20_1_1 / внутренний
    код Альты)
  - 23: `DeliveryTerms_DeliveryTermsStringCode` (строковый код условий, напр. `EXW`; источник для графы 20: G_20_1)

  - 24: `DeliveryTerms_Contract_PrDocumentName` (`КОНТРАКТ`, константа; графы 44: G44/G444)
  - 25: `DeliveryTerms_Contract_PrDocumentNumber` (№ контракта; графа 44: G44/G442)
  - 26: `DeliveryTerms_Contract_PrDocumentDate` (дата контракта; графа 44: G44/G443)

  - 27: `DeliveryTerms_Invoice_PrDocumentName` (`ИВОЙС`, константа; графы 44: G44/G444)
  - 28: `DeliveryTerms_Invoice_PrDocumentNumber` (№ инвойса; графа 44: G44/G442)
  - 29: `DeliveryTerms_Invoice_PrDocumentDate` (дата инвойса; графа 44: G44/G443)

  - 30: `DeliveryTerms_Registration_PrDocumentName` (`УПАКОВОЧНЫЙ ЛИСТ`, константа; графа 44: G44/G444)
  - 31: `DeliveryTerms_Registration_PrDocumentNumber` (№ упаковочного; графа 44: G44/G442)
  - 32: `DeliveryTerms_Registration_PrDocumentDate` (дата упаковочного; графа 44: G44/G443)

- **Неформализуемые поля:**
  - 33: `doc_code` (04131 — константа)
  - 34: `doc_name` (`УПАКОВОЧНЫЙ ЛИСТ` — константа)
  - 35: `doc_number` (= `DeliveryTerms_Registration_PrDocumentNumber`)
  - 36: `doc_date` (= `DeliveryTerms_Registration_PrDocumentDate`)

- _audit: 36

- **Товарные/грузовые строки** (каждый элемент соответствует узлу `<Goods>...</Goods>`; это строки “по местам/грузовым
  единицам”, не по товарам ДТ):
  - `Goods[n]`
    - 01: `GoodsDescription` (описание строки как в документе; для сверок/контекста; может быть агрегированным текстом)
    - 02: `GoodsQuantity` (количество мест/грузовых единиц в строке; источник приоритета #2/#3 для графы 6: G_6_1 через derived)
    - 03: `GrossWeightQuantity` (брутто по строке; для сверок)
    - 04: `NetWeightQuantity` (нетто по строке; для сверок)
    -  `PackingInfo` (группирующий тег)
    - 05: `PakingQuantity` (кол-во упаковок/мест в упаковке; **опечатка PakingQuantity**; в эталонах может быть 0/пусто)
  
  - _item_audit: 5

- **Транспорт** (каждый элемент соответствует узлу `<TransportMeans>...</TransportMeans>`; источник для графы 18: G_18
  и связанных derived-полей):
  - `TransportMeans[n]`
    - 01: `Number` (регистрационный номер; источник для графы 18: G_18)
    - 02: `ModeCode` (код вида транспорта; источник для граф 25/26: G_25_1, G_26_1; для автосостава обычно 31)
    - 03: `NationalityCode` (код “национальности” ТС в структуре Альты; для сверок/совместимости, в эталонах может быть `000`)
    - 04: `MoverIndicator` (`true` для тягача, `false` для прицепа; нужно для порядка/логики формирования G_18)

  - _item_audit: 4

**Правило:** если известны номер тягача и номер прицепа, сохранять их как ДВА элемента:
- transport_1 (MoverIndicator=true) — тягач
- transport_2 (MoverIndicator=false) — прицеп

#### 3.5. CMR / Международная товарно-транспортная накладная (02015)

CMR является транспортным документом и может не содержать детализацию товаров (в отличие от Invoice).

**Правило:** Если в CMR отсутствует детализация груза (поля/разделы `9 Product name` пустые и нет списка строк), то:
- материализовать ровно 1 элемент `CMRGoods_1`;
- `CMRGoods_1.GoodsNumeric` = `1`, `status: CD`, `note: авто-нумерация единственной строки`;
- `CMRGoods_1.GoodsDescription`:
  - если в `non_formalized.svh_1` (ДО-1) есть фраза вида `Товар загружен согласно спецификации к Invoice № ...`,
    перенести её сюда;
  - `status: CD`, `note: исключение CMRGoodsDescription — источник non_formalized.svh_1`.

- **xml_target_root:** `AltaE3CMR`
- **uqi_prefix:** `formalized.cmr_[n]`

- **Поля:**
  - 01: `LanguageCode` (язык документа; для импорта/совместимости; обычно `RU`; константа; derived)
  - 02: `CMR_Choice` (системный выбор/вариант Альты; для импорта, обычно `1`; константа; derived)

  - 03: `RegistrationDocument_RegID` (номер CMR; графа 44: G44/G442)
  - 04: `RegistrationDocument_DateInf` (дата CMR; графа 44: G44/G443)
  - 05: `RegistrationDocument_Place` (место составления; для сверок/контекста)

  - 06: `TrakingCargo_TakingCargoDate` (дата CMR; **опечатка TrakingCargo**; для сверок/контекста)
  - 07: `TrakingCargo_TakingCargoPlace_CountryCode` (страна принятия груза alpha-2; для сверок/контекста)
  - 08: `TrakingCargo_TakingCargoPlace_CounryName` (страна принятия груза, текст; **опечатка CounryName**; для сверок/контекста)

  - 09: `DeliveryPlace_CountryCode` (страна доставки alpha-2; для сверок/контекста)
  - 10: `DeliveryPlace_CounryName` (страна доставки, текст; **опечатка CounryName**; для сверок/контекста)

  - 11: `DeliveryTerms_DeliveryPlace` (место поставки по Incoterms, если указано в CMR; источник/сверка для графы 20: G_20_2)
  - 12: `DeliveryTerms_DeliveryTermsStringCode` (условия поставки, напр. `EXW`; источник/сверка для графы 20: G_20_1)

  - 13: `GoodsQuantity` (общее количество грузовых мест/упаковок по CMR; для сверки с графой 6: G_6_1 и с инвойсом/PL)
  - 14: `CMRGoodsWeight_GrossWeightQuantity` (общий вес брутто по CMR; ключевой источник сверки брутто для графы 35: G_35_1
    (агрегации))

  - 15: `CMRTransport_PrimeMoverStateSignID` (гос. номер тягача; источник/сверка для графы 18: G_18)
  - 16: `CMRTransport_TrailerStateSignID` (гос. номер прицепа; источник/сверка для графы 18: G_18)

- **Отправитель (как в структуре Альты):**
  - 17: `Consignor_NameInf` (наименование; для сверок с инвойсом/контрактом)
  - 18: `Consignor_ShortName` (краткое наименование; для сверок)
  - 19: `Consignor_PostalAddress_CountryCode` (страна alpha-2; для сверок)
  - 20: `Consignor_Address_CounryName` (страна, текст; **опечатка CounryName**; для сверок)
  - 21: `Consignor_Address_Region` (регион; для сверок)
  - 22: `Consignor_Address_City` (город/район; для сверок)
  - 23: `Consignor_Address_StreetHouse` (улица/дом одной строкой; для сверок)

- **Гарант отправителя** (если присутствует в структуре; для сверок/аудита, обычно не переносится в dt.xml напрямую):
  - 24: `Consignor_Guarantee_OrganizationName` (наименование гаранта)
  - 25: `Consignor_Guarantee_ShortName` (краткое наименование)
  - 26: `Consignor_Guarantee_Address_CountryCode` (страна alpha-2)
  - 27: `Consignor_Guarantee_Address_CounryName` (страна, текст; **опечатка CounryName**)
  - 28: `Consignor_Guarantee_Address_Region` (регион)
  - 29: `Consignor_Guarantee_Address_City` (город/район)
  - 30: `Consignor_Guarantee_Address_StreetHouse` (улица/дом одной строкой)

- **Получатель:**
  - 31: `Consignee_NameInf` (наименование получателя; для сверок/мастер-данных → графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM)
  - 32: `Consignee_ShortName` (краткое наименование; для сверок)
  - 33: `Consignee_OGRNID` (ОГРН; **суффикс ID — часть тега Альты**; графы 8/9/14: G_8_1, G_9_1, G_14_1)
  - 34: `Consignee_INNID` (ИНН; **суффикс ID — часть тега Альты**; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - 35: `Consignee_KPPCode` (КПП; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - 36: `Consignee_PostalAddress_PostalCode` (индекс; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS)
  - 37: `Consignee_PostalAddress_CountryCode` (страна alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC)
  - 38: `Consignee_Address_CounryName` (страна, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, G_14_CN)
  - 39: `Consignee_Address_Region` (регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB)
  - 40: `Consignee_Address_City` (город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT)
  - 41: `Consignee_Address_StreetHouse` (улица/дом/офис одной строкой; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR)

- **Неформализуемые поля:**
  - 42: `doc_code` (02015 — константа)
  - 43: `doc_name` (`CMR` — константа)
  - 44: `doc_number` (= `RegistrationDocument_RegID`)
  - 45: `doc_date` (= `RegistrationDocument_DateInf`)

- _audit: 45

- **Товарные/грузовые строки** (каждый элемент соответствует узлу `<CMRGoods>...</CMRGoods>`; это строки
  “по местам/упаковкам”, не по товарам ДТ):
  - `CMRGoods[n]`
    - 01: `GoodsNumeric` (порядковый номер строки)
    - 02: `GoodsDescription` (описание груза/товара как в CMR; для сверок и при необходимости для дополнения к графе 31)
    - `GoodsPackingInfo` (группирующий тег)
      - 03: `PakingQuantity` (кол-во упаковок/мест; **опечатка PakingQuantity**; для сверок/контекста)
  - _item_audit: 3

#### 3.6. Payment Order / Платежное поручение (04023)

- **xml_target_root:** `AltaPaymentOrder`
- **uqi_prefix:** `formalized.payment_order_[n]`

- **Поля:**
  - 01: `DocumentCode` (04023 — код вида документа для графы 44: G44/G441; константа; derived)

  - 02: `PaymentModeCode` (системный код способа платежа в структуре Альты; для импорта/совместимости; derived)
  - 03: `PaymentAmount` (сумма платежа; для сверок с оплатой по поставке; обычно не переносится в dt.xml напрямую)
  - 04: `TransactionKind` (`01` - вид операции/код; системное поле Альты; константа; derived)
  - 05: `Priority` (`"."` -очередность; системное поле; константа; derived)
  - 06: `Purpose` (назначение платежа; содержит ссылки на контракт/инвойс; используется для сверки связей документов)
  - 07: `ValueSpelledOut` (сумма прописью; для сверок/контекста)

  - 08: `DocumentReference_PrDocumentNumber` (номер платежного поручения; графа 44: G44/G442)
  - 09: `DocumentReference_PrDocumentDate` (дата платежного поручения; графа 44: G44/G443)

  - 10: `Payer_OrganizationName` (плательщик; для сверок/контекста)
  - 11: `Payer_INN` (ИНН плательщика; для сверок)
  - 12: `Payer_KPP` (КПП плательщика; для сверок)
  - 13: `Payer_Bank_BankName` (в теге часто лежит блок реквизитов/адреса; может быть многострочным; сохранять как есть;
    для сверок/контекста)

  - 14: `Payee_OrganizationName` (получатель платежа; может быть многострочным/с переносами; сохранять как есть; для
    сверок/контекста)
  - 15: `Payee_Bank_BankName` (реквизиты банка получателя; может быть многострочным; сохранять как есть; для сверок/контекста)

  - `PayerSign` (подписант/плательщик; для сверок/аудита; группирующий тег)
    - 16: `PersonSurname` (фамилия)
    - 17: `PersonName` (имя)

- **Неформализуемые поля:**
  - 18: `doc_code` (04023 — константа)
  - 19: `doc_name` (`ПЛАТЕЖНОЕ ПОРУЧЕНИЕ` — константа)
  - 20: `doc_number` (= `DocumentReference_PrDocumentNumber`)
  - 21: `doc_date` (= `DocumentReference_PrDocumentDate`)

- _audit: expected=21

#### 3.7. Service Invoice / Счет за перевозку (04031)

- **xml_target_root:** `AltaServiceInvoice`
- **uqi_prefix:** `formalized.service_invoice_[n]`

- **Поля:**
  - 01: `DocumentSign` (системный признак документа Альты; для импорта/совместимости, обычно `1`; derived)
  - 02: `TotalServiceCost` (итого по услугам; для расчётов/сверок, при необходимости может участвовать в графах стоимости)
  - 03: `Currency` (валюта итого ISO 4217 alpha-3; для расчётов/сверок)

  - 04: `ServiceProvider_Name` (исполнитель услуг/перевозчик; для сверок/контекста)
  -  `ServiceProvider_PaymentRequisitions` (группирующий тег)
  - 05: `BankName` (банк исполнителя; для сверок/контекста)

  - 06: `ContractDetails_PrDocumentNumber` (№ договора на услуги/перевозку; графа 44: G44/G442 (если прикладывается как документ))
  - 07: `ContractDetails_PrDocumentDate` (дата договора на услуги/перевозку; графа 44: G44/G443)

  - `PaymentDocument` (связанный документ/заказ в структуре Альты; используется для связи документов/сверок; группирующий тег)
    - 08: `PrDocumentNumber` (номер; графа 44: G44/G442)
    - 09: `PrDocumentDate` (дата; графа 44: G44/G443)

  - 10: `Registration_PrDocumentName` (наименование счета; графа 44: G44/G444)
  - 11: `Registration_PrDocumentNumber` (номер счета; графа 44: G44/G442)
  - 12: `Registration_PrDocumentDate` (дата счета; графа 44: G44/G443)

  - 13: `Consignor_OrganizationName` (грузоотправитель; для сверок/контекста)
  - `Consignor_SubjectAddressDetails` (группирующий тег)
    - 14: `PostalCode` (индекс; для сверок)
    - 15: `CountryCode` (страна alpha-2; для сверок)
    - 16: `CounryName` (страна, текст; **опечатка CounryName**; для сверок)
    - 17: `Region` (регион; для сверок)
    - 18: `Town` (город/район; тег отличается от `City`; для сверок)
    - 19: `StreetHouse` (улица/дом одной строкой; для сверок)

  - 20: `Consignee_OrganizationName` (грузополучатель; для сверок/мастер-данных → графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM)
  - 21: `Consignee_RFOrganizationFeatures_OGRN` (ОГРН; графы 8/9/14: G_8_1, G_9_1, G_14_1)
  - 22: `Consignee_RFOrganizationFeatures_INN` (ИНН; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - 23: `Consignee_RFOrganizationFeatures_KPP` (КПП; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Consignee_SubjectAddressDetails` (группирующий тег)
    - 24: `PostalCode` (индекс; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS)
    - 25: `CountryCode` (страна alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC)
    - 26: `CounryName` (страна, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, G_14_CN)
    - 27: `Region` (регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB)
    - 28: `Town` (город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT)
    - 29: `StreetHouse` (улица; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR)
    - 30: `House` (дом; графы 8/9/14: G_8_BLD, G_9_BLD, G_14_BLD)
    - 31: `Room` (офис/кв; графы 8/9/14: G_8_ROM, G_9_ROM, G_14_ROM)

- **Подписи**
  - 32: `Signature_Choice` (вариант подписи: 1 - если счет подписан индивидуальным предпринимателем (ИП),
    то руководитель, бухгалтер = пусто; 2 - если счет подписан руководителем и бухгалтером, то поля ИП = пусто;
    `status = CD` - всегда, для всех полей блока, для обоих вариантов. pending НЕ ставить.)
  - 33: `IndividualEntrepreneur_PersonSurname` (фамилия ИП)
  - 34: `IndividualEntrepreneur_PersonName` (первый инициал/имя ИП)
  - 35: `IndividualEntrepreneur_PersonMiddleName` (второй инициал/отчество индивидуального предпринимателя)
  - 36: `SignatureDirectorChiefAccountant_Director_PersonSurname` (фамилия руководителя)
  - 37: `SignatureDirectorChiefAccountant_Director_PersonName` (первый инициал/имя руководителя)
  - 38: `SignatureDirectorChiefAccountant_Director_PersonMiddleName` (второй инициал/отчество руководителя)
  - 39: `SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname` (фамилия бухгалтера)
  - 40: `SignatureDirectorChiefAccountant_ChiefAccountant_PersonName` (первый инициал/имя бухгалтера)
  - 41: `SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName` (второй инициал/отчество бухгалтера)

- **Неформализуемые поля:**
  - 42: `doc_code` (04031 — константа)
  - 43: `doc_name` (`СЧЕТ ЗА ПЕРЕВОЗКУ` — константа)
  - 44: `doc_number` (= `Registration_PrDocumentNumber`)
  - 45: `doc_date` (= `Registration_PrDocumentDate`)

- **Неформализуемые поля:**
  - 46: `transport_to_border` (если найдена стоимость маршрута “до границы” — всегда берем ее, остальные услуги игнорируем)
  - 47: `transport_currency` (= `ServiceDescription[1].ServiceCost_Currency`)

- _audit: 47

- **Услуги** (каждый элемент соответствует узлу `<ServiceDescription>...</ServiceDescription>`):
  - `ServiceDescription[n]`
    - 01: `GoodsDescription` (многострочное описание услуги — сохранять как есть; **может отсутствовать** в отдельных строках)
    - 02: `CurrencyCode` (валюта строки ISO alpha-3; для сверок/расчётов)
    - 03: `ServiceName` (наименование/маршрут; для сверок/контекста)
    - 04: `TaxRate` (ставка налога; для сверок/расчётов)
    - 05: `TaxSum` (сумма налога; для сверок/расчётов)
    - 06: `ServiceCost_Amount` (стоимость строки; для сверок/расчётов)
    - 07: `ServiceCost_Currency` (валюта стоимости строки; для сверок/расчётов)

  - _item_audit: 7

#### 3.8. Insurance Services Invoice / Счет за страховые услуги (04111)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.insurance_document_[n]`

- **Поля:**
  - 01: `DocumentCode` (04111 — код вида документа для графы 44: G44/G441; константа; derived)
  - 02: `DocumentHead_DocumentName` (наименование документа; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата документа; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер документа; графа 44: G44/G442)
  - 05: `TextPara` (основной текст/условия; в primary.md хранить `link` на файл-источник; `DocumentBody_TextSection`)

- **Неформализуемые поля:**
  - 06: `doc_code` (04111 — константа)
  - 07: `doc_name` (`СЧЕТ ЗА СТРАХОВКУ` — константа)
  - 08: `doc_number` (= `DocumentHead_DocumentNumber`)
  - 09: `doc_date` (= `DocumentHead_DocumentDate`)

- **Неформализуемые поля:**
  - 10: `insurance_to_border` (стоимость страхования продавцом)
  - 11: `insurance_currency` (валюта страхования)

- _audit: 11

#### 3.9. TechDescription / Техническое описание (05999)

Наличие нескольких технических описаний для разных товаров допустимо. Но, если несколько технических описаний
относятся к одному и тому же товару (совпадает наименование, модель или явная ссылка на товар) или не может быть
соотнесено с товаром, AI не делает предположений об их релевантности и обязан вынести вопрос в раздел нерешенных вопросов.

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.tech_description_[n]`

- **Поля:**
  - 01: `DocumentCode` (05999 — код вида документа для графы 44: G44/G44; константа; derived)
  - 02: `DocumentHead_DocumentName` (наименование техописания; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата техописания; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер техописания; графа 44: G44/G442)
    `DocumentBody_TextSection` (группирующий тег: G_31)
    - 05: `TextPara` (технический текст без пересказа; в primary.md хранить `link` на файл-источник;

- **Неформализуемые поля:**
  - 06: `doc_code` (05999 — константа)
  - 07: `doc_name` (`ТЕХНИЧЕСКОЕ ОПИСАНИЕ` — константа)
  - 08: `doc_number` (= `DocumentHead_DocumentNumber`)
  - 09: `doc_date` (= `DocumentHead_DocumentDate`)

- _audit: 9

#### 3.10. FreeDoc / Прочие текстовые документы (09999)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.free_doc_[n]`

- **Поля:**
  - 01: `DocumentCode` (09999 — код вида документа для графы 44: G44/G441; константа; derived)
  - 02: `DocumentHead_DocumentName` (наименование документа; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата документа; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер документа; графа 44: G44/G442)
  - 05: `TextPara` (основной текст; в primary.md хранить `link` на файл-источник; `DocumentBody_TextSection`)
-
- **Неформализуемые поля:**
  - 06: `doc_code` (09999 — константа)
  - 07: `doc_name` (`ПРОЧИЙ ДОКУМЕНТ` — константа)
  - 08: `doc_number` (= `DocumentHead_DocumentNumber`)
  - 09: `doc_date` (= `DocumentHead_DocumentDate`)

- _audit: 9

#### 3.11. FreeBinaryDoc / Бинарное приложение (например PDF-вложение)

- **xml_target_root:** `AltaFreeBinaryDoc`
- **uqi_prefix:** `formalized.free_binary_doc_[n]`

- **Поля:**
  - 01: `DocumentCode` (код вида документа; для графы 44: G44/G441; если неизвестен — уточнить по `cb:doc`, иначе `pending`;
    derived)
  - 02: `DocumentInfo_PrDocumentName` (наименование вложения; графа 44: G44/G444)
  - 03: `DocumentInfo_PrDocumentNumber` (номер документа-основания/вложения; графа 44: G44/G442)
  - 04: `DocumentInfo_PrDocumentDate` (дата документа-основания/вложения; графа 44: G44/G443)

  - 05: `DocumentBody_FileName` (имя файла вложения; для импорта/контроля)
  - 06: `DocumentBody_FileData` (base64 содержимого; в primary.md  хранить `link` на исходный файл)
  - 07: `Thumbnail` (миниатюра/base64; если не требуется для импорта — не материализовывать)

- **Неформализуемые поля:**
  - 08: `doc_code` (= `DocumentCode`) (код вида документа; derived; код бери из справочника `cb:doc`)
  - 09: `doc_name` (= `DocumentInfo_PrDocumentName`)
  - 10: `doc_number` (= `DocumentInfo_PrDocumentNumber`)
  - 11: `doc_date` (= `DocumentInfo_PrDocumentDate`)

- _audit: 11

#### 3.12. Personal Passport / Паспорт (11001)
Источники: основной - `master_data.md`, первоисточник в каталоге `stable_source`.

- **xml_target_root:** `AltaPassport`
- **uqi_prefix:** `formalized.passport_[n]`

- **Поля для графы 44:**
  - 01: `DocumentCode` (value = `11001`) (код вида документа; для графы 44; константа; derived)
  - 02: `DocumentHead_DocumentName` (value = `ПАСПОРТ`) (наименование документа; для графы 44; derived)
  - 03: `DocumentHead_DocumentDate` (value = `CardDate`) (дата документа; для графы 44; derived)
  - 04: `DocumentHead_DocumentNumber` (value = `CardSeries + " " + CardNumber`, например `"63 09 449948"`) (derived)
    (номер документа; для графы 44)

- **Поля:**
  - 05: `CardSeries` (серия; источник для графы 54: G_54_12)
  - 06: `CardNumber` (номер; источник для графы 54: G_54_100)
  - 07: `OrganizationName` (кем выдан; источник для графы 54: G_54_13)
  - 08: `CardDate` (дата выдачи; источник для графы 54: G_54_101)

  - 09: `PersonInfo_PersonSurname` (фамилия; источник для графы 54: G_54_3)
  - 10: `PersonInfo_PersonName` (имя; источник для графы 54: G_54_3NM)
  - 11: `PersonInfo_PersonMiddleName` (отчество; источник для графы 54: G_54_3MD)
  - 12: `PersonInfo_Sex` (пол; для сверок/контекста, в dt.xml обычно не переносится)
  - 13: `PersonInfo_Birthday` (дата рождения; для сверок/контекста)
  - 14: `PersonInfo_Birthplace` (место рождения; для сверок/контекста)

  - 15: `ResidencePlace_PostalCode` (индекс; для сверок/контекста)
  - 16: `ResidencePlace_CountryCode` (страна alpha-2; для сверок/контекста)
  - 17: `ResidencePlace_CounryName` (страна, текст; возможна **опечатка CounryName**; для сверок/контекста)
  - 18: `ResidencePlace_Region` (регион; для сверок/контекста)
  - 19: `ResidencePlace_City` (город; для сверок/контекста)
  - 20: `ResidencePlace_StreetHouse` (адрес одной строкой; для сверок/контекста)

- **Неформализуемые поля:**
  - 21: `doc_code` (11001 — константа)
  - 22: `doc_name` (`ПАСПОРТ` — константа)
  - 23: `doc_number` (= `DocumentHead_DocumentNumber`)
  - 24: `doc_date` (= `DocumentHead_DocumentDate`)

- _audit: 24

#### 3.13. Letter of Attorney / Доверенность (11004)
Источники: основной - `master_data.md`, первоисточник в каталоге `stable_source`.

- **xml_target_root:** `AltaLetterOfAttorney`
- **uqi_prefix:** `formalized.letter_of_attorney_[n]`

- **Поля для графы 44:**
  - 01: `DocumentCode` (value = `11004`) (код вида документа; для графы 44; константа; derived)
  - 02: `DocumentHead_DocumentName` (value = `DocumentReference_PrDocumentName`) (наименование документа; для графы 44; derived)
  - 03: `DocumentHead_DocumentDate` (value = `DocumentReference_PrDocumentDate`) (дата документа; для графы 44; derived)
  - 04: `DocumentHead_DocumentNumber` (value = `DocumentReference_PrDocumentNumber`) (номер документа; для графы 44; derived)

- **Поля:**
  - 05: `Subject` (текст доверенности; в primary.md хранить `link` на файл-источник в `stable_source`; используется 
    для формирования печатного блока графы 54: G_54P при необходимости)
  - 06: `EndDate` (действительна до; источник для графы 54: G_54_61)

  - 07: `DocumentReference_PrDocumentName` (наименование доверенности; источник для графы 54: G_54_4)
  - 08: `DocumentReference_PrDocumentNumber` (номер доверенности; источник для графы 54: G_54_5)
  - 09: `DocumentReference_PrDocumentDate` (дата доверенности; источник для графы 54: G_54_60)

  - 10: `Organization_OrganizationName` (выдавшая организация; для сверок/контекста)
  - 11: `Organization_ShortName` (краткое наименование; для сверок/контекста)
  - 12: `Organization_OGRN` (ОГРН; для сверок/контекста)
  - 13: `Organization_INN` (ИНН; для сверок/контекста)
  - 14: `Organization_KPP` (КПП; для сверок/контекста)
  - 15: `Organization_Address_PostalCode` (индекс; для сверок/контекста)
  - 16: `Organization_Address_CountryCode` (страна alpha-2; для сверок/контекста)
  - 17: `Organization_Address_CounryName` (страна, текст; возможна **опечатка CounryName**; для сверок/контекста)
  - 18: `Organization_Address_Region` (регион; для сверок/контекста)
  - 19: `Organization_Address_City` (город; для сверок/контекста)
  - 20: `Organization_Address_StreetHouse` (улица/дом одной строкой; для сверок/контекста)

  - 21: `Organization_OrganizationPerson_PersonSurname` (подписант от организации; для сверок/контекста)
  - 22: `Organization_OrganizationPerson_PersonName` (имя/инициалы; для сверок/контекста)
  - 23: `Organization_OrganizationPerson_PersonMiddleName` (отчество; для сверок/контекста)
  - 24: `Organization_OrganizationPerson_PersonPost` (должность; для сверок/контекста)

  - 25: `EmpoweredPerson_PersonSurname` (уполномоченное лицо; источник для графы 54: G_54_3)
  - 26: `EmpoweredPerson_PersonName` (имя; источник для графы 54: G_54_3NM)
  - 27: `EmpoweredPerson_PersonMiddleName` (отчество; источник для графы 54: G_54_3MD)
  - 28: `EmpoweredPerson_PersonPost` (роль/должность; источник для графы 54: G_54_7)

  - 29: `EmpoweredPerson_Passport_IdentityCardCode` (код документа; источник для графы 54: G_54_8)
  - 30: `EmpoweredPerson_Passport_IdentityCardName` (наименование документа; источник для графы 54: G_54_9)
  - 31: `EmpoweredPerson_Passport_IdentityCardSeries` (серия; источник для графы 54: G_54_12)
  - 32: `EmpoweredPerson_Passport_IdentityCardNumber` (номер; источник для графы 54: G_54_100)
  - 33: `EmpoweredPerson_Passport_IdentityCardDate` (дата выдачи; источник для графы 54: G_54_101)
  - 34: `EmpoweredPerson_Passport_OrganizationName` (кем выдан; источник для графы 54: G_54_13)

- **Неформализуемые поля:**
  - 35: `doc_code` (11004 — константа)
  - 36: `doc_name` (= `DocumentHead_DocumentName`)
  - 37: `doc_number` (= `DocumentHead_DocumentNumber`)
  - 38: `doc_date` (= `DocumentHead_DocumentDate`)

- _audit: 38

#### 3.14. Transport Contract / Договор транспортной экспедиции (04033)
Источники: основной - `master_data.md`, первоисточник в каталоге `stable_source`.

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.transport_contract_[n]`

- **Поля для графы 44:**
  - 01: `DocumentCode` (value = `04033` — код вида документа для графы 44: G44/G441; константа; derived)
  - 02: `DocumentHead_DocumentName` (наименование договора; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата договора; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер договора; графа 44: G44/G442)
  - 05: `TextPara` (текст договора; в primary.md хранить `link` на файл-источник в `stable_source`; `DocumentBody_TextSection`)

- **Неформализуемые поля:**
  - 06: `doc_code` (04033 — константа)
  - 07: `doc_name` (`ДОГОВОР ТРАНСПОРТНОЙ ЭКСПЕДИЦИИ` — константа)
  - 08: `doc_number` (= `DocumentHead_DocumentNumber`)
  - 09: `doc_date` (= `DocumentHead_DocumentDate`)

- _audit: 9

#### 3.15. EGRUL / Учредительные документы (04011) (в т.ч. выписка из ЕГРЮЛ)
Источники: основной - `master_data.md`, первоисточник в каталоге `stable_source`.
- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.egrul_[n]`

- **Поля для графы 44:**
  - 01: `DocumentCode` (04011 — код вида документа для графы 44: G44/G441; константа; derived)
  - 02: `DocumentHead_DocumentName` (наименование выписки; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата выписки; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер выписки; графа 44: G44/G442)
  - 05: `TextPara` (текст выписки; в primary.md хранить `link` на файл-источник в `stable_source`; используется как источник
    мастер-данных для граф 8/9/14/54: G_8_*, G_9_*, G_14_*, G_54_*; `DocumentBody_TextSection`)

- **Неформализуемые поля:**
  - 06: `doc_code` (04011 — константа)
  - 07: `doc_name` (`ВЫПИСКА ИЗ ЕГРЮЛ` — константа)
  - 08: `doc_number` (= `DocumentHead_DocumentNumber`)
  - 09: `doc_date` (= `DocumentHead_DocumentDate`)

- _audit: 9

---

## Раздел 4. `non_formalized` (Шаблоны неформализуемых документов)
Документы, которые не требуют генерации в XML. Содержат данные, необходимые для ДТ.

### 4.1 Storage Report / Отчет СВХ (ДО-1 / ДО-2) (10061/10062)

- **uqi_prefix:** `non_formalized.svh_[n]`
- **Зачем:** факты для граф 6, 30 и для товарных блоков ДТ (места/вес/стоимость в разрезе кодов ТН ВЭД — если присутствует в ДО).
- **Ключевые поля:**
  - 01: `warehouse_license_number` (номер лицензии/свидетельства СВХ; цель: графа 30: G_30_1)
  - 02: `warehouse_license_date` (дата лицензии/свидетельства СВХ; цель: графа 30: G_30_DATE)
  - 03: `actual_gross_weight` (фактический вес по весам; цель: сверка с общим брутто: графа 35: G_35_1 (контроль))
  - 04: `actual_places` (фактическое количество мест; цель: графа 6: G_6_1 (приоритет #1))
  - 05: `transport_reg_number` (номер ТС при въезде/по отчету СВХ; цель: сверка с графой 18: G_18)

- _audit: 5

- **Товары в разрезе строк ДО (если в документе есть таблица с разбиением):**
  - `goods_[n]`
    - 01: `tnved` (код товара; цель: сопоставление с товарными блоками ДТ)
    - 02: `places` (кол-во грузовых мест по строке; цель: `BLOCK/G_31/PLACE` и контроль графы 6)
    - 03: `gross_weight_kg` (вес брутто по строке; цель: `BLOCK/G_35_1`)
    - 04: `cost` (стоимость по строке; цель: `BLOCK/G_42_1` (контроль по валюте инвойса))
    - 05: `currency_code` (буквенный код валюты, напр. `CNY`; цель: контроль)

  - _item_audit: 5

**ЖЕСТКОЕ ПРАВИЛО (MUST):**
Если документ ДО-1/ДО-2 (Storage Report) найден, но в нем **нет разбивки по товарам / в разбивке не хватает данных**,
AI **ОБЯЗАН** материализовать массив `goods[n]` **как минимум одним элементом** (`goods_1`), заполнить поля `tnved`,
`places`, `gross_weight_kg`, `cost`, `currency_code` (value пусто) со `status: pending`, и добавить
соответствующую запись в `Issues` (причина: “в ДО проблема разбивки по товарам”).

### 4.2 Storage Report Additional Sheet

- **uqi_prefix:** `non_formalized.svh_additional_sheet_[n]`
- **Зачем:** адрес и код таможни СВХ для графы 30.
- **Ключевые поля:**
  - 01: `number` (№ доп.листа/приложения; цель: G_30P_1;)
  - 02: `date` (дата доп.листа; цель: G_30P_1; используется совместно с `number` при компоновке графы 30 печатной формы ДТ)
  - 03: `actual_gross_weight` (фактический вес по весам; цель: сверка с графой 35: G_35_1 (контроль))
  - 04: `actual_places` (фактическое количество мест; цель: уточнение/сверка для графы 6: G_6_1)
  - 05: `transport_reg_number` (номер ТС при въезде; цель: сверка с графой 18: G_18)
  - 06: `svh_address_region` (регион СВХ; цель: графа 30: G_30_SUB)
  - 07: `svh_address_city` (город/нас.пункт СВХ; цель: графа 30: G_30_CIT)
  - 08: `svh_address_street_house` (улица/дом СВХ как в отчете, без “улучшений”; цель: графа 30: G_30_STR)
  - 09: `svh_customs_code` (код таможенного органа в зоне СВХ; цель: графа 30: G_30_12)

- _audit: 9

### 4.3 Certificate of Origin / Сертификат происхождения (06013)

- **uqi_prefix:** `non_formalized.certificate_of_origin_[n]`
- **Зачем:** документ для графы 44 (если прикладывается) и для обоснования страны происхождения.
- **Поля:**
  - 01: `number` (номер сертификата; цель: графа 44: G44/G442)
  - 02: `date` (дата сертификата; цель: графа 44: G44/G443)

- _audit: 2

### 4.4 Conformity Document / Декларация о соответствии EAC (01191)

- **uqi_prefix:** `non_formalized.conformity_document_[n]`
- **Зачем:** документ для графы 44 (если прикладывается) и подтверждение требований (если применимо к товару).
- **Поля:**
  - 01: `number` (номер декларации/сертификата; цель: графа 44: G44/G442)
  - 02: `date_start` (дата начала действия; цель: обычно только для сверок/контекста, в dt.xml напрямую не переносится)
  - 03: `date_end` (дата окончания действия; цель: обычно только для сверок/контекста, в dt.xml напрямую не переносится)

- _audit: 3

### 4.5 Transit Declaration / Транзитная декларация (09013)
Не обязателен для поставки.
- **uqi_prefix:** `non_formalized.td_[n]`
- **Зачем:** источник данных для графы 29 ДТ (таможенный орган) + реквизиты документа для графы 44. Если ТД не
  прикладывается, то G44/G442 и G44/G443 остаются пустыми, а G_29_1, G_29_2 будут взяты из других документов.
- **Поля:**
  - 01: `number` (номер ТД; цель: G44/G442)
  - 02: `date` (дата ТД; цель: G44/G443)
  - 03: `customs_post_code` (код таможенного органа; цель: графа 29: G_29_1)
  - 04: `customs_post_name` (наименование таможенного органа; цель: графа 29: G_29_2)
  - 05: `transport_reg_number` (ТС по ТД; цель: сверка с графой 18: G_18)

- _audit: 5

### 4.6 Master Data / Мастер-данные констант (stable_data)
Те, что не вошли в основные документы (декларант, доверенность).
- **uqi_prefix:** `non_formalized.stable_data_[n]`
- **Зачем:** контактные данные декларанта и представителя для заполнения граф ДТ (графы 14, 54).
- **Поля:**
  - 01: `master_data.declarant.organization_name` (наименование декларанта, напр. `ООО "СКИФ"`)
  - 02: `master_data.declarant.phone` (телефон декларанта; цель: G14)
  - 03: `master_data.declarant.email` (email декларанта; цель: G14)
  - 04: `master_data.representative.name` (ФИО представителя, напр. `Анастасия Константиновна Арбузова`; цель: G54)
  - 05: `master_data.representative.phone` (телефон представителя; цель: G54)
  - 06: `master_data.representative.email` (email представителя; цель: G54)

- _audit: 6

---

## Раздел 5. Формат `primary.md`
Описывает правила формирования `primary.md`. Примеры фрагментов разметки приводятся в fenced blocks (```primary), но в сам
primary.md fenced blocks не переносятся.

### 5.1 Разделы `primary.md`
1) Метаданные
2) formalized
3) non_formalized
4) Итоги по файлу
5) Нерешенные вопросы (Issues)
6) `unreliable_fields`

```primary
# Первичные данные

## 1. meta:
- `название кейса`: <название кейса>
- `путь к папке поставки`: <путь к папке поставки>
- `direction`: <ИМ / ЭК> (импорт / экспорт) 
- `тип поставки`: <например: 1 ДТ / 1 товар>
- `источники данных:` <например: md + operator_provided_data + master_data + stable_source>

## 2. formalized / non_formalized:

### `document`: <тип документа>
  - `num`: <номер документа 1..N>
  - `uqi_prefix`: <префикс, например formalized.invoice_1>
  - `xml_target_root`: <корневой тег XML, если применим>
  - `path`: <путь к файлу>
  - `file_name`: <имя файла>
  - `note`: <(опционально) пояснение>
```

### 5.2 Таблицы полей
- Таблицы:
  - для каждого документа/массива строится отдельная таблица;
  - AI обязан материализовать все поля, указанные в шаблоне документа;
  - для пустых значений полей ячейка таблицы остается пустой;
  - если для поля не удалось установить значение, status=pending;
  - пустая строка перед заголовком (требование маркдауна).

- Сокращения:
  `S` (`status`):
  - `CD` - confirmed_document, подтвержденное значение, взято из исходных документов,
  - `CO` - confirmed_operator, значение явно задано оператором,
  - `pending` — данных недостаточно или есть конфликт.

- **Нумерация и контроль потерь (Жесткие индексы):**
  - Все поля в шаблонах пронумерованы в формате `NN: FieldName`, начиная с 1 без пропусков.
  - Индексы полей совпадают 1:1 в схеме и `primary.md`. AI **ОБЯЗАН** подставлять эти номера в таблицы полей  
    в том же порядке.
  - В конце списка полей документа и в конце каждого массива AI **ОБЯЗАН** поместить взятые из схемы строки
    (необходимо для скрипта проверки):
    - `_audit` (для документа),
    - `_item_audit` (для массива).

- Таблицы строятся для каждого документа, например:
  - `` `### `document`: Contract ``,
  - `` `### `document`: Supplementary Contract` ``,
- и для каждого элемента массива, например:
  - `#### InvoiceGoods[3]`,
  - `#### Goods[6]`.

- Формат таблицы:
```primary
пустая строка
| num          | field       | value             | status          | description         | note             |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| <номер поля> | <FieldName> | <value или пусто> | <CD/CO/pending> | <назначение поля>   | <note или пусто> |
```

### 5.3 Реквизиты полей
Реквизиты полей документов получают следующие значения:
- `value`: <значение> | `link`: <ссылка на первичный документ> (для длинных текстовых полей)
- `description`: краткое описание, извлекается из текста в скобках в полей шаблонов. Например, 
  поле шаблона: `` `DocumentCode` (03011 — код вида документа для графы 44: G44/G441; константа; derived)``, 
  здесь `description` = `код вида документа`.
- `note`: особенности, тонкости, замечания (не заполняется в режиме быстрой генерации, умолчание).

### 5.4 Link вместо большого текста  (описание, не материализуется)
Если по схеме допускается link (например ContractTerms_ContractText), то:
- value = `link:<relative_path>`
- status = CD

### 5.5 Если в md-документе поле взято из участка, помеченного как `unreliable_parts`,
то `status` → pending, `value` остается как было принято, поле добавляется в раздел `unreliable_fields`. 

### 5.6 После каждого документа:
**!!!ВАЖНО ДЛЯ СКРИПТА ПРОВЕРКИ!!!** - выводится опорная строка:
- _audit: <N> (N - число полей документа, взятое из шаблона).
- doc_status: <confirmed, pending>

Пример (Contract):
```primary
- _audit: 29
- `doc_status`: confirmed
```

### 5.7 Массивы

#### 5.7.1 Заголовок массива

```primary
#### Массив: <ArrayName>[N] (где N - размерность массива)
```

#### 5.7.2 Заголовок элемента массива
**!!!ВАЖНО ДЛЯ СКРИПТА ПРОВЕРКИ!!!** - выводится опорная строка:
  - _element_num: <i> (где i - порядковый номер элемента массива, начиная с 1)

```primary
##### <ArrayName>[i]
- _element_num: <i>
пустая строка
таблица полей этого элемента указанным выше форматом.
```

#### 5.7.3 После каждого элемента массива:
**!!!ВАЖНО ДЛЯ СКРИПТА ПРОВЕРКИ!!!** - выводится опорная строка:
  - _item_audit: <N> (N - число полей массива, взятое из схемы; )

Пример для элемента массива (Packing List, Goods[i]):
```primary
- _item_audit: 5
```

#### 5.7.4 После каждого массива:
**!!!ВАЖНО ДЛЯ СКРИПТА ПРОВЕРКИ!!!** - выводятся опорные строки:
  - _item_audit: <n> (n - число полей массива, взятое из схемы; )
  - _array_audit: N (где N - реальная размерность массива)
  - `array_status`: <confirmed/pending>

Пример для массива (Packing List, Goods[]):
```primary
- _item_audit: 5
- _array_audit: 2
- `array_status`: <confirmed/pending>
```

### 5.8 Итого, по файлу:
Полный итог:
```
### Итогo, по файлу:

`total_unreliable_fields`: n (число недостоверно распознанных полей)
`formalization_status`: <confirmed / pending>
```

### 5.9 Нерешенные вопросы (Issues)

```
## 5. Нерешенные вопросы (Issues)

**Для полей:**
- `<UQI поля со статусом pending>`
  - `question`: <текст вопроса AI>

**Для общих вопросов:**
- `[Общий]`
  - `question`: <текст вопроса AI>
```

### 5.10 `unreliable_fields`
Список полей, принятых из частей md-документов, помеченных как `unreliable_parts`.
```
## 6. `unreliable_fields`:
<список недостоверно распознанных полей>
```

---

## 6. Правила сборки

### 6.0 **ЖЕСТКОЕ ПРАВИЛО**
Поля в шаблонах идут с возрастающими на единицу номерами. В `primary.md` генерируй их с теми же номерами, номер **ВСЕГДА**
возрастает на единицу.

### 6.1 Запрет сокращений и симуляции:
  - **ЗАПРЕЩЕНА СОКРАЩЕННАЯ СБОРКА `primary.md`**. Запрещено исключение документов и полей документов, описанных в шаблонах. 
  - **ЗАПРЕЩЕНО** генерировать описание документа в `primary.md` **ПРИ ОТСУТСТВИИ ИСХОДНОГО ДОКУМЕНТА**.

### 6.2 Никаких догадок
Запрещено:
  - подставлять коды “на глаз”, используй справочники;
  - использовать как источник фактов новой поставки:
    - `alta\reference\...`, `...\выгрузки\...` (эталонные ДТ/xml/скриншоты);
    - результаты прошлых прогонов (`primary.md`, формализованные `*.xml`).
  - использовать динамические скрипты powershell. Где возможно, обходись средствами Хобота, иначе, согласуй с оператором.

AI обязан:
  - **`primary.md` — формальная база данных**. Все поля и все массивы, предусмотренные шаблоном документа, обязаны
    присутствовать, даже если `status: pending`.   
  - **Никаких догадок:** Если поле отсутствует или неоднозначно и в шаблоне не описаны другие действия, то `value` пустое,
    `status` = `pending` и фиксация в разделе нерешенных вопросов. Исключение - случаи, подпадающие под Раздел 3
    (контролируемое слияние).
  - Если master data (`alta\master_data\*`) конфликтуют с первичкой — фиксировать конфликт в разделе нерешенных вопросов.
  - **Предпочтение русских вариантов текста:**
    - если документ (например, контракт) продублирован на русском и английском языках, используй русскую версию,
    - визуально-эквивалентные буквы в номерах транспортных средств переводим в кириллицу (во всех документах, содержащих
      номера ТС),
    - если в документе одновременно встречаются и русские и английские названия стран, выбираем русский вариант.
  - **Очевидные решения.** Если md-файлы не имеют явно выделенного смыслового блока для заполнения поля шаблона,
    читай текст документа. Если ты можешь уверенно найти нужные, не выноси этот вопрос на обсуждение с оператором.
  - **Приоритет каталога `md` перед другими первичными документами:** сначала используй его, если имеется.
  - **Правило: Invoice consignor = seller.** Если в инвойсе нет отдельного consignor (shipper) — заполняй
    `formalized.invoice_[n].Consignor_*` значениями `formalized.invoice_[n].Seler_*` (внутри этого же документа).  
    `status: CD`, `note: нормализация: consignor=seller`.

### 6.3 Приоритет источников (детерминированно)
При выборе значения для целевого поля использовать первый подходящий источник из списка:

1. `operator_provided_data.md`
  - `status = CO`
  - `note = operator:<ключ/путь>`

2. `alta\master_data\master_data.md` (мастер-данные: ЕГРЮЛ, доверенность, паспорт, договор экспедиции)
  - `status = CD`
  - `note = master_data:<file>`

3. Первичка/md-документы текущей поставки
  - `status = CD`
  - `note = copied_from:<uqi_prefix>.<field> (<relative_path>)`

### 6.4 Контролируемое слияние

#### 6.4.1 Разрешение кросс-док заполнения
Поле любого документа в `primary.md` **разрешено** заполнять значением из другого документа, если:
- в целевом документе это поле пустое/отсутствует/нечитабельно,
- перенос не меняет смысл факта (это один и тот же факт поставки),
- перенос выполняется детерминированно по приоритетам (см. ниже),
- перенос **всегда** помечен в `note`.

#### 6.4.2 Правила кросс-док
Приоритеты:

- Адрес/данные продавца:
  `invoice` → `contract` → `cmr`

- Адрес/данные покупателя/получателя:
  `egrul` → `cmr` → `invoice` → `packing_list` → `payment_order`

- Веса/места/стоимость (контрольные итоги):
  `svh_additional_sheet` → `svh` → `packing_list` → `cmr` → `invoice`

- Транспортные средства (тягач/прицеп):
  `cmr` → `td` → `svh`

#### 6.4.3 Обязательная маркировка переноса
Любое значение, полученное не из “родного” документа, обязательно помечать:
- `note`:
  - `operator:<...>` или
  - `master_data:<file>` или
  - `copied_from:<uqi_prefix>.<field> (<relative_path>)`

#### 6.4.5 Конфликты
Если источники дают разные значения для одного поля:
- перенос не делается “молча”;
- конфликт фиксируется в `primary_review.md`;
- вопрос оператору.

### 6.5 Разрешения

- Для AI **РАЗРЕШЕНА** запись в `alta\stage_1.0_result\<кейс>\...`, файлы `primary.md`,
  `primary_review.md` пишутся в этот каталог.

- Для AI **РАЗРЕШЕНА** запись в `alta\source\<кейс>\operator\...`. Если был диалог с оператором, в этот
  каталог помещается файл `operator_provided_data.md`, содержащий информацию, которая может быть
  использована совместно с первичкой на следующих прогонах.

### 6.6 Инструменты доступа к файлам
- Для файлов документов использовать следующие способы доступа:
  - текстовые файлы: команды Хобота read_file, write_file;
  - xlsx, docx, png, pdf: перетаскивание в поле ввода;
  - xml, используемые для импорта / экспорта: read_file, write_file с параметром кодировки windows-1251;

### 6.7 Проверка скриптом `check_stage_result.bat` полностью/частично сформированного файла`primary.md`
Запусти скрипт `alta\service\script\check_stage_result.bat alta\stage_2.0_result\<case>\dt_fields.md`. Он проверит:
- корректность строк таблиц (разделители '|', количество колонок)
- корректность нумерации num (01..N без пропусков) внутри каждого блока
- сверка фактического числа полей с маркерами _audit/_item_audit
- некоторые ошибки форматирования:
  - перед заголовком таблицы есть пустая строка
  - внутри таблицы нет пустых строк (пустая строка считается концом таблицы)

Важно: скрипт корректно проверит только файл (полный или незаконченный), который завершен строками аудита: 
`_audit`/`_item_audit`/`_array_audit`.

---

## Раздел 7. Порядок работы (задание).

### 7.1 Разведка

#### 7.1.1 Инвентаризация каталогов
  - Прочитай/убедись, что структура следующих каталогов, включая размеры файлов, известна: 
    - `alta\source\<кейс>\...\<папка первички>\md\*.md`,
    - `alta\master_data\*`,
    - `alta\stable_source\*`,
    - `alta\source\<кейс>\...\<папка первички>\operator\operator_provided_data.md`, если есть,
    - `alta\prompt\codebook.md` - используется только, если нет данных в вырезках.

#### 7.1.2 Проверка наличия исходных документов
  - Если существует папка `alta\source\<кейс>\...\md\` и в ней есть файл  `doc_conversion_review.md` (содержит инвентарь),
  - если инвентарь совпадает с реальным содержимым папки,
  - то считать, что все доступные документы конвертированы в md формат и использовать их.
  - иначе, если эти условия не выполнены частично или полностью получить разрешение оператора на прямую работу с первичкой.

#### 7.1.3 Определение стратегии.
- Подхвати готовые `primary.md`, `primary_review.md`, если есть. Запроси у оператора нужна ли доработка или генерация с нуля? 
- Если оператор не запрашивал полную генерацию, работай в режиме быстрой генерации без заполнения `note`.

### 7.2 Генерация
сгенерируй/доработай `primary.md`:
  - Не используй плейсхолдеры. Пиши чанками: `write_file`, затем `write_file "utf-8" "append"`. Заканчивай каждый
    чанк строкой `_audit`/`_item_audit`. Это необходимо для правильной работы проверочного скрипта.

  - При каждой записи `primary.md`, целиком/части/завершении, **в этой же директиве AI ОБЯЗАН ЗАПУСТИТЬ ПРОВЕРОЧНЫЙ
    СКРИПТ `check_stage_result.bat`**, см. п. 5.5.

  - Используй patch_file только после сборки, для точечных фиксов.

### 7.3 Чек-лист: 

#### 7.3.1 Начало генерации:
  
  - ✅ Проверь, что мета-данные заполнены.

#### 7.3.2 В цикле генерации:

  - Перед выполнением каждой директивы записи/дозаписи `primary.md`:
    - ✅ Проверь совпадение имен полей со схемой.
    - ✅ Проверь, содержит ли директива вызов скрипта `check_stage_result.bat`.
    - ✅ Все документы материализованы согласно шаблонам, включая массивы и подмассивы.

  - После после выполнения директивы записи/дозаписи `primary.md`:
    - ✅ если выявлены ошибки, найди `_audit`/`_item_audit` последнего правильно сгенерированного раздела и перегенерируй
      хвост файла.
    - ✅ Пройди по всем полям со статусом `pending`, проверь, что поля, действительно, нельзя определить.

### 7.4 Завершение
  - **ПРОВЕРЬ, ЧТО БЫЛИ ПРИНЯТЫ ВСЕ md-ФАЙЛЫ** (или, в случае прямой генерации, все первичные документы).
  - Сгенерируй `primary_review.md`.
  - Совместно с оператором разреши `pending` для formalized полей. 
  - **Зафиксируй ответы в `operator_provided_data.md`**.
  - После разрешения `pendig`, выполни патчи/перегенерацию.

---

## ПРИЛОЖЕНИЕ. Вырезки из справочников
Полные справочники — в `alta\prompt\codebook.md`.

### Идентификаторы справочников (codebook)

| Идентификатор  | Описание                          | Графы                  |
|----------------|-----------------------------------|------------------------|
| `cb:procedure` | Классификатор таможенных процедур | 1.2, 37                |
| `cb:regime`    | Режимы движения товаров (ИМ/ЭК)   | 1.1                    |
| `cb:country`   | Страны и их коды                  | 2, 8, 9, 11, 14–17, 30 |
| `cb:unit`      | Единицы измерения                 | 31, 41                 |
| `cb:doc`       | Коды видов документов             | 44                     |
| `cb:payment`   | Виды платежей и способы расчётов  | 47                     |
| `cb:transport` | Виды и режимы транспорта          | 18, 21, 25, 26         |
| `cb:location`  | Типы местонахождения товаров      | 30                     |

---

### `cb:country` — Страны (вырезка)

| Код  | Alpha-2 | Наименование |
|------|---------|--------------|
| 156  | CN      | Китай        |
| 643  | RU      | Россия       |
| 112  | BY      | Беларусь     |
| 398  | KZ      | Казахстан    |

---

### `cb:unit` — Единицы измерения (вырезка)

| Код  | Наименование          |
|------|-----------------------|
| 055  | м² (квадратный метр)  |
| 166  | кг (килограмм)        |
| 796  | шт (штука)            |
| 163  | г (грамм)             |
| 168  | т (тонна)             |
| 006  | м (метр)              |
| 121  | м³ (кубический метр)  |
| 112  | л (литр)              |
| 798  | 1000 шт (тысяча штук) |
| 214  | кВт (киловатт)        |

