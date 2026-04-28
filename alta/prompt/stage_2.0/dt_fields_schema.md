# Stage 2.0 — подготовка полей ДТ

## 1. Назначение и границы

Выход этапа оформляется либо в полном, либо в кратком варианте. 
- В полном варианте сформировать набор полей ДТ для этапа 2.1 в двух представлениях:
  - `dt_fields.yaml` — машинный файл (источник истины для stage 2.1).
  - `dt_fields.md` — человеческий файл для просмотра оператором (табличный формат).
- В кратком варианте сформировать только `dt_fields.yaml`.
  
Также формируется `dt_fields_review.md` — краткий отчет по результату построения. Источником истины служит
`dt_fields.yaml`.

Этап 2.0:
- использует факты только из `primary.yaml`;
- рассчитывает производные значения по явно описанным правилам;
- фиксирует все пробелы/конфликты в `Нерешенные вопросы`.

Этап 2.0 НЕ делает:
- не читает первичные документы поставки напрямую;
- не генерирует `dt.xml` (это этап 2.1).

Важно: 
- `dt_fields.yaml` — это полный набор полей ДТ по этой схеме. Если отсутствует хотя бы одно поле из раздела 
  «Поля ДТ» — это ошибка этапа.
- stage 2.1 использует только `dt_fields.yaml`, а `dt_fields.md` — только для чтения человеком.

---

## 2. Правила работы с данными

### 2.1. Источник фактов — только `primary.yaml`

Этап 2.0 не читает первичные документы поставки.
Если для поля ДТ нужен факт, которого нет в `primary.yaml`, либо он там не подтвержден/конфликтный — поле в `dt_fields.yaml`
должно быть `pending`, а вопрос вынесен в `issues` (`dt_fields.yaml`) и (в полном режиме) в раздел `Нерешенные вопросы` 
в `dt_fields.md`.

### 2.2. Никаких догадок

Запрещено:
- подставлять коды “на глаз”;
- использовать как источник фактов новой поставки:
  - `alta\reference\...`, `...\выгрузки\...` (эталонные ДТ/xml/скриншоты);
  - результаты прошлых прогонов (`stage_*_result`, `trash`).

Разрешено:
- переносить подтвержденные значения из `primary.yaml`;
- выводить производные значения только из подтвержденных данных и по явно записанному правилу;

### 2.3. Статус поля в `dt_fields.yaml`

`status`:
- `confirmed_primary` — значение напрямую из `primary.yaml` и там подтверждено;
- `confirmed_operator` — значение явно задано оператором на этапе 2.0;
- `derived` — значение вычислено по правилу из подтвержденных данных;
- `pending` — данных недостаточно или есть конфликт.

Если `status: pending` — `value` отсутствует или равен `null`.

---

## 3. Формат выходных файлов

Этап 2.0 имеет два режима выхода:
- краткий: формируется только `dt_fields.yaml`;
- полный: формируется `dt_fields.yaml` + `dt_fields.md` + `dt_fields_review.md`.

### 3.1. `dt_fields.yaml` (машинный файл)

Требования:
- Валидный YAML.
- **Не смешивать с Markdown**: никаких markdown-таблиц `|...|`, никаких markdown-заголовков/списков как “текстового оформления”.
- Только структуры YAML: map/array/scalar.

Состав:
1) `meta`
2) `fields`
3) `issues`

#### 3.1.1. meta
Минимум:
- `case_name`
- `generated_at`
- `primary_input_path` - путь к primary.yaml

#### 3.1.2. fields
`fields` содержит все поля по схеме (раздел «Поля ДТ»).

Формат поля:
- `<field_path>`:
  - `value`: `<значение>`  *(если pending — отсутствует или `null`)*
  - `status`: `<confirmed_primary | confirmed_operator | derived | pending>`
  - `source`: `[<primary_path1>, <primary_path2>]` *(опционально)*
  - `note`: `<строка>` *(опционально)*

Правила:
- Если `status: derived` — правило вычисления кратко фиксируется в `note` (или через `source`, если достаточно).
- Если `status: pending` — `value` отсутствует или `null`.

#### 3.1.3. issues
`issues` — это “Нерешенные вопросы”, предназначенные оператору/возврату в stage 1.
Формат записи вопроса:

- `<field_path>`:
  - `question`: `<один конкретный вопрос>`
  - `impact`: `<что блокирует>`
  - `requested_from`: `<operator | stage_1_fix>`

Если вопрос общий:
- `[General]`:
  - `question`: ...
  - `impact`: ...
  - `requested_from`: ...

---

### 3.2. `dt_fields.md` (человеческий файл)

Требования:
- Чистый Markdown.
- Для просмотра оператором.
- **Не является источником истины**: все значения должны соответствовать `dt_fields.yaml`, MD не добавляет уникальных фактов.

Состав:
1) `Meta` (кейс, дата генерации, входной `primary.yaml`)
2) `Поля ДТ` — по разделам схемы (5.1, 5.2, …)
3) `Нерешенные вопросы`

