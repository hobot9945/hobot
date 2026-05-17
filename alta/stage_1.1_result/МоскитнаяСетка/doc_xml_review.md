# doc_xml_review.md — Review этапа 1.1 (Формализация XML)

## 1. Метаданные

- `название кейса`: МоскитнаяСетка
- `путь к primary.md`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\stage_1.0_result\МоскитнаяСетка\primary.md
- `дата генерации`: 2026-05-16
- `режим`: рабочий

## 2. Проверка входных данных

### 2.1 Статус primary.md

- `status: pending` в таблицах полей `formalized` документов: **не обнаружено** (по поиску `| pending |`).
- `doc_formalization_status: pending` встречается у многих документов и трактуется как «не сформировано/не выгружено в XML», а не как неполнота фактов.

## 3. Сформированные XML-документы

Файлы сохранены в:
`alta\stage_1.1_result\МоскитнаяСетка\formalized_docs\`

| Документ (uqi_prefix) | xml_target_root | Имя файла | Статус | Примечание |
|---|---|---|---|---|
| formalized.contract_1 | AltaE2CONT | Contract_LM-2553.xml | ✅ | Текст контракта подставлен из md, оставлен русский вариант |
| formalized.supplementary_contract_1 | AltaSupplementaryContract | SupplementaryContract_1.xml | ✅ | Текст доп. соглашения подставлен из md, русский вариант |
| formalized.invoice_1 | AltaE2I | Invoice_LM-2591.xml | ✅ | InvoiceGoods (7 элементов) |
| formalized.packing_list_1 | AltaE2PACK | PackingList_LM-2591.xml | ✅ | Goods (7), TransportMeans (2) |
| formalized.cmr_1 | AltaE3CMR | CMR_00378.xml | ✅ | CMRGoods (1); PakingQuantity помещен в GoodsPackingInfo по schema |
| formalized.payment_order_1 | AltaPaymentOrder | PaymentOrder_7_2025-11-28.xml | ✅ | PayerSign вложенный блок |
| formalized.payment_order_2 | AltaPaymentOrder | PaymentOrder_1_2026-01-13.xml | ✅ | PayerSign вложенный блок |
| formalized.service_invoice_1 | AltaServiceInvoice | ServiceInvoice_26-00378-tl.xml | ✅ | ServiceDescription (2) |
| formalized.insurance_document_1 | AltaFreeDoc | Insurance_26-00378-tl_1_2026-01-14.xml | ✅ | TextPara подставлен из md |
| formalized.tech_description_1 | AltaFreeDoc | TechDescription_05999_2025-10-30.xml | ✅ | TextPara подставлен из md |
| formalized.transport_contract_1 | AltaFreeDoc | TransportContract_КООО_26651_М_2025-05-13.xml | ✅ | TextPara подставлен из stable_source xml |
| formalized.egrul_1 | AltaFreeDoc | EGRUL_ЮЭ9965-25-106893283_2025-07-14.xml | ✅ | TextPara подставлен из stable_source xml |
| formalized.passport_1 | AltaPassport | Passport_63_09_449948.xml | ✅ |  |
| formalized.letter_of_attorney_1 | AltaLetterOfAttorney | LetterOfAttorney_1.xml | ✅ | Subject подставлен из stable_source xml |

## 4. Проверка структуры и переноса данных

- Корневые теги файлов соответствуют `xml_target_root`.
- `link:` в сгенерированных XML не оставлено.
- Даты приведены к `YYYY-MM-DD` для полей, которые в primary.md были в формате `DD.MM.YYYY`.
- Кодировка записи: `windows-1251`.

## 5. Работа с линками

Использованные `link:` и их разрешение:
- `link:md\\SALES CONTRACT NoLM-2553.md` → вставлен русский текст из `## Текст документа`.
- `link:md\\1 Supplementary agreement to the contract.md` → вставлен русский текст.
- `link:md\\Счет_№26-00378-tl_1_от_14-01-2026.md` → вставлен текст счета.
- `link:md\\техничка Антикот, антипыльца антимошка .md` → вставлен технический текст.
- `link:stable_source\\FreeDoc_КООО_26651_М.xml` → вставлен TextPara (с `&#13;&#10;`).
- `link:stable_source\\FreeDoc_ЮЭ9965-25-106893283.xml` → вставлен TextPara (с `&#13;&#10;`).
- `link:stable_source\\LetterOfAttorney_1.xml` → вставлен Subject (с `&#13;&#10;`).

## 6. Итог этапа 1.1

- ☑ Этап завершен корректно
- ☐ Завершен в режиме отладки
- ☐ Требуется возврат к этапу 1.0

Комментарий:
XML-файлы сформированы для всех документов раздела `formalized`.
