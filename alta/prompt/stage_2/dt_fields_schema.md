# Stage 2.0 — подготовка полей ДТ

## 0. Части файла `dt_fields_schema.md`

1. Часть 1. Workflow

2. Часть 2. Правила шаблонов

3. Часть 3. Шаблоны полей ДТ

4. Часть 4. Формат `dt_fields.md`

5. Часть 5. Правила работы

6. Часть 6. Порядок работы (задание)

7. ПРИЛОЖЕНИЕ. Вырезки из справочников

## 1. Workflow

Вход этапа:
  - `alta\result\<ИмяКейса>\primary.md` - факты поставки,
  - короткие версии справочников в этой схеме,
  - `alta\prompt\codebook.md` - полные справочники.
  - НЕ читает первичные документы поставки напрямую.

Выход этапа:
  - `alta\result\<ИмяКейса>\dt_fields.md` (источник истины для stage 2.1),
  - `alta\result\<ИмяКейса>\review\dt_fields_review.md` — краткий отчет.

Этап 2.0, используя факты только из `primary.md`, 
- рассчитывает производные значения по явно описанным правилам,
- `dt_fields.md` - это полный набор полей ДТ по этой схеме, не описание, а формальная база данных, вход 
  для этапа 2.1. Если отсутствует хотя бы одно поле из раздела "Шаблоны полей ДТ" — это ошибка этапа. AI обязан 
  фиксировать все пробелы/конфликты в части `issues` файла `dt_fields.md`, откуда они попадут в `dt_fields_review.md`

---

## Часть 2. Правила шаблонов
- **Нумерация полей разделов (жесткие индексы):**
  - Все поля в разделах пронумерованы в формате `NN: field_name`, начиная с 1 без пропусков.
  - Массивы не считаются полями, их элементы имеют свою внутреннюю нумерацию полей.
  - Индексы полей совпадают 1:1 в схеме и `dt_fields.md`.
  - В конце раздела выводится контрольная строка:
    - `_audit: <N>`, где N - общее число полей в разделе, должно совпасть с номером последнего поля. 

- **Нумерация полей элементов плоских массивов:**
  - Все поля пронумерованы в формате `NN: field_name`, начиная с 1 без пропусков.
  - Индексы полей совпадают 1:1 в схеме и `dt_fields.md`.
  - В конце элемента массива выводится контрольная строка:
    - `_item_audit: <N>`, где N - общее число полей в разделе, должно совпасть с номером последнего поля.

- **Нумерация элементов плоских массивов:**
  - сразу после заголовка элемента массива идет строка с его номером:
    - `_element_num: <i>`, где i - номер текущего элемента.

- **Аудит массивов:**
- Сразу после заголовка массива идет контрольная строка `_array_audit: <N>`, где N - размерность массива. Для массивов, 
  имеющих вложенные массивы, идентификатор квалифицируется именем массива, для плоских массивов квалификация отсутствует:
  - массив с подмассивами: `<имя_массива>._array_audit: <N>`, например `goods._array_audit: 2`,
  - плоский массив: `_array_audit: <N>`, например, для goods[i].tovg[j]: `_array_audit: 5`.
- Служебное поле аудита массива идет **в заголовке массива**, после вывода последнего элемента поля аудита числа элементов
  **НЕ** предусмотрено.


#### 2.0.1 Сокращения
- Сокращения для полей здесь, в `dt_fields_schema.md` и в `dt_fields.md`:
  - `V` = `value` - значение, при материализации берется как есть 
  - `VR` = `value.rule` - правило или ссылка на поле со значением. При материализации разрешается.
  - `S` = `status` - <CD|CO|D|pending>
  - `N` = `note` - пояснение к полю. В схеме обязательное и детальное, в `dt_fields.md` опциональное и редко используемое,
    чтобы не раздувать файл.

Номера полей в схеме мапируются 1:1 на номера полей в `dt_fields.md`, полная связность. 

#### 2.0.2 Статус поля в `primary.md`
`S` (`status`):
  - `CD` - confirmed_document, подтвержденное значение, взято из исходных документов,
  - `CO` - confirmed_operator, значение явно задано оператором,
  - `pending` — данных недостаточно или есть конфликт.

---

## Часть 3. Шаблоны полей ДТ

### 3.1 Заголовок декларации (графа 1)

- 01: declaration.direction:
  - VR: meta.direction
  - S: CD | pending
  - N: направление декларации (импорт / экспорт) (G_1_1)

- 02: declaration.procedure:
  - value: 40
  - S: CO | pending
  - N: код таможенной процедуры. Требует подтверждения оператора. Значение из cb:procedure. (G_1_2)

- 03: declaration.form:
  - V: ЭД
  - S: D | pending
  - N: форма подачи декларации; для Альты всегда ЭД (G_1_31)

- _audit: 3 

---

### 3.2 Отправитель (графа 2)

- 01: sender.country_name:
  - VR: formalized.invoice_1.Seler_PostalAddress_CounryName
  - S: CP | pending
  - N: текстовое название страны (G_2_50)

- 02: sender.country_code:
  - VR: formalized.invoice_1.Seler_PostalAddress_CountryCode
  - S: CP | pending
  - N: код страны alpha-2 (G_2_7)

- 03: sender.name:
  - VR: formalized.invoice_1.Seler_Name
  - S: CP | pending
  - N: полное наименование отправителя (G_2_NAM)

- 04: sender.region:
  - VR: formalized.invoice_1.Seler_PostalAddress_Region
  - S: CP | pending
  - N: область/район (G_2_SUB)

- 05: sender.city:
  - VR: formalized.invoice_1.Seler_PostalAddress_City
  - S: CP | pending
  - N: город (G_2_CIT)

- 06: sender.street:
  - VR: formalized.invoice_1.Seler_PostalAddress_StreetHouse
  - S: CP | pending
  - N: улица и дом (G_2_STR)

- _audit: 6

---

### 3.3 Количество товаров и мест (графы 5, 6)

- 01: shipment.total_goods_number:
  - VR: размер массива goods
  - S: D | pending
  - N: гр. 5 — количество товарных позиций в ДТ (G_5_1)

- 02: shipment.packages_flag:
  - VR: всегда true (места считаются)
  - S: D
  - N: гр. 6 — признак подсчёта мест (G_6_0)

