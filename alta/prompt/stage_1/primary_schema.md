# Инструкция по этапу 1: Сбор и формализация первичных данных (primary_schema.md)

## 1. Цель этапа

Полное извлечение фактов из папки `source` и `stable_source`, а также получение в чате, путем диалога с 
оператором, недостающих данных и разрешение конфликтов. Результатом работы является файл `primary.md`, 
который служит входом для этапа 2. На его основе также строится `review_1.md` и генерируется пакет 
xml-файлов формализованных документов, предназначенных для импорта в Альту.

Важно: **primary.md — это не summary, а формальная база данных. Неполная структура = ошибка этапа.**

## 2. Общие принципы

0. В формировании `primary.md` участвуют **ВСЕ** первичные документы, которые могут быть представлены приведенными 
   в этом промпте шаблонами.

1. primary.md является строго нормализованной структурной базой данных. Запрещено сокращать, агрегировать 
   или схлопывать структуру документа. Все поля, предусмотренные шаблоном типа документа, обязаны присутствовать 
   в primary.md, даже если их значение — pending. Недопустимо заменять структурные блоки (включая массивы goods_[n], 
   реквизиты, статусы) кратким описанием или сводкой. Отсутствие обязательного поля считается ошибкой этапа 1.

2. **Завершенность обработки:** Все вопросы, касающиеся первичных данных должны быть решены на этом этапе либо на основе 
   документов, либо в диалоге с оператором в чате AI. По результату корректируется значение поля, статус выставляется 
   в `confirmed_operator`. Наличие любого поля со статусом `pending` автоматически означает невозможность перехода 
   к следующему этапу в рабочем режиме. Переход к следующему этапу при недостатке / конфликте данных может допускаться 
   при отладке / формировании черновиков по явной просьбе оператора.

3. **Тотальное покрытие:** Каждый релевантный файл в папке `source` и `stable_source` должен быть обработан. 
   `stable_source` не считается эталоном. Он используется наравне с фактами поставки.

4. **Никаких догадок (Pending = Вопрос):** Если поле в документе отсутствует, не читается или неоднозначно —
   `value` устанавливается в `pending`, `status` = `pending`. AI обязан создать в **Разделе III** запись
   с точным UQI этого поля и **конкретным вопросом о его значении**.

5. **Документная изоляция:** На этапе 1 каждый документ обрабатывается независимо. AI не сопоставляет товарные позиции
   между различными документами, не агрегирует их и не пытается привести к единой структуре поставки. Все расхождения
   между документами фиксируются как конфликт, статус приемного поля/полей переходит в pending, описание проблемы идет в
   Раздел III.

6. **Сохранение полноты:** Формализация не должна приводить к потере информации.
  - Структурированные документы — максимально раскладываются по полям.
  - Текстовые документы — помимо реквизитов, сохраняют основной текст в поле `text_body` или путь к файлу, если
    текст большой.

7. **Связи:** Если документ ссылается на другой (например, Инвойс на Контракт), эта связь отражается в соответствующих
   полях.

8. На этапе 1 AI не имеет права использовать как источник фактов:
  - файлы из каталогов выгрузок;
  - эталонные xml;
  - результаты прошлых прогонов;
  - ранее сгенерированные `primary.md`, `review_1.md` и иные производные файлы;
  - любые документы, специально подготовленные для отладки промптов.

Такие материалы допустимы только в режиме отладки промптов и не должны влиять на формализацию текущей поставки в рабочем
режиме.

Исключением является файл `operator_provided_data.md`, формируемый в процессе прогона. Он предназначен для дополнения
первичных документов в последующих прогонах этапа 1, если потребуются дополнительные прогоны.

## 3. Архитектура файла primary.md

Файл `primary.md` строго разделен на три логических раздела:

1. **Раздел I `formalized` (Формализуемые документы):** Документы, на базе которых будут строиться XML-файлы для Альты 
   (Контракт, Инвойс, CMR, Упаковочный лист, Счета, Техописания, Личные документы). Для них идентификаторы полей 
   совпадают с XML тегами.

2. **Раздел II `non_formalized` (Неформализуемые документы):** Документы, которые не требуют генерации в XML, но содержат 
   критичные факты для ДТ (Отчет СВХ / ДО-1, транспортный контракт и т.д.).

3. **Раздел III: Нерешенные вопросы:** вопросы к оператору - пробелы, конфликты.

## 4. UQI документов и пути к полям

Каждый документ в `primary.md` имеет строго определенный строковый идентификатор `uqi_prefix` (формат: 
`[раздел].[тип_документа]_[n]`).

Примеры: `formalized.invoice_1`, `non_formalized.svh_1`.

Поля отдельным атрибутом `uqi` не снабжаются.
Для Этапа 2 логический путь к полю строится динамически:
- обычное поле: `<uqi_prefix>.<имя_поля>`
- элемент массива: `<uqi_prefix>.<имя_массива>_[n].<имя_поля>`

Здесь `n` - индекс документа или массива, начиная с 1, без ведущих нулей. Обязателен даже в случае одного элемента.

**Примеры:**

