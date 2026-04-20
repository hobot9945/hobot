# dt_fields_schema.md — Stage 2 (primary.md → dt_fields.md → dt.xml)

## 1. Цель этапа

Сформировать модель ДТ в файле `dt_fields.md` на основе `primary.md`, включая:
- прямое перенесение данных из `primary.md` в поля ДТ;
- производные значения (расчёт, композиция, выбор поля по условию) для полей ДТ;
- список нерешённых вопросов (`pending`).

На основе `dt_fields.md` строится `review_2.md` и генерируется `dt.xml` (windows-1251), пригодный для импорта в Альту.

Важно: `dt_fields.md` — это не summary, а нормализованная база полей ДТ. Неполная структура = ошибка этапа.

---

## 2. Общие принципы

### 2.1. Никаких догадок
Любое значение, попадающее в `dt_fields.md`, должно быть:
- взято из `primary.md` (и там иметь статус `confirmed_document/confirmed_operator`), или
- подтверждено оператором в рамках этапа 2, или
- однозначно выведено из подтверждённых данных по явно описанному правилу (derived).
- коды, найденные по справочникам, должны быть надежно идентифицированы (особое внимание на справочник кодов документов).

Если данных не хватает — статус устанавливается в `pending`, проблема фиксируется в разделе `unresolved_questions`.

### 2.2. Источник фактов stage 2 — только `primary.md`
Stage 2 не читает первичные документы поставки напрямую.
Если нужного факта нет в `primary.md` или он конфликтный/`pending` — это:
- вопрос оператору, или
- требование вернуться и исправить stage 1 (дополнить `primary.md`).

### 2.3. Полное покрытие схемы
Все поля должны присутствовать в `dt_fields.md`. Под "полями" понимаются все поля, перечисленные в разделе структуры 
`dt_fields`. Если значение неизвестно — `status: pending` и фиксация в разделе `Нерешенные вопросы`.

---

## 3. Архитектура файла `dt_fields.md`

`dt_fields.md` имеет 3 раздела:

1. **Meta:** общее описание файла.

2. **Раздел I `Поля ДТ`:** значения полей ДТ и статусы.

3. **Раздел II: `Нерешенные вопросы`:** вопросы к оператору - пробелы, конфликты.

---

## 4. Шаблон описания поля `dt_fields`

- `<идентификатор_поля>`:
  - `value`: `<значение | {rule: описание правила}>`
  - `status`: `<confirmed_primary | confirmed_operator | derived | pending>`
  - `source`: `<(опционально) список полей-источников`
  - `note`: `<(опционально) пояснение>`

### Пояснения

- `value`:
  - содержит либо ссылку на поле-источник (например: `invoice.total_cost`),
  - либо правило получения значения (`rule`), если значение не берётся напрямую;
  - не материализуется, если `status` = `pending`.

- `status`:
  - `confirmed_primary` — значение напрямую из `primary.md`;
  - `confirmed_operator` — значение от оператора;
  - `derived` — значение получено по правилу;
  - `pending` — данных недостаточно.

- `source`:
  - указывает смысловой источник значения (например: `invoice.total_cost`, `svh.number`);
  - используется, если происхождение значения неочевидно из `value`;
  - для производных значений — перечисляет поля, участвующие в вычислении;

- `note`:
Имеет разные цели в схеме и в материализованном `dt_fields.md`.
  - в `dt_fields_schema.md`:
    - поясняет смысл поля;
    - уточняет правила применения и снимает двусмысленности;
    - может дополнять правило, но не заменяет его.

  - в `dt_fields.md`:
    - пояснения AI для оператора;
    - не влияет на генерацию `dt.xml`.

### Формат схемы vs формат dt_fields.md

Схема допускает служебные конструкции (`value.rule`).

При формировании `dt_fields.md`:
- если `value` задан как ссылка — подставляется фактическое значение;
- если `value` задан как `rule` — подставляется вычисленное значение;
- при `status=pending` `value` не материализуется;
- `note` — как объяснение, сюда могут переноситься сведения из `value.rule` и `source`.

### Требования

- AI обязан материализовать все поля, указанные в схеме.
- Если значение отсутствует — материализоват со статусом `pending`.

---

## 5. Шаблоны полей ДТ (в порядке XML)

### 5.1. Заголовок декларации

#### Направление и процедура

- declaration.direction:
  - value:
    rule: если декларация на импорт → ИМ
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

### 5.2. Отправитель (графа 2)

- sender.country_name:
  - value: invoice.seller.country_name
  - status: confirmed_primary | pending
  - note: графа 2 — текстовое название страны (G_2_50)

- sender.country_code:
  - value: invoice.seller.country_code
  - status: confirmed_primary | pending
  - note: графа 2 — код страны alpha-2 (G_2_7)

- sender.name:
  - value: invoice.seller.name
  - status: confirmed_primary | pending
  - note: графа 2 — полное наименование отправителя (G_2_NAM)