- 03: shipment.total_packages:
  - VR: взять подтверждённое количество мест по приоритету:
    svh.actual_places → packing_list.places_total → invoice.places_quantity
  - S: D | pending
  - N: гр. 6 — общее количество грузовых мест (G_6_1)

- _audit: 3

---

### 3.4 Получатель (графа 8)
Если consignee.same_as_declarant=true, то все поля графы 8, включая пустоту, и их статусы, включая pending, копируются 
из графы 14 (декларант).

- 01: consignee.same_as_declarant:
  - VR: true (константа)
  - S: D
  - N: графа 8 — признак «см. графу 14» (G_8_SM14)

- 02: consignee.ogrn:
  - N: графа 8 — ОГРН получателя (G_8_1)

- 03: consignee.inn_kpp:
  - N: графа 8 — ИНН/КПП через "/" (G_8_6)

- 04: consignee.name:
  - N: графа 8 — наименование организации (G_8_NAM)

- 05: consignee.country_code:
  - N: графа 8 — код страны alpha-2 (G_8_7)

- 06: consignee.country_name:
  - N: графа 8 — страна, наименование (G_8_50)

- 07: consignee.postcode:
  - N: графа 8 — почтовый индекс (G_8_POS)

- 08: consignee.region:
  - N: графа 8 — регион (G_8_SUB)

- 09: consignee.city:
  - N: графа 8 — населённый пункт (G_8_CIT)

- 10: consignee.street:
  - N: графа 8 — улица (G_8_STR)

- 11: consignee.building:
  - N: графа 8 — дом (G_8_BLD)

- 12: consignee.room:
  - N: графа 8 — помещение/офис (G_8_ROM)

- 13: consignee.phone:
  - N: графа 8 — телефон (G_8_PHONE)

- 14: consignee.email:
  - N: графа 8 — e-mail (G_8_EMAIL)
    
- _audit: 14

---

### 3.5 Финансовое урегулирование (графа 9) — как “см. графу 14”
Если financial.same_as_declarant=true, то все поля графы 9, включая пустоту, и их статусы, включая pending, копируются
из графы 14 (декларант). Исключение - financial.country_code_alt, копируется из declarant.country_code.

- 01: financial.same_as_declarant:
  - VR: true (константа)
  - S: D
  - N: графа 9 — признак «см. графу 14» (G_9_SM14)

- 02: financial.ogrn:
  - N: графа 9 — ОГРН (G_9_1)

- 03: financial.inn_kpp:
  - N: графа 9 — ИНН/КПП (G_9_4)

- 04: financial.name:
  - N: графа 9 — наименование (G_9_NAM)

- 05: financial.country_code:
  - N: графа 9 — код страны (G_9_CC)

- 06: financial.country_name:
  - N: графа 9 — наименование страны (G_9_CN)

- 07: financial.postcode:
  - N: графа 9 — индекс (G_9_POS)

- 08: financial.region:
  - N: графа 9 — регион (G_9_SUB)

- 09: financial.city:
  - N: графа 9 — город (G_9_CIT)

- 10: financial.street:
  - N: графа 9 — улица (G_9_STR)

- 11: financial.building:
  - N: графа 9 — дом (G_9_BLD)

- 12: financial.room:
  - N: графа 9 — помещение (G_9_ROM)

- 13: financial.country_code_alt:
  - VR: declarant.country_code
  - S: declarant.status
  - N: графа 9 — дублирующий код страны (G_9_7)

- 14: financial.phone:
  - N: графа 9 — телефон (G_9_PHONE)

- 15: financial.email:
  - N: графа 9 — e-mail (G_9_EMAIL)

- _audit: 15

---

### 3.6 Торгующая страна (графа 11)

- 01: shipment.trade_country_code:
  - VR: formalized.invoice_1.DeliveryTerms_TradingCountryCode
  - S: CP | pending
  - N: графа 11 — код торгующей страны alpha-2 (G_11_1)

- _audit: 1

---

### 3.7 Декларант (графа 14)

- 01: declarant.ogrn:
  - VR: master_data.egrul.OGRN
  - S: CP | pending
  - N: графа 14 — ОГРН декларанта (G_14_1)

- 02: declarant.inn_kpp:
  - VR: master_data.egrul.INN + "/" + master_data.egrul.KPP
  - S: D | pending
  - N: графа 14 — ИНН/КПП через "/" (G_14_4)

- 03: declarant.name:
  - VR: master_data.egrul.OrganizationName
  - S: CP | pending
  - N: графа 14 — наименование организации (G_14_NAM)

- 04: declarant.country_code:
  - VR: master_data.egrul.Address_CountryCode
  - S: CP | pending
  - N: графа 14 — код страны (G_14_CC)

- 05: declarant.country_name:
  - VR: master_data.egrul.Address_CounryName
  - S: CP | pending
  - N: графа 14 — наименование страны (G_14_CN)

- 06: declarant.postcode:
  - VR: master_data.egrul.Address_PostalCode
  - S: CP | pending
  - N: графа 14 — почтовый индекс (G_14_POS)

- 07: declarant.region:
  - VR: master_data.egrul.Address_Region
  - S: CP | pending
  - N: графа 14 — регион (G_14_SUB)

- 08: declarant.city:
  - VR: master_data.egrul.Address_City
  - S: CP | pending
  - N: графа 14 — населённый пункт (G_14_CIT)

- 09: declarant.street:
  - VR: master_data.egrul.Address_StreetHouse
  - S: CP | pending
  - N: графа 14 — улица (G_14_STR)

- 10: declarant.building:
  - VR: master_data.egrul.Address_StreetHouse
  - S: CP | pending
  - N: графа 14 — дом (G_14_BLD)

- 11: declarant.room:
  - VR: извлечь офис/помещение из master_data.egrul.Address_StreetHouse, если отдельно не задано.
  - S: D | pending
  - N: графа 14 — помещение/офис (G_14_ROM)

- 12: declarant.phone:
  - VR: master_data.egrul.Phone
  - S: CP | pending
  - N: графа 14 — телефон (G_14_PHONE)

- 13: declarant.email:
  - VR: master_data.egrul.Email
  - S: CP | pending
  - N: графа 14 — e-mail (G_14_EMAIL)

- _audit: 13

---

