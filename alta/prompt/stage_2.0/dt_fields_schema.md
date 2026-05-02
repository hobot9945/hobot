# Stage 2.0 — подготовка полей ДТ

## 1. Назначение и границы

Вход этапа:
- `primary.md` (факты поставки),
- короткие версии справочников в этой схеме,
- `alta\prompt\codebook.md` - полные справочники.

- Выход этапа:
- `dt_fields.md` (источник истины для stage 2.1),
- `dt_fields_review.md` — краткий отчет.

Этап 2.0, используя факты только из `primary.md`, 
- рассчитывает производные значения по явно описанным правилам,
- это полный набор полей ДТ по этой схеме, не просто текстовый документ, а формальная база данных для этапа 2.1. 
  Если отсутствует хотя бы одно поле из раздела "Шаблоны полей ДТ" — это ошибка этапа, AI фиксирует все 
  пробелы/конфликты в `dt_fields.md/issues`.
- НЕ читает первичные документы поставки напрямую.

---

## 2. Правила работы с данными

### 2.1. Никаких догадок

Запрещено:
- подставлять коды “на глаз”, используй справочники;
- использовать как источник фактов новой поставки:
  - `alta\reference\...`, `...\выгрузки\...` (эталонные ДТ/xml/скриншоты);
  - результаты прошлых прогонов (`dt_fields.md`, `dt_xml`) можно использовать для доработки/обновления, но запрещено
    использовать в начальной генерации.

AI обязан:
- переносить только подтвержденные значения из `primary.md`;
- выводить производные значения по явно записанному правилу;
- если не хватает данных — pending.

### 2.2. Статус поля в `primary.md`
`status`:
- `CD` - confirmed_document, подтвержденное значение, взято из исходных документов,
- `CO` - confirmed_operator, значение явно задано оператором,
- `pending` — данных недостаточно или есть конфликт.

### 2.3. Статус поля в `dt_fields.md`

`status`:
- `CP` - confirmed_primary, подтвержденное значение, взято напрямую из `primary.md`,
- `CO` - confirmed_operator, значение явно задано оператором,
- `D` - derived, значение вычислено по правилу из подтвержденных данных,
- `pending` — данных недостаточно или есть конфликт.

Если `status: pending`, то `value` пустое.

### 2.4. `value` - это значение, полученное по ссылке
Если `value` задан как путь вида `formalized.*` / `non_formalized.*` / `declarant.*`, это означает:
что при материализации `value` должно получить значение, взятое по этому пути.

---

## 3. Шаблоны полей ДТ
ПТП - правило требует подтверждения. Добавляется в note, если нет уверенности.

### 3.1. Заголовок декларации

- 01: declaration.direction:
  - value.rule: meta.direction
  - status: CD | pending
  - note: графа 1.1 — направление декларации (импорт / экспорт) (G_1_1)

- 02: declaration.procedure:
  - value: 40
  - status: CO | pending
  - note: графа 1.2 — код таможенной процедуры. Требует подтверждения оператора. Значение из cb:procedure. (G_1_2)

- 03: declaration.form:
  - value: ЭД
  - status: D | pending
  - note: графа 1.31 — форма подачи декларации; для Альты всегда ЭД (G_1_31)

- _audit: 3 

---

### 3.2. Отправитель (графа 2)

- 01: sender.country_name:
  - value: formalized.invoice_1.Seler_PostalAddress_CounryName
  - status: CP | pending
  - note: графа 2 — текстовое название страны (G_2_50)

- 02: sender.country_code:
  - value: formalized.invoice_1.Seler_PostalAddress_CountryCode
  - status: CP | pending
  - note: графа 2 — код страны alpha-2 (G_2_7)

- 03: sender.name:
  - value: formalized.invoice_1.Seler_Name
  - status: CP | pending
  - note: графа 2 — полное наименование отправителя (G_2_NAM)

- 04: sender.region:
  - value: formalized.invoice_1.Seler_PostalAddress_Region
  - status: CP | pending
  - note: графа 2 — область/район (G_2_SUB)

- 05: sender.city:
  - value: formalized.invoice_1.Seler_PostalAddress_City
  - status: CP | pending
  - note: графа 2 — город (G_2_CIT)

- 06: sender.street:
  - value: formalized.invoice_1.Seler_PostalAddress_StreetHouse
  - status: CP | pending
  - note: графа 2 — улица и дом (G_2_STR)

- _audit: 6

---

### 3.3. Количество товаров и мест (графы 5, 6)

- 01: shipment.total_goods_number:
  - value.rule: размер массива goods
  - status: D | pending
  - source: goods
  - note: графа 5 — количество товарных позиций в ДТ (G_5_1)

- 02: shipment.packages_flag:
  - value.rule: всегда true (места считаются)
  - status: D
  - note: графа 6 — признак подсчёта мест (G_6_0)

- 03: shipment.total_packages:
  - value.rule: взять подтверждённое количество мест по приоритету:
    svh.actual_places → packing_list.places_total → invoice.places_quantity
  - source: svh.actual_places; packing_list.places_total; invoice.places_quantity
  - status: D | pending
  - note: графа 6 — общее количество грузовых мест (G_6_1)

- _audit: 3

---

### 3.4. Получатель (графа 8)

- 01: consignee.ogrn:
  - value.rule: если formalized.invoice_1.Consignee_OGRN есть → взять его, иначе formalized.letter_of_attorney_1.Organization_OGRN
  - status: CP | pending
  - note: графа 8 — ОГРН получателя (G_8_1)

- 02: consignee.name_display:
  - value.rule: если consignee.same_as_declarant=true → "СМ. ГРАФУ 14 ДТ", иначе consignee.name
  - status: D
  - note: графа 8 — текст в поле «Получатель» в форме/печати (G_8/NAME)

- 03: consignee.country_name:
  - value: formalized.invoice_1.Buyer_PostalAddress_CounryName
  - status: CP | pending
  - note: графа 8 — страна, наименование (G_8_50)

- 04: consignee.inn_kpp:
  - value.rule: formalized.invoice_1.Buyer_CompanyID + "/" + formalized.invoice_1.Buyer_KPPCode
  - status: D | pending
  - source: formalized.invoice_1.Buyer_CompanyID; formalized.invoice_1.Buyer_KPPCode
  - note: графа 8 — ИНН/КПП через "/" (G_8_6)

- 05: consignee.country_code:
  - value: formalized.invoice_1.Buyer_PostalAddress_CountryCode
  - status: CP | pending
  - note: графа 8 — код страны alpha-2 (G_8_7)

