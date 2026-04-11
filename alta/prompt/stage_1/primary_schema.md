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

1. primary.md является строго нормализованной структурной базой данных, а не отчетом. Запрещено сокращать, агрегировать 
   или схлопывать структуру документа. Все поля, предусмотренные шаблоном типа документа, обязаны присутствовать 
   в primary.md, даже если их значение — pending. Недопустимо заменять структурные блоки (включая массивы goods_[n], 
   реквизиты, статусы, xml_target) кратким описанием или сводкой. Отсутствие обязательного поля считается ошибкой этапа 1.

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
   (Контракт, Инвойс, CMR, Упаковочный лист, Счета, Техописания, Личные документы). Для них обязателен атрибут 
   `xml_target`.

2. **Раздел II `non_formalized` (Неформализуемые документы):** Документы, которые не требуют генерации в XML, но содержат 
   критичные факты для ДТ (Отчет СВХ / ДО-1, транспортный контракт и т.д.) Для них `xml_target` не требуется.

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
  - `full_path`: <путь к файлу>
  - `name`: <имя файла>
  - `xml_target_root`: <корневой тег XML, если применим>
  - `status`: <confirmed / pending> (pending - если документ не найден или хотя бы одно поле имеет статус pending)
  - `note`: <(опционально) пояснение>

### Формат описания Поля:

- `имя_поля`
  - `value`: <значение или pending>
  - `xml_target`: <имя xml тега, если применимо>
  - `status`: <confirmed_document / confirmed_operator / pending>
  - `note`: <(опционально) пояснение>

Все поля, предусмотренные шаблоном соответствующего типа документа, обязаны присутствовать в структуре документа 
в `primary.md`. Если значение поля отсутствует в документе или не читается, поле создаётся со значением `pending`.

AI не имеет права опускать поле, указанное в шаблоне документа.

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
  - используется обычный формат Поля;
  - указывается `xml_target`, если он применим;
  - именно эти поля считаются источником для дальнейшей xml-формализации.

2. **Дополнительные нефомализуемые данные** — сведения, присутствующие в документе, но не участвующие напрямую 
   в построении XML. Такие данные необходимо сохранять в `primary.md`, если они зафиксированы в этой схеме. Если они 
   не зафиксированы в схеме, но требуются для формирования ДТ, они обязательно должны попасть в Раздел III. 

Для дополнительных нефомализуемых данных AI обязан:
- оформлять их отдельно от формализуемых полей;
- явно помечать, что они **не участвуют в xml-формализации**;
- не смешивать их с полями, предназначенными для генерации XML.

Рекомендуемое оформление таких данных:
- `value`: <значение> | `link`: <ссылка на первичный документ>
- `status`: confirmed_document
- `formalization_role`: non_xml
- `note`: дополнительное поле, не используется для генерации xml

Если по документу извлекаются только формализуемые поля — дополнительный нефомализуемый блок не требуется.  
Если документ содержит значимый текст, расширенные характеристики, пояснения, маркетинговые описания, диапазоны значений,
технические детали или иные сведения вне XML-схемы, они должны быть сохранены отдельно как нефомализуемые данные.

### Объемные документы
Если поле `value` грозит стать слишком большим (например, техническое описание), оно заменяется на поле `link`, которое
содержит ссылку на первичный документ. При формировании xml текст можно будет взять из него.

### 6.1. Contract / Supplementary Agreement (03011 / 03012)

- **xml_target_root:** `AltaE2CONT` (или `AltaSupplementaryContract`)
- **uqi_prefix:** `formalized.contract_[n]`
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
  - `deal_sign`: [xml_target: ContractTerms_DealSign] (обычно 1)
  - `signed_person_surname`: [xml_target: ContractSignedPerson_PersonSurname]
  - `signed_person_name`: [xml_target: ContractSignedPerson_PersonName]
  - (Только для Supplementary) `stock_category_sign`: [xml_target: ContractDescription_StockCategorySign]
  - (Только для Supplementary) `buyer_limitation_sign`: [xml_target: ContractDescription_BuyerLimitationSign]
  - (Только для Supplementary) `insurance_sign`: [xml_target: ContractDescription_InsuranceSign]
  