### 3.8 Страны (графы 15, 16, 17)

- 01: shipment.dispatch_country_code:
  - VR: formalized.invoice_1.DeliveryTerms_DispatchCountryCode
  - S: CP | pending
  - N: графа 15A — код страны отправления alpha-2 (G_15A_1)

- 02: shipment.destination_country_code:
  - VR: formalized.invoice_1.DeliveryTerms_DestinationCountryCode
  - S: CP | pending
  - N: графа 17A — код страны назначения alpha-2 (G_17A_1)

- 03: shipment.dispatch_country_name:
  - VR: получить наименование страны по shipment.dispatch_country_code через cb:country
  - S: D | pending
  - N: графа 15 — страна отправления, текст (G_15_1)

- 04: shipment.destination_country_name:
  - VR: получить наименование страны по shipment.destination_country_code через cb:country
  - S: D | pending
  - N: графа 17 — страна назначения, текст (G_17_1)

- 05: shipment.origin_country_code:
  - VR: если у всех InvoiceGoods[*] один OriginCountryCode → нормализовать в alpha-2 через 
    cb:country (numeric/alpha-2/alpha-3), иначе pending
  - S: D | pending
  - N: графа 16 — код страны происхождения alpha-2 (G_16_2)

- 06: shipment.origin_country_name:
  - VR: получить наименование страны по shipment.origin_country_code через cb:country
  - S: D | pending
  - N: графа 16 — страна происхождения, текст (G_16_1)

- _audit: 6

---

### 3.9 Условия поставки (графа 20)

- 01: delivery.terms_code:
  - VR: приоритет источников:
    formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode →
    formalized.packing_list.DeliveryTerms_DeliveryTermsStringCode
  - S: D | pending
  - N: графа 20 — условия поставки (G_20_20)

- 02: delivery.place_name:
  - VR: приоритет источников: formalized.invoice_1.DeliveryTerms_DeliveryPlace →
    formalized.packing_list.DeliveryTerms_DeliveryPlace
  - S: D | pending
  - N: графа 20 — место поставки (G_20_21)

- _audit: 2

---

### 3.10 Транспорт (графы 18, 19, 21)
Правило: Автоперевозка = если в primary.md присутствует formalized.cmr_1 (документ найден).

- 01: transport.vehicles_count:
  - VR: количество ТС = число блоков TransportMeans_* в formalized.packing_list
  - S: D | pending
  - N: графа 18 — количество транспортных средств (G_18_0)

- 02: transport.identification:
  - VR: приоритет источников:
    join(formalized.packing_list.TransportMeans_*.Number, "/") → non_formalized.td_1.transport_reg_number →
    non_formalized.svh.transport_reg_number
  - S: D | pending
  - N: графа 18 — идентификация ТС (G_18)

- 03: transport.registration_country_code:
  - VR: если formalized.packing_list.TransportMeans_1.NationalityCode = "000" → "00", иначе взять как есть;
    если данных нет → pending
  - S: D | pending
  - N: графа 18 — код страны регистрации ТС (G_18_2)

- 04: transport.container_flag:
  - VR: 0; константа
  - S: CO | pending
  - N: графа 19 — признак контейнера (G_19_1): 0 / 1 (перевозка без /в контейнере); пока заглушка 0.

- 05: transport.border_mode:
  - VR: если автоперевозка → 1, иначе pending
  - S: D | pending
  - N: графа 21 — код активного ТС на границе (G_21_0)

- _audit: 5

---

### 3.11 Валюта и стоимость (графа 22)

- 01: shipment.invoice_currency_numeric:
  - VR: если formalized.invoice_1.CurrencyCode содержит alpha-3, преобразовать в numeric код ISO 
    по `cd:currency_okv`; если содержит код, оставить как есть.
  - S: D | pending
  - N: графа 22 — цифровой код валюты (G_22_1)

- 02: shipment.invoice_currency_alpha:
  - VR: formalized.invoice_1.CurrencyCode (возможно, содержит numeric код ISO)
  - S: CP | pending
  - N: графа 22 — буквенный код валюты (G_22_3)

- 03: shipment.invoice_amount:
  - VR: formalized.invoice_1.TotalCost
  - S: CP | pending
  - N: графа 22 — сумма по счёту (G_22_2)

- _audit: 3

---

### 3.12 Вид транспорта (графы 25, 26)

- 01: transport.border_transport_code:
  - VR: для автотранспорта → 31
  - S: D
  - N: графа 25 — код вида транспорта на границе (G_25_1)

- 02: transport.internal_transport_code:
  - VR: для автотранспорта → 31 (совпадает с графой 25)
  - S: D
  - N: графа 26 — код вида транспорта внутри страны (G_26_1)

- _audit: 2

---

### 3.13 Таможня на границе (графа 29)

- 01: customs.border_code:
  - VR: non_formalized.td.customs_post_code
  - S: CP | pending
  - N: графа 29 — код таможенного органа на границе (G_29_1); источник: ТД (09013)

- 02: customs.border_name:
  - VR: non_formalized.td.customs_post_name
  - S: CP | pending
  - N: графа 29 — наименование таможенного поста (G_29_2); источник: ТД (09013)

- _audit: 2

---

### 3.14 Местонахождение товаров (графа 30)

- 01: location.type:
  - VR: для СВХ → 11
  - S: D
  - N: графа 30 — тип места нахождения товаров (G_30_0); 11 = склад временного хранения

- 02: location.document_kind:
  - VR: для лицензии СВХ → 2
  - S: D
  - N: графа 30 — вид документа, подтверждающего место хранения (G_30_10); 2 = свидетельство/лицензия

- 03: location.document_number:
  - VR: non_formalized.svh.warehouse_license_number
  - S: CP | pending
  - N: графа 30 — номер документа СВХ (G_30_1)

- 04: location.document_date:
  - VR: non_formalized.svh.warehouse_license_date
  - S: CP | pending
  - N: графа 30 — дата документа СВХ (G_30_DATE)

- 05: location.address.country_code:
  - VR: для склада в РФ → RU
  - S: D
  - N: графа 30 — код страны местонахождения товаров (G_30_CC)

- 06: location.address.region:
  - VR: non_formalized.svh_additional_sheet_1.svh_address_region
  - S: CP | pending
  - N: графа 30 — регион (G_30_SUB)

