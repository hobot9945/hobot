# Review Stage 1 — МоскитнаяСетка

## 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **Статус готовности:** Blocked (есть pending в формализуемых документах)
- **Всего обработано документов:** 19 (formalized + non_formalized)
- **Количество конфликтов:** 7
- **Количество недостающих данных (Pending):** 21

## 2. Сводка по документам
- [Contract 03011] — SALES CONTRACT NoLM-2553.md — OK (конфликт суммы с доп. соглашением; артефакт подписи покупателя)
- [Supplementary Contract 03012] — 1 Supplementary agreement to the contract.md — OK
- [Invoice 04021] — CL на сетку .md — OK
- [Packing List 04131] — PL на сетку .md — OK (номера ТС в кириллице, конфликт с латиницей в ДО-1/ТД)
- [CMR 02015] — СМР от СВХ.md — Внимание (pending по обязательным полям; конфликт по месту/стране принятия груза)
- [Payment order 04023] — currency_transfer_7_28.11.2025.md — Внимание (pending PayerSign)
- [Payment order 04023] — currency_transfer_1_13.01.2026.md — Внимание (pending PayerSign)
- [Service invoice 04031] — Счет_№26-00378-tl_от_27-01-2026.md — Внимание (pending подписи + индекс отправителя)
- [Insurance document 04111] — Счет_№26-00378-tl_1_от_14-01-2026.md — OK
- [TechDescription 05999] — техничка Антикот, антипыльца антимошка .md — OK
- [TechDescription 05999] — техничка .md — OK (конфликт по «ориентировочному ТН ВЭД» в тексте)
- [EGRUL 04011] — FreeDoc_ЮЭ9965-25-106893283.xml — OK
- [Transport Contract 04033] — FreeDoc_КООО_26651_М.xml — OK
- [Passport 11001] — Passport_63_09_449948.xml — OK
- [Letter of Attorney 11004] — LetterOfAttorney_1.xml — OK
- [Storage report (ДО-1)] — ДО 14431420260204161621.md — OK
- [Storage report additional sheet] — ДО доп 14431520260204161645.md — Внимание (pending дата/адрес/код таможни)
- [Transit declaration] — ТД 10719110_240126_5011363_reg00378тд.md — OK (конфликт OCR по номеру прицепа)
- [Master data] — stable_source + документы поставки — Внимание (pending e-mail декларанта/представителя)

## 3. Существенные данные первички, которые не попали в primary.md
- В инвойсе указан второй адрес Buyer (185001, Республика Карелия, Петрозаводск...). В схеме нет отдельного набора полей под альтернативный адрес, поэтому он не материализован (сохранен только как замечание).

## 4. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)

- **Конфликт #1: Сумма контракта**
  - Поле (UQI): formalized.contract_1.ContractTerms_Amount / formalized.supplementary_contract_1.ContractDescription_Amount
  - Contract: 41904.30 RMB
  - Supplementary agreement №1: 270000.00 RMB
  - **Вопрос оператору:** Подтверди, что актуальная сумма для контроля = 270000.00 RMB.

- **Конфликт #2: Подпись покупателя в контракте (артефакт распознавания)**
  - Поле (UQI): formalized.contract_1.ContractTerms_ContractText (контекст)
  - Contract: «Director ______ Дмитрий Олегович Скифов»
  - ЕГРЮЛ/доп. соглашение/доверенность: Саранов Дмитрий Олегович
  - **Вопрос оператору:** Игнорируем как артефакт?

- **Конфликт #3: Номер прицепа (OCR) в ТД vs ДО-1/PL**
  - Поле (UQI): non_formalized.td_1.transport_reg_number
  - ТД: B1734974 RU
  - ДО-1: BT374974
  - PL/operator: ВТ374974 (кириллица)
  - **Вопрос оператору:** Подтверди правильный номер прицепа.

- **Конфликт #4: Формат/алфавит номера ТС (кириллица vs латиница)**
  - Поле (UQI): formalized.packing_list_1.TransportMeans_[n].Number / formalized.cmr_1.CMRTransport_* / non_formalized.svh_1.transport_reg_number
  - PL/operator: О157АО774 / ВТ374974 (кириллица)
  - ДО-1/ТД: O157A0774 / BT374974 (латиница)
  - **Вопрос оператору:** В xml для Альты какой формат номера ТС предпочитается (кириллица или латиница)?

- **Конфликт #5: Место/страна принятия груза в CMR**
  - Поле (UQI): formalized.cmr_1.TrakingCargo_TakingCargoPlace_CountryCode
  - Решение оператора (operator_provided_data): CN
  - CMR md: место приема груза указано как СВХ ООО «ЛОГИКАМ», Набережные Челны (РФ)
  - **Вопрос оператору:** Подтверди страну принятия груза (CN или RU).

- **Конфликт #6: ТН ВЭД для «Антимошка» в техничке vs инвойс/ТД/ДО-1**
  - Документ 1 (техничка .md): «ориентировочно 6307909800 или 5903909000»
  - Документы 2 (инвойс/ТД/ДО-1): 7019900095
  - **Вопрос оператору:** Подтверди, что используем 7019900095.

- **Конфликт #7: Наименование экспедитора (опечатки)**
  - Поле (UQI): formalized.service_invoice_1.ServiceProvider_Name
  - Счет 27.01.2026: ООО «Трансмипериал»
  - Счет 14.01.2026: ООО «Транснипериал»
  - Договор (stable_source): ООО «Трансимпериал»
  - **Вопрос оператору:** Какое написание фиксируем в xml?

## 5. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)

- **Вопрос #1: Дата принятия груза в CMR**
  - Документ/Сущность: CMR
  - Поле (UQI): formalized.cmr_1.TrakingCargo_TakingCargoDate
  - **Вопрос оператору:** Укажи дату.

- **Вопрос #2–#5: Подписи плательщика (PayerSign) для платежек**
  - Поля (UQI): formalized.payment_order_1.PayerSign.PersonSurname/PersonName; formalized.payment_order_2.PayerSign.PersonSurname/PersonName
  - **Вопрос оператору:** Какие ФИО/инициалы ставим в подписи?

- **Вопрос #6–#9: Подписи в ServiceInvoice**
  - Поля (UQI): formalized.service_invoice_1.SignatureDirectorChiefAccountant_* (4 поля)
  - **Вопрос оператору:** Какие ФИО/инициалы ставим или оставляем пустыми?

- **Вопрос #10: Индекс отправителя (продавца) для ServiceInvoice**
  - Поле (UQI): formalized.service_invoice_1.Consignor_SubjectAddressDetails.PostalCode
  - **Вопрос оператору:** Оставляем пустым или есть индекс?

- **Вопрос #11–#14: Доплист ДО-1 (адрес/код СВХ)**
  - Поля (UQI): non_formalized.svh_additional_sheet_1.date, transport_reg_number, svh_address_*, svh_customs_code
  - **Вопрос оператору:** Подтверди дату, ТС и адрес/код таможни СВХ.

- **Вопрос #15–#16: e-mail**
  - Поля (UQI): non_formalized.master_data_1.declarant_email, non_formalized.master_data_1.representative_email
  - **Вопрос оператору:** Укажи e-mail (если нужен).
