## Метаданные:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `тип поставки`: 1 ДТ / 7 товаров
- `агрегация ДТ`: 2 товара ДТ (группировка по коду ТН ВЭД: 5804101000, 7019900095)
- `источники данных:` primary.md + operator_provided_data + stable_source (xml)

## Раздел I: Поля ДТ

### 3.1. Заголовок декларации

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | declaration.direction | ИМ | CP | meta.direction | графа 1.1 — направление декларации (импорт / экспорт) (G_1_1) | |
| 02 | declaration.procedure | 40 | CO | cb:procedure | графа 1.2 — код таможенной процедуры. Требует подтверждения оператора. Значение из cb:procedure. (G_1_2) | |
| 03 | declaration.form | ЭД | D | | графа 1.31 — форма подачи декларации; для Альты всегда ЭД (G_1_31) | |

#### Итого, по разделу:
- `fields`: 3 из 3
- `partition_status`: confirmed

### 3.2. Отправитель (графа 2)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | sender.country_name | КИТАЙ | CP | formalized.invoice_1.Seler_PostalAddress_CounryName | графа 2 — текстовое название страны (G_2_50) | |
| 02 | sender.country_code | CN | CP | formalized.invoice_1.Seler_PostalAddress_CountryCode | графа 2 — код страны alpha-2 (G_2_7) | |
| 03 | sender.name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | formalized.invoice_1.Seler_Name | графа 2 — полное наименование отправителя (G_2_NAM) | |
| 04 | sender.region | Hebei | CP | formalized.invoice_1.Seler_PostalAddress_Region | графа 2 — область/район (G_2_SUB) | |
| 05 | sender.city | Shijiazhuang | CP | formalized.invoice_1.Seler_PostalAddress_City | графа 2 — город (G_2_CIT) | |
| 06 | sender.street | No. 5 Gaodong street | CP | formalized.invoice_1.Seler_PostalAddress_StreetHouse | графа 2 — улица и дом (G_2_STR) | |

#### Итого, по разделу:
- `fields`: 6 из 6
- `partition_status`: confirmed

### 3.3. Количество товаров и мест (графы 5, 6)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | shipment.total_goods_number | 2 | D | goods | графа 5 — количество товарных позиций в ДТ (G_5_1) | |
| 02 | shipment.packages_flag | true | D | | графа 6 — признак подсчёта мест (G_6_0) | |
| 03 | shipment.total_packages | 127 | D | non_formalized.svh_additional_sheet_1.actual_places | графа 6 — общее количество грузовых мест (G_6_1) | приоритет #1: svh.actual_places=127, источник: ДО доп.лист |

#### Итого, по разделу:
- `fields`: 3 из 3
- `partition_status`: confirmed

### 3.4. Получатель (графа 8)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | consignee.ogrn | 1201600020390 | CP | formalized.invoice_1.Consignee_OGRN | графа 8 — ОГРН получателя (G_8_1) | |
| 02 | consignee.name_display |  | D | consignee.name | графа 8 — текст в поле «Получатель» в форме/печати (G_8/NAME) | будет заполнено после вычисления same_as_declarant |
| 03 | consignee.country_name | РОССИЯ | CP | formalized.invoice_1.Buyer_PostalAddress_CounryName | графа 8 — страна, наименование (G_8_50) | |
| 04 | consignee.inn_kpp | 1650389298/165001001 | D | formalized.invoice_1.Buyer_CompanyID; formalized.invoice_1.Buyer_KPPCode | графа 8 — ИНН/КПП через "/" (G_8_6) | |
| 05 | consignee.country_code | RU | CP | formalized.invoice_1.Buyer_PostalAddress_CountryCode | графа 8 — код страны alpha-2 (G_8_7) | |
| 06 | consignee.name | ООО «СКИФ» | CP | formalized.invoice_1.Buyer_Name | графа 8 — наименование организации (G_8_NAM) | |
| 07 | consignee.postcode | 423800 | CP | formalized.invoice_1.Buyer_PostalAddress_PostalCode | графа 8 — почтовый индекс (G_8_POS) | |
| 08 | consignee.region | Республика Татарстан | CP | formalized.invoice_1.Buyer_PostalAddress_Region | графа 8 — регион (G_8_SUB) | |
| 09 | consignee.city | Набережные Челны | CP | formalized.invoice_1.Buyer_PostalAddress_City | графа 8 — населённый пункт (G_8_CIT) | |
| 10 | consignee.street | проезд Хлебный, дом 30, офис 211 | CP | formalized.invoice_1.Buyer_PostalAddress_StreetHouse | графа 8 — улица (G_8_STR) | |
| 11 | consignee.building | 30 | D | formalized.invoice_1.Buyer_PostalAddress_StreetHouse | графа 8 — дом (G_8_BLD) | извлечено из StreetHouse |
| 12 | consignee.room | 211 | D | formalized.invoice_1.Buyer_PostalAddress_StreetHouse | графа 8 — помещение/офис (G_8_ROM) | извлечено из StreetHouse |
| 13 | consignee.same_as_declarant | true | D | consignee.inn_kpp; declarant.inn_kpp | графа 8 — признак «см. графу 14» (G_8_SM14) | ИНН/КПП совпадают: 1650389298/165001001 |
| 14 | consignee.phone |  | pending | | графа 8 — телефон (G_8_PHONE) | в первичных документах отсутствует. птп |
| 15 | consignee.email |  | pending | | графа 8 — e-mail (G_8_EMAIL) | в первичных документах отсутствует. птп |

#### Итого, по разделу:
- `fields`: 15 из 15
- `partition_status`: pending

### 3.5. Финансовое урегулирование (графа 9) — как «см. графу 14»

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | financial.same_as_declarant | true | D | | графа 9 — признак «см. графу 14» (G_9_SM14) | |
| 02 | financial.name_display | СМ. ГРАФУ 14 ДТ | D | | графа 9 — текст в поле графы 9 в форме/печати (G_9/NAME) | |
| 03 | financial.ogrn | 1201600020390 | D | declarant.ogrn | графа 9 — ОГРН (G_9_1) | |
| 04 | financial.inn_kpp | 1650389298/165001001 | D | declarant.inn_kpp | графа 9 — ИНН/КПП (G_9_4) | |
| 05 | financial.name | ООО «СКИФ» | D | declarant.name | графа 9 — наименование (G_9_NAM) | |
| 06 | financial.country_code | RU | D | declarant.country_code | графа 9 — код страны (G_9_CC) | |
| 07 | financial.country_name | РОССИЯ | D | declarant.country_name | графа 9 — наименование страны (G_9_CN) | |
| 08 | financial.postcode | 423800 | D | declarant.postcode | графа 9 — индекс (G_9_POS) | |
| 09 | financial.region | РЕСПУБЛИКА ТАТАРСТАН | D | declarant.region | графа 9 — регион (G_9_SUB) | |
| 10 | financial.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | declarant.city | графа 9 — город (G_9_CIT) | |
| 11 | financial.street | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | D | declarant.street | графа 9 — улица (G_9_STR) | |
| 12 | financial.building | 30 | D | declarant.building | графа 9 — дом (G_9_BLD) | |
| 13 | financial.room | 211 | D | declarant.room | графа 9 — помещение (G_9_ROM) | |
| 14 | financial.country_code_alt | RU | D | declarant.country_code | графа 9 — дублирующий код страны (G_9_7) | |
| 15 | financial.phone |  | pending | declarant.phone | графа 9 — телефон (G_9_PHONE) | значение pending, т.к. declarant.phone pending |
| 16 | financial.email |  | pending | declarant.email | графа 9 — e-mail (G_9_EMAIL) | значение pending, т.к. declarant.email pending |

#### Итого, по разделу:
- `fields`: 16 из 16
- `partition_status`: pending

### 3.6. Торгующая страна (графа 11)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | shipment.trade_country_code | CN | CP | formalized.invoice_1.DeliveryTerms_TradingCountryCode | графа 11 — код торгующей страны alpha-2 (G_11_1) | |

#### Итого, по разделу:
- `fields`: 1 из 1
- `partition_status`: confirmed