- sender.region:
  - value: invoice.seller.region
  - status: confirmed_primary | pending
  - note: графа 2 — область/район (G_2_SUB)

- sender.city:
  - value: invoice.seller.city
  - status: confirmed_primary | pending
  - note: графа 2 — город (G_2_CIT)

- sender.street:
  - value: invoice.seller.street
  - status: confirmed_primary | pending
  - note: графа 2 — улица и дом (G_2_STR)

---

### 5.3. Формы (графа 3)

- declaration.forms.sheet_no:
  - value: 
    rule: всегда 1
  - status: derived
  - note: графа 3.1 — номер основного листа (G_3_1)

- declaration.forms.total_sheets:
  - value: shipment.total_goods_number
  - status: confirmed_primary | pending
  - note: графа 3.2 — количество дополнительных листов (G_3_2)

---

### 5.4. Количество товаров и мест (графы 5, 6)

- shipment.total_goods_number:
  - value: 
    rule: размер массива goods[]
  - status: derived | pending
  - source: goods[]
  - note: графа 5 — количество товарных позиций в ДТ (G_5_1)

- shipment.packages_flag:
  - value:
    rule: всегда true (места считаются)
  - status: derived
  - note: графа 6 — признак подсчёта мест (G_6_0)

- shipment.total_packages:
  - value:
    rule: взять подтверждённое количество мест по приоритету:
          svh.actual_places → packing_list.places_total → invoice.places_quantity 
  - source: svh.actual_places; packing_list.places_total; invoice.places_quantity
  - status: derived | pending
  - note: графа 6 — общее количество грузовых мест (G_6_1)

---

### 5.5. Получатель (графа 8)

- consignee.ogrn:
  - value:
    rule: если master_data.consignee.ogrn есть → взять его, иначе invoice.buyer.ogrn
  - status: confirmed_primary | pending
  - note: графа 8 — ОГРН получателя (G_8_1)

- consignee.name_display:
  - value:
    rule: если consignee.same_as_declarant=true → "СМ. ГРАФУ 14 ДТ", иначе consignee.name
  - status: derived
  - note: графа 8 — текст в поле «Получатель» в форме/печати (G_8/NAME)

- consignee.country_name:
  - value:
    rule: если master_data.consignee.country_name есть → взять его, иначе invoice.buyer.country_name
  - status: confirmed_primary | pending
  - note: графа 8 — страна, наименование (G_8_50)

- consignee.inn_kpp:
  - value:
    rule: если master_data.consignee.inn_kpp есть → взять его, иначе invoice.buyer.inn_kpp
  - status: confirmed_primary | pending
  - note: графа 8 — ИНН/КПП через "/" (G_8_6)

- consignee.country_code:
  - value:
    rule: если master_data.consignee.country_code есть → взять его, иначе invoice.buyer.country_code
  - status: confirmed_primary | pending
  - note: графа 8 — код страны alpha-2 (G_8_7)

- consignee.name:
  - value:
    rule: если master_data.consignee.name есть → взять его, иначе invoice.buyer.name
  - status: confirmed_primary | pending
  - note: графа 8 — наименование организации (G_8_NAM)

- consignee.postcode:
  - value:
    rule: если master_data.consignee.postcode есть → взять его, иначе invoice.buyer.postcode
  - status: confirmed_primary | pending
  - note: графа 8 — почтовый индекс (G_8_POS)

- consignee.region:
  - value:
    rule: если master_data.consignee.region есть → взять его, иначе invoice.buyer.region
  - status: confirmed_primary | pending
  - note: графа 8 — регион (G_8_SUB)

- consignee.city:
  - value:
    rule: если master_data.consignee.city есть → взять его, иначе invoice.buyer.city
  - status: confirmed_primary | pending
  - note: графа 8 — населённый пункт (G_8_CIT)

- consignee.street:
  - value:
    rule: если master_data.consignee.street есть → взять его, иначе invoice.buyer.street
  - status: confirmed_primary | pending
  - note: графа 8 — улица (G_8_STR)

- consignee.building:
  - value:
    rule: если master_data.consignee.building есть → взять его, иначе invoice.buyer.building
  - status: confirmed_primary | pending
  - note: графа 8 — дом (G_8_BLD)

- consignee.room:
  - value:
    rule: если master_data.consignee.room есть → взять его, иначе invoice.buyer.room
  - status: confirmed_primary | pending
  - note: графа 8 — помещение/офис (G_8_ROM)

- consignee.same_as_declarant:
  - value:
    rule: true если consignee.inn_kpp == declarant.inn_kpp (или если в master_data задан флаг same_as_declarant)
  - status: derived
  - note: графа 8 — признак «см. графу 14» (G_8_SM14)

- consignee.phone:
  - value:
    rule: если master_data.consignee.phone есть → взять его, иначе invoice.buyer.phone
  - status: confirmed_primary | pending
  - note: графа 8 — телефон (G_8_PHONE)

- consignee.email:
  - value:
    rule: если master_data.consignee.email есть → взять его, иначе invoice.buyer.email
  - status: confirmed_primary | pending
  - note: графа 8 — e-mail (G_8_EMAIL)
  