- 06: consignee.name:
  - value: formalized.invoice_1.Buyer_Name
  - status: CP | pending
  - note: графа 8 — наименование организации (G_8_NAM)

- 07: consignee.postcode:
  - value: formalized.invoice_1.Buyer_PostalAddress_PostalCode
  - status: CP | pending
  - note: графа 8 — почтовый индекс (G_8_POS)

- 08: consignee.region:
  - value: formalized.invoice_1.Buyer_PostalAddress_Region
  - status: CP | pending
  - note: графа 8 — регион (G_8_SUB)

- 09: consignee.city:
  - value: formalized.invoice_1.Buyer_PostalAddress_City
  - status: CP | pending
  - note: графа 8 — населённый пункт (G_8_CIT)

- 10: consignee.street:
  - value: formalized.invoice_1.Buyer_PostalAddress_StreetHouse
  - status: CP | pending
  - note: графа 8 — улица (G_8_STR)

- 11: consignee.building:
  - value.rule: извлечь дом из formalized.invoice_1.Buyer_PostalAddress_StreetHouse, если отдельно не задано; иначе pending
  - status: D | pending
  - source: formalized.invoice_1.Buyer_PostalAddress_StreetHouse
  - note: графа 8 — дом (G_8_BLD)

- 12: consignee.room:
  - value.rule: извлечь офис/помещение из formalized.invoice_1.Buyer_PostalAddress_StreetHouse, если отдельно не задано;
    иначе pending
  - status: D | pending
  - source: formalized.invoice_1.Buyer_PostalAddress_StreetHouse
  - note: графа 8 — помещение/офис (G_8_ROM)

- 13: consignee.same_as_declarant:
  - value.rule: true если consignee.inn_kpp == declarant.inn_kpp
    same_as_declarant)
  - status: D
  - note: графа 8 — признак «см. графу 14» (G_8_SM14)

- 14: consignee.phone:
  - value.rule: pending (в первичных документах отсутствует). птп
  - status: pending
  - note: графа 8 — телефон (G_8_PHONE)

- 15: consignee.email:
  - value.rule: pending (в первичных документах отсутствует). птп
  - status: pending
  - note: графа 8 — e-mail (G_8_EMAIL)
    
- _audit: 15

---

### 3.5. Финансовое урегулирование (графа 9) — как “см. графу 14”

- 01: financial.same_as_declarant:
  - value.rule: всегда true (в этом проекте графа 9 = графа 14)
  - status: D
  - note: графа 9 — признак «см. графу 14» (G_9_SM14)

- 02: financial.name_display:
  - value.rule: всегда "СМ. ГРАФУ 14 ДТ"
  - status: D
  - note: графа 9 — текст в поле графы 9 в форме/печати (G_9/NAME)

- 03: financial.ogrn:
  - value: declarant.ogrn
  - status: D | pending
  - note: графа 9 — ОГРН (G_9_1)

- 04: financial.inn_kpp:
  - value: declarant.inn_kpp
  - status: D | pending
  - note: графа 9 — ИНН/КПП (G_9_4)

- 05: financial.name:
  - value: declarant.name
  - status: D | pending
  - note: графа 9 — наименование (G_9_NAM)

- 06: financial.country_code:
  - value: declarant.country_code
  - status: D | pending
  - note: графа 9 — код страны (G_9_CC)

- 07: financial.country_name:
  - value: declarant.country_name
  - status: D | pending
  - note: графа 9 — наименование страны (G_9_CN)

- 08: financial.postcode:
  - value: declarant.postcode
  - status: D | pending
  - note: графа 9 — индекс (G_9_POS)

- 09: financial.region:
  - value: declarant.region
  - status: D | pending
  - note: графа 9 — регион (G_9_SUB)

- 10: financial.city:
  - value: declarant.city
  - status: D | pending
  - note: графа 9 — город (G_9_CIT)

- 11: financial.street:
  - value: declarant.street
  - status: D | pending
  - note: графа 9 — улица (G_9_STR)

- 12: financial.building:
  - value: declarant.building
  - status: D | pending
  - note: графа 9 — дом (G_9_BLD)

- 13: financial.room:
  - value: declarant.room
  - status: D | pending
  - note: графа 9 — помещение (G_9_ROM)

- 14: financial.country_code_alt:
  - value: declarant.country_code
  - status: D | pending
  - note: графа 9 — дублирующий код страны (G_9_7)

- 15: financial.phone:
  - value: declarant.phone
  - status: D | pending
  - note: графа 9 — телефон (G_9_PHONE)

- 16: financial.email:
  - value: declarant.email
  - status: D | pending
  - note: графа 9 — e-mail (G_9_EMAIL)

- _audit: 16

---

### 3.6. Торгующая страна (графа 11)

- 01: shipment.trade_country_code:
  - value: formalized.invoice_1.DeliveryTerms_TradingCountryCode
  - status: CP | pending
  - note: графа 11 — код торгующей страны alpha-2 (G_11_1)

---

### 3.7. Декларант (графа 14)

- 01: declarant.ogrn:
  - value: formalized.letter_of_attorney_1.Organization_OGRN
  - status: CP | pending
  - note: графа 14 — ОГРН декларанта (G_14_1)

- 02: declarant.name_display:
  - value.rule: собрать строку печатного блока из declarant.name + адрес + контакты
  - status: D | pending
  - source: declarant.*
  - note: графа 14 — текст в поле графы 14 в форме/печати (G_14/NAME)

- 03: declarant.inn_kpp:
  - value.rule: formalized.letter_of_attorney_1.Organization_INN + "/" + formalized.letter_of_attorney_1.Organization_KPP
  - status: D | pending
  - source: formalized.letter_of_attorney_1.Organization_INN; formalized.letter_of_attorney_1.Organization_KPP
  - note: графа 14 — ИНН/КПП через "/" (G_14_4)

- 04: declarant.name:
  - value: formalized.letter_of_attorney_1.Organization_OrganizationName
  - status: CP | pending
  - note: графа 14 — наименование организации (G_14_NAM)

- 05: declarant.country_code:
  - value: formalized.letter_of_attorney_1.Organization_Address_CountryCode
  - status: CP | pending
  - note: графа 14 — код страны (G_14_CC)

- 06: declarant.country_name:
  - value: formalized.letter_of_attorney_1.Organization_Address_CounryName
  - status: CP | pending
  - note: графа 14 — наименование страны (G_14_CN)

- 07: declarant.postcode:
  - value: formalized.letter_of_attorney_1.Organization_Address_PostalCode
  - status: CP | pending
  - note: графа 14 — почтовый индекс (G_14_POS)

