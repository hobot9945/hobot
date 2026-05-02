# Инструкция по этапу 1.0: Сбор и нормализация фактов

## 1. Разделы файла `primary_schema.md`

0. **Раздел 0 Вводная часть**

1. **Раздел I `formalized` (Шаблоны формализуемых документов):** Документы, на базе которых на этапе 1.1 будут строиться 
   XML-файлы для Альты. Идентификаторы полей здесь и далее совпадают с целевыми XML тегами.

2. **Раздел II `non_formalized` (Шаблоны неформализуемых документов):** Документы, которые не требуют генерации в XML.
   Содержат данные, необходимые для ДТ.

3. **Раздел III: Формат `primary.md`**

4. **Раздел IV: Порядок работы:** - правила выполнения этапа

5. **ПРИЛОЖЕНИЕ. Вырезки из справочников**

## 0. РАЗДЕЛ 0 Вводная часть

### 1. Цель этапа

Полное извлечение фактов из папок `alta\source\<кейс>\...` и `alta\stable_source\*.xml` (cp1251), а также, получение 
от оператора, недостающих данных и разрешение конфликтов.

Выход этапа - база фактов поставки `primary.md` — формализованный файл, источник истины для этапа 1.1 и этапа 2.0.

Важно: 
**`primary.md` — это не summary, а формальная база данных. Неполная структура = ошибка этапа. !!!СОКРАЩЕНИЯ НЕДОПУСТИМЫ!!!**

### 2 Источники данных