Формат представления полей: **таблицы**

Таблица для каждого раздела:
| field_path | value | status | source | note |
|---|---|---|---|---|

Для массивов (`goods[i]`, `goods[i].tovg[j]`, `goods[i].txt[j]`) — подзаголовки и отдельные таблицы.

---

## 4. Шаблоны полей ДТ
птп - правило требует подтверждения. Добавляется в note, если нет уверенности.

### 4.1. Заголовок декларации

#### Направление и процедура

- declaration.direction:
  - value.rule: если декларация на импорт → ИМ
  - status: derived | pending
  - note: графа 1.1 — направление декларации (G_1_1)

- declaration.procedure:
  - value: 40
  - status: confirmed_operator | pending
  - note: графа 1.2 — код таможенной процедуры. Требует подтверждения оператора. Значение из cb:procedure. (G_1_2)
  
- declaration.form:
  - value: ЭД
  - status: derived | pending
  - note: графа 1.31 — форма подачи декларации; для Альты всегда ЭД (G_1_31)
  
---

### 4.2. Отправитель (графа 2)

- sender.country_name:
  - value: formalized.invoice_1.Seler_PostalAddress_CounryName
  - status: confirmed_primary | pending
  - note: графа 2 — текстовое название страны (G_2_50)

- sender.country_code:
  - value: formalized.invoice_1.Seler_PostalAddress_CountryCode
  - status: confirmed_primary | pending
  - note: графа 2 — код страны alpha-2 (G_2_7)

- sender.name:
  - value: formalized.invoice_1.Seler_Name
  - status: confirmed_primary | pending
  - note: графа 2 — полное наименование отправителя (G_2_NAM)

- sender.region:
  - value: formalized.invoice_1.Seler_PostalAddress_Region
  - status: confirmed_primary | pending
  - note: графа 2 — область/район (G_2_SUB)

- sender.city:
  - value: formalized.invoice_1.Seler_PostalAddress_City
  - status: confirmed_primary | pending
  - note: графа 2 — город (G_2_CIT)

- sender.street:
  - value: formalized.invoice_1.Seler_PostalAddress_StreetHouse
  - status: confirmed_primary | pending
  - note: графа 2 — улица и дом (G_2_STR)

---

### 4.3. Количество товаров и мест (графы 5, 6)

- shipment.total_goods_number:
  - value.rule: размер массива goods
  - status: derived | pending
  - source: goods
  - note: графа 5 — количество товарных позиций в ДТ (G_5_1)

- shipment.packages_flag:
  - value.rule: всегда true (места считаются)
  - status: derived
  - note: графа 6 — признак подсчёта мест (G_6_0)

- shipment.total_packages:
  - value.rule: взять подтверждённое количество мест по приоритету:
          svh.actual_places → packing_list.places_total → invoice.places_quantity 
  - source: svh.actual_places; packing_list.places_total; invoice.places_quantity
  - status: derived | pending
  - note: графа 6 — общее количество грузовых мест (G_6_1)

---

### 4.4. Получатель (графа 8)

- consignee.ogrn:
  - value.rule: если formalized.invoice_1.Consignee_OGRN есть → взять его, иначе non_formalized.master_data_1.declarant_ogrn
  - status: confirmed_primary | pending
  - note: графа 8 — ОГРН получателя (G_8_1)

- consignee.name_display:
  - value.rule: если consignee.same_as_declarant=true → "СМ. ГРАФУ 14 ДТ", иначе consignee.name
  - status: derived
  - note: графа 8 — текст в поле «Получатель» в форме/печати (G_8/NAME)

- consignee.country_name:
  - value: formalized.invoice_1.Buyer_PostalAddress_CounryName
  - status: confirmed_primary | pending
  - note: графа 8 — страна, наименование (G_8_50)

- consignee.inn_kpp:
  - value.rule: formalized.invoice_1.Buyer_CompanyID + "/" + formalized.invoice_1.Buyer_KPPCode
  - status: derived | pending
  - source: formalized.invoice_1.Buyer_CompanyID; formalized.invoice_1.Buyer_KPPCode
  - note: графа 8 — ИНН/КПП через "/" (G_8_6)

- consignee.country_code:
  - value: formalized.invoice_1.Buyer_PostalAddress_CountryCode
  - status: confirmed_primary | pending
  - note: графа 8 — код страны alpha-2 (G_8_7)

- consignee.name:
  - value: formalized.invoice_1.Buyer_Name
  - status: confirmed_primary | pending
  - note: графа 8 — наименование организации (G_8_NAM)

- consignee.postcode:
  - value: formalized.invoice_1.Buyer_PostalAddress_PostalCode
  - status: confirmed_primary | pending
  - note: графа 8 — почтовый индекс (G_8_POS)

- consignee.region:
  - value: formalized.invoice_1.Buyer_PostalAddress_Region
  - status: confirmed_primary | pending
  - note: графа 8 — регион (G_8_SUB)