### 6.2. Invoice (04021)

- **xml_target_root:** `AltaE2I`
- **uqi_prefix:** `formalized.invoice_[n]`
- **Заголовок:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date`: [xml_target: Registration_PrDocumentDate]
  - `total_amount`: [xml_target: TotalCost]
  - `currency_code`: [xml_target: CurrencyCode] (ISO 4217 alpha-3, например CNY, USD)
  - `exchange_rate`: [xml_target: CurrencyRate]
  - `places_quantity`: [xml_target: PlacesQuantity]
  - `places_description`: [xml_target: PlacesDescription]
  - `total_gross_weight`: [xml_target: GrossWeightQuantity]
  - `total_net_weight`: [xml_target: NetWeightQuantity]
  - `dispatch_country_code`: [xml_target: DeliveryTerms_DispatchCountryCode]
  - `destination_country_code`: [xml_target: DeliveryTerms_DestinationCountryCode]
  - `contract_ref_number`: [xml_target: Contract_PrDocumentNumber]
  - `contract_ref_date`: [xml_target: Contract_PrDocumentDate]
- **Товарные позиции (goods_[n]):**
  - `item_no`: [порядковый номер]
  - `description`: [xml_target: GoodsDescription]
  - `tnved`: [xml_target: GoodsCode]
  - `quantity`: [xml_target: GoodsQuantity]
  - `unit`: [xml_target: MeasureUnitQualifierName]
  - `price`: [xml_target: Price]
  - `amount`: [xml_target: TotalCost]
  - `gross_weight`: [xml_target: GrossWeightQuantity]
  - `net_weight`: [xml_target: NetWeightQuantity]
  - `origin_country_code`: [xml_target: OriginCountryCode] (цифровой код страны / ISO 3166-1 numeric, пример: Китай 156, РФ 643)
  - `manufacturer`: [xml_target: AdditionalGoodsDescription_Manufacturer]
  - `model`: [xml_target: AdditionalGoodsDescription_GoodsModel]
  - `trade_mark`: [xml_target: AdditionalGoodsDescription_TradeMark] (если нет - писать "ОТСУТСТВУЕТ", status=confirmed_document)
  - `goods_mark`: [xml_target: AdditionalGoodsDescription_GoodsMark] (если нет - писать "ОТСУТСТВУЕТ", status=confirmed_document)
  
### 6.3. Packing List (04131)

- **xml_target_root:** `AltaE2PACK`
- **uqi_prefix:** `formalized.packing_list_[n]`
- **Заголовок:**
  - `number`: [xml_target: DeliveryTerms_Registration_PrDocumentNumber]
  - `date`: [xml_target: DeliveryTerms_Registration_PrDocumentDate]
  - `total_gross`: [xml_target: GrossWeightQuantity]
  - `total_net`: [xml_target: NetWeightQuantity]
  - `contract_ref`: [xml_target: DeliveryTerms_Contract_PrDocumentNumber]
  - `invoice_ref`: [xml_target: DeliveryTerms_Invoice_PrDocumentNumber]
  - `total_places`: [xml_target: TotalPlacesQuantity]
  - `delivery_place`: [xml_target: DeliveryTerms_DeliveryPlace]
  - `delivery_terms_string_code`: [xml_target: DeliveryTerms_DeliveryTermsStringCode]
  - `registration_doc_name`: [xml_target: DeliveryTerms_Registration_PrDocumentName]
  - `registration_doc_number`: [xml_target: DeliveryTerms_Registration_PrDocumentNumber]
  - `registration_doc_date`: [xml_target: DeliveryTerms_Registration_PrDocumentDate]
- **Товарные позиции (goods_[n]):**
  - `item_no`: [порядковый номер]
  - `description`: [xml_target: GoodsDescription]
  - `quantity_places_or_units`: [xml_target: GoodsQuantity]
  - `gross_weight`: [xml_target: GrossWeightQuantity]
  - `net_weight`: [xml_target: NetWeightQuantity]
  - `packing_quantity`: [xml_target: PakingQuantity]
- **Транспорт (transport_[n]):**
  - `number`: [xml_target: Number] (гос. номер)
  - `mode_code`: [xml_target: ModeCode] (код вида транспорта, например 31)
  - `nationality_code`: [xml_target: NationalityCode]
  - `mover_indicator`: [xml_target: MoverIndicator] (true для тягача, false для прицепа)
  
**Правило:** если известны номер тягача и номер прицепа, сохранять их как ДВА элемента:
  - transport_1 (mover_indicator=true) — тягач
  - transport_2 (mover_indicator=false) — прицеп

### 6.4. CMR (02015)

- **xml_target_root:** `AltaE3CMR`
- **uqi_prefix:** `formalized.cmr_[n]`
- **Заголовок:**
  - `number`: [xml_target: RegistrationDocument_RegID]
  - `date`: [xml_target: RegistrationDocument_DateInf]
  - `registration_place`: [xml_target: RegistrationDocument_Place]
  - `taking_cargo_date`: [xml_target: TrakingCargo_TakingCargoDate]
  - `taking_cargo_country_code`: [xml_target: TrakingCargo_TakingCargoPlace_CountryCode]
  - `delivery_country_code`: [xml_target: DeliveryPlace_CountryCode]
  - `language_code`: [xml_target: LanguageCode] (обычно RU)
  - `cmr_choice`: [xml_target: CMR_Choice] (обычно 1)
- **Транспорт:**
  - `truck_number`: [xml_target: CMRTransport_PrimeMoverStateSignID]
  - `trailer_number`: [xml_target: CMRTransport_TrailerStateSignID]
  - `total_places`: [xml_target: GoodsQuantity]
  - `total_gross_weight`: [xml_target: CMRGoodsWeight_GrossWeightQuantity]
- **Товарные позиции (goods_[n]):**
  - `item_no`: [xml_target: GoodsNumeric]
  - `description`: [xml_target: GoodsDescription]
  - `tnved`: [xml_target: GoodsNomenclatureCode]
  - `quantity_places_or_units`: [xml_target: GoodsQuantity]
  - `gross_weight`: [xml_target: GrossWeightQuantity]
  - `packing_code`: [xml_target: PackingCode]
  - `packing_quantity`: [xml_target: PakingQuantity]
  - `packing_description`: [xml_target: PackingDescription]

**Правило:** если в CMR перечислены несколько товарных строк/упаковок — сохранять их как goods_1..goods_N. Не 
агрегировать в одну строку "по инвойсу".

### 6.5. Payment Order (04023)

- **xml_target_root:** `AltaPaymentOrder`
- **uqi_prefix:** `formalized.payment_order_[n]`
- **Поля:**
  - `number`: [xml_target: DocumentReference_PrDocumentNumber]
  - `date`: [xml_target: DocumentReference_PrDocumentDate]
  - `amount`: [xml_target: PaymentAmount]
  - `payment_mode_code`: [xml_target: PaymentModeCode]
  - `transaction_kind`: [xml_target: TransactionKind]
  - `purpose`: [xml_target: Purpose]
  - `payer_name`: [xml_target: Payer_OrganizationName]
  - `payer_inn`: [xml_target: Payer_INN]
  - `payer_kpp`: [xml_target: Payer_KPP]
  - `payee_name`: [xml_target: Payee_OrganizationName]
  - `payer_sign_surname`: [xml_target: PersonSurname]
  - `payer_sign_name`: [xml_target: PersonName]
  - `payer_bank_name`: [xml_target: Payer_Bank_BankName] (включая адрес и реквизиты банка плательщика)
  - `payee_bank_name`: [xml_target: Payee_Bank_BankName] (включая адрес и реквизиты банка получателя)

### 6.6. Service Invoice / Счет за перевозку (04031)

- **xml_target_root:** `AltaServiceInvoice`
- **uqi_prefix:** `formalized.service_invoice_[n]`
- **Заголовок:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date`: [xml_target: Registration_PrDocumentDate]
  - `total_amount`: [xml_target: TotalServiceCost]
  - `currency`: [xml_target: Currency]
  - `service_provider_name`: [xml_target: ServiceProvider_Name]
  - `contract_ref_number`: [xml_target: ContractDetails_PrDocumentNumber]
  - `contract_ref_date`: [xml_target: ContractDetails_PrDocumentDate]
  - `service_provider_bank_name`: [xml_target: ServiceProvider_PaymentRequisitions_BankName]