#### 2.1. Проверка наличия исходных документов
- Если существует папка `alta\source\<кейс>\...\md\` и в ней есть файл  `doc_conversion_review.md` (содержит инвентарь),
- если инвентарь совпадает с реальным содержимым папки,
- то считать, что все доступные документы конвертированы в md формат и использовать их.
- если эти условия не выполнены частично или полностью:
  - получить разрешение оператора на прямую работу с первичкой,
  - взять все отсутствующие в `md` первичные документы,
  - выполнить генерацию, поместить факт прямой работы с первичкой в `primary_review/issues`. 

#### 2.2. Обработка md-папки (обязательный шаг)
Имя файла НЕ считается надежным источником типа документа. AI обязан прочитать ВСЕ файлы в `...\md\`:
- целиком или минимум первые 30–60 строк каждого файла, если файл объемный,
- классифицировать документ по содержанию и сопоставить с шаблоном.
- Если md-файл не сопоставляется ни с одним шаблоном:
  - добавить запись в primary_review.md, раздел "Существенные данные, которые не попали в primary.md" или 
    отдельный раздел "Unclassified md files",
  - указать: имя файла + первые строки/признаки + почему не сматчен.  
- Если для ожидаемого по смыслу документа в md-папке не найдено соответствующего md-файла: 
  - получить разрешение оператора на прямую работу с первичкой,
  - искать нужные документы в первичке,
  - выполнить генерацию, поместить факт прямой работы с первичкой в `primary_review/issues`.
 
### 3. Общие правила

1. **Нормализация:** primary.md является строго нормализованной структурной базой данных. Запрещено сокращать или 
   агрегировать структуру. Все поля, предусмотренные шаблоном, обязаны присутствовать в primary.md, даже если их 
   статус — pending.
 
2. **Блокеры:** для генерации формализованных xml документов на этапе 1.1 блокерами служат **формализуемые** поля со 
   статусом `pending`. Второй этап **не** блокируется вообще.

3. **Документная изоляция:**
  - AI **сверяет** данные между документами, чтобы находить **конфликты**. Расхождения между документами фиксировать как 
    конфликт/вопрос.
  - AI **НЕ переносит** значения полей из одного документа в другой: `value` заполняется только из источника этого документа.
  - Исключение: явные значения от оператора (`operator_provided_data.md`) → `status=CO`. Это единственное разрешенное
    нарушение принципа изоляции.

4. **Никаких догадок:** Если поле отсутствует или неоднозначно и в `note` не описаны другие действия, то `value` пустое, 
   `status` = `pending` и фиксация в Разделе III. 

5. В рабочем режиме AI не имеет права использовать как источник фактов новой поставки:
  - файлы из каталогов `alta\reference\...` и любых `...\выгрузки\...`;
  - эталонные ДТ, эталонные xml/скриншоты
  - результаты прошлых прогонов (`primary.md`, `primary_review.md`) можно дорабатывать, но следует полностью игнорировать 
    при первой генерации.

6. Если master data (`alta\stable_source\`) конфликтуют с первичкой — фиксировать конфликт в Разделе III.

7. **Предпочтение русских вариантов текста:** 
  - если документ (например, контракт) продублирован на русском и английском языках, используй русскую версию,
  - визуально-эквивалентные буквы в номерах транспортных средств переводим в кириллицу (во всех документах, содержащих 
    номера ТС),  
  - если в документе одновременно встречаются и русские и английские названия стран, выбираем русский вариант, 

8. **Если страна в первичке указана текстом** (например China/Китай/Россия), а поле требует alpha-2 код:
  - мапить в alpha-2 по `cb:country`,
  - `status: CD`,
  - `note`: `нормализация по cb:country`.

8. **Приоритет каталога `md` перед другими первичными документами:** сначала используй его, если имеется.

9. **Решения оператора:** если в `operator_provided_data.md` или в чате оператор дал значение для поля или 
   по группе полей, AI **ОБЯЗАН**:
   1) записать это значение в `primary.md` в соответствующее поле (поля);
   2) установить `status: CO` (confirmed_operator);
   3) обновить/удалить соответствующий вопрос в `primary.md/issues`.
   Решения оператора имеют приоритет над другими правилами, например, могут нарушать принцип документной изоляции.

10. **Правило: Invoice consignor = seller.** Если в инвойсе нет отдельного consignor (shipper) — заполняй 
    `formalized.invoice_[n].Consignor_*` значениями `formalized.invoice_[n].Seler_*` (внутри этого же документа).  
    `status: CO`, `note: нормализация: consignor=seller`.

## 4. UQI документов и пути к полям

Идентификаторы документов, например, `formalized.invoice_1`, `non_formalized.svh_1` (`[раздел].[тип_документа]_[n]`), 
используются как `uqi_prefix` для ссылок на поля: `<uqi_prefix>.<имя_поля>`, `<uqi_prefix>.<имя_массива>_[n].<имя_поля>`. 

## 5. Правила составления шаблонов документов и полей:

- **Формат поля документа в схеме:** занимает одну строку. Первым идет идентификатор, далее, в скобках, пояснения.

- **Идентификаторы полей:** Для формализуемых документов идентификатор совпадает с xml тегом. Для неформализуемых -
  snake_case.

- **Объемные тексты:** Если текст слишком большой (техописание, контракт), в `primary.md` хранится 
  `value: link:<имя_файла>`. Полный текст будет подставлен на этапе 1.1.

- **Реквизиты:** Максимально разбивать адреса на компоненты. Если надежно разбить нельзя — сохранять исходную строку 
  в соответствующее поле и назначать статус `pending`.

---

## 6. РАЗДЕЛ I: Формализуемые документы (Шаблоны)

### Общее правило

Каждый формализуемый документ в `primary.md` может быть разделен на два смысловых слоя:

1. **Формализуемые поля** — данные, которые участвуют в последующем построении XML-документов Альты.  
   Для таких полей:
  - имя поля совпадает с тегом XML;
  - именно эти поля считаются источником для дальнейшей xml-формализации.

2. **Дополнительные неформализуемые поля** — сведения, присутствующие в документе, не участвующие в построении XML,
   но необходимые для составления ДТ.

Рекомендуемое оформление таких данных:
- `value`: <значение> | `link`: <ссылка на первичный документ>
- `status`: < CD | CO | pending> (Сокращения: CD - confirmed_document, CO - confirmed_operator)
- `note`: дополнительное поле, не используется для генерации xml

### 6.1. Contract / Контракт (03011)

- **xml_target_root:** `AltaE2CONT`
- **uqi_prefix:** `formalized.contract_[n]`

- **Поля:**
  - 01: `DocumentCode` (03011 — код вида документа для графы 44: G44/G441)
  - 02: `ContractRegistration_PrDocumentNumber` (№ контракта; графа 44: G44/G442)
  - 03: `ContractRegistration_PrDocumentDate` (дата контракта; графа 44: G44/G443)

  - 04: `ContractTerms_Amount` (общая сумма контракта; для контроля/сверки; в dt.xml обычно напрямую не печатается)
  - 05: `ContractTerms_CurrencyCode` (цифровой код валюты ISO 4217 numeric; для контроля/сверки; напр. CNY=156)
  - 06: `ContractTerms_LastDate` (срок действия/исполнения; для контроля/сверки)
  - 07: `ContractTerms_OtherTerms` (условия поставки / Incoterms, напр. `EXW ...`; источник для графы 20: G_20_1)
  - 08: `ContractTerms_ContractText` (текст контракта; в primary.md хранить `link` на файл-источник)
  - 09: `ContractTerms_DealSign` (системный признак Альты; для импорта, обычно `1`)

  - 10: `ForeignPerson_OrganizationName` (продавец/сторона контракта; обычно совпадает с отправителем; может использоваться 
    для сверок)
  - 11: `ForeignPerson_Address_CountryCode` (страна продавца alpha-2 из `cb:country`; для сверок)
  - 12: `ForeignPerson_Address_CounryName` (страна продавца, текст; **опечатка тега CounryName**; для сверок)
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
 
- _audit: 25

**Примечание:**
`ContractTerms_ContractText` в `primary.md` не копировать полный текст контракта, сохранять только `link`
на файл-источник. Полный текст подставлять только при генерации XML. Нужно для сохранения компактности `primary.md`.

### 6.2. Supplementary Contract / Дополнительное соглашение к контракту (03012)

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
  - 07: `ContractDescription_DealSign` (системный признак Альты; для импорта, обычно `1`)
  - 08: `ContractDescription_StockCategorySign` (системный признак Альты; для импорта, обычно `0` если не используется)
  - 09: `ContractDescription_BuyerLimitationSign` (системный признак Альты; для импорта, обычно `0` если не используется)
  - 10: `ContractDescription_InsuranceSign` (системный признак Альты; для импорта, обычно `0` если не используется)

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

  - `ContractSignedPerson` (подписант доп. соглашения; для сверок/аудита)
    - 29: `PersonSurname` (фамилия подписанта)
    - 30: `PersonName` (имя подписанта)
    - 31: `PersonMiddleName` (отчество подписанта)

- _audit: 31

### 6.3. Invoice (04021)

- **xml_target_root:** `AltaE2I`
- **uqi_prefix:** `formalized.invoice_[n]`

- **Поля шапки / реквизиты документа:**
  - 01: `CurrencyRate` (курс валюты; источник для графы 23: G_23_1, G_23_2)
  - 02: `CurrencyCode` (валюта инвойса ISO 4217 alpha-3, напр. `CNY`, `USD`; источник для графы 22: G_22_3)
  - 03: `DocumentCode` (04021 — код вида документа для графы 44: G44/G441)
  - 04: `PlacesQuantity` (кол-во грузовых мест по инвойсу; приоритет #3 для графы 6: G_6_1)
  - 05: `PlacesDescription` (описание мест, напр. "Поддон"; для сверок/контекста, обычно не в dt.xml напрямую)
  - 06: `GrossWeightQuantity` (общий вес брутто по инвойсу; для сверок с CMR/PL/СВХ)
  - 07: `NetWeightQuantity` (общий вес нетто по инвойсу; для сверок)
  - 08: `GCost` (системное поле Альты; дубль `TotalCost`; для импорта/совместимости)
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

- **Товарные позиции** (каждый элемент соответствует узлу `<InvoiceGoods>...</InvoiceGoods>`; источник для блока товаров 
  dt.xml: `BLOCK/TOVG/TXT`):
  - 52.n: `InvoiceGoods_[n]`
    - 01: `GoodsCode` (код ТН ВЭД; источник для графы 33: G_33_1)
    - 02: `GoodsDescription` (описание товара как в инвойсе; источник для графы 31: G_31/NAME и для строк дополнения TXT/TEXT)
    - 03: `GoodsQuantity` (кол-во по строке инвойса в “основной” единице строки; для сверок; не использовать
      как TOVG/KOLVO, если в инвойсе есть отдельная колонка доп.кол-ва)
    - 04: `goods_supplementary_quantity` (количество в доп.ед.изм для ДТ; например, `Quantity in M2`; неформализуемое поле)
    - 05: `goods_supplementary_uom_name` (наименование доп.ед.изм из `cb:unit`; неформализуемое поле)    
    - 06: `MeasureUnitQualifierName` (единица измерения доп.количества для ДТ, наименование из `cb:unit`; цель: TOVG/NAME_EDI)
    - 07: `GrossWeightQuantity` (брутто по строке; источник для веса: G_35_1 (агрегация) и TOVG/G31_35)
    - 08: `NetWeightQuantity` (нетто по строке; источник для веса: G_38_1 (агрегация) и TOVG/G31_38)
    - 09: `Price` (цена за единицу; для сверок/контроля; обычно не переносится в dt.xml напрямую)
    - 10: `TotalCost` (стоимость по строке; источник для графы 42 (агрегация) и TOVG/INVOICCOST)
    - 11: `OriginCountryCode` (цифровой код страны происхождения; источник для графы 34 после нормализации в alpha-2: G_34_1)

    - 12: `AdditionalGoodsDescription_Manufacturer` (производитель; источник для графы 31: G_31/FIRMA и TOVG/G31_11)
    - 13: `AdditionalGoodsDescription_TradeMark` (товарная марка/ТМ; источник для графы 31: G_31/TM и TOVG/G31_12; если 
      отсутствует в первичке — "ОТСУТСТВУЕТ")
    - 14: `AdditionalGoodsDescription_GoodsMark` (товарный знак/маркировка; источник для графы 31 и TOVG/G31_14; если 
      отсутствует — "ОТСУТСТВУЕТ")
    - 15: `AdditionalGoodsDescription_GoodsModel` (модель/модификация; источник для графы 31 и TOVG/G31_15_MOD)

  - _item_audit: 15

- _audit: 52

### 6.4. Packing List / Упаковочный лист (04131)

- **xml_target_root:** `AltaE2PACK`
- **uqi_prefix:** `formalized.packing_list_[n]`

- **Поля:**
  - 01: `GrossWeightQuantity` (общий вес брутто по упаковочному; используется для сверок; может участвовать в derived 
    по графам 35/38 при необходимости)
  - 02: `NetWeightQuantity` (общий вес нетто по упаковочному; используется для сверок)

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

  - 24: `DeliveryTerms_Contract_PrDocumentName` (наименование контракта для печати/графы 44: G44/G444)
  - 25: `DeliveryTerms_Contract_PrDocumentNumber` (№ контракта; графа 44: G44/G442)
  - 26: `DeliveryTerms_Contract_PrDocumentDate` (дата контракта; графа 44: G44/G443)

  - 27: `DeliveryTerms_Invoice_PrDocumentName` (наименование инвойса для печати/графы 44: G44/G444)
  - 28: `DeliveryTerms_Invoice_PrDocumentNumber` (№ инвойса; графа 44: G44/G442)
  - 29: `DeliveryTerms_Invoice_PrDocumentDate` (дата инвойса; графа 44: G44/G443)

  - 30: `DeliveryTerms_Registration_PrDocumentName` (наименование упаковочного; графа 44: G44/G444)
  - 31: `DeliveryTerms_Registration_PrDocumentNumber` (№ упаковочного; графа 44: G44/G442)
  - 32: `DeliveryTerms_Registration_PrDocumentDate` (дата упаковочного; графа 44: G44/G443)

- Неформализуемые поля (для ДТ, не для XML упаковочного листа, не влияют на XML генерацию)
  - 33: `registration_doc_name` (наименование документа для графы 44: G44/G444; напр. `УПАКОВОЧНЫЙ ЛИСТ`)
  - 34: `registration_doc_number` (номер документа для графы 44: G44/G442; если в первичке “Б/Н” — так и писать)
  - 35: `registration_doc_date` (дата документа для графы 44: G44/G443)

- **Товарные/грузовые строки** (каждый элемент соответствует узлу `<Goods>...</Goods>`; это строки “по местам/грузовым 
  единицам”, не по товарам ДТ):
  - 36: `Goods_[n]`
    - 01: `GoodsDescription` (описание строки как в документе; для сверок/контекста; может быть агрегированным текстом)
    - 02: `GoodsQuantity` (количество мест/грузовых единиц в строке; источник приоритета #2/#3 для графы 6: G_6_1 через derived)
    - 03: `GrossWeightQuantity` (брутто по строке; для сверок)
    - 04: `NetWeightQuantity` (нетто по строке; для сверок)
    -  `PackingInfo`
      - 05: `PakingQuantity` (кол-во упаковок/мест в упаковке; **опечатка PakingQuantity**; в эталонах может быть 0/пусто)
    - _item_audit: 5

- **Транспорт** (каждый элемент соответствует узлу `<TransportMeans>...</TransportMeans>`; источник для графы 18: G_18 
  и связанных derived-полей):
  - 37: `TransportMeans_[n]`
    - 01: `Number` (регистрационный номер; источник для графы 18: G_18)
    - 02: `ModeCode` (код вида транспорта; источник для граф 25/26: G_25_1, G_26_1; для автосостава обычно 31)
    - 03: `NationalityCode` (код “национальности” ТС в структуре Альты; для сверок/совместимости, в эталонах может быть `000`)
    - 04: `MoverIndicator` (`true` для тягача, `false` для прицепа; нужно для порядка/логики формирования G_18)

  - _item_audit: 4

- _audit: 37

**Правило:** если известны номер тягача и номер прицепа, сохранять их как ДВА элемента:
  - transport_1 (MoverIndicator=true) — тягач
  - transport_2 (MoverIndicator=false) — прицеп

### 6.5. CMR / Международная товарно-транспортная накладная (02015)

CMR является транспортным документом и может не содержать детализацию товаров (в отличие от Invoice).

#### Исключение для CMRGoods (разрешены вывод значения поля и нарушение документной изоляции)
Если в CMR отсутствует детализация груза (поля/разделы `9 Product name` пустые и нет списка строк), то:
- материализовать ровно 1 элемент `CMRGoods_1`;
- `CMRGoods_1.GoodsNumeric` = `1`, `status: CD`, `note: авто-нумерация единственной строки`;
- `CMRGoods_1.GoodsDescription`:
  - если в `non_formalized.svh_1` (ДО-1) есть фраза вида `Товар загружен согласно спецификации к Invoice № ...`, 
    перенести её сюда;
  - `status: CO`, `note: исключение CMRGoodsDescription — источник non_formalized.svh_1`.

- **xml_target_root:** `AltaE3CMR`
- **uqi_prefix:** `formalized.cmr_[n]`

- **Поля:**
  - 01: `LanguageCode` (язык документа; для импорта/совместимости; обычно `RU`)
  - 02: `CMR_Choice` (системный выбор/вариант Альты; для импорта, обычно `1`)

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

- **Товарные/грузовые строки** (каждый элемент соответствует узлу `<CMRGoods>...</CMRGoods>`; это строки 
  “по местам/упаковкам”, не по товарам ДТ):
  - 42: `CMRGoods_[n]`
    - 01: `GoodsNumeric` (порядковый номер строки)
    - 02: `GoodsDescription` (описание груза/товара как в CMR; для сверок и при необходимости для дополнения к графе 31)
    - `GoodsPackingInfo`
      - 03: `PakingQuantity` (кол-во упаковок/мест; **опечатка PakingQuantity**; для сверок/контекста)
  - _item_audit: 3

- _audit: 42

### 6.6. Payment Order / Платежное поручение (04023)

- **xml_target_root:** `AltaPaymentOrder`
- **uqi_prefix:** `formalized.payment_order_[n]`

- **Поля:**
  - 01: `DocumentCode` (04023 — код вида документа для графы 44: G44/G441)

  - 02: `PaymentModeCode` (системный код способа платежа в структуре Альты; для импорта/совместимости)
  - 03: `PaymentAmount` (сумма платежа; для сверок с оплатой по поставке; обычно не переносится в dt.xml напрямую)
  - 04: `TransactionKind` (вид операции/код; системное поле Альты; для импорта/совместимости)
  - 05: `Priority` (очередность; системное поле; в эталонах может быть `"."`; для импорта/совместимости)
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

  - `PayerSign` (подписант/плательщик; для сверок/аудита)
    - 16: `PersonSurname` (фамилия)
    - 17: `PersonName` (имя)

- _audit: expected=17

### 6.7. Service Invoice / Счет за перевозку (04031)

- **xml_target_root:** `AltaServiceInvoice`
- **uqi_prefix:** `formalized.service_invoice_[n]`

- **Поля:**
  - 01: `DocumentSign` (системный признак документа Альты; для импорта/совместимости, обычно `1`)
  - 02: `TotalServiceCost` (итого по услугам; для расчётов/сверок, при необходимости может участвовать в графах стоимости)
  - 03: `Currency` (валюта итого ISO 4217 alpha-3; для расчётов/сверок)

  - 04: `ServiceProvider_Name` (исполнитель услуг/перевозчик; для сверок/контекста)
  -  `ServiceProvider_PaymentRequisitions`
    - 05: `BankName` (банк исполнителя; для сверок/контекста)

  - 06: `ContractDetails_PrDocumentNumber` (№ договора на услуги/перевозку; графа 44: G44/G442 (если прикладывается как документ))
  - 07: `ContractDetails_PrDocumentDate` (дата договора на услуги/перевозку; графа 44: G44/G443)

  - `PaymentDocument` (связанный документ/заказ в структуре Альты; используется для связи документов/сверок)
    - 08: `PrDocumentNumber` (номер; графа 44: G44/G442)
    - 09: `PrDocumentDate` (дата; графа 44: G44/G443)

  - 10: `Registration_PrDocumentName` (наименование счета; графа 44: G44/G444)
  - 11: `Registration_PrDocumentNumber` (номер счета; графа 44: G44/G442)
  - 12: `Registration_PrDocumentDate` (дата счета; графа 44: G44/G443)

  - 13: `Consignor_OrganizationName` (грузоотправитель; для сверок/контекста)
  - `Consignor_SubjectAddressDetails`
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
  - `Consignee_SubjectAddressDetails`
    - 24: `PostalCode` (индекс; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS)
    - 25: `CountryCode` (страна alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC)
    - 26: `CounryName` (страна, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, G_14_CN)
    - 27: `Region` (регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB)
    - 28: `Town` (город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT)
    - 29: `StreetHouse` (улица; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR)
    - 30: `House` (дом; графы 8/9/14: G_8_BLD, G_9_BLD, G_14_BLD)
    - 31: `Room` (офис/кв; графы 8/9/14: G_8_ROM, G_9_ROM, G_14_ROM)

- **Подписи** (системный блок структуры Альты; для импорта/совместимости):
  - 32: `Signature_Choice` (вариант подписи; системное поле)
  - 33: `SignatureDirectorChiefAccountant_Director_PersonSurname` (фамилия руководителя)
  - 34: `SignatureDirectorChiefAccountant_Director_PersonName` (инициалы/имя руководителя)
  - 35: `SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname` (фамилия бухгалтера)
  - 36: `SignatureDirectorChiefAccountant_ChiefAccountant_PersonName` (инициалы/имя бухгалтера)

- **Услуги** (каждый элемент соответствует узлу `<ServiceDescription>...</ServiceDescription>`):
  - 37: `ServiceDescription_[n]`
    - 01: `GoodsDescription` (многострочное описание услуги — сохранять как есть; **может отсутствовать** в отдельных строках)
    - 02: `CurrencyCode` (валюта строки ISO alpha-3; для сверок/расчётов)
    - 03: `ServiceName` (наименование/маршрут; для сверок/контекста)
    - 04: `TaxRate` (ставка налога; для сверок/расчётов)
    - 05: `TaxSum` (сумма налога; для сверок/расчётов)
    - 06: `ServiceCost_Amount` (стоимость строки; для сверок/расчётов)
    - 07: `ServiceCost_Currency` (валюта стоимости строки; для сверок/расчётов)

- _audit: 37

### 6.8. Insurance Document / Счет за страховку (04111)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.insurance_document_[n]`

