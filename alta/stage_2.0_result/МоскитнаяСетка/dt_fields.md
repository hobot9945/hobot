## Метаданные:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `тип поставки`: 1 ДТ / 2 товара
- `агрегация ДТ`: 2 товара по кодам ТН ВЭД (5804101000, 7019900095)
- `источники данных`: primary.md

## Раздел I: Поля ДТ

### 3.1. Заголовок декларации

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | declaration.direction | ИМ | CP | meta.direction | графа 1.1 | |
| 2 | declaration.procedure | 40 | CO | cb:procedure | графа 1.2 | ИМ40; подтверждено оператором |
| 3 | declaration.form | ЭД | D | правило | графа 1.31 | |

#### Итого, по разделу:
- `fields`: 3 из 3
- `partition_status`: confirmed

### 3.2. Отправитель (графа 2)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | sender.country_name | КИТАЙ | CP | formalized.invoice_1.Seler_PostalAddress_CounryName | графа 2 (G_2_50) | |
| 2 | sender.country_code | CN | CP | formalized.invoice_1.Seler_PostalAddress_CountryCode | графа 2 (G_2_7) | |
| 3 | sender.name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CP | formalized.invoice_1.Seler_Name | графа 2 (G_2_NAM) | |
| 4 | sender.region | HEBEI | CP | formalized.invoice_1.Seler_PostalAddress_Region | графа 2 (G_2_SUB) | |
| 5 | sender.city | SHIJIAZHUANG | CP | formalized.invoice_1.Seler_PostalAddress_City | графа 2 (G_2_CIT) | |
| 6 | sender.street | No. 5 Gaodong street | CP | formalized.invoice_1.Seler_PostalAddress_StreetHouse | графа 2 (G_2_STR) | |

#### Итого, по разделу:
- `fields`: 6 из 6
- `partition_status`: confirmed

### 3.3. Количество товаров и мест (графы 5, 6)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | shipment.total_goods_number | 2 | D | goods | графа 5 (G_5_1) | 2 уникальных GoodsCode |
| 2 | shipment.packages_flag | true | D | правило | графа 6 (G_6_0) | |
| 3 | shipment.total_packages | 127 | D | non_formalized.svh_1.actual_places | графа 6 (G_6_1) | приоритет SVH |

#### Итого, по разделу:
- `fields`: 3 из 3
- `partition_status`: confirmed

### 3.4. Получатель (графа 8)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | consignee.ogrn | 1201600020390 | CP | formalized.invoice_1.Consignee_OGRN | графа 8 (G_8_1) | |
| 2 | consignee.name_display | СМ. ГРАФУ 14 ДТ | D | правило | графа 8 (G_8/NAME) | получатель=декларант |
| 3 | consignee.country_name | РОССИЯ | CP | formalized.invoice_1.Buyer_PostalAddress_CounryName | графа 8 (G_8_50) | |
| 4 | consignee.inn_kpp | 1650389298/165001001 | D | formalized.invoice_1.Buyer_CompanyID; Buyer_KPPCode | графа 8 (G_8_6) | |
| 5 | consignee.country_code | RU | CP | formalized.invoice_1.Buyer_PostalAddress_CountryCode | графа 8 (G_8_7) | |
| 6 | consignee.name | ООО «СКИФ» | CP | formalized.invoice_1.Buyer_Name | графа 8 (G_8_NAM) | |
| 7 | consignee.postcode | 423800 | CP | formalized.invoice_1.Buyer_PostalAddress_PostalCode | графа 8 (G_8_POS) | |
| 8 | consignee.region | Республика Татарстан | CP | formalized.invoice_1.Buyer_PostalAddress_Region | графа 8 (G_8_SUB) | |
| 9 | consignee.city | Набережные Челны | CP | formalized.invoice_1.Buyer_PostalAddress_City | графа 8 (G_8_CIT) | |
| 10 | consignee.street | Хлебный проезд | D | formalized.invoice_1.Buyer_PostalAddress_StreetHouse | графа 8 (G_8_STR) | извлечено из StreetHouse |
| 11 | consignee.building | 30 | D | formalized.invoice_1.Buyer_PostalAddress_StreetHouse | графа 8 (G_8_BLD) | извлечено |
| 12 | consignee.room | 211 | D | formalized.invoice_1.Buyer_PostalAddress_StreetHouse | графа 8 (G_8_ROM) | извлечено офис |
| 13 | consignee.same_as_declarant | true | D | правило (inn_kpp совпадает с declarant) | графа 8 (G_8_SM14) | |
| 14 | consignee.phone | +7 937 779-26-56 | CP | non_formalized.master_data_1.declarant_phone | графа 8 (G_8_PHONE) | |
| 15 | consignee.email | | CP | non_formalized.master_data_1.declarant_email | графа 8 (G_8_EMAIL) | пусто |

#### Итого, по разделу:
- `fields`: 15 из 15
- `partition_status`: confirmed