- `formalized.invoice_1.date` — дата первого инвойса.
- `formalized.invoice_1.goods_2.net_weight` — вес нетто второго товара в первом инвойсе.
- `formalized.service_invoice_1.services_1.amount` — сумма первой услуги в счете за перевозку.
- `non_formalized.svh_1.actual_gross_weight` — фактический вес брутто по первому отчету ДО-1.

## 5. Форматы элементов в primary.md

### Метаданные (в начале файла):

- `case_name`: <название кейса>
- `source_folder`: <путь к папке поставки>
- `dt_scope`: <например: 1 ДТ / 1 товар>
- `status`: <draft / in_progress / ready / pending>
- `unresolved_conflicts_count`: <число>
- `unresolved_pending_count`: <число>
- `note`: <(опционально) короткое пояснение>

### Формат описания Документа:

- `document`: <тип документа>
  - `uqi_prefix`: <префикс, например formalized.invoice_1>
  - `xml_target_root`: <корневой тег XML, если применим>
  - `full_path`: <путь к файлу>
  - `file_name`: <имя файла>
  - `status`: <confirmed / pending> (pending - если документ не найден или хотя бы одно поле имеет статус pending)
  - `note`: <(опционально) пояснение>

### Формат описания Поля:
Для формализуемых документов идентификатор поля совпадает с xml тегом. Как правило, это camel case. Для неформализуемых
документов идентификаторы поля строчными латинскими буквами через подчеркивания.

- `<идентификатор поля>`
  - `value`: <значение или pending>
  - `status`: <confirmed_document / confirmed_operator / pending>
  - `note`: <(опционально) пояснение, при материализии копируется из шаблона (в скобках, после идентификатора поля)>

**AI не имеет права опускать поле, указанное в шаблоне документа:**
Все поля, предусмотренные шаблоном соответствующего типа документа, обязаны присутствовать в структуре документа 
в `primary.md`. Если значение поля отсутствует в документе или не читается, поле создаётся со значением `pending`.

**Для экономии места при материализации поля оформляются таблицей**

### Общие правила для реквизитов и адресов:
- Для всех формализуемых документов реквизиты сторон (Продавец, Покупатель, Отправитель, Получатель) должны максимально 
  полно извлекаться из текста.
- Обязательно выделять: `inn`, `kpp`, `ogrn` (для российских компаний), `short_name`.
- Адреса необходимо по возможности разбивать на логические компоненты: `country_code`, `country_name`, `region`, `city`, 
  `street_house`, `postal_code`, чтобы на этапе 3 они корректно легли в XML.

### Общие правила для товарных массивов:

- Каждая товарная позиция (`goods_[n]`) оформляется строго в соответствии с шаблоном конкретного документа.
- Поля, предусмотренные шаблоном документа, обязательны к присутствию в структуре.
- Если значение для предусмотренного поля отсутствует в документе, `value` устанавливается в `pending`.
- AI не имеет права опускать поле, указанное в шаблоне документа.

### Общие правила для текстовых документов:

- Если документ по своей природе является текстовым (Contract, FreeDoc), формализация должна включать: реквизиты, 
  полный текст или основной текстовый блок, ссылки на связанные документы.
- Нельзя сокращать текст до «краткого пересказа», если это приводит к потере значимой информации.

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

2. **Дополнительные нефомализуемые данные** — сведения, присутствующие в документе, но не участвующие в построении XML. 
   Такие данные необходимо сохранять в `primary.md`, если они зафиксированы в этой схеме. Если они 
   не зафиксированы в схеме, но требуются для формирования ДТ, вопрос помещается в Раздел III. 

Рекомендуемое оформление таких данных:
- `value`: <значение> | `link`: <ссылка на первичный документ>
- `status`: <confirmed_document | confirmed_operator | pending>
- `note`: дополнительное поле, не используется для генерации xml

### Объемные документы
Если поле `value` грозит стать слишком большим (например, техническое описание), оно заменяется на поле `link`, которое
содержит ссылку на первичный документ. При формировании xml текст можно будет взять из него.

### 6.1. Contract / Контракт (03011)

- **xml_target_root:** `AltaE2CONT`
- **uqi_prefix:** `formalized.contract_[n]`

- **Поля:**
  - `DocumentCode` (03011)
  - `ContractRegistration_PrDocumentNumber` (№ контракта)
  - `ContractRegistration_PrDocumentDate` (дата контракта)

  - `ContractTerms_Amount` (общая сумма контракта)
  - `ContractTerms_CurrencyCode` (цифровой код валюты (ISO 4217 numeric); пример `CNY` = 156)
  - `ContractTerms_LastDate` (срок действия/срок исполнения)
  - `ContractTerms_OtherTerms` (условия поставки / Incoterms; пример: `EXW ...`)
  - `ContractTerms_ContractText` (полный текст контракта - хранить `link` на файл-источник; см. примечание)
  - `ContractTerms_DealSign` (обычно `1`)

  - `ForeignPerson_OrganizationName` (продавец)
  - `ForeignPerson_Address_CountryCode` (код страны, alpha-2; пример: `CN`)
  - `ForeignPerson_Address_CounryName` (НАМЕРЕННО так: **опечатка в теге** `CounryName`)
  - `ForeignPerson_Address_Region`
  - `ForeignPerson_Address_City`
  - `ForeignPerson_Address_StreetHouse`

  - `RussianPerson_OrganizationName` (покупатель)
  - `RussianPerson_OGRN` (ОГРН)
  - `RussianPerson_INN` (ИНН)
  - `RussianPerson_KPP` (КПП)
  - `RussianPerson_Address_PostalCode`
  - `RussianPerson_Address_CountryCode` (alpha-2; пример: `RU`)
  - `RussianPerson_Address_CounryName` (НАМЕРЕННО так: **опечатка в теге** `CounryName`)
  - `RussianPerson_Address_Region`
  - `RussianPerson_Address_City`
  - `RussianPerson_Address_StreetHouse`

