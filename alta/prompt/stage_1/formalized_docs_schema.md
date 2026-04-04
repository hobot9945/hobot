# Схема formalized_docs.md: правила формализации документов

# Инструкция по этапу 1: Формализация документов

## 0. Назначение этапа
1. **Главная цель:** Подготовить первичные документы к подаче в таможню через Альту.
2. **Вторичная цель:** Сформировать базу данных, достаточную для автоматического построения ДТ (Этап 2) без повторного 
   обращения к PDF/сканам.
3. **Эталоны (образцы XML):** Используются ИСКЛЮЧИТЕЛЬНО для понимания структуры документа и имен тегов (`xml_target`). 
   Запрещено переносить из эталонов любые факты (номера, даты, телефоны, суммы), не подтвержденные первичкой текущего кейса.

## 1. Общие принципы формализации
1. **Никаких догадок:** Если поле в документе отсутствует или не читается — статус `pending`.
2. **Документная изоляция:** Каждый формализованный документ отражает только то, что написано в нем самом. Если Инвойс 
   противоречит Контракту — фиксируем оба «как есть». Конфликты будут разрешаться на этапе 2.
3. **Сохранение полноты:** Формализация не должна приводить к потере информации.
   - Структурированные документы (Invoice, CMR, PL) — максимально раскладываются по полям.
   - Текстовые документы (Контракт, Техописание, Свободные документы) — сохраняют основной текст в специальном поле 
     (например, `text_body`), помимо ключевых реквизитов (номер, дата).
4. **Связи:** Если документ ссылается на другой (например, Инвойс на Контракт), эта связь должна быть отражена в 
   соответствующих полях.

## 2. Типы документов и их специфика
- **Contract / SupplementaryContract:** Реквизиты + полный текст.
- **Invoice:** По-позиционная детализация товаров, стоимостей, весов и условий.
- **PackingList:** Упаковка, веса, места, связь с инвойсом.
- **CMR:** Транспорт, водитель, номера ТС, упаковка, отметки о приеме.
- **PaymentOrder:** Сумма, валюта, дата, назначение платежа (связь с инвойсом/контрактом).
- **ServiceInvoice:** Транспортные расходы с обязательным выделением затрат "до границы" и "после границы".
- **TechDescription:** Обычно формализуется как `FreeDoc` (Свободный документ): заголовок + ключевые реквизиты + полный 
  структурированный текст описания.

## Метаданные

- case_name: <название кейса>
- source_folder: <путь к папке поставки>
- dt_scope: <например: 1 ДТ / 1 товар>
- status: <draft / in_progress / blocked>
- ready_for_next_step: <yes / partial / no>
- unresolved_conflicts_count: <число>
- unresolved_missing_critical_data_count: <число>
- note: <(опционально) короткое пояснение по стадии>

## Файл:

- `document`: <например contract, invoice, CMR>
  - `full_path`: <путь к файлу> 
  - `name`: <имя файла>
  - `xml_target`: <имя xml тега>
  - `status`: <статус>
  - `blocking_for_next_step`: <yes / no>
  - note: <(опциональное) пояснение значения>

## Поле:

- `идентификатор поля`
  - `value`: <значение>
  - `xml_target`: <имя xml тега>
  - `status`: <статус>
  - note: <(опциональное) пояснение значения>

## Статусы значений

- `confirmed_document` — подтверждено документами поставки.
- `confirmed_operator` — подтверждено оператором.
- `pending` — искали, но не нашли.

## 3. Библиотека шаблонов

### 3.1. Contract / Supplementary Agreement (03011 / 03012)
- **xml_target_root:** `AltaE2CONT`
- **Поля:**
  - `number`: [xml_target: ContractRegistration_PrDocumentNumber]
  - `date`: [xml_target: ContractRegistration_PrDocumentDate]
  - `total_amount`: [xml_target: ContractTerms_Amount]
  - `currency_code`: [xml_target: ContractTerms_CurrencyCode] (цифровой код)
  - `delivery_terms`: [xml_target: ContractTerms_OtherTerms] (например, EXW Хэншуй)
  - `expiry_date`: [xml_target: ContractTerms_LastDate]
  - `seller_name`: [xml_target: ForeignPerson_OrganizationName]
  - `buyer_name`: [xml_target: RussianPerson_OrganizationName]
  - `text_body`: [xml_target: ContractTerms_ContractText] (полный текст)

