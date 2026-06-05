# Отчет по этапу 1.0: Сбор и нормализация первичных данных

## 1. Метаданные и статус
- **Кейс:** `МоскитнаяСеткаWuqiang`
- **Статус готовности:** `Ready` (оставшиеся pending заполняются Альтой автоматически)
- **Всего обработано документов:** `11`
- **Всего сформировано полей:** `648`
- **Количество конфликтов:** `0`
- **Количество недостающих данных (Pending):** `2`

## 2. Использованные документы

### Contract
- master_data.contract
- md\КОНТРАКТ СЕТКА МОСКИТНАЯ Wuqiang 31.03.26.md
- OK

### EGRUL
- master_data.egrul
- alta\master_data.md
- OK

### Personal Passport
- master_data.passport
- alta\master_data.md
- OK

### Letter of Attorney
- master_data.letter_of_attorney
- alta\master_data.md
- OK

### Transport Contract
- master_data.transport_contract
- alta\master_data.md
- OK

### Exemption Letter
- master_data.exemption_letter
- alta\master_data.md
- OK

### Exemption Letter (source)
- master_data.exemption_letter_source
- alta\master_data.md
- OK

### Invoice 1
- formalized.invoice_1
- md\CL 26HL-1103 .md
- Pending (1)

### Invoice 2
- formalized.invoice_2
- md\CL 26HL-1103-A.md
- Pending (1)

### Packing List
- formalized.packing_list
- md\PL.md
- OK

### Payment Order 1
- formalized.payment_order_1
- md\currency_transfer_5_03.04.2026.md
- OK

### Payment Order 2
- formalized.payment_order_2
- md\currency_transfer_6_08.04.2026.md
- OK

### Service Invoice
- formalized.service_invoice
- md\Счет_№26-09225-tl_от_12-05-2026.md
- OK

### Insurance Services Invoice
- formalized.insurance_invoice
- md\Счет_№26-09225-tl_1_от_11-05-2026.md
- OK

### Transit Declaration
- non_formalized.td
- md\Транзитка 10719110_300526_5086483_reg.md
- OK

## 3. Существенные данные первички, которые не попали в `primary.md`
- Нет.

## 4. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)
- Нет.

## 5. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)

- **Вопрос #1**: Отсутствует курс валюты CNY для Invoice 1.
  - Документ/Сущность: `Invoice 1`
  - Поле (UQI): `formalized.invoice_1.CurrencyRate`
  - Контекст: `md\CL 26HL-1103 .md`
  - **Решение оператора:** Оставить pending, Альта подберет курс автоматически при импорте.

- **Вопрос #2**: Отсутствует курс валюты CNY для Invoice 2.
  - Документ/Сущность: `Invoice 2`
  - Поле (UQI): `formalized.invoice_2.CurrencyRate`
  - Контекст: `md\CL 26HL-1103-A.md`
  - **Решение оператора:** Оставить pending, Альта подберет курс автоматически при импорте.