---

### 5.6. Финансовое урегулирование (графа 9)

- financial.ogrn:
  - value:
    rule: если master_data.financial.ogrn есть → взять его, иначе master_data.consignee.ogrn, иначе invoice.buyer.ogrn
  - status: confirmed_primary | pending
  - note: графа 9 — ОГРН лица, ответственного за финансовое урегулирование (G_9_1)

- financial.name_display:
  - value:
    rule: если financial.same_as_declarant=true → "СМ. ГРАФУ 14 ДТ", иначе financial.name
  - status: derived
  - note: графа 9 — текст в поле графы 9 в форме/печати (G_9/NAME)

- financial.inn_kpp:
  - value:
    rule: если master_data.financial.inn_kpp есть → взять его, иначе master_data.consignee.inn_kpp, иначе invoice.buyer.inn_kpp
  - status: confirmed_primary | pending
  - note: графа 9 — ИНН/КПП через "/" (G_9_4)

- financial.name:
  - value:
    rule: если master_data.financial.name есть → взять его, иначе master_data.consignee.name, иначе invoice.buyer.name
  - status: confirmed_primary | pending
  - note: графа 9 — наименование организации (G_9_NAM)

- financial.country_code:
  - value:
    rule: если master_data.financial.country_code есть → взять его, иначе master_data.consignee.country_code, иначе invoice.buyer.country_code
  - status: confirmed_primary | pending
  - note: графа 9 — код страны (G_9_CC)

- financial.country_name:
  - value:
    rule: если master_data.financial.country_name есть → взять его, иначе master_data.consignee.country_name, иначе invoice.buyer.country_name
  - status: confirmed_primary | pending
  - note: графа 9 — наименование страны (G_9_CN)

- financial.postcode:
  - value:
    rule: если master_data.financial.postcode есть → взять его, иначе master_data.consignee.postcode, иначе invoice.buyer.postcode
  - status: confirmed_primary | pending
  - note: графа 9 — почтовый индекс (G_9_POS)

- financial.region:
  - value:
    rule: если master_data.financial.region есть → взять его, иначе master_data.consignee.region, иначе invoice.buyer.region
  - status: confirmed_primary | pending
  - note: графа 9 — регион (G_9_SUB)

- financial.city:
  - value:
    rule: если master_data.financial.city есть → взять его, иначе master_data.consignee.city, иначе invoice.buyer.city
  - status: confirmed_primary | pending
  - note: графа 9 — населённый пункт (G_9_CIT)

- financial.street:
  - value:
    rule: если master_data.financial.street есть → взять его, иначе master_data.consignee.street, иначе invoice.buyer.street
  - status: confirmed_primary | pending
  - note: графа 9 — улица (G_9_STR)

- financial.building:
  - value:
    rule: если master_data.financial.building есть → взять его, иначе master_data.consignee.building, иначе invoice.buyer.building
  - status: confirmed_primary | pending
  - note: графа 9 — дом (G_9_BLD)

- financial.room:
  - value:
    rule: если master_data.financial.room есть → взять его, иначе master_data.consignee.room, иначе invoice.buyer.room
  - status: confirmed_primary | pending
  - note: графа 9 — помещение/офис (G_9_ROM)

- financial.same_as_declarant:
  - value:
    rule: true если financial.inn_kpp == declarant.inn_kpp (или если в master_data задан флаг same_as_declarant)
  - status: derived
  - note: графа 9 — признак «см. графу 14» (G_9_SM14)

- financial.country_code_alt:
  - value:
    rule: если master_data.financial.country_code есть → взять его, иначе master_data.consignee.country_code, иначе invoice.buyer.country_code
  - status: confirmed_primary | pending
  - note: графа 9 — дублирующий тег кода страны (G_9_7)

- financial.phone:
  - value:
    rule: если master_data.financial.phone есть → взять его, иначе master_data.consignee.phone, иначе invoice.buyer.phone
  - status: confirmed_primary | pending
  - note: графа 9 — телефон (G_9_PHONE)

- financial.email:
  - value:
    rule: если master_data.financial.email есть → взять его, иначе master_data.consignee.email, иначе invoice.buyer.email
  - status: confirmed_primary | pending
  - note: графа 9 — e-mail (G_9_EMAIL)

---

### 5.7. Торгующая страна (графа 11)

- shipment.trade_country_code:
  - value: invoice.trade_country_code
  - status: confirmed_primary | pending
  - note: графа 11 — код торгующей страны alpha-2 (G_11_1)

---

### 5.8. Декларант (графа 14)
> Если master_data отсутствует и данные взяты из инвойса, фиксировать в разделе нерешенных вопросов.

- declarant.ogrn:
  - value:
    rule: если master_data.declarant.ogrn есть → взять его, иначе master_data.consignee.ogrn, иначе invoice.buyer.ogrn
  - status: confirmed_primary | pending
  - note: графа 14 — ОГРН декларанта (G_14_1)

