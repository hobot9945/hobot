# Отчет по этапу 1.0 — Сбор и нормализация первичных данных

### 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **Статус готовности:** Ready
- **Всего обработано документов:** 11
- **Всего сформировано полей:** 263
- **Количество конфликтов:** 0
- **Количество недостающих данных (Pending):** 0

### 2. Использованные документы

### Contract
- formalized.contract_1
- md\\SALES CONTRACT NoLM-2553.md
- OK

### Supplementary Contract
- formalized.supplementary_contract_1
- md\\1 Supplementary agreement to the contract.md
- OK

### Invoice
- formalized.invoice_1
- md\\CL на сетку.md
- OK

### Packing List
- formalized.packing_list_1
- md\\PL на сетку.md
- OK

### CMR
- formalized.cmr_1
- md\\СМР от СВХ.md
- OK

### Payment Order
- formalized.payment_order_1
- md\\currency_transfer_7_28.11.2025.md
- OK

### Payment Order
- formalized.payment_order_2
- md\\currency_transfer_1_13.01.2026.md
- OK

### Service Invoice
- formalized.service_invoice_1
- md\\Счет_№26-00378-tl_от_27-01-2026.md
- OK

### Insurance Document
- formalized.insurance_document_1
- md\\Счет_№26-00378-tl_1_от_14-01-2026.md
- OK

### TechDescription
- formalized.tech_description_1
- md\\техничка Антикот, антипыльца антимошка.md
- OK

### Storage Report (SVH)
- non_formalized.svh_1
- md\\ДО 14431420260204161621.md
- OK

### Storage Report Additional Sheet
- non_formalized.svh_additional_sheet_1
- md\\ДО доп 14431520260204161645.md
- OK

### Transit Declaration
- non_formalized.td_1
- md\\ТД 10719110_240126_5011363_reg00378тд.md
- OK

### Master data
- non_formalized.master_data_1
- alta\\stable_source\\FreeDoc_ЮЭ9965-25-106893283.xml
- alta\\stable_source\\LetterOfAttorney_1.xml
- alta\\stable_source\\Passport_63_09_449948.xml
- OK

### 4. Существенные данные первички, которые не попали в `primary.md`
- Источник: md\\SALES CONTRACT NoLM-2553.md
  - Данные: реквизиты банков (счета/свифт) продавца/покупателя
  - Причина: в primary_schema для Contract нет отдельных полей под банковские реквизиты
  - Влияние: не блокирует этап 1.1/2.0

- Источник: md\\СМР от СВХ.md
  - Данные: рукописные пометы (строки 111–113) помечены как [[неразборчиво]]
  - Причина: не читается надежно
  - Влияние: не блокирует этап 1.1

### 5. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)
Не выявлены.

### 6. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)
Отсутствуют.
