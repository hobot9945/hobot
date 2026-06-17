# doc_xml_review.md — Review этапа 1.1 (Формализация XML)

## 1. Метаданные

- `название кейса`: ПриточнаяВентиляция
- `путь к primary.md`: alta\result\ПриточнаяВентиляция\primary.md
- `дата генерации`: 2025-07-14
- `режим`: механика (через gen_doc_xml.bat)

## 2. Проверка входных данных

### 2.1 Статус primary.md

- Есть ли pending в документах `formalized`, которые должны быть преобразованы в XML?
    - Нет. Все поля `formalized` имеют статус `CD` или `CO`.

## 3. Сформированные XML-документы

| Документ (uqi_prefix)          | xml_target_root | Имя файла      | Статус      | Примечание   |
|--------------------------------|-----------------|----------------|-------------|--------------|
| Invoice / formalized.invoice_1 | AltaE2I         | Invoice_1_04021.xml  | ✅ |              |
| Packing List / formalized.packing_list | AltaE2PACK | Packing List_1_04131.xml | ✅ |  |
| CMR / formalized.cmr           | AltaE3CMR       | CMR_1_02015.xml | ✅ |              |
| Payment Order / formalized.payment_order_1 | AltaPaymentOrder | Payment Order_1_04023.xml | ✅ |  |
| Service Invoice / formalized.service_invoice | AltaServiceInvoice | Service Invoice_1_04031.xml | ✅ |  |
| Tech Description / formalized.tech_description | AltaFreeDoc | Tech Description_1_05999.xml | ✅ | link разрешён |

## 4. Проверка структуры и переноса данных

Все файлы:
- Корневой тег соответствует `xml_target_root`
- XML well-formed
- Даты приведены к формату `YYYY-MM-DD`
- Кодировка `windows-1251`
- Расхождений не выявлено

## 5. Работа с линками

- `<formalized.tech_description>`
    - поле: `TextPara`
    - link: `md\техничка КИВ 125.md`
    - проблема: отсутствовала
    - действие: текст подставлен из md-файла, без сокращений.

## 6. Итог этапа 1.1

- ☑ Этап завершен корректно
- ☐ Завершен в режиме отладки
- ☐ Требуется возврат к этапу 1.0

Комментарий:
Все запланированные XML-файлы успешно созданы. Ссылка в техническом описании разрешена.