- declarant.name_display:
  - value:
    rule: собрать строку печатного блока из declarant.name + адрес + контакты (как в эталоне)
  - status: derived | pending
  - source: master_data.declarant.*
  - note: графа 14 — текст в поле графы 14 в форме/печати (G_14/NAME)

- declarant.inn_kpp:
  - value:
    rule: если master_data.declarant.inn_kpp есть → взять его, иначе master_data.consignee.inn_kpp, иначе invoice.buyer.inn_kpp
  - status: confirmed_primary | pending
  - note: графа 14 — ИНН/КПП через "/" (G_14_4)

- declarant.name:
  - value:
    rule: если master_data.declarant.name есть → взять его, иначе master_data.consignee.name, иначе invoice.buyer.name
  - status: confirmed_primary | pending
  - note: графа 14 — наименование организации (G_14_NAM)

- declarant.country_code:
  - value:
    rule: если master_data.declarant.country_code есть → взять его, иначе master_data.consignee.country_code, иначе invoice.buyer.country_code
  - status: confirmed_primary | pending
  - note: графа 14 — код страны (G_14_CC)

- declarant.country_name:
  - value:
    rule: если master_data.declarant.country_name есть → взять его, иначе master_data.consignee.country_name, иначе invoice.buyer.country_name
  - status: confirmed_primary | pending
  - note: графа 14 — наименование страны (G_14_CN)

- declarant.postcode:
  - value:
    rule: если master_data.declarant.postcode есть → взять его, иначе master_data.consignee.postcode, иначе invoice.buyer.postcode
  - status: confirmed_primary | pending
  - note: графа 14 — почтовый индекс (G_14_POS)

- declarant.region:
  - value:
    rule: если master_data.declarant.region есть → взять его, иначе master_data.consignee.region, иначе invoice.buyer.region
  - status: confirmed_primary | pending
  - note: графа 14 — регион (G_14_SUB)

- declarant.city:
  - value:
    rule: если master_data.declarant.city есть → взять его, иначе master_data.consignee.city, иначе invoice.buyer.city
  - status: confirmed_primary | pending
  - note: графа 14 — населённый пункт (G_14_CIT)

- declarant.street:
  - value:
    rule: если master_data.declarant.street есть → взять его, иначе master_data.consignee.street, иначе invoice.buyer.street
  - status: confirmed_primary | pending
  - note: графа 14 — улица (G_14_STR)

- declarant.building:
  - value:
    rule: если master_data.declarant.building есть → взять его, иначе master_data.consignee.building, иначе invoice.buyer.building
  - status: confirmed_primary | pending
  - note: графа 14 — дом (G_14_BLD)

- declarant.room:
  - value:
    rule: если master_data.declarant.room есть → взять его, иначе master_data.consignee.room, иначе invoice.buyer.room
  - status: confirmed_primary | pending
  - note: графа 14 — помещение/офис (G_14_ROM)

- declarant.phone:
  - value:
    rule: если master_data.declarant.phone есть → взять его, иначе master_data.consignee.phone, иначе invoice.buyer.phone
  - status: confirmed_primary | pending
  - note: графа 14 — телефон (G_14_PHONE)

- declarant.email:
  - value:
    rule: если master_data.declarant.email есть → взять его, иначе master_data.consignee.email, иначе invoice.buyer.email
  - status: confirmed_primary | pending
  - note: графа 14 — e-mail (G_14_EMAIL)
  
---

### 5.9. Страны (графы 15, 16, 17)

- shipment.dispatch_country_name:
  - value: invoice.dispatch_country_name
  - status: confirmed_primary | pending
  - note: графа 15 — страна отправления, текст (G_15_1)

- shipment.dispatch_country_code:
  - value: invoice.dispatch_country_code
  - status: confirmed_primary | pending
  - note: графа 15A — код страны отправления alpha-2 (G_15A_1)

- shipment.origin_country_name:
  - value: invoice.origin_country_name
  - status: confirmed_primary | pending
  - note: графа 16 — страна происхождения, текст (G_16_1)

- shipment.origin_country_code:
  - value: invoice.origin_country_code
  - status: confirmed_primary | pending
  - note: графа 16 — код страны происхождения alpha-2 (G_16_2)

- shipment.destination_country_name:
  - value: invoice.destination_country_name
  - status: confirmed_primary | pending
  - note: графа 17 — страна назначения, текст (G_17_1)

- shipment.destination_country_code:
  - value: invoice.destination_country_code
  - status: confirmed_primary | pending
  - note: графа 17A — код страны назначения alpha-2 (G_17A_1)

---

### 5.10. Транспорт (графы 18, 19, 21)

- transport.mode_code_internal:
  - value: packing_list.transport_means_count
  - status: confirmed_primary | derived
  - note: графа 18 — количество транспортных средств (G_18_0); в эталоне "2" = тягач + прицеп