- 07: location.address.city:
  - VR: non_formalized.svh_additional_sheet_1.svh_address_city
  - S: CP | pending
  - N: графа 30 — город (G_30_CIT)

- 08: location.address.street:
  - VR: non_formalized.svh_additional_sheet_1.svh_address_street_house
  - S: CP | pending
  - N: графа 30 — улица и дом (G_30_STR)

- 09: location.customs_code:
  - VR: non_formalized.svh_additional_sheet_1.svh_customs_code
  - S: CP | pending
  - N: графа 30 — код таможенного органа, в зоне которого находится СВХ (G_30_12)

- _audit: 09

---

### 3.15 Массив товаров goods[] (BLOCK, графы 31–47)
- Описание структуры массива товаров goods:
  - `goods[]` — массив товаров ДТ (каждый элемент = один `BLOCK` в XML и один товар в интерфейсе Альты).

  - Описание элементов массива товаров разбито на подблоки `3.16.1 Графа 31`, `3.16.2 Графы 32–38` и т.д. 
    _item_audit по ним считаются отдельно. 

  - массив `goods[]` имеет подмассивы:
    - goods[i].txt[] - массив строк, содержит текстовое описание единиц товара, имеющих один `tnved_code`.
    - goods[i].tovg[] - массив полей, содержит формализованное описание единиц товара, имеющих один `tnved_code`.
    - элементы массивов goods[i].txt[j] и goods[i].tovg[j], имеющие одинаковые индексы [i,j] описывают одну и ту же
      единицу товара.
    - подмассив goods[i].g44_docs[k] - массив полей, описывает документы, относящиеся к товару [i]. 

- Правило агрегации единиц в товар (строго):
  - взять все строки `invoice.InvoiceGoods[*]`;
  - сгруппировать строки по `GoodsCode`; `GoodsCode` = значение `invoice.InvoiceGoods[*].GoodsCode` 
    (оно же заполняет `goods[i].tnved_code`, графа 33);
  - число элементов `goods` должно быть равно числу уникальных `GoodsCode`;
  - различия между строками внутри одного `GoodsCode` (артикул/вид/модель/описание/количество) НЕ создают новый элемент 
    `goods[i]`, они отражаются в подмассивах `goods[i].tovg[]` и/или `goods[i].txt[]`;
  - веса/стоимости товара ДТ = суммы по строкам группы.

- Материализация этого раздела в `dt_fields.md`:
   - представлен заголовком `### 3.15 Массив goods[]`,
   - имеет служебное поле `goods._array_audit: N`.

Материализация заголовка содержит служебное поле `goods._array_audit: <N>`:
где N - реальная размерность массива goods[]. Нужно для аудита числа элементов массива. Имя поля квалифицировано 
именем массива. Такая квалификация используется только для массивов, имеющих вложенные массивы.
```dt_fields
### 3.15 Массив: goods[]
- goods._array_audit: <N>
```

#### 3.15.0 Элемент массива товаров goods[i]
Материализация заголовка содержит служебное поле `goods._element_num: <i>`, где i - 1-based индекс элемента.
```dt_fields
#### 3.15.0 Элемент массива: goods[i]
- goods._element_num: <i>
```

#### 3.15.1 Графа 31 — описание товаров (G_31)
Часть полей элемента массива goods[i], для аудита выглядит как обычный раздел. То же 3.15.2, 3.15.3.

- 01: goods[i].g31.name:
  - VR: non_formalized.goods_description_[n].description + "СМ.ДОПОЛНЕНИЕ"
  - S: D | pending
  - N: графа 31 — описание товара (G_31/NAME). "ДОПОЛНЕНИЕ" в `goods[i].txt[]` / `goods[i].tovg[]`.

- 02: goods[i].g31.manufacturer:
  - VR: если у всех строк группы один производитель → он, иначе "СМ.ДОПОЛНЕНИЕ"
  - S: D | pending
  - N: графа 31 — производитель (G_31/FIRMA)

- 03: goods[i].g31.trade_mark:
  - VR: если у всех строк группы ТМ одинаковая → она, иначе "СМ.ДОПОЛНЕНИЕ"; если ТМ отсутствует → "ОТСУТСТВУЕТ"
  - S: D | pending
  - N: графа 31 — товарный знак / ТМ (G_31/TM)

- 04: goods[i].places:
  - VR: non_formalized.svh.goods_[n].places, где non_formalized.svh.goods_[n].tnved == goods[i].tnved_code.value
  - S: D | pending
  - N: графа 31 — количество мест по товару (G_31/PLACE)

- _audit: 4

#### 3.15.2 Графы 32–38 — код товара, страна, веса, процедура

- 01: goods[i].item_no:
  - VR: порядковый номер товара в ДТ (1..N)
  - S: D
  - N: графа 32 — номер товара (G_32_1)

- 02: goods[i].tnved_code:
  - VR: код ТН ВЭД товара ДТ = `invoice.InvoiceGoods[*].GoodsCode` для этой группы
  - S: D | pending
  - N: графа 33 — код товара (G_33_1)

- 03: goods[i].tnved.flag_1:
  - VR: `С`; константа
  - S: D | pending
  - N: графа 33 — доп. признак (G_33_4): `С`, если не требуются разрешительные документы; иначе пусто (не заполняется);
    сейчас `С` - заглушка.

- 04: goods[i].tnved.flag_2:
  - VR: `N` нет торговой марки; иначе пусто
  - S: D | pending
  - N: графа 33 — доп. признак (G_33_5)

- 05: goods[i].origin_country_code:
  - VR: alpha-2 страны происхождения товара (нормализовать OriginCountryCode numeric/alpha-2/alpha-3 → alpha-2 
    через cb:country)
  - S: D | pending
  - N: графа 34 — код страны происхождения (G_34_1)

- 06: goods[i].gross_weight:
  - VR: приоритет источников брутто по товару:
    non_formalized.svh.goods_[n].gross_weight_kg (по tnved) → сумма invoice.InvoiceGoods[*].GrossWeightQuantity по группе
  - S: D | pending
  - N: графа 35 — вес брутто по товару (G_35_1)

- 07: goods[i].preference:
  - VR: `ОООО-ОО` - отсутствие преференций; константа
  - S: D | pending
  - N: графа 36 — преференция (G_36_2)

