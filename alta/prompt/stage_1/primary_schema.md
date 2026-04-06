# Инструкция по этапу 1: Сбор и формализация первичных данных (primary_schema.md)

## 0. Цель этапа

Полное извлечение фактов из папки `source` и `stable_source`, а также получение в чате, путем диалога с 
оператором, недостающих данных и разрешение конфликтов. Результатом работы является файл `primary.md`, 
который служит входом для этапа 2. На его основе также строится `review_1.md` и генерируется пакет 
xml-файлов формализованных документов, предназначенных для импорта в Альту.

## 1. Архитектура файла primary.md

Файл `primary.md` строго разделен на три логических раздела:

1. **Раздел `formalized` (Формализуемые документы):** Документы, на базе которых будут строиться XML-файлы для Альты 
   (Контракт, Инвойс, CMR, Упаковочный лист, Счета, Техописания, Личные документы). Для них обязателен атрибут 
   `xml_target`.
2. **Раздел `non_formalized` (Неформализуемые документы):** Документы, которые не требуют генерации в XML, но содержат критичные факты для ДТ (Отчет СВХ / ДО-1, Транзитная декларация, Сертификаты). Для них `xml_target` опционален.
3. **Раздел `operator` (Данные от оператора):** Ответы оператора на вопросы AI. Сюда попадают восполненные пробелы (разрешенные `pending`) и решения по конфликтам.

## 2. Уникальные квалифицированные идентификаторы (UQI)

Каждое извлекаемое поле в `primary.md` должно иметь строго определенный строковый путь (UQI), по которому Этап 2 сможет его однозначно прочитать.

**Формат UQI:** `[раздел].[тип_документа]_[индекс].[имя_поля]`
Для товарных позиций/массивов: `[раздел].[тип_документа]_[индекс].[имя_массива]_[индекс].[имя_поля]`

**Примеры:**

- `formalized.invoice_1.date` — дата первого инвойса.
- `formalized.invoice_1.goods_2.net_weight` — вес нетто второго товара в первом инвойсе.
- `formalized.service_invoice_1.services_1.amount` — сумма первой услуги в счете за перевозку.
- `non_formalized.svh_1.actual_gross_weight` — фактический вес брутто по первому отчету ДО-1.
- `operator.contract_1_date` — дата контракта, подтвержденная оператором.

## 3. Общие принципы формализации

1. **Тотальное покрытие:** Каждый релевантный файл в папке `source` и `stable_source` должен быть обработан.
2. **Никаких догадок (Pending = Вопрос):** Если поле в документе отсутствует или не читается — `value` устанавливается в `pending`. Запрещено выводить данные из косвенных ссылок без подтверждения оператора. Любой `pending` — это триггер для внесения вопроса в `review_1.md` и последующей фиксации ответа в разделе `operator`.
3. **Документная изоляция:** Каждый документ отражает только то, что написано в нем самом. Если Инвойс противоречит Контракту — фиксируем оба «как есть». Конфликты разрешаются через оператора.
4. **Сохранение полноты:** Формализация не должна приводить к потере информации.
   - Структурированные документы — максимально раскладываются по полям.
   - Текстовые документы — сохраняют основной текст в поле `text_body`, помимо реквизитов.
5. **Связи:** Если документ ссылается на другой (например, Инвойс на Контракт), эта связь отражается в соответствующих полях.
6. **Эталоны (образцы XML):** Используются ИСКЛЮЧИТЕЛЬНО для понимания структуры документа и имен тегов (`xml_target`). Запрещено переносить из эталонов любые факты, не подтвержденные первичкой текущего кейса.

## 4. Форматы элементов в primary.md

### Метаданные (в начале файла):

- `case_name`: <название кейса>
- `source_folder`: <путь к папке поставки>
- `dt_scope`: <например: 1 ДТ / 1 товар>
- `status`: <draft / in_progress / blocked>
- `ready_for_next_step`: <yes / partial / no>
- `unresolved_conflicts_count`: <число>
- `unresolved_pending_count`: <число>
- `note`: <(опционально) короткое пояснение>

### Формат описания Документа:

- `document`: <тип документа>
  - `uqi_prefix`: <префикс, например formalized.invoice_1>
  - `full_path`: <путь к файлу>
  - `name`: <имя файла>
  - `xml_target_root`: <корневой тег XML, если применим>
  - `status`: <confirmed_document / pending>
  - `blocking_for_next_step`: <yes / no>
  - `note`: <(опционально) пояснение>

### Формат описания Поля:

