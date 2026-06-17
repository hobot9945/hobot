# Итоговый отчет по этапу 1.0: Сбор и нормализация данных

### 1. Метаданные и статус
- **Кейс:** МоскитнаяСеткаWuqiang (поставка 01)
- **Статус готовности:** Ready
- **Всего обработано документов:** 13
- **Всего сформировано полей:** 241
- **Количество конфликтов:** 1 (опечатка в инвойсе, исправлено)
- **Количество недостающих данных (Pending):** 0

### 2. Использованные документы

- **Contract** (master_data.contract) — `master_data.md` | OK
- **EGRUL** (master_data.egrul) — `master_data.md` | OK
- **Personal Passport** (master_data.passport) — `master_data.md` | OK
- **Letter of Attorney** (master_data.letter_of_attorney) — `master_data.md` | OK
- **Transport Contract** (master_data.transport_contract) — `master_data.md` | OK
- **Exemption Letter** (master_data.exemption_letter) — `master_data.md` | OK
- **Exemption Letter (source)** (master_data.exemption_letter_source) — `master_data.md` | OK
- **Invoice 1** (formalized.invoice_1) — `md\CL 26HL-1103-A.md` | OK
- **Invoice 2** (formalized.invoice_2) — `md\CL 26HL-1103.md` | OK
- **Packing List** (formalized.packing_list) — `md\PL.md` | OK
- **CMR** (formalized.cmr) — `md\СМР.md` | OK
- **Payment Order 1** (formalized.payment_order_1) — `md\currency_transfer_5_03.04.2026.md` | OK
- **Payment Order 2** (formalized.payment_order_2) — `md\currency_transfer_6_08.04.2026.md` | OK
- **Service Invoice** (formalized.service_invoice) — `md\Счет_№26-09225-tl_от_12-05-2026.md` | OK
- **Insurance Invoice** (formalized.insurance_invoice) — `md\Счет_№26-09225-tl_1_от_11-05-2026.md` | OK
- **Tech Description** (formalized.tech_description) — `md\техничка Антикот, антипыльца нержавейка плесе  .md` | OK
- **Transit Declaration** (non_formalized.td) — `md\Транзитка 10719110_300526_5086483_reg.md` | OK
- **Storage Report** (non_formalized.svh) — `md\ДО_ОТЧЕТ.md` | OK
- **Storage Report Add Sheet** (non_formalized.svh_additional_sheet_1) — `md\ДО_ДОБАВОЧНЫЙ_ЛИСТ.md` | OK

### 3. Существенные данные первички, которые не попали в primary.md
- **Сводная спецификация (`md\спецификация.md`):** Не переносилась как отдельный инвойс, так как является консолидированным документом. Ее товарные позиции полностью покрываются раздельными инвойсами `CL 26HL-1103-A` (позиция 14 спецификации) и `CL 26HL-1103` (позиции 1–13 спецификации). Использовалась как вспомогательный источник для сверки весов и объемов.

### 4. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)
- **Конфликт #1:** В табличной части Инвойса 2 (`CL 26HL-1103.md`) в позициях 12 и 13 в графе «Price per roll» ошибочно указано значение `163.2` CNY за рулон. При этом итоговая стоимость строк (`1073.86` CNY за 3 рулона и `4594.58` CNY за 10 рулонов соответственно) рассчитана верно по цене за м2. Для устранения математических расхождений в полях `Price` базы `primary.md` были сохранены расчетные верные цены за рулон: `357.95` CNY (для поз. 12) и `459.46` CNY (для поз. 13).

### 5. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)
- **Нет.** Все вопросы по недостающим данным были заранее закрыты регламентом кросс-дока и файлом ответов оператора (`operator_provided_data.md`):
  - УНК подтвержден как отсутствующий (doc_number: ОТСУТСТВУЕТ, status: CO).
  - Номер упаковочного листа подтвержден как б/н (doc_number: Б/Н, status: CO).
  - Условия поставки для CMR кросс-докнуты из инвойса (EXW HEBEI, status: CD).