- **Поля:**
  - 01: `DocumentCode` (04111 — код вида документа для графы 44: G44/G441)
  - 02: `DocumentHead_DocumentName` (наименование документа; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата документа; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер документа; графа 44: G44/G442)
  - 05: `TextPara_[n]` (основной текст/условия; в primary.md хранить `link` на файл-источник; `DocumentBody_TextSection`)

- _audit: 5

### 6.9. TechDescription / Техническое описание (05999)

Наличие нескольких технических описаний для разных товаров допустимо. Но, если несколько технических описаний 
относятся к одному и тому же товару (совпадает наименование, модель или явная ссылка на товар) или не может быть
соотнесено с товаром, AI не делает предположений об их релевантности и обязан вынести вопрос в Раздел III.

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.tech_description_[n]`

- **Поля:**
  - 01: `DocumentCode` (05999 — код вида документа для графы 44: G44/G441)
  - 02: `DocumentHead_DocumentName` (наименование техописания; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата техописания; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер техописания; графа 44: G44/G442)
  - 05: `TextPara_[n]` (технический текст без пересказа; в primary.md хранить `link` на файл-источник; 
    `DocumentBody_TextSection`; используется для дополнения/обоснований к графе 31: G_31 
    (через TXT/TEXT при необходимости))

- _audit: 5

### 6.10. FreeDoc / Прочие текстовые документы (09999)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.free_doc_[n]`