### 3.5. Финансовое урегулирование (графа 9)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | financial.same_as_declarant | true | D | правило | графа 9 (G_9_SM14) | |
| 2 | financial.name_display | СМ. ГРАФУ 14 ДТ | D | правило | графа 9 (G_9/NAME) | |
| 3 | financial.ogrn | 1201600020390 | D | declarant.ogrn | графа 9 (G_9_1) | |
| 4 | financial.inn_kpp | 1650389298/165001001 | D | declarant.inn_kpp | графа 9 (G_9_4) | |
| 5 | financial.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ «СКИФ» | D | declarant.name | графа 9 (G_9_NAM) | |
| 6 | financial.country_code | RU | D | declarant.country_code | графа 9 (G_9_CC) | |
| 7 | financial.country_name | РОССИЯ | D | declarant.country_name | графа 9 (G_9_CN) | |
| 8 | financial.postcode | 423800 | D | declarant.postcode | графа 9 (G_9_POS) | |
| 9 | financial.region | РЕСПУБЛИКА ТАТАРСТАН (ТАТАРСТАН) | D | declarant.region | графа 9 (G_9_SUB) | |
| 10 | financial.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | declarant.city | графа 9 (G_9_CIT) | |
| 11 | financial.street | ПР-Д ХЛЕБНЫЙ | D | declarant.street | графа 9 (G_9_STR) | |
| 12 | financial.building | 30 | D | declarant.building | графа 9 (G_9_BLD) | |
| 13 | financial.room | 211 | D | declarant.room | графа 9 (G_9_ROM) | |
| 14 | financial.country_code_alt | RU | D | declarant.country_code | графа 9 (G_9_7) | |
| 15 | financial.phone | +7 937 779-26-56 | D | declarant.phone | графа 9 (G_9_PHONE) | |
| 16 | financial.email | | D | declarant.email | графа 9 (G_9_EMAIL) | пусто |

#### Итого, по разделу:
- `fields`: 16 из 16
- `partition_status`: confirmed

### 3.6. Торгующая страна (графа 11)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | shipment.trade_country_code | CN | CP | formalized.invoice_1.DeliveryTerms_TradingCountryCode | графа 11 (G_11_1) | |

#### Итого, по разделу:
- `fields`: 1 из 1
- `partition_status`: confirmed

### 3.7. Декларант (графа 14)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | declarant.ogrn | 1201600020390 | CP | non_formalized.master_data_1.declarant_ogrn | графа 14 (G_14_1) | |
| 2 | declarant.name_display | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ «СКИФ», 423800, РЕСПУБЛИКА ТАТАРСТАН (ТАТАРСТАН), НАБЕРЕЖНЫЕ ЧЕЛНЫ, ПР-Д ХЛЕБНЫЙ, 30, 211, +7 937 779-26-56 | D | declarant.* | графа 14 (G_14/NAME) | печатный блок |
| 3 | declarant.inn_kpp | 1650389298/165001001 | D | master_data.declarant_inn; declarant_kpp | графа 14 (G_14_4) | |
| 4 | declarant.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ «СКИФ» | CP | master_data.declarant_name | графа 14 (G_14_NAM) | |
| 5 | declarant.country_code | RU | CP | master_data.declarant_address_country_code | графа 14 (G_14_CC) | |
| 6 | declarant.country_name | РОССИЯ | CP | master_data.declarant_address_country_name | графа 14 (G_14_CN) | |
| 7 | declarant.postcode | 423800 | CP | master_data.declarant_address_postal_code | графа 14 (G_14_POS) | |
| 8 | declarant.region | РЕСПУБЛИКА ТАТАРСТАН (ТАТАРСТАН) | CP | master_data.declarant_address_region | графа 14 (G_14_SUB) | |
| 9 | declarant.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | master_data.declarant_address_city | графа 14 (G_14_CIT) | |
| 10 | declarant.street | ПР-Д ХЛЕБНЫЙ | CP | master_data.declarant_address_street | графа 14 (G_14_STR) | |
| 11 | declarant.building | 30 | CP | master_data.declarant_address_building | графа 14 (G_14_BLD) | |
| 12 | declarant.room | 211 | CP | master_data.declarant_address_room | графа 14 (G_14_ROM) | |
| 13 | declarant.phone | +7 937 779-26-56 | CP | master_data.declarant_phone | графа 14 (G_14_PHONE) | |
| 14 | declarant.email | | CP | master_data.declarant_email | графа 14 (G_14_EMAIL) | пусто |

#### Итого, по разделу:
- `fields`: 14 из 14
- `partition_status`: confirmed

### 3.8. Страны (графы 15, 16, 17)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | shipment.dispatch_country_code | CN | CP | formalized.invoice_1.DeliveryTerms_DispatchCountryCode | графа 15A (G_15A_1) | |
| 2 | shipment.destination_country_code | RU | CP | formalized.invoice_1.DeliveryTerms_DestinationCountryCode | графа 17A (G_17A_1) | |
| 3 | shipment.dispatch_country_name | КИТАЙ | D | shipment.dispatch_country_code → cb:country | графа 15 (G_15_1) | CN → Китай |
| 4 | shipment.destination_country_name | РОССИЯ | D | shipment.destination_country_code → cb:country | графа 17 (G_17_1) | RU → Россия |
| 5 | shipment.origin_country_code | CN | D | InvoiceGoods_*.OriginCountryCode=156 → cb:country | графа 16 (G_16_2) | все строки 156→CN |
| 6 | shipment.origin_country_name | КИТАЙ | D | shipment.origin_country_code → cb:country | графа 16 (G_16_1) | |

#### Итого, по разделу:
- `fields`: 6 из 6
- `partition_status`: confirmed

### 3.9. Условия поставки (графа 20)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | delivery.terms_code | EXW | D | formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode | графа 20 (G_20_20) | приоритет: инвойс |
| 2 | delivery.place_name | HEBEI | D | formalized.invoice_1.DeliveryTerms_DeliveryPlace | графа 20 (G_20_21) | |

#### Итого, по разделу:
- `fields`: 2 из 2
- `partition_status`: confirmed