- consignee.city:
  - value: formalized.invoice_1.Buyer_PostalAddress_City
  - status: confirmed_primary | pending
  - note: графа 8 — населённый пункт (G_8_CIT)

- consignee.street:
  - value: formalized.invoice_1.Buyer_PostalAddress_StreetHouse
  - status: confirmed_primary | pending
  - note: графа 8 — улица (G_8_STR)

- consignee.building:
  - value.rule: извлечь дом из formalized.invoice_1.Buyer_PostalAddress_StreetHouse, если отдельно не задано; иначе pending
  - status: derived | pending
  - source: formalized.invoice_1.Buyer_PostalAddress_StreetHouse
  - note: графа 8 — дом (G_8_BLD)

- consignee.room:
  - value.rule: извлечь офис/помещение из formalized.invoice_1.Buyer_PostalAddress_StreetHouse, если отдельно не задано; 
    иначе pending
  - status: derived | pending
  - source: formalized.invoice_1.Buyer_PostalAddress_StreetHouse
  - note: графа 8 — помещение/офис (G_8_ROM)

- consignee.same_as_declarant:
  - value.rule: true если consignee.inn_kpp == declarant.inn_kpp (или если в non_formalized.master_data_1 задан флаг 
    same_as_declarant)
  - status: derived
  - note: графа 8 — признак «см. графу 14» (G_8_SM14)

- consignee.phone:
  - value: non_formalized.master_data_1.declarant_phone
  - status: confirmed_primary | pending
  - note: графа 8 — телефон (G_8_PHONE)

- consignee.email:
  - value: non_formalized.master_data_1.declarant_email
  - status: confirmed_primary | pending
  - note: графа 8 — e-mail (G_8_EMAIL)
  
---

### 4.5. Финансовое урегулирование (графа 9) — как “см. графу 14”

- financial.same_as_declarant:
  - value.rule: всегда true (в этом проекте графа 9 = графа 14)
  - status: derived
  - note: графа 9 — признак «см. графу 14» (G_9_SM14)

- financial.name_display:
  - value.rule: всегда "СМ. ГРАФУ 14 ДТ"
  - status: derived
  - note: графа 9 — текст в поле графы 9 в форме/печати (G_9/NAME)

- financial.ogrn:
  - value: declarant.ogrn
  - status: derived | pending
  - note: графа 9 — ОГРН (G_9_1)

- financial.inn_kpp:
  - value: declarant.inn_kpp
  - status: derived | pending
  - note: графа 9 — ИНН/КПП (G_9_4)

- financial.name:
  - value: declarant.name
  - status: derived | pending
  - note: графа 9 — наименование (G_9_NAM)

- financial.country_code:
  - value: declarant.country_code
  - status: derived | pending
  - note: графа 9 — код страны (G_9_CC)

- financial.country_name:
  - value: declarant.country_name
  - status: derived | pending
  - note: графа 9 — наименование страны (G_9_CN)

- financial.postcode:
  - value: declarant.postcode
  - status: derived | pending
  - note: графа 9 — индекс (G_9_POS)

- financial.region:
  - value: declarant.region
  - status: derived | pending
  - note: графа 9 — регион (G_9_SUB)

- financial.city:
  - value: declarant.city
  - status: derived | pending
  - note: графа 9 — город (G_9_CIT)

- financial.street:
  - value: declarant.street
  - status: derived | pending
  - note: графа 9 — улица (G_9_STR)

- financial.building:
  - value: declarant.building
  - status: derived | pending
  - note: графа 9 — дом (G_9_BLD)

- financial.room:
  - value: declarant.room
  - status: derived | pending
  - note: графа 9 — помещение (G_9_ROM)

- financial.country_code_alt:
  - value: declarant.country_code
  - status: derived | pending
  - note: графа 9 — дублирующий код страны (G_9_7)

- financial.phone:
  - value: declarant.phone
  - status: derived | pending
  - note: графа 9 — телефон (G_9_PHONE)

- financial.email:
  - value: declarant.email
  - status: derived | pending
  - note: графа 9 — e-mail (G_9_EMAIL)

---

### 4.6. Торгующая страна (графа 11)

- shipment.trade_country_code:
  - value: formalized.invoice_1.DeliveryTerms_TradingCountryCode
  - status: confirmed_primary | pending
  - note: графа 11 — код торгующей страны alpha-2 (G_11_1)

---

### 4.7. Декларант (графа 14)

- declarant.ogrn:
  - value: non_formalized.master_data_1.declarant_ogrn
  - status: confirmed_primary | pending
  - note: графа 14 — ОГРН декларанта (G_14_1)

- declarant.name_display:
  - value.rule: собрать строку печатного блока из declarant.name + адрес + контакты
  - status: derived | pending
  - source: declarant.*
  - note: графа 14 — текст в поле графы 14 в форме/печати (G_14/NAME)

- declarant.inn_kpp:
  - value.rule: non_formalized.master_data_1.declarant_inn + "/" + non_formalized.master_data_1.declarant_kpp
  - status: derived | pending
  - source: non_formalized.master_data_1.declarant_inn; non_formalized.master_data_1.declarant_kpp
  - note: графа 14 — ИНН/КПП через "/" (G_14_4)

