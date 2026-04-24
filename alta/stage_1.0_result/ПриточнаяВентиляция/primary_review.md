# review_1 — ПриточнаяВентиляция (этап 1)

## 1. Метаданные и статус
- **Кейс:** ПриточнаяВентиляция
- **Статус готовности:** **Partial** (есть pending-поля в формализуемых документах)
- **Всего обработано документов:** 12
- **Количество конфликтов:** 1
- **Количество недостающих данных (Pending):** 15 (оценка по primary.md)

---

## 2. Сводка по документам

Формализуемые:
- [Contract 03011] — `1 Supplementary agreement to the _25AZC003.pdf` — **Внимание** (pending: валюта numeric / уточнения)
- [Invoice 04021] — `1\Инвойс 25AZC003B.pdf` — **Внимание** (pending: CurrencyRate, EXW numeric code, единица, места)
- [Packing List 04131] — `1\PL 25AZC003B.pdf` — **Внимание** (pending: short_name сторон, EXW numeric code, транспортный блок)
- [CMR 02015] — `1\СМР.pdf` — **Внимание** (pending: место составления, гарант отправителя)
- [Payment Order 04023] — `1\Платежка.pdf` — **Внимание** (pending: PaymentModeCode, TransactionKind, Priority)
- [Service Invoice 04031] — `Счет_№25-12327-k_от_22-05-2025 (2).pdf` — **Внимание** (pending: Signature-блок, consignor-блок)
- [Service Invoice 04031] — `Счет_№25-12327-k_1_от_22-05-2025 (3).pdf` — **Внимание** (pending: Signature-блок, consignor-блок)
- [TechDescription 05999] — `1\тех описание\техничка КИВ 125.pdf` — **Внимание** (текст страницы 2 не распознан, в XML вставлен краткий текст)

Мастер-данные / стабильные документы (использованы как эталонные реквизиты):
- [Transport contract 04033] — `alta\stable_source\FreeDoc_КООО_26651_М.xml` — OK
- [EGRUL 04011] — `alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml` — OK
- [Passport 11001] — `alta\stable_source\Passport_63_09_449948.xml` — OK
- [Letter of attorney 11004] — `alta\stable_source\LetterOfAttorney_1.xml` — OK

Неформализуемые (для ДТ):
- [СВХ ДО-1] — `1\СВХ\до.pdf` — OK (best-effort распознавание)
- [Уведомление на регистрацию документов] — `1\СВХ\ВТамПостНабережныхЧелнов.pdf` — **Внимание** (часть реквизитов/полей отсутствует в документе)
- [Транзитная декларация] — `1\документы от Любы\ТД_12327.pdf` — **Внимание** (конфликт в номере прицепа, нечеткое имя поста)

---

## 3. Существенные данные первички, которые не попали в primary.md

1) **Полный текст контракта и техописания** в primary.md хранится ссылкой (`link:...`) для компактности.
2) **Техописание, страница 2**: распознавание текста не получено (в md отмечено как изображения). В XML `TechDescription_KIV-125.xml` вставлен краткий текст по странице 1; при необходимости для Альты/графы 31 нужно вручную/повторно извлечь содержимое страницы 2.

---

## 4. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)

- **Конфликт #1: номер прицепа (латиница I vs цифра 1)**
  - Поле (UQI): `non_formalized.td_1.transport_reg_number`
  - Документ 1 (`Transit declaration 10719110-060725-5070039 (parsed).md`): `A6726I5` (как распознано)
  - Документ 2 (`CMR_12327 (parsed from screenshots).md` / `SVH_DO-1_Report_0000478_14.07.2025 (parsed).md`): `А67261-5`
  - **Вопрос оператору:** Какое написание номера прицепа использовать как основное?

---

## 5. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)

- **Вопрос #1: numeric код валюты контракта (RMB)**
  - Документ/Сущность: Contract (03011)
  - Поле (UQI): `formalized.contract_1.ContractTerms_CurrencyCode`
  - **Вопрос оператору:** В контракте валюта указана как RMB. Подтверди, что ставим ISO 4217 numeric **156 (CNY)**, или нужен другой код?

- **Вопрос #2: курс валюты**
  - Документ/Сущность: Invoice (04021)
  - Поле (UQI): `formalized.invoice_1.CurrencyRate`
  - **Вопрос оператору:** Какой курс CNY использовать (и нужен ли он в импорте инвойса)?

- **Вопрос #3: внутренний numeric-код Альты для EXW**
  - Документ/Сущность: Invoice / PackingList
  - Поля (UQI): `formalized.invoice_1.DeliveryTerms_DeliveryTermsNumericCode`, `formalized.packing_list_1.DeliveryTerms_DeliveryTermsNumericCode`
  - **Вопрос оператору:** Какой numeric-код в Альте соответствует Incoterms **EXW**?

- **Вопрос #4: единица измерения товара**
  - Документ/Сущность: InvoiceGoods
  - Поле (UQI): `formalized.invoice_1.InvoiceGoods_1.MeasureUnitQualifierName`
  - **Вопрос оператору:** В инвойсе количество указано как `Q-ty, Sets`. Для Альты/ДТ в `MeasureUnitQualifierName` ставим **"шт"** или другое (например, "набор")?

- **Вопрос #5: описание грузовых мест**
  - Документ/Сущность: Invoice
  - Поле (UQI): `formalized.invoice_1.PlacesDescription`
  - **Вопрос оператору:** Что означает `2 cl` (cartons?) и как правильно заполнить `PlacesDescription`?

- **Вопрос #6: платежные системные коды**
  - Документ/Сущность: Payment Order
  - Поля (UQI): `formalized.payment_order_1.PaymentModeCode`, `formalized.payment_order_1.TransactionKind`, `formalized.payment_order_1.Priority`
  - **Вопрос оператору:** Есть ли у вас эталон/правила заполнения этих полей для заявления на перевод (100% TT in advance)? Если нет — можно ли оставлять пустыми при импорте?

- **Вопрос #7: CMR место составления**
  - Документ/Сущность: CMR
  - Поле (UQI): `formalized.cmr_1.RegistrationDocument_Place`
  - **Вопрос оператору:** Нужно ли заполнять место составления CMR для импорта? Если да — какое значение?

- **Вопрос #8: CMR гарант отправителя**
  - Документ/Сущность: CMR
  - Поля (UQI): `formalized.cmr_1.Consignor_Guarantee_*`
  - **Вопрос оператору:** В СМР есть блок гаранта отправителя? Если есть — нужны реквизиты, иначе можно оставить пустыми.

- **Вопрос #9: подписи в ServiceInvoice (Трансимпериал)**
  - Документ/Сущность: ServiceInvoice
  - Поля (UQI): `formalized.service_invoice_1.Signature_*`, `formalized.service_invoice_2.Signature_*`
  - **Вопрос оператору:** Требует ли Альта заполнения Signature-блока в `AltaServiceInvoice`? Если да — откуда брать ФИО директора/бухгалтера?