### 3.10. Транспорт (графы 18, 19, 21)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | transport.vehicles_count | 2 | D | formalized.packing_list_1.TransportMeans_* | графа 18 (G_18_0) | тягач+прицеп |
| 2 | transport.identification | О157АО774/ВТ374974 | D | formalized.packing_list_1.TransportMeans_*.Number | графа 18 (G_18) | |
| 3 | transport.registration_country_code | 00 | D | TransportMeans_1.NationalityCode=000 | графа 18 (G_18_2) | |
| 4 | transport.container_flag | 0 | D | автоперевозка без контейнера | графа 19 (G_19_1) | |
| 5 | transport.border_mode | 1 | D | автоперевозка | графа 21 (G_21_0) | |

#### Итого, по разделу:
- `fields`: 5 из 5
- `partition_status`: confirmed

### 3.11. Валюта и стоимость (графа 22)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | shipment.invoice_currency_alpha | CNY | CP | formalized.invoice_1.CurrencyCode | графа 22 (G_22_3) | |
| 2 | shipment.invoice_currency_numeric | 156 | D | CNY → ISO numeric 156 | графа 22 (G_22_1) | cb:country: CNY=156 |
| 3 | shipment.invoice_amount | 97260.00 | CP | formalized.invoice_1.TotalCost | графа 22 (G_22_2) | |

#### Итого, по разделу:
- `fields`: 3 из 3
- `partition_status`: confirmed

### 3.12. Курс валюты (графа 23)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | shipment.currency_rate | 10.9430 | CP | formalized.invoice_1.CurrencyRate | графа 23 (G_23_1, G_23_2) | |

#### Итого, по разделу:
- `fields`: 1 из 1
- `partition_status`: confirmed

### 3.13. Вид транспорта (графы 25, 26)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | transport.border_transport_code | 31 | D | cb:transport: состав ТС | графа 25 (G_25_1) | |
| 2 | transport.internal_transport_code | 31 | D | автотранспорт | графа 26 (G_26_1) | |

#### Итого, по разделу:
- `fields`: 2 из 2
- `partition_status`: confirmed

### 3.14. Таможня на границе (графа 29)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | customs.border_code | 10404083 | CP | non_formalized.td_1.customs_post_code | графа 29 (G_29_1) | |
| 2 | customs.border_name | ОТО И ТК №3 Т/П НАБЕРЕЖНОЧЕЛНИНСКИЙ | CP | non_formalized.td_1.customs_post_name | графа 29 (G_29_2) | |

#### Итого, по разделу:
- `fields`: 2 из 2
- `partition_status`: confirmed

### 3.15. Местонахождение товаров (графа 30)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | location.type | 11 | D | cb:location: СВХ | графа 30 (G_30_0) | |
| 2 | location.document_kind | 2 | D | правило: лицензия СВХ | графа 30 (G_30_10) | |
| 3 | location.document_number | 10404/141210/10092/5 | CP | non_formalized.svh_1.warehouse_license_number | графа 30 (G_30_1) | |
| 4 | location.document_date | 18.09.2025 | CP | non_formalized.svh_1.warehouse_license_date | графа 30 (G_30_DATE) | |
| 5 | location.address.country_code | RU | D | склад в РФ | графа 30 (G_30_CC) | |
| 6 | location.address.region | Республика Татарстан | CP | non_formalized.svh_additional_sheet_1.svh_address_region | графа 30 (G_30_SUB) | |
| 7 | location.address.city | Набережные Челны | CP | non_formalized.svh_additional_sheet_1.svh_address_city | графа 30 (G_30_CIT) | |
| 8 | location.address.street | Производственный пр-д, д.45 | CP | non_formalized.svh_additional_sheet_1.svh_address_street_house | графа 30 (G_30_STR) | |
| 9 | location.customs_code | 10404083 | CP | non_formalized.svh_additional_sheet_1.svh_customs_code | графа 30 (G_30_12) | |
| 10 | location.printed | 11, 10404083, Республика Татарстан Набережные Челны Производственный пр-д, д.45, 10404/141210/10092/5 ОТ 18.09.2025 | D | location.* | графа 30 (G_30P_1) | |

#### Итого, по разделу:
- `fields`: 10 из 10
- `partition_status`: confirmed

### 3.16. Товары (графы 31–47)

#### goods_1 (код 5804101000)

##### 3.16.1. Графа 31 — описание товара

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].g31.name | МОСКИТНАЯ СЕТКА ИЗ ПОЛИЭСТЕРА И СТЕКЛОВОЛОКНА: «АНТИКОТ», «АНТИПЫЛЬЦА», ТРЕХСЛОЙНАЯ «АНТИПЫЛЬЦА», В РУЛОНАХ, СМ.ДОПОЛНЕНИЕ | D | goods[1].tovg | графа 31 (G_31/NAME) | |
| 2 | goods[1].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | goods[1].tovg.manufacturer | графа 31 (G_31/FIRMA) | единый для всех строк |
| 3 | goods[1].g31.trademark | ОТСУТСТВУЕТ | D | goods[1].tovg.trade_mark | графа 31 (G_31/TM) | |
| 4 | goods[1].places | 100 | D | non_formalized.svh_1.goods_2.places (tnved=5804101000) | графа 31 (G_31/PLACE) | |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

