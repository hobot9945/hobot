# Отчет по этапу 1.0: Сбор и нормализация первичных данных

## 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **Статус готовности:** Ready
- **Всего обработано документов:** 14 (md) + 5 (stable_source)
- **Всего сформировано полей:** 441 (total_fields)
- **Количество конфликтов:** 2
- **Количество недостающих данных (Pending):** 0

## 2. Использованные документы

### Contract
- formalized.contract_1
- md/SALES CONTRACT NoLM-2553.md
- OK

### Supplementary Contract
- formalized.supplementary_contract_1
- md/1 Supplementary agreement to the contract.md
- OK

### Invoice
- formalized.invoice_1
- md/CL на сетку .md
- OK

### Packing List
- formalized.packing_list_1
- md/PL на сетку .md
- OK

### CMR
- formalized.cmr_1
- md/СМР от СВХ.md
- OK

### Payment Order
- formalized.payment_order_1
- md/currency_transfer_7_28.11.2025.md
- OK

### Payment Order
- formalized.payment_order_2
- md/currency_transfer_1_13.01.2026.md
- OK

### Service Invoice
- formalized.service_invoice_1
- md/Счет_№26-00378-tl_от_27-01-2026.md
- OK

### Insurance Document
- formalized.insurance_document_1
- md/Счет_№26-00378-tl_1_от_14-01-2026.md
- OK

### TechDescription
- formalized.tech_description_1
- md/техничка Антикот, антипыльца антимошка .md
- OK

### Storage Report
- non_formalized.svh_1
- md/ДО 14431420260204161621.md
- OK

### Storage Report Additional Sheet
- non_formalized.svh_additional_sheet_1
- md/ДО доп 14431520260204161645.md
- OK

### Transit Declaration
- non_formalized.td_1
- md/ТД 10719110_240126_5011363_reg00378тд.md
- OK

### Stable Data
- non_formalized.stable_data_1
- stable_source/stable_data.md
- OK

### Master data (stable_source)
- stable_source/FreeDoc_ЮЭ9965-25-106893283.xml (ЕГРЮЛ)
- stable_source/LetterOfAttorney_1.xml (доверенность)
- stable_source/Passport_63_09_449948.xml (паспорт)
- stable_source/FreeDoc_КООО_26651_М.xml (договор экспедиции)

## 4. Существенные данные первички, которые не попали в primary.md
- Нет.

## 5. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)

- **Конфликт #1**: Сумма контракта. В контракте SALES CONTRACT NoLM-2553 (стр. 1) общая сумма составляет 41904.30 RMB. В дополнительном соглашении №1 сумма была заменена на 270 000.00 CNY.
  - Поле (UQI): formalized.contract_1.ContractTerms_Amount
  - Решение: Использовать уточненную сумму 270 000.00 CNY из доп. соглашения №1 (CO).

- **Конфликт #2**: Валюта контракта/инвойса. В контракте используется валюта китайский юань (ISO CNY, цифровой код 156), но в некоторых оригинальных банковских документах упоминалась валюта RMB.
  - Поле (UQI): formalized.contract_1.ContractTerms_CurrencyCode
  - Решение: Использовать официальный цифровой код валюты CNY = 156 (CO).

## 6. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)
- Нет. Все вопросы успешно закрыты на основе файла решений оператора `operator_provided_data.md`.
