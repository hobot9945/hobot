# Отчет по этапу 1.1: Формализация XML

## 1. Метаданные
- `название кейса`: МоскитнаяСеткаWuqiang
- `путь к primary.md`: alta\result\МоскитнаяСеткаWuqiang\primary.md
- `дата генерации`: 2026-06-05
- `режим`: рабочий

## 2. Проверка входных данных

### 2.1 Статус primary.md
- Есть ли pending в документах `formalized`, которые должны быть преобразованы в XML?
    - Да (некритичные, согласованы с оператором):
        - `formalized.invoice_1.CurrencyRate` (курс CNY)
        - `formalized.invoice_2.CurrencyRate` (курс CNY)
    - Решение:
        - ☑ Продолжено (согласовано с оператором, Альта подберет курс автоматически при импорте)

## 3. Сформированные XML-документы

| Документ (uqi_prefix) | xml_target_root | Имя файла | Статус | Примечание |
|---|---|---|---|---|
| Invoice / formalized.invoice_1 | AltaE2I | Invoice_1_04021.xml | ✅ | Успешно создан |
| Invoice / formalized.invoice_2 | AltaE2I | Invoice_2_04021.xml | ✅ | Успешно создан |
| Packing List / formalized.packing_list | AltaE2PACK | Packing List_1_04131.xml | ✅ | Успешно создан |
| Payment Order / formalized.payment_order_1 | AltaPaymentOrder | Payment Order_1_04023.xml | ✅ | Успешно создан |
| Payment Order / formalized.payment_order_2 | AltaPaymentOrder | Payment Order_2_04023.xml | ✅ | Успешно создан |
| Service Invoice / formalized.service_invoice | AltaServiceInvoice | Service Invoice_1_04031.xml | ✅ | Успешно создан |
| Insurance Services Invoice / formalized.insurance_invoice | AltaFreeDoc | Insurance Services Invoice_1_04111.xml | ✅ | Успешно создан |

## 4. Проверка структуры и переноса данных
- Все файлы успешно прошли проверку скриптом `check_xml.bat` (статус `[OK]`).
- Кодировка файлов: `windows-1251`.
- Даты приведены к формату `YYYY-MM-DD`.
- Расхождений не обнаружено.

## 5. Работа с линками
- В данном кейсе линки `link:<путь>` в `primary.md` не использовались. Текст страховки был перенесен напрямую в `primary.md` из-за малого объема. Проблем нет.

## 6. Итог этапа 1.1
- ☑ Этап завершен корректно

Комментарий:
Все XML-документы успешно сгенерированы механическим способом, прошли валидацию структуры и готовы к импорту в Альту.