- **Поля:**
  - 01: `DocumentCode` (09999 — код вида документа для графы 44: G44/G441)
  - 02: `DocumentHead_DocumentName` (наименование документа; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата документа; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер документа; графа 44: G44/G442)
  - 05: `TextPara_[n]` (основной текст; в primary.md хранить `link` на файл-источник; `DocumentBody_TextSection`)

- _audit: 5

### 6.11. FreeBinaryDoc / Бинарное приложение (например PDF-вложение)

- **xml_target_root:** `AltaFreeBinaryDoc`
- **uqi_prefix:** `formalized.free_binary_doc_[n]`

- **Поля:**
  - 01: `DocumentCode` (код вида документа; для графы 44: G44/G441; если неизвестен — уточнить по `cb:doc`, иначе `pending`)
  - 02: `DocumentInfo_PrDocumentName` (наименование вложения; графа 44: G44/G444)
  - 03: `DocumentInfo_PrDocumentNumber` (номер документа-основания/вложения; графа 44: G44/G442)
  - 04: `DocumentInfo_PrDocumentDate` (дата документа-основания/вложения; графа 44: G44/G443)

  - 05: `DocumentBody_FileName` (имя файла вложения; для импорта/контроля)
  - 06: `DocumentBody_FileData` (base64 содержимого; в primary.md  хранить `link` на исходный файл)
  - 07: `Thumbnail` (миниатюра/base64; если не требуется для импорта — не материализовывать)