### 3.2. Invoice (04021)
- **xml_target_root:** `AltaE2I`
- **Заголовок:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date`: [xml_target: Registration_PrDocumentDate]
  - `total_amount`: [xml_target: TotalCost]
  - `currency_code`: [xml_target: CurrencyCode] (буквенный код)
  - `exchange_rate`: [xml_target: CurrencyRate]
- **Товарные позиции (InvoiceGoods):**
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

### 3.3. Packing List (04131)
- **xml_target_root:** `AltaE2PACK`
- **Заголовок:**
  - `number`: [xml_target: DeliveryTerms_Registration_PrDocumentNumber]
  - `date`: [xml_target: DeliveryTerms_Registration_PrDocumentDate]
  - `total_gross`: [xml_target: GrossWeightQuantity]
  - `total_net`: [xml_target: NetWeightQuantity]
  - `contract_ref`: [xml_target: DeliveryTerms_Contract_PrDocumentNumber]
  - `invoice_ref`: [xml_target: DeliveryTerms_Invoice_PrDocumentNumber]
- **Товарные позиции (Goods):**
  - `item_no`: [порядковый номер]
  - `description`: [xml_target: GoodsDescription]
  - `quantity_places_or_units`: [xml_target: GoodsQuantity]
  - `gross_weight`: [xml_target: GrossWeightQuantity]
  - `net_weight`: [xml_target: NetWeightQuantity]
  - `packing_quantity`: [xml_target: PakingQuantity]

### 3.4. CMR (02015)
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
- **Товарные позиции (CMRGoods):**
  - `item_no`: [xml_target: GoodsNumeric]
  - `description`: [xml_target: GoodsDescription]
  - `tnved`: [xml_target: GoodsNomenclatureCode]
  - `quantity_places_or_units`: [xml_target: GoodsQuantity]
  - `gross_weight`: [xml_target: GrossWeightQuantity]
  - `packing_code`: [xml_target: PackingCode]
  - `packing_quantity`: [xml_target: PakingQuantity]
  - `packing_description`: [xml_target: PackingDescription]

### 3.5. Payment Order (04023)
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

### 3.6. Service Invoice / Счет за перевозку (04031)
- **xml_target_root:** `AltaServiceInvoice`
- **Заголовок:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date`: [xml_target: Registration_PrDocumentDate]
  - `total_amount`: [xml_target: TotalServiceCost]
  - `currency`: [xml_target: Currency]
  - `service_provider_name`: [xml_target: ServiceProvider_Name]
  - `contract_ref_number`: [xml_target: ContractDetails_PrDocumentNumber]
  - `contract_ref_date`: [xml_target: ContractDetails_PrDocumentDate]
- **Услуги (ServiceDescription):**
  - `item_no`: [порядковый номер]
  - `goods_description`: [xml_target: GoodsDescription]
  - `route_description`: [xml_target: ServiceName]
  - `amount`: [xml_target: ServiceCost_Amount]
  - `currency`: [xml_target: ServiceCost_Currency]
  - `tax_rate`: [xml_target: TaxRate]
  - `tax_sum`: [xml_target: TaxSum]
- **Примечание по интерпретации:**
  - если в документе явно выделены плечи перевозки до и после границы, их нужно сохранять как отдельные позиции услуг, 
    а не сливать.

### 3.7. TechDescription / FreeDoc (05999)
- **xml_target_root:** `AltaFreeDoc`
- **Поля:**
  - `doc_name`: [xml_target: DocumentHead_DocumentName]
  - `number`: [xml_target: DocumentHead_DocumentNumber]
  - `date`: [xml_target: DocumentHead_DocumentDate]
  - `text_body`: [xml_target: TextPara] (весь технический текст)
  - `document_sign`: [xml_target: DocumentSign]
- **Правило формализации:**
  - для документов типа `FreeDoc` не требуется атомарно раскладывать весь текст на отдельные технические поля, если 
    документ в эталоне представлен как единый текстовый блок;
  - обязательно сохранить шапку документа и полный текст без потери содержания.

### 3.8. FreeBinaryDoc / бинарное приложение
- **Назначение:** документ, подаваемый как бинарное вложение (например, PDF), если Альта хранит его не как 
  структурированный текстовый документ, а как контейнер бинарных данных.
- **Поля верхнего уровня:**
  - `doc_name`: <если читается из связанного текстового документа или имени файла>
  - `number`: <если читается>
  - `date`: <если читается>
  - `binary_file_ref`: <ссылка на исходный файл / имя файла>
  - `note`: <что это за вложение и с каким документом связано>
- **Правило формализации:**
  - если рядом существует текстовый `FreeDoc`, содержащий реквизиты и содержание документа, то `FreeBinaryDoc` 
    используется как приложение-носитель бинарного файла;
  - в `formalized_docs.md` достаточно отразить связь бинарного вложения с содержательным документом и исходным файлом.

### 3.9. Общие правила для товарных массивов
- Для повторяющихся товарных позиций использовать единый шаблон массива:
  - `item_no`
  - `description`
  - `tnved` / `tnved_code`
  - `quantity`
  - `unit`
  - `gross_weight`
  - `net_weight`
  - `amount`
  - `model`
  - `manufacturer`
  - `origin_country_code`
- Если конкретный документ не содержит часть полей из этого шаблона, отсутствующие поля фиксируются со статусом 
  `pending`, а не выдумываются.

### 3.10. Общие правила для текстовых документов
- Если документ по своей природе является текстовым и в эталоне представлен как `Contract` или `FreeDoc`, формализация 
  должна включать:
  - реквизиты документа;
  - полный текст или основной текстовый блок;
  - ссылки на связанные документы, если они явно указаны в тексте.
- Нельзя сокращать текст до «краткого пересказа», если это приводит к потере юридически или технически значимой информации.
