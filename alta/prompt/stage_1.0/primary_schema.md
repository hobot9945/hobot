# Инструкция по этапу 1.0: Сбор и нормализация фактов

## 1. Разделы файла `primary_schema.md`

0. **Раздел 0 Вводная часть**

1. **Раздел I `formalized` (Шаблоны формализуемых документов):** Документы, на базе которых на этапе 1.1 будут строиться 
   XML-файлы для Альты. Идентификаторы полей здесь и далее совпадают с целевыми XML тегами.

2. **Раздел II `non_formalized` (Шаблоны неформализуемых документов):** Документы, которые не требуют генерации в XML, 
   но содержат критичные факты для ДТ.

3. **Раздел III: Формат раздела нерешенных вопросов:** вопросы к оператору - пробелы, конфликты.

4. **Раздел IV: Порядок работы:** промпт по выполнению этапа.

## 0. РАЗДЕЛ 0 Вводная часть

### 1. Цель этапа

Полное извлечение фактов из папок `alta\source\<кейс>\...` и `alta\stable_source\`, а также
получение от оператора, недостающих данных и разрешение конфликтов.

Выход этапа оформляется либо в полном, либо в кратком варианте.

- В полном варианте сформировать базу фактов поставки в двух представлениях:
  - `primary.yaml` — машинный файл (источник истины для этапа 1.1 и этапа 2.0).
  - `primary.md` — человеческий файл для просмотра оператором (табличный формат).
- В кратком варианте сформировать только `primary.yaml`.

Важно: **primary.yaml — это не summary, а формальная база данных. Неполная структура = ошибка этапа.**

### 2 Источники данных

Используй `alta\source\<кейс>\...\md\` (конвертированная в md первичка), если есть. Если данных не хватает или они 
вызывают сомнения, обращайся к оригиналам (docx, pdf, png). Факт обращения к оригиналам при наличии md-версий фиксируй 
в ревью.

### 3. Общие принципы

1. **Нормализация:** primary.md является строго нормализованной структурной базой данных. Запрещено сокращать или 
   агрегировать структуру. Все поля, предусмотренные шаблоном, обязаны присутствовать в primary.md, даже если их 
   статус — pending.
 
2. **Блокеры:** для генерации формализованных xml документов на этапе 1.1 блокерами служат **формализуемые** поля со 
   статусом `pending`. Второй этап **не** блокируется вообще.
 
3. **Документная изоляция:** На этом этапе каждый документ обрабатывается независимо. AI не сопоставляет товарные 
   позиции между различными документами. Все расхождения фиксируются как конфликт в Разделе III, статус полей — `pending`.

4. **Никаких догадок:** Если поле отсутствует или неоднозначно — `value` пустое, `status` = `pending`. Обязательна 
   фиксация в Разделе III с конкретным вопросом к оператору.

5. В рабочем режиме AI не имеет права использовать как источник фактов новой поставки:
  - файлы из каталогов `alta\reference\...` и любых `...\выгрузки\...`;
  - эталонные ДТ, эталонные xml/скриншоты, результаты прошлых прогонов (`stage_*_result`, `trash`).

6. Если master data (`alta\stable_source\`) конфликтуют с первичкой — фиксировать конфликт в Разделе III.

7. **Предпочтение русских вариантов текста:** если документ (например, контракт) продублирован на русском и английском 
    языках, используй русскую версию.

8. **Приоритет каталога `md` перед другими первичными документами:** сначала используй его, если имеется.

## 4. UQI документов и пути к полям

Идентификаторы документов, например, `formalized.invoice_1`, `non_formalized.svh_1` (`[раздел].[тип_документа]_[n]`), 
используются как `uqi_prefix` для ссылок на поля: `<uqi_prefix>.<имя_поля>`, `<uqi_prefix>.<имя_массива>_[n].<имя_поля>`. 

## 5. Правила составления шаблонов документов и полей:

- **Формат поля документа в схеме:** занимает одну строку. Первым идет идентификатор, далее, в скобках, пояснения.

- **Идентификаторы полей:** Для формализуемых документов идентификатор совпадает с xml тегом. Для неформализуемых -
  snake_case.

- **Объемные тексты:** Если текст слишком большой (техописание, контракт), в `primary.md` хранится 
  `value: link:<путь_к_файлу>`. Полный текст будет подставлен на этапе 1.1.

- **Реквизиты:** Максимально разбивать адреса на компоненты. Если надежно разбить нельзя — сохранять исходную строку 
  в соответствующее поле и назначать статус `pending`.

---

## 6. РАЗДЕЛ I: Формализуемые документы (Шаблоны)

### Обязательные документы для текущей конфигурации (автотранспорт)

- Для формирования ДТ поставки автотранспортом обязательны:
  - Contract
  - Invoice
  - Если в документах присутствуют явные признаки автоперевозки (номер транспортного средства, ссылка на CMR, указание
    автомобильного транспорта), в наборе первички должен присутствовать CMR.
  - Любые другие документы, которые могут быть представлены приведенными в этом промпте шаблонами.

### Общее правило

Каждый формализуемый документ в `primary.md` может быть разделен на два смысловых слоя:

1. **Формализуемые поля** — данные, которые участвуют в последующем построении XML-документов Альты.  
   Для таких полей:
  - имя поля совпадает с тегом XML;
  - именно эти поля считаются источником для дальнейшей xml-формализации.

2. **Дополнительные нефомализуемые поля** — сведения, присутствующие в документе, не участвующие в построении XML,
   но необходимые для составления ДТ.

Рекомендуемое оформление таких данных:
- `value`: <значение> | `link`: <ссылка на первичный документ>
- `status`: <confirmed_document | confirmed_operator | pending>
- `note`: дополнительное поле, не используется для генерации xml

### 6.1. Contract / Контракт (03011)

- **xml_target_root:** `AltaE2CONT`
- **uqi_prefix:** `formalized.contract_[n]`

- **Поля:**
- **Поля:**
  - `DocumentCode` (03011 — код вида документа для графы 44: G44/G441)
  - `ContractRegistration_PrDocumentNumber` (№ контракта; графа 44: G44/G442)
  - `ContractRegistration_PrDocumentDate` (дата контракта; графа 44: G44/G443)

  - `ContractTerms_Amount` (общая сумма контракта; для контроля/сверки; в dt.xml обычно напрямую не печатается)
  - `ContractTerms_CurrencyCode` (цифровой код валюты ISO 4217 numeric; для контроля/сверки; напр. CNY=156)
  - `ContractTerms_LastDate` (срок действия/исполнения; для контроля/сверки)
  - `ContractTerms_OtherTerms` (условия поставки / Incoterms, напр. `EXW ...`; источник для графы 20: G_20_1)
  - `ContractTerms_ContractText` (текст контракта; в primary.md хранить `link` на файл-источник; в dt.xml напрямую не 
     переносится)
  - `ContractTerms_DealSign` (системный признак Альты; для импорта, обычно `1`)

  - `ForeignPerson_OrganizationName` (продавец/сторона контракта; обычно совпадает с отправителем; может использоваться 
     для сверок)
  - `ForeignPerson_Address_CountryCode` (страна продавца alpha-2 из `cb:country`; для сверок)
  - `ForeignPerson_Address_CounryName` (страна продавца, текст; **опечатка тега CounryName**; для сверок)
  - `ForeignPerson_Address_Region` (регион/область продавца; для сверок)
  - `ForeignPerson_Address_City` (город/район продавца; для сверок)
  - `ForeignPerson_Address_StreetHouse` (улица/дом продавца одной строкой; для сверок)

  - `RussianPerson_OrganizationName` (покупатель/сторона контракта; обычно совпадает с декларантом/получателем; для сверок)
  - `RussianPerson_OGRN` (ОГРН покупателя; для сверок/мастер-данных)
  - `RussianPerson_INN` (ИНН покупателя; для сверок/мастер-данных)
  - `RussianPerson_KPP` (КПП покупателя; для сверок/мастер-данных)
  - `RussianPerson_Address_PostalCode` (индекс покупателя; для сверок/мастер-данных)
  - `RussianPerson_Address_CountryCode` (страна покупателя alpha-2; для сверок/мастер-данных)
  - `RussianPerson_Address_CounryName` (страна покупателя, текст; **опечатка тега CounryName**; для сверок/мастер-данных)
  - `RussianPerson_Address_Region` (регион покупателя; для сверок/мастер-данных)
  - `RussianPerson_Address_City` (город покупателя; для сверок/мастер-данных)
  - `RussianPerson_Address_StreetHouse` (улица/дом/офис одной строкой; для сверок/мастер-данных)

**Примечание:**
`ContractTerms_ContractText` в `primary.md` не копировать полный текст контракта, сохранять только `link`
на файл-источник. Полный текст подставлять только при генерации XML. Нужно для сохранения компактности `primary.md`.

### 6.2. Supplementary Contract / Дополнительное соглашение к контракту (03012)

- **xml_target_root:** `AltaSupplementaryContract`
- **uqi_prefix:** `formalized.supplementary_contract_[n]`

- **Поля:**
  - `DocumentNumber` (№ доп. соглашения; графа 44: G44/G442)
  - `IssueDate` (дата доп. соглашения; графа 44: G44/G443)

  - `ContractDescription_Amount` (новая/уточненная сумма контракта; для контроля/сверки)
  - `ContractDescription_CurrencyCode` (цифровой код валюты ISO 4217 numeric; для контроля/сверки)
  - `ContractDescription_LastDate` (новый срок действия/исполнения; для контроля/сверки)
  - `ContractDescription_ContractText` (текст доп. соглашения; в primary.md хранить `link` на файл-источник; в dt.xml 
     напрямую не переносится)
  - `ContractDescription_DealSign` (системный признак Альты; для импорта, обычно `1`)
  - `ContractDescription_StockCategorySign` (системный признак Альты; для импорта, обычно `0` если не используется)
  - `ContractDescription_BuyerLimitationSign` (системный признак Альты; для импорта, обычно `0` если не используется)
  - `ContractDescription_InsuranceSign` (системный признак Альты; для импорта, обычно `0` если не используется)

  - `RussianPerson_OrganizationName` (российская сторона; покупатель; для сверок/мастер-данных)
  - `RussianPerson_ShortName` (краткое наименование; для сверок/мастер-данных)
  - `RussianPerson_OGRN` (ОГРН; для сверок/мастер-данных)
  - `RussianPerson_INN` (ИНН; для сверок/мастер-данных)
  - `RussianPerson_KPP` (КПП; для сверок/мастер-данных)
  - `RussianPerson_Address_PostalCode` (индекс; для сверок/мастер-данных)
  - `RussianPerson_Address_CountryCode` (страна alpha-2; для сверок/мастер-данных)
  - `RussianPerson_Address_CounryName` (страна, текст; **опечатка тега CounryName**; для сверок/мастер-данных)
  - `RussianPerson_Address_Region` (регион; для сверок/мастер-данных)
  - `RussianPerson_Address_City` (город; для сверок/мастер-данных)
  - `RussianPerson_Address_StreetHouse` (улица/дом одной строкой; для сверок/мастер-данных)

  - `ForeignPerson_OrganizationName` (иностранная сторона; продавец; для сверок)
  - `ForeignPerson_ShortName` (краткое наименование; для сверок)
  - `ForeignPerson_Address_CountryCode` (страна alpha-2 из `cb:country`; для сверок)
  - `ForeignPerson_Address_CounryName` (страна, текст; **опечатка тега CounryName**; для сверок)
  - `ForeignPerson_Address_Region` (регион; для сверок)
  - `ForeignPerson_Address_City` (город/район; для сверок)
  - `ForeignPerson_Address_StreetHouse` (улица/дом одной строкой; для сверок)

  - `ContractSignedPerson` (подписант доп. соглашения; для сверок/аудита)
    - `PersonSurname` (фамилия подписанта)
    - `PersonName` (имя подписанта)
    - `PersonMiddleName` (отчество подписанта)

### 6.3. Invoice (04021)

- **xml_target_root:** `AltaE2I`
- **uqi_prefix:** `formalized.invoice_[n]`

- **Поля шапки / реквизиты документа:**
  - `CurrencyRate` (курс валюты; источник для графы 23: G_23_1, G_23_2)
  - `CurrencyCode` (валюта инвойса ISO 4217 alpha-3, напр. `CNY`, `USD`; источник для графы 22: G_22_3)
  - `DocumentCode` (04021 — код вида документа для графы 44: G44/G441)
  - `PlacesQuantity` (кол-во грузовых мест по инвойсу; приоритет #3 для графы 6: G_6_1)
  - `PlacesDescription` (описание мест, напр. "Поддон"; для сверок/контекста, обычно не в dt.xml напрямую)
  - `GrossWeightQuantity` (общий вес брутто по инвойсу; для сверок с CMR/PL/СВХ)
  - `NetWeightQuantity` (общий вес нетто по инвойсу; для сверок)
  - `GCost` (системное поле Альты; дубль `TotalCost`; для импорта/совместимости)
  - `TotalCost` (итого по инвойсу; источник для графы 22: G_22_2)

  - `DeliveryTerms_DeliveryPlace` (место поставки по Incoterms; источник для графы 20: G_20_2)
  - `DeliveryTerms_DeliveryTermsNumericCode` (числовой код условий поставки; источник для графы 20: G_20_1_1 / внутренний 
     код Альты)
  - `DeliveryTerms_DeliveryTermsStringCode` (строковый код условий, напр. `EXW`; источник для графы 20: G_20_1)
  - `DeliveryTerms_DispatchCountryCode` (страна отправления alpha-2; источник для графы 15A: G_15A_1)
  - `DeliveryTerms_TradingCountryCode` (торгующая страна alpha-2; источник для графы 11: G_11_1)
  - `DeliveryTerms_DestinationCountryCode` (страна назначения alpha-2; источник для графы 17A: G_17A_1)

  - `Registration_PrDocumentName` (наименование документа для печати/сверок; может использоваться в графе 44: G44/G444)
  - `Registration_PrDocumentNumber` (номер инвойса; графа 44: G44/G442)
  - `Registration_PrDocumentDate` (дата инвойса; графа 44: G44/G443)

  - `Contract_PrDocumentNumber` (№ контракта-ссылки; для связи документов; графа 44: G44/G442 (для контракта))
  - `Contract_PrDocumentDate` (дата контракта-ссылки; для связи документов; графа 44: G44/G443 (для контракта))

- **Стороны (местами теги “кривые” — это часть структуры Альты):**
  - `Buyer_CompanyID` (ИНН покупателя; источник для мастер-данных; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Buyer_KPPCode` (КПП покупателя; источник для мастер-данных; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Buyer_Name` (наименование покупателя; графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM)
  - `Buyer_PostalAddress_PostalCode` (индекс покупателя; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS)
  - `Buyer_PostalAddress_CountryCode` (страна покупателя alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC)
  - `Buyer_PostalAddress_CounryName` (страна покупателя, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, 
     G_14_CN)
  - `Buyer_PostalAddress_Region` (регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB)
  - `Buyer_PostalAddress_City` (город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT)
  - `Buyer_PostalAddress_StreetHouse` (улица/дом/офис одной строкой; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR)

  - `Seler_Name` (продавец; **опечатка Seler**; источник для графы 2: G_2_NAM)
  - `Seler_PostalAddress_CountryCode` (страна продавца alpha-2 из `cb:country`; графа 2: G_2_7)
  - `Seler_PostalAddress_CounryName` (страна продавца, текст; **опечатка CounryName**; графа 2: G_2_50)
  - `Seler_PostalAddress_Region` (регион продавца; графа 2: G_2_SUB)
  - `Seler_PostalAddress_City` (город/район продавца; графа 2: G_2_CIT)
  - `Seler_PostalAddress_StreetHouse` (улица/дом одной строкой; графа 2: G_2_STR)

  - `Consignor_OrganizationName` (грузоотправитель; если отличается от продавца — для сверок/графы 2)
  - `Consignor_Address_CountryCode` (страна грузоотправителя alpha-2 из `cb:country`; для сверок)
  - `Consignor_Address_CounryName` (страна грузоотправителя, текст; **опечатка CounryName**; для сверок)
  - `Consignor_Address_Region` (регион; для сверок)
  - `Consignor_Address_City` (город/район; для сверок)
  - `Consignor_Address_StreetHouse` (улица/дом одной строкой; для сверок)

  - `Consignee_OrganizationName` (грузополучатель; обычно получатель/декларант; графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM)
  - `Consignee_OGRN` (ОГРН; графы 8/9/14: G_8_1, G_9_1, G_14_1)
  - `Consignee_INN` (ИНН; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Consignee_KPP` (КПП; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Consignee_Address_PostalCode` (индекс; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS)
  - `Consignee_Address_CountryCode` (страна alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC)
  - `Consignee_Address_CounryName` (страна, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, G_14_CN)
  - `Consignee_Address_Region` (регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB)
  - `Consignee_Address_City` (город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT)
  - `Consignee_Address_StreetHouse` (улица/дом/офис одной строкой; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR)

- **Товарные позиции** (каждый элемент соответствует узлу `<InvoiceGoods>...</InvoiceGoods>`; источник для блока товаров 
  dt.xml: `BLOCK/TOVG/TXT`):
  - `InvoiceGoods_[n]`
    - `GoodsCode` (код ТН ВЭД; источник для графы 33: G_33_1)
    - `GoodsDescription` (описание товара как в инвойсе; источник для графы 31: G_31/NAME и для строк дополнения TXT/TEXT)
    - `GoodsQuantity` (кол-во по строке инвойса в “основной” единице строки; для сверок; не использовать
       как TOVG/KOLVO, если в инвойсе есть отдельная колонка доп.кол-ва)
    - `goods_supplementary_quantity` (количество в доп.ед.изм для ДТ; например, `Quantity in M2`; неформализуемое поле)
    - `goods_supplementary_uom_name` (наименование доп.ед.изм из `cb:unit`; неформализуемое поле)    
    - `MeasureUnitQualifierName` (единица измерения доп.количества для ДТ, наименование из `cb:unit`; цель: TOVG/NAME_EDI)
    - `GrossWeightQuantity` (брутто по строке; источник для веса: G_35_1 (агрегация) и TOVG/G31_35)
    - `NetWeightQuantity` (нетто по строке; источник для веса: G_38_1 (агрегация) и TOVG/G31_38)
    - `Price` (цена за единицу; для сверок/контроля; обычно не переносится в dt.xml напрямую)
    - `TotalCost` (стоимость по строке; источник для графы 42 (агрегация) и TOVG/INVOICCOST)
    - `OriginCountryCode` (цифровой код страны происхождения; источник для графы 34 после нормализации в alpha-2: G_34_1)

    - `AdditionalGoodsDescription_Manufacturer` (производитель; источник для графы 31: G_31/FIRMA и TOVG/G31_11)
    - `AdditionalGoodsDescription_TradeMark` (товарная марка/ТМ; источник для графы 31: G_31/TM и TOVG/G31_12; если 
      отсутствует в первичке — "ОТСУТСТВУЕТ")
    - `AdditionalGoodsDescription_GoodsMark` (товарный знак/маркировка; источник для графы 31 и TOVG/G31_14; если 
      отсутствует — "ОТСУТСТВУЕТ")
    - `AdditionalGoodsDescription_GoodsModel` (модель/модификация; источник для графы 31 и TOVG/G31_15_MOD)

### 6.4. Packing List / Упаковочный лист (04131)

- **xml_target_root:** `AltaE2PACK`
- **uqi_prefix:** `formalized.packing_list_[n]`

- **Поля:**
  - `GrossWeightQuantity` (общий вес брутто по упаковочному; используется для сверок; может участвовать в derived 
    по графам 35/38 при необходимости)
  - `NetWeightQuantity` (общий вес нетто по упаковочному; используется для сверок)

  - `Consignor_OrganizationName` (грузоотправитель; для сверок с инвойсом/CMR)
  - `Consignor_ShortName` (краткое наименование; для сверок)
  - `Consignor_Address_CountryCode` (страна грузоотправителя alpha-2, используй `cb:country`; для сверок)
  - `Consignor_Address_CounryName` (страна грузоотправителя, текст; **опечатка CounryName**; для сверок)
  - `Consignor_Address_Region` (регион; для сверок)
  - `Consignor_Address_City` (город/район; для сверок)
  - `Consignor_Address_StreetHouse` (улица/дом одной строкой; для сверок)

  - `Consignee_OrganizationName` (грузополучатель; для сверок/мастер-данных)
  - `Consignee_ShortName` (краткое наименование; для сверок/мастер-данных)
  - `Consignee_OGRN` (ОГРН; для сверок/мастер-данных → графы 8/9/14: G_8_1, G_9_1, G_14_1)
  - `Consignee_INN` (ИНН; для сверок/мастер-данных → графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Consignee_KPP` (КПП; для сверок/мастер-данных → графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Consignee_Address_PostalCode` (индекс; для сверок/мастер-данных → G_8_POS, G_9_POS, G_14_POS)
  - `Consignee_Address_CountryCode` (страна alpha-2; для сверок/мастер-данных → G_8_7, G_9_CC, G_14_CC)
  - `Consignee_Address_CounryName` (страна, текст; **опечатка CounryName**; для сверок/мастер-данных → G_8_50, G_9_CN, 
    G_14_CN)
  - `Consignee_Address_Region` (регион; для сверок/мастер-данных → G_8_SUB, G_9_SUB, G_14_SUB)
  - `Consignee_Address_City` (город; для сверок/мастер-данных → G_8_CIT, G_9_CIT, G_14_CIT)
  - `Consignee_Address_StreetHouse` (улица/дом/офис одной строкой; для сверок/мастер-данных → G_8_STR, G_9_STR, G_14_STR)

  - `DeliveryTerms_DeliveryPlace` (место поставки по Incoterms; источник для графы 20: G_20_2)
  - `DeliveryTerms_DeliveryTermsNumericCode` (внутренний числовой код условий; источник для графы 20: G_20_1_1 / внутренний 
    код Альты)
  - `DeliveryTerms_DeliveryTermsStringCode` (строковый код условий, напр. `EXW`; источник для графы 20: G_20_1)

  - `DeliveryTerms_Contract_PrDocumentName` (наименование контракта для печати/графы 44: G44/G444)
  - `DeliveryTerms_Contract_PrDocumentNumber` (№ контракта; графа 44: G44/G442)
  - `DeliveryTerms_Contract_PrDocumentDate` (дата контракта; графа 44: G44/G443)

  - `DeliveryTerms_Invoice_PrDocumentName` (наименование инвойса для печати/графы 44: G44/G444)
  - `DeliveryTerms_Invoice_PrDocumentNumber` (№ инвойса; графа 44: G44/G442)
  - `DeliveryTerms_Invoice_PrDocumentDate` (дата инвойса; графа 44: G44/G443)

  - `DeliveryTerms_Registration_PrDocumentName` (наименование упаковочного; графа 44: G44/G444)
  - `DeliveryTerms_Registration_PrDocumentNumber` (№ упаковочного; графа 44: G44/G442)
  - `DeliveryTerms_Registration_PrDocumentDate` (дата упаковочного; графа 44: G44/G443)

- Неформализуемые поля (для ДТ, не для XML упаковочного листа)
  - `registration_doc_name` (наименование документа для графы 44: G44/G444; напр. `УПАКОВОЧНЫЙ ЛИСТ`)
  - `registration_doc_number` (номер документа для графы 44: G44/G442; если в первичке “Б/Н” — так и писать)
  - `registration_doc_date` (дата документа для графы 44: G44/G443)

- **Товарные/грузовые строки** (каждый элемент соответствует узлу `<Goods>...</Goods>`; это строки “по местам/грузовым 
  единицам”, не по товарам ДТ):
  - `Goods_[n]`
    - `GoodsDescription` (описание строки как в документе; для сверок/контекста; может быть агрегированным текстом)
    - `GoodsQuantity` (количество мест/грузовых единиц в строке; источник приоритета #2/#3 для графы 6: G_6_1 через derived)
    - `GrossWeightQuantity` (брутто по строке; для сверок)
    - `NetWeightQuantity` (нетто по строке; для сверок)
    - `PackingInfo`
      - `PakingQuantity` (кол-во упаковок/мест в упаковке; **опечатка PakingQuantity**; в эталонах может быть 0/пусто)

- **Транспорт** (каждый элемент соответствует узлу `<TransportMeans>...</TransportMeans>`; источник для графы 18: G_18 
  и связанных derived-полей):
  - `TransportMeans_[n]`
    - `Number` (регистрационный номер; источник для графы 18: G_18)
    - `ModeCode` (код вида транспорта; источник для граф 25/26: G_25_1, G_26_1; для автосостава обычно 31)
    - `NationalityCode` (код “национальности” ТС в структуре Альты; для сверок/совместимости, в эталонах может быть `000`)
    - `MoverIndicator` (`true` для тягача, `false` для прицепа; нужно для порядка/логики формирования G_18)

**Правило:** если известны номер тягача и номер прицепа, сохранять их как ДВА элемента:
  - transport_1 (MoverIndicator=true) — тягач
  - transport_2 (MoverIndicator=false) — прицеп

### 6.5. CMR / Международная товарно-транспортная накладная (02015)

CMR является транспортным документом и может не содержать детализацию товаров (в отличие от Invoice).

- **xml_target_root:** `AltaE3CMR`
- **uqi_prefix:** `formalized.cmr_[n]`

- **Поля:**
  - `LanguageCode` (язык документа; для импорта/совместимости; обычно `RU`)
  - `CMR_Choice` (системный выбор/вариант Альты; для импорта, обычно `1`)

  - `RegistrationDocument_RegID` (номер CMR; графа 44: G44/G442)
  - `RegistrationDocument_DateInf` (дата CMR; графа 44: G44/G443)
  - `RegistrationDocument_Place` (место составления; для сверок/контекста)

  - `TrakingCargo_TakingCargoDate` (дата CMR; **опечатка TrakingCargo**; для сверок/контекста)
  - `TrakingCargo_TakingCargoPlace_CountryCode` (страна принятия груза alpha-2; для сверок/контекста)
  - `TrakingCargo_TakingCargoPlace_CounryName` (страна принятия груза, текст; **опечатка CounryName**; для сверок/контекста)

  - `DeliveryPlace_CountryCode` (страна доставки alpha-2; для сверок/контекста)
  - `DeliveryPlace_CounryName` (страна доставки, текст; **опечатка CounryName**; для сверок/контекста)

  - `DeliveryTerms_DeliveryPlace` (место поставки по Incoterms, если указано в CMR; источник/сверка для графы 20: G_20_2)
  - `DeliveryTerms_DeliveryTermsStringCode` (условия поставки, напр. `EXW`; источник/сверка для графы 20: G_20_1)

  - `GoodsQuantity` (общее количество грузовых мест/упаковок по CMR; для сверки с графой 6: G_6_1 и с инвойсом/PL)
  - `CMRGoodsWeight_GrossWeightQuantity` (общий вес брутто по CMR; ключевой источник сверки брутто для графы 35: G_35_1 
    (агрегации))

  - `CMRTransport_PrimeMoverStateSignID` (гос. номер тягача; источник/сверка для графы 18: G_18)
  - `CMRTransport_TrailerStateSignID` (гос. номер прицепа; источник/сверка для графы 18: G_18)

- **Отправитель (как в структуре Альты):**
  - `Consignor_NameInf` (наименование; для сверок с инвойсом/контрактом)
  - `Consignor_ShortName` (краткое наименование; для сверок)
  - `Consignor_PostalAddress_CountryCode` (страна alpha-2; для сверок)
  - `Consignor_Address_CounryName` (страна, текст; **опечатка CounryName**; для сверок)
  - `Consignor_Address_Region` (регион; для сверок)
  - `Consignor_Address_City` (город/район; для сверок)
  - `Consignor_Address_StreetHouse` (улица/дом одной строкой; для сверок)

- **Гарант отправителя** (если присутствует в структуре; для сверок/аудита, обычно не переносится в dt.xml напрямую):
  - `Consignor_Guarantee_OrganizationName` (наименование гаранта)
  - `Consignor_Guarantee_ShortName` (краткое наименование)
  - `Consignor_Guarantee_Address_CountryCode` (страна alpha-2)
  - `Consignor_Guarantee_Address_CounryName` (страна, текст; **опечатка CounryName**)
  - `Consignor_Guarantee_Address_Region` (регион)
  - `Consignor_Guarantee_Address_City` (город/район)
  - `Consignor_Guarantee_Address_StreetHouse` (улица/дом одной строкой)

- **Получатель:**
  - `Consignee_NameInf` (наименование получателя; для сверок/мастер-данных → графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM)
  - `Consignee_ShortName` (краткое наименование; для сверок)
  - `Consignee_OGRNID` (ОГРН; **суффикс ID — часть тега Альты**; графы 8/9/14: G_8_1, G_9_1, G_14_1)
  - `Consignee_INNID` (ИНН; **суффикс ID — часть тега Альты**; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Consignee_KPPCode` (КПП; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Consignee_PostalAddress_PostalCode` (индекс; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS)
  - `Consignee_PostalAddress_CountryCode` (страна alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC)
  - `Consignee_Address_CounryName` (страна, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, G_14_CN)
  - `Consignee_Address_Region` (регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB)
  - `Consignee_Address_City` (город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT)
  - `Consignee_Address_StreetHouse` (улица/дом/офис одной строкой; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR)

- **Товарные/грузовые строки** (каждый элемент соответствует узлу `<CMRGoods>...</CMRGoods>`; это строки 
  “по местам/упаковкам”, не по товарам ДТ):
  - `CMRGoods_[n]`
    - `GoodsNumeric` (порядковый номер строки)
    - `GoodsDescription` (описание груза/товара как в CMR; для сверок и при необходимости для дополнения к графе 31)
    - `GoodsPackingInfo`
      - `PakingQuantity` (кол-во упаковок/мест; **опечатка PakingQuantity**; для сверок/контекста)
  
**Правило (без детализации товаров):** Если в CMR отсутствует детализация товаров:
  - формируется одна строка CMRGoods_1;
  - отсутствующие поля не заполняются и не считаются pending.
 
**Правило (строки):** если в CMR перечислены несколько строк `CMRGoods` — материализовать их все как `CMRGoods_1..CMRGoods_N`.
Не агрегировать в одну строку.

**Правило (гарант):** если блок `Consignor_Guarantee_*` отсутствует в первичке/не читается — ставить `pending` и вынести
вопрос в Раздел III. Не подставлять “ОТСУТСТВУЕТ” автоматически (только по явному решению оператора).

### 6.6. Payment Order / Платежное поручение (04023)

- **xml_target_root:** `AltaPaymentOrder`
- **uqi_prefix:** `formalized.payment_order_[n]`

- **Поля:**
  - `DocumentCode` (04023 — код вида документа для графы 44: G44/G441)

  - `PaymentModeCode` (системный код способа платежа в структуре Альты; для импорта/совместимости)
  - `PaymentAmount` (сумма платежа; для сверок с оплатой по поставке; обычно не переносится в dt.xml напрямую)
  - `TransactionKind` (вид операции/код; системное поле Альты; для импорта/совместимости)
  - `Priority` (очередность; системное поле; в эталонах может быть `"."`; для импорта/совместимости)
  - `Purpose` (назначение платежа; содержит ссылки на контракт/инвойс; используется для сверки связей документов)
  - `ValueSpelledOut` (сумма прописью; для сверок/контекста)

  - `DocumentReference_PrDocumentNumber` (номер платежного поручения; графа 44: G44/G442)
  - `DocumentReference_PrDocumentDate` (дата платежного поручения; графа 44: G44/G443)

  - `Payer_OrganizationName` (плательщик; для сверок/контекста)
  - `Payer_INN` (ИНН плательщика; для сверок)
  - `Payer_KPP` (КПП плательщика; для сверок)
  - `Payer_Bank_BankName` (в теге часто лежит блок реквизитов/адреса; может быть многострочным; сохранять как есть; 
    для сверок/контекста)

  - `Payee_OrganizationName` (получатель платежа; может быть многострочным/с переносами; сохранять как есть; для 
    сверок/контекста)
  - `Payee_Bank_BankName` (реквизиты банка получателя; может быть многострочным; сохранять как есть; для сверок/контекста)

  - `PayerSign` (подписант/плательщик; для сверок/аудита)
    - `PersonSurname` (фамилия)
    - `PersonName` (имя)


### 6.7. Service Invoice / Счет за перевозку (04031)

- **xml_target_root:** `AltaServiceInvoice`
- **uqi_prefix:** `formalized.service_invoice_[n]`

- **Поля:**
  - `DocumentSign` (системный признак документа Альты; для импорта/совместимости, обычно `1`)
  - `TotalServiceCost` (итого по услугам; для расчётов/сверок, при необходимости может участвовать в графах стоимости)
  - `Currency` (валюта итого ISO 4217 alpha-3; для расчётов/сверок)

  - `ServiceProvider_Name` (исполнитель услуг/перевозчик; для сверок/контекста)
  - `ServiceProvider_PaymentRequisitions`
    - `BankName` (банк исполнителя; для сверок/контекста)

  - `ContractDetails_PrDocumentNumber` (№ договора на услуги/перевозку; графа 44: G44/G442 (если прикладывается как документ))
  - `ContractDetails_PrDocumentDate` (дата договора на услуги/перевозку; графа 44: G44/G443)

  - `PaymentDocument` (связанный документ/заказ в структуре Альты; используется для связи документов/сверок)
    - `PrDocumentNumber` (номер; графа 44: G44/G442)
    - `PrDocumentDate` (дата; графа 44: G44/G443)

  - `Registration_PrDocumentName` (наименование счета; графа 44: G44/G444)
  - `Registration_PrDocumentNumber` (номер счета; графа 44: G44/G442)
  - `Registration_PrDocumentDate` (дата счета; графа 44: G44/G443)

  - `Consignor_OrganizationName` (грузоотправитель; для сверок/контекста)
  - `Consignor_SubjectAddressDetails`
    - `PostalCode` (индекс; для сверок)
    - `CountryCode` (страна alpha-2; для сверок)
    - `CounryName` (страна, текст; **опечатка CounryName**; для сверок)
    - `Region` (регион; для сверок)
    - `Town` (город/район; тег отличается от `City`; для сверок)
    - `StreetHouse` (улица/дом одной строкой; для сверок)

  - `Consignee_OrganizationName` (грузополучатель; для сверок/мастер-данных → графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM)
  - `Consignee_RFOrganizationFeatures_OGRN` (ОГРН; графы 8/9/14: G_8_1, G_9_1, G_14_1)
  - `Consignee_RFOrganizationFeatures_INN` (ИНН; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Consignee_RFOrganizationFeatures_KPP` (КПП; графы 8/9/14: G_8_6, G_9_4, G_14_4)
  - `Consignee_SubjectAddressDetails`
    - `PostalCode` (индекс; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS)
    - `CountryCode` (страна alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC)
    - `CounryName` (страна, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, G_14_CN)
    - `Region` (регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB)
    - `Town` (город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT)
    - `StreetHouse` (улица; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR)
    - `House` (дом; графы 8/9/14: G_8_BLD, G_9_BLD, G_14_BLD)
    - `Room` (офис/кв; графы 8/9/14: G_8_ROM, G_9_ROM, G_14_ROM)

- **Услуги** (каждый элемент соответствует узлу `<ServiceDescription>...</ServiceDescription>`):
  - `ServiceDescription_[n]`
    - `GoodsDescription` (многострочное описание услуги — сохранять как есть; **может отсутствовать** в отдельных строках)
    - `CurrencyCode` (валюта строки ISO alpha-3; для сверок/расчётов)
    - `ServiceName` (наименование/маршрут; для сверок/контекста)
    - `TaxRate` (ставка налога; для сверок/расчётов)
    - `TaxSum` (сумма налога; для сверок/расчётов)
    - `ServiceCost_Amount` (стоимость строки; для сверок/расчётов)
    - `ServiceCost_Currency` (валюта стоимости строки; для сверок/расчётов)

- **Подписи** (системный блок структуры Альты; для импорта/совместимости):
  - `Signature_Choice` (вариант подписи; системное поле)
  - `SignatureDirectorChiefAccountant_Director_PersonSurname` (фамилия руководителя)
  - `SignatureDirectorChiefAccountant_Director_PersonName` (инициалы/имя руководителя)
  - `SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname` (фамилия бухгалтера)
  - `SignatureDirectorChiefAccountant_ChiefAccountant_PersonName` (инициалы/имя бухгалтера)

### 6.8. Insurance Document / Счет за страховку (04111)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.insurance_document_[n]`

- **Поля:**
  - `DocumentCode` (04111 — код вида документа для графы 44: G44/G441)
  - `DocumentHead_DocumentName` (наименование документа; графа 44: G44/G444)
  - `DocumentHead_DocumentDate` (дата документа; графа 44: G44/G443)
  - `DocumentHead_DocumentNumber` (номер документа; графа 44: G44/G442)

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (основной текст/условия; в primary.md хранить `link` на файл-источник; в dt.xml обычно не переносится)

### 6.9. TechDescription / Техническое описание (05999)

Наличие нескольких технических описаний для разных товаров допустимо. Но, если несколько технических описаний 
относятся к одному и тому же товару (совпадает наименование, модель или явная ссылка на товар) или не может быть
соотнесено с товаром, AI не делает предположений об их релевантности и обязан вынести вопрос в Раздел III.

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.tech_description_[n]`

- **Поля:**
  - `DocumentCode` (05999 — код вида документа для графы 44: G44/G441)
  - `DocumentHead_DocumentName` (наименование техописания; графа 44: G44/G444)
  - `DocumentHead_DocumentDate` (дата техописания; графа 44: G44/G443)
  - `DocumentHead_DocumentNumber` (номер техописания; графа 44: G44/G442)

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (технический текст без пересказа; в primary.md хранить `link` на файл-источник; используется для 
      дополнения/обоснований к графе 31: G_31 (через TXT/TEXT при необходимости))

### 6.10. FreeDoc / Прочие текстовые документы (09999)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.free_doc_[n]`

- **Поля:**
  - `DocumentCode` (09999 — код вида документа для графы 44: G44/G441)
  - `DocumentHead_DocumentName` (наименование документа; графа 44: G44/G444)
  - `DocumentHead_DocumentDate` (дата документа; графа 44: G44/G443)
  - `DocumentHead_DocumentNumber` (номер документа; графа 44: G44/G442)

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (основной текст; в primary.md хранить `link` на файл-источник; в dt.xml обычно не переносится)

### 6.11. FreeBinaryDoc / Бинарное приложение (например PDF-вложение)

- **xml_target_root:** `AltaFreeBinaryDoc`
- **uqi_prefix:** `formalized.free_binary_doc_[n]`

- **Поля:**
  - `DocumentCode` (код вида документа; для графы 44: G44/G441; если неизвестен — уточнить по `cb:doc`, иначе `pending`)
  - `DocumentInfo_PrDocumentName` (наименование вложения; графа 44: G44/G444)
  - `DocumentInfo_PrDocumentNumber` (номер документа-основания/вложения; графа 44: G44/G442)
  - `DocumentInfo_PrDocumentDate` (дата документа-основания/вложения; графа 44: G44/G443)

  - `DocumentBody_FileName` (имя файла вложения; для импорта/контроля)
  - `DocumentBody_FileData` (base64 содержимого; **в primary.md не хранить base64**, хранить `link` на исходный файл; 
    base64 формировать только при генерации XML)
  - `Thumbnail` (миниатюра/base64; если не требуется для импорта — не материализовывать)

### 6.12. Personal Passport / Паспорт (11001)

- **xml_target_root:** `AltaPassport`
- **uqi_prefix:** `formalized.passport_[n]`

- **Поля:**
  - `CardSeries` (серия; источник для графы 54: G_54_12)
  - `CardNumber` (номер; источник для графы 54: G_54_100)
  - `OrganizationName` (кем выдан; источник для графы 54: G_54_13)
  - `CardDate` (дата выдачи; источник для графы 54: G_54_101)

  - `PersonInfo_PersonSurname` (фамилия; источник для графы 54: G_54_3)
  - `PersonInfo_PersonName` (имя; источник для графы 54: G_54_3NM)
  - `PersonInfo_PersonMiddleName` (отчество; источник для графы 54: G_54_3MD)
  - `PersonInfo_Sex` (пол; для сверок/контекста, в dt.xml обычно не переносится)
  - `PersonInfo_Birthday` (дата рождения; для сверок/контекста)
  - `PersonInfo_Birthplace` (место рождения; для сверок/контекста)

  - `ResidencePlace_PostalCode` (индекс; для сверок/контекста)
  - `ResidencePlace_CountryCode` (страна alpha-2; для сверок/контекста)
  - `ResidencePlace_CounryName` (страна, текст; возможна **опечатка CounryName**; для сверок/контекста)
  - `ResidencePlace_Region` (регион; для сверок/контекста)
  - `ResidencePlace_City` (город; для сверок/контекста)
  - `ResidencePlace_StreetHouse` (адрес одной строкой; для сверок/контекста)


### 6.13. Letter of Attorney / Доверенность (11004)

- **xml_target_root:** `AltaLetterOfAttorney`
- **uqi_prefix:** `formalized.letter_of_attorney_[n]`

- **Поля:**
  - `Subject` (текст доверенности; в primary.md хранить `link` на файл-источник; используется для формирования печатного 
    блока графы 54: G_54P при необходимости)
  - `EndDate` (действительна до; источник для графы 54: G_54_61)

  - `DocumentReference_PrDocumentName` (наименование доверенности; источник для графы 54: G_54_4)
  - `DocumentReference_PrDocumentNumber` (номер доверенности; источник для графы 54: G_54_5)
  - `DocumentReference_PrDocumentDate` (дата доверенности; источник для графы 54: G_54_60)

  - `Organization_OrganizationName` (выдавшая организация; для сверок/контекста)
  - `Organization_ShortName` (краткое наименование; для сверок/контекста)
  - `Organization_OGRN` (ОГРН; для сверок/контекста)
  - `Organization_INN` (ИНН; для сверок/контекста)
  - `Organization_KPP` (КПП; для сверок/контекста)
  - `Organization_Address_PostalCode` (индекс; для сверок/контекста)
  - `Organization_Address_CountryCode` (страна alpha-2; для сверок/контекста)
  - `Organization_Address_CounryName` (страна, текст; возможна **опечатка CounryName**; для сверок/контекста)
  - `Organization_Address_Region` (регион; для сверок/контекста)
  - `Organization_Address_City` (город; для сверок/контекста)
  - `Organization_Address_StreetHouse` (улица/дом одной строкой; для сверок/контекста)

  - `Organization_OrganizationPerson_PersonSurname` (подписант от организации; для сверок/контекста)
  - `Organization_OrganizationPerson_PersonName` (имя/инициалы; для сверок/контекста)
  - `Organization_OrganizationPerson_PersonMiddleName` (отчество; для сверок/контекста)
  - `Organization_OrganizationPerson_PersonPost` (должность; для сверок/контекста)

  - `EmpoweredPerson_PersonSurname` (уполномоченное лицо; источник для графы 54: G_54_3)
  - `EmpoweredPerson_PersonName` (имя; источник для графы 54: G_54_3NM)
  - `EmpoweredPerson_PersonMiddleName` (отчество; источник для графы 54: G_54_3MD)
  - `EmpoweredPerson_PersonPost` (роль/должность; источник для графы 54: G_54_7)

  - `EmpoweredPerson_Passport_IdentityCardCode` (код документа; источник для графы 54: G_54_8)
  - `EmpoweredPerson_Passport_IdentityCardName` (наименование документа; источник для графы 54: G_54_9)
  - `EmpoweredPerson_Passport_IdentityCardSeries` (серия; источник для графы 54: G_54_12)
  - `EmpoweredPerson_Passport_IdentityCardNumber` (номер; источник для графы 54: G_54_100)
  - `EmpoweredPerson_Passport_IdentityCardDate` (дата выдачи; источник для графы 54: G_54_101)
  - `EmpoweredPerson_Passport_OrganizationName` (кем выдан; источник для графы 54: G_54_13)

### 6.14. Transport Contract / Договор транспортной экспедиции (04033)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.transport_contract_[n]`

- **Поля:**
  - `DocumentCode` (04033 — код вида документа для графы 44: G44/G441)
  - `DocumentHead_DocumentName` (наименование договора; графа 44: G44/G444)
  - `DocumentHead_DocumentDate` (дата договора; графа 44: G44/G443)
  - `DocumentHead_DocumentNumber` (номер договора; графа 44: G44/G442)

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (текст договора; в primary.md хранить `link` на файл-источник; в dt.xml обычно не переносится)

### 6.15. EGRUL / Выписка из ЕГРЮЛ (04011)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.egrul_[n]`

- **Поля:**
  - `DocumentCode` (04011 — код вида документа для графы 44: G44/G441)
  - `DocumentHead_DocumentName` (наименование выписки; графа 44: G44/G444)
  - `DocumentHead_DocumentDate` (дата выписки; графа 44: G44/G443)
  - `DocumentHead_DocumentNumber` (номер выписки; графа 44: G44/G442)

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (текст выписки; в primary.md хранить `link` на файл-источник; используется как источник 
      мастер-данных для граф 8/9/14/54: G_8_*, G_9_*, G_14_*, G_54_*)

---

## 7. РАЗДЕЛ II: Неформализуемые документы

Документы извлекаются ради фактов для сборки ДТ.

### 7.1. Storage Report / Отчет СВХ (ДО-1 / ДО-2) (10061/10062)

- **uqi_prefix:** `non_formalized.svh_[n]`
- **Зачем:** факты для граф 6, 30 и для товарных блоков ДТ (места/вес/стоимость в разрезе кодов ТН ВЭД — если присутствует в ДО).
- **Ключевые поля:**
  - `number` (№ ДО-1/ДО-2; цель: для графы 30 в составе печатной строки: G_30P_1 (как часть derived))
  - `date` (дата ДО-1/ДО-2; цель: для графы 30 в составе печатной строки: G_30P_1 (как часть derived))
  - `warehouse_license_number` (номер лицензии/свидетельства СВХ; цель: графа 30: G_30_1)
  - `warehouse_license_date` (дата лицензии/свидетельства СВХ; цель: графа 30: G_30_DATE)
  - `actual_gross_weight` (фактический вес по весам; цель: сверка с общим брутто: графа 35: G_35_1 (контроль))
  - `actual_places` (фактическое количество мест; цель: графа 6: G_6_1 (приоритет #1))
  - `transport_reg_number` (номер ТС при въезде/по отчету СВХ; цель: сверка с графой 18: G_18)

- **Товары в разрезе строк ДО (если в документе есть таблица с разбиением):**
  - `goods_[n]`
    - `tnved` (код товара; цель: сопоставление с товарными блоками ДТ)
    - `places` (кол-во грузовых мест по строке; цель: `BLOCK/G_31/PLACE` и контроль графы 6)
    - `gross_weight_kg` (вес брутто по строке; цель: `BLOCK/G_35_1`)
    - `cost` (стоимость по строке; цель: `BLOCK/G_42_1` (контроль по валюте инвойса))
    - `currency_code` (буквенный код валюты, напр. `CNY`; цель: контроль)

**Правило:** если в ДО есть только общие итоги без разбивки по товарам — массив `goods_[n]` всё равно материализовать
как минимум одним элементом и ставить `pending` в `tnved/places/gross_weight_kg/cost/currency_code`, с вопросом в Раздел III.

### 7.2. Storage Report Additional Sheet

- **uqi_prefix:** `non_formalized.svh_additional_sheet_[n]`
- **Зачем:** адрес и код таможни СВХ для графы 30.
- **Ключевые поля:**
  - `number` (№ доп.листа/приложения; цель: для графы 30 в составе печатной строки: G_30P_1 (как часть derived))
  - `date` (дата доп.листа; цель: для графы 30 в составе печатной строки: G_30P_1 (как часть derived))
  - `actual_gross_weight` (фактический вес по весам; цель: сверка с графой 35: G_35_1 (контроль))
  - `actual_places` (фактическое количество мест; цель: уточнение/сверка для графы 6: G_6_1)
  - `transport_reg_number` (номер ТС при въезде; цель: сверка с графой 18: G_18)
  - `svh_address_region` (регион СВХ; цель: графа 30: G_30_SUB)
  - `svh_address_city` (город/нас.пункт СВХ; цель: графа 30: G_30_CIT)
  - `svh_address_street_house` (улица/дом СВХ как в отчете, без “улучшений”; цель: графа 30: G_30_STR)
  - `svh_customs_code` (код таможенного органа в зоне СВХ; цель: графа 30: G_30_12)

### 7.3. Master data / Мастер-данные (из `stable_source`)

- **uqi_prefix:** `non_formalized.master_data_[n]`
- **Зачем:** единый “слепок” реквизитов для stage 2 (графы 8/9/14/54). Stage 2 должен брать данные отсюда, а не парсить 
  произвольные поля `formalized.*`.
- **Источники (примеры):**
  - `alta\stable_source\Passport_*.xml` (паспорт представителя)
  - `alta\stable_source\LetterOfAttorney_*.xml` (доверенность)
  - `alta\stable_source\FreeDoc_ЮЭ*.xml` (ЕГРЮЛ)
  - (опционально) `alta\stable_source\FreeDoc_КООО_*.xml` (договор перевозки, если он “стабильный”)

- **Поля:**
  - `declarant_name` (наименование декларанта; цель: графа 14: G_14_NAM / печатный блок G_14/NAME)
  - `declarant_ogrn` (ОГРН; цель: графа 14: G_14_1)
  - `declarant_inn` (ИНН; цель: графа 14: участвует в G_14_4)
  - `declarant_kpp` (КПП; цель: графа 14: участвует в G_14_4)
  - `declarant_address_postal_code` (индекс; цель: графа 14: G_14_POS)
  - `declarant_address_country_code` (страна alpha-2; цель: графа 14: G_14_CC)
  - `declarant_address_country_name` (страна, текст; цель: графа 14: G_14_CN)
  - `declarant_address_region` (регион; цель: графа 14: G_14_SUB)
  - `declarant_address_city` (город; цель: графа 14: G_14_CIT)
  - `declarant_address_street` (улица; цель: графа 14: G_14_STR)
  - `declarant_address_building` (дом; цель: графа 14: G_14_BLD)
  - `declarant_address_room` (офис/помещение; цель: графа 14: G_14_ROM)
  - `declarant_phone` (телефон; цель: графа 14: G_14_PHONE)
  - `declarant_email` (e-mail; цель: графа 14: G_14_EMAIL)

  - `representative_last_name` (фамилия; цель: графа 54: G_54_3)
  - `representative_first_name` (имя; цель: графа 54: G_54_3NM)
  - `representative_middle_name` (отчество; цель: графа 54: G_54_3MD)
  - `representative_position` (должность/статус; цель: графа 54: G_54_7)
  - `representative_phone` (телефон; цель: графа 54: G_54_21)
  - `representative_email` (e-mail; цель: графа 54: G_54_EMAIL)

  - `representative_passport_code` (код документа, напр. RU01001; цель: графа 54: G_54_8)
  - `representative_passport_name` (наименование документа, напр. ПАСРФ; цель: графа 54: G_54_9)
  - `representative_passport_series` (серия; цель: графа 54: G_54_12)
  - `representative_passport_number` (номер; цель: графа 54: G_54_100)
  - `representative_passport_date` (дата выдачи; цель: графа 54: G_54_101)
  - `representative_passport_issuer` (кем выдан; цель: графа 54: G_54_13)

  - `representative_authority_doc_name` (наименование документа полномочий, обычно "ДОВЕРЕННОСТЬ"; цель: графа 54: G_54_4)
  - `representative_authority_doc_number` (№ доверенности; цель: графа 54: G_54_5)
  - `representative_authority_doc_date_from` (дата начала; цель: графа 54: G_54_60)
  - `representative_authority_doc_date_to` (дата окончания; цель: графа 54: G_54_61)

  - `note` (кратко: из каких файлов взято и где возможны расхождения; цель: пояснения, в dt.xml не переносится)

### 7.4. Certificate of Origin / Сертификат происхождения (06013)

- **uqi_prefix:** `non_formalized.certificate_of_origin_[n]`
- **Зачем:** документ для графы 44 (если прикладывается) и для обоснования страны происхождения.
- **Поля:**
  - `number` (номер сертификата; цель: графа 44: G44/G442)
  - `date` (дата сертификата; цель: графа 44: G44/G443)

### 7.5. Conformity Document / Декларация о соответствии EAC (01191)

- **uqi_prefix:** `non_formalized.conformity_document_[n]`
- **Зачем:** документ для графы 44 (если прикладывается) и подтверждение требований (если применимо к товару).
- **Поля:**
  - `number` (номер декларации/сертификата; цель: графа 44: G44/G442)
  - `date_start` (дата начала действия; цель: обычно только для сверок/контекста, в dt.xml напрямую не переносится)
  - `date_end` (дата окончания действия; цель: обычно только для сверок/контекста, в dt.xml напрямую не переносится)

### 7.6. Transit Declaration / Транзитная декларация (09013)

- **uqi_prefix:** `non_formalized.td_[n]`
- **Зачем:** источник данных для derived по графе 29 ДТ (таможенный орган) + реквизиты документа для графы 44 
  (если прикладывается).
- **Поля:**
  - `number` (номер ТД; цель: G44/G442 (если прикладывается) и/или derived)
  - `date` (дата ТД; цель: G44/G443 (если прикладывается) и/или derived)
  - `customs_post_code` (код таможенного органа; цель: графа 29: G_29_1)
  - `customs_post_name` (наименование таможенного органа; цель: графа 29: G_29_2)
  - `transport_reg_number` (ТС по ТД; цель: сверка с графой 18: G_18; если не читается — pending)

**Правило:** если `customs_post_code` / `customs_post_name` отсутствуют в md-версии ТД или не читаются надежно — 
`pending` + вопрос в Раздел III.

---

## Формат выходных файлов

Этап 1.0 формирует (в зависимости от режима) `primary.yaml` и (опционально) `primary.md`.

### A) `primary.yaml` (машинный файл, источник истины)

Требования:
- Валидный YAML.
- Не смешивать с Markdown: никаких markdown-таблиц `|...|`, никаких markdown-заголовков/списков как оформления.
- Только структуры YAML: map/array/scalar.

Структура YAML (рекомендуемая):
1) `meta`
2) `formalized`
3) `non_formalized`
4) `issues`

#### meta
Минимум:
- `case_name`
- `shipment_path`
- `generated_at`

#### formalized / non_formalized
Каждый документ — объект со следующими полями:
- `document` (тип документа, строка)
- `uqi_prefix`
- `xml_target_root` (если применимо)
- `path`
- `file_name`
- `status` (`confirmed` | `pending`)
- `fields` — map полей документа

Формат поля документа в YAML:
- `<FieldName>`:
  - `value`: <значение> *(если pending — отсутствует или `null`)*
  - `status`: <confirmed_document | confirmed_operator | pending>
  - `note`: <строка> *(опционально)*

Для массивов:
- `<ArrayName>`: массив элементов, где каждый элемент — объект полей по тому же формату.

Link вместо большого текста:
- если по схеме допускается link (например `ContractTerms_ContractText`), то `value` = `link:<путь_к_файлу>`.

#### issues
`issues` — нерешенные вопросы (пробелы/конфликты), предназначенные оператору.
Формат:
- `<UQI поля со статусом pending>`:
  - `question`: <текст вопроса>

Для общего вопроса:
- `[Общий]`:
  - `question`: <текст вопроса>

---

### B) `primary.md` (человеческий файл, табличный формат)

Файл `primary.md` — чистый Markdown и предназначен для просмотра оператором.

Правило: `primary.md` является представлением данных из `primary.yaml` и не должен содержать уникальных фактов,
которых нет в `primary.yaml`.

Разделы и порядок:
1) Метаданные
2) Раздел I: formalized
3) Раздел II: non_formalized
4) Раздел III: Нерешенные вопросы