- 08: declarant.region:
  - value: formalized.letter_of_attorney_1.Organization_Address_Region
  - status: CP | pending
  - note: графа 14 — регион (G_14_SUB)

- 09: declarant.city:
  - value: formalized.letter_of_attorney_1.Organization_Address_City
  - status: CP | pending
  - note: графа 14 — населённый пункт (G_14_CIT)

- 10: declarant.street:
  - value: formalized.letter_of_attorney_1.Organization_Address_StreetHouse
  - status: CP | pending
  - note: графа 14 — улица (G_14_STR)

- 11: declarant.building:
  - value: formalized.letter_of_attorney_1.Organization_Address_StreetHouse
  - status: CP | pending
  - note: графа 14 — дом (G_14_BLD)

- 12: declarant.room:
  - value.rule: извлечь офис/помещение из formalized.letter_of_attorney_1.Organization_Address_StreetHouse, если отдельно не задано; иначе pending
  - status: D | pending
  - source: formalized.letter_of_attorney_1.Organization_Address_StreetHouse
  - note: графа 14 — помещение/офис (G_14_ROM)

- 13: declarant.phone:
  - value.rule: pending (в доверенности и ЕГРЮЛ отсутствует). птп
  - status: pending
  - note: графа 14 — телефон (G_14_PHONE); требуется решение оператора

- 14: declarant.email:
  - value.rule: pending (в доверенности и ЕГРЮЛ отсутствует). птп
  - status: pending
  - note: графа 14 — e-mail (G_14_EMAIL); требуется решение оператора
    
- _audit: 14

---

### 3.8. Страны (графы 15, 16, 17)

- 01: shipment.dispatch_country_code:
  - value: formalized.invoice_1.DeliveryTerms_DispatchCountryCode
  - status: CP | pending
  - note: графа 15A — код страны отправления alpha-2 (G_15A_1)

- 02: shipment.destination_country_code:
  - value: formalized.invoice_1.DeliveryTerms_DestinationCountryCode
  - status: CP | pending
  - note: графа 17A — код страны назначения alpha-2 (G_17A_1)

- 03: shipment.dispatch_country_name:
  - value.rule: получить наименование страны по shipment.dispatch_country_code через cb:country
  - status: D | pending
  - source: shipment.dispatch_country_code
  - note: графа 15 — страна отправления, текст (G_15_1)

- 04: shipment.destination_country_name:
  - value.rule: получить наименование страны по shipment.destination_country_code через cb:country
  - status: D | pending
  - source: shipment.destination_country_code
  - note: графа 17 — страна назначения, текст (G_17_1)

- 05: shipment.origin_country_code:
  - value.rule: если у всех InvoiceGoods_* один OriginCountryCode → нормализовать в alpha-2 через 
    cb:country (numeric/alpha-2/alpha-3), иначе pending
  - status: D | pending
  - source: formalized.invoice_1.InvoiceGoods_*.OriginCountryCode
  - note: графа 16 — код страны происхождения alpha-2 (G_16_2)

- 06: shipment.origin_country_name:
  - value.rule: получить наименование страны по shipment.origin_country_code через cb:country
  - status: D | pending
  - source: shipment.origin_country_code
  - note: графа 16 — страна происхождения, текст (G_16_1)

- _audit: 6

---

### 3.9. Условия поставки (графа 20)

- 01: delivery.terms_code:
  - value.rule: приоритет источников:
    formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode →
    formalized.packing_list_1.DeliveryTerms_DeliveryTermsStringCode → formalized.contract_1.ContractTerms_OtherTerms (парсинг)
  - status: D | pending
  - source: formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode;
    formalized.packing_list_1.DeliveryTerms_DeliveryTermsStringCode; formalized.contract_1.ContractTerms_OtherTerms
  - note: графа 20 — условия поставки (G_20_20)

- 02: delivery.place_name:
  - value.rule: приоритет источников: formalized.invoice_1.DeliveryTerms_DeliveryPlace →
    formalized.packing_list_1.DeliveryTerms_DeliveryPlace → formalized.contract_1.ContractTerms_OtherTerms (парсинг)
  - status: D | pending
  - source: formalized.invoice_1.DeliveryTerms_DeliveryPlace; formalized.packing_list_1.DeliveryTerms_DeliveryPlace;
    formalized.contract_1.ContractTerms_OtherTerms
  - note: графа 20 — место поставки (G_20_21)

- _audit: 2

---

### 3.10. Транспорт (графы 18, 19, 21)
Правило: Автоперевозка = если в primary.md присутствует formalized.cmr_1 (документ найден).

- 01: transport.vehicles_count:
  - value.rule: количество ТС = число блоков TransportMeans_* в formalized.packing_list_1
  - status: D | pending
  - source: formalized.packing_list_1.TransportMeans_*
  - note: графа 18 — количество транспортных средств (G_18_0)

- 02: transport.identification:
  - value.rule: приоритет источников:
    join(formalized.packing_list_1.TransportMeans_*.Number, "/") → non_formalized.td_1.transport_reg_number →
    non_formalized.svh_1.transport_reg_number
  - status: D | pending
  - source: formalized.packing_list_1.TransportMeans_*.Number; non_formalized.td_1.transport_reg_number;
    non_formalized.svh_1.transport_reg_number
  - note: графа 18 — идентификация ТС (G_18)

- 03: transport.registration_country_code:
  - value.rule: если formalized.packing_list_1.TransportMeans_1.NationalityCode = "000" → "00", иначе взять как есть;
    если данных нет → pending
  - status: D | pending
  - source: formalized.packing_list_1.TransportMeans_1.NationalityCode
  - note: графа 18 — код страны регистрации ТС (G_18_2)

- 04: transport.container_flag:
  - value.rule: 0 (перевозка без контейнера); если правило не подтверждено для кейса → pending
  - status: D | pending
  - note: графа 19 — признак контейнера (G_19_1)

- 05: transport.border_mode:
  - value.rule: если автоперевозка → 1, иначе pending
  - status: D | pending
  - note: графа 21 — код активного ТС на границе (G_21_0)

- _audit: 5

---

### 3.11. Валюта и стоимость (графа 22)

- 01: shipment.invoice_currency_alpha:
  - value: formalized.invoice_1.CurrencyCode
  - status: CP | pending
  - note: графа 22 — буквенный код валюты (G_22_3)

- 02: shipment.invoice_currency_numeric:
  - value.rule: преобразовать shipment.invoice_currency_alpha в numeric код ISO
  - status: D | pending
  - source: shipment.invoice_currency_alpha
  - note: графа 22 — цифровой код валюты (G_22_1)