- `имя_поля`
  - `uqi`: <полный идентификатор, например formalized.invoice_1.number>
  - `value`: <значение или pending>
  - `xml_target`: <имя xml тега, если применимо>
  - `status`: <confirmed_document / confirmed_operator / pending>
  - `note`: <(опционально) пояснение>

### Общие правила для товарных массивов:

- Каждая товарная позиция (item) — это массив структурированных полей. ЗАПРЕЩЕНО выводить товар просто списком строк.
- Каждое поле внутри товара должно следовать общему формату Поля (с обязательным `uqi`).
- Минимальный набор полей для товара (если применимо): `item_no`, `description`, `tnved`, `quantity`, `unit`, `price`, `amount`, `gross_weight`, `net_weight`, `packing_quantity`, `model`, `manufacturer`, `origin_country_code`.

### Общие правила для текстовых документов:

- Если документ по своей природе является текстовым (Contract, FreeDoc), формализация должна включать: реквизиты, полный текст или основной текстовый блок, ссылки на связанные документы.
- Нельзя сокращать текст до «краткого пересказа», если это приводит к потере значимой информации.

---

## 5. РАЗДЕЛ I: Формализуемые документы (Шаблоны)

### 5.1. Contract / Supplementary Agreement (03011 / 03012)

- **xml_target_root:** `AltaE2CONT` (или `AltaSupplementaryContract`)
- **Поля:**
  - `number`: [xml_target: ContractRegistration_PrDocumentNumber]
  - `date`: [xml_target: ContractRegistration_PrDocumentDate]
  - `total_amount`: [xml_target: ContractTerms_Amount]
  - `currency_code`: [xml_target: ContractTerms_CurrencyCode] (цифровой код)
  - `delivery_terms`: [xml_target: ContractTerms_OtherTerms]
  - `expiry_date`: [xml_target: ContractTerms_LastDate]
  - `seller_name`: [xml_target: ForeignPerson_OrganizationName]
  - `buyer_name`: [xml_target: RussianPerson_OrganizationName]
  - `text_body`: [xml_target: ContractTerms_ContractText] (полный текст)

### 5.2. Invoice (04021)