### 3.7. Декларант (графа 14)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | declarant.ogrn | 1201600020390 | CP | formalized.letter_of_attorney_1.Organization_OGRN | графа 14 — ОГРН декларанта (G_14_1) | |
| 02 | declarant.name_display | ООО «СКИФ», 423800, РОССИЯ, РЕСПУБЛИКА ТАТАРСТАН, НАБЕРЕЖНЫЕ ЧЕЛНЫ, ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | D | declarant.name; declarant.postcode; declarant.country_name; declarant.region; declarant.city; declarant.street | графа 14 — текст в поле графы 14 в форме/печати (G_14/NAME) | контакты (телефон, e-mail) pending, в name_display не включены |
| 03 | declarant.inn_kpp | 1650389298/165001001 | D | formalized.letter_of_attorney_1.Organization_INN; formalized.letter_of_attorney_1.Organization_KPP | графа 14 — ИНН/КПП через "/" (G_14_4) | |
| 04 | declarant.name | ООО «СКИФ» | CP | formalized.letter_of_attorney_1.Organization_OrganizationName | графа 14 — наименование организации (G_14_NAM) | |
| 05 | declarant.country_code | RU | CP | formalized.letter_of_attorney_1.Organization_Address_CountryCode | графа 14 — код страны (G_14_CC) | |
| 06 | declarant.country_name | РОССИЯ | CP | formalized.letter_of_attorney_1.Organization_Address_CounryName | графа 14 — наименование страны (G_14_CN) | |
| 07 | declarant.postcode | 423800 | CP | formalized.letter_of_attorney_1.Organization_Address_PostalCode | графа 14 — почтовый индекс (G_14_POS) | |
| 08 | declarant.region | РЕСПУБЛИКА ТАТАРСТАН | CP | formalized.letter_of_attorney_1.Organization_Address_Region | графа 14 — регион (G_14_SUB) | |
| 09 | declarant.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | formalized.letter_of_attorney_1.Organization_Address_City | графа 14 — населённый пункт (G_14_CIT) | |
| 10 | declarant.street | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | CP | formalized.letter_of_attorney_1.Organization_Address_StreetHouse | графа 14 — улица (G_14_STR) | |
| 11 | declarant.building | 30 | D | formalized.letter_of_attorney_1.Organization_Address_StreetHouse | графа 14 — дом (G_14_BLD) | извлечено из StreetHouse |
| 12 | declarant.room | 211 | D | formalized.letter_of_attorney_1.Organization_Address_StreetHouse | графа 14 — помещение/офис (G_14_ROM) | извлечено из StreetHouse |
| 13 | declarant.phone |  | pending | | графа 14 — телефон (G_14_PHONE); требуется решение оператора | в доверенности и ЕГРЮЛ отсутствует. птп |
| 14 | declarant.email |  | pending | | графа 14 — e-mail (G_14_EMAIL); требуется решение оператора | в доверенности и ЕГРЮЛ отсутствует. птп |

#### Итого, по разделу:
- `fields`: 14 из 14
- `partition_status`: pending

### 3.8. Страны (графы 15, 16, 17)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | shipment.dispatch_country_code | CN | CP | formalized.invoice_1.DeliveryTerms_DispatchCountryCode | графа 15A — код страны отправления alpha-2 (G_15A_1) | |
| 02 | shipment.destination_country_code | RU | CP | formalized.invoice_1.DeliveryTerms_DestinationCountryCode | графа 17A — код страны назначения alpha-2 (G_17A_1) | |
| 03 | shipment.dispatch_country_name | КИТАЙ | D | shipment.dispatch_country_code; cb:country | графа 15 — страна отправления, текст (G_15_1) | |
| 04 | shipment.destination_country_name | РОССИЯ | D | shipment.destination_country_code; cb:country | графа 17 — страна назначения, текст (G_17_1) | |
| 05 | shipment.origin_country_code | CN | D | formalized.invoice_1.InvoiceGoods_*.OriginCountryCode; cb:country | графа 16 — код страны происхождения alpha-2 (G_16_2) | все товары: OriginCountryCode=156 → CN |
| 06 | shipment.origin_country_name | КИТАЙ | D | shipment.origin_country_code; cb:country | графа 16 — страна происхождения, текст (G_16_1) | |

#### Итого, по разделу:
- `fields`: 6 из 6
- `partition_status`: confirmed

### 3.9. Условия поставки (графа 20)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | delivery.terms_code | EXW | D | formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode | графа 20 — условия поставки (G_20_20) | приоритет #1: invoice |
| 02 | delivery.place_name | HEBEI | D | formalized.invoice_1.DeliveryTerms_DeliveryPlace | графа 20 — место поставки (G_20_21) | приоритет #1: invoice |

#### Итого, по разделу:
- `fields`: 2 из 2
- `partition_status`: confirmed

### 3.10. Транспорт (графы 18, 19, 21)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | transport.vehicles_count | 2 | D | formalized.packing_list_1.TransportMeans_* | графа 18 — количество транспортных средств (G_18_0) | TransportMeans_1 + TransportMeans_2 |
| 02 | transport.identification | О157АО774/ВТ374974 | D | formalized.packing_list_1.TransportMeans_*.Number | графа 18 — идентификация ТС (G_18) | join через "/" |
| 03 | transport.registration_country_code | 00 | D | formalized.packing_list_1.TransportMeans_1.NationalityCode | графа 18 — код страны регистрации ТС (G_18_2) | NationalityCode=000 → 00 |
| 04 | transport.container_flag | 0 | D | | графа 19 — признак контейнера (G_19_1) | перевозка без контейнера |
| 05 | transport.border_mode | 1 | D | | графа 21 — код активного ТС на границе (G_21_0) | автоперевозка → 1 |

#### Итого, по разделу:
- `fields`: 5 из 5
- `partition_status`: confirmed

### 3.11. Валюта и стоимость (графа 22)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | shipment.invoice_currency_alpha | CNY | CP | formalized.invoice_1.CurrencyCode | графа 22 — буквенный код валюты (G_22_3) | |
| 02 | shipment.invoice_currency_numeric | 156 | D | shipment.invoice_currency_alpha; cb:country | графа 22 — цифровой код валюты (G_22_1) | CNY numeric=156 |
| 03 | shipment.invoice_amount | 97260.00 | CP | formalized.invoice_1.TotalCost | графа 22 — сумма по счёту (G_22_2) | |

#### Итого, по разделу:
- `fields`: 3 из 3
- `partition_status`: confirmed

### 3.12. Курс валюты (графа 23)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | shipment.currency_rate | 10.9430 | CP | formalized.invoice_1.CurrencyRate | графа 23 — курс валюты к рублю на дату подачи (G_23_1, G_23_2) | |

#### Итого, по разделу:
- `fields`: 1 из 1
- `partition_status`: confirmed

### 3.13. Вид транспорта (графы 25, 26)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | transport.border_transport_code | 31 | D | cb:transport | графа 25 — код вида транспорта на границе (G_25_1) | автоперевозка → 31 |
| 02 | transport.internal_transport_code | 31 | D | cb:transport | графа 26 — код вида транспорта внутри страны (G_26_1) | автоперевозка → 31 |

#### Итого, по разделу:
- `fields`: 2 из 2
- `partition_status`: confirmed

### 3.14. Таможня на границе (графа 29)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | customs.border_code | 10719110 | CP | non_formalized.td_1.customs_post_code | графа 29 — код таможенного органа на границе (G_29_1); источник: ТД (09013) | |
| 02 | customs.border_name | ТАМОЖЕННЫЙ ПОСТ МАПП ЗАБАЙКАЛЬСК | CP | non_formalized.td_1.customs_post_name | графа 29 — наименование таможенного поста (G_29_2); источник: ТД (09013) | |

#### Итого, по разделу:
- `fields`: 2 из 2
- `partition_status`: confirmed

### 3.15. Местонахождение товаров (графа 30)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | location.type | 11 | D | cb:location | графа 30 — тип места нахождения товаров (G_30_0); 11 = склад временного хранения | |
| 02 | location.document_kind | 2 | D | | графа 30 — вид документа, подтверждающего место хранения (G_30_10); 2 = свидетельство/лицензия | |
| 03 | location.document_number | 10404/141210/10092/5 | CP | non_formalized.svh_1.warehouse_license_number | графа 30 — номер документа СВХ (G_30_1) | |
| 04 | location.document_date | 18.09.2025 | CP | non_formalized.svh_1.warehouse_license_date | графа 30 — дата документа СВХ (G_30_DATE) | |
| 05 | location.address.country_code | RU | D | | графа 30 — код страны местонахождения товаров (G_30_CC) | склад в РФ → RU |
| 06 | location.address.region | Республика Татарстан | CP | non_formalized.svh_additional_sheet_1.svh_address_region | графа 30 — регион (G_30_SUB) | |
| 07 | location.address.city | Набережные Челны | CP | non_formalized.svh_additional_sheet_1.svh_address_city | графа 30 — город (G_30_CIT) | |
| 08 | location.address.street | Производственный пр-д, д. 45 | CP | non_formalized.svh_additional_sheet_1.svh_address_street_house | графа 30 — улица и дом (G_30_STR) | |
| 09 | location.customs_code | 10404083 | CP | non_formalized.svh_additional_sheet_1.svh_customs_code | графа 30 — код таможенного органа, в зоне которого находится СВХ (G_30_12) | |
| 10 | location.printed | 11, 10404083, Республика Татарстан Набережные Челны Производственный пр-д, д. 45, 10404/141210/10092/5 ОТ 18.09.2025 | D | location.* | графа 30 — печатная строка местонахождения (G_30P_1); формируется автоматически | |

