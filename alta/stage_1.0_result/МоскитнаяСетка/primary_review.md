# primary_review.md

## 1. Метаданные и статус
- Кейс: МоскитнаяСетка
- Статус готовности: Partial

## 2. Сводка по документам
- Contract — SALES CONTRACT NoLM-2553.md — OK
- SupplementaryContract — 1 Supplementary agreement to the contract.md — OK
- Invoice — CL на сетку.md — Внимание (есть pending по справочным строковым названиям и единицам)
- PackingList — PL на сетку.md — Внимание (pending по строковым наименованиям PrDocumentName)
- CMR — СМР от СВХ.md — Внимание (pending по строковым названиям стран)
- PaymentOrder — currency_transfer_7_28.11.2025.md — OK
- PaymentOrder — currency_transfer_1_13.01.2026.md — OK
- ServiceInvoice — Счет_№26-00378-tl_от_27-01-2026.md — Внимание (много pending по Consignor_SubjectAddressDetails)
- InsuranceDocument — Счет_№26-00378-tl_1_от_14-01-2026.md — OK
- TechDescription — техничка Антикот, антипыльца антимошка.md — OK

- StorageReport — ДО 14431420260204161621.md — Внимание (pending actual_gross_weight)
- StorageReportAdditionalSheet — ДО доп 14431520260204161645.md — Blocked (нет данных адреса/кода СВХ)
- TransitDeclaration — ТД 10719110_240126_5011363_reg00378тд.md — OK
- MasterData — stable_source xml — OK

## 3. Конфликты
Явных конфликтов по суммам/местам/весу не найдено: 127 мест и 3500 кг一致 в PL/CMR/ТД/ДО.

## 4. Pending (нужен ответ)
1) formalized.contract_1.ForeignPerson_Address_CounryName — как писать страну текстом (Китай/China)
2) formalized.invoice_1.Registration_PrDocumentName — как называть документ
3) formalized.invoice_1.InvoiceGoods[*].MeasureUnitQualifierName — подтвердить 'м²' (cb:unit 055)
4) non_formalized.svh_1.actual_gross_weight — что считать "фактическим" по ДО-1
5) non_formalized.svh_additional_sheet_1.* — нужен адрес/код таможни СВХ из оригинала/от тебя