Метаданные (в начале файла):
- `название кейса`: <название кейса>
- `путь к папке поставки`: <путь к папке поставки>
- `тип поставки`: <например: 1 ДТ / 1 товар>
- `агрегация ДТ`: определяется правилами stage 2.0

### Формат описания Документа:

- `document`: <тип документа>
  - `uqi_prefix`: <префикс, например formalized.invoice_1>
  - `xml_target_root`: <корневой тег XML, если применим>
  - `path`: <путь к файлу>
  - `file_name`: <имя файла>
  - `status`: <confirmed / pending> (pending - если документ не найден или хотя бы одно поле имеет статус pending)
  - `note`: <(опционально) пояснение>

### Таблица полей
Далее идет таблица:
- AI обязан материализовать все поля, указанные в шаблоне документа;
- Для пустых значений полей ячейка таблицы остается пустой;
- Если для поля не удалось установить значение, status=pending;

| field       | value             | description       | status                                          | note             |
|-------------|-------------------|-------------------|-------------------------------------------------|------------------|
| <FieldName> | <value или пусто> | <назначение поля> | <confirmed_document/confirmed_operator/pending> | <note или пусто> |

### Для вложенных структур/массивов
- Массивы оформлять подзаголовками:
  #### <ArrayName>_<n>
  затем таблица полей этого элемента тем же форматом.