**Примечание:**
`ContractTerms_ContractText` в `primary.md` не копировать полный текст контракта, сохранять только `link`
на файл-источник. Полный текст подставлять только при генерации XML. Нужно для сохранения компактности `primary.md`.

### 6.2. Supplementary Contract / Дополнительное соглашение к контракту (03012)

- **xml_target_root:** `AltaSupplementaryContract`
- **uqi_prefix:** `formalized.supplementary_contract_[n]`

- **Поля:**
  - `DocumentNumber` (№ доп. соглашения; не как в контракте `ContractRegistration_*`)
  - `IssueDate` (дата доп. соглашения)

  - `ContractDescription_Amount` (новая/уточненная сумма)
  - `ContractDescription_CurrencyCode` (цифровой код валюты)
  - `ContractDescription_LastDate` (срок действия/исполнения)
  - `ContractDescription_ContractText` (полный текст контракта - хранить `link` на файл-источник)
  - `ContractDescription_DealSign` (`1`; смысл неясен)
  - `ContractDescription_StockCategorySign`
  - `ContractDescription_BuyerLimitationSign`
  - `ContractDescription_InsuranceSign`

  - `RussianPerson_OrganizationName`
  - `RussianPerson_ShortName`
  - `RussianPerson_OGRN`
  - `RussianPerson_INN`
  - `RussianPerson_KPP`
  - `RussianPerson_Address_PostalCode`
  - `RussianPerson_Address_CountryCode`
  - `RussianPerson_Address_CounryName` (опечатка `CounryName`)
  - `RussianPerson_Address_Region`
  - `RussianPerson_Address_City`
  - `RussianPerson_Address_StreetHouse`

  - `ForeignPerson_OrganizationName`
  - `ForeignPerson_ShortName`
  - `ForeignPerson_Address_CountryCode`
  - `ForeignPerson_Address_CounryName` (опечатка `CounryName`)
  - `ForeignPerson_Address_Region`
  - `ForeignPerson_Address_City`
  - `ForeignPerson_Address_StreetHouse`

  - `ContractSignedPerson` (подписант в доп. соглашении)
    - `PersonSurname`
    - `PersonName`
    - `PersonMiddleName`

### 6.3. Invoice (04021)

- **xml_target_root:** `AltaE2I`
- **uqi_prefix:** `formalized.invoice_[n]`

- **Поля шапки / реквизиты документа:**
  - `CurrencyRate` (курс валюты; пример: 10.9430)
  - `CurrencyCode` (валюта инвойса, ISO 4217 alpha-3, например `CNY`, `USD`)
  - `DocumentCode` (04021)
  - `PlacesQuantity` (кол-во мест)
  - `PlacesDescription` (описание мест, напр. "Поддон")
  - `GrossWeightQuantity` (общий вес брутто)
  - `NetWeightQuantity` (общий вес нетто)
  - `GCost` (дублирует TotalCost; системное поле Альты, назначение не очевидно)
  - `TotalCost` (итого по инвойсу)

  - `DeliveryTerms_DeliveryPlace` (место поставки по инкотермс)
  - `DeliveryTerms_DeliveryTermsNumericCode` (числовой код условий; пример: `01`)
  - `DeliveryTerms_DeliveryTermsStringCode` (строковый код условий; пример: `EXW`)
  - `DeliveryTerms_DispatchCountryCode` (код страны отправления, alpha-2; пример: `CN`)
  - `DeliveryTerms_TradingCountryCode` (торгующая страна, alpha-2; пример: `CN`)
  - `DeliveryTerms_DestinationCountryCode` (страна назначения, alpha-2; пример: `RU`)

  - `Registration_PrDocumentName` (наименование документа)
  - `Registration_PrDocumentNumber` (номер инвойса)
  - `Registration_PrDocumentDate` (дата инвойса)

  - `Contract_PrDocumentNumber` (номер контракта-ссылки)
  - `Contract_PrDocumentDate` (дата контракта-ссылки)

