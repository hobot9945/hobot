## 1. Метаданные
- `название кейса`: МоскитнаяСетка / HEBEI LANGMAI IMPORT AND EXPORT / 02
- `путь к primary.md`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\stage_1.0_result\МоскитнаяСетка\primary.md
- `дата генерации`: 2026-05-08
- `режим`: рабочий

## 2. Проверка входных данных

### 2.1 Статус primary.md
- Есть ли pending в документах `formalized`, которые должны быть преобразованы в XML?
  - нет
- Решение:
  - ☑ Продолжено в рабочем режиме

## 3. Сформированные XML-документы

| Документ (uqi_prefix) | xml_target_root | Имя файла | Статус | Примечание |
|---|---|---|---|---|
| formalized.contract_1 | AltaE2CONT | formalized_docs\Contract_03011.xml | ✅ | ContractText: извлечен русский текст из md |
| formalized.supplementary_contract_1 | AltaSupplementaryContract | formalized_docs\SupplementaryContract_03012.xml | ✅ | ContractText: извлечен русский текст из md |
| formalized.invoice_1 | AltaE2I | formalized_docs\Invoice_04021.xml | ✅ | 7 элементов InvoiceGoods |
| formalized.packing_list_1 | AltaE2PACK | formalized_docs\PackingList_04131.xml | ✅ | 7 Goods; 2 TransportMeans |
| formalized.cmr_1 | AltaE3CMR | formalized_docs\CMR_02015.xml | ✅ | 1 CMRGoods |
| formalized.payment_order_1 | AltaPaymentOrder | formalized_docs\PaymentOrder_1_04023.xml | ✅ | - |
| formalized.payment_order_2 | AltaPaymentOrder | formalized_docs\PaymentOrder_7_04023.xml | ✅ | - |
| formalized.service_invoice_1 | AltaServiceInvoice | formalized_docs\ServiceInvoice_04031.xml | ✅ | 2 ServiceDescription |
| formalized.insurance_document_1 | AltaFreeDoc | formalized_docs\Insurance_04111.xml | ✅ | TextPara: извлечен русский текст из md |
| formalized.tech_description_1 | AltaFreeDoc | formalized_docs\TechDescription_05999.xml | ✅ | TextPara: извлечен русский текст из md (русская часть) |

## 4. Проверка структуры и переноса данных
- Валидация: синтаксически корректный XML (визуальная проверка по шаблону тегов).
- Корневые теги соответствуют `xml_target_root`.
- Даты приведены к `YYYY-MM-DD`.
- Кодировка: `windows-1251` при записи.

## 5. Работа с линками
- formalized.contract_1.ContractTerms_ContractText: link md\SALES CONTRACT NoLM-2553.md → подставлен русский текст.
- formalized.supplementary_contract_1.ContractDescription_ContractText: link md\1 Supplementary agreement... → подставлен русский текст.
- formalized.insurance_document_1.TextPara: link md\Счет_№26-00378-tl_1_от_14-01-2026.md → подставлен русский текст.
- formalized.tech_description_1.TextPara: link md\техничка... → подставлен русский текст.

## 6. Итог этапа 1.1
- ☑ Этап завершен корректно

Комментарий:
XML сформированы для всех документов раздела formalized. Известный конфликт суммы контракта (в тексте контракта 41904.30 vs решение оператора 270000.00) зафиксирован на этапе 1.0; XML использует значение из primary.md.