- 03: shipment.invoice_amount:
  - value: formalized.invoice_1.TotalCost
  - status: CP | pending
  - note: графа 22 — сумма по счёту (G_22_2)

- _audit: 3

---

### 3.12. Курс валюты (графа 23)

- 01: shipment.currency_rate:
  - value: formalized.invoice_1.CurrencyRate
  - status: CP | pending
  - note: графа 23 — курс валюты к рублю на дату подачи (G_23_1, G_23_2)

- _audit: 1

---

### 3.13. Вид транспорта (графы 25, 26)

- 01: transport.border_transport_code:
  - value.rule: для автотранспорта → 31
  - status: D
  - note: графа 25 — код вида транспорта на границе (G_25_1)

- 02: transport.internal_transport_code:
  - value.rule: для автотранспорта → 31 (совпадает с графой 25)
  - status: D
  - note: графа 26 — код вида транспорта внутри страны (G_26_1)

- _audit: 2

---

### 3.14. Таможня на границе (графа 29)

- 01: customs.border_code:
  - value: non_formalized.td_1.customs_post_code
  - status: CP | pending
  - note: графа 29 — код таможенного органа на границе (G_29_1); источник: ТД (09013)

- 02: customs.border_name:
  - value: non_formalized.td_1.customs_post_name
  - status: CP | pending
  - note: графа 29 — наименование таможенного поста (G_29_2); источник: ТД (09013)

- _audit: 2

---

### 3.15. Местонахождение товаров (графа 30)

- 01: location.type:
  - value.rule: для СВХ → 11
  - status: D
  - note: графа 30 — тип места нахождения товаров (G_30_0); 11 = склад временного хранения

- 02: location.document_kind:
  - value.rule: для лицензии СВХ → 2
  - status: D
  - note: графа 30 — вид документа, подтверждающего место хранения (G_30_10); 2 = свидетельство/лицензия

- 03: location.document_number:
  - value: non_formalized.svh_1.warehouse_license_number
  - status: CP | pending
  - note: графа 30 — номер документа СВХ (G_30_1)

- 04: location.document_date:
  - value: non_formalized.svh_1.warehouse_license_date
  - status: CP | pending
  - note: графа 30 — дата документа СВХ (G_30_DATE)

- 05: location.address.country_code:
  - value.rule: для склада в РФ → RU
  - status: D
  - note: графа 30 — код страны местонахождения товаров (G_30_CC)

- 06: location.address.region:
  - value: non_formalized.svh_additional_sheet_1.svh_address_region
  - status: CP | pending
  - note: графа 30 — регион (G_30_SUB)

- 07: location.address.city:
  - value: non_formalized.svh_additional_sheet_1.svh_address_city
  - status: CP | pending
  - note: графа 30 — город (G_30_CIT)

- 08: location.address.street:
  - value: non_formalized.svh_additional_sheet_1.svh_address_street_house
  - status: CP | pending
  - note: графа 30 — улица и дом (G_30_STR)

- 09: location.customs_code:
  - value: non_formalized.svh_additional_sheet_1.svh_customs_code
  - status: CP | pending
  - note: графа 30 — код таможенного органа, в зоне которого находится СВХ (G_30_12)

- 10: location.printed:
  - value.rule: собрать из location.type + ", " + location.customs_code + ", " + location.address.region + " " +
    location.address.city + " " + location.address.street + ", " + location.document_number + " ОТ " +
    location.document_date
  - status: D | pending
  - source: location.*
  - note: графа 30 — печатная строка местонахождения (G_30P_1); формируется автоматически

- _audit: 10

---

### 3.16 Товары (BLOCK, графы 31–47)

`goods` — массив товаров ДТ (каждый элемент = один `BLOCK` в XML и один товар в интерфейсе Альты).

Правило агрегации (строго):
- взять все строки `invoice.InvoiceGoods_*`;
- `GoodsCode` = значение `invoice.InvoiceGoods_*.GoodsCode` (оно же заполняет `goods[i].tnved_code`, графа 33);
- сгруппировать строки по `GoodsCode`;
- число элементов `goods` должно быть равно числу уникальных `GoodsCode`;
- различия между строками внутри одного `GoodsCode` (артикул/вид/модель/описание/количество) НЕ создают новый `goods[i]`:
  они отражаются в `goods[i].tovg[]` и/или `goods[i].txt[]`;
- веса/стоимости товара ДТ = суммы по строкам группы (по подтверждённым значениям из `primary.md`).

#### 3.16.1. Графа 31 — описание товаров (G_31)

- 01: goods[i].g31.name:
  - value.rule: сформировать обобщённое описание группы товаров по данным `goods[i].tovg` + "СМ.ДОПОЛНЕНИЕ"
  - status: D | pending
  - source: goods[i].tovg
  - note: графа 31 — описание товара (G_31/NAME). "ДОПОЛНЕНИЕ" в `goods[i].txt[]` / `goods[i].tovg[]`.

- 02: goods[i].g31.manufacturer:
  - value.rule: если у всех строк группы один производитель → он, иначе "СМ.ДОПОЛНЕНИЕ"
  - status: D | pending
  - source: goods[i].tovg.manufacturer
  - note: графа 31 — производитель (G_31/FIRMA)

- 03: goods[i].g31.trademark:
  - value.rule: если у всех строк группы ТМ одинаковая → она, иначе "СМ.ДОПОЛНЕНИЕ"; если ТМ отсутствует → "ОТСУТСТВУЕТ"
  - status: D | pending
  - source: goods[i].tovg.trade_mark
  - note: графа 31 — товарный знак / ТМ (G_31/TM)

- 04: goods[i].places:
  - value.rule: non_formalized.svh_1.goods_[n].places, где non_formalized.svh_1.goods_[n].tnved == goods[i].tnved_code.value
  - status: D | pending
  - source: non_formalized.svh_1.goods_[n].tnved; non_formalized.svh_1.goods_[n].places; goods[i].tnved_code
  - note: графа 31 — количество мест по товару (G_31/PLACE)

- _item_audit: 4

#### 3.16.2. Графы 32–38 — код товара, страна, веса, процедура

- 01: goods[i].item_no:
  - value.rule: порядковый номер товара в ДТ (1..N)
  - status: D
  - note: графа 32 — номер товара (G_32_1)

- 02: goods[i].tnved_code:
  - value.rule: код ТН ВЭД товара ДТ = `invoice.InvoiceGoods_*.GoodsCode` для этой группы
  - status: D | pending
  - source: invoice.InvoiceGoods_*.GoodsCode
  - note: графа 33 — код товара (G_33_1)

- 03: goods[i].tnved.flag_1:
  - value.rule: значение-литера после кода. птп.
  - status: D | pending
  - note: графа 33 — доп. признак (G_33_4)