- **Стороны (местами теги “кривые” — это часть структуры Альты):**
  - `Buyer_CompanyID` (это ИНН покупателя; название тега вводит в заблуждение)
  - `Buyer_KPPCode` (КПП покупателя)
  - `Buyer_Name` (наименование покупателя)
  - `Buyer_PostalAddress_PostalCode`
  - `Buyer_PostalAddress_CountryCode`
  - `Buyer_PostalAddress_CounryName` (опечатка **CounryName**)
  - `Buyer_PostalAddress_Region`
  - `Buyer_PostalAddress_City`
  - `Buyer_PostalAddress_StreetHouse`

  - `Seler_Name` (опечатка **Seler**; это продавец)
  - `Seler_PostalAddress_CountryCode`
  - `Seler_PostalAddress_CounryName` (опечатка **CounryName**)
  - `Seler_PostalAddress_Region`
  - `Seler_PostalAddress_City`
  - `Seler_PostalAddress_StreetHouse`

  - `Consignor_OrganizationName` (грузоотправитель)
  - `Consignor_Address_CountryCode`
  - `Consignor_Address_CounryName` (опечатка **CounryName**)
  - `Consignor_Address_Region`
  - `Consignor_Address_City`
  - `Consignor_Address_StreetHouse`

  - `Consignee_OrganizationName` (грузополучатель)
  - `Consignee_OGRN`
  - `Consignee_INN`
  - `Consignee_KPP`
  - `Consignee_Address_PostalCode`
  - `Consignee_Address_CountryCode`
  - `Consignee_Address_CounryName` (опечатка **CounryName**)
  - `Consignee_Address_Region`
  - `Consignee_Address_City`
  - `Consignee_Address_StreetHouse`

- **Товарные позиции** (каждый элемент соответствует узлу `<InvoiceGoods>...</InvoiceGoods>`):
  - `InvoiceGoods_[n]`
    - `GoodsCode` (ТН ВЭД)
    - `GoodsDescription` (описание товара как в инвойсе; часто включает “ОТСУТСТВУЕТ”)
    - `GoodsQuantity` (кол-во)
    - `MeasureUnitQualifierName` (единица измерения, напр. "М2" - квадратные метры)
    - `GrossWeightQuantity` (брутто по строке)
    - `NetWeightQuantity` (нетто по строке)
    - `Price` (цена за единицу)
    - `TotalCost` (стоимость по строке)
    - `OriginCountryCode` (цифровой код страны происхождения; пример: 156)

    - `AdditionalGoodsDescription_Manufacturer` (производитель)
    - `AdditionalGoodsDescription_TradeMark` (торг. марка; если отсутствует в первичке — заполнять "ОТСУТСТВУЕТ")
    - `AdditionalGoodsDescription_GoodsMark` (товарный знак/маркировка; если отсутствует — "ОТСУТСТВУЕТ")
    - `AdditionalGoodsDescription_GoodsModel` (тег называется *GoodsModel*, но по факту используется как наименование
      позиции, напр. "АНТИКОТ 1.4 * 30")

### 6.4. Packing List / Упаковочный лист (04131)

- **xml_target_root:** `AltaE2PACK`
- **uqi_prefix:** `formalized.packing_list_[n]`

- **Поля:**
  - `GrossWeightQuantity` (общий вес брутто)
  - `NetWeightQuantity` (общий вес нетто)

  - `Consignor_OrganizationName` (грузоотправитель)
  - `Consignor_ShortName` (краткое наименование; часто совпадает с полным)
  - `Consignor_Address_CountryCode` (код страны, alpha-2; пример: `CN`)
  - `Consignor_Address_CounryName` (опечатка в теге: `CounryName`)
  - `Consignor_Address_Region`
  - `Consignor_Address_City`
  - `Consignor_Address_StreetHouse`

  - `Consignee_OrganizationName` (грузополучатель)
  - `Consignee_ShortName`
  - `Consignee_OGRN`
  - `Consignee_INN`
  - `Consignee_KPP`
  - `Consignee_Address_PostalCode`
  - `Consignee_Address_CountryCode` (alpha-2; пример: `RU`)
  - `Consignee_Address_CounryName` (опечатка в теге: `CounryName`)
  - `Consignee_Address_Region`
  - `Consignee_Address_City`
  - `Consignee_Address_StreetHouse`

  - `DeliveryTerms_DeliveryPlace` (место поставки по Incoterms)
  - `DeliveryTerms_DeliveryTermsNumericCode` (числовой код условий; пример: `01`)
  - `DeliveryTerms_DeliveryTermsStringCode` (строковый код условий; пример: `EXW`)

  - `DeliveryTerms_Contract_PrDocumentName` (наименование документа-контракта)
  - `DeliveryTerms_Contract_PrDocumentNumber` (номер контракта)
  - `DeliveryTerms_Contract_PrDocumentDate` (дата контракта)

  - `DeliveryTerms_Invoice_PrDocumentName` (наименование документа-инвойса)
  - `DeliveryTerms_Invoice_PrDocumentNumber` (номер инвойса)
  - `DeliveryTerms_Invoice_PrDocumentDate` (дата инвойса)

  - `DeliveryTerms_Registration_PrDocumentName` (наименование упаковочного листа)
  - `DeliveryTerms_Registration_PrDocumentNumber` (номер упаковочного листа)
  - `DeliveryTerms_Registration_PrDocumentDate` (дата упаковочного листа)

