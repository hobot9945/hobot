# primary_review — МоскитнаяСетка / этап 1.0

## 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **generated_at:** 2026-04-29T17:04:48.2953790Z
- **Статус готовности:** Partial (есть pending в non_formalized)
- **Всего обработано документов первички (md):** 14
- **Всего использовано stable_source:** 4
- **Файл оператора:** operator_provided_data.md (использован)

## 2. Сводка по документам
### formalized
- Contract / formalized.contract_1 — SALES CONTRACT NoLM-2553.md — OK
- SupplementaryContract / formalized.supplementary_contract_1 — 1 Supplementary agreement to the contract.md — OK
- Invoice / formalized.invoice_1 — CL на сетку.md — OK
- PackingList / formalized.packing_list_1 — PL на сетку.md — OK
- CMR / formalized.cmr_1 — СМР от СВХ.md — OK
- PaymentOrder / formalized.payment_order_1 — currency_transfer_7_28.11.2025.md — OK
- PaymentOrder / formalized.payment_order_2 — currency_transfer_1_13.01.2026.md — OK
- ServiceInvoice / formalized.service_invoice_1 — Счет_№26-00378-tl_от_27-01-2026.md — OK
- InsuranceDocument / formalized.insurance_document_1 — Счет_№26-00378-tl_1_от_14-01-2026.md — OK
- TechDescription / formalized.tech_description_1 — техничка Антикот, антипыльца антимошка.md — OK

### non_formalized
- SVH / non_formalized.svh_1 — ДО 14431420260204161621.md — OK
- SVH_AdditionalSheet / non_formalized.svh_additional_sheet_1 — ДО доп 14431520260204161645.md — Внимание (pending)
- MasterData / non_formalized.master_data_1 — stable_source xml — OK
- TransitDeclaration / non_formalized.td_1 — ТД 10719110_240126_5011363_reg00378тд.md — Внимание (pending)

## 3. Список использованных источников
- alta\source\МоскитнаяСетка\...\02\operator\operator_provided_data.md — решения оператора (курс, delivery terms, веса по строкам, defaults)
- alta\source\МоскитнаяСетка\...\02\md\SALES CONTRACT NoLM-2553.md — контракт №/дата/сумма/срок/стороны
- alta\source\МоскитнаяСетка\...\02\md\1 Supplementary agreement to the contract.md — сумма по доп. соглашению 270000.00 CNY
- alta\source\МоскитнаяСетка\...\02\md\CL на сетку.md — инвойс LM-2591 (7 строк, суммы, qty, TN VED, EXW)
- alta\source\МоскитнаяСетка\...\02\md\PL на сетку.md — packing list (totals net/gross, веса по строкам, ТС)
- alta\source\МоскитнаяСетка\...\02\md\СМР от СВХ.md — CMR №00378 (места 127, вес 3500, ТС)
- alta\source\МоскитнаяСетка\...\02\md\ДО 14431420260204161621.md — СВХ ДО-1 №0000080 (разбивка по ТНВЭД)
- alta\source\МоскитнаяСетка\...\02\md\ДО доп 14431520260204161645.md — добавочный лист (итоги)
- alta\source\МоскитнаяСетка\...\02\md\ТД 10719110_240126_5011363_reg00378тд.md — транзитная декларация (номер, дата, ТС)
- alta\source\МоскитнаяСетка\...\02\md\currency_transfer_7_28.11.2025.md — платеж №7
- alta\source\МоскитнаяСетка\...\02\md\currency_transfer_1_13.01.2026.md — платеж №1
- alta\source\МоскитнаяСетка\...\02\md\Счет_№26-00378-tl_от_27-01-2026.md — услуги/перевозка
- alta\source\МоскитнаяСетка\...\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md — страхование
- alta\source\МоскитнаяСетка\...\02\md\техничка Антикот, антипыльца антимошка.md — техописание
- alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml — ЕГРЮЛ (реквизиты)
- alta\stable_source\LetterOfAttorney_1.xml — доверенность
- alta\stable_source\Passport_63_09_449948.xml — паспорт
- alta\stable_source\FreeDoc_КООО_26651_М.xml — договор ТЭО

## 4. Существенные данные первички, которые не попали в primary.yaml
- В ДО-1 md есть фрагмент с [[неразборчиво]] в строке про инвойс, но инвойс прочитан из CL/PL, поэтому этот кусок не использовался как источник.

## 5. Выявленные конфликты (требуется решение)
- **Конфликт #1: сумма контракта (контроль/сверка)**
  - formalized.contract_1.ContractTerms_Amount = 41904.30 (контракт)
  - formalized.supplementary_contract_1.ContractDescription_Amount = 270000.00 (доп. соглашение №1)
  - Примечание: обе суммы сохранены как факты разных документов; для этапов 1.1/2.0 это не блокер.

## 6. Pending вопросы (требуется ответ)
1) non_formalized.td_1.customs_post_code — нужен код таможенного поста назначения для графы 29.
2) non_formalized.td_1.customs_post_name — нужно наименование таможенного поста назначения.
3) non_formalized.svh_additional_sheet_1.svh_customs_code — нужен код таможни в зоне СВХ для графы 30.
4) non_formalized.svh_additional_sheet_1.svh_address_region — адрес СВХ (регион).
5) non_formalized.svh_additional_sheet_1.svh_address_city — адрес СВХ (город).
6) non_formalized.svh_additional_sheet_1.svh_address_street_house — адрес СВХ (улица/дом).

## 7. Контроль pending
- Все pending-поля отражены в primary.yaml/issues.