##### 3.16.2. Графы 32–38

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].item_no | 1 | D | порядковый номер | графа 32 (G_32_1) | |
| 2 | goods[1].tnved_code | 5804101000 | D | InvoiceGoods_1.GoodsCode | графа 33 (G_33_1) | |
| 3 | goods[1].tnved.flag_1 | | pending | птп | графа 33 (G_33_4) | требуется подтверждение |
| 4 | goods[1].tnved.flag_2 | | pending | птп | графа 33 (G_33_5) | требуется подтверждение |
| 5 | goods[1].origin_country_code | CN | D | InvoiceGoods_*.OriginCountryCode=156→cb:country | графа 34 (G_34_1) | |
| 6 | goods[1].gross_weight | 1790 | D | non_formalized.svh_1.goods_2.gross_weight_kg | графа 35 (G_35_1) | приоритет SVH |
| 7 | goods[1].preference | | pending | птп | графа 36 (G_36_2) | требуется подтверждение |
| 8 | goods[1].procedure_code | 4000000 | D | ИМ40 | графа 37 (G_37_1) | птп |
| 9 | goods[1].net_weight | 1687.40 | D | сумма InvoiceGoods_1,2,3,4,7.NetWeightQuantity | графа 38 (G_38_1) | 806.60+460.80+252.00+144.00+24.00 |

#### Итого, по элементу массива:
- `item_fields`: 9 из 9

##### 3.16.3. Графы 42–46

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].invoice_cost | 55032.00 | D | сумма InvoiceGoods_1,2,3,4,7.TotalCost | графа 42 (G_42_1) | |
| 2 | goods[1].mos_code_main | | pending | птп | графа 43 (G_43_1) | требуется подтверждение |
| 3 | goods[1].mos_code_extra | | pending | птп | графа 43 (G_43_2) | требуется подтверждение |
| 4 | goods[1].customs_value | | pending | правило | графа 45 (G_45_0, G_45_1) | рассчитывается Альтой |
| 5 | goods[1].statistical_value | | pending | правило | графа 46 (G_46_1) | рассчитывается Альтой |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

##### 3.16.5. Дополнение к графе 31 — TXT

#### txt_1

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].txt[1].text | 1) ANTI-CAT MESH ROLL SIZE 1.4*30 M2, POLYESTER / МОСКИТНАЯ СЕТКА «АНТИКОТ» 1,4*30 М2, ПОЛИЭСТЕР, 2520 М2, 855.00/806.60 КГ, 14742.00 CNY | D | InvoiceGoods_1 | графа 31 дополнение | |

#### Итого, по элементу массива:
- `item_fields`: 1 из 1

#### txt_2

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].txt[2].text | 2) ANTI-CAT MESH ROLL SIZE 1.6*30 M2 / МОСКИТНАЯ СЕТКА «АНТИКОТ» 1,6*30 М2, 1440 М2, 490.00/460.80 КГ, 8424.00 CNY | D | InvoiceGoods_2 | графа 31 дополнение | |

#### Итого, по элементу массива:
- `item_fields`: 1 из 1

#### txt_3

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].txt[3].text | 3) ANTI-POLLEN MESH 1.4*30 M2, POLYESTER / СЕТКА «АНТИПЫЛЬЦА» 1,4*30 М2, ПОЛИЭСТЕР, 2520 М2, 265.00/252.00 КГ, 16002.00 CNY | D | InvoiceGoods_3 | графа 31 дополнение | |

#### Итого, по элементу массива:
- `item_fields`: 1 из 1

#### txt_4

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].txt[4].text | 4) ANTI-POLLEN MESH 1.6*30 M2, POLYESTER / СЕТКА «АНТИПЫЛЬЦА» 1,6*30 М2, ПОЛИЭСТЕР, 1440 М2, 155.00/144.00 КГ, 9144.00 CNY | D | InvoiceGoods_4 | графа 31 дополнение | |

#### Итого, по элементу массива:
- `item_fields`: 1 из 1

#### txt_5

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].txt[5].text | 5) GRID WITH 3 LAYER POLYESTER 1.6*30 M2 / ТРЕХСЛОЙНАЯ СЕТКА «АНТИПЫЛЬЦА» 1,6*30 М2, ПОЛИЭСТЕР, 240 М2, 25.00/24.00 КГ, 6720.00 CNY | D | InvoiceGoods_7 | графа 31 дополнение | |

#### Итого, по элементу массива:
- `item_fields`: 1 из 1

#### Итого, по массиву txt:
- `array_elements`: 5
- `item_fields`: всего полей 5 из 5
- `array_status`: confirmed

##### 3.16.6. Таблица TOVG

