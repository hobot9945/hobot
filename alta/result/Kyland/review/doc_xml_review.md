# doc_xml_review.md — Review этапа 1.1 (Формализация XML)

## 1. Метаданные

- `название кейса`: Kyland
- `путь к primary.md`: alta\result\Kyland\primary.md
- `дата генерации`: 2026-07-01
- `режим`: рабочий (механика с доработкой линков)

---

## 2. Проверка входных данных

### 2.1 Статус primary.md

- Есть ли pending в документах `formalized`, которые должны быть преобразованы в XML?
    - Нет, проверка `check_pendings.bat` завершилась статусом: `[OK] Проверка пройдена. Полей со статусом 'pending' не обнаружено.`

- Решение:
    - ☒ Этап продолжен в рабочем режиме

---

## 3. Сформированные XML-документы

| Документ (uqi_prefix) | xml_target_root | Имя файла | Статус | Примечание |
|---|---|---|---|---|
| Invoice / formalized.invoice_1 | AltaE2I | Invoice_1_04021.xml | ✅ успешно создан | Добавлено недостающее поле кода условий EXW |
| Packing List / formalized.packing_list | AltaE2PACK | Packing List_1_04131.xml | ✅ успешно создан | |
| Air Waybill / formalized.awb | AltaE3AWB | Air Waybill_1_02017.xml | ✅ успешно создан | |
| Payment Order / formalized.payment_order_1 | AltaPaymentOrder | Payment Order_1_04023.xml | ✅ успешно создан | |
| Service Invoice / formalized.service_invoice | AltaServiceInvoice | Service Invoice_1_04031.xml | ✅ успешно создан | |
| Tech Description / formalized.tech_description | AltaFreeDoc | Tech Description_1_05999.xml | ✅ успешно создан | Линки разрешены и подставлены |
| Insurance Invoice / formalized.insurance_invoice | AltaFreeDoc | Insurance Invoice_1_04111.xml | ✅ успешно создан | Линки разрешены и подставлены |

---

## 4. Проверка структуры и переноса данных

- Все сгенерированные и дополненные файлы проверены утилитой `check_xml.bat`.
- Статус валидации для всех файлов: `[OK]`.
- Изменения в `primary.md`:
    - В раздел `formalized.invoice_1` добавлено поле `DeliveryTerms_DeliveryTermsNumericCode` (код `55`, значение `01`), так как генератор выдавал ошибку отсутствия обязательного поля.

---

## 5. Работа с линками

- `Tech Description_1_05999.xml`
    - Поле: `TextPara`
    - Link: `alta\source\Kyland\01\md\TechDescription.md`
    - Действие: Текст подставлен полностью, разметка маркдауна очищена.

- `Insurance Invoice_1_04111.xml`
    - Поле: `TextPara`
    - Link: `alta\source\Kyland\01\md\Счет на оплату № 27611 от 27 декабря 2022 г.md`
    - Действие: Текст подставлен полностью, маркдаун таблиц преобразован в читаемый текст без спецсимволов разметки, кавычки экранированы (`&quot;`).

---

## 6. Итог этапа 1.1

- ☒ Этап завершен корректно

Комментарий:
Все XML-файлы для импорта успешно сформированы, линки разрешены, валидация пройдена.