- _audit: 7

### 6.12. Personal Passport / Паспорт (11001)

- **xml_target_root:** `AltaPassport`
- **uqi_prefix:** `formalized.passport_[n]`

- **Поля для графы 44:**
  - 01: `DocumentCode` (value = `11001`) (код вида документа; для графы 44)
  - 02: `DocumentHead_DocumentName` (value = `ПАСПОРТ`) (наименование документа; для графы 44)
  - 03: `DocumentHead_DocumentDate` (value = `CardDate`) (дата документа; для графы 44)
  - 04: `DocumentHead_DocumentNumber` (value = `CardSeries + " " + CardNumber`, например `"63 09 449948"`) 
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

- _audit: 20

### 6.13. Letter of Attorney / Доверенность (11004)

- **xml_target_root:** `AltaLetterOfAttorney`
- **uqi_prefix:** `formalized.letter_of_attorney_[n]`

- **Поля для графы 44:**
  - 01: `DocumentCode` (value = `11004`) (код вида документа; для графы 44)
  - 02: `DocumentHead_DocumentName` (value = `DocumentReference_PrDocumentName`) (наименование документа; для графы 44)
  - 03: `DocumentHead_DocumentDate` (value = `DocumentReference_PrDocumentDate`) (дата документа; для графы 44)
  - 04: `DocumentHead_DocumentNumber` (value = `DocumentReference_PrDocumentNumber`) (номер документа; для графы 44)

