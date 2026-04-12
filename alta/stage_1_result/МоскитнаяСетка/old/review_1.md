# review_1.md — Этап 1 — МоскитнаяСетка

## 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **Статус готовности:** Partial (остались pending по полям, отсутствующим в первичке)
- **Всего обработано документов:** 16
- **Количество конфликтов:** 0
- **Количество недостающих данных (Pending):** >0 (см. раздел 4)

## 2. Сводка по документам
- [Contract] — SALES CONTRACT NoLM-2553.pdf (прочитан по PNG) — OK
- [Supplementary Contract] — 1 Supplementary agreement to the contract.pdf — Внимание (ForeignPerson_ShortName, PersonMiddleName)
- [Invoice] — CL на сетку .pdf (Invoice LM-2591) — OK (ключевые решения подтверждены оператором)
- [Packing List] — PL на сетку .pdf — Внимание (ShortName сторон)
- [CMR] — СМР от СВХ.PNG (CMR №00378) — Внимание (Incoterms в CMR отсутствуют; Guarantee-блок отсутствует)
- [Transit Declaration] — ТД 10719110/240126/5011363 — OK
- [Payment] — currency_transfer_7_28.11.2025.pdf — OK (Priority=5 по решению оператора)
- [Payment] — currency_transfer_1_13.01.2026.pdf — OK (Priority=5 по решению оператора)
- [Service Invoice] — Счет_№26-00378-tl_от_27-01-2026.pdf — Внимание (системные поля и consignor)
- [Insurance/Other] — Счет_№26-00378-tl_1_от_14-01-2026.pdf — OK (как insurance_document)
- [TechDescription] — техничка .pdf — OK
- [TechDescription] — техничка Антикот, антипыльца антимошка .pdf — OK
- [Stable: EGRUL] — FreeDoc_ЮЭ9965-25-106893283.xml — OK
- [Stable: LOA] — LetterOfAttorney_1.xml — OK
- [Stable: Passport] — Passport_63_09_449948.xml — OK
- [Stable: Transport Contract] — FreeDoc_КООО_26651_М.xml — OK
- [Non-formalized: СВХ ДО-1] — ДО 14431420260204161621.PNG + доплист — OK

## 3. Существенные данные первички, которые не попали в primary.md
- Нет.

## 4. Pending (ТРЕБУЕТСЯ РЕШЕНИЕ)
Ниже перечислены поля, которые отсутствуют в первичке и требуют решения, чтобы довести primary.md до статуса ready.

1) **Supplementary Contract: ForeignPerson_ShortName**
   - Поле (UQI): formalized.supplementary_contract_1.ForeignPerson_ShortName
   - Вопрос: заполняем ShortName = OrganizationName (HEBEI LANGMAI IMPORT AND EXPORT CO., LTD)?

2) **Packing List: ShortName сторон**
   - Поля (UQI):
     - formalized.packing_list_1.Consignor_ShortName
     - formalized.packing_list_1.Consignee_ShortName
   - Вопрос: заполняем ShortName = OrganizationName для обеих сторон?

3) **CMR: Incoterms в структуре CMR**
   - Поля (UQI):
     - formalized.cmr_1.DeliveryTerms_DeliveryPlace
     - formalized.cmr_1.DeliveryTerms_DeliveryTermsStringCode
   - Вопрос: в CMR Incoterms нет. Как закрываем:
     - оставить пустым (но confirmed_operator), или
     - проставить EXW / HEBEI как в invoice/PL?

4) **CMR: блок гаранта отправителя**
   - Поля (UQI): formalized.cmr_1.Consignor_Guarantee_*
   - Вопрос: если блока нет в CMR — проставляем "ОТСУТСТВУЕТ" во все поля Guarantee?

5) **Service Invoice: системные поля/ссылки/consignor**
   - Поля (UQI): formalized.service_invoice_1.DocumentSign, formalized.service_invoice_1.Signature_Choice,
     formalized.service_invoice_1.PaymentDocument.*, formalized.service_invoice_1.Consignor_*,
     formalized.service_invoice_1.Consignee_SubjectAddressDetails.House/Room,
     formalized.service_invoice_1.ServiceDescription_[1..2].ServiceName
   - Вопрос: подтверди набор решений (я применю в primary.md):
     - DocumentSign=1, Signature_Choice=1;
     - PaymentDocument: оставить пусто;
     - Consignor = seller из invoice;
     - House=30, Room=211;
     - ServiceName: оставить пусто.