- **Услуги (services_[n]):** (Обязательно сохранять разделение "до" и "после" границы, если есть)
  - `item_no`: [порядковый номер]
  - `goods_description`: [xml_target: GoodsDescription]
  - `route_description`: [xml_target: ServiceName]
  - `amount`: [xml_target: ServiceCost_Amount]
  - `currency`: [xml_target: ServiceCost_Currency]
  - `tax_rate`: [xml_target: TaxRate]
  - `tax_sum`: [xml_target: TaxSum]

### 6.7. Insurance Document / Счет за страховку (04111)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.insurance_document_[n]`
- **Поля:**
  - `number`: [xml_target: DocumentHead_DocumentNumber]
  - `date`: [xml_target: DocumentHead_DocumentDate]
  - `amount`: [xml_target: TotalCost]
  - `currency`: [xml_target: CurrencyCode]
  - `text_body`: [xml_target: TextPara] (условия страхования)

### 6.8. TechDescription / FreeDoc (05999)

**Правила для TechDescription / FreeDoc:**

- Основной технический текст должен сохраняться в `text_body` без пересказа. Если текст большой, используется ссылка на 
  исходный файл.

- Наличие нескольких технических описаний для разных товаров допустимо. Но, если несколько технических описаний 
  относятся к одному и тому же товару (совпадает наименование, модель или явная ссылка на товар) или не может быть
  соотнесено с товаром, AI не делает предположений об их релевантности и обязан вынести вопрос в Раздел III.
 