- 04: goods[i].tnved.flag_2:
  - value.rule: значение-литера после кода. птп.
  - status: D | pending
  - note: графа 33 — доп. признак (G_33_5)

- 05: goods[i].origin_country_code:
  - value.rule: alpha-2 страны происхождения товара (нормализовать OriginCountryCode numeric/alpha-2/alpha-3 → alpha-2 
    через cb:country)
  - status: D | pending
  - source: invoice.InvoiceGoods_*.OriginCountryCode; cb:country
  - note: графа 34 — код страны происхождения (G_34_1)

- 06: goods[i].gross_weight:
  - value.rule: приоритет источников брутто по товару:
    non_formalized.svh_1.goods_[n].gross_weight_kg (по tnved) → сумма invoice.InvoiceGoods_*.GrossWeightQuantity по группе
  - status: D | pending
  - source: non_formalized.svh_1.goods_[n].tnved; non_formalized.svh_1.goods_[n].gross_weight_kg;
    invoice.InvoiceGoods_*.GrossWeightQuantity
  - note: графа 35 — вес брутто по товару (G_35_1)

- 07: goods[i].preference:
  - value.rule: код преференции. птп.
  - status: D | pending
  - note: графа 36 — преференция (G_36_2)

- 08: goods[i].procedure_code:
  - value.rule: код процедуры по товару (часто 4000000 для ИМ40). птп.
  - status: D | pending
  - source: declaration.direction; declaration.procedure
  - note: графа 37 — процедура по товару (G_37_1)

- 09: goods[i].net_weight:
  - value.rule: сумма нетто по строкам группы
  - status: D | pending
  - source: invoice.InvoiceGoods_*.NetWeightQuantity
  - note: графа 38 — вес нетто по товару (G_38_1)

- _item_audit: 9

#### 3.16.3. Графы 42–46 — стоимости по товару

- 01: goods[i].invoice_cost:
  - value.rule: сумма стоимости по инвойсу по строкам группы (валюта графы 22)
  - status: D | pending
  - source: invoice.InvoiceGoods_*.TotalCost
  - note: графа 42 — цена товара (G_42_1)

- 02: goods[i].mos_code_main:
  - value.rule: код МОС. птп.
  - status: D | pending
  - note: графа 43 — код МОС (G_43_1)

- 03: goods[i].mos_code_extra:
  - value.rule: доп. код МОС. птп.
  - status: D | pending
  - note: графа 43 — доп. признак (G_43_2)

- 04: goods[i].customs_value:
  - value.rule: рассчитывается Альтой по ДТС; в dt_fields.md можно оставить pending/не заполнять
  - status: pending
  - note: графа 45 — таможенная стоимость (G_45_0, G_45_1)

- 05: goods[i].statistical_value:
  - value.rule: рассчитывается Альтой; если требуется — D по правилам Альты, иначе pending
  - status: pending
  - note: графа 46 — статистическая стоимость (G_46_1)

- _item_audit: 5

#### 3.16.4. Графа 47 — исчисление платежей (по товару) (ПТП: Это, кажется, Альта сама считает. Не надо исключить?)
**Этот раздел не материализуем до выяснения**
- 01: goods[i].payments[k].payment_code:
  - value.rule: вид платежа (например 1010/2010/5010)
  - status: D | pending
  - note: графа 47 — вид платежа (G_47_*_*_1). См. cb:payment

- 02: goods[i].payments[k].tax_base:
  - value.rule: база начисления
  - status: D | pending
  - note: графа 47 — основа начисления (G_47_*_*_2)

- 03: goods[i].payments[k].rate:
  - value.rule: ставка (может быть % или фикс, как "4924РУБ.")
  - status: D | pending
  - note: графа 47 — ставка (G_47_*_*_3)

- 04: goods[i].payments[k].amount:
  - value.rule: сумма платежа
  - status: D | pending
  - note: графа 47 — сумма (G_47_*_*_4)

- 05: goods[i].payments[k].payment_method:
  - value.rule: способ уплаты. птп.
  - status: D | pending
  - note: графа 47 — СП (G_47_*_*_5)

- _item_audit: 5

#### 3.16.5. Дополнение к графе 31 — TXT (детальные строки)

- 01: goods[i].txt[j].text:
  - value.rule: сформировать строки дополнения к графе 31 из `goods[i].tovg`. птп.
  - status: D | pending
  - source: goods[i].tovg
  - note: графа 31 — строки дополнения (TXT/TEXT)

- _item_audit: 1

#### 3.16.6. Таблица описания — TOVG (строки внутри товара)

- 01: goods[i].tovg[j].line_no:
  - value.rule: порядковый номер строки внутри товара (1..M)
  - status: D
  - note: графа 31 — № строки таблицы (TOVG/G32G)

- 02: goods[i].tovg[j].description:
  - value.rule: описание строки (как в инвойсе + нормализация/перевод при наличии)
  - status: D | pending
  - source: invoice.InvoiceGoods_*.GoodsDescription
  - note: графа 31 — наименование (TOVG/G31_1)

- 03: goods[i].tovg[j].manufacturer:
  - value.rule: производитель (из primary)
  - status: D | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_Manufacturer
  - note: графа 31 — производитель (TOVG/G31_11)

- 04: goods[i].tovg[j].trade_mark:
  - value.rule: ТМ (из primary; если отсутствует → "ОТСУТСТВУЕТ")
  - status: D | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_TradeMark
  - note: графа 31 — марка/ТМ (TOVG/G31_12)

- 05: goods[i].tovg[j].goods_mark:
  - value.rule: товарный знак/маркировка (если отсутствует → "ОТСУТСТВУЕТ")
  - status: D | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_GoodsMark
  - note: графа 31 — товарный знак (TOVG/G31_14)

- 06: goods[i].tovg[j].model:
  - value.rule: модель/модификация (из primary; при наличии размеров/параметров — включить их в модель). птп.
  - status: D | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_GoodsModel; invoice.InvoiceGoods_*.GoodsDescription
  - note: графа 31 — модель (TOVG/G31_15_MOD)

- 07: goods[i].tovg[j].quantity:
  - value: invoice.InvoiceGoods_[j].goods_supplementary_quantity
  - status: CP | pending
  - source: invoice.InvoiceGoods_[j].goods_supplementary_quantity
  - note: графа 31 — количество в доп.ед.изм (TOVG/KOLVO)