- **Поля:**
  - 05: `Subject` (текст доверенности; в primary.md хранить `link` на файл-источник; используется для формирования печатного 
    блока графы 54: G_54P при необходимости)
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

- _audit: 34

### 6.14. Transport Contract / Договор транспортной экспедиции (04033)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.transport_contract_[n]`

- **Поля для графы 44:**
  - 01: `DocumentCode` (value = `04033` — код вида документа для графы 44: G44/G441)
  - 02: `DocumentHead_DocumentName` (наименование договора; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата договора; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер договора; графа 44: G44/G442)
  - 05: `TextPara` (текст договора; в primary.md хранить `link` на файл-источник; `DocumentBody_TextSection`)

- _audit: 5

### 6.15. EGRUL / Выписка из ЕГРЮЛ (04011)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.egrul_[n]`

- **Поля для графы 44:**
  - 01: `DocumentCode` (04011 — код вида документа для графы 44: G44/G441)
  - 02: `DocumentHead_DocumentName` (наименование выписки; графа 44: G44/G444)
  - 03: `DocumentHead_DocumentDate` (дата выписки; графа 44: G44/G443)
  - 04: `DocumentHead_DocumentNumber` (номер выписки; графа 44: G44/G442)
  - 05: `TextPara` (текст выписки; в primary.md хранить `link` на файл-источник; используется как источник 
    мастер-данных для граф 8/9/14/54: G_8_*, G_9_*, G_14_*, G_54_*; `DocumentBody_TextSection`)

- _audit: 5

---

## 7. РАЗДЕЛ II: Неформализуемые документы

Документы извлекаются ради фактов для сборки ДТ.

### 7.1. Storage Report / Отчет СВХ (ДО-1 / ДО-2) (10061/10062)