- declarant.name:
  - value: non_formalized.master_data_1.declarant_name
  - status: confirmed_primary | pending
  - note: графа 14 — наименование организации (G_14_NAM)

- declarant.country_code:
  - value: non_formalized.master_data_1.declarant_address_country_code
  - status: confirmed_primary | pending
  - note: графа 14 — код страны (G_14_CC)

- declarant.country_name:
  - value: non_formalized.master_data_1.declarant_address_country_name
  - status: confirmed_primary | pending
  - note: графа 14 — наименование страны (G_14_CN)

- declarant.postcode:
  - value: non_formalized.master_data_1.declarant_address_postal_code
  - status: confirmed_primary | pending
  - note: графа 14 — почтовый индекс (G_14_POS)

- declarant.region:
  - value: non_formalized.master_data_1.declarant_address_region
  - status: confirmed_primary | pending
  - note: графа 14 — регион (G_14_SUB)

- declarant.city:
  - value: non_formalized.master_data_1.declarant_address_city
  - status: confirmed_primary | pending
  - note: графа 14 — населённый пункт (G_14_CIT)

- declarant.street:
  - value: non_formalized.master_data_1.declarant_address_street
  - status: confirmed_primary | pending
  - note: графа 14 — улица (G_14_STR)

- declarant.building:
  - value: non_formalized.master_data_1.declarant_address_building
  - status: confirmed_primary | pending
  - note: графа 14 — дом (G_14_BLD)

- declarant.room:
  - value: non_formalized.master_data_1.declarant_address_room
  - status: confirmed_primary | pending
  - note: графа 14 — помещение/офис (G_14_ROM)

- declarant.phone:
  - value: non_formalized.master_data_1.declarant_phone
  - status: confirmed_primary | pending
  - note: графа 14 — телефон (G_14_PHONE)

- declarant.email:
  - value: non_formalized.master_data_1.declarant_email
  - status: confirmed_primary | pending
  - note: графа 14 — e-mail (G_14_EMAIL)

---

### 4.8. Страны (графы 15, 16, 17)

- shipment.dispatch_country_code:
  - value: formalized.invoice_1.DeliveryTerms_DispatchCountryCode
  - status: confirmed_primary | pending
  - note: графа 15A — код страны отправления alpha-2 (G_15A_1)

- shipment.destination_country_code:
  - value: formalized.invoice_1.DeliveryTerms_DestinationCountryCode
  - status: confirmed_primary | pending
  - note: графа 17A — код страны назначения alpha-2 (G_17A_1)

- shipment.dispatch_country_name:
  - value.rule: получить наименование страны по shipment.dispatch_country_code через cb:country
  - status: derived | pending
  - source: shipment.dispatch_country_code
  - note: графа 15 — страна отправления, текст (G_15_1)

- shipment.destination_country_name:
  - value.rule: получить наименование страны по shipment.destination_country_code через cb:country
  - status: derived | pending
  - source: shipment.destination_country_code
  - note: графа 17 — страна назначения, текст (G_17_1)

- shipment.origin_country_code:
  - value.rule: если у всех InvoiceGoods_* один OriginCountryCode (numeric) → нормализовать в alpha-2 через cb:country, 
    иначе pending
  - status: derived | pending
  - source: formalized.invoice_1.InvoiceGoods_*.OriginCountryCode
  - note: графа 16 — код страны происхождения alpha-2 (G_16_2)

- shipment.origin_country_name:
  - value.rule: получить наименование страны по shipment.origin_country_code через cb:country
  - status: derived | pending
  - source: shipment.origin_country_code
  - note: графа 16 — страна происхождения, текст (G_16_1)

---

### 4.9. Условия поставки (графа 20)

- delivery.terms_code:
  - value.rule: приоритет источников:
    formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode → 
    formalized.packing_list_1.DeliveryTerms_DeliveryTermsStringCode → formalized.contract_1.ContractTerms_OtherTerms (парсинг)
  - status: derived | pending
  - source: formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode; 
    formalized.packing_list_1.DeliveryTerms_DeliveryTermsStringCode; formalized.contract_1.ContractTerms_OtherTerms
  - note: графа 20 — условия поставки (G_20_20)

- delivery.place_name:
  - value.rule: приоритет источников: formalized.invoice_1.DeliveryTerms_DeliveryPlace → 
    formalized.packing_list_1.DeliveryTerms_DeliveryPlace → formalized.contract_1.ContractTerms_OtherTerms (парсинг)
  - status: derived | pending
  - source: formalized.invoice_1.DeliveryTerms_DeliveryPlace; formalized.packing_list_1.DeliveryTerms_DeliveryPlace; 
    formalized.contract_1.ContractTerms_OtherTerms
  - note: графа 20 — место поставки (G_20_21)

---

### 4.10. Транспорт (графы 18, 19, 21)