#### Итого, по разделу:
- `fields`: 10 из 10
- `partition_status`: confirmed

### 3.16. Товары (BLOCK, графы 31–47)

#### goods[1]

##### 3.16.2. Графы 32–38 — код товара, страна, веса, процедура (товар 1)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].item_no | 1 | D | | графа 32 — номер товара (G_32_1) | |
| 02 | goods[1].tnved_code | 5804101000 | D | formalized.invoice_1.InvoiceGoods_1.GoodsCode | графа 33 — код товара (G_33_1) | группа: InvoiceGoods_1,2,3,4,7 |
| 03 | goods[1].tnved.flag_1 |  | pending | | графа 33 — доп. признак (G_33_4) | птп |
| 04 | goods[1].tnved.flag_2 |  | pending | | графа 33 — доп. признак (G_33_5) | птп |
| 05 | goods[1].origin_country_code | CN | D | formalized.invoice_1.InvoiceGoods_1.OriginCountryCode; cb:country | графа 34 — код страны происхождения (G_34_1) | 156 → CN |
| 06 | goods[1].gross_weight | 1790.00 | D | non_formalized.svh_1.goods_2.gross_weight_kg | графа 35 — вес брутто по товару (G_35_1) | SVH строка 2 (tnved=5804101000): 1790; сумма по инвойсу: 855+490+265+155+25=1790, значения совпадают |
| 07 | goods[1].preference |  | pending | | графа 36 — преференция (G_36_2) | птп |
| 08 | goods[1].procedure_code | 4000000 | D | declaration.direction; declaration.procedure | графа 37 — процедура по товару (G_37_1) | ИМ40. птп |
| 09 | goods[1].net_weight | 1687.40 | D | formalized.invoice_1.InvoiceGoods_1.NetWeightQuantity + InvoiceGoods_2.NetWeightQuantity + InvoiceGoods_3.NetWeightQuantity + InvoiceGoods_4.NetWeightQuantity + InvoiceGoods_7.NetWeightQuantity | графа 38 — вес нетто по товару (G_38_1) | 806.60+460.80+252.00+144.00+24.00=1687.40 |

##### Итого, по элементу массива:
- `item_fields`: 9 из 9

##### 3.16.1. Графа 31 — описание товара (товар 1)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].g31.name | Москитные сетки из полиэстера (антикот, антипыльца, трёхслойные), рулоны размеров 1.4*30M2, 1.6*30M2 СМ.ДОПОЛНЕНИЕ | D | goods[1].tovg | графа 31 — описание товара (G_31/NAME) | |
| 02 | goods[1].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | goods[1].tovg.manufacturer | графа 31 — производитель (G_31/FIRMA) | единый производитель |
| 03 | goods[1].g31.trademark | ОТСУТСТВУЕТ | D | goods[1].tovg.trade_mark | графа 31 — товарный знак / ТМ (G_31/TM) | |
| 04 | goods[1].places | 100 | D | non_formalized.svh_1.goods_2.places | графа 31 — количество мест по товару (G_31/PLACE) | SVH строка 2 (tnved=5804101000) |

##### Итого, по элементу массива:
- `item_fields`: 4 из 4

##### 3.16.3. Графы 42–46 — стоимости по товару (товар 1)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].invoice_cost | 55032.00 | D | formalized.invoice_1.InvoiceGoods_1.TotalCost + InvoiceGoods_2.TotalCost + InvoiceGoods_3.TotalCost + InvoiceGoods_4.TotalCost + InvoiceGoods_7.TotalCost | графа 42 — цена товара (G_42_1) | 14742+8424+16002+9144+6720=55032 |
| 02 | goods[1].mos_code_main |  | pending | | графа 43 — код МОС (G_43_1) | птп |
| 03 | goods[1].mos_code_extra |  | pending | | графа 43 — доп. признак (G_43_2) | птп |
| 04 | goods[1].customs_value |  | pending | | графа 45 — таможенная стоимость (G_45_0, G_45_1) | рассчитывается Альтой |
| 05 | goods[1].statistical_value |  | pending | | графа 46 — статистическая стоимость (G_46_1) | рассчитывается Альтой |

##### Итого, по элементу массива:
- `item_fields`: 5 из 5

##### 3.16.4. Графа 47 — исчисление платежей (товар 1)

Не материализуется до выяснения (раздел помечен как «не материализуем»).

##### 3.16.6. TOVG — строки таблицы описания (товар 1)