- **xml_target_root:** `AltaE2I`
- **Заголовок:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date`: [xml_target: Registration_PrDocumentDate]
  - `total_amount`: [xml_target: TotalCost]
  - `currency_code`: [xml_target: CurrencyCode] (буквенный код)
  - `exchange_rate`: [xml_target: CurrencyRate]
- **Товарные позиции (goods_N):**
  - `item_no`: [порядковый номер]
  - `description`: [xml_target: GoodsDescription]
  - `tnved`: [xml_target: GoodsCode]
  - `quantity`: [xml_target: GoodsQuantity]
  - `unit`: [xml_target: MeasureUnitQualifierName]
  - `price`: [xml_target: Price]
  - `amount`: [xml_target: TotalCost]
  - `gross_weight`: [xml_target: GrossWeightQuantity]
  - `net_weight`: [xml_target: NetWeightQuantity]
  - `origin_country_code`: [xml_target: OriginCountryCode]
  - `manufacturer`: [xml_target: AdditionalGoodsDescription_Manufacturer]
  - `model`: [xml_target: AdditionalGoodsDescription_GoodsModel]

### 5.3. Packing List (04131)

- **xml_target_root:** `AltaE2PACK`
- **Заголовок:**
  - `number`: [xml_target: DeliveryTerms_Registration_PrDocumentNumber]
  - `date`: [xml_target: DeliveryTerms_Registration_PrDocumentDate]
  - `total_gross`: [xml_target: GrossWeightQuantity]
  - `total_net`: [xml_target: NetWeightQuantity]
  - `contract_ref`: [xml_target: DeliveryTerms_Contract_PrDocumentNumber]
  - `invoice_ref`: [xml_target: DeliveryTerms_Invoice_PrDocumentNumber]
- **Товарные позиции (goods_N):**
  - `item_no`: [порядковый номер]
  - `description`: [xml_target: GoodsDescription]
  - `quantity_places_or_units`: [xml_target: GoodsQuantity]
  - `gross_weight`: [xml_target: GrossWeightQuantity]
  - `net_weight`: [xml_target: NetWeightQuantity]
  - `packing_quantity`: [xml_target: PakingQuantity]

### 5.4. CMR (02015)

- **xml_target_root:** `AltaE3CMR`
- **Заголовок:**
  - `number`: [xml_target: RegistrationDocument_RegID]
  - `date`: [xml_target: RegistrationDocument_DateInf]
  - `registration_place`: [xml_target: RegistrationDocument_Place]
  - `taking_cargo_date`: [xml_target: TrakingCargo_TakingCargoDate]
  - `taking_cargo_country_code`: [xml_target: TrakingCargo_TakingCargoPlace_CountryCode]
  - `delivery_country_code`: [xml_target: DeliveryPlace_CountryCode]
- **Транспорт:**
  - `truck_number`: [xml_target: CMRTransport_PrimeMoverStateSignID]
  - `trailer_number`: [xml_target: CMRTransport_TrailerStateSignID]
  - `total_places`: [xml_target: GoodsQuantity]
  - `total_gross_weight`: [xml_target: CMRGoodsWeight_GrossWeightQuantity]
- **Товарные позиции (goods_N):**
  - `item_no`: [xml_target: GoodsNumeric]
  - `description`: [xml_target: GoodsDescription]
  - `tnved`: [xml_target: GoodsNomenclatureCode]
  - `quantity_places_or_units`: [xml_target: GoodsQuantity]
  - `gross_weight`: [xml_target: GrossWeightQuantity]
  - `packing_code`: [xml_target: PackingCode]
  - `packing_quantity`: [xml_target: PakingQuantity]
  - `packing_description`: [xml_target: PackingDescription]

### 5.5. Payment Order (04023)

- **xml_target_root:** `AltaPaymentOrder`
- **Поля:**
  - `number`: [xml_target: DocumentReference_PrDocumentNumber]
  - `date`: [xml_target: DocumentReference_PrDocumentDate]
  - `amount`: [xml_target: PaymentAmount]
  - `currency_mode_code`: [xml_target: PaymentModeCode]
  - `transaction_kind`: [xml_target: TransactionKind]
  - `purpose`: [xml_target: Purpose]
  - `payer_name`: [xml_target: Payer_OrganizationName]
  - `payer_inn`: [xml_target: Payer_INN]
  - `payer_kpp`: [xml_target: Payer_KPP]
  - `payee_name`: [xml_target: Payee_OrganizationName]
  - `payer_sign_surname`: [xml_target: PersonSurname]
  - `payer_sign_name`: [xml_target: PersonName]

### 5.6. Service Invoice / Счет за перевозку (04031)

- **xml_target_root:** `AltaServiceInvoice`
- **Заголовок:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date`: [xml_target: Registration_PrDocumentDate]
  - `total_amount`: [xml_target: TotalServiceCost]
  - `currency`: [xml_target: Currency]
  - `service_provider_name`: [xml_target: ServiceProvider_Name]
  - `contract_ref_number`: [xml_target: ContractDetails_PrDocumentNumber]
  - `contract_ref_date`: [xml_target: ContractDetails_PrDocumentDate]
- **Услуги (services_N):** (Обязательно сохранять разделение "до" и "после" границы, если есть)
  - `item_no`: [порядковый номер]
  - `goods_description`: [xml_target: GoodsDescription]
  - `route_description`: [xml_target: ServiceName]
  - `amount`: [xml_target: ServiceCost_Amount]
  - `currency`: [xml_target: ServiceCost_Currency]
  - `tax_rate`: [xml_target: TaxRate]
  - `tax_sum`: [xml_target: TaxSum]

### 5.7. Insurance Document / Счет за страховку (04111)

- **xml_target_root:** `AltaFreeDoc`
- **Поля:**
  - `number`: [xml_target: DocumentHead_DocumentNumber]
  - `date`: [xml_target: DocumentHead_DocumentDate]
  - `amount`: [xml_target: TotalCost]
  - `currency`: [xml_target: CurrencyCode]
  - `text_body`: [xml_target: TextPara] (условия страхования)

### 5.8. TechDescription / FreeDoc (05999)

- **xml_target_root:** `AltaFreeDoc`
- **Поля:**
  - `doc_name`: [xml_target: DocumentHead_DocumentName]
  - `number`: [xml_target: DocumentHead_DocumentNumber]
  - `date`: [xml_target: DocumentHead_DocumentDate]
  - `text_body`: [xml_target: TextPara] (весь технический текст)
  - `document_sign`: [xml_target: DocumentSign]

### 5.9. FreeBinaryDoc / бинарное приложение

- **Назначение:** Документ, подаваемый как бинарное вложение (PDF). Если рядом существует текстовый `FreeDoc`, то `FreeBinaryDoc` используется как носитель файла.
- **Поля:**
  - `doc_name`: <если читается>
  - `number`: <если читается>
  - `date`: <если читается>
  - `binary_file_ref`: <ссылка на исходный файл>
  - `note`: <с каким документом связано>

### 5.10. Personal Passport / Паспорт (11001)