- AI не извлекает отдельные технические характеристики из текста, если такие поля явно не предусмотрены данной схемой.

**xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.tech_description_[n]`
- **Формализуемые поля:**
  - `doc_name`: [xml_target: DocumentHead_DocumentName]
  - `number`: [xml_target: DocumentHead_DocumentNumber]
  - `date`: [xml_target: DocumentHead_DocumentDate]
  - `text_body`: [xml_target: DocumentBody_TextSection/TextPara] (весь технический текст или основной технический текстовый блок)
  - `document_sign`: [xml_target: DocumentSign]

- **Дополнительные нефомализуемые данные (`non_xml_fields`)** — при наличии и только если извлекаются без догадок:
  - `product_group_code`
  - `material`
  - `roll_sizes`
  - `dimensions`
  - `weight_range`
  - `density`
  - `manufacture_method`
  - `warranty`
  - `application`
  - `consumer_properties`

### 6.9. FreeBinaryDoc / бинарное приложение

Возможные имена документов должны быть зафиксированы здесь, чтобы не было разнобоя с этапом 2. Пока их нет.

- **Назначение:** Документ, подаваемый как бинарное вложение (PDF). Если рядом существует текстовый `FreeDoc`, то 
  `FreeBinaryDoc` используется как носитель файла.
- **uqi_prefix:** `formalized.<имя документа>_[n]`
- **Поля:**
  - `doc_name`: <если читается>
  - `number`: <если читается>
  - `date`: <если читается>
  - `binary_file_ref`: <ссылка на исходный файл>
  - `note`: <с каким документом связано>

### 6.10. Personal Passport / Паспорт (11001)

- **xml_target_root:** `AltaPassport`
- **uqi_prefix:** `formalized.passport_[n]`
- **Поля:**
  - `series`: [xml_target: CardSeries]
  - `number`: [xml_target: CardNumber]
  - `issue_date`: [xml_target: CardDate]
  - `issued_by`: [xml_target: OrganizationName]
  - `full_name`: [xml_target: PersonInfo_PersonSurname + PersonInfo_PersonName + PersonInfo_PersonMiddleName]
  - `birth_date`: [xml_target: PersonInfo_Birthday]
  - `birth_place`: [xml_target: PersonInfo_Birthplace]
  - `residence_address`: [xml_target: ResidencePlace_*] (собирать как текстовый адрес без потери компонентов)
  - `sex`: [xml_target: PersonInfo_Sex] (1 - мужской, 2 - женский)

### 6.11. Letter of Attorney / Доверенность (11004)

- **xml_target_root:** `AltaLetterOfAttorney`
- **uqi_prefix:** `formalized.letter_of_attorney_[n]`
- **Поля:**
  - `number`: [xml_target: DocumentReference_PrDocumentNumber]
  - `date`: [xml_target: DocumentReference_PrDocumentDate]
  - `valid_until`: [xml_target: EndDate]
  - `attorney_name`: [xml_target: EmpoweredPerson_*]
  - `attorney_passport_series`: [xml_target: EmpoweredPerson_Passport_IdentityCardSeries]
  - `attorney_passport_number`: [xml_target: EmpoweredPerson_Passport_IdentityCardNumber]
  - `attorney_passport_issue_date`: [xml_target: EmpoweredPerson_Passport_IdentityCardDate]
  - `attorney_passport_issued_by`: [xml_target: EmpoweredPerson_Passport_OrganizationName]
  - `issuer_name`: [xml_target: Organization_OrganizationName]
  - `issuer_inn`: [xml_target: Organization_INN]
  - `issuer_kpp`: [xml_target: Organization_KPP]
  - `issuer_ogrn`: [xml_target: Organization_OGRN]
  - `subject`: [xml_target: Subject] (ПОЛНЫЙ текст доверенности)
  - `issuer_director_surname`: [xml_target: Organization_OrganizationPerson_PersonSurname] (фамилия выдавшего руководителя)
  - `issuer_director_name`: [xml_target: Organization_OrganizationPerson_PersonName] (имя выдавшего руководителя)
  - `issuer_director_post`: [xml_target: Organization_OrganizationPerson_PersonPost] (должность, например "Директор")
  
### 6.12. EGRUL / Выписка из ЕГРЮЛ (04011)

- **xml_target_root:** `AltaFreeDoc`
- **uqi_prefix:** `formalized.egrul_[n]`
- **Поля:**
  - `number`: [xml_target: DocumentHead_DocumentNumber]
  - `date`: [xml_target: DocumentHead_DocumentDate]
  - `company_name`: [xml_target: OrganizationName]
  - `inn`: [xml_target: INN]
  - `kpp`: [xml_target: KPP]
  - `director_name`: [xml_target: PersonName]

### 6.13. Certificate of Origin / Сертификат происхождения (06011)

- **xml_target_root:** `AltaCertificateOfOrigin`
- **uqi_prefix:** `formalized.certificate_of_origin_[n]`
- **Поля:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date`: [xml_target: Registration_PrDocumentDate]
  - `origin_country`: [xml_target: OriginCountryCode]
  - `goods_description`: [xml_target: GoodsDescription]