#### tovg[1] — строка InvoiceGoods_1

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].tovg[1].line_no | 1 | D | | графа 31 — № строки таблицы (TOVG/G32G) | |
| 02 | goods[1].tovg[1].description | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CP | formalized.invoice_1.InvoiceGoods_1.GoodsDescription | графа 31 — наименование (TOVG/G31_1) | |
| 03 | goods[1].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | formalized.invoice_1.InvoiceGoods_1.AdditionalGoodsDescription_Manufacturer | графа 31 — производитель (TOVG/G31_11) | |
| 04 | goods[1].tovg[1].trade_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_1.AdditionalGoodsDescription_TradeMark | графа 31 — марка/ТМ (TOVG/G31_12) | |
| 05 | goods[1].tovg[1].goods_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_1.AdditionalGoodsDescription_GoodsMark | графа 31 — товарный знак (TOVG/G31_14) | |
| 06 | goods[1].tovg[1].model | 1.4*30 | D | formalized.invoice_1.InvoiceGoods_1.GoodsDescription | графа 31 — модель (TOVG/G31_15_MOD) | размер рулона |
| 07 | goods[1].tovg[1].quantity | 2520 | CP | formalized.invoice_1.InvoiceGoods_1.goods_supplementary_quantity | графа 31 — количество в доп.ед.изм (TOVG/KOLVO) | |
| 08 | goods[1].tovg[1].unit_code | 055 | D | cb:unit | графа 31 — код ЕИ (TOVG/CODE_EDI) | м² |
| 09 | goods[1].tovg[1].unit_name | м² | CP | formalized.invoice_1.InvoiceGoods_1.goods_supplementary_uom_name | графа 31 — наименование ЕИ (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[1].gross_weight | 855.00 | CP | formalized.invoice_1.InvoiceGoods_1.GrossWeightQuantity | графа 35 — вес брутто по строке (TOVG/G31_35) | |
| 11 | goods[1].tovg[1].net_weight | 806.60 | CP | formalized.invoice_1.InvoiceGoods_1.NetWeightQuantity | графа 38 — вес нетто по строке (TOVG/G31_38) | |
| 12 | goods[1].tovg[1].invoice_cost | 14742.00 | CP | formalized.invoice_1.InvoiceGoods_1.TotalCost | графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg[2] — строка InvoiceGoods_2

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].tovg[2].line_no | 2 | D | | графа 31 — № строки таблицы (TOVG/G32G) | |
| 02 | goods[1].tovg[2].description | Anti-cat mesh Roll size 1.6 * 30 / Москитная сетка «Антикот» Размер рулона 1,6*30 | CP | formalized.invoice_1.InvoiceGoods_2.GoodsDescription | графа 31 — наименование (TOVG/G31_1) | |
| 03 | goods[1].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | formalized.invoice_1.InvoiceGoods_2.AdditionalGoodsDescription_Manufacturer | графа 31 — производитель (TOVG/G31_11) | |
| 04 | goods[1].tovg[2].trade_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_2.AdditionalGoodsDescription_TradeMark | графа 31 — марка/ТМ (TOVG/G31_12) | |
| 05 | goods[1].tovg[2].goods_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_2.AdditionalGoodsDescription_GoodsMark | графа 31 — товарный знак (TOVG/G31_14) | |
| 06 | goods[1].tovg[2].model | 1.6*30 | D | formalized.invoice_1.InvoiceGoods_2.GoodsDescription | графа 31 — модель (TOVG/G31_15_MOD) | размер рулона |
| 07 | goods[1].tovg[2].quantity | 1440 | CP | formalized.invoice_1.InvoiceGoods_2.goods_supplementary_quantity | графа 31 — количество в доп.ед.изм (TOVG/KOLVO) | |
| 08 | goods[1].tovg[2].unit_code | 055 | D | cb:unit | графа 31 — код ЕИ (TOVG/CODE_EDI) | м² |
| 09 | goods[1].tovg[2].unit_name | м² | CP | formalized.invoice_1.InvoiceGoods_2.goods_supplementary_uom_name | графа 31 — наименование ЕИ (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[2].gross_weight | 490.00 | CP | formalized.invoice_1.InvoiceGoods_2.GrossWeightQuantity | графа 35 — вес брутто по строке (TOVG/G31_35) | |
| 11 | goods[1].tovg[2].net_weight | 460.80 | CP | formalized.invoice_1.InvoiceGoods_2.NetWeightQuantity | графа 38 — вес нетто по строке (TOVG/G31_38) | |
| 12 | goods[1].tovg[2].invoice_cost | 8424.00 | CP | formalized.invoice_1.InvoiceGoods_2.TotalCost | графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg[3] — строка InvoiceGoods_3

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].tovg[3].line_no | 3 | D | | графа 31 — № строки таблицы (TOVG/G32G) | |
| 02 | goods[1].tovg[3].description | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 / Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2 / Материал: полиэстер | CP | formalized.invoice_1.InvoiceGoods_3.GoodsDescription | графа 31 — наименование (TOVG/G31_1) | |
| 03 | goods[1].tovg[3].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | formalized.invoice_1.InvoiceGoods_3.AdditionalGoodsDescription_Manufacturer | графа 31 — производитель (TOVG/G31_11) | |
| 04 | goods[1].tovg[3].trade_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_3.AdditionalGoodsDescription_TradeMark | графа 31 — марка/ТМ (TOVG/G31_12) | |
| 05 | goods[1].tovg[3].goods_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_3.AdditionalGoodsDescription_GoodsMark | графа 31 — товарный знак (TOVG/G31_14) | |
| 06 | goods[1].tovg[3].model | 1.4*30 | D | formalized.invoice_1.InvoiceGoods_3.GoodsDescription | графа 31 — модель (TOVG/G31_15_MOD) | размер рулона |
| 07 | goods[1].tovg[3].quantity | 2520 | CP | formalized.invoice_1.InvoiceGoods_3.goods_supplementary_quantity | графа 31 — количество в доп.ед.изм (TOVG/KOLVO) | |
| 08 | goods[1].tovg[3].unit_code | 055 | D | cb:unit | графа 31 — код ЕИ (TOVG/CODE_EDI) | м² |
| 09 | goods[1].tovg[3].unit_name | м² | CP | formalized.invoice_1.InvoiceGoods_3.goods_supplementary_uom_name | графа 31 — наименование ЕИ (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[3].gross_weight | 265.00 | CP | formalized.invoice_1.InvoiceGoods_3.GrossWeightQuantity | графа 35 — вес брутто по строке (TOVG/G31_35) | |
| 11 | goods[1].tovg[3].net_weight | 252.00 | CP | formalized.invoice_1.InvoiceGoods_3.NetWeightQuantity | графа 38 — вес нетто по строке (TOVG/G31_38) | |
| 12 | goods[1].tovg[3].invoice_cost | 16002.00 | CP | formalized.invoice_1.InvoiceGoods_3.TotalCost | графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg[4] — строка InvoiceGoods_4

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].tovg[4].line_no | 4 | D | | графа 31 — № строки таблицы (TOVG/G32G) | |
| 02 | goods[1].tovg[4].description | ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2 / Сетка против пыльцы Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CP | formalized.invoice_1.InvoiceGoods_4.GoodsDescription | графа 31 — наименование (TOVG/G31_1) | |
| 03 | goods[1].tovg[4].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | formalized.invoice_1.InvoiceGoods_4.AdditionalGoodsDescription_Manufacturer | графа 31 — производитель (TOVG/G31_11) | |
| 04 | goods[1].tovg[4].trade_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_4.AdditionalGoodsDescription_TradeMark | графа 31 — марка/ТМ (TOVG/G31_12) | |
| 05 | goods[1].tovg[4].goods_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_4.AdditionalGoodsDescription_GoodsMark | графа 31 — товарный знак (TOVG/G31_14) | |
| 06 | goods[1].tovg[4].model | 1.6*30 | D | formalized.invoice_1.InvoiceGoods_4.GoodsDescription | графа 31 — модель (TOVG/G31_15_MOD) | размер рулона |
| 07 | goods[1].tovg[4].quantity | 1440 | CP | formalized.invoice_1.InvoiceGoods_4.goods_supplementary_quantity | графа 31 — количество в доп.ед.изм (TOVG/KOLVO) | |
| 08 | goods[1].tovg[4].unit_code | 055 | D | cb:unit | графа 31 — код ЕИ (TOVG/CODE_EDI) | м² |
| 09 | goods[1].tovg[4].unit_name | м² | CP | formalized.invoice_1.InvoiceGoods_4.goods_supplementary_uom_name | графа 31 — наименование ЕИ (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[4].gross_weight | 155.00 | CP | formalized.invoice_1.InvoiceGoods_4.GrossWeightQuantity | графа 35 — вес брутто по строке (TOVG/G31_35) | |
| 11 | goods[1].tovg[4].net_weight | 144.00 | CP | formalized.invoice_1.InvoiceGoods_4.NetWeightQuantity | графа 38 — вес нетто по строке (TOVG/G31_38) | |
| 12 | goods[1].tovg[4].invoice_cost | 9144.00 | CP | formalized.invoice_1.InvoiceGoods_4.TotalCost | графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg[5] — строка InvoiceGoods_7

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].tovg[5].line_no | 5 | D | | графа 31 — № строки таблицы (TOVG/G32G) | |
| 02 | goods[1].tovg[5].description | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 / Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CP | formalized.invoice_1.InvoiceGoods_7.GoodsDescription | графа 31 — наименование (TOVG/G31_1) | |
| 03 | goods[1].tovg[5].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | formalized.invoice_1.InvoiceGoods_7.AdditionalGoodsDescription_Manufacturer | графа 31 — производитель (TOVG/G31_11) | |
| 04 | goods[1].tovg[5].trade_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_7.AdditionalGoodsDescription_TradeMark | графа 31 — марка/ТМ (TOVG/G31_12) | |
| 05 | goods[1].tovg[5].goods_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_7.AdditionalGoodsDescription_GoodsMark | графа 31 — товарный знак (TOVG/G31_14) | |
| 06 | goods[1].tovg[5].model | 1.6*30 трёхслойная | D | formalized.invoice_1.InvoiceGoods_7.GoodsDescription | графа 31 — модель (TOVG/G31_15_MOD) | размер рулона, трёхслойная |
| 07 | goods[1].tovg[5].quantity | 240 | CP | formalized.invoice_1.InvoiceGoods_7.goods_supplementary_quantity | графа 31 — количество в доп.ед.изм (TOVG/KOLVO) | |
| 08 | goods[1].tovg[5].unit_code | 055 | D | cb:unit | графа 31 — код ЕИ (TOVG/CODE_EDI) | м² |
| 09 | goods[1].tovg[5].unit_name | м² | CP | formalized.invoice_1.InvoiceGoods_7.goods_supplementary_uom_name | графа 31 — наименование ЕИ (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[5].gross_weight | 25.00 | CP | formalized.invoice_1.InvoiceGoods_7.GrossWeightQuantity | графа 35 — вес брутто по строке (TOVG/G31_35) | |
| 11 | goods[1].tovg[5].net_weight | 24.00 | CP | formalized.invoice_1.InvoiceGoods_7.NetWeightQuantity | графа 38 — вес нетто по строке (TOVG/G31_38) | |
| 12 | goods[1].tovg[5].invoice_cost | 6720.00 | CP | formalized.invoice_1.InvoiceGoods_7.TotalCost | графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### Итого, по массиву tovg:
- `array_elements`: 5
- `item_fields`: всего полей 60 из 12 * 5
- `array_status`: confirmed

##### 3.16.5. TXT — дополнение к графе 31 (товар 1)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].txt[1].text | СМ.ДОПОЛНЕНИЕ | D | goods[1].tovg | графа 31 — строки дополнения (TXT/TEXT) | птп |

#### Итого, по элементу массива:
- `item_fields`: 1 из 1

#### Итого, по массиву txt:
- `array_elements`: 1
- `item_fields`: всего полей 1 из 1 * 1
- `array_status`: confirmed

#### Итого, по goods[1]:
- `total_item_fields`: 19 (графы 32-38) + 4 (G31) + 5 (стоимости) + 60 (tovg) + 1 (txt) = 89
- `status`: pending (поля tnved.flag_1, tnved.flag_2, preference, procedure_code, mos_code_main, mos_code_extra, customs_value, statistical_value = pending + раздел 47 не материализован)

#### goods[2]

##### 3.16.2. Графы 32–38 — код товара, страна, веса, процедура (товар 2)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[2].item_no | 2 | D | | графа 32 — номер товара (G_32_1) | |
| 02 | goods[2].tnved_code | 7019900095 | D | formalized.invoice_1.InvoiceGoods_5.GoodsCode | графа 33 — код товара (G_33_1) | группа: InvoiceGoods_5,6 |
| 03 | goods[2].tnved.flag_1 |  | pending | | графа 33 — доп. признак (G_33_4) | птп |
| 04 | goods[2].tnved.flag_2 |  | pending | | графа 33 — доп. признак (G_33_5) | птп |
| 05 | goods[2].origin_country_code | CN | D | formalized.invoice_1.InvoiceGoods_5.OriginCountryCode; cb:country | графа 34 — код страны происхождения (G_34_1) | 156 → CN |
| 06 | goods[2].gross_weight | 1710.00 | D | non_formalized.svh_1.goods_1.gross_weight_kg | графа 35 — вес брутто по товару (G_35_1) | SVH строка 1 (tnved=7019900095): 1710; сумма по инвойсу: 520+1190=1710, значения совпадают |
| 07 | goods[2].preference |  | pending | | графа 36 — преференция (G_36_2) | птп |
| 08 | goods[2].procedure_code | 4000000 | D | declaration.direction; declaration.procedure | графа 37 — процедура по товару (G_37_1) | ИМ40. птп |
| 09 | goods[2].net_weight | 1614.60 | D | formalized.invoice_1.InvoiceGoods_5.NetWeightQuantity + InvoiceGoods_6.NetWeightQuantity | графа 38 — вес нетто по товару (G_38_1) | 491.40+1123.20=1614.60 |

##### Итого, по элементу массива:
- `item_fields`: 9 из 9

##### 3.16.1. Графа 31 — описание товара (товар 2)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[2].g31.name | Москитные сетки из стекловолокна «Антимошка», рулоны размеров 1.4*30M2, 1.6*30M2 СМ.ДОПОЛНЕНИЕ | D | goods[2].tovg | графа 31 — описание товара (G_31/NAME) | |
| 02 | goods[2].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | goods[2].tovg.manufacturer | графа 31 — производитель (G_31/FIRMA) | единый производитель |
| 03 | goods[2].g31.trademark | ОТСУТСТВУЕТ | D | goods[2].tovg.trade_mark | графа 31 — товарный знак / ТМ (G_31/TM) | |
| 04 | goods[2].places | 27 | D | non_formalized.svh_1.goods_1.places | графа 31 — количество мест по товару (G_31/PLACE) | SVH строка 1 (tnved=7019900095) |

##### Итого, по элементу массива:
- `item_fields`: 4 из 4

##### 3.16.3. Графы 42–46 — стоимости по товару (товар 2)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[2].invoice_cost | 42228.00 | D | formalized.invoice_1.InvoiceGoods_5.TotalCost + InvoiceGoods_6.TotalCost | графа 42 — цена товара (G_42_1) | 12852+29376=42228 |
| 02 | goods[2].mos_code_main |  | pending | | графа 43 — код МОС (G_43_1) | птп |
| 03 | goods[2].mos_code_extra |  | pending | | графа 43 — доп. признак (G_43_2) | птп |
| 04 | goods[2].customs_value |  | pending | | графа 45 — таможенная стоимость (G_45_0, G_45_1) | рассчитывается Альтой |
| 05 | goods[2].statistical_value |  | pending | | графа 46 — статистическая стоимость (G_46_1) | рассчитывается Альтой |

##### Итого, по элементу массива:
- `item_fields`: 5 из 5

##### 3.16.6. TOVG — строки таблицы описания (товар 2)

#### tovg[1] — строка InvoiceGoods_5

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[2].tovg[1].line_no | 1 | D | | графа 31 — № строки таблицы (TOVG/G32G) | |
| 02 | goods[2].tovg[1].description | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | CP | formalized.invoice_1.InvoiceGoods_5.GoodsDescription | графа 31 — наименование (TOVG/G31_1) | |
| 03 | goods[2].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | formalized.invoice_1.InvoiceGoods_5.AdditionalGoodsDescription_Manufacturer | графа 31 — производитель (TOVG/G31_11) | |
| 04 | goods[2].tovg[1].trade_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_5.AdditionalGoodsDescription_TradeMark | графа 31 — марка/ТМ (TOVG/G31_12) | |
| 05 | goods[2].tovg[1].goods_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_5.AdditionalGoodsDescription_GoodsMark | графа 31 — товарный знак (TOVG/G31_14) | |
| 06 | goods[2].tovg[1].model | 1.4*30 | D | formalized.invoice_1.InvoiceGoods_5.GoodsDescription | графа 31 — модель (TOVG/G31_15_MOD) | размер рулона |
| 07 | goods[2].tovg[1].quantity | 3780 | CP | formalized.invoice_1.InvoiceGoods_5.goods_supplementary_quantity | графа 31 — количество в доп.ед.изм (TOVG/KOLVO) | |
| 08 | goods[2].tovg[1].unit_code | 055 | D | cb:unit | графа 31 — код ЕИ (TOVG/CODE_EDI) | м² |
| 09 | goods[2].tovg[1].unit_name | м² | CP | formalized.invoice_1.InvoiceGoods_5.goods_supplementary_uom_name | графа 31 — наименование ЕИ (TOVG/NAME_EDI) | |
| 10 | goods[2].tovg[1].gross_weight | 520.00 | CP | formalized.invoice_1.InvoiceGoods_5.GrossWeightQuantity | графа 35 — вес брутто по строке (TOVG/G31_35) | |
| 11 | goods[2].tovg[1].net_weight | 491.40 | CP | formalized.invoice_1.InvoiceGoods_5.NetWeightQuantity | графа 38 — вес нетто по строке (TOVG/G31_38) | |
| 12 | goods[2].tovg[1].invoice_cost | 12852.00 | CP | formalized.invoice_1.InvoiceGoods_5.TotalCost | графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg[2] — строка InvoiceGoods_6

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[2].tovg[2].line_no | 2 | D | | графа 31 — № строки таблицы (TOVG/G32G) | |
| 02 | goods[2].tovg[2].description | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 : Fiberglass / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | CP | formalized.invoice_1.InvoiceGoods_6.GoodsDescription | графа 31 — наименование (TOVG/G31_1) | |
| 03 | goods[2].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | formalized.invoice_1.InvoiceGoods_6.AdditionalGoodsDescription_Manufacturer | графа 31 — производитель (TOVG/G31_11) | |
| 04 | goods[2].tovg[2].trade_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_6.AdditionalGoodsDescription_TradeMark | графа 31 — марка/ТМ (TOVG/G31_12) | |
| 05 | goods[2].tovg[2].goods_mark | ОТСУТСТВУЕТ | CP | formalized.invoice_1.InvoiceGoods_6.AdditionalGoodsDescription_GoodsMark | графа 31 — товарный знак (TOVG/G31_14) | |
| 06 | goods[2].tovg[2].model | 1.6*30 | D | formalized.invoice_1.InvoiceGoods_6.GoodsDescription | графа 31 — модель (TOVG/G31_15_MOD) | размер рулона |
| 07 | goods[2].tovg[2].quantity | 8640 | CP | formalized.invoice_1.InvoiceGoods_6.goods_supplementary_quantity | графа 31 — количество в доп.ед.изм (TOVG/KOLVO) | |
| 08 | goods[2].tovg[2].unit_code | 055 | D | cb:unit | графа 31 — код ЕИ (TOVG/CODE_EDI) | м² |
| 09 | goods[2].tovg[2].unit_name | м² | CP | formalized.invoice_1.InvoiceGoods_6.goods_supplementary_uom_name | графа 31 — наименование ЕИ (TOVG/NAME_EDI) | |
| 10 | goods[2].tovg[2].gross_weight | 1190.00 | CP | formalized.invoice_1.InvoiceGoods_6.GrossWeightQuantity | графа 35 — вес брутто по строке (TOVG/G31_35) | |
| 11 | goods[2].tovg[2].net_weight | 1123.20 | CP | formalized.invoice_1.InvoiceGoods_6.NetWeightQuantity | графа 38 — вес нетто по строке (TOVG/G31_38) | |
| 12 | goods[2].tovg[2].invoice_cost | 29376.00 | CP | formalized.invoice_1.InvoiceGoods_6.TotalCost | графа 42 — цена по строке (TOVG/G31_42, TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### Итого, по массиву tovg:
- `array_elements`: 2
- `item_fields`: всего полей 24 из 12 * 2
- `array_status`: confirmed

##### 3.16.5. TXT — дополнение к графе 31 (товар 2)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[2].txt[1].text | СМ.ДОПОЛНЕНИЕ | D | goods[2].tovg | графа 31 — строки дополнения (TXT/TEXT) | птп |

#### Итого, по элементу массива:
- `item_fields`: 1 из 1

#### Итого, по массиву txt:
- `array_elements`: 1
- `item_fields`: всего полей 1 из 1 * 1
- `array_status`: confirmed

#### Итого, по goods[2]:
- `total_item_fields`: 19 (графы 32-38) + 4 (G31) + 5 (стоимости) + 24 (tovg) + 1 (txt) = 53
- `status`: pending (поля tnved.flag_1, tnved.flag_2, preference, procedure_code, mos_code_main, mos_code_extra, customs_value, statistical_value = pending + раздел 47 не материализован)

#### Итого, по массиву goods:
- `array_elements`: 2
- `item_fields`: всего полей 142 из 89 + 53
- `array_status`: pending

### 3.17. Графа 44 — представляемые документы

#### goods[1] g44_docs

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].g44_docs[1].doc_code | 03011 | CP | formalized.contract_1.DocumentCode | графа 44 — код документа (G44/G441) | Договор (контракт) |
| 02 | goods[1].g44_docs[1].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 03 | goods[1].g44_docs[1].doc_number | LM-2553 | D | formalized.contract_1.ContractRegistration_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 04 | goods[1].g44_docs[1].doc_date | 02.07.2025 | D | formalized.contract_1.ContractRegistration_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 05 | goods[1].g44_docs[1].doc_name | Договор (контракт) | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 06 | goods[1].g44_docs[2].doc_code | 03012 | CP | formalized.supplementary_contract_1.DocumentCode (предполагается) | графа 44 — код документа (G44/G441) | Доп. соглашение к контракту (код 03012 по аналогии) |
| 07 | goods[1].g44_docs[2].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 08 | goods[1].g44_docs[2].doc_number | 1 | D | formalized.supplementary_contract_1.DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 09 | goods[1].g44_docs[2].doc_date | 25.11.2025 | D | formalized.supplementary_contract_1.IssueDate | графа 44 — дата документа (G44/G443) | |
| 10 | goods[1].g44_docs[2].doc_name | Дополнительное соглашение к контракту | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 11 | goods[1].g44_docs[3].doc_code | 04021 | CP | formalized.invoice_1.DocumentCode | графа 44 — код документа (G44/G441) | Счет-фактура (инвойс) |
| 12 | goods[1].g44_docs[3].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 13 | goods[1].g44_docs[3].doc_number | LM-2591 | D | formalized.invoice_1.Registration_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 14 | goods[1].g44_docs[3].doc_date | 30.10.2025 | D | formalized.invoice_1.Registration_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 15 | goods[1].g44_docs[3].doc_name | КОММЕРЧЕСКИЙ ИНВОЙС | D | formalized.invoice_1.Registration_PrDocumentName | графа 44 — наименование документа (G44/G444) | |
| 16 | goods[1].g44_docs[4].doc_code | 04131 | CP | formalized.packing_list_1.DocumentCode (предполагается) | графа 44 — код документа (G44/G441) | Упаковочный лист |
| 17 | goods[1].g44_docs[4].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 18 | goods[1].g44_docs[4].doc_number | LM-2591 | D | formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 19 | goods[1].g44_docs[4].doc_date | 30.10.2025 | D | formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 20 | goods[1].g44_docs[4].doc_name | УПАКОВОЧНЫЙ ЛИСТ | D | formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentName | графа 44 — наименование документа (G44/G444) | |
| 21 | goods[1].g44_docs[5].doc_code | 02015 | CP | formalized.cmr_1.DocumentCode (предполагается) | графа 44 — код документа (G44/G441) | CMR (транспортная накладная) |
| 22 | goods[1].g44_docs[5].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 23 | goods[1].g44_docs[5].doc_number | 00378 | D | formalized.cmr_1.RegistrationDocument_RegID | графа 44 — номер документа (G44/G442) | |
| 24 | goods[1].g44_docs[5].doc_date | 20.01.2026 | D | formalized.cmr_1.RegistrationDocument_DateInf | графа 44 — дата документа (G44/G443) | |
| 25 | goods[1].g44_docs[5].doc_name | CMR (транспортная накладная) | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 26 | goods[1].g44_docs[6].doc_code | 04023 | CP | formalized.payment_order_1.DocumentCode | графа 44 — код документа (G44/G441) | Банковские документы / платежное поручение |
| 27 | goods[1].g44_docs[6].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 28 | goods[1].g44_docs[6].doc_number | 1 | D | formalized.payment_order_1.DocumentReference_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 29 | goods[1].g44_docs[6].doc_date | 13.01.2026 | D | formalized.payment_order_1.DocumentReference_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 30 | goods[1].g44_docs[6].doc_name | Банковские документы / платежное поручение | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 31 | goods[1].g44_docs[7].doc_code | 04023 | CP | formalized.payment_order_2.DocumentCode | графа 44 — код документа (G44/G441) | Банковские документы / платежное поручение |
| 32 | goods[1].g44_docs[7].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 33 | goods[1].g44_docs[7].doc_number | 7 | D | formalized.payment_order_2.DocumentReference_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 34 | goods[1].g44_docs[7].doc_date | 28.11.2025 | D | formalized.payment_order_2.DocumentReference_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 35 | goods[1].g44_docs[7].doc_name | Банковские документы / платежное поручение | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 36 | goods[1].g44_docs[8].doc_code | 04031 | CP | formalized.service_invoice_1.DocumentCode | графа 44 — код документа (G44/G441) | Счет за перевозку |
| 37 | goods[1].g44_docs[8].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 38 | goods[1].g44_docs[8].doc_number | 26-00378-tl | D | formalized.service_invoice_1.Registration_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 39 | goods[1].g44_docs[8].doc_date | 27.01.2026 | D | formalized.service_invoice_1.Registration_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 40 | goods[1].g44_docs[8].doc_name | Счет на оплату | D | formalized.service_invoice_1.Registration_PrDocumentName | графа 44 — наименование документа (G44/G444) | |
| 41 | goods[1].g44_docs[9].doc_code | 04033 | CP | formalized.transport_contract_1.DocumentCode | графа 44 — код документа (G44/G441) | Договор перевозки |
| 42 | goods[1].g44_docs[9].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 43 | goods[1].g44_docs[9].doc_number | КООО/26651/М | D | formalized.transport_contract_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 44 | goods[1].g44_docs[9].doc_date | 13.05.2025 | D | formalized.transport_contract_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 45 | goods[1].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | D | formalized.transport_contract_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 46 | goods[1].g44_docs[10].doc_code | 04011 | CP | formalized.egrul_1.DocumentCode | графа 44 — код документа (G44/G441) | Выписка из ЕГРЮЛ |
| 47 | goods[1].g44_docs[10].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 48 | goods[1].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | D | formalized.egrul_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 49 | goods[1].g44_docs[10].doc_date | 14.07.2025 | D | formalized.egrul_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 50 | goods[1].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | D | formalized.egrul_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 51 | goods[1].g44_docs[11].doc_code | 11001 | CP | formalized.passport_1.DocumentCode | графа 44 — код документа (G44/G441) | Паспорт |
| 52 | goods[1].g44_docs[11].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 53 | goods[1].g44_docs[11].doc_number | 63 09 449948 | D | formalized.passport_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 54 | goods[1].g44_docs[11].doc_date | 11.03.2010 | D | formalized.passport_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 55 | goods[1].g44_docs[11].doc_name | ПАСПОРТ | D | formalized.passport_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 56 | goods[1].g44_docs[12].doc_code | 11004 | CP | formalized.letter_of_attorney_1.DocumentCode | графа 44 — код документа (G44/G441) | Доверенность |
| 57 | goods[1].g44_docs[12].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 58 | goods[1].g44_docs[12].doc_number | 1 | D | formalized.letter_of_attorney_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 59 | goods[1].g44_docs[12].doc_date | 01.02.2026 | D | formalized.letter_of_attorney_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 60 | goods[1].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | D | formalized.letter_of_attorney_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 61 | goods[1].g44_docs[13].doc_code | 04111 | CP | formalized.insurance_document_1.DocumentCode | графа 44 — код документа (G44/G441) | Страховой документ / страховой полис |
| 62 | goods[1].g44_docs[13].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 63 | goods[1].g44_docs[13].doc_number | 26-00378-tl/1 | D | formalized.insurance_document_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 64 | goods[1].g44_docs[13].doc_date | 14.01.2026 | D | formalized.insurance_document_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 65 | goods[1].g44_docs[13].doc_name | Счет на оплату (страхование) | D | formalized.insurance_document_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 66 | goods[1].g44_docs[14].doc_code | 05999 | CP | formalized.tech_description_1.DocumentCode | графа 44 — код документа (G44/G441) | Техническое описание / иные документы |
| 67 | goods[1].g44_docs[14].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 68 | goods[1].g44_docs[14].doc_number | Б/Н | D | formalized.tech_description_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 69 | goods[1].g44_docs[14].doc_date | 30.10.2025 | D | formalized.tech_description_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 70 | goods[1].g44_docs[14].doc_name | ТЕХНИЧЕСКИЕ ХАРАКТЕРИСТИКИ | D | formalized.tech_description_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |

#### Итого, по массиву g44_docs (goods[1]):
- `array_elements`: 14
- `item_fields`: всего полей 70 из 5 * 14
- `array_status`: pending (kind_code pending для всех записей)

#### goods[2] g44_docs
Массив документов идентичен goods[1], значения те же.

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[2].g44_docs[1].doc_code | 03011 | CP | formalized.contract_1.DocumentCode | графа 44 — код документа (G44/G441) | Договор (контракт) |
| 02 | goods[2].g44_docs[1].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 03 | goods[2].g44_docs[1].doc_number | LM-2553 | D | formalized.contract_1.ContractRegistration_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 04 | goods[2].g44_docs[1].doc_date | 02.07.2025 | D | formalized.contract_1.ContractRegistration_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 05 | goods[2].g44_docs[1].doc_name | Договор (контракт) | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 06 | goods[2].g44_docs[2].doc_code | 03012 | CP | formalized.supplementary_contract_1.DocumentCode (предполагается) | графа 44 — код документа (G44/G441) | Доп. соглашение к контракту |
| 07 | goods[2].g44_docs[2].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 08 | goods[2].g44_docs[2].doc_number | 1 | D | formalized.supplementary_contract_1.DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 09 | goods[2].g44_docs[2].doc_date | 25.11.2025 | D | formalized.supplementary_contract_1.IssueDate | графа 44 — дата документа (G44/G443) | |
| 10 | goods[2].g44_docs[2].doc_name | Дополнительное соглашение к контракту | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 11 | goods[2].g44_docs[3].doc_code | 04021 | CP | formalized.invoice_1.DocumentCode | графа 44 — код документа (G44/G441) | Счет-фактура (инвойс) |
| 12 | goods[2].g44_docs[3].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 13 | goods[2].g44_docs[3].doc_number | LM-2591 | D | formalized.invoice_1.Registration_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 14 | goods[2].g44_docs[3].doc_date | 30.10.2025 | D | formalized.invoice_1.Registration_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 15 | goods[2].g44_docs[3].doc_name | КОММЕРЧЕСКИЙ ИНВОЙС | D | formalized.invoice_1.Registration_PrDocumentName | графа 44 — наименование документа (G44/G444) | |
| 16 | goods[2].g44_docs[4].doc_code | 04131 | CP | formalized.packing_list_1.DocumentCode (предполагается) | графа 44 — код документа (G44/G441) | Упаковочный лист |
| 17 | goods[2].g44_docs[4].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 18 | goods[2].g44_docs[4].doc_number | LM-2591 | D | formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 19 | goods[2].g44_docs[4].doc_date | 30.10.2025 | D | formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 20 | goods[2].g44_docs[4].doc_name | УПАКОВОЧНЫЙ ЛИСТ | D | formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentName | графа 44 — наименование документа (G44/G444) | |
| 21 | goods[2].g44_docs[5].doc_code | 02015 | CP | formalized.cmr_1.DocumentCode (предполагается) | графа 44 — код документа (G44/G441) | CMR (транспортная накладная) |
| 22 | goods[2].g44_docs[5].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 23 | goods[2].g44_docs[5].doc_number | 00378 | D | formalized.cmr_1.RegistrationDocument_RegID | графа 44 — номер документа (G44/G442) | |
| 24 | goods[2].g44_docs[5].doc_date | 20.01.2026 | D | formalized.cmr_1.RegistrationDocument_DateInf | графа 44 — дата документа (G44/G443) | |
| 25 | goods[2].g44_docs[5].doc_name | CMR (транспортная накладная) | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 26 | goods[2].g44_docs[6].doc_code | 04023 | CP | formalized.payment_order_1.DocumentCode | графа 44 — код документа (G44/G441) | Банковские документы / платежное поручение |
| 27 | goods[2].g44_docs[6].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 28 | goods[2].g44_docs[6].doc_number | 1 | D | formalized.payment_order_1.DocumentReference_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 29 | goods[2].g44_docs[6].doc_date | 13.01.2026 | D | formalized.payment_order_1.DocumentReference_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 30 | goods[2].g44_docs[6].doc_name | Банковские документы / платежное поручение | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 31 | goods[2].g44_docs[7].doc_code | 04023 | CP | formalized.payment_order_2.DocumentCode | графа 44 — код документа (G44/G441) | Банковские документы / платежное поручение |
| 32 | goods[2].g44_docs[7].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 33 | goods[2].g44_docs[7].doc_number | 7 | D | formalized.payment_order_2.DocumentReference_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 34 | goods[2].g44_docs[7].doc_date | 28.11.2025 | D | formalized.payment_order_2.DocumentReference_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 35 | goods[2].g44_docs[7].doc_name | Банковские документы / платежное поручение | D | cb:doc | графа 44 — наименование документа (G44/G444) | |
| 36 | goods[2].g44_docs[8].doc_code | 04031 | CP | formalized.service_invoice_1.DocumentCode | графа 44 — код документа (G44/G441) | Счет за перевозку |
| 37 | goods[2].g44_docs[8].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 38 | goods[2].g44_docs[8].doc_number | 26-00378-tl | D | formalized.service_invoice_1.Registration_PrDocumentNumber | графа 44 — номер документа (G44/G442) | |
| 39 | goods[2].g44_docs[8].doc_date | 27.01.2026 | D | formalized.service_invoice_1.Registration_PrDocumentDate | графа 44 — дата документа (G44/G443) | |
| 40 | goods[2].g44_docs[8].doc_name | Счет на оплату | D | formalized.service_invoice_1.Registration_PrDocumentName | графа 44 — наименование документа (G44/G444) | |
| 41 | goods[2].g44_docs[9].doc_code | 04033 | CP | formalized.transport_contract_1.DocumentCode | графа 44 — код документа (G44/G441) | Договор перевозки |
| 42 | goods[2].g44_docs[9].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 43 | goods[2].g44_docs[9].doc_number | КООО/26651/М | D | formalized.transport_contract_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 44 | goods[2].g44_docs[9].doc_date | 13.05.2025 | D | formalized.transport_contract_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 45 | goods[2].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | D | formalized.transport_contract_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 46 | goods[2].g44_docs[10].doc_code | 04011 | CP | formalized.egrul_1.DocumentCode | графа 44 — код документа (G44/G441) | Выписка из ЕГРЮЛ |
| 47 | goods[2].g44_docs[10].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 48 | goods[2].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | D | formalized.egrul_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 49 | goods[2].g44_docs[10].doc_date | 14.07.2025 | D | formalized.egrul_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 50 | goods[2].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | D | formalized.egrul_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 51 | goods[2].g44_docs[11].doc_code | 11001 | CP | formalized.passport_1.DocumentCode | графа 44 — код документа (G44/G441) | Паспорт |
| 52 | goods[2].g44_docs[11].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 53 | goods[2].g44_docs[11].doc_number | 63 09 449948 | D | formalized.passport_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 54 | goods[2].g44_docs[11].doc_date | 11.03.2010 | D | formalized.passport_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 55 | goods[2].g44_docs[11].doc_name | ПАСПОРТ | D | formalized.passport_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 56 | goods[2].g44_docs[12].doc_code | 11004 | CP | formalized.letter_of_attorney_1.DocumentCode | графа 44 — код документа (G44/G441) | Доверенность |
| 57 | goods[2].g44_docs[12].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 58 | goods[2].g44_docs[12].doc_number | 1 | D | formalized.letter_of_attorney_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 59 | goods[2].g44_docs[12].doc_date | 01.02.2026 | D | formalized.letter_of_attorney_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 60 | goods[2].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | D | formalized.letter_of_attorney_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 61 | goods[2].g44_docs[13].doc_code | 04111 | CP | formalized.insurance_document_1.DocumentCode | графа 44 — код документа (G44/G441) | Страховой документ / страховой полис |
| 62 | goods[2].g44_docs[13].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 63 | goods[2].g44_docs[13].doc_number | 26-00378-tl/1 | D | formalized.insurance_document_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 64 | goods[2].g44_docs[13].doc_date | 14.01.2026 | D | formalized.insurance_document_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 65 | goods[2].g44_docs[13].doc_name | Счет на оплату (страхование) | D | formalized.insurance_document_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |
| 66 | goods[2].g44_docs[14].doc_code | 05999 | CP | formalized.tech_description_1.DocumentCode | графа 44 — код документа (G44/G441) | Техническое описание / иные документы |
| 67 | goods[2].g44_docs[14].kind_code |  | pending | | графа 44 — признак записи (G44/G4403) | птп |
| 68 | goods[2].g44_docs[14].doc_number | Б/Н | D | formalized.tech_description_1.DocumentHead_DocumentNumber | графа 44 — номер документа (G44/G442) | |
| 69 | goods[2].g44_docs[14].doc_date | 30.10.2025 | D | formalized.tech_description_1.DocumentHead_DocumentDate | графа 44 — дата документа (G44/G443) | |
| 70 | goods[2].g44_docs[14].doc_name | ТЕХНИЧЕСКИЕ ХАРАКТЕРИСТИКИ | D | formalized.tech_description_1.DocumentHead_DocumentName | графа 44 — наименование документа (G44/G444) | |

#### Итого, по массиву g44_docs (goods[2]):
- `array_elements`: 14
- `item_fields`: всего полей 70 из 5 * 14
- `array_status`: pending (kind_code pending для всех записей)

### 3.17.1. Поле G_44 (текстовое поле в карточке товара)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | goods[1].g44.text | СМ.ДОПОЛНЕНИЕ | D | goods[1].txt; goods[1].tovg | графа 44 — текстовое поле (G_44) | |
| 02 | goods[2].g44.text | СМ.ДОПОЛНЕНИЕ | D | goods[2].txt; goods[2].tovg | графа 44 — текстовое поле (G_44) | |

#### Итого, по разделу:
- `fields`: 2 из 2
- `partition_status`: confirmed

### 3.18. Теги после товаров и документов (графы 51–54)

#### 3.18.1. Графа 42 (доп. признак)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | declaration.g42_2 |  | pending | | графа 42 — доп. признак (G_42_2) | птп |

#### Итого, по разделу:
- `fields`: 1 из 1
- `partition_status`: pending

#### 3.18.3. Графа 54 — уполномоченное лицо / представитель

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 01 | representative.date |  | pending | | графа 54 — дата заполнения/подачи (G_54_20) | задаётся оператором |
| 02 | representative.phone |  | pending | | графа 54 — телефон (G_54_21) | в доверенности и паспорте отсутствует. птп |
| 03 | representative.email |  | pending | | графа 54 — e-mail (G_54_EMAIL) | в доверенности и паспорте отсутствует |
| 04 | representative.last_name | АРБУЗОВА | CP | formalized.letter_of_attorney_1.EmpoweredPerson_PersonSurname | графа 54 — фамилия (G_54_3) | |
| 05 | representative.first_name | АНАСТАСИЯ | CP | formalized.letter_of_attorney_1.EmpoweredPerson_PersonName | графа 54 — имя (G_54_3NM) | |
| 06 | representative.middle_name | КОНСТАНТИНОВНА | CP | formalized.letter_of_attorney_1.EmpoweredPerson_PersonMiddleName | графа 54 — отчество (G_54_3MD) | |
| 07 | representative.authority_doc_name | ДОВЕРЕННОСТЬ | CP | formalized.letter_of_attorney_1.DocumentHead_DocumentName | графа 54 — документ полномочий (G_54_4) | |
| 08 | representative.authority_doc_number | 1 | CP | formalized.letter_of_attorney_1.DocumentHead_DocumentNumber | графа 54 — № документа полномочий (G_54_5) | |
| 09 | representative.authority_doc_date_from | 01.02.2026 | CP | formalized.letter_of_attorney_1.DocumentHead_DocumentDate | графа 54 — дата начала действия (G_54_60) | |
| 10 | representative.authority_doc_date_to | 31.12.2026 | CP | formalized.letter_of_attorney_1.EndDate | графа 54 — дата окончания действия (G_54_61) | |
| 11 | representative.position | УПОЛНОМОЧЕННОЕ ЛИЦО | CP | formalized.letter_of_attorney_1.EmpoweredPerson_PersonPost | графа 54 — должность/статус (G_54_7) | |
| 12 | representative.passport_code | RU01001 | D | | графа 54 — код документа удостоверения личности (G_54_8) | птп |
| 13 | representative.passport_name | ПАСРФ | D | | графа 54 — наименование документа (G_54_9) | птп |
| 14 | representative.passport_number | 449948 | CP | formalized.passport_1.CardNumber | графа 54 — номер паспорта (G_54_100) | |
| 15 | representative.passport_date | 11.03.2010 | CP | formalized.passport_1.CardDate | графа 54 — дата выдачи паспорта (G_54_101) | |
| 16 | representative.passport_series | 63 09 | CP | formalized.passport_1.CardSeries | графа 54 — серия паспорта (G_54_12) | |
| 17 | representative.passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CP | formalized.passport_1.OrganizationName | графа 54 — кем выдан (G_54_13) | |
| 18 | representative.printed_block | АРБУЗОВА АНАСТАСИЯ КОНСТАНТИНОВНА, ПАСРФ 63 09 449948, УПОЛНОМОЧЕННОЕ ЛИЦО, ДОВЕРЕННОСТЬ 1 от 01.02.2026 | D | representative.* | графа 54 — печатный блок (G_54P) | телефон и e-mail pending, в printed_block не включены. птп |

#### Итого, по разделу:
- `fields`: 18 из 18
- `partition_status`: pending

### Итог:
- `total_fields`: 351
- `dt_status`: pending

### Раздел II: Issues (нерешенные вопросы)

- `consignee.phone`: вопрос: телефон получателя отсутствует в primary.md; нужен от оператора.
- `consignee.email`: вопрос: email получателя отсутствует в primary.md; нужен от оператора.
- `declarant.phone`: вопрос: телефон декларанта отсутствует в доверенности/ЕГРЮЛ; нужен от оператора.
- `declarant.email`: вопрос: email декларанта отсутствует в доверенности/ЕГРЮЛ; нужен от оператора.
- `representative.phone`: вопрос: телефон представителя отсутствует в доверенности/паспорте; нужен от оператора.
- `representative.email`: вопрос: email представителя отсутствует в доверенности/паспорте; нужен от оператора.
- `representative.date`: вопрос: дата заполнения/подачи ДТ; задаётся оператором.
- `declaration.g42_2`: вопрос: доп. признак графы 42 (например "В ДТС"); нужен от оператора. птп.
- `goods[*].tnved.flag_1`: вопрос: значение-литера после кода ТН ВЭД; нужен от оператора. птп.
- `goods[*].tnved.flag_2`: вопрос: доп. признак после кода ТН ВЭД; нужен от оператора. птп.
- `goods[*].preference`: вопрос: код преференции; нужен от оператора. птп.
- `goods[*].procedure_code`: вопрос: процедура по товару (вычислено как 4000000 для ИМ40), требует подтверждения. птп.
- `goods[*].mos_code_main`: вопрос: код МОС; нужен от оператора. птп.
- `goods[*].mos_code_extra`: вопрос: доп. код МОС; нужен от оператора. птп.
- `goods[*].customs_value`: вопрос: таможенная стоимость; рассчитывается Альтой.
- `goods[*].statistical_value`: вопрос: статистическая стоимость; рассчитывается Альтой.
- `goods[*].g44_docs[*].kind_code`: вопрос: признак записи G4403; нужен от оператора или из codebook. птп.
- `[Общий]`: вопрос: раздел 3.16.4 (графа 47 — исчисление платежей) не материализован до выяснения, Альта считает сама.
