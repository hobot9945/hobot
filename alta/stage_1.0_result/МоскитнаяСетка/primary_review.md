# Отчет по этапу 1.0: Сбор и нормализация первичных данных

## 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **Статус готовности:** Partial (есть pending/вопросы по Service Invoice и по полям СВХ)
- **Всего обработано документов:** 13 (md=13) + stable_source=4 xml + stable_data.md
- **Всего сформировано полей:** см. primary.md (аудиты по документам выполнены)
- **Количество конфликтов:** 1 (doc_code/doc_name в Service Invoice: несоответствие в схеме)
- **Количество недостающих данных (Pending):** несколько полей (Service Invoice signatures + ИП; СВХ транспорт_reg_number; итоги доплиста ДО)

## 2. Использованные документы

### Contract
- formalized.contract_1
- md\SALES CONTRACT NoLM-2553.md
- OK

### Supplementary Contract
- formalized.supplementary_contract_1
- md\1 Supplementary agreement to the contract.md
- OK

### Invoice
- formalized.invoice_1
- md\CL на сетку .md
- OK

### Packing List
- formalized.packing_list_1
- md\PL на сетку .md
- OK

### CMR
- formalized.cmr_1
- md\СМР от СВХ.md
- OK

### Payment Order
- formalized.payment_order_1
- md\currency_transfer_7_28.11.2025.md
- OK

### Payment Order
- formalized.payment_order_2
- md\currency_transfer_1_13.01.2026.md
- OK

### Service Invoice
- formalized.service_invoice_1
- md\Счет_№26-00378-tl_от_27-01-2026.md
- Pending

### Insurance Document
- formalized.insurance_document_1
- md\Счет_№26-00378-tl_1_от_14-01-2026.md
- OK

### TechDescription
- formalized.tech_description_1
- md\техничка Антикот, антипыльца антимошка .md
- OK

### Storage Report (ДО-1)
- non_formalized.svh_1
- md\ДО 14431420260204161621.md
- Pending (1)

### Storage Report Additional Sheet
- non_formalized.svh_additional_sheet_1
- md\ДО доп 14431520260204161645.md
- Pending (2)

### Transit Declaration
- non_formalized.td_1
- md\ТД 10719110_240126_5011363_reg00378тд.md
- OK

### Master data (stable_source)
- stable_source\FreeDoc_ЮЭ9965-25-106893283.xml (ЕГРЮЛ)
- stable_source\FreeDoc_КООО_26651_М.xml (договор ТЭО)
- stable_source\Passport_63_09_449948.xml
- stable_source\LetterOfAttorney_1.xml

## 4. Существенные данные первички, которые не попали в primary.md
- Источник: md\SALES CONTRACT NoLM-2553.md
  - Данные: банковские реквизиты сторон
  - Причина: не требуются в схемах текущих документов этапа 1.0
  - Влияние: не блокирует 1.1/2.0

- Источник: md\ТД 10719110_240126_5011363_reg00378тд.md
  - Данные: перечень документов графы 44 по ТД (09024, 09034, 09999, 11001, 11002, 11004)
  - Причина: в primary_schema.md на этапе 1.0 нет отдельного шаблона под эти документы (кроме паспорта/доверенности), а сами документы отсутствуют в источниках
  - Влияние: потребуется на этапах 2.0/2.1 для графы 44 при наличии документов

## 5. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)
- **Конфликт #1**: Service Invoice doc_code/doc_name в шаблоне
  - Поле (UQI): formalized.service_invoice_1.doc_code / doc_name
  - Шаблон primary_schema.md (Service Invoice): doc_code=04023, doc_name=ПЛАТЕЖНОЕ ПОРУЧЕНИЕ
  - По смыслу документа (счет за перевозку): ожидается Service Invoice (04031)
  - **Вопрос оператору:** какой код/имя использовать для service_invoice в графе 44 и в XML?

## 6. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)

- **Вопрос #1**: Поля ИП в Service Invoice
  - Документ/Сущность: Service Invoice
  - Поле (UQI): formalized.service_invoice_1.IndividualEntrepreneur_PersonSurname/Name/MiddleName
  - Контекст: md\Счет_№26-00378-tl_от_27-01-2026.md — счет выставлен ООО «Трансимпериал», ИП отсутствует
  - **Вопрос оператору:** оставляем пусто (pending) или заполняем "ОТСУТСТВУЕТ"?

- **Вопрос #2**: Отчества директора/бухгалтера в Service Invoice
  - Документ/Сущность: Service Invoice
  - Поле (UQI): formalized.service_invoice_1.SignatureDirectorChiefAccountant_*_PersonMiddleName
  - Контекст: md\Счет_№26-00378-tl_от_27-01-2026.md — указаны Климовин Л.А., Лехно О.А.
  - **Вопрос оператору:** оставляем pending или считаем отчество отсутствующим?

- **Вопрос #3**: Номер ТС в ДО-1 (unreliable)
  - Документ/Сущность: ДО-1
  - Поле (UQI): non_formalized.svh_1.transport_reg_number
  - Контекст: md\ДО 14431420260204161621.md, строка 15–16 (unreliable_parts)
  - **Вопрос оператору:** можно ли брать номер из CMR/ТД (0157AO774/BT374974) и в каком формате?

- **Вопрос #4**: Итоги (места/вес) для доплиста ДО
  - Документ/Сущность: Доп.лист ДО
  - Поле (UQI): non_formalized.svh_additional_sheet_1.actual_gross_weight / actual_places
  - Контекст: md\ДО доп 14431520260204161645.md — итоги не читаются
  - **Вопрос оператору:** подтверждаешь заполнение итогов из ДО-1 (127 мест, 3500 кг) со статусом CD?

- **Вопрос #5**: RMB vs CNY
  - Документ/Сущность: Общий
  - Поле (UQI): currency code mapping
  - Контекст: контракт/допсоглашение используют RMB, прочие документы — CNY
  - **Вопрос оператору:** подтверждаешь, что RMB трактуем как CNY (numeric 156) везде?