- 08: goods[i].net_weight:
  - VR: сумма нетто по строкам группы
  - S: D | pending
  - N: графа 38 — вес нетто по товару (G_38_1)

- _audit: 8

#### 3.15.3 Графы 42–46 — исходные данные стоимости по товару
Поля НЕ используются для генерации dt.xml и статусы не являются блокерами. Зарезервированы на будущее для проверки
общей таможенной стоимости.

- 01: goods[i].transport_cost:
  - VR: formalized.service_invoice.transport_to_border * (goods[i].gross_weight / sum(goods[*].gross_weight))
  - S: D | pending
  - N: доля транспортных расходов до границы ЕАЭС по товару i (пропорционально весу брутто)

- 02: goods[i].transport_currency:
  - VR: formalized.service_invoice.transport_currency
  - S: CP | pending
  - N: валюта транспортных расходов

- 03: goods[i].insurance_cost:
  - VR: formalized.insurance_invoice.insurance_to_border * (goods[i].invoice_cost / sum(goods[*].invoice_cost))
  - S: D | pending
  - N: доля расходов на страхование по товару i (пропорционально фактурной стоимости товара)

- 04: goods[i].insurance_currency:
  - VR: formalized.insurance_invoice.insurance_currency
  - S: CP | pending
  - N: валюта страхования

- _audit: 4


#### 3.15.4 Дополнение к графе 31 — TXT (детальные строки)
`goods[i].txt[]` — массив строк дополнения к графе 31 (описание товара); `txt[j]` соответствует `tovg[j]` (1:1).
Правило маппинга: Чтобы сформировать элементы goods[i].txt[j] и goods[i].tovg[j], AI должен найти в primary.md 
ту строку invoice.InvoiceGoods[k], у которой dt_item_index == i и dt_tovg_index == j.

Материализация:
```dt_fields
#### 3.15.4 Массив: goods[i].txt[]
- _array_audit: <N>
```

#### 3.15.5 Элемент массива goods[i].txt[j]
Материализация:
```dt_fields
#### 3.15.5 Элемент массива: goods[i].txt[j]
- _element_num: <j>
```

- 01: goods[i].txt[j].line_1:
  - VR: `"АРТ: - " + goods[i].tovg[j].quantity + " " + goods[i].tovg[j].unit_name`
  - S: D | pending
  - N: графа 31 — TXT строка 1 (арт/кол-во/ед)

- 02: goods[i].txt[j].line_2:
  - VR: goods[i].tovg[j].description
  - S: D | pending
  - N: графа 31 — TXT строка 2 (наименование)

- _item_audit: 2

#### 3.15.6 Таблица описания — TOVG
`goods[i].tovg[]` — массив групп товаров, графа 31 (таблица). См. правило маппинга, п.3.16.4.

Материализация:
```dt_fields
#### 3.15.6 Массив: goods[i].tovg[j]
- _array_audit: <N>
```

#### 3.15.7 Элемент массива goods[i].tovg[j]
Материализация:
```dt_fields
#### 3.15.7 Элемент массива: goods[i].tovg[j]
- _element_num: <j>
```

- 01: goods[i].tovg[j].line_no:
  - VR: порядковый номер позиции внутри товара (1..M)
  - S: D
  - N: графа 31 — № строки таблицы (TOVG/G32G)

- 02: goods[i].tovg[j].description:
  - VR: описание позиции, как в инвойсе + нормализация/перевод при наличии
  - S: D | pending
  - N: графа 31 — наименование (TOVG/G31_1)

- 03: goods[i].tovg[j].manufacturer:
  - VR: производитель из инвойса
  - S: D | pending
  - N: графа 31 — производитель (TOVG/G31_11)

- 04: goods[i].tovg[j].trade_mark:
  - VR: ТМ из инвойса; если отсутствует → "ОТСУТСТВУЕТ"
  - S: D | pending
  - N: графа 31 — марка/ТМ (TOVG/G31_12)

- 05: goods[i].tovg[j].goods_mark:
  - VR: товарный знак/маркировка из инвойса, если отсутствует → "ОТСУТСТВУЕТ"
  - S: D | pending
  - N: графа 31 — товарный знак (TOVG/G31_14)

- 06: goods[i].tovg[j].model:
  - VR: модель/модификация из инвойса; при наличии размеров/параметров — включить их в модель.
  - S: D | pending
  - N: графа 31 — модель/модификация (TOVG/G31_15_MOD)

- 07: goods[i].tovg[j].quantity:
  - VR: invoice.InvoiceGoods[k].goods_supplementary_quantity (где dt_item_index == i и dt_tovg_index == j)
  - S: CP | pending
  - N: графа 31 — количество в доп.ед.изм (TOVG/KOLVO)

- 08: goods[i].tovg[j].unit_code:
  - VR: найти код ЕИ по cb:unit по наименованию invoice.InvoiceGoods[j].goods_supplementary_uom_name 
    (где dt_item_index == i и dt_tovg_index == j)
  - S: D | pending
  - N: графа 31 — код ЕИ (TOVG/CODE_EDI)

- 09: goods[i].tovg[j].unit_name:
  - VR: invoice.InvoiceGoods[j].goods_supplementary_uom_name (где dt_item_index == i и dt_tovg_index == j)
  - S: CP | pending
  - N: графа 31 — наименование ЕИ (TOVG/NAME_EDI)

- 10: goods[i].tovg[j].gross_weight:
  - VR: брутто по строке инвойса
  - S: CP | pending
  - N: графа 35 — вес брутто по строке (TOVG/G31_35)

- 11: goods[i].tovg[j].net_weight:
  - VR: нетто по строке инвойса
  - S: CP | pending
  - N: графа 38 — вес нетто по строке (TOVG/G31_38)

- 12: goods[i].tovg[j].invoice_cost:
  - VR: стоимость по строке инвойса
  - S: CP | pending
  - N: графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST)
  
- _item_audit: 12

---

#### 3.15.8 Графа 44 — представляемые документы
В графу 44 включаются все документы, имеющие признак `doc_gr44: true`.

#### 3.15.9 Поле G_44 (текстовое поле в карточке товара)

- 01: goods[i].g44.text:
  - VR: СМ.ДОПОЛНЕНИЕ; константа
  - S: D
  - N: графа 44 — текстовое поле (G_44)