- transport.mode_code_internal:
  - value.rule: количество ТС = число блоков TransportMeans_* в formalized.packing_list_1
  - status: derived | pending
  - source: formalized.packing_list_1.TransportMeans_*
  - note: графа 18 — количество транспортных средств (G_18_0)

- transport.identification:
  - value.rule: приоритет источников:
    join(formalized.packing_list_1.TransportMeans_*.Number, "/") → non_formalized.td_1.transport_reg_number → 
    non_formalized.svh_1.transport_reg_number
  - status: derived | pending
  - source: formalized.packing_list_1.TransportMeans_*.Number; non_formalized.td_1.transport_reg_number; 
    non_formalized.svh_1.transport_reg_number
  - note: графа 18 — идентификация ТС (G_18)

- transport.registration_country_code:
  - value.rule: если formalized.packing_list_1.TransportMeans_1.NationalityCode = "000" → "00", иначе взять как есть; 
    если данных нет → pending
  - status: derived | pending
  - source: formalized.packing_list_1.TransportMeans_1.NationalityCode
  - note: графа 18 — код страны регистрации ТС (G_18_2)

- transport.container_flag:
  - value.rule: 0 (перевозка без контейнера); если правило не подтверждено для кейса → pending
  - status: derived | pending
  - note: графа 19 — признак контейнера (G_19_1)

- transport.border_mode:
  - value.rule: для автоперевозки → 1
  - status: derived
  - note: графа 21 — код активного ТС на границе (G_21_0)

---

### 4.11. Валюта и стоимость (графа 22)

- shipment.invoice_currency_alpha:
  - value: formalized.invoice_1.CurrencyCode
  - status: confirmed_primary | pending
  - note: графа 22 — буквенный код валюты (G_22_3)

- shipment.invoice_currency_numeric:
  - value.rule: преобразовать shipment.invoice_currency_alpha в numeric код ISO
  - status: derived | pending
  - source: shipment.invoice_currency_alpha
  - note: графа 22 — цифровой код валюты (G_22_1)

- shipment.invoice_amount:
  - value: formalized.invoice_1.TotalCost
  - status: confirmed_primary | pending
  - note: графа 22 — сумма по счёту (G_22_2)

---

### 4.12. Курс валюты (графа 23)

- shipment.currency_rate:
  - value: formalized.invoice_1.CurrencyRate
  - status: confirmed_primary | pending
  - note: графа 23 — курс валюты к рублю на дату подачи (G_23_1, G_23_2)

---

### 4.13. Вид транспорта (графы 25, 26)

- transport.border_transport_code:
  - value.rule: для автотранспорта → 31
  - status: derived
  - note: графа 25 — код вида транспорта на границе (G_25_1)

- transport.internal_transport_code:
  - value.rule: для автотранспорта → 31 (совпадает с графой 25)
  - status: derived
  - note: графа 26 — код вида транспорта внутри страны (G_26_1)

---

### 4.14. Таможня на границе (графа 29)

- customs.border_code:
  - value: non_formalized.td_1.customs_post_code
  - status: confirmed_primary | pending
  - note: графа 29 — код таможенного органа на границе (G_29_1); источник: ТД (09013)

- customs.border_name:
  - value: non_formalized.td_1.customs_post_name
  - status: confirmed_primary | pending
  - note: графа 29 — наименование таможенного поста (G_29_2); источник: ТД (09013)
  
---

### 4.15. Местонахождение товаров (графа 30)

- location.type:
  - value.rule: для СВХ → 11
  - status: derived
  - note: графа 30 — тип места нахождения товаров (G_30_0); 11 = склад временного хранения

- location.document_kind:
  - value.rule: для лицензии СВХ → 2
  - status: derived
  - note: графа 30 — вид документа, подтверждающего место хранения (G_30_10); 2 = свидетельство/лицензия

- location.document_number:
  - value: non_formalized.svh_1.warehouse_license_number
  - status: confirmed_primary | pending
  - note: графа 30 — номер документа СВХ (G_30_1)

- location.document_date:
  - value: non_formalized.svh_1.warehouse_license_date
  - status: confirmed_primary | pending
  - note: графа 30 — дата документа СВХ (G_30_DATE)

- location.address.country_code:
  - value.rule: для склада в РФ → RU
  - status: derived
  - note: графа 30 — код страны местонахождения товаров (G_30_CC)

- location.address.region:
  - value: non_formalized.svh_additional_sheet_1.svh_address_region
  - status: confirmed_primary | pending
  - note: графа 30 — регион (G_30_SUB)

- location.address.city:
  - value: non_formalized.svh_additional_sheet_1.svh_address_city
  - status: confirmed_primary | pending
  - note: графа 30 — город (G_30_CIT)

- location.address.street:
  - value: non_formalized.svh_additional_sheet_1.svh_address_street_house
  - status: confirmed_primary | pending
  - note: графа 30 — улица и дом (G_30_STR)