- **Товарные/грузовые строки** (каждый элемент соответствует узлу `<Goods>...</Goods>`):
  - `Goods_[n]`
    - `GoodsDescription` (описание “строки” как в документе; может быть агрегированным текстом)
    - `GoodsQuantity` (количество мест/грузовых единиц в строке; это НЕ количество товара в штуках/м2 и т.п.)
    - `GrossWeightQuantity` (брутто по строке)
    - `NetWeightQuantity` (нетто по строке)
    - `PackingInfo`
      - `PakingQuantity` (опечатка в теге: `PakingQuantity`; смысл — количество упаковок/мест в упаковке, может быть 0/не заполняться)

- **Транспорт** (каждый элемент соответствует узлу `<TransportMeans>...</TransportMeans>`):
  - `TransportMeans_[n]`
    - `Number` (регистрационный номер)
    - `ModeCode` (код вида транспорта; пример: `31`)
    - `NationalityCode` (может быть `000`/неочевидно — сохраняем как есть)
    - `MoverIndicator` (`true` для тягача, `false` для прицепа)
    
**Правило:** если известны номер тягача и номер прицепа, сохранять их как ДВА элемента:
  - transport_1 (MoverIndicator=true) — тягач
  - transport_2 (MoverIndicator=false) — прицеп

### 6.5. CMR / Международная товарно-транспортная накладная (02015)

- **xml_target_root:** `AltaE3CMR`
- **uqi_prefix:** `formalized.cmr_[n]`

- **Поля:**
  - `LanguageCode` (язык документа; обычно `RU`)
  - `CMR_Choice` (системный выбор/вариант; обычно `1`)

  - `RegistrationDocument_RegID` (номер CMR)
  - `RegistrationDocument_DateInf` (дата CMR)
  - `RegistrationDocument_Place` (место составления/оформления)

  - `TrakingCargo_TakingCargoDate` (дата принятия груза к перевозке; опечатка в теге: `TrakingCargo`)
  - `TrakingCargo_TakingCargoPlace_CountryCode` (страна принятия груза, alpha-2)
  - `TrakingCargo_TakingCargoPlace_CounryName` (опечатка в теге: `CounryName`)

  - `DeliveryPlace_CountryCode` (страна доставки, alpha-2)
  - `DeliveryPlace_CounryName` (опечатка в теге: `CounryName`)

  - `DeliveryTerms_DeliveryPlace` (место поставки по Incoterms, если присутствует в CMR)
  - `DeliveryTerms_DeliveryTermsStringCode` (условия поставки, строковый код, напр. `EXW`)

  - `GoodsQuantity` (общее количество грузовых мест/упаковок по CMR; не “кол-во товара”)
  - `CMRGoodsWeight_GrossWeightQuantity` (общий вес брутто по CMR)

  - `CMRTransport_PrimeMoverStateSignID` (гос. номер тягача)
  - `CMRTransport_TrailerStateSignID` (гос. номер прицепа)

- **Отправитель (как в структуре Альты):**
  - `Consignor_NameInf` (наименование)
  - `Consignor_ShortName` (краткое наименование)
  - `Consignor_PostalAddress_CountryCode` (alpha-2)
  - `Consignor_Address_CounryName` (опечатка: `CounryName`)
  - `Consignor_Address_Region`
  - `Consignor_Address_City`
  - `Consignor_Address_StreetHouse`

- **Гарант отправителя** (если присутствует в структуре):
  - `Consignor_Guarantee_OrganizationName`
  - `Consignor_Guarantee_ShortName`
  - `Consignor_Guarantee_Address_CountryCode`
  - `Consignor_Guarantee_Address_CounryName` (опечатка: `CounryName`)
  - `Consignor_Guarantee_Address_Region`
  - `Consignor_Guarantee_Address_City`
  - `Consignor_Guarantee_Address_StreetHouse`

- **Получатель:**
  - `Consignee_NameInf` (наименование)
  - `Consignee_ShortName`
  - `Consignee_OGRNID` (ОГРН; имя тега вводит в заблуждение — это не “ID документа”, а реквизит организации)
  - `Consignee_INNID` (ИНН; аналогично)
  - `Consignee_KPPCode` (КПП)
  - `Consignee_PostalAddress_PostalCode`
  - `Consignee_PostalAddress_CountryCode` (alpha-2)
  - `Consignee_Address_CounryName` (опечатка: `CounryName`)
  - `Consignee_Address_Region`
  - `Consignee_Address_City`
  - `Consignee_Address_StreetHouse`

- **Товарные/грузовые строки** (каждый элемент соответствует узлу `<CMRGoods>...</CMRGoods>`):
  - `CMRGoods_[n]`
    - `GoodsNumeric` (порядковый номер строки)
    - `GoodsDescription` (описание груза/товара как в CMR)
    - `GoodsNomenclatureCode` (код товара; по факту используется как ТН ВЭД)
    - `GoodsQuantity` (количество мест/упаковок в строке; не “кол-во товара”)
    - `GrossWeightQuantity` (вес брутто по строке)
    - `GoodsPackingInfo`
      - `PackingCode` (код вида упаковки, напр. `PX`)
      - `PakingQuantity` (опечатка в теге: `PakingQuantity`; количество упаковок/мест)
      - `PackingDescription` (описание упаковки, напр. `ПОДДОН`)

**Правило:** если в CMR перечислены несколько товарных строк/упаковок — сохранять их как goods_1..goods_N. Не 
агрегировать в одну строку "по инвойсу".