- transport.identification:
  - value:
    rule: собрать номера всех ТС через "/"
  - status: derived
  - source: packing_list.transport_means
  - note: графа 18 — идентификация ТС (G_18); тягач/прицеп через "/"

- transport.registration_country_code:
  - value:
    rule: в эталоне "00" — process-значение для автотранспорта
  - status: derived
  - note: графа 18 — код страны регистрации ТС (G_18_2)

- transport.container_flag:
  - value:
    rule: если перевозка без контейнера → 0
  - status: derived
  - note: графа 19 — признак контейнера (G_19_1)

- transport.border_mode:
  - value:
    rule: для автоперевозки → 1
  - status: derived
  - note: графа 21 — код активного ТС на границе (G_21_0)

---

### 5.11. Валюта и стоимость (графа 22)

- shipment.invoice_currency_numeric:
  - value:
    rule: преобразовать invoice.currency_alpha в numeric код ISO
  - status: derived
  - source: invoice.currency_alpha
  - note: графа 22 — цифровой код валюты (G_22_1)

- shipment.invoice_amount:
  - value: invoice.total_cost
  - status: confirmed_primary | pending
  - note: графа 22 — сумма по счёту (G_22_2)

- shipment.invoice_currency_alpha:
  - value: invoice.currency_alpha
  - status: confirmed_primary | pending
  - note: графа 22 — буквенный код валюты (G_22_3)

---

### 5.12. Курс валюты (графа 23)

- shipment.currency_rate:
  - value: invoice.currency_rate
  - status: confirmed_operator | pending
  - note: графа 23 — курс валюты к рублю на дату подачи (G_23_1, G_23_2)

---

### 5.13. Вид транспорта (графы 25, 26)

- transport.border_transport_code:
  - value:
    rule: для автотранспорта → 31
  - status: derived
  - note: графа 25 — код вида транспорта на границе (G_25_1)

- transport.internal_transport_code:
  - value:
    rule: для автотранспорта → 31 (совпадает с графой 25)
  - status: derived
  - note: графа 26 — код вида транспорта внутри страны (G_26_1)

---

### 5.14. Таможня на границе (графа 29)

- customs.border_code:
  - value:
    rule: определить по маршруту и данным CMR/ТД
  - status: derived | pending
  - source: cmr, td
  - note: графа 29 — код таможенного органа на границе (G_29_1); в эталоне 10719110

- customs.border_name:
  - value:
    rule: определить по коду таможни
  - status: derived | pending
  - note: графа 29 — наименование таможенного поста (G_29_2); в эталоне "Т/П МАПП ЗАБАЙКАЛЬСК"
  
---

### 5.15. Местонахождение товаров (графа 30)

- location.type:
  - value:
    rule: для СВХ → 11
  - status: derived
  - note: графа 30 — тип места нахождения товаров (G_30_0); 11 = склад временного хранения

- location.document_kind:
  - value:
    rule: для лицензии СВХ → 2
  - status: derived
  - note: графа 30 — вид документа, подтверждающего место хранения (G_30_10); 2 = свидетельство/лицензия

- location.document_number:
  - value: svh.warehouse_license_number
  - status: confirmed_primary | pending
  - note: графа 30 — номер документа СВХ (G_30_1); в эталоне номер лицензии 10404/141210/10092/5

- location.document_date:
  - value: svh.warehouse_license_date
  - status: confirmed_primary | pending
  - note: графа 30 — дата документа СВХ (G_30_DATE)

- location.address.country_code:
  - value:
    rule: для склада в РФ → RU
  - status: derived
  - note: графа 30 — код страны местонахождения товаров (G_30_CC)

- location.address.region:
  - value:
    rule: определить по адресу СВХ
  - status: derived | pending
  - source: svh
  - note: графа 30 — регион (G_30_SUB)

- location.address.city:
  - value:
    rule: определить по адресу СВХ
  - status: derived | pending
  - source: svh
  - note: графа 30 — город (G_30_CIT)

- location.address.street:
  - value:
    rule: определить по адресу СВХ
  - status: derived | pending
  - source: svh
  - note: графа 30 — улица и дом (G_30_STR)

- location.customs_code:
  - value:
    rule: определить по коду таможенного поста СВХ
  - status: derived | pending
  - source: svh
  - note: графа 30 — код таможенного органа, в зоне которого находится СВХ (G_30_12); в эталоне 10404083

- location.printed:
  - value:
    rule: собрать из type + ", " + customs_code + ", " + region + " " + city + " " + street + ", " + document_number + " ОТ " + document_date
  - status: derived
  - note: графа 30 — печатная строка местонахождения (G_30P_1); формируется автоматически
  
---

### 5.16 Товары (BLOCK, графы 31–47)
`goods` — массив товаров ДТ (каждый элемент соответствует одному `BLOCK` в XML). Правило агрегации 
(для построения `goods`):
- сгруппировать строки `invoice.InvoiceGoods_*` по коду ТН ВЭД (`GoodsCode`) + стране происхождения (если влияет);
- по каждой группе сформировать 1 товар ДТ (`goods[i]`);
- внутри товара сформировать таблицу `tovg[]` (повторяющиеся строки) из исходных строк инвойса в этой группе;
- веса/стоимости товара ДТ = суммы по строкам группы (по подтверждённым значениям из `primary.md`).

