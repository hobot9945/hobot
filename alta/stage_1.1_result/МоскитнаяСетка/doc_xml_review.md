# doc_xml_review.md — МоскитнаяСетка (Stage 1.1)

## 1. Метаданные
- `название кейса`: МоскитнаяСетка
- `путь к primary.md`: alta\stage_1.0_result\МоскитнаяСетка\primary.md
- `дата генерации`: 2026-04-30
- `режим`: рабочий

## 2. Проверка входных данных

### 2.1 Статус primary.md
- Pending в документах `formalized`, необходимых для генерации XML: нет
- Решение:
  - ☑ Продолжено в рабочем режиме

## 3. Сформированные XML-документы

| Документ (uqi_prefix) | xml_target_root | Имя файла | Статус | Примечание |
|---|---|---|---|---|
| Contract / formalized.contract_1 | AltaE2CONT | Contract_LM-2553.xml | ✅ | |
| SupplementaryContract / formalized.supplementary_contract_1 | AltaSupplementaryContract | SupplementaryContract_1_LM-2553.xml | ✅ | |
| Invoice / formalized.invoice_1 | AltaE2I | Invoice_LM-2591.xml | ✅ | |
| PackingList / formalized.packing_list_1 | AltaE2PACK | PackingList_LM-2591.xml | ✅ | |
| CMR / formalized.cmr_1 | AltaE3CMR | CMR_00378.xml | ✅ | |
| PaymentOrder / formalized.payment_order_1 | AltaPaymentOrder | PaymentOrder_7_2025-11-28.xml | ✅ | |
| PaymentOrder / formalized.payment_order_2 | AltaPaymentOrder | PaymentOrder_1_2026-01-13.xml | ✅ | |
| ServiceInvoice / formalized.service_invoice_1 | AltaServiceInvoice | ServiceInvoice_26-00378-tl_2026-01-27.xml | ✅ | Consignor_* заполнен решением оператора (=seller) |
| InsuranceDocument / formalized.insurance_document_1 | AltaFreeDoc | Insurance_26-00378-tl_1_2026-01-14.xml | ✅ | TextPara подставлен из link |
| TechDescription / formalized.tech_description_1 | AltaFreeDoc | TechDescription_2025-10-30_BN.xml | ✅ | TextPara подставлен из link |

## 4. Проверка структуры и переноса данных
- Корневые теги соответствуют `xml_target_root` из `primary.md` (выборочно проверено).
- Кодировка: `windows-1251` (выборочно проверено чтением файлов).
- Экранирование спецсимволов: проверено наличие `&amp;` в CMR; неэкранированных `&` в проверенном фрагменте нет.

## 5. Работа с линками
Использованы линк-поля:
- `formalized.contract_1.ContractTerms_ContractText` → `md\SALES CONTRACT NoLM-2553.md`
- `formalized.supplementary_contract_1.ContractDescription_ContractText` → `md\1 Supplementary agreement to the contract.md`
- `formalized.insurance_document_1.TextPara_[n]` → `md\Счет_№26-00378-tl_1_от_14-01-2026.md`
- `formalized.tech_description_1.TextPara_[n]` → `md\техничка Антикот, антипыльца антимошка.md`
Проблем не возникло.

## 6. Итог этапа 1.1
- ☑ Этап завершен корректно

Комментарий:
XML-файлы для всех документов `formalized` сформированы и записаны в `stage_1.1_result\...\formalized_docs\`.
