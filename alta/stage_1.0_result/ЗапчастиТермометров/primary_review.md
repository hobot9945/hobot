# Отчет по этапу 1.0: Сбор и нормализация первичных данных

## 1. Метаданные и статус
- **Кейс:** ЗапчастиТермометров
- **Статус готовности:** Ready (все вопросы согласованы с оператором)
- **Всего обработано документов:** 15
- **Всего сформировано полей:** ~480
- **Количество конфликтов:** 0 (все разрешены)
- **Количество недостающих данных (Pending):** 0 (все согласованы)

## 2. Использованные документы

### Contract
- formalized.contract
- alta\master_data\master_proto.md
- OK

### Supplementary Contract
- formalized.supplementary_contract_1
- alta\master_data\master_proto.md
- OK

### Invoice
- formalized.invoice_1
- md\Инвойс HNKY260226 с печатью .md
- OK

### Packing List
- formalized.packing_list_1
- md\Упаковочный .md
- OK

### CMR
- formalized.cmr_1
- md\СМР.md
- OK

### Payment Order
- formalized.payment_order_1
- md\mt103_2.md
- OK

### Service Invoice
- formalized.service_invoice_1
- md\Счет_№26-09886-tl_от_12-05-2026.md
- OK

### Insurance Services Invoice
- formalized.insurance_document_1
- md\Счет_№26-09886-tl_1_от_05-05-2026.md
- OK

### Tech Description
- formalized.tech_description_1
- md\тех описание .md
- OK

### EGRUL
- master_data.egrul_1
- alta\master_data\master_proto.md
- OK

### Personal Passport
- master_data.passport_1
- alta\master_data\master_proto.md
- OK

### Letter of Attorney
- master_data.letter_of_attorney_1
- alta\master_data\master_proto.md
- OK

### Transport Contract
- master_data.transport_contract_1
- alta\master_data\master_proto.md
- OK

### Exemption Letter
- master_data.exemption_letter_1
- alta\master_data\master_proto.md
- OK

### Exemption Letter (source)
- master_data.exemption_letter_source_1
- alta\master_data\master_proto.md
- OK

## 4. Существенные данные первички, которые не попали в `primary.md`
- **Источник:** `Заявка_с_клиентом_№26-09886-tl_от_13.04.2026.md` (Транспортный заказ)
- **Данные:** Условия перевозки, фрахтовая сумма (485 USD до СВХ).
- **Причина:** Для транспортного заказа нет отдельного XML-шаблона в схеме. Данные использованы как контекст для заполнения связанных полей в счете за перевозку (`formalized.service_invoice_1`).

## 5. Выявленные конфликты (РАЗРЕШЕНЫ)

- **Конфликт #1**: Расхождение в ОГРН Покупателя (ООО «СКИФ»)
  - Поле (UQI): `formalized.invoice_1.Buyer_CompanyID` / `master_data.egrul_1.OGRN`
  - Документ 1 (`КОНТРАКТ 29.09.25.md`): `1201600020323`
  - Документ 2 (`master_proto.md`): `1201600020390`
  - **Решение:** Использовано значение из ЕГРЮЛ (`1201600020390`). Подтверждено оператором.

- **Конфликт #2**: Расхождение в номере и дате инвойса в Упаковочном листе
  - Поле (UQI): `formalized.packing_list_1.DeliveryTerms_Invoice_PrDocumentNumber` / `DeliveryTerms_Invoice_PrDocumentDate`
  - Документ 1 (`Упаковочный .md`): `HNKY260226-1` от `06.02.2026`
  - Документ 2 (`Инвойс HNKY260226 с печатью .md`): `HNKY260226` от `26.02.2026`
  - **Решение:** Реквизиты инвойса в упаковочном листе принудительно приравнены к оригинальному инвойсу (`HNKY260226` от `26.02.2026`). Подтверждено оператором.

## 6. Недостающие данные / Pending (СОГЛАСОВАНО)
- Все поля со статусом `pending` (курс валюты, коды условий EXW, отсутствующие модели и места, а также дополнительные единицы измерения товаров) по согласованию с оператором и согласно тарифу ТН ВЭД успешно разрешены (заполнены или переведены в статус CO/not_applicable).