- 08: goods[i].tovg[j].unit_code:
  - value.rule: найти код ЕИ по cb:unit по наименованию invoice.InvoiceGoods_[j].goods_supplementary_uom_name
  - status: D | pending
  - source: invoice.InvoiceGoods_[j].goods_supplementary_uom_name; cb:unit
  - note: графа 31 — код ЕИ (TOVG/CODE_EDI)

- 09: goods[i].tovg[j].unit_name:
  - value: invoice.InvoiceGoods_[j].goods_supplementary_uom_name
  - status: CP | pending
  - source: invoice.InvoiceGoods_[j].goods_supplementary_uom_name
  - note: графа 31 — наименование ЕИ (TOVG/NAME_EDI)

- 10: goods[i].tovg[j].gross_weight:
  - value.rule: брутто по строке
  - status: CP | pending
  - source: invoice.InvoiceGoods_*.GrossWeightQuantity
  - note: графа 35 — вес брутто по строке (TOVG/G31_35)

- 11: goods[i].tovg[j].net_weight:
  - value.rule: нетто по строке
  - status: CP | pending
  - source: invoice.InvoiceGoods_*.NetWeightQuantity
  - note: графа 38 — вес нетто по строке (TOVG/G31_38)

- 12: goods[i].tovg[j].invoice_cost:
  - value.rule: стоимость по строке инвойса
  - status: CP | pending
  - source: invoice.InvoiceGoods_*.TotalCost
  - note: графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST)

- _item_audit: 12

---

### 3.17. Графа 44 — представляемые документы

Принцип:
- В графу 44 включаются **все формализуемые документы** поставки (все объекты `formalized.*`, присутствующие в
  `primary.md`), включая `transport_contract`, `egrul`, `letter_of_attorney`, `passport` и др.
- Неформализуемые (`non_formalized.*`, например СВХ/ТД) **не включаются** в графу 44 stage 2.0.

#### 3.17.1. Поле G_44 (текстовое поле в карточке товара)

- 01: goods[i].g44.text:
  - value.rule: если у товара есть доп.описание в `goods[i].txt[]` или `goods[i].tovg[]` → "СМ.ДОПОЛНЕНИЕ", иначе пусто
  - status: D
  - source: goods[i].txt; goods[i].tovg
  - note: графа 44 — текстовое поле (G_44)

- _item_audit: 1

#### 3.17.2. Таблица документов (массив записей графы 44)

`goods[i].g44_docs[]` — массив документов, подлежащих представлению.
Правило построения массива:
- для каждого формализуемого документа `formalized.<doc>_<n>` в `primary.md` создать одну запись `goods[i].g44_docs[k]`,
- порядок: как в `primary.md/formalized`.

Поля записи:

- 01: goods[i].g44_docs[k].doc_code:
  - value.rule: взять код вида документа из `formalized.*.DocumentCode`
  - status: CP | pending
  - source: formalized.*.DocumentCode
  - note: графа 44 — код документа (G44/G441), см. cb:doc

- 02: goods[i].g44_docs[k].kind_code:
  - value.rule: G4403 (тип/признак записи). Источник должен быть в `primary.md`. птп.
  - status: pending
  - note: графа 44 — признак записи (G44/G4403).

- 03: goods[i].g44_docs[k].doc_number:
  - value.rule: взять номер документа из типового поля регистрации/ссылки:
    - приоритет: `formalized.*.Registration_PrDocumentNumber` → `formalized.*.DocumentHead_DocumentNumber` →
      `formalized.*.ContractRegistration_PrDocumentNumber` → pending
  - status: D | pending
  - source: formalized.*
  - note: графа 44 — номер документа (G44/G442)

- 04: goods[i].g44_docs[k].doc_date:
  - value.rule: взять дату документа из типового поля регистрации/ссылки:
    - приоритет: `formalized.*.Registration_PrDocumentDate` → `formalized.*.DocumentHead_DocumentDate` →
      `formalized.*.ContractRegistration_PrDocumentDate` → pending
  - status: D | pending
  - source: formalized.*
  - note: графа 44 — дата документа (G44/G443)

- 05: goods[i].g44_docs[k].doc_name:
  - value.rule: короткое наименование документа; приоритет:
    - `formalized.*.Registration_PrDocumentName` → `formalized.*.DocumentHead_DocumentName` → имя из cb:doc по doc_code
  - status: D | pending
  - source: formalized.*; cb:doc
  - note: графа 44 — наименование документа (G44/G444)

- _item_audit: 4

---

### 3.18. Теги после товаров и документов (графы 51–54)

#### 3.18.1. Графа 42 (доп. признак)

- 01: declaration.g42_2:
  - value.rule: доп. признак графы 42 (например "В ДТС" если применяется). птп.
  - status: D | pending
  - note: графа 42 — доп. признак (G_42_2)

- _audit: 1

#### 3.18.3. Графа 54 — уполномоченное лицо / представитель

- 01: representative.date:
  - value.rule: дата заполнения/подачи ДТ (задается оператором на этапе 2 или берется как текущая дата по явному решению)
  - status: D | pending
  - note: графа 54 — дата заполнения/подачи (G_54_20)

- 02: representative.phone:
  - value.rule: pending (в доверенности и паспорте отсутствует). птп
  - status: pending
  - note: графа 54 — телефон (G_54_21)

- 03: representative.email:
  - value.rule: pending (в доверенности и паспорте отсутствует)
  - status: pending
  - note: графа 54 — e-mail (G_54_EMAIL)

- 04: representative.last_name:
  - value: formalized.letter_of_attorney_1.EmpoweredPerson_PersonSurname
  - status: CP | pending
  - note: графа 54 — фамилия (G_54_3)

- 05: representative.first_name:
  - value: formalized.letter_of_attorney_1.EmpoweredPerson_PersonName
  - status: CP | pending
  - note: графа 54 — имя (G_54_3NM)

- 06: representative.middle_name:
  - value: formalized.letter_of_attorney_1.EmpoweredPerson_PersonMiddleName
  - status: CP | pending
  - note: графа 54 — отчество (G_54_3MD)

- 07: representative.authority_doc_name:
  - value: formalized.letter_of_attorney_1.DocumentHead_DocumentName
  - status: CP | pending
  - note: графа 54 — документ полномочий (G_54_4)

- 08: representative.authority_doc_number:
  - value: formalized.letter_of_attorney_1.DocumentHead_DocumentNumber
  - status: CP | pending
  - note: графа 54 — № документа полномочий (G_54_5)

- 09: representative.authority_doc_date_from:
  - value: formalized.letter_of_attorney_1.DocumentHead_DocumentDate
  - status: CP | pending
  - note: графа 54 — дата начала действия (G_54_60)