- **uqi_prefix:** `non_formalized.svh_[n]`
- **Зачем:** факты для граф 6, 30 и для товарных блоков ДТ (места/вес/стоимость в разрезе кодов ТН ВЭД — если присутствует в ДО).
- **Ключевые поля:**
  - 01: `number` (№ ДО-1/ДО-2; цель: для графы 30 в составе печатной строки: G_30P_1 (как часть derived))
  - 02: `date` (дата ДО-1/ДО-2; цель: для графы 30 в составе печатной строки: G_30P_1 (как часть derived))
  - 03: `warehouse_license_number` (номер лицензии/свидетельства СВХ; цель: графа 30: G_30_1)
  - 04: `warehouse_license_date` (дата лицензии/свидетельства СВХ; цель: графа 30: G_30_DATE)
  - 05: `actual_gross_weight` (фактический вес по весам; цель: сверка с общим брутто: графа 35: G_35_1 (контроль))
  - 06: `actual_places` (фактическое количество мест; цель: графа 6: G_6_1 (приоритет #1))
  - 07: `transport_reg_number` (номер ТС при въезде/по отчету СВХ; цель: сверка с графой 18: G_18)

- **Товары в разрезе строк ДО (если в документе есть таблица с разбиением):**
  - 08: `goods_[n]`
    - 01: `tnved` (код товара; цель: сопоставление с товарными блоками ДТ)
    - 02: `places` (кол-во грузовых мест по строке; цель: `BLOCK/G_31/PLACE` и контроль графы 6)
    - 03: `gross_weight_kg` (вес брутто по строке; цель: `BLOCK/G_35_1`)
    - 04: `cost` (стоимость по строке; цель: `BLOCK/G_42_1` (контроль по валюте инвойса))
    - 05: `currency_code` (буквенный код валюты, напр. `CNY`; цель: контроль)
  
  - _item_audit: 5

- _audit: 8

**Правило:** если в ДО есть только общие итоги без разбивки по товарам — массив `goods_[n]` всё равно материализовать
как минимум одним элементом и ставить `pending` в `tnved/places/gross_weight_kg/cost/currency_code`, с вопросом в Раздел III.

### 7.2. Storage Report Additional Sheet

- **uqi_prefix:** `non_formalized.svh_additional_sheet_[n]`
- **Зачем:** адрес и код таможни СВХ для графы 30.
- **Ключевые поля:**
  - 01: `number` (№ доп.листа/приложения; цель: для графы 30 в составе печатной строки: G_30P_1 (как часть derived))
  - 02: `date` (дата доп.листа; цель: для графы 30 в составе печатной строки: G_30P_1 (как часть derived))
  - 03: `actual_gross_weight` (фактический вес по весам; цель: сверка с графой 35: G_35_1 (контроль))
  - 04: `actual_places` (фактическое количество мест; цель: уточнение/сверка для графы 6: G_6_1)
  - 05: `transport_reg_number` (номер ТС при въезде; цель: сверка с графой 18: G_18)
  - 06: `svh_address_region` (регион СВХ; цель: графа 30: G_30_SUB)
  - 07: `svh_address_city` (город/нас.пункт СВХ; цель: графа 30: G_30_CIT)
  - 08: `svh_address_street_house` (улица/дом СВХ как в отчете, без “улучшений”; цель: графа 30: G_30_STR)
  - 09: `svh_customs_code` (код таможенного органа в зоне СВХ; цель: графа 30: G_30_12)

- _audit: 9

### 7.4. Certificate of Origin / Сертификат происхождения (06013)

- **uqi_prefix:** `non_formalized.certificate_of_origin_[n]`
- **Зачем:** документ для графы 44 (если прикладывается) и для обоснования страны происхождения.
- **Поля:**
  - 01: `number` (номер сертификата; цель: графа 44: G44/G442)
  - 02: `date` (дата сертификата; цель: графа 44: G44/G443)

- _audit: 2

### 7.5. Conformity Document / Декларация о соответствии EAC (01191)

- **uqi_prefix:** `non_formalized.conformity_document_[n]`
- **Зачем:** документ для графы 44 (если прикладывается) и подтверждение требований (если применимо к товару).
- **Поля:**
  - 01: `number` (номер декларации/сертификата; цель: графа 44: G44/G442)
  - 02: `date_start` (дата начала действия; цель: обычно только для сверок/контекста, в dt.xml напрямую не переносится)
  - 03: `date_end` (дата окончания действия; цель: обычно только для сверок/контекста, в dt.xml напрямую не переносится)

- _audit: 3

### 7.6. Transit Declaration / Транзитная декларация (09013)

- **uqi_prefix:** `non_formalized.td_[n]`
- **Зачем:** источник данных для derived по графе 29 ДТ (таможенный орган) + реквизиты документа для графы 44 
  (если прикладывается).
- **Поля:**
  - 01: `number` (номер ТД; цель: G44/G442 (если прикладывается) и/или derived)
  - 02: `date` (дата ТД; цель: G44/G443 (если прикладывается) и/или derived)
  - 03: `customs_post_code` (код таможенного органа; цель: графа 29: G_29_1)
  - 04: `customs_post_name` (наименование таможенного органа; цель: графа 29: G_29_2)
  - 05: `transport_reg_number` (ТС по ТД; цель: сверка с графой 18: G_18; если не читается — pending)

- _audit: 5

**Правило:** если `customs_post_code` / `customs_post_name` отсутствуют в md-версии ТД или не читаются надежно — 
`pending` + вопрос в Раздел III.

---

## Раздел III: Формат `primary.md`

primary.md — обычный Markdown файл. В этой схеме примеры фрагментов разметки приводятся в fenced blocks (```), 
но в самом primary.md fenced blocks использовать не нужно.

### Разделы:
1) Метаданные
2) formalized
3) non_formalized
4) Нерешенные вопросы (Issues)
```
## meta:
- `название кейса`: <название кейса>
- `путь к папке поставки`: <путь к папке поставки>
- `direction`: <ИМ / ЭК> (импорт / экспорт) 
- `тип поставки`: <например: 1 ДТ / 1 товар>
- `источники данных:` <например: md + operator_provided_data + stable_source (xml)>

## formalized / non_formalized:

### `document`: <тип документа>
  - `uqi_prefix`: <префикс, например formalized.invoice_1>
  - `xml_target_root`: <корневой тег XML, если применим>
  - `path`: <путь к файлу>
  - `file_name`: <имя файла>
  - `note`: <(опционально) пояснение>
```

### Таблица полей
Далее идет таблица:
- AI обязан материализовать все поля, указанные в шаблоне документа;
- Для пустых значений полей ячейка таблицы остается пустой;
- Если для поля не удалось установить значение, status=pending;

**Нумерация и контроль потерь (Жесткие индексы):**
- Все поля в шаблонах пронумерованы в формате `NN: FieldName`, начиная с 1 без пропусков.
- AI **ОБЯЗАН** подставлять эти номера в таблицу полей `primary.md` в том же порядке.
- В конце каждого документа и в конце каждого массива AI **ОБЯЗАН** вывести фактическое и требуемое число полей:
  - `_audit` (для документа),
  - `_item_audit` (для массива).
  - Несовпадение = автоматический признак потери данных.

Формат таблицы:
```
| num                | field       | value             | status          | description         | note             |
|--------------------|-------------|-------------------|-----------------|---------------------|------------------|
| <порядковый номер> | <FieldName> | <value или пусто> | <CD/CO/pending> | <назначение поля>   | <note или пусто> |
```

### Link вместо большого текста  (описание, не материализуется)
Если по схеме допускается link (например ContractTerms_ContractText), то:
- value = `link:<relative_path>`
- status = CD

### Для вложенных структур/массивов
Массивы оформлять подзаголовками:
```
#### <ArrayName>_<n>
затем таблица полей этого элемента тем же форматом.
```

### После каждого элемента массива:
Выводится итог материализации:
```
#### Итого, по элементу массива:
- `item_fields`: <число полей> из <_item_audit>
```

### После каждого массива:
Выводится итог материализации:
```
#### Итого, по массиву:
- `array_elements`: <число элементов массива>
- `item_fields`: всего полей <число полей> из <_item_audit> * <array_elements>
- `array_status`: <confirmed / pending>
```

### После каждого документа:
Выводится итог материализации:
```
#### Итого, по документу:
- `doc_fields`: <число полей> из <_audit>
- `doc_formalization_status`: <confirmed / pending>
```
Поле `doc_formalization_status` oпциональноe, только для формализуемых документов - готовность к генерации xml. 
pending - если документ не найден или хотя бы одно **ФОРМАЛИЗУЕМОЕ** поле имеет статус pending. Неформализуемые поля 
не учитываются.

### Итого, по файлу:
Полный итог:
```
### Итогo, по файлу:

`total_doc_fields` - <сумма всех doc_fields по всем документам, включая non_formalized, без учета массивов>
`total_fields` - <сумма всех полей, включая поля массивы>
`formalization_status` - confirmed, если ВСЕ формализуемые документы получили статус confirmed.

```
## Нерешенные вопросы (Issues)

**Для полей:**
- `<UQI поля со статусом pending>`
  - `question`: <текст вопроса AI>

**Для общих вопросов:**
- `[Общий]`
  - `question`: <текст вопроса AI>
```

---

## РАЗДЕЛ IV: Порядок работы (задание).

### 1. Прочитать / убедиться, что структура следующих каталогов, включая размеры файлов, известна. 
  - `alta\source\...` (первичные документы всех кейсов) 
  - `alta\source\<кейс>\...`  (первичные документы текущей поставки)
  - `alta\stable_source\` (документы, не меняющиеся между поставками)
  - `alta\source\<кейс>\operator\` (Опционально. Данные, предоставленные оператором. Каталог может 
    отсутствовать / быть пустым, если диалога с оператором не было.)
   
### 2. Прочитать все исходные файлы, шаблоны для которых имеются в данной схеме.
  - Если файл кажется посторонним, прочесть хотя бы его начало, чтобы убедиться, что он является таковым.
  - **ПРИОРИТЕТ ФОРМАТОВ. ПЕРВЫМИ ПРОБУЙ ЛЕГКООБРАБАТЫВАЕМЫЕ ФОРМАТЫ.** Если документ представлен в нескольких форматах,
    выбирай самый удобный. Приоритет:
  - Если документ в текстовом формате (txt, md, xml), это самый лучший вариант.
  - Если сайт принимает файлы Docx, Xlsx, то предпочитай эти форматы.
  - Если есть образ или набор образов документа (часто документ не помещается в один скриншот). Это PNG и другие форматы.
  - PDF, если нет других вариантов. Его обработка тяжелее и менее надежна.

### 3. Сгенерировать `primary.md`.

- После генерации AI **ОБЯЗАН ПРОВЕРИТЬ `primary.md`**. Чек-лист:

1) **Фиксация версии и контекста**
- ✅ Проверить правильность заполнения раздела метаданных.

2) **Полнота состава документов**
- ✅ все документы, которые **присутствуют в первичке** и для которых есть шаблоны в этом промпте, присутствуют в 
  `formalized`/`non_formalized`,

3) **Полнота полей по каждому документу**
- ✅ проверить полное соответствие номеров и идентификаторов полей в схеме и `primary.md`,
- ✅ для массивов (`InvoiceGoods`, `PackingList.Goods`, `CMRGoods`, `non_formalized.svh.goods`) **МАТЕРИАЛИЗОВАНЫ ВСЕ 
  ЭЛЕМЕНТЫ `1..N` СОГЛАСНО ПЕРВИЧКЕ** (не агрегированы и не потеряны).

4) **Проверка сохранности существенных фактов (“якоря”)**
- ✅ для каждого ключевого документа выписаны 3–10 “якорей” и сверены с первичкой:
  - Contract: №/дата, сумма/валюта, срок, стороны.
  - Invoice: №/дата, валюта, `TotalCost`, места, веса, delivery terms, число товарных строк.
  - PackingList: totals (net/gross), места, число строк.
  - CMR: №/дата, места, gross, авто.
  - SVH/ДО-1: №/дата, лицензия, места, разбивка по товарам (если есть).
  - TransitDeclaration: №/дата, `customs_post_code/name`, ТС.
- ✅ если по “якорям” есть расхождения между документами — это вынесено в `primary_review.md` как конфликт 
  (требуется решение).

5) **Проверка достоверности статусов `pending` для полей:** для каждого поля со статусом `pending` проверяется 
   невозможность нахождения правильного значения `value`.

6) **Совместно с оператором разреши `pending` для formalized полей**

7) **Зафиксируй ответы в `operator_provided_data.md`**, чтобы в будущих прогонах не задавать те же самые вопросы.

### 4. Сгенерировать `primary_review.md`

Генерируется на основе `primary.md`. Если имеют место нехватка / конфликты данных, в чате предложить оператору:
- ответить на вопросы
- пересоздать `operator_provided_data.md` с включением полученных данных

Перегенерировать/корректировать и проверить `primary.md`, заново выполнив `` ### 3. Сгенерировать `primary.md` ``.

### Примечания

- Для AI **РАЗРЕШЕНА** запись в `alta\stage_1.0_result\<кейс>\...`, файлы `primary.md`, 
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

  