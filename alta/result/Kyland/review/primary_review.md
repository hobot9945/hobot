# primary_review: Kyland 01

### 1. Метаданные и статус
- **Кейс:** Kyland
- **Статус готовности:** Ready
- **Всего обработано документов:** 16
- **Всего сформировано полей:** ~250
- **Количество конфликтов:** 1
- **Количество недостающих данных (Pending):** 0

### 2. Использованные документы

### Master Data (Contract, UNK, EGRUL, etc.)
- master_data.contract, master_data.egrul, etc.
- source\Kyland\master_data.md
- OK

### Invoice
- formalized.invoice_1
- source\Kyland\01\md\CI, PL final_Invoice.md
- OK

### Packing List
- formalized.packing_list
- source\Kyland\01\md\CI, PL final_PackingList.md
- OK

### Air Waybill
- formalized.awb
- source\Kyland\01\md\АвиаНакладная.md
- OK

### Payment Order
- formalized.payment_order_1
- source\Kyland\01\md\ПЛАТЕЖКА.md
- OK

### Service Invoice
- formalized.service_invoice
- source\Kyland\01\md\Счет на оплату № VIG2227802 от 28.12.2022.md
- OK

### Insurance Invoice
- formalized.insurance_invoice
- source\Kyland\01\md\Счет на оплату № 27611 от 27 декабря 2022 г.md
- OK

### Tech Description
- formalized.tech_description
- source\Kyland\01\md\TechDescription.md
- OK

### 3. Существенные данные первички, которые не попали в primary.md
- Данные о размерах коробок (Measurement) из Packing List. Не предусмотрены текущим шаблоном formalized.packing_list, но зафиксированы в md-файле.

### 4. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)
- **Конфликт #1**: Расхождение в дате инвойса.
  - Поле (UQI): formalized.invoice_1.Registration_PrDocumentDate
  - Документ 1 (CI, PL final_Invoice.md): 12.12.2022
  - Документ 2 (CI, PL final_PackingList.md): 23.12.2022
  - **Решение AI:** Использована дата 12.12.2022, так как она подтверждается Платежным поручением и Счетом за перевозку.

### 5. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)
- Нет.
