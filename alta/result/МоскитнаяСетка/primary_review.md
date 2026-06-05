# Отчет по этапу 1.0: Сбор и нормализация первичных данных

### 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **Статус готовности:** Ready (Данные собраны, пропуски зафиксированы)
- **Всего обработано документов:** 13
- **Всего сформировано полей:** 1123
- **Количество конфликтов:** 0
- **Количество недостающих данных (Pending):** 19

### 2. Использованные документы

### Invoice
- formalized.invoice_1
- md\CL на сетку .md
- OK (Pending: CurrencyRate)

### Packing List
- formalized.packing_list
- md\PL на сетку .md
- OK (Pending: Consignor_ShortName)

### CMR
- formalized.cmr
- md\СМР от СВХ.md
- OK (Pending: ShortNames, Guarantee info)

### Payment Orders
- formalized.payment_order_1, formalized.payment_order_2
- md\currency_transfer_*.md
- OK

### Service Invoice
- formalized.service_invoice
- md\Счет_№26-00378-tl_от_27-01-2026.md
- OK (Pending: MiddleNames, Consignor Index)

### Tech Description
- formalized.tech_description
- md\техничка Антикот, антипыльца антимошка .md
- OK (Pending: Doc Number/Date)

### 3. Существенные данные первички, которые не попали в primary.md
- Нет.

### 4. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)
- Конфликтов не обнаружено.

### 5. Недостающие данные / Pending (ИНФОРМАЦИОННО)
Следующие данные отсутствуют в первичке и оставлены пустыми:
1. **CurrencyRate**: Курс RMB (заполняется на дату подачи ДТ).
2. **ShortNames**: Краткие наименования компаний (не критично).
3. **Guarantee info (CMR)**: Данные о гаранте в CMR отсутствуют (стандартно для автоперевозок).
4. **MiddleNames**: Отчества руководителей/бухгалтеров в счете за перевозку.
5. **Tech Doc Info**: Техническое описание не имеет внутреннего номера и даты.
