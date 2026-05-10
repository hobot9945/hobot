# primary_review.md

### 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка / HEBEI LANGMAI IMPORT AND EXPORT / 02
- **Статус готовности:** Ready (для этапа 1.1)
- **Всего обработано документов:** 13 (formalized=10, non_formalized=3)
- **Всего сформировано полей:** 288 (без учета полей массивов)
- **Количество конфликтов:** 1
- **Количество недостающих данных (Pending):** 1

### 2. Использованные документы

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

### Payment Order #1
- formalized.payment_order_1
- md\currency_transfer_1_13.01.2026.md
- OK

### Payment Order #7
- formalized.payment_order_2
- md\currency_transfer_7_28.11.2025.md
- OK

### Service Invoice
- formalized.service_invoice_1
- md\Счет_№26-00378-tl_от_27-01-2026.md
- OK

### Insurance Document
- formalized.insurance_document_1
- md\Счет_№26-00378-tl_1_от_14-01-2026.md
- OK

### Tech Description
- formalized.tech_description_1
- md\техничка Антикот, антипыльца антимошка .md
- OK

### SVH (Storage Report ДО-1)
- non_formalized.svh_1
- md\ДО 14431420260204161621.md
- OK

### SVH Additional Sheet
- non_formalized.svh_additional_sheet_1
- md\ДО доп 14431520260204161645.md
- Pending (1)

### Transit Declaration
- non_formalized.td_1
- md\ТД 10719110_240126_5011363_reg00378тд.md
- OK

### 4. Существенные данные первички, которые не попали в `primary.md`
- Источник: md\SALES CONTRACT NoLM-2553.md (Page 4)
  - Данные: адрес/реквизиты продавца читаются с confidence=mid
  - Причина: OCR mid, требуется подтверждение при необходимости
  - Влияние: не блокирует этап 1.1, но может потребоваться для сверок/печати

### 5. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)
- **Конфликт #1**: Сумма контракта в тексте контракта vs доп. соглашение/решение оператора
  - Поле (UQI): formalized.contract_1.ContractTerms_Amount
  - Документ 1 (md\SALES CONTRACT NoLM-2553.md, Page 1): 41904.30 CNY
  - Документ 2 (md\1 Supplementary agreement to the contract.md, Page 1): 270000.00 CNY
  - Решение: использовать 270000.00 (источник operator_provided_data.md: formalized.contract_1.ContractTerms_Amount, status=CO)

### 6. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)
- **Вопрос #1**: Номер ТС при въезде для добавочного листа ДО
  - Документ/Сущность: SVH Additional Sheet
  - Поле (UQI): non_formalized.svh_additional_sheet_1.transport_reg_number
  - Контекст: md\ДО доп 14431520260204161645.md — поле отсутствует; в ДО-1/CMR фигурирует тягач О157АО774
  - **Вопрос оператору:** подтверждаешь, что для этого поля использовать тягач **О157АО774**?