- _audit: 1

#### 3.15.10 Таблица документов (массив записей графы 44)
`goods[i].g44_docs[k]` — массив документов, подлежащих представлению.

Материализация:
```dt_fields
#### 3.15.10 Массив: goods[i].g44_docs[k]
- _array_audit: <N>
```

#### 3.15.11 Элемент массива goods[i].g44_docs[k]
Для каждого формализуемого документа - одна запись `goods[i].g44_docs[k]`.
Материализация:
```dt_fields
#### 3.15.11 Элемент массива: goods[i].g44_docs[k]
- _element_num: <k>
```

- 01: goods[i].g44_docs[k].doc_code:
  - VR: `<документ>.doc_code.value`
  - S: CP | pending
  - N: графа 44 — код документа (G44/G441), см. cb:doc

- 02: goods[i].g44_docs[k].kind_code:
  - V: 0  
  - S: CO
  - N: графа 44 — признак записи (G44/G4403); `0` - документ прикладывается к ДТ; значение корректируется оператором в Альте

- 03: goods[i].g44_docs[k].doc_name:
  - VR: `<документ>.doc_name`
  - S: CP | pending
  - N: графа 44 — наименование документа (G44/G444)

- 04: goods[i].g44_docs[k].doc_number:
  - VR: `<документ>.doc_number`
  - S: CP | pending
  - N: графа 44 — номер документа (G44/G442)

- 05: goods[i].g44_docs[k].doc_date:
  - VR: `<документ>.doc_date`
  - S: CP | pending
  - N: графа 44 — дата документа (G44/G443)

- _item_audit: 5

---

### 3.16 Теги после товаров и документов (графы 51–54)

#### 3.16.1 Графа 54 — уполномоченное лицо / представитель

- 01: representative.date:
  - VR: <текущая дата> - дата заполнения/подачи ДТ
  - S: D | pending
  - N: графа 54 — дата заполнения/подачи (G_54_20)

- 02: representative.phone:
  - VR: master_data.passport.Phone
  - S: CP | pending
  - N: графа 54 — телефон (G_54_21)

- 03: representative.email:
  - VR: master_data.passport.Email
  - S: CP | pending
  - N: графа 54 — e-mail (G_54_EMAIL)

- 04: representative.last_name:
  - VR: master_data.passport.PersonSurname
  - S: CP | pending
  - N: графа 54 — фамилия (G_54_3)

- 05: representative.first_name:
  - VR: master_data.passport.PersonName
  - S: CP | pending
  - N: графа 54 — имя (G_54_3NM)

- 06: representative.middle_name:
  - VR: master_data.passport.PersonMiddleName
  - S: CP | pending
  - N: графа 54 — отчество (G_54_3MD)

- 07: representative.authority_doc_name:
  - VR: master_data.letter_of_attorney.doc_name
  - S: CP | pending
  - N: графа 54 — документ полномочий (G_54_4)

- 08: representative.authority_doc_number:
  - VR: master_data.letter_of_attorney.DocumentNumber
  - S: CP | pending
  - N: графа 54 — № документа полномочий (G_54_5)

- 09: representative.authority_doc_date_from:
  - VR: master_data.letter_of_attorney.DocumentDate
  - S: CP | pending
  - N: графа 54 — дата начала действия (G_54_60)

- 10: representative.authority_doc_date_to:
  - VR: master_data.letter_of_attorney.EndDate
  - S: CP | pending
  - N: графа 54 — дата окончания действия (G_54_61)

- 11: representative.position:
  - VR: master_data.letter_of_attorney.EmpoweredPost
  - S: CP | pending
  - N: графа 54 — должность/статус (G_54_7)

- 12: representative.passport_code:
  - V: RU01001
  - S: CP | pending
  - N: графа 54 — код документа удостоверения личности (G_54_8)

- 13: representative.passport_name:
  - V: `ПАСРФ`
  - S: CP | pending
  - N: графа 54 — наименование документа (G_54_9)

- 14: representative.passport_number:
  - VR: master_data.passport.CardNumber
  - S: CP | pending
  - N: графа 54 — номер паспорта (G_54_100)

- 15: representative.passport_date:
  - VR: master_data.passport.CardDate
  - S: CP | pending
  - N: графа 54 — дата выдачи паспорта (G_54_101)

- 16: representative.passport_series:
  - VR: master_data.passport.CardSeries
  - S: CP | pending
  - N: графа 54 — серия паспорта (G_54_12)

- 17: representative.passport_issuer:
  - VR: master_data.passport.OrganizationName
  - S: CP | pending
  - N: графа 54 — кем выдан (G_54_13)

- _audit: 17

---

## Часть 4. Формат `dt_fields.md`
primary.md — обычный Markdown файл. В этой схеме примеры фрагментов разметки приводятся в fenced blocks (```),
но в самом primary.md fenced blocks использовать не нужно.

### 4.1 Части `dt_fields.md`
1) Метаданные
2) Часть I: Поля ДТ
   Часть разбита на разделы, соответствующие схеме, которые оформлены таблицами, например:
   - `### 1. Заголовок декларации (графа 1)`,
   - `### 2. Отправитель (графа 2)` и т.д.

4) Часть II: Issues (нерешенные вопросы)

```primary
# Исходные данные для ДТ

## Метаданные:
- `название кейса`: <название кейса>
- `путь к папке поставки`: <путь к папке поставки>
- `тип поставки`: <например: 1 ДТ / 1 товар>
- `агрегация ДТ`: определяется правилами stage 2.0
- `источники данных:` <например: primary.md + operator_provided_data>

## Часть I: Поля ДТ
```

### 4.2 Таблицы полей
Правила таблицы:
  - AI обязан материализовать все поля, указанные в шаблоне документа;
  - для пустых значений полей ячейка таблицы остается пустой;
  - если для поля не удалось установить значение, status=pending;

- Структура таблицы:
  - **ДЛЯ КАЖДОГО РАЗДЕЛА/ЭЛЕМЕНТА МАССИВА СХЕМЫ СТРОИТСЯ ОТДЕЛЬНАЯ ТАБЛИЦА**; Это необходимо для работы проверочного скрипта, 
    так как в после каждой таблицы есть разметочная строка `_audit`/`_item_audit`.
  - таблицы, соответствуют разделам шаблонов в схеме, например, 
    - `### 4.1. Заголовок декларации`,
    - `### 4.2. Отправитель (графа 2)` и т.д.
  - Перед заголовком таблицы выводится пустая строка (требование маркдауна).