### 6.14. Conformity Document / Декларация о соответствии EAC (01191)

- **xml_target_root:** `AltaConformityDoc`
- **uqi_prefix:** `formalized.conformity_document_[n]`
- **Поля:**
  - `number`: [xml_target: Registration_PrDocumentNumber]
  - `date_start`: [xml_target: IssueDate]
  - `date_end`: [xml_target: ExpirationDate]
  - `goods_list`: [xml_target: GoodsDescription]

### 6.15. Transit Declaration / Транзитная декларация (09013)

- **xml_target_root:** `AltaTD`
- **uqi_prefix:** `formalized.td_[n]`
- **Поля:**
  - `number`: [xml_target: TransitRegistrationNumber] (в формате XXXXXXXX/XXXXXX/XXXXXXX)
  - `date`: [xml_target: RegistrationDate]
  - `total_gross_weight`: [xml_target: TotalGrossWeight]
  - `total_places`: [xml_target: TotalPackageQuantity]
  - `seals_info`: [xml_target: SealsNumber]
  - `customs_office_code`: [xml_target: CustomsCode]
  - `destination_customs_code`: [xml_target: DestinationCustomsCode]

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

### 7.3. Transport Contract

- **uqi_prefix:** `non_formalized.transport_contract_[n]`
- **Зачем:** Данные для Графы 30.
- **Ключевые поля:**
  - `number`
  - `date`
  - `non_xml_fields`: (пока не формализовано)

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
  - `alta\source\<каталог поставки>\` (первичные документы текущей поставки)
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

Для AI **РАЗРЕШЕНА** запись в `alta\source\<каталог поставки>\operator\`. Если был диалог с оператором, в этот каталог 
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

  