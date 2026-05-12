# primary_review

## 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **Статус готовности:** Partial
- **Комментарий:** Вопросы по Доп. соглашению закрыты (дата 25.11.2025, сумма 270000.00 CNY извлечены из документа). Остались вопросы по сервисным счетам/СВХ.
- **Количество конфликтов:** 0

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

### Service Invoice
- formalized.service_invoice_1, formalized.service_invoice_2
- md\Счет_№26-00378-tl*.md
- Pending

### Transit Declaration
- non_formalized.transit_declaration_1
- md\ТД 10719110_240126_5011363_reg00378тд.md
- OK

## 3. Незакрытые вопросы / Pending (ТРЕБУЕТСЯ ОТВЕТ)

**Вопрос #1**: Дата транспортного заказа
- Поле (UQI): formalized.service_invoice_1.PrDocumentDate
- Контекст: Счет №26-00378-tl/1 от 14.01.2026 (в primary оставлено pending)
- Требуется: подтвердить дату транспортного заказа (если 12.01.2026 — ок; иначе дать правильную дату).

**Вопрос #2**: Валюта счета на 2700.00
- Поле (UQI): formalized.service_invoice_2.Currency
- Контекст: Счет №26-00378-tl от 27.01.2026 (сумма 2700.00)
- Требуется: указать валюту (RUB/USD/…); в md/primary может быть не зафиксировано.

**Вопрос #3**: Код ТН ВЭД для строки СВХ (ДО-1)
- Поле (UQI): non_formalized.svh_1.goods_1.tnved
- Контекст: в ДО-1 нет таблицы с разбивкой по товарам; без решения нельзя однозначно выбрать один код.
- Требуется: правило/решение (оставить pending или указать код(ы) по инвойсу).