#### 5.16.1. Графа 31 — описание товаров (G_31)

- goods[i].g31.name:
  - value:
    rule: сформировать обобщённое описание группы товаров (как в эталоне) + "СМ.ДОПОЛНЕНИЕ"
  - status: derived | pending
  - source: goods[i].tovg
  - note: графа 31 — описание товара (G_31/NAME)

- goods[i].g31.manufacturer:
  - value:
    rule: если у всех строк группы один производитель → он, иначе "СМ.ДОПОЛНЕНИЕ"
  - status: derived | pending
  - source: goods[i].tovg.manufacturer
  - note: графа 31 — производитель (G_31/FIRMA)

- goods[i].g31.trademark:
  - value:
    rule: если у всех строк группы ТМ одинаковая → она, иначе "СМ.ДОПОЛНЕНИЕ"; если ТМ отсутствует → "ОТСУТСТВУЕТ"
  - status: derived | pending
  - source: goods[i].tovg.trade_mark
  - note: графа 31 — товарный знак / ТМ (G_31/TM)

- goods[i].g31.pl:
  - value:
    rule: оставить пустым, если не используется в кейсе
  - status: derived
  - note: графа 31 — служебное поле PL (G_31/PL)

- goods[i].g31.places:
  - value:
    rule: взять количество мест по группе (если есть разбиение в СВХ) иначе pending
  - status: derived | pending
  - source: svh.non_xml_fields, packing_list, invoice
  - note: графа 31 — кол-во мест по товару (G_31/PLACE)

#### 5.16.2. Графы 32–38 — код товара, страна, веса, процедура

- goods[i].item_no:
  - value:
    rule: порядковый номер товара в ДТ (1..N)
  - status: derived
  - note: графа 32 — номер товара (G_32_1)

- goods[i].tnved_code:
  - value:
    rule: общий код ТН ВЭД группы (из invoice.InvoiceGoods_*.GoodsCode)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.GoodsCode
  - note: графа 33 — код товара (G_33_1)

- goods[i].tnved.flag_1:
  - value:
    rule: значение-литера после кода (как в эталоне: "С"); если правила нет → pending
  - status: derived | pending
  - note: графа 33 — доп. признак (G_33_4)

- goods[i].tnved.flag_2:
  - value:
    rule: значение-литера после кода (как в эталоне: "N"); если правила нет → pending
  - status: derived | pending
  - note: графа 33 — доп. признак (G_33_5)

- goods[i].origin_country_code:
  - value:
    rule: alpha-2 страны происхождения товара (нормализовать из numeric/alpha в primary)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.OriginCountryCode
  - note: графа 34 — код страны происхождения (G_34_1)

- goods[i].gross_weight:
  - value:
    rule: сумма брутто по строкам группы
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.GrossWeightQuantity
  - note: графа 35 — вес брутто по товару (G_35_1)

- goods[i].preference:
  - value:
    rule: код преференции (в эталоне "ОООО-ОО"); если правила нет → pending
  - status: derived | pending
  - note: графа 36 — преференция (G_36_2)

- goods[i].procedure_code:
  - value:
    rule: как правило 4000000 для ИМ40 (как в эталоне); если не уверен → pending
  - status: derived | pending
  - source: declaration.direction, declaration.procedure
  - note: графа 37 — процедура по товару (G_37_1)

- goods[i].net_weight:
  - value:
    rule: сумма нетто по строкам группы
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.NetWeightQuantity
  - note: графа 38 — вес нетто по товару (G_38_1)

#### 5.16.3. Графы 42–46 — стоимости по товару

- goods[i].invoice_cost:
  - value:
    rule: сумма стоимости по инвойсу по строкам группы (валюта графы 22)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.TotalCost
  - note: графа 42 — цена товара (G_42_1)

- goods[i].mos_code_main:
  - value:
    rule: код МОС (в эталоне G_43_1=1); если правил нет → pending
  - status: derived | pending
  - note: графа 43 — код МОС (G_43_1)

- goods[i].mos_code_extra:
  - value:
    rule: доп. код МОС (в эталоне G_43_2=0); если правил нет → pending
  - status: derived | pending
  - note: графа 43 — доп. признак (G_43_2)

- goods[i].g44_text:
  - value:
    rule: "СМ.ДОПОЛНЕНИЕ" если документы перечисляются в дополнении
  - status: derived | pending
  - note: графа 44 — текстовое поле на основном листе (G_44)

- goods[i].customs_value:
  - value:
    rule: рассчитывается Альтой по ДТС; в dt_fields.md можно оставить pending/не заполнять
  - status: pending
  - note: графа 45 — таможенная стоимость (G_45_0, G_45_1)