- location.customs_code:
  - value: non_formalized.svh_additional_sheet_1.svh_customs_code
  - status: confirmed_primary | pending
  - note: графа 30 — код таможенного органа, в зоне которого находится СВХ (G_30_12)

- location.printed:
  - value.rule: собрать из location.type + ", " + location.customs_code + ", " + location.address.region + " " + 
    location.address.city + " " + location.address.street + ", " + location.document_number + " ОТ " + 
    location.document_date
  - status: derived | pending
  - source: location.*
  - note: графа 30 — печатная строка местонахождения (G_30P_1); формируется автоматически
  
---

### 4.16 Товары (BLOCK, графы 31–47)

`goods` — массив товаров ДТ (каждый элемент = один `BLOCK` в XML и один товар в интерфейсе Альты).

Правило агрегации (строго):
- взять все строки `invoice.InvoiceGoods_*`;
- `GoodsCode` = значение `invoice.InvoiceGoods_*.GoodsCode` (оно же заполняет `goods[i].tnved_code`, графа 33);
- сгруппировать строки по `GoodsCode`;
- число элементов `goods` должно быть равно числу уникальных `GoodsCode`;
- различия между строками внутри одного `GoodsCode` (артикул/вид/модель/описание/количество) НЕ создают новый `goods[i]`:
  они отражаются в `goods[i].tovg[]` и/или `goods[i].txt[]`;
- веса/стоимости товара ДТ = суммы по строкам группы (по подтверждённым значениям из `primary.yaml`).

#### 4.16.1. Графа 31 — описание товаров (G_31)

- goods[i].g31.name:
  - value.rule: сформировать обобщённое описание группы товаров по данным `goods[i].tovg` + "СМ.ДОПОЛНЕНИЕ"
  - status: derived | pending
  - source: goods[i].tovg
  - note: графа 31 — описание товара (G_31/NAME). "ДОПОЛНЕНИЕ" в `goods[i].txt[]` / `goods[i].tovg[]`.

- goods[i].g31.manufacturer:
  - value.rule: если у всех строк группы один производитель → он, иначе "СМ.ДОПОЛНЕНИЕ"
  - status: derived | pending
  - source: goods[i].tovg.manufacturer
  - note: графа 31 — производитель (G_31/FIRMA)

- goods[i].g31.trademark:
  - value.rule: если у всех строк группы ТМ одинаковая → она, иначе "СМ.ДОПОЛНЕНИЕ"; если ТМ отсутствует → "ОТСУТСТВУЕТ"
  - status: derived | pending
  - source: goods[i].tovg.trade_mark
  - note: графа 31 — товарный знак / ТМ (G_31/TM)

- goods[i].places:
  - value.rule: non_formalized.svh_1.goods_[n].places, где non_formalized.svh_1.goods_[n].tnved == goods[i].tnved_code.value
  - status: derived | pending
  - source: non_formalized.svh_1.goods_[n].tnved; non_formalized.svh_1.goods_[n].places; goods[i].tnved_code
  - note: графа 31 — количество мест по товару (G_31/PLACE)
  
#### 4.16.2. Графы 32–38 — код товара, страна, веса, процедура

- goods[i].item_no:
  - value.rule: порядковый номер товара в ДТ (1..N)
  - status: derived
  - note: графа 32 — номер товара (G_32_1)

- goods[i].tnved_code:
  - value.rule: код ТН ВЭД товара ДТ = `invoice.InvoiceGoods_*.GoodsCode` для этой группы
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.GoodsCode
  - note: графа 33 — код товара (G_33_1)

- goods[i].tnved.flag_1:
  - value.rule: значение-литера после кода. птп.
  - status: derived | pending
  - note: графа 33 — доп. признак (G_33_4)

- goods[i].tnved.flag_2:
  - value.rule: значение-литера после кода. птп.
  - status: derived | pending
  - note: графа 33 — доп. признак (G_33_5)

- goods[i].origin_country_code:
  - value.rule: alpha-2 страны происхождения товара (нормализовать из numeric/alpha в primary)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.OriginCountryCode
  - note: графа 34 — код страны происхождения (G_34_1)

- goods[i].gross_weight:
  - value.rule: приоритет источников брутто по товару:
    non_formalized.svh_1.goods_[n].gross_weight_kg (по tnved) → сумма invoice.InvoiceGoods_*.GrossWeightQuantity по группе
  - status: derived | pending
  - source: non_formalized.svh_1.goods_[n].tnved; non_formalized.svh_1.goods_[n].gross_weight_kg; 
    invoice.InvoiceGoods_*.GrossWeightQuantity
  - note: графа 35 — вес брутто по товару (G_35_1)

- goods[i].preference:
  - value.rule: код преференции. птп.
  - status: derived | pending
  - note: графа 36 — преференция (G_36_2)

- goods[i].procedure_code:
  - value.rule: код процедуры по товару (часто 4000000 для ИМ40). птп.
  - status: derived | pending
  - source: declaration.direction; declaration.procedure
  - note: графа 37 — процедура по товару (G_37_1)