- 10: representative.authority_doc_date_to:
  - value: formalized.letter_of_attorney_1.EndDate
  - status: CP | pending
  - note: графа 54 — дата окончания действия (G_54_61)

- 11: representative.position:
  - value: formalized.letter_of_attorney_1.EmpoweredPerson_PersonPost
  - status: CP | pending
  - note: графа 54 — должность/статус (G_54_7)

- 12: representative.passport_code:
  - value: RU01001  (может быть, Альта  вставляет сама). птп.
  - status: CP | pending
  - note: графа 54 — код документа удостоверения личности (G_54_8)

- 13: representative.passport_name:
  - value: ПАСРФ (может быть, Альта  вставляет сама). птп.
  - status: CP | pending
  - note: графа 54 — наименование документа (G_54_9)
    
- 14: representative.passport_number:
  - value: formalized.passport_1.CardNumber
  - status: CP | pending
  - note: графа 54 — номер паспорта (G_54_100)

- 15: representative.passport_date:
  - value: formalized.passport_1.CardDate
  - status: CP | pending
  - note: графа 54 — дата выдачи паспорта (G_54_101)

- 16: representative.passport_series:
  - value: formalized.passport_1.CardSeries
  - status: CP | pending
  - note: графа 54 — серия паспорта (G_54_12)
    
- 17: representative.passport_issuer:
  - value: formalized.passport_1.OrganizationName
  - status: CP | pending
  - note: графа 54 — кем выдан (G_54_13)
    
- 18: representative.printed_block:
  - value.rule: собрать печатную строку представителя (ФИО + паспорт + роль + контакты + доверенность). птп.
  - status: D | pending
  - source: representative.*
  - note: графа 54 — печатный блок (G_54P)

- _audit: 18

---

## 4. Формат `dt_fields.md`
primary.md — обычный Markdown файл. В этой схеме примеры фрагментов разметки приводятся в fenced blocks (```),
но в самом primary.md fenced blocks использовать не нужно.

### Разделы:
1) Метаданные
2) Раздел I: Поля ДТ
3) Раздел II: Issues (нерешенные вопросы)

```
## Метаданные:
- `название кейса`: <название кейса>
- `путь к папке поставки`: <путь к папке поставки>
- `тип поставки`: <например: 1 ДТ / 1 товар>
- `агрегация ДТ`: определяется правилами stage 2.0
- `источники данных:` <например: primary.md + operator_provided_data>
```

### Таблица полей
Далее идет таблица:
- AI обязан материализовать все поля, указанные в шаблоне документа;
- Для пустых значений полей ячейка таблицы остается пустой;
- Если для поля не удалось установить значение, status=pending;

Структура таблицы:
- таблица разбита на разделы, соответствующие схеме, например, `### 4.1. Заголовок декларации`,
  `### 4.2. Отправитель (графа 2)` и т.д.

**Нумерация и контроль потерь (Жесткие индексы):**
- Все поля в шаблонах пронумерованы в формате `NN: field_name`, начиная с 1 без пропусков.
- AI **ОБЯЗАН** подставлять эти номера в таблицу полей `dt_fields.md` в том же порядке.
- В конце каждого документа и в конце каждого массива AI **ОБЯЗАН** вывести фактическое и требуемое число полей:
  - `_audit` (для документа),
  - `_item_audit` (для массива).
  - Несовпадение = автоматический признак потери данных.

Формат таблицы:
```
### Раздел I: Поля ДТ

| num                | field       | value             | status            | source      | description       | note             |
|--------------------|-------------|-------------------|-------------------|-------------|-------------------|------------------|
| <порядковый номер> | <имя поля>  | <value или пусто> | <CP/CO/D/pending> | <источники> | <назначение поля> | <note или пусто> |
```

### Для вложенных структур/массивов
Массивы (`goods[i]`, `goods[i].tovg[j]`, `goods[i].txt[j]`) оформлять подзаголовками:
```
#### <имя массива[i]>
затем таблица полей этого элемента тем же форматом.
```

### После каждого элемента массива:
Выводится итог материализации:
```
#### Итого, по элементу массива:
- `item_fields`: <число полей> из <_item_audit>
```

### После каждого массива:
Выводится итог материализации:
```
#### Итого, по массиву:
- `array_elements`: <число элементов массива>
- `item_fields`: всего полей <число полей> из <_item_audit> * <array_elements>
- `array_status`: <confirmed / pending>
```

### После каждого раздела:
Выводится итог материализации:
```
#### Итого, по разделу:
- `fields`: <число полей> из <_audit>
- `partition_status`: <confirmed / pending>
```
Поле `partition_status` pending - если хотя бы одно поле раздела, включая массивы, имеет статус `pending`.

### Итог:
```
### Итог:
- `total_fields`: <полное число полей>
- `dt_status`: <confirmed / pending>

```
`total_fields` - <сумма ВСЕХ полей, включая поля массивов>
`dt_status`- готовность к генерации xml. `confirmed`, если ВСЕ поля получили статус `CP` | `CO` | `D`.

### Раздел II: Issues (нерешенные вопросы)

Формат для полей:
- `<UQI поля со статусом pending>`:
  - `question`: <текст вопроса>

Для общих вопросов:
- `[Общий]`:
  - `question`: <текст вопроса>

```
### Раздел II: Issues (нерешенные вопросы)
<Вопросы>
```

---

## 5. Порядок работы (задание).

- Прочитать `primary.md` и сгенерировать `dt_fields.md`. Выполнить проверки. Чек-лист:
  1) Фиксация версии и контекста: 
     - ✅ Мета-данные заполнены.

  2) Полнота разделов и состава разделов:
     - ✅ Все разделы шаблонов материализованы, включая массивы и подмассивы. 
     - ✅ Число материализованных полей соответствует в каждом разделе / массиве соответствует переменным `_audit`, `item_audit`. 

  3) Достоверность статусов `pending`:
     - ✅ Пройти по всем полям со статусом `pending`, проверить невозможность получения/вывода значения поля.

  4) Проверить правильность расчета полного итога.

  5) Если обнаружены ошибки:
     - пересчитать и внести правки / перегенерировать `dt_fields.md`,
     - повторить проход по чек-листу, начиная с пункта 1.

- Сгенерировать `dt_fields_review.md`
- Выйти в режим диалога, сообщить оператору о результатах.

---

## Приложение. Вырезки из справочников

### Идентификаторы справочников (codebook)