- goods[i].statistical_value:
  - value:
    rule: рассчитывается Альтой; если требуется — derived по правилам Альты, иначе pending
  - status: pending
  - note: графа 46 — статистическая стоимость (G_46_1)

#### 5.16.4. Графа 47 — исчисление платежей (по товару)

- goods[i].payments[k].payment_code:
  - value:
    rule: вид платежа (например 1010/2010/5010)
  - status: derived | pending
  - note: графа 47 — вид платежа (G_47_*_*_1). См. cb:payment

- goods[i].payments[k].tax_base:
  - value:
    rule: база начисления
  - status: derived | pending
  - note: графа 47 — основа начисления (G_47_*_*_2)

- goods[i].payments[k].rate:
  - value:
    rule: ставка (может быть % или фикс, как "4924РУБ.")
  - status: derived | pending
  - note: графа 47 — ставка (G_47_*_*_3)

- goods[i].payments[k].amount:
  - value:
    rule: сумма платежа
  - status: derived | pending
  - note: графа 47 — сумма (G_47_*_*_4)

- goods[i].payments[k].payment_method:
  - value:
    rule: способ уплаты (в эталоне "ИУ"); если правил нет → pending
  - status: derived | pending
  - note: графа 47 — СП (G_47_*_*_5)

#### 5.16.5. Дополнение к графе 31 — TXT (детальные строки)

- goods[i].txt[j].text:
  - value:
    rule: сформировать строки дополнения к графе 31 из goods[i].tovg (как в эталоне: "АРТ: - {qty} {unit}" + описание)
  - status: derived | pending
  - source: goods[i].tovg
  - note: графа 31 — строки дополнения (TXT/TEXT)

#### 5.16.6. Таблица описания — TOVG (строки внутри товара)

- goods[i].tovg[j].line_no:
  - value:
    rule: порядковый номер строки внутри товара (1..M)
  - status: derived
  - note: графа 31 — № строки таблицы (TOVG/G32G)

- goods[i].tovg[j].description:
  - value:
    rule: описание строки (как в инвойсе + нормализация/перевод при наличии)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.GoodsDescription
  - note: графа 31 — наименование (TOVG/G31_1)

- goods[i].tovg[j].manufacturer:
  - value:
    rule: производитель (из primary)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_Manufacturer
  - note: графа 31 — производитель (TOVG/G31_11)

- goods[i].tovg[j].trade_mark:
  - value:
    rule: ТМ (из primary; если отсутствует → "ОТСУТСТВУЕТ")
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_TradeMark
  - note: графа 31 — марка/ТМ (TOVG/G31_12)

- goods[i].tovg[j].goods_mark:
  - value:
    rule: товарный знак/маркировка (если отсутствует → "ОТСУТСТВУЕТ")
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_GoodsMark
  - note: графа 31 — товарный знак (TOVG/G31_14)

- goods[i].tovg[j].model:
  - value:
    rule: модель/модификация (из primary; в эталоне типа "АНТИКОТ 1.4 * 30")
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.AdditionalGoodsDescription_GoodsModel + размеры из описания
  - note: графа 31 — модель (TOVG/G31_15_MOD)

- goods[i].tovg[j].quantity:
  - value:
    rule: количество по строке инвойса
  - status: confirmed_primary | pending
  - source: invoice.InvoiceGoods_*.GoodsQuantity
  - note: графа 31 — количество (TOVG/KOLVO)

- goods[i].tovg[j].unit_code:
  - value:
    rule: код единицы измерения Альты (например 055 для м2)
  - status: derived | pending
  - source: invoice.InvoiceGoods_*.MeasureUnitQualifierName
  - note: графа 31 — код ЕИ (TOVG/CODE_EDI)

- goods[i].tovg[j].unit_name:
  - value:
    rule: наименование единицы измерения (например "М2")
  - status: derived | pending
  - note: графа 31 — ЕИ (TOVG/NAME_EDI)

- goods[i].tovg[j].gross_weight:
  - value:
    rule: брутто по строке
  - status: confirmed_primary | pending
  - source: invoice.InvoiceGoods_*.GrossWeightQuantity
  - note: графа 35 — вес брутто по строке (TOVG/G31_35)

- goods[i].tovg[j].net_weight:
  - value:
    rule: нетто по строке
  - status: confirmed_primary | pending
  - source: invoice.InvoiceGoods_*.NetWeightQuantity
  - note: графа 38 — вес нетто по строке (TOVG/G31_38)

- goods[i].tovg[j].invoice_cost:
  - value:
    rule: стоимость по строке инвойса
  - status: confirmed_primary | pending
  - source: invoice.InvoiceGoods_*.TotalCost
  - note: графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST)

#### 5.16.7. Графа 44 — документы по товару (повторяющиеся G44)

- goods[i].documents[n].doc_code:
  - value:
    rule: код вида документа (например 04021)
  - status: derived | pending
  - source: primary.formalized.*
  - note: графа 44 — код документа (G44/G441)