- Сокращения:
`S` (`status`):
  - `CP` - confirmed_primary, подтвержденное значение, взято напрямую из `primary.md`,
  - `CO` - confirmed_operator, значение явно задано оператором,
  - `D` - derived, значение вычислено по правилу из подтвержденных данных,
  - `pending` — данных недостаточно или есть конфликт.
Если `status: pending`, то `value` пустое.

- **Нумерация и контроль потерь (жесткие индексы полей):**
  - Все поля в шаблонах пронумерованы в формате `NN: field_name`, начиная с 1 без пропусков.
  - Индексы полей совпадают 1:1 в схеме и `dt_fields.md`. AI **ОБЯЗАН** подставлять эти номера в таблицы полей 
    в том же порядке.
  - В конце каждого документа и в конце каждого массива AI **ОБЯЗАН** поместить взятые из схемы строки 
    (необходимо для скрипта проверки):
    - `_audit` (для документа),
    - `_item_audit` (для массива).

- Таблицы строятся для каждого раздела, например:
  - `### Заголовок декларации (графа 1)`, 
  - `### Отправитель (графа 2)`,
- и для каждого элемента массива, например:
  - `#### 3.16.7 Элемент массива goods[i].tovg[j]`,

- Формат таблицы:
```primary
пустая строка
| num                | field       | value             | status            | description       | note             |
|--------------------|-------------|-------------------|-------------------|-------------------|------------------|
| <порядковый номер> | <имя поля>  | <value или пусто> | <CP/CO/D/pending> | <назначение поля> | <note или пусто> |
```

### 4.3 Реквизиты полей
Реквизиты полей документов получают следующие значения:
- `value`: <значение> | `link`: <ссылка на первичный документ>
- `description`: краткое описание, извлекается из реквизитов `N` (`note`) шаблонов. Например,
  шаблон `09: location.customs_code`, реквизит 
  ``- N: графа 30 — код таможенного органа, в зоне которого находится СВХ (G_30_12)``,
  здесь `description` = `код таможенного органа`.
- `note`: особенности, тонкости, замечания (не заполняется в режиме быстрой генерации, умолчание).

### 4.4 Для массивов
Индексы массивов **ВСЕГДА 1-BASED**.


#### 4.4.1 Заголовок массива
**!!!ВАЖНО ДЛЯ СКРИПТА ПРОВЕРКИ!!!** - выводится опорная строка `goods._array_audit: <N>`,
где N - реальная размерность массива. Имя поля квалифицировано именем массива. Такая квалификация 
используется только для массивов, имеющих вложенные массивы.

Пример для массивов, содержащих вложенные массивы:
```dt_fields
### 3.15 Массив: goods[]
- goods._array_audit: <N>
```

Пример для вложенных массивов:
```dt_fields
#### 3.15.6 Массив: goods[i].tovg[j]
- _array_audit: <N>
```

#### 4.4.2 Заголовок элемента массива
**!!!ВАЖНО ДЛЯ СКРИПТА ПРОВЕРКИ!!!** - выводится опорная строка:
  - _element_num: <i> (где i - порядковый номер элемента массива, начиная с 1)

Пример для массивов, содержащих вложенные массивы:
```dt_fields
#### 3.15.0 Элемент массива: goods[i]
- goods._element_num: <i>
пустая строка
следующая таблица
```

Пример для вложенных массивов:
```dt_fields
#### 3.15.7 Элемент массива: goods[i].tovg[j]
- _element_num: <j>
пустая строка
таблица полей этого элемента указанным выше форматом
```

#### 4.4.3 После каждого элемента массива:

```primary
- _item_audit: N
```

### 4.5 После каждого раздела (НЕ массива):
**!!!ВАЖНО ДЛЯ СКРИПТА ПРОВЕРКИ!!!** - выводится опорная строка:
- _audit: <N> (N - число полей раздела, взятое из шаблона).

Пример:
```primary
- _audit: <N>
```

### 4.6 Итого, по файлу:
Полный итог:
```primary
### Итог:
  - `dt_status`: <confirmed / pending>
```

## Часть II: Issues (нерешенные вопросы)

```primary
### Часть II: Issues (нерешенные вопросы)

Формат для полей:
- `<UQI поля со статусом pending>`:
  - `question`: <текст вопроса>

Для общих вопросов:
- `[Общий]`:
  - `question`: <текст вопроса>
```

---

## Раздел 5. Правила сборки

### 5.0 **ЖЕСТКОЕ ПРАВИЛО**
Поля в шаблонах идут с возрастающими на единицу номерами. В `dt_fields.md` генерируй их с теми же номерами, 
номер **ВСЕГДА** возрастает на единицу.

### 5.1 Запрет сокращений и симуляции:
  - **ЗАПРЕЩЕНА СОКРАЩЕННАЯ СБОРКА `dt_fields.md`**. Запрещено исключение разделов и полей, описанных в шаблонах.

### 5.2 Никаких догадок
Запрещено:
  - подставлять коды “на глаз”, используй справочники;
  - использовать как источник фактов новой поставки:
    - `alta\reference\...`, `...\выгрузки\...` (эталонные ДТ/xml/скриншоты);
    - результаты прошлых прогонов (`dt_fields.md`, `dt.xml`).
    - использовать динамические скрипты powershell. Где возможно, обходись средствами Хобота, иначе, согласуй с оператором.

AI обязан:
  - **`dt_fields.md` — формальная база данных**. Все поля и все массивы, предусмотренные шаблоном документа, обязаны
    присутствовать, даже если `status: pending`.
  - **Никаких догадок:** Если поле отсутствует или неоднозначно и в шаблоне не описаны другие действия, то `value` пустое,
    `status` = `pending` и фиксация в разделе нерешенных вопросов.
  - выводить производные значения по явно записанному правилу;
  - `value` - всегда материализуется, содержит готовое значение поля ДТ (строку/число/дату), а не ссылку.

### 5.3 Разное
Подгружай `codebook.md` **ТОЛЬКО** если недостаточно данных в вырезках справочников внутри схемы. Сообщай оператору,
чтобы он мог пополнить вырезки.