| Идентификатор | Описание | Графы |
|---------------|----------|-------|
| `cb:procedure` | Классификатор таможенных процедур | 1.2, 37 |
| `cb:regime`    | Режимы движения товаров (ИМ/ЭК) | 1.1 |
| `cb:country`   | Страны и их коды | 2, 8, 9, 11, 14–17, 30 |
| `cb:unit`      | Единицы измерения | 31, 41 |
| `cb:doc`       | Коды видов документов | 44 |
| `cb:payment`   | Виды платежей и способы расчётов | 47 |
| `cb:transport` | Виды и режимы транспорта | 18, 21, 25, 26 |
| `cb:location`  | Типы местонахождения товаров | 30 |

---

### `cb:procedure` — Таможенные процедуры

| Код  | Наименование |
|------|--------------|
| 40   | Выпуск для внутреннего потребления |
| 51   | Переработка на таможенной территории |
| 53   | Временный ввоз |
| 60   | Реимпорт |

---

### `cb:regime` — Таможенные режимы

| Код    | Наименование |
|--------|--------------|
| ИМ 40  | Импорт, выпуск для внутреннего потребления |
| ИМ 51  | Переработка на таможенной территории |
| ИМ 53  | Временный ввоз |

---

### `cb:country` — Страны

| Код  | Alpha-2 | Наименование |
|------|---------|--------------|
| 156  | CN      | Китай |
| 643  | RU      | Россия |
| 112  | BY      | Беларусь |
| 398  | KZ      | Казахстан |

---

### `cb:unit` — Единицы измерения

| Код  | Наименование |
|------|--------------|
| 055  | м² (квадратный метр) |
| 166  | кг (килограмм) |
| 796  | шт (штука) |
| 163  | г (грамм) |
| 168  | т (тонна) |
| 006  | м (метр) |
| 121  | м³ (кубический метр) |
| 112  | л (литр) |
| 798  | 1000 шт (тысяча штук) |
| 214  | кВт (киловатт) |

---

### `cb:doc` — Коды видов документов

| Код    | Наименование |
|--------|--------------|
| 03011  | Договор (контракт) |
| 03012  | Дополнительное соглашение к контракту |
| 04021  | Счет-фактура (инвойс) |
| 04131  | Упаковочный лист |
| 02015  | CMR (транспортная накладная) |
| 04023  | Банковские документы / платежное поручение |
| 05999  | Техническое описание / иные документы |
| 06011  | Сертификат о происхождении СТ-1 |
| 06014  | Непреференциальный сертификат о происхождении |
| 11004  | Доверенность |
| 11001  | Паспорт |
| 04033  | Договор перевозки |
| 04031  | Счет за перевозку |
| 04111  | Страховой документ / страховой полис |
| 09023  | Графические материалы (сканы документов) |
| 03031  | Документ валютного контроля (учётный номер контракта) |
| 04025  | Счет-проформа |
| 05013  | Предварительное решение о классификации по ТН ВЭД |
| 07011  | Документы, подтверждающие льготы по уплате платежей |
| 09013  | Транзитная декларация |
| 02017  | Авианакладная |
| 02019  | Почтовая накладная |
| 04051  | Документы о товарах, предоставленных бесплатно |
| 04091  | Бухгалтерская документация производителя |
| 04101  | Счет за проектирование, разработку, дизайн |
| 05019  | Подтверждение целевого назначения товара |
| 07032  | Банковская гарантия |
| 08034  | Документ о регистрации резидента СЭЗ |
| 09018  | Декларация таможенной стоимости |
| 09037  | Заявление о выпуске товаров до подачи ДТ |
| 10044  | Номер предыдущей ДТ (по ст. 114 ТК ЕАЭС) |
| 03999  | Иные документы, подтверждающие право владения |
| 04999  | Иные коммерческие документы |
| 08999  | Иные документы об условиях помещения под процедуру |
| 09999  | Иные документы |
| 10999  | Иные сведения |
| 12990  | Иные документы о результатах таможенного контроля |
| 14010  | Документы о правах на объекты интеллектуальной собственности |
| 14020  | Лицензионный договор / право использования объектов ИС |

---

### `cb:payment` — Виды платежей

| Код   | Наименование |
|-------|--------------|
| 1010  | Таможенные сборы за таможенные операции |
| 1020  | Таможенные сборы за таможенное сопровождение |
| 1030  | Таможенные сборы за хранение |
| 2010  | Ввозная таможенная пошлина |
| 2020  | Ввозная таможенная пошлина (обязанность до 01.09.2010) |
| 2040  | Специальная пошлина (ЕАЭС) |
| 2050  | Антидемпинговая пошлина (ЕАЭС) |
| 2060  | Компенсационная пошлина (ЕАЭС) |
| 2270  | Пошлина на товары электронной торговли (физлица) |
| 5010  | НДС 22% (РФ) |
| 6010  | Пошлины/налоги по единым ставкам (личное пользование) |
| 6020  | Совокупный таможенный платеж (личное пользование) |
| 9070  | Авансовые платежи (в счет будущих платежей) |
| 9080  | Обеспечение исполнения обязанности (кроме денежного залога) |
| 9090  | Денежный залог (обеспечение) |
| 9100  | Денежный залог (обеспечение обязательств по использованию акцизных марок) |
| 9110  | Плата за выдачу акцизных марок |
| 9120  | Утилизационный сбор (КТС/прицепы, ввоз в РФ; кроме из РБ) |
| 9130  | Утилизационный сбор (КТС/прицепы, ввоз в РФ из РБ) |
| 9140  | Госпошлина за предварительное решение по классификации ТН ВЭД ЕАЭС |

---

### `cb:transport` — Виды и режимы транспорта

| Код | Наименование |
|---:|---|
| 10 | Морской/речной транспорт (в т.ч. ТС, перевозимое этим видом транспорта) |
| 20 | Железнодорожный транспорт (в т.ч. ТС, перевозимое этим видом транспорта) |
| 30 | Автодорожный транспорт (кроме кодов 31, 32) |
| 31 | Состав ТС (тягач с полуприцепом или прицепом) |
| 32 | Состав ТС (тягач с прицепом(-ами) и полуприцепом(-ами)) |
| 40 | Воздушный транспорт (в т.ч. ТС, перевозимое этим видом транспорта) |
| 50 | Почтовое отправление |
| 71 | Трубопроводный транспорт |
| 72 | Линии электропередачи |
| 80 | Внутренний водный транспорт (в т.ч. ТС, перевозимое этим видом транспорта) |
| 90 | Транспортное средство, перемещающееся в качестве товара своим ходом |
| 99 | Прочие |

### `cb:location`  - Типы местонахождения товаров

| Код | Наименование |
|---:|---|
| 11 | Склад временного хранения |
| 21 | Таможенный склад |
| 99 | Иное место нахождения товаров |

---

**Полные справочники** — в `alta\prompt\codebook.md`.