- goods[i].net_weight:
  - value.rule: сумма нетто по строкам группы
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.NetWeightQuantity
  - note: графа 38 — вес нетто по товару (G_38_1)

#### 4.16.3. Графы 42–46 — стоимости по товару

- goods[i].invoice_cost:
  - value.rule: сумма стоимости по инвойсу по строкам группы (валюта графы 22)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.TotalCost
  - note: графа 42 — цена товара (G_42_1)

- goods[i].mos_code_main:
  - value.rule: код МОС. птп.
  - status: derived | pending
  - note: графа 43 — код МОС (G_43_1)

- goods[i].mos_code_extra:
  - value.rule: доп. код МОС. птп.
  - status: derived | pending
  - note: графа 43 — доп. признак (G_43_2)

- goods[i].customs_value:
  - value.rule: рассчитывается Альтой по ДТС; в dt_fields.md можно оставить pending/не заполнять
  - status: pending
  - note: графа 45 — таможенная стоимость (G_45_0, G_45_1)

- goods[i].statistical_value:
  - value.rule: рассчитывается Альтой; если требуется — derived по правилам Альты, иначе pending
  - status: pending
  - note: графа 46 — статистическая стоимость (G_46_1)

#### 4.16.4. Графа 47 — исчисление платежей (по товару) (ПТП: Это, кажется, Альта сама считает. Не надо исключить?)
**Этот раздел не материализуем до выяснения**
- goods[i].payments[k].payment_code:
  - value.rule: вид платежа (например 1010/2010/5010)
  - status: derived | pending
  - note: графа 47 — вид платежа (G_47_*_*_1). См. cb:payment

- goods[i].payments[k].tax_base:
  - value.rule: база начисления
  - status: derived | pending
  - note: графа 47 — основа начисления (G_47_*_*_2)

- goods[i].payments[k].rate:
  - value.rule: ставка (может быть % или фикс, как "4924РУБ.")
  - status: derived | pending
  - note: графа 47 — ставка (G_47_*_*_3)

- goods[i].payments[k].amount:
  - value.rule: сумма платежа
  - status: derived | pending
  - note: графа 47 — сумма (G_47_*_*_4)

- goods[i].payments[k].payment_method:
  - value.rule: способ уплаты. птп.
  - status: derived | pending
  - note: графа 47 — СП (G_47_*_*_5)

#### 4.16.5. Дополнение к графе 31 — TXT (детальные строки)

- goods[i].txt[j].text:
  - value.rule: сформировать строки дополнения к графе 31 из `goods[i].tovg`. птп.
  - status: derived | pending
  - source: goods[i].tovg
  - note: графа 31 — строки дополнения (TXT/TEXT)

#### 4.16.6. Таблица описания — TOVG (строки внутри товара)

- goods[i].tovg[j].line_no:
  - value.rule: порядковый номер строки внутри товара (1..M)
  - status: derived
  - note: графа 31 — № строки таблицы (TOVG/G32G)

- goods[i].tovg[j].description:
  - value.rule: описание строки (как в инвойсе + нормализация/перевод при наличии)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.GoodsDescription
  - note: графа 31 — наименование (TOVG/G31_1)

- goods[i].tovg[j].manufacturer:
  - value.rule: производитель (из primary)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_Manufacturer
  - note: графа 31 — производитель (TOVG/G31_11)

- goods[i].tovg[j].trade_mark:
  - value.rule: ТМ (из primary; если отсутствует → "ОТСУТСТВУЕТ")
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_TradeMark
  - note: графа 31 — марка/ТМ (TOVG/G31_12)

- goods[i].tovg[j].goods_mark:
  - value.rule: товарный знак/маркировка (если отсутствует → "ОТСУТСТВУЕТ")
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_GoodsMark
  - note: графа 31 — товарный знак (TOVG/G31_14)

- goods[i].tovg[j].model:
  - value.rule: модель/модификация (из primary; при наличии размеров/параметров — включить их в модель). птп.
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_GoodsModel; invoice.InvoiceGoods_*.GoodsDescription
  - note: графа 31 — модель (TOVG/G31_15_MOD)
  
- goods[i].tovg[j].quantity:
  - value: invoice.InvoiceGoods_[j].goods_supplementary_quantity
  - status: confirmed_primary | pending
  - source: invoice.InvoiceGoods_[j].goods_supplementary_quantity
  - note: графа 31 — количество в доп.ед.изм (TOVG/KOLVO)

- goods[i].tovg[j].unit_code:
  - value.rule: найти код ЕИ по cb:unit по наименованию invoice.InvoiceGoods_[j].goods_supplementary_uom_name
  - status: derived | pending
  - source: invoice.InvoiceGoods_[j].goods_supplementary_uom_name; cb:unit
  - note: графа 31 — код ЕИ (TOVG/CODE_EDI)

- goods[i].tovg[j].unit_name:
  - value: invoice.InvoiceGoods_[j].goods_supplementary_uom_name
  - status: confirmed_primary | pending
  - source: invoice.InvoiceGoods_[j].goods_supplementary_uom_name
  - note: графа 31 — наименование ЕИ (TOVG/NAME_EDI)
  