### 5.4 Проверка скриптами полностью/частично сформированного файла `dt_fields.md`
Скрипты просматривают файл построчно от начала к концу и анализируют **ТОЛЬКО** индексы полей и маркеры. Маркеры:
  - `_audit`, `_item_audit` **ФИНАЛИЗИРУЮТ ПОДСЧЕТ ПОЛЕЙ**,
  - а, маркер `_array_audit` **ИНИЦИИРУЕТ ПОДСЧЕТ ЭЛЕМЕНТОВ МАССИВА**, то есть ожидает встретить столько маркеров
    `_element_num`, сколько ожидается элементов массива.
  - Оба скрипта игнорируют остальной текст файла, в том числе заголовки разделов/массивов.

- Скрипт `alta\service\script\gen_result_fields_audit.bat alta\result\<ИмяКейса>\dt_fields.md` проверит:
  - корректность строк таблиц (разделители '|', количество колонок)
  - корректность нумерации num (01..N без пропусков) внутри каждого блока
  - сверка фактического числа полей с маркерами _audit/_item_audit
  - некоторые ошибки форматирования:
    - перед заголовком таблицы есть пустая строка
    - внутри таблицы нет пустых строк (пустая строка считается концом таблицы)
  - **Важно:** скрипт корректно проверит только файл (полный или незаконченный), который завершен строкой 
    `_audit`/`_item_audit`.

- Скрипт `alta\service\script\gen_result_full_audit.bat alta\result\<ИмяКейса>\dt_fields.md`. Он:
  - выполнит все предыдущие проверки,
  - проверит, что для всех массивов, аннотированных `_array_audit: <N>`, материализовано ровно N элементов. 
  - **Важно:** скрипт корректно проверит только файл (полный или незаконченный), который завершен строкой
    `_audit`/`_item_audit` **И ЗАКОНЧИЛ МАТЕРИАЛИЗАЦИЮ ВСЕХ МАССИВОВ, КОТОРЫЕ НАЧАЛ МАТЕРИАЛИЗОВЫВАТЬ**.

### 5.5 Проверка на наличие статусов pending
- После завершения генерации или любого изменения файлов `dt_fields.md` AI обязан запустить проверку:
  `alta\service\script\check_pendings.bat <путь_к_файлу>`

Скрипт проверит поля и выдаст все строки таблиц, содержащие `pending`.

---

## Раздел 6. Порядок работы (задание)

### 6.1 Разведка
  - Вспомни ограничения сайта.
  - Если уже существует `dt_fields.md`, `dt_fields_review.md`, запроси у оператора нужна ли доработка или генерация с нуля?

### 6.2 Подготовка
  - прочитай `primary.md` одной командой Хобота (если позволяют ограничения сайта).

### 6.3 Генерация 
сгенерируй/доработай `dt_fields.md`:
  - Не используй плейсхолдеры: Пиши чанками: write_file, затем `write_file "utf-8" "append"`. 
    Заканчивай каждый чанк строкой `_audit`/`_item_audit`. Это необходимо для правильной работы проверочного скрипта.

  - `_array_audit`, `_element_num` **НЕ ЯВЛЯЮТСЯ ЗАКОННЫМИ ТОЧКАМИ ОСТАНОВКИ**, т.к. они начинают/продолжают блоки 
    проверки, а не финализируют их.

  - При каждой записи `dt_fields.md`, целиком/части/завершении, **в этой же директиве AI ОБЯЗАН ЗАПУСТИТЬ ПРОВЕРОЧНЫЙ
    СКРИПТ `gen_result_fields_audit.bat`**, см. п. 5.4.

  - Используй patch_file, для точечных фиксов.
  
### 6.4 Чек-лист:

#### 6.4.1 Начало генерации:

  - ✅ Проверь, что мета-данные заполнены.

#### 6.4.2 В цикле генерации:

  - Перед выполнением каждой директивы записи/дозаписи `dt_fields.md`:
    - ✅ Проверь совпадение имен полей со схемой.
    - ✅ Проверь, содержит ли директива вызов скрипта `gen_result_fields_audit.bat`.
    - ✅ Все поля материализованы согласно шаблонам, включая массивы и подмассивы.

  - После после выполнения директивы записи/дозаписи `dt_fields.md`:
    - ✅ если выявлены ошибки, найди `_audit`/`_item_audit` последнего правильно сгенерированного раздела и перегенерируй
      хвост файла.
    - ✅ Пройди по всем полям со статусом `pending`, проверь, что поля, действительно, нельзя определить.

### 6.4.3 Завершение
  1) ✅ Запусти полную проверку `gen_result_full_audit.bat` (выполнит аудит полей разделов и аудит массивов).
  2) ✅ проверь, что были материализованы **ВСЕ** разделы шаблонов ДТ из схемы.
  3) **ВЫПОЛНИ:** `alta\service\script\check_pendings.bat <путь_к_dt_fields.md>`. Скрипт найдет все pending-поля.
  4) Совместно с оператором разреши вопросы pending-полей, если есть, выполни патчи/перегенерацию.
  5) **Зафиксируй ответы в `operator_provided_data.md`**.
  6) Сгенерируй `dt_fields_review.md`.

---

## Приложение. Вырезки из справочников
**Полные справочники** — в `alta\prompt\codebook.md`.

### Идентификаторы справочников (codebook)

| Идентификатор     | Описание |
|-------------------|----------|
| `cb:currency_okv` | Классификатор валюты |
| `cb:procedure`    | Классификатор таможенных процедур |
| `cb:regime`       | Режимы движения товаров (ИМ/ЭК) |
| `cb:country`      | Страны и их коды |
| `cb:unit`         | Единицы измерения |
| `cb:doc`          | Коды видов документов |
| `cb:payment`      | Виды платежей и способы расчётов |
| `cb:transport`    | Виды и режимы транспорта |
| `cb:location`     | Типы местонахождения товаров |

---

### `cb:currency_okv` - Коды, обозначения, наименования валют

| numeric_code | alpha-3 | name |
|---:|:---:|---|
| 643 | RUB | Российский Рубль |
| 156 | CNY | Китайский юань |
| 978 | EUR | ЕВРО |
| 840 | USD | Доллары США |

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