- goods[i].documents[n].doc_number:
  - value:
    rule: номер документа
  - status: derived | pending
  - note: графа 44 — номер (G44/G442)

- goods[i].documents[n].doc_reg_number:
  - value:
    rule: регистрационный номер документа (если есть, как G442R в эталоне), иначе пусто
  - status: derived
  - note: графа 44 — рег. номер (G44/G442R)

- goods[i].documents[n].doc_date:
  - value:
    rule: дата документа YYYY-MM-DD
  - status: derived | pending
  - note: графа 44 — дата (G44/G443)

- goods[i].documents[n].doc_name:
  - value:
    rule: наименование/тип документа (для печати)
  - status: derived | pending
  - note: графа 44 — наименование (G44/G444)

- goods[i].documents[n].back_flag:
  - value:
    rule: 1 если документ приложен, иначе 0
  - status: derived | pending
  - note: графа 44 — BACK (G44/BACK)

- goods[i].documents[n].ed_type:
  - value:
    rule: тип электронного документа (как в эталоне ED_TYP); если неизвестно → pending
  - status: derived | pending
  - note: графа 44 — ED_TYP

- goods[i].documents[n].ed_id:
  - value:
    rule: идентификатор в архиве Альты (ED_ID); если неизвестно → оставить пустым
  - status: derived
  - note: графа 44 — ED_ID

- goods[i].documents[n].doc_text:
  - value:
    rule: строка для печати (как DOCTEXT в эталоне)
  - status: derived | pending
  - note: графа 44 — текст для печати (G44/DOCTEXT)

---

### 5.17. Теги после товаров (графы 51–54)

#### 5.17.1. Графа 42 (доп. признак)

- declaration.g42_2:
  - value:
    rule: как в эталоне "В ДТС" (если применяется); иначе pending
  - status: derived | pending
  - note: графа 42 — доп. признак (G_42_2)

#### 5.17.2. Графа 51–53 (транзит/гарантия/назначение) — обычно пусто

- transit.placeholder:
  - value:
    rule: не заполнять, если в кейсе нет транзита/гарантии
  - status: derived
  - note: графы 51–53 — в большинстве ИМ40 не заполняются (G_51_5, G_53)

#### 5.17.3. Графа 54 — уполномоченное лицо / представитель

- representative.date:
  - value: master_data.representative.date
  - status: confirmed_primary | pending
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
  - value:
    rule: собрать печатную строку как в эталоне (ФИО + паспорт + роль + контакты + доверенность)
  - status: derived | pending
  - source: representative.*
  - note: графа 54 — печатный блок (G_54P)

---

## 6. Раздел unresolved_questions

### Для конкретного поля
- `<FIELD_PATH>`:
  - `question`: `<конкретный вопрос>`
  - `impact`: `<что блокирует>`
  - `requested_from`: `<operator | stage_1_fix>`

### Общие вопросы
- `[General]`:
  - `question`: `<вопрос>`
  - `impact`: `<что блокирует>`
  - `requested_from`: `<operator | stage_1_fix>`

---

## 10. Генерация dt.xml (встроено в stage 2)

После формирования `dt_fields.md` AI генерирует `dt.xml` по правилам `doc_xml_schema.md`-стиля:

- XML декларация: `<?xml version="1.0" encoding="windows-1251"?>`
- root: `<AltaGTD>...</AltaGTD>`
- скаляры → теги;
- вложенные объекты → вложенные узлы;
- массивы `BLOCK[i]`, `G44[n]`, `TXT[j]`, `TOVG[k]`, `G_47[p]` → повторяющиеся узлы
  (суффиксы индексов не включаются в имена XML-тегов);
- обязательное XML-экранирование (`& < > " '`);
- даты: `YYYY-MM-DD`;
- числа — строковым представлением без принудительного округления.

Если значение `pending` или статус `pending` для поля, без которого нельзя строить корректный XML:
- фиксировать блокер в `review_2.md`,
- либо по явной просьбе оператора генерировать skeleton/partial XML с пометками.

---

## 11. Выходные файлы (stage_2_result)

- `dt_fields.md`
- `review_2.md`

---

## 12. Чек-лист перед записью

- ✅ прочитан `primary.md` текущего кейса
- ✅ все поля схемы материализованы в `dt_fields.md`
- ✅ для каждого derived поля описано правило в `note` или через `conditional_fields`
- ✅ все pending вынесены в `unresolved_questions`
- ✅ `dt.xml` сгенерирован в кодировке windows-1251
- ✅ в `review_2.md` есть сводка pending/блокеров

---

## 8. РАЗДЕЛ XX: Нерешенные вопросы

Здесь размещаются все вопросы к оператору:
- по конкретным полям
- общие

### Формат записи:

**Для поля:**
- `<UQI поля со статусом pending>`
  - `question`: <текст вопроса AI>

**Для общего вопроса:**
- `[Общий]`
  - `question`: <текст вопроса AI>

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


Полные версии справочников находятся в `alta\prompt\codebook.md`.
Полные справочники — в `alta\prompt\codebook.md`.