### 6.6. Payment Order / Платежное поручение (04023)

- **xml_target_root:** `AltaPaymentOrder`
- **uqi_prefix:** `formalized.payment_order_[n]`

- **Поля:**
  - `DocumentCode` (04023)

  - `PaymentModeCode` (код способа платежа; системное поле)
  - `PaymentAmount` (сумма платежа)
  - `TransactionKind` (вид операции/код; системное поле)
  - `Priority` (очередность; системное поле, может быть символом вроде `"."`)
  - `Purpose` (назначение платежа; может включать ссылки на контракт/инвойс)
  - `ValueSpelledOut` (сумма прописью)

  - `DocumentReference_PrDocumentNumber` (номер платежного поручения)
  - `DocumentReference_PrDocumentDate` (дата платежного поручения)

  - `Payer_OrganizationName` (плательщик)
  - `Payer_INN` (ИНН плательщика)
  - `Payer_KPP` (КПП плательщика)
  - `Payer_Bank_BankName` (в структуре Альты сюда часто попадает блок реквизитов/адреса плательщика; название тега вводит
    в заблуждение — это не только “название банка”)

  - `Payee_OrganizationName` (получатель)
  - `Payee_Bank_BankName` (аналогично: реквизиты банка/получателя, может быть многострочным текстом)

  - `PayerSign` (подписант/плательщик)
    - `PersonSurname`
    - `PersonName`


### 6.7. Service Invoice / Счет за перевозку (04031)

- **xml_target_root:** `AltaServiceInvoice`
- **uqi_prefix:** `formalized.service_invoice_[n]`

- **Поля:**
  - `DocumentSign` (признак документа; системное поле, обычно `1`)
  - `TotalServiceCost` (итого по услугам)
  - `Currency` (валюта итого, ISO 4217 alpha-3, например `USD`)

  - `ServiceProvider_Name` (исполнитель услуг)
  - `ServiceProvider_PaymentRequisitions`
    - `BankName` (банк исполнителя)

  - `ContractDetails_PrDocumentNumber` (номер договора на услуги/перевозку)
  - `ContractDetails_PrDocumentDate` (дата договора на услуги/перевозку)

  - `PaymentDocument` (связанный документ, в структуре Альты так названо)
    - `PrDocumentNumber` (номер)
    - `PrDocumentDate` (дата)

  - `Registration_PrDocumentName` (наименование счета)
  - `Registration_PrDocumentNumber` (номер счета)
  - `Registration_PrDocumentDate` (дата счета)

  - `Consignor_OrganizationName` (грузоотправитель)
  - `Consignor_SubjectAddressDetails`
    - `PostalCode`
    - `CountryCode` (alpha-2)
    - `CounryName` (опечатка в теге: `CounryName`)
    - `Region`
    - `Town` (город/нас.пункт; тег отличается от `City`)
    - `StreetHouse`

  - `Consignee_OrganizationName` (грузополучатель)
  - `Consignee_RFOrganizationFeatures_OGRN` (ОГРН)
  - `Consignee_RFOrganizationFeatures_INN` (ИНН)
  - `Consignee_RFOrganizationFeatures_KPP` (КПП)
  - `Consignee_SubjectAddressDetails`
    - `PostalCode`
    - `CountryCode`
    - `CounryName` (опечатка в теге: `CounryName`)
    - `Region`
    - `Town`
    - `StreetHouse`
    - `House` (дом — отдельным полем, если присутствует)
    - `Room` (офис/кв — отдельным полем, если присутствует)

- **Услуги** (каждый элемент соответствует узлу `<ServiceDescription>...</ServiceDescription>`):
  - `ServiceDescription_[n]`
    - `GoodsDescription` (многострочное описание услуги — сохранять как есть)
    - `CurrencyCode` (валюта строки; ISO alpha-3)
    - `ServiceName` (наименование/маршрут)
    - `TaxRate` (ставка налога; часто `0.00`)
    - `TaxSum` (сумма налога; часто `0.00`)
    - `ServiceCost_Amount` (стоимость строки)
    - `ServiceCost_Currency` (валюта стоимости строки)

- **Подписи** (системный блок структуры Альты):
  - `Signature_Choice`
  - `SignatureDirectorChiefAccountant_Director_PersonSurname`
  - `SignatureDirectorChiefAccountant_Director_PersonName`
  - `SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname`
  - `SignatureDirectorChiefAccountant_ChiefAccountant_PersonName`

### 6.8. Insurance Document / Счет за страховку (04111)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.insurance_document_[n]`

- **Поля:**
  - `DocumentCode` (04111)
  - `DocumentHead_DocumentName` (наименование документа)
  - `DocumentHead_DocumentDate` (дата документа)
  - `DocumentHead_DocumentNumber` (номер документа)

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (основной текст; `link` на файл-источник)

### 6.9. TechDescription / Техническое описание (05999)