#### tovg_1 (строка инвойса 1)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].tovg[1].line_no | 1 | D | порядковый номер | графа 31 (TOVG/G32G) | |
| 2 | goods[1].tovg[1].description | ANTI-CAT MESH ROLL SIZE 1.4*30 M2, POLYESTER / МОСКИТНАЯ СЕТКА «АНТИКОТ» 1,4*30 М2, ПОЛИЭСТЕР | D | InvoiceGoods_1.GoodsDescription | графа 31 (TOVG/G31_1) | |
| 3 | goods[1].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | InvoiceGoods_1.Manufacturer | графа 31 (TOVG/G31_11) | |
| 4 | goods[1].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_1.TradeMark | графа 31 (TOVG/G31_12) | |
| 5 | goods[1].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_1.GoodsMark | графа 31 (TOVG/G31_14) | |
| 6 | goods[1].tovg[1].model | NOT APPLICABLE | D | InvoiceGoods_1.GoodsModel | графа 31 (TOVG/G31_15_MOD) | |
| 7 | goods[1].tovg[1].quantity | 2520 | CP | InvoiceGoods_1.goods_supplementary_quantity | графа 31 (TOVG/KOLVO) | |
| 8 | goods[1].tovg[1].unit_code | 055 | D | cb:unit: м² | графа 31 (TOVG/CODE_EDI) | |
| 9 | goods[1].tovg[1].unit_name | м² (квадратный метр) | CP | InvoiceGoods_1.goods_supplementary_uom_name | графа 31 (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[1].gross_weight | 855.00 | CP | InvoiceGoods_1.GrossWeightQuantity | графа 35 (TOVG/G31_35) | |
| 11 | goods[1].tovg[1].net_weight | 806.60 | CP | InvoiceGoods_1.NetWeightQuantity | графа 38 (TOVG/G31_38) | |
| 12 | goods[1].tovg[1].invoice_cost | 14742.00 | CP | InvoiceGoods_1.TotalCost | графа 42 (TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg_2 (строка инвойса 2)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].tovg[2].line_no | 2 | D | порядковый номер | графа 31 (TOVG/G32G) | |
| 2 | goods[1].tovg[2].description | ANTI-CAT MESH ROLL SIZE 1.6*30 M2 / МОСКИТНАЯ СЕТКА «АНТИКОТ» 1,6*30 М2 | D | InvoiceGoods_2.GoodsDescription | графа 31 (TOVG/G31_1) | |
| 3 | goods[1].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | InvoiceGoods_2.Manufacturer | графа 31 (TOVG/G31_11) | |
| 4 | goods[1].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_2.TradeMark | графа 31 (TOVG/G31_12) | |
| 5 | goods[1].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_2.GoodsMark | графа 31 (TOVG/G31_14) | |
| 6 | goods[1].tovg[2].model | NOT APPLICABLE | D | InvoiceGoods_2.GoodsModel | графа 31 (TOVG/G31_15_MOD) | |
| 7 | goods[1].tovg[2].quantity | 1440 | CP | InvoiceGoods_2.goods_supplementary_quantity | графа 31 (TOVG/KOLVO) | |
| 8 | goods[1].tovg[2].unit_code | 055 | D | cb:unit: м² | графа 31 (TOVG/CODE_EDI) | |
| 9 | goods[1].tovg[2].unit_name | м² (квадратный метр) | CP | InvoiceGoods_2.goods_supplementary_uom_name | графа 31 (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[2].gross_weight | 490.00 | CP | InvoiceGoods_2.GrossWeightQuantity | графа 35 (TOVG/G31_35) | |
| 11 | goods[1].tovg[2].net_weight | 460.80 | CP | InvoiceGoods_2.NetWeightQuantity | графа 38 (TOVG/G31_38) | |
| 12 | goods[1].tovg[2].invoice_cost | 8424.00 | CP | InvoiceGoods_2.TotalCost | графа 42 (TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg_3 (строка инвойса 3)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].tovg[3].line_no | 3 | D | порядковый номер | графа 31 (TOVG/G32G) | |
| 2 | goods[1].tovg[3].description | ANTI-POLLEN MESH 1.4*30 M2, POLYESTER / СЕТКА «АНТИПЫЛЬЦА» 1,4*30 М2, ПОЛИЭСТЕР | D | InvoiceGoods_3.GoodsDescription | графа 31 (TOVG/G31_1) | |
| 3 | goods[1].tovg[3].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | InvoiceGoods_3.Manufacturer | графа 31 (TOVG/G31_11) | |
| 4 | goods[1].tovg[3].trade_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_3.TradeMark | графа 31 (TOVG/G31_12) | |
| 5 | goods[1].tovg[3].goods_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_3.GoodsMark | графа 31 (TOVG/G31_14) | |
| 6 | goods[1].tovg[3].model | NOT APPLICABLE | D | InvoiceGoods_3.GoodsModel | графа 31 (TOVG/G31_15_MOD) | |
| 7 | goods[1].tovg[3].quantity | 2520 | CP | InvoiceGoods_3.goods_supplementary_quantity | графа 31 (TOVG/KOLVO) | |
| 8 | goods[1].tovg[3].unit_code | 055 | D | cb:unit: м² | графа 31 (TOVG/CODE_EDI) | |
| 9 | goods[1].tovg[3].unit_name | м² (квадратный метр) | CP | InvoiceGoods_3.goods_supplementary_uom_name | графа 31 (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[3].gross_weight | 265.00 | CP | InvoiceGoods_3.GrossWeightQuantity | графа 35 (TOVG/G31_35) | |
| 11 | goods[1].tovg[3].net_weight | 252.00 | CP | InvoiceGoods_3.NetWeightQuantity | графа 38 (TOVG/G31_38) | |
| 12 | goods[1].tovg[3].invoice_cost | 16002.00 | CP | InvoiceGoods_3.TotalCost | графа 42 (TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg_4 (строка инвойса 4)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].tovg[4].line_no | 4 | D | порядковый номер | графа 31 (TOVG/G32G) | |
| 2 | goods[1].tovg[4].description | ANTI-POLLEN MESH 1.6*30 M2, POLYESTER / СЕТКА «АНТИПЫЛЬЦА» 1,6*30 М2, ПОЛИЭСТЕР | D | InvoiceGoods_4.GoodsDescription | графа 31 (TOVG/G31_1) | |
| 3 | goods[1].tovg[4].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | InvoiceGoods_4.Manufacturer | графа 31 (TOVG/G31_11) | |
| 4 | goods[1].tovg[4].trade_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_4.TradeMark | графа 31 (TOVG/G31_12) | |
| 5 | goods[1].tovg[4].goods_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_4.GoodsMark | графа 31 (TOVG/G31_14) | |
| 6 | goods[1].tovg[4].model | NOT APPLICABLE | D | InvoiceGoods_4.GoodsModel | графа 31 (TOVG/G31_15_MOD) | |
| 7 | goods[1].tovg[4].quantity | 1440 | CP | InvoiceGoods_4.goods_supplementary_quantity | графа 31 (TOVG/KOLVO) | |
| 8 | goods[1].tovg[4].unit_code | 055 | D | cb:unit: м² | графа 31 (TOVG/CODE_EDI) | |
| 9 | goods[1].tovg[4].unit_name | м² (квадратный метр) | CP | InvoiceGoods_4.goods_supplementary_uom_name | графа 31 (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[4].gross_weight | 155.00 | CP | InvoiceGoods_4.GrossWeightQuantity | графа 35 (TOVG/G31_35) | |
| 11 | goods[1].tovg[4].net_weight | 144.00 | CP | InvoiceGoods_4.NetWeightQuantity | графа 38 (TOVG/G31_38) | |
| 12 | goods[1].tovg[4].invoice_cost | 9144.00 | CP | InvoiceGoods_4.TotalCost | графа 42 (TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg_5 (строка инвойса 7)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[1].tovg[5].line_no | 5 | D | порядковый номер | графа 31 (TOVG/G32G) | |
| 2 | goods[1].tovg[5].description | GRID WITH 3 LAYER POLYESTER 1.6*30 M2 / ТРЕХСЛОЙНАЯ СЕТКА «АНТИПЫЛЬЦА» 1,6*30 М2, ПОЛИЭСТЕР | D | InvoiceGoods_7.GoodsDescription | графа 31 (TOVG/G31_1) | |
| 3 | goods[1].tovg[5].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | InvoiceGoods_7.Manufacturer | графа 31 (TOVG/G31_11) | |
| 4 | goods[1].tovg[5].trade_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_7.TradeMark | графа 31 (TOVG/G31_12) | |
| 5 | goods[1].tovg[5].goods_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_7.GoodsMark | графа 31 (TOVG/G31_14) | |
| 6 | goods[1].tovg[5].model | NOT APPLICABLE | D | InvoiceGoods_7.GoodsModel | графа 31 (TOVG/G31_15_MOD) | |
| 7 | goods[1].tovg[5].quantity | 240 | CP | InvoiceGoods_7.goods_supplementary_quantity | графа 31 (TOVG/KOLVO) | |
| 8 | goods[1].tovg[5].unit_code | 055 | D | cb:unit: м² | графа 31 (TOVG/CODE_EDI) | |
| 9 | goods[1].tovg[5].unit_name | м² (квадратный метр) | CP | InvoiceGoods_7.goods_supplementary_uom_name | графа 31 (TOVG/NAME_EDI) | |
| 10 | goods[1].tovg[5].gross_weight | 25.00 | CP | InvoiceGoods_7.GrossWeightQuantity | графа 35 (TOVG/G31_35) | |
| 11 | goods[1].tovg[5].net_weight | 24.00 | CP | InvoiceGoods_7.NetWeightQuantity | графа 38 (TOVG/G31_38) | |
| 12 | goods[1].tovg[5].invoice_cost | 6720.00 | CP | InvoiceGoods_7.TotalCost | графа 42 (TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### Итого, по массиву tovg:
- `array_elements`: 5
- `item_fields`: всего полей 60 из 60
- `array_status`: confirmed

#### Итого, по товару goods_1:
- `item_fields`: 18 (g31:4 + 32-38:9 + 42-46:5) + txt:5 + tovg:60 = 83
- `goods_status`: pending (есть птп: tnved_flag_1, tnved_flag_2, preference, procedure_code, mos_code_main, mos_code_extra, customs_value, statistical_value)

#### goods_2 (код 7019900095)

##### 3.16.1. Графа 31 — описание товара

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[2].g31.name | МОСКИТНАЯ СЕТКА ИЗ СТЕКЛОВОЛОКНА «АНТИМОШКА», В РУЛОНАХ, СМ.ДОПОЛНЕНИЕ | D | goods[2].tovg | графа 31 (G_31/NAME) | |
| 2 | goods[2].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | goods[2].tovg.manufacturer | графа 31 (G_31/FIRMA) | |
| 3 | goods[2].g31.trademark | ОТСУТСТВУЕТ | D | goods[2].tovg.trade_mark | графа 31 (G_31/TM) | |
| 4 | goods[2].places | 27 | D | non_formalized.svh_1.goods_1.places (tnved=7019900095) | графа 31 (G_31/PLACE) | |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

##### 3.16.2. Графы 32–38

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[2].item_no | 2 | D | порядковый номер | графа 32 (G_32_1) | |
| 2 | goods[2].tnved_code | 7019900095 | D | InvoiceGoods_5.GoodsCode | графа 33 (G_33_1) | |
| 3 | goods[2].tnved.flag_1 | | pending | птп | графа 33 (G_33_4) | |
| 4 | goods[2].tnved.flag_2 | | pending | птп | графа 33 (G_33_5) | |
| 5 | goods[2].origin_country_code | CN | D | InvoiceGoods_*.OriginCountryCode=156→cb:country | графа 34 (G_34_1) | |
| 6 | goods[2].gross_weight | 1710 | D | non_formalized.svh_1.goods_1.gross_weight_kg | графа 35 (G_35_1) | приоритет SVH |
| 7 | goods[2].preference | | pending | птп | графа 36 (G_36_2) | |
| 8 | goods[2].procedure_code | 4000000 | D | ИМ40 | графа 37 (G_37_1) | птп |
| 9 | goods[2].net_weight | 1614.60 | D | сумма InvoiceGoods_5,6.NetWeightQuantity | графа 38 (G_38_1) | 491.40+1123.20 |

#### Итого, по элементу массива:
- `item_fields`: 9 из 9

##### 3.16.3. Графы 42–46

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[2].invoice_cost | 42228.00 | D | сумма InvoiceGoods_5,6.TotalCost | графа 42 (G_42_1) | |
| 2 | goods[2].mos_code_main | | pending | птп | графа 43 (G_43_1) | |
| 3 | goods[2].mos_code_extra | | pending | птп | графа 43 (G_43_2) | |
| 4 | goods[2].customs_value | | pending | правило | графа 45 (G_45_0, G_45_1) | |
| 5 | goods[2].statistical_value | | pending | правило | графа 46 (G_46_1) | |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

##### 3.16.5. Дополнение к графе 31 — TXT

#### txt_1

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[2].txt[1].text | 1) MIDGE MESH FIBERGLASS 1.4*30 M2 / СЕТКА «АНТИМОШКА» 1,4*30 М2, СТЕКЛОВОЛОКНО, 3780 М2, 520.00/491.40 КГ, 12852.00 CNY | D | InvoiceGoods_5 | графа 31 дополнение | |

#### Итого, по элементу массива:
- `item_fields`: 1 из 1

#### txt_2

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[2].txt[2].text | 2) MIDGE MESH FIBERGLASS 1.6*30 M2 / СЕТКА «АНТИМОШКА» 1,6*30 М2, СТЕКЛОВОЛОКНО, 8640 М2, 1190.00/1123.20 КГ, 29376.00 CNY | D | InvoiceGoods_6 | графа 31 дополнение | |

#### Итого, по элементу массива:
- `item_fields`: 1 из 1

#### Итого, по массиву txt:
- `array_elements`: 2
- `item_fields`: всего полей 2 из 2
- `array_status`: confirmed

##### 3.16.6. Таблица TOVG

#### tovg_1 (строка инвойса 5)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[2].tovg[1].line_no | 1 | D | порядковый номер | графа 31 (TOVG/G32G) | |
| 2 | goods[2].tovg[1].description | MIDGE MESH FIBERGLASS 1.4*30 M2 / СЕТКА «АНТИМОШКА» 1,4*30 М2, СТЕКЛОВОЛОКНО | D | InvoiceGoods_5.GoodsDescription | графа 31 (TOVG/G31_1) | |
| 3 | goods[2].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | InvoiceGoods_5.Manufacturer | графа 31 (TOVG/G31_11) | |
| 4 | goods[2].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_5.TradeMark | графа 31 (TOVG/G31_12) | |
| 5 | goods[2].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_5.GoodsMark | графа 31 (TOVG/G31_14) | |
| 6 | goods[2].tovg[1].model | NOT APPLICABLE | D | InvoiceGoods_5.GoodsModel | графа 31 (TOVG/G31_15_MOD) | |
| 7 | goods[2].tovg[1].quantity | 3780 | CP | InvoiceGoods_5.goods_supplementary_quantity | графа 31 (TOVG/KOLVO) | |
| 8 | goods[2].tovg[1].unit_code | 055 | D | cb:unit: м² | графа 31 (TOVG/CODE_EDI) | |
| 9 | goods[2].tovg[1].unit_name | м² (квадратный метр) | CP | InvoiceGoods_5.goods_supplementary_uom_name | графа 31 (TOVG/NAME_EDI) | |
| 10 | goods[2].tovg[1].gross_weight | 520.00 | CP | InvoiceGoods_5.GrossWeightQuantity | графа 35 (TOVG/G31_35) | |
| 11 | goods[2].tovg[1].net_weight | 491.40 | CP | InvoiceGoods_5.NetWeightQuantity | графа 38 (TOVG/G31_38) | |
| 12 | goods[2].tovg[1].invoice_cost | 12852.00 | CP | InvoiceGoods_5.TotalCost | графа 42 (TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### tovg_2 (строка инвойса 6)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | goods[2].tovg[2].line_no | 2 | D | порядковый номер | графа 31 (TOVG/G32G) | |
| 2 | goods[2].tovg[2].description | MIDGE MESH FIBERGLASS 1.6*30 M2 / СЕТКА «АНТИМОШКА» 1,6*30 М2, СТЕКЛОВОЛОКНО | D | InvoiceGoods_6.GoodsDescription | графа 31 (TOVG/G31_1) | |
| 3 | goods[2].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | InvoiceGoods_6.Manufacturer | графа 31 (TOVG/G31_11) | |
| 4 | goods[2].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_6.TradeMark | графа 31 (TOVG/G31_12) | |
| 5 | goods[2].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | InvoiceGoods_6.GoodsMark | графа 31 (TOVG/G31_14) | |
| 6 | goods[2].tovg[2].model | NOT APPLICABLE | D | InvoiceGoods_6.GoodsModel | графа 31 (TOVG/G31_15_MOD) | |
| 7 | goods[2].tovg[2].quantity | 8640 | CP | InvoiceGoods_6.goods_supplementary_quantity | графа 31 (TOVG/KOLVO) | |
| 8 | goods[2].tovg[2].unit_code | 055 | D | cb:unit: м² | графа 31 (TOVG/CODE_EDI) | |
| 9 | goods[2].tovg[2].unit_name | м² (квадратный метр) | CP | InvoiceGoods_6.goods_supplementary_uom_name | графа 31 (TOVG/NAME_EDI) | |
| 10 | goods[2].tovg[2].gross_weight | 1190.00 | CP | InvoiceGoods_6.GrossWeightQuantity | графа 35 (TOVG/G31_35) | |
| 11 | goods[2].tovg[2].net_weight | 1123.20 | CP | InvoiceGoods_6.NetWeightQuantity | графа 38 (TOVG/G31_38) | |
| 12 | goods[2].tovg[2].invoice_cost | 29376.00 | CP | InvoiceGoods_6.TotalCost | графа 42 (TOVG/INVOICCOST) | |

#### Итого, по элементу массива:
- `item_fields`: 12 из 12

#### Итого, по массиву tovg:
- `array_elements`: 2
- `item_fields`: всего полей 24 из 24
- `array_status`: confirmed

#### Итого, по товару goods_2:
- `item_fields`: 18 + txt:2 + tovg:24 = 44
- `goods_status`: pending (есть птп)

#### Итого, по массиву goods:
- `array_elements`: 2
- `total_goods_fields`: 83 + 44 = 127
- `array_status`: pending

### 3.17. Теги после товаров (графы 51–54)

#### 3.17.1. Графа 42 (доп. признак)

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | declaration.g42_2 | | pending | птп | графа 42 (G_42_2) | |

#### Итого, по разделу:
- `fields`: 1 из 1
- `partition_status`: pending

#### 3.17.3. Графа 54 — уполномоченное лицо / представитель

| num | field | value | status | source | description | note |
|-----|-------|-------|--------|--------|-------------|------|
| 1 | representative.date | | pending | птп | графа 54 (G_54_20) | требуется дата подачи ДТ |
| 2 | representative.phone | +7-927-030-70-07 | CP | master_data.representative.phone | графа 54 (G_54_21) | |
| 3 | representative.email | | CP | master_data.representative.email | графа 54 (G_54_EMAIL) | пусто |
| 4 | representative.last_name | АРБУЗОВА | CP | master_data.representative.last_name | графа 54 (G_54_3) | |
| 5 | representative.first_name | АНАСТАСИЯ | CP | master_data.representative.first_name | графа 54 (G_54_3NM) | |
| 6 | representative.middle_name | КОНСТАНТИНОВНА | CP | master_data.representative.middle_name | графа 54 (G_54_3MD) | |
| 7 | representative.authority_doc_name | ДОВЕРЕННОСТЬ | CP | master_data.representative.authority_doc_name | графа 54 (G_54_4) | |
| 8 | representative.authority_doc_number | 1 | CP | master_data.representative.authority_doc_number | графа 54 (G_54_5) | |
| 9 | representative.authority_doc_date_from | 2026-02-01 | CP | master_data.representative.authority_doc_date_from | графа 54 (G_54_60) | |
| 10 | representative.authority_doc_date_to | 2026-12-31 | CP | master_data.representative.authority_doc_date_to | графа 54 (G_54_61) | |
| 11 | representative.position | УПОЛНОМОЧЕННОЕ ЛИЦО | CP | master_data.representative.position | графа 54 (G_54_7) | |
| 12 | representative.passport_code | RU01001 | CP | master_data.representative.passport_code | графа 54 (G_54_8) | |
| 13 | representative.passport_name | ПАСРФ | CP | master_data.representative.passport_name | графа 54 (G_54_9) | |
| 14 | representative.passport_number | 449948 | CP | master_data.representative.passport_number | графа 54 (G_54_100) | |
| 15 | representative.passport_date | 2010-03-11 | CP | master_data.representative.passport_date | графа 54 (G_54_101) | |
| 16 | representative.passport_series | 63 09 | CP | master_data.representative.passport_series | графа 54 (G_54_12) | |
| 17 | representative.passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CP | master_data.representative.passport_issuer | графа 54 (G_54_13) | |
| 18 | representative.printed_block | АРБУЗОВА АНАСТАСИЯ КОНСТАНТИНОВНА, ПАСРФ 63 09 449948 ОТ 2010-03-11, УПОЛНОМОЧЕННОЕ ЛИЦО, ДОВЕРЕННОСТЬ №1 ОТ 01.02.2026 ДО 31.12.2026, +7-927-030-70-07 | D | representative.* | графа 54 (G_54P) | птп |

#### Итого, по разделу:
- `fields`: 18 из 18
- `partition_status`: pending (representative.date pending; printed_block птп)

### Итог:
- `total_fields`: разделы 3.1(3)+3.2(6)+3.3(3)+3.4(15)+3.5(16)+3.6(1)+3.7(14)+3.8(6)+3.9(2)+3.10(5)+3.11(3)+3.12(1)+3.13(2)+3.14(2)+3.15(10)+3.16(127)+3.17.1(1)+3.17.3(18)=235
- `dt_status`: pending

## Раздел II: Issues (нерешенные вопросы)

**Для полей:**
- `goods[*].tnved.flag_1`
  - `question`: Нужны ли литеры после кода ТН ВЭД для графы 33 (G_33_4)?

- `goods[*].tnved.flag_2`
  - `question`: Нужны ли литеры после кода ТН ВЭД для графы 33 (G_33_5)?

- `goods[*].preference`
  - `question`: Какой код преференции (графа 36, G_36_2) применить для товаров из Китая? (например, ОД — развивающиеся страны)

- `goods[*].procedure_code`
  - `question`: Код процедуры для товара — 4000000 (ИМ40) подтвержден?

- `goods[*].mos_code_main`
  - `question`: Код МОС (метода определения стоимости) для графы 43 (G_43_1)?

- `goods[*].mos_code_extra`
  - `question`: Доп. код МОС для графы 43 (G_43_2)?

- `declaration.g42_2`
  - `question`: Доп. признак графы 42 (G_42_2) — например, «В ДТС»? Нужен?

- `representative.date`
  - `question`: Дата подачи/заполнения ДТ (G_54_20)?