### Link вместо большого текста
Если по схеме допускается link (например ContractTerms_ContractText), то:
- value = `link:<путь_к_файлу>`
- status = confirmed_document (если ссылка взята из первички)

---

## 8. РАЗДЕЛ III: Нерешенные вопросы

Здесь размещаются все вопросы к оператору по конкретным полям или общие проблемы.

### Формат записи:

**Для поля:**
- `<UQI поля со статусом pending>`
  - `question`: <текст вопроса AI>

**Для общего вопроса:**
- `[Общий]`
  - `question`: <текст вопроса AI>

---

## РАЗДЕЛ IV: Порядок работы (задание).

### 1. Прочитать / убедиться, что структура следующих каталогов, включая размеры файлов, известна. 
  - `alta\source\...` (первичные документы всех кейсов) 
  - `alta\source\<кейс>\...`  (первичные документы текущей поставки)
  - `alta\stable_source\` (документы, не меняющиеся между поставками)
  - `alta\operator\` (Опционально. Данные, предоставленные оператором. Каталог может отсутствовать / быть пустым, если
     диалога с оператором не было.)
   
### 2. Прочитать все исходные файлы, шаблоны для которых имеются в данной схеме.
  - Если файл кажется посторонним, прочесть хотя бы его начало, чтобы убедиться, что он является таковым.
  - **ПРИОРИТЕТ ФОРМАТОВ. ПЕРВЫМИ ПРОБУЙ ЛЕГКООБРАБАТЫВАЕМЫЕ ФОРМАТЫ.** Если документ представлен в нескольких форматах,
    выбирай самый удобный. Приоритет:
  - Если документ в текстовом формате (txt, md, xml), это самый лучший вариант.
  - Если сайт принимает файлы Docx, Xlsx, то предпочитай эти форматы.
  - Если есть образ или набор образов документа (часто документ не помещается в один скриншот). Это PNG и другие форматы.
  - PDF, если нет других вариантов. Его обработка тяжелее и менее надежна.

### 3. Сгенерировать `primary.yaml` (и при полном режиме также `primary.md`).

- После генерации AI **ОБЯЗАН ПРОВЕРИТЬ `primary.yaml`**. Чек-лист:
- ✅ прочитаны все исходные документы во всех каталогах исходных документов
- ✅ все документы, для которых имеются шаблоны, присутствуют
- ✅ все поля шаблонов присутствуют, даже если status=pending
- ✅ Никакие существенные данные из первички не потеряны. Если есть данные, которые не могли быть помещены в
  `primary.yaml`, они должны быть зафиксированы в `primary_review.md` с указанием причины, например, 
  "нет подходящего шаблона".
- (Полный режим) ✅ `primary.md` соответствует `primary.yaml` и не содержит уникальных фактов.

### 4. Сгенерировать `primary_review.md`

Генерируется на основе `primary.yaml`. Если имеют место нехватка / конфликты данных, в чате предложить оператору:
- ответить на вопросы
- пересоздать `operator_provided_data.md` с включением полученных данных

Сгенерировать и проверить `primary.yaml` (и при полном режиме также `primary.md`), заново выполнив пункт 3.

### Примечание:

- Для AI **РАЗРЕШЕНА** запись в `alta\stage_1.0_result\<кейс>\...`, файлы `primary.yaml`, `primary.md`, 
  `primary_review.md` пишутся в этот каталог.

- Для AI **РАЗРЕШЕНА** запись в `alta\source\<кейс>\operator\...`. Если был диалог с оператором, в этот 
  каталог помещается файл `operator_provided_data.md`, содержащий информацию, которая может быть 
  использована совместно с первичкой на следующих прогонах.

- Для файлов документов использовать следующие способы доступа:
  - текстовые файлы: команды Хобота read_file, write_file;
  - xlsx, docx, png, pdf: перетаскивание в поле ввода;
  - xml, используемые для импорта / экспорта: read_file, write_file с параметром кодировки windows-1251; 

## ПРИЛОЖЕНИЕ. Вырезки из справочников

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

---

Полные справочники — в `alta\prompt\codebook.md`.

  