Наличие нескольких технических описаний для разных товаров допустимо. Но, если несколько технических описаний 
относятся к одному и тому же товару (совпадает наименование, модель или явная ссылка на товар) или не может быть
соотнесено с товаром, AI не делает предположений об их релевантности и обязан вынести вопрос в Раздел III.

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.tech_description_[n]`

- **Поля:**
  - `DocumentCode` (05999)
  - `DocumentHead_DocumentName` (наименование документа)
  - `DocumentHead_DocumentDate` (дата документа)
  - `DocumentHead_DocumentNumber` (номер документа)

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (технический текст без пересказа; хранить `link` на файл-источник)

### 6.10. FreeDoc / Прочие текстовые документы (09999)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.free_doc_[n]`

- **Поля:**
  - `DocumentCode` (09999)
  - `DocumentHead_DocumentName` (например: отказное письмо и т.п.)
  - `DocumentHead_DocumentDate`
  - `DocumentHead_DocumentNumber`

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (основной текст; хранить `link` на файл-источник)

### 6.11. FreeBinaryDoc / Бинарное приложение (например PDF-вложение)

- **xml_target_root:** `AltaFreeBinaryDoc`
- **uqi_prefix:** `formalized.free_binary_doc_[n]`

- **Поля:**
  - `DocumentCode` (код документа; в бинарных обычно свой, зависит от типа вложения)
  - `DocumentInfo_PrDocumentName` (наименование документа, напр. "КОММЕРЧЕСКИЙ ИНВОЙС")
  - `DocumentInfo_PrDocumentNumber` (номер документа-основания/вложения)
  - `DocumentInfo_PrDocumentDate` (дата документа-основания/вложения)

  - `DocumentBody_FileName` (имя файла вложения, напр. `...pdf`)
  - `DocumentBody_FileData` (base64 содержимого файла; **в primary.md не хранить base64**, хранить `link` на исходный
    файл; base64 формировать только при генерации XML)
  - `Thumbnail` (миниатюра/base64; если не требуется для импорта — не материализовывать, можно опускать)

### 6.12. Personal Passport / Паспорт (11001)

- **xml_target_root:** `AltaPassport`
- **uqi_prefix:** `formalized.passport_[n]`

- **Поля:**
  - `CardSeries` (серия)
  - `CardNumber` (номер)
  - `OrganizationName` (кем выдан)
  - `CardDate` (дата выдачи)

  - `PersonInfo_PersonSurname`
  - `PersonInfo_PersonName`
  - `PersonInfo_PersonMiddleName`
  - `PersonInfo_Sex` (1/2; значение брать как в документе)
  - `PersonInfo_Birthday`
  - `PersonInfo_Birthplace`

  - `ResidencePlace_PostalCode`
  - `ResidencePlace_CountryCode`
  - `ResidencePlace_CounryName` (если встречается; возможна опечатка `CounryName` как в других структурах)
  - `ResidencePlace_Region`
  - `ResidencePlace_City`
  - `ResidencePlace_StreetHouse` (адрес одной строкой, если так дано в структуре)


### 6.13. Letter of Attorney / Доверенность (11004)

- **xml_target_root:** `AltaLetterOfAttorney`
- **uqi_prefix:** `formalized.letter_of_attorney_[n]`

- **Поля:**
  - `Subject` (полный текст доверенности; `link` на файл-источник)
  - `EndDate` (действительна до)

  - `DocumentReference_PrDocumentName` (наименование документа)
  - `DocumentReference_PrDocumentNumber` (номер доверенности)
  - `DocumentReference_PrDocumentDate` (дата доверенности)

  - `Organization_OrganizationName` (выдавшая организация)
  - `Organization_ShortName`
  - `Organization_OGRN`
  - `Organization_INN`
  - `Organization_KPP`
  - `Organization_Address_PostalCode`
  - `Organization_Address_CountryCode`
  - `Organization_Address_CounryName` (если используется; возможна опечатка)
  - `Organization_Address_Region`
  - `Organization_Address_City`
  - `Organization_Address_StreetHouse`

  - `Organization_OrganizationPerson_PersonSurname` (руководитель, подписант)
  - `Organization_OrganizationPerson_PersonName`
  - `Organization_OrganizationPerson_PersonMiddleName`
  - `Organization_OrganizationPerson_PersonPost`

  - `EmpoweredPerson_PersonSurname` (уполномоченное лицо)
  - `EmpoweredPerson_PersonName`
  - `EmpoweredPerson_PersonMiddleName`
  - `EmpoweredPerson_PersonPost`

  - `EmpoweredPerson_Passport_IdentityCardCode`
  - `EmpoweredPerson_Passport_IdentityCardName`
  - `EmpoweredPerson_Passport_IdentityCardSeries`
  - `EmpoweredPerson_Passport_IdentityCardNumber`
  - `EmpoweredPerson_Passport_IdentityCardDate`
  - `EmpoweredPerson_Passport_OrganizationName`

### 6.14. Transport Contract / Договор транспортной экспедиции (04033)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.transport_contract_[n]`

- **Поля:**
  - `DocumentCode` (04033)
  - `DocumentHead_DocumentName` (наименование документа)
  - `DocumentHead_DocumentDate` (дата документа)
  - `DocumentHead_DocumentNumber` (номер документа)

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (текст договора; хранить `link` на файл-источник)

### 6.15. EGRUL / Выписка из ЕГРЮЛ (04011)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.egrul_[n]`