- goods[i].tovg[j].gross_weight:
  - value.rule: брутто по строке
  - status: confirmed_primary | pending
  - source: invoice.InvoiceGoods_*.GrossWeightQuantity
  - note: графа 35 — вес брутто по строке (TOVG/G31_35)

- goods[i].tovg[j].net_weight:
  - value.rule: нетто по строке
  - status: confirmed_primary | pending
  - source: invoice.InvoiceGoods_*.NetWeightQuantity
  - note: графа 38 — вес нетто по строке (TOVG/G31_38)

- goods[i].tovg[j].invoice_cost:
  - value.rule: стоимость по строке инвойса
  - status: confirmed_primary | pending
  - source: invoice.InvoiceGoods_*.TotalCost
  - note: графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST)

---

### 4.17. Теги после товаров (графы 51–54)

#### 4.17.1. Графа 42 (доп. признак)

- declaration.g42_2:
  - value.rule: доп. признак графы 42 (например "В ДТС" если применяется). птп.
  - status: derived | pending
  - note: графа 42 — доп. признак (G_42_2)

#### 4.17.3. Графа 54 — уполномоченное лицо / представитель

- representative.date:
  - value.rule: дата заполнения/подачи ДТ (задается оператором на этапе 2 или берется как текущая дата по явному решению)
  - status: derived | pending
  - note: графа 54 — дата заполнения/подачи (G_54_20)
  
- representative.phone:
  - value: master_data.representative.phone
  - status: confirmed_primary | pending
  - note: графа 54 — телефон (G_54_21)

- representative.email:
  - value: master_data.representative.email
  - status: confirmed_primary | pending
  - note: графа 54 — e-mail (G_54_EMAIL)

- representative.last_name:
  - value: master_data.representative.last_name
  - status: confirmed_primary | pending
  - note: графа 54 — фамилия (G_54_3)

- representative.first_name:
  - value: master_data.representative.first_name
  - status: confirmed_primary | pending
  - note: графа 54 — имя (G_54_3NM)

- representative.middle_name:
  - value: master_data.representative.middle_name
  - status: confirmed_primary | pending
  - note: графа 54 — отчество (G_54_3MD)

- representative.authority_doc_name:
  - value: master_data.representative.authority_doc_name
  - status: confirmed_primary | pending
  - note: графа 54 — документ полномочий (G_54_4)

- representative.authority_doc_number:
  - value: master_data.representative.authority_doc_number
  - status: confirmed_primary | pending
  - note: графа 54 — № документа полномочий (G_54_5)

- representative.authority_doc_date_from:
  - value: master_data.representative.authority_doc_date_from
  - status: confirmed_primary | pending
  - note: графа 54 — дата начала действия (G_54_60)

- representative.authority_doc_date_to:
  - value: master_data.representative.authority_doc_date_to
  - status: confirmed_primary | pending
  - note: графа 54 — дата окончания действия (G_54_61)

- representative.position:
  - value: master_data.representative.position
  - status: confirmed_primary | pending
  - note: графа 54 — должность/статус (G_54_7)

- representative.passport_code:
  - value: master_data.representative.passport_code
  - status: confirmed_primary | pending
  - note: графа 54 — код документа удостоверения личности (G_54_8)

- representative.passport_name:
  - value: master_data.representative.passport_name
  - status: confirmed_primary | pending
  - note: графа 54 — наименование документа (G_54_9)

- representative.passport_number:
  - value: master_data.representative.passport_number
  - status: confirmed_primary | pending
  - note: графа 54 — номер паспорта (G_54_100)

- representative.passport_date:
  - value: master_data.representative.passport_date
  - status: confirmed_primary | pending
  - note: графа 54 — дата выдачи паспорта (G_54_101)

- representative.passport_series:
  - value: master_data.representative.passport_series
  - status: confirmed_primary | pending
  - note: графа 54 — серия паспорта (G_54_12)

- representative.passport_issuer:
  - value: master_data.representative.passport_issuer
  - status: confirmed_primary | pending
  - note: графа 54 — кем выдан (G_54_13)

- representative.printed_block:
  - value.rule: собрать печатную строку представителя (ФИО + паспорт + роль + контакты + доверенность). птп.
  - status: derived | pending
  - source: representative.*
  - note: графа 54 — печатный блок (G_54P)

---

## 5. Самопроверка после формирования

- `dt_fields.yaml` валиден как YAML
- Все поля из раздела «Поля ДТ» присутствуют в `dt_fields.yaml`
- Если выбран полный режим:
  - `dt_fields.md` содержит табличное представление всех полей схемы (на основе `dt_fields.yaml`)
  - `dt_fields_review.md` сформирован и соответствует шаблону review
- Для каждого `derived` поля правило указано в `note` или `source`
- Все `pending`/конфликты вынесены в `issues` (YAML) и (в полном режиме) в `Нерешенные вопросы` (MD)

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