- **xml_target_root:** `AltaPassport`
- **Поля:**
  - `series_number`: [xml_target: IdentityCard_SeriesNumber]
  - `issue_date`: [xml_target: IdentityCard_IssueDate]
  - `issued_by`: [xml_target: IdentityCard_OrganizationName]
  - `full_name`: [xml_target: IdentityCard_PersonName]

### 5.11. Letter of Attorney / Доверенность (11004)

- **xml_target_root:** `AltaLetterOfAttorney`
- **Поля:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date`: [xml_target: Registration_PrDocumentDate]
  - `valid_until`: [xml_target: ExpirationDate]
  - `attorney_name`: [xml_target: Attorney_PersonName]
  - `issuer_name`: [xml_target: Issuer_OrganizationName]

### 5.12. EGRUL / Выписка из ЕГРЮЛ (04011)

- **xml_target_root:** `AltaFreeDoc`
- **Поля:**
  - `number`: [xml_target: DocumentHead_DocumentNumber]
  - `date`: [xml_target: DocumentHead_DocumentDate]
  - `company_name`: [xml_target: OrganizationName]
  - `inn`: [xml_target: INN]
  - `kpp`: [xml_target: KPP]
  - `director_name`: [xml_target: PersonName]

### 5.13. Certificate of Origin / Сертификат происхождения (06011)

- **xml_target_root:** `AltaCertificateOfOrigin`
- **Поля:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date`: [xml_target: Registration_PrDocumentDate]
  - `origin_country`: [xml_target: OriginCountryCode]
  - `goods_description`: [xml_target: GoodsDescription]

### 5.14. Conformity Document / Декларация о соответствии EAC (01191)

- **xml_target_root:** `AltaConformityDoc`
- **Поля:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date_start`: [xml_target: IssueDate]
  - `date_end`: [xml_target: ExpirationDate]
  - `goods_list`: [xml_target: GoodsDescription]

### 5.15. Transit Declaration / Транзитная декларация (09013)

- **xml_target_root:** `AltaTD`
- **Поля:**
  - `number`: [xml_target: TransitRegistrationNumber] (в формате XXXXXXXX/XXXXXX/XXXXXXX)
  - `date`: [xml_target: RegistrationDate]
  - `total_gross_weight`: [xml_target: TotalGrossWeight]
  - `total_places`: [xml_target: TotalPackageQuantity]
  - `seals_info`: [xml_target: SealsNumber]
  - `customs_office_code`: [xml_target: CustomsCode]
  - `destination_customs_code`: [xml_target: DestinationCustomsCode]

---

## 6. РАЗДЕЛ II: Неформализуемые документы

Документы извлекаются ради фактов для сборки ДТ. Теги `xml_target` опциональны. Поля оформляются: `value`, `uqi`, `status`.

### 6.1. Transit Declaration / Транзитная декларация (09013)

- **Зачем:** Данные о прибытии груза.
- **Ключевые поля:**
  - `number`: (в формате XXXXXXXX/XXXXXX/XXXXXXX)
  - `date`
  - `total_gross_weight`
  - `total_places`
  - `seals_info`: (номера пломб)
  - `customs_office_code`: (код поста оформления транзита)
  - `destination_customs_code`: (код поста назначения)

### 6.2. Storage Report / Отчет СВХ (ДО-1 / ДО-2) (10061/10062)

- **Зачем:** Данные для Графы 30.
- **Ключевые поля:**
  - `number`
  - `date`
  - `warehouse_license`: (лицензия СВХ)
  - `actual_gross_weight`: (фактический вес по весам)
  - `actual_places`: (фактическое количество мест)
  - `transport_reg_number`: (номер ТС при въезде)

### 6.3. Certificate of Origin / Сертификат происхождения (06011)

- **Зачем:** Преференции по стране.
- **Ключевые поля:**
  - `number`
  - `date`
  - `origin_country`
  - `goods_description`

### 6.4. Conformity Document / Декларация о соответствии EAC (01191)

- **Зачем:** Разрешительный документ.
- **Ключевые поля:**
  - `number`
  - `date_start`
  - `date_end`
  - `goods_list`: (перечень артикулов)

---

## 7. РАЗДЕЛ III: Данные оператора

Заполняется по итогам ответов оператора на вопросы из `review_1.md` (для каждого разрешенного `pending`).

### Формат записи:

- `уникальное_имя_факта`
  - `uqi`: <идентификатор, например operator.contract_1_date>
  - `question`: <текст вопроса AI>
  - `answer`: <текст ответа оператора>
  - `value`: <итоговое значение для использования>
  - `status`: confirmed_operator