- **Поля:**
  - `DocumentCode` (04011)
  - `DocumentHead_DocumentName` (наименование документа)
  - `DocumentHead_DocumentDate` (дата выписки)
  - `DocumentHead_DocumentNumber` (номер выписки)

  - `DocumentBody_TextSection`
    - `TextPara_[n]` (текст выписки; хранить `link` на файл-источник)

---

## 7. РАЗДЕЛ II: Неформализуемые документы

Документы извлекаются ради фактов для сборки ДТ.

### 7.1. Storage Report / Отчет СВХ (ДО-1 / ДО-2) (10061/10062)

- **uqi_prefix:** `non_formalized.svh_[n]`
- **Зачем:** Данные для Графы 30.
- **Ключевые поля:**
  - `number`
  - `date`
  - `warehouse_license`: (лицензия СВХ)
  - `actual_gross_weight`: (фактический вес по весам)
  - `actual_places`: (фактическое количество мест)
  - `transport_reg_number`: (номер ТС при въезде)
  - `non_xml_fields`: (пока не формализовано)

### 7.2. Storage Report Additional Sheet

- **uqi_prefix:** `non_formalized.svh_additional_sheet_[n]`
- **Зачем:** Данные для Графы 30.
- **Ключевые поля:**
  - `number`
  - `date`
  - `actual_gross_weight`: (фактический вес по весам)
  - `actual_places`: (фактическое количество мест)
  - `transport_reg_number`: (номер ТС при въезде)
  - `non_xml_fields`: (пока не формализовано)

### 7.3. Certificate of Origin / Сертификат происхождения (06013)

- **uqi_prefix:** `formalized.certificate_of_origin_[n]`
- **Поля:**
  - `number`
  - `date`

### 7.4. Conformity Document / Декларация о соответствии EAC (01191)

- **uqi_prefix:** `formalized.conformity_document_[n]`
- **Поля:**
  - `number`
  - `date_start`
  - `date_end`
  - 
### 7.5. Transit Declaration / Транзитная декларация (09013)

- **uqi_prefix:** `formalized.td_[n]`
- **Поля:**
  - `number` (в формате XXXXXXXX/XXXXXX/XXXXXXX)
  - `date`

---

## 8. РАЗДЕЛ III: Нерешенные вопросы

Здесь размещаются все вопросы к оператору:
- по конкретным полям
- общие

### Формат записи:

**Для поля:**
- `<UQI поля со статусом pending>`
  - `question`: <текст вопроса AI>

**Для общего вопроса:**
- `[Общий]`
  - `question`: <текст вопроса AI>

---

# ПОРЯДОК РАБОТЫ

### 1. Прочитать / убедиться, что структура следующих каталогов, включая размеры файлов, известна. 
  - `alta\source\...` (первичные документы текущей поставки). Пример: 
    `alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02`
  - `alta\stable_source` (документы, не меняющиеся между поставками)
  - `alta\operator` (Опционально. Данные, предоставленные оператором. Каталог может отсутствовать / быть пустым, если
     диалога с оператором не было.)
   
### 2. Прочитать все файлы, шаблоны для которых имеются в данной схеме.
  - Если файл кажется посторонним, прочесть хотя бы его начало, чтобы убедиться, что он является таковым.
  - Файлы, имеющие одинаковые имена, но разные типы считать разными форматами одного и того же файла.
  - Для чтения выбирать самый удобный формат. Например, если есть xlsx и pdf варианты файла, выбрать pdf.

### 3. Сгенерировать `primary.md`, `review_1.md`.
Если имеют место нехватка / конфликты данных, в чате предложить оператору варианты:
  - ответить на вопросы, потом сгенерировать `primary.md`;
  - сгенерировать черновой `primary.md`. 

Для AI **РАЗРЕШЕНА** запись в `alta\stage_1_result\<каталог поставки>\`, файлы `primary.md`, `review_1.md` пишутся
в этот каталог.

Для AI **РАЗРЕШЕНА** запись в `alta\source\...\operator\` (вместе с первичкой). Если был диалог с оператором, в этот каталог 
помещается файл `operator_provided_data.md`, содержащий информацию, полученную от оператора, которая может быть 
использована как первичка на следующих прогонах. Если перед диалогом этот файл существовал, старые данные дополняются 
новыми.

Перед записью файлов AI выполняет чек-лист:
- ✅ прочитаны все исходные документы во всех двух (трех) каталогах исходных документов
- ✅ все документы, для которых имеются шаблоны, присутствуют
- ✅ все поля шаблона присутствуют, даже если value=pending
- ✅ Никакие существенные данные из первички не потеряны. Если есть данные, которые, не могли быть помещены в
  `primary.md`, они должны быть зафиксированы в `review.md` с указанием причины, например, "нет подходящего шаблона".

### 4. Сгенерировать `review_1.md`
Генерируется на основе `primary.md` / исходных документов.

### Примечание:
 Для файлов документов использовать следующие способы доступа:
  - текстовые файлы: команды Хобота read_file, write_file;
  - pdf, png: перетаскивание в поле ввода;
  - xml, используемые для импорта / экспорта: read_file, write_file с параметрами кодировки cp1251; 

  