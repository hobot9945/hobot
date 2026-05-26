# Исходные данные для ДТ

## Метаданные:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `тип поставки`: 1 ДТ / 2 товара
- `агрегация ДТ`: 2 товара (группировка по GoodsCode: 5804101000, 7019900095)
- `источники данных`: primary.md + master_data.md

## Часть I: Поля ДТ

### 3.1 Заголовок декларации (графа 1)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | declaration.direction | ИМ | CD | направление декларации (импорт / экспорт) | meta.direction |
| 02 | declaration.procedure | 40 | CO | код таможенной процедуры | cb:procedure |
| 03 | declaration.form | ЭД | D | форма подачи декларации | для Альты всегда ЭД |

- _audit: 3

### 3.2 Отправитель (графа 2)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | sender.country_name | Китай | CP | текстовое название страны | formalized.invoice_1.Seler_PostalAddress_CounryName |
| 02 | sender.country_code | CN | CP | код страны alpha-2 | formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 03 | sender.name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CP | полное наименование отправителя | formalized.invoice_1.Seler_Name |
| 04 | sender.region | Hebei | CP | область/район | formalized.invoice_1.Seler_PostalAddress_Region |
| 05 | sender.city | Shijiazhuang | CP | город | formalized.invoice_1.Seler_PostalAddress_City |
| 06 | sender.street | No. 5 Gaodong street | CP | улица и дом | formalized.invoice_1.Seler_PostalAddress_StreetHouse |

- _audit: 6

### 3.3 Количество товаров и мест (графы 5, 6)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.total_goods_number | 2 | D | гр. 5 — количество товарных позиций в ДТ | размер массива goods |
| 02 | shipment.packages_flag | true | D | гр. 6 — признак подсчёта мест | всегда true |
| 03 | shipment.total_packages | 127 | D | гр. 6 — общее количество грузовых мест | non_formalized.svh_1.actual_places |

- _audit: 3

### 3.4 Получатель (графа 8)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | consignee.same_as_declarant | true | D | графа 8 — признак «см. графу 14» | константа |
| 02 | consignee.ogrn | 1201600020390 | D | графа 8 — ОГРН получателя | из графы 14 |
| 03 | consignee.inn_kpp | 1650389298/165001001 | D | графа 8 — ИНН/КПП через "/" | из графы 14 |
| 04 | consignee.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | графа 8 — наименование организации | из графы 14 |
| 05 | consignee.country_code | RU | D | графа 8 — код страны alpha-2 | из графы 14 |
| 06 | consignee.country_name | РОССИЯ | D | графа 8 — страна, наименование | из графы 14 |
| 07 | consignee.postcode | 423800 | D | графа 8 — почтовый индекс | из графы 14 |
| 08 | consignee.region | РЕСПУБЛИКА ТАТАРСТАН | D | графа 8 — регион | из графы 14 |
| 09 | consignee.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | графа 8 — населённый пункт | из графы 14 |
| 10 | consignee.street | проезд Хлебный | D | графа 8 — улица | из графы 14 |
| 11 | consignee.building | 30 | D | графа 8 — дом | из графы 14 |
| 12 | consignee.room | 211 | D | графа 8 — помещение/офис | из графы 14 |
| 13 | consignee.phone | +7 (843) 207 18 90 | D | графа 8 — телефон | из графы 14 |
| 14 | consignee.email | PROM_TAT@MAIL.RU | D | графа 8 — e-mail | из графы 14 |

- _audit: 14

### 3.5 Финансовое урегулирование (графа 9)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | financial.same_as_declarant | true | D | графа 9 — признак «см. графу 14» | константа |
| 02 | financial.ogrn | 1201600020390 | D | графа 9 — ОГРН | из графы 14 |
| 03 | financial.inn_kpp | 1650389298/165001001 | D | графа 9 — ИНН/КПП | из графы 14 |
| 04 | financial.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | графа 9 — наименование | из графы 14 |
| 05 | financial.country_code | RU | D | графа 9 — код страны | из графы 14 |
| 06 | financial.country_name | РОССИЯ | D | графа 9 — наименование страны | из графы 14 |
| 07 | financial.postcode | 423800 | D | графа 9 — индекс | из графы 14 |
| 08 | financial.region | РЕСПУБЛИКА ТАТАРСТАН | D | графа 9 — регион | из графы 14 |
| 09 | financial.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | графа 9 — город | из графы 14 |
| 10 | financial.street | проезд Хлебный | D | графа 9 — улица | из графы 14 |
| 11 | financial.building | 30 | D | графа 9 — дом | из графы 14 |
| 12 | financial.room | 211 | D | графа 9 — помещение | из графы 14 |
| 13 | financial.country_code_alt | RU | D | графа 9 — дублирующий код страны | declarant.country_code |
| 14 | financial.phone | +7 (843) 207 18 90 | D | графа 9 — телефон | из графы 14 |
| 15 | financial.email | PROM_TAT@MAIL.RU | D | графа 9 — e-mail | из графы 14 |

- _audit: 15

### 3.6 Торгующая страна (графа 11)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.trade_country_code | CN | CP | графа 11 — код торгующей страны alpha-2 | formalized.invoice_1.DeliveryTerms_TradingCountryCode |

- _audit: 1

### 3.7 Декларант (графа 14)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | declarant.ogrn | 1201600020390 | CP | графа 14 — ОГРН декларанта | formalized.letter_of_attorney_1.Organization_OGRN |
| 02 | declarant.inn_kpp | 1650389298/165001001 | D | графа 14 — ИНН/КПП через "/" | formalized.letter_of_attorney_1.Organization_INN + "/" + KPP |
| 03 | declarant.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CP | графа 14 — наименование организации | formalized.letter_of_attorney_1.Organization_OrganizationName |
| 04 | declarant.country_code | RU | CP | графа 14 — код страны | formalized.letter_of_attorney_1.Organization_Address_CountryCode |
| 05 | declarant.country_name | РОССИЯ | CP | графа 14 — наименование страны | formalized.letter_of_attorney_1.Organization_Address_CounryName |
| 06 | declarant.postcode | 423800 | CP | графа 14 — почтовый индекс | formalized.letter_of_attorney_1.Organization_Address_PostalCode |
| 07 | declarant.region | РЕСПУБЛИКА ТАТАРСТАН | CP | графа 14 — регион | formalized.letter_of_attorney_1.Organization_Address_Region |
| 08 | declarant.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | графа 14 — населённый пункт | formalized.letter_of_attorney_1.Organization_Address_City |
| 09 | declarant.street | проезд Хлебный, д. 30, офис 211 | CP | графа 14 — улица | formalized.letter_of_attorney_1.Organization_Address_StreetHouse |
| 10 | declarant.building | 30 | D | графа 14 — дом | извлечено из Organization_Address_StreetHouse |
| 11 | declarant.room | 211 | D | графа 14 — помещение/офис | извлечено из Organization_Address_StreetHouse |
| 12 | declarant.phone | +7 (843) 207 18 90 | CD | графа 14 — телефон | master_data.declarant.phone |
| 13 | declarant.email | PROM_TAT@MAIL.RU | CD | графа 14 — e-mail | master_data.declarant.email |

- _audit: 13

### 3.8 Страны (графы 15, 16, 17)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.dispatch_country_code | CN | CP | графа 15A — код страны отправления alpha-2 | formalized.invoice_1.DeliveryTerms_DispatchCountryCode |
| 02 | shipment.destination_country_code | RU | CP | графа 17A — код страны назначения alpha-2 | formalized.invoice_1.DeliveryTerms_DestinationCountryCode |
| 03 | shipment.dispatch_country_name | Китай | D | графа 15 — страна отправления, текст | cb:country (CN) |
| 04 | shipment.destination_country_name | Россия | D | графа 17 — страна назначения, текст | cb:country (RU) |
| 05 | shipment.origin_country_code | CN | D | графа 16 — код страны происхождения alpha-2 | все InvoiceGoods OriginCountryCode=156 -> CN |
| 06 | shipment.origin_country_name | Китай | D | графа 16 — страна происхождения, текст | cb:country (CN) |

- _audit: 6

### 3.9 Условия поставки (графа 20)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | delivery.terms_code | EXW | D | графа 20 — условия поставки | formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode |
| 02 | delivery.place_name | HEBEI | D | графа 20 — место поставки | formalized.invoice_1.DeliveryTerms_DeliveryPlace |

- _audit: 2

### 3.10 Транспорт (графы 18, 19, 21)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | transport.vehicles_count | 2 | D | графа 18 — количество транспортных средств | число блоков TransportMeans в packing_list_1 |
| 02 | transport.identification | O157AO774/BT374974 | D | графа 18 — идентификация ТС | join TransportMeans_*.Number |
| 03 | transport.registration_country_code | 00 | D | графа 18 — код страны регистрации ТС | NationalityCode=000 -> 00 |
| 04 | transport.container_flag | 0 | CO | графа 19 — признак контейнера | заглушка 0 |
| 05 | transport.border_mode | 1 | D | графа 21 — код активного ТС на границе | автоперевозка (есть cmr_1) |

- _audit: 5

### 3.11 Валюта и стоимость (графа 22)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.invoice_currency_numeric | 156 | D | графа 22 — цифровой код валюты | CNY -> 156 по cb:currency_okv |
| 02 | shipment.invoice_currency_alpha | CNY | CP | графа 22 — буквенный код валюты | formalized.invoice_1.CurrencyCode |
| 03 | shipment.invoice_amount | 97260.00 | CP | графа 22 — сумма по счёту | formalized.invoice_1.TotalCost |

- _audit: 3

### 3.12 Курс валюты (графа 23)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.currency_rate | 10.9430 | CP | графа 23 — курс валюты к рублю | formalized.invoice_1.CurrencyRate |

- _audit: 1

### 3.13 Вид транспорта (графы 25, 26)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | transport.border_transport_code | 31 | D | графа 25 — код вида транспорта на границе | для автотранспорта |
| 02 | transport.internal_transport_code | 31 | D | графа 26 — код вида транспорта внутри страны | для автотранспорта |

- _audit: 2

### 3.14 Таможня на границе (графа 29)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | customs.border_code | 10719110 | CP | графа 29 — код таможенного органа на границе | non_formalized.transit_declaration_1.customs_post_code |
| 02 | customs.border_name | ОТО И ТК №3 Таможенный пост Набережночелнинский | CP | графа 29 — наименование таможенного поста | non_formalized.transit_declaration_1.customs_post_name |

- _audit: 2

### 3.15 Местонахождение товаров (графа 30)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | location.type | 11 | D | графа 30 — тип места нахождения товаров | для СВХ |
| 02 | location.document_kind | 2 | D | графа 30 — вид документа | для лицензии СВХ |
| 03 | location.document_number | 10404/141210/10092/5 | CP | графа 30 — номер документа СВХ | non_formalized.svh_1.warehouse_license_number |
| 04 | location.document_date | 18.09.2025 | CP | графа 30 — дата документа СВХ | non_formalized.svh_1.warehouse_license_date |
| 05 | location.address.country_code | RU | D | графа 30 — код страны местонахождения | склад в РФ |
| 06 | location.address.region | РЕСПУБЛИКА ТАТАРСТАН | CP | графа 30 — регион | non_formalized.svh_additional_sheet_1.svh_address_region |
| 07 | location.address.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | графа 30 — город | non_formalized.svh_additional_sheet_1.svh_address_city |
| 08 | location.address.street | ПРОИЗВОДСТВЕННЫЙ ПР-Д, Д. 45 | CP | графа 30 — улица и дом | non_formalized.svh_additional_sheet_1.svh_address_street_house |
| 09 | location.customs_code | 10404083 | CP | графа 30 — код таможенного органа СВХ | non_formalized.svh_additional_sheet_1.svh_customs_code |

- _audit: 9

### 3.16 Массив: goods[]
- goods._array_audit: 2

#### 3.16.0 Элемент массива: goods[1]
- goods._element_num: 1

#### 3.16.1 Графа 31 — описание товаров

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g31.name | Москитные сетки из полиэстера различных типов и размеров. СМ.ДОПОЛНЕНИЕ | D | графа 31 — описание товара | обобщенное описание + СМ.ДОПОЛНЕНИЕ |
| 02 | goods[1].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | графа 31 — производитель | у всех строк группы один производитель |
| 03 | goods[1].g31.trade_mark | ОТСУТСТВУЕТ | D | графа 31 — товарный знак / ТМ | у всех строк группы ТМ отсутствует |
| 04 | goods[1].places | 100 | D | графа 31 — количество мест по товару | non_formalized.svh_1.goods[2].places (tnved=5804101000) |

- _audit: 4

#### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].item_no | 1 | D | графа 32 — номер товара | порядковый номер |
| 02 | goods[1].tnved_code | 5804101000 | D | графа 33 — код товара | GoodsCode группы |
| 03 | goods[1].tnved.flag_1 | С | D | графа 33 — доп. признак | заглушка С |
| 04 | goods[1].tnved.flag_2 | N | D | графа 33 — доп. признак | нет торговой марки |
| 05 | goods[1].origin_country_code | CN | D | графа 34 — код страны происхождения | OriginCountryCode=156 -> CN |
| 06 | goods[1].gross_weight | 1790.00 | D | графа 35 — вес брутто по товару | non_formalized.svh_1.goods[2].gross_weight_kg |
| 07 | goods[1].preference | ОООО-ОО | D | графа 36 — преференция | отсутствие преференций |
| 08 | goods[1].net_weight | 1687.40 | D | графа 38 — вес нетто по товару | сумма нетто по строкам группы: 806.60+460.80+252.00+144.00+24.00 |

- _audit: 8

#### 3.16.3 Графы 42–46 — стоимости по товару

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].invoice_cost | 55032.00 | D | графа 42 — цена товара | сумма стоимости по строкам группы: 14742+8424+16002+9144+6720 |
| 02 | goods[1].customs_value | 608215.44 | D | графа 45 — таможенная стоимость | goods[1].invoice_cost_in_rub (602217.60) + transport_to_border + insurance_to_border |
| 03 | goods[1].transport_to_border | 5087.50 | D | графа 45 — транспорт до границы | доля в общих расходах пропорционально брутто: (1790/3500)*1404*9.9430 |

- _audit: 3

#### 3.16.4 Массив: goods[1].txt[]
- _array_audit: 5

#### 3.16.5 Элемент массива: goods[1].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].txt[1].line_1 | АРТ: - 60 рулонов | D | графа 31 — TXT строка 1 | InvoiceGoods[1].GoodsQuantity + MeasureUnitQualifierName |
| 02 | goods[1].txt[1].line_2 | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | D | графа 31 — TXT строка 2 | InvoiceGoods[1].GoodsDescription |

- _item_audit: 2

#### 3.16.5 Элемент массива: goods[1].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].txt[2].line_1 | АРТ: - 30 рулонов | D | графа 31 — TXT строка 1 | InvoiceGoods[2].GoodsQuantity + MeasureUnitQualifierName |
| 02 | goods[1].txt[2].line_2 | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | D | графа 31 — TXT строка 2 | InvoiceGoods[2].GoodsDescription |

- _item_audit: 2

#### 3.16.5 Элемент массива: goods[1].txt[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].txt[3].line_1 | АРТ: - 60 рулонов | D | графа 31 — TXT строка 1 | InvoiceGoods[3].GoodsQuantity + MeasureUnitQualifierName |
| 02 | goods[1].txt[3].line_2 | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | D | графа 31 — TXT строка 2 | InvoiceGoods[3].GoodsDescription |

- _item_audit: 2

#### 3.16.5 Элемент массива: goods[1].txt[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].txt[4].line_1 | АРТ: - 30 рулонов | D | графа 31 — TXT строка 1 | InvoiceGoods[4].GoodsQuantity + MeasureUnitQualifierName |
| 02 | goods[1].txt[4].line_2 | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | D | графа 31 — TXT строка 2 | InvoiceGoods[4].GoodsDescription |

- _item_audit: 2

#### 3.16.5 Элемент массива: goods[1].txt[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].txt[5].line_1 | АРТ: - 5 рулонов | D | графа 31 — TXT строка 1 | InvoiceGoods[7].GoodsQuantity + MeasureUnitQualifierName |
| 02 | goods[1].txt[5].line_2 | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | D | графа 31 — TXT строка 2 | InvoiceGoods[7].GoodsDescription |

- _item_audit: 2

#### 3.16.6 Массив: goods[1].tovg[]
- _array_audit: 5

#### 3.16.7 Элемент массива: goods[1].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].tovg[1].line_no | 1 | D | графа 31 — № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[1].description | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | D | графа 31 — наименование | InvoiceGoods[1].GoodsDescription |
| 03 | goods[1].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | графа 31 — производитель | InvoiceGoods[1].AdditionalGoodsDescription_Manufacturer |
| 04 | goods[1].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | графа 31 — марка/ТМ | InvoiceGoods[1].AdditionalGoodsDescription_TradeMark |
| 05 | goods[1].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | графа 31 — товарный знак | InvoiceGoods[1].AdditionalGoodsDescription_GoodsMark |
| 06 | goods[1].tovg[1].model | NOT APPLICABLE | D | графа 31 — модель/модификация | InvoiceGoods[1].AdditionalGoodsDescription_GoodsModel |
| 07 | goods[1].tovg[1].quantity | 2520 | CP | графа 31 — количество в доп.ед.изм | InvoiceGoods[1].goods_supplementary_quantity |
| 08 | goods[1].tovg[1].unit_code | 055 | D | графа 31 — код ЕИ | M2 -> 055 по cb:unit |
| 09 | goods[1].tovg[1].unit_name | M2 | CP | графа 31 — наименование ЕИ | InvoiceGoods[1].goods_supplementary_uom_name |
| 10 | goods[1].tovg[1].gross_weight | 855.00 | CP | графа 35 — вес брутто по строке | InvoiceGoods[1].GrossWeightQuantity |
| 11 | goods[1].tovg[1].net_weight | 806.60 | CP | графа 38 — вес нетто по строке | InvoiceGoods[1].NetWeightQuantity |
| 12 | goods[1].tovg[1].invoice_cost | 14742.00 | CP | графа 42 — цена по строке | InvoiceGoods[1].TotalCost |

- _item_audit: 12

#### 3.16.7 Элемент массива: goods[1].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].tovg[2].line_no | 2 | D | графа 31 — № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[2].description | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | D | графа 31 — наименование | InvoiceGoods[2].GoodsDescription |
| 03 | goods[1].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | графа 31 — производитель | InvoiceGoods[2].AdditionalGoodsDescription_Manufacturer |
| 04 | goods[1].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | графа 31 — марка/ТМ | InvoiceGoods[2].AdditionalGoodsDescription_TradeMark |
| 05 | goods[1].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | графа 31 — товарный знак | InvoiceGoods[2].AdditionalGoodsDescription_GoodsMark |
| 06 | goods[1].tovg[2].model | NOT APPLICABLE | D | графа 31 — модель/модификация | InvoiceGoods[2].AdditionalGoodsDescription_GoodsModel |
| 07 | goods[1].tovg[2].quantity | 1440 | CP | графа 31 — количество в доп.ед.изм | InvoiceGoods[2].goods_supplementary_quantity |
| 08 | goods[1].tovg[2].unit_code | 055 | D | графа 31 — код ЕИ | M2 -> 055 по cb:unit |
| 09 | goods[1].tovg[2].unit_name | M2 | CP | графа 31 — наименование ЕИ | InvoiceGoods[2].goods_supplementary_uom_name |
| 10 | goods[1].tovg[2].gross_weight | 490.00 | CP | графа 35 — вес брутто по строке | InvoiceGoods[2].GrossWeightQuantity |
| 11 | goods[1].tovg[2].net_weight | 460.80 | CP | графа 38 — вес нетто по строке | InvoiceGoods[2].NetWeightQuantity |
| 12 | goods[1].tovg[2].invoice_cost | 8424.00 | CP | графа 42 — цена по строке | InvoiceGoods[2].TotalCost |

- _item_audit: 12

#### 3.16.7 Элемент массива: goods[1].tovg[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].tovg[3].line_no | 3 | D | графа 31 — № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[3].description | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | D | графа 31 — наименование | InvoiceGoods[3].GoodsDescription |
| 03 | goods[1].tovg[3].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | графа 31 — производитель | InvoiceGoods[3].AdditionalGoodsDescription_Manufacturer |
| 04 | goods[1].tovg[3].trade_mark | ОТСУТСТВУЕТ | D | графа 31 — марка/ТМ | InvoiceGoods[3].AdditionalGoodsDescription_TradeMark |
| 05 | goods[1].tovg[3].goods_mark | ОТСУТСТВУЕТ | D | графа 31 — товарный знак | InvoiceGoods[3].AdditionalGoodsDescription_GoodsMark |
| 06 | goods[1].tovg[3].model | NOT APPLICABLE | D | графа 31 — модель/модификация | InvoiceGoods[3].AdditionalGoodsDescription_GoodsModel |
| 07 | goods[1].tovg[3].quantity | 2520 | CP | графа 31 — количество в доп.ед.изм | InvoiceGoods[3].goods_supplementary_quantity |
| 08 | goods[1].tovg[3].unit_code | 055 | D | графа 31 — код ЕИ | M2 -> 055 по cb:unit |
| 09 | goods[1].tovg[3].unit_name | M2 | CP | графа 31 — наименование ЕИ | InvoiceGoods[3].goods_supplementary_uom_name |
| 10 | goods[1].tovg[3].gross_weight | 265.00 | CP | графа 35 — вес брутто по строке | InvoiceGoods[3].GrossWeightQuantity |
| 11 | goods[1].tovg[3].net_weight | 252.00 | CP | графа 38 — вес нетто по строке | InvoiceGoods[3].NetWeightQuantity |
| 12 | goods[1].tovg[3].invoice_cost | 16002.00 | CP | графа 42 — цена по строке | InvoiceGoods[3].TotalCost |

- _item_audit: 12

#### 3.16.7 Элемент массива: goods[1].tovg[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].tovg[4].line_no | 4 | D | графа 31 — № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[4].description | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | D | графа 31 — наименование | InvoiceGoods[4].GoodsDescription |
| 03 | goods[1].tovg[4].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | графа 31 — производитель | InvoiceGoods[4].AdditionalGoodsDescription_Manufacturer |
| 04 | goods[1].tovg[4].trade_mark | ОТСУТСТВУЕТ | D | графа 31 — марка/ТМ | InvoiceGoods[4].AdditionalGoodsDescription_TradeMark |
| 05 | goods[1].tovg[4].goods_mark | ОТСУТСТВУЕТ | D | графа 31 — товарный знак | InvoiceGoods[4].AdditionalGoodsDescription_GoodsMark |
| 06 | goods[1].tovg[4].model | NOT APPLICABLE | D | графа 31 — модель/модификация | InvoiceGoods[4].AdditionalGoodsDescription_GoodsModel |
| 07 | goods[1].tovg[4].quantity | 1440 | CP | графа 31 — количество в доп.ед.изм | InvoiceGoods[4].goods_supplementary_quantity |
| 08 | goods[1].tovg[4].unit_code | 055 | D | графа 31 — код ЕИ | M2 -> 055 по cb:unit |
| 09 | goods[1].tovg[4].unit_name | M2 | CP | графа 31 — наименование ЕИ | InvoiceGoods[4].goods_supplementary_uom_name |
| 10 | goods[1].tovg[4].gross_weight | 155.00 | CP | графа 35 — вес брутто по строке | InvoiceGoods[4].GrossWeightQuantity |
| 11 | goods[1].tovg[4].net_weight | 144.00 | CP | графа 38 — вес нетто по строке | InvoiceGoods[4].NetWeightQuantity |
| 12 | goods[1].tovg[4].invoice_cost | 9144.00 | CP | графа 42 — цена по строке | InvoiceGoods[4].TotalCost |

- _item_audit: 12

#### 3.16.7 Элемент массива: goods[1].tovg[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].tovg[5].line_no | 5 | D | графа 31 — № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[5].description | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | D | графа 31 — наименование | InvoiceGoods[7].GoodsDescription |
| 03 | goods[1].tovg[5].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | графа 31 — производитель | InvoiceGoods[7].AdditionalGoodsDescription_Manufacturer |
| 04 | goods[1].tovg[5].trade_mark | ОТСУТСТВУЕТ | D | графа 31 — марка/ТМ | InvoiceGoods[7].AdditionalGoodsDescription_TradeMark |
| 05 | goods[1].tovg[5].goods_mark | ОТСУТСТВУЕТ | D | графа 31 — товарный знак | InvoiceGoods[7].AdditionalGoodsDescription_GoodsMark |
| 06 | goods[1].tovg[5].model | NOT APPLICABLE | D | графа 31 — модель/модификация | InvoiceGoods[7].AdditionalGoodsDescription_GoodsModel |
| 07 | goods[1].tovg[5].quantity | 240 | CP | графа 31 — количество в доп.ед.изм | InvoiceGoods[7].goods_supplementary_quantity |
| 08 | goods[1].tovg[5].unit_code | 055 | D | графа 31 — код ЕИ | M2 -> 055 по cb:unit |
| 09 | goods[1].tovg[5].unit_name | M2 | CP | графа 31 — наименование ЕИ | InvoiceGoods[7].goods_supplementary_uom_name |
| 10 | goods[1].tovg[5].gross_weight | 25.00 | CP | графа 35 — вес брутто по строке | InvoiceGoods[7].GrossWeightQuantity |
| 11 | goods[1].tovg[5].net_weight | 24.00 | CP | графа 38 — вес нетто по строке | InvoiceGoods[7].NetWeightQuantity |
| 12 | goods[1].tovg[5].invoice_cost | 6720.00 | CP | графа 42 — цена по строке | InvoiceGoods[7].TotalCost |

- _item_audit: 12

#### 3.16.9 Поле G_44 (текстовое поле в карточке товара)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44.text | СМ.ДОПОЛНЕНИЕ | D | графа 44 — текстовое поле | константа |

- _audit: 1

#### 3.16.10 Массив: goods[1].g44_docs[]
- _array_audit: 14

#### 3.16.11 Элемент массива: goods[1].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[1].doc_code | 03011 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[1].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[1].doc_name | КОНТРАКТ | CP | графа 44 — наименование документа | formalized.contract_1 |
| 04 | goods[1].g44_docs[1].doc_number | LM-2553 | CP | графа 44 — номер документа | formalized.contract_1 |
| 05 | goods[1].g44_docs[1].doc_date | 02.07.2025 | CP | графа 44 — дата документа | formalized.contract_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[2].doc_code | 03012 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[2].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | графа 44 — наименование документа | formalized.supplementary_contract_1 |
| 04 | goods[1].g44_docs[2].doc_number | 1 | CP | графа 44 — номер документа | formalized.supplementary_contract_1 |
| 05 | goods[1].g44_docs[2].doc_date | 25.11.2025 | CP | графа 44 — дата документа | formalized.supplementary_contract_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[3].doc_code | 04021 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[3].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[3].doc_name | ИНВОЙС | CP | графа 44 — наименование документа | formalized.invoice_1 |
| 04 | goods[1].g44_docs[3].doc_number | LM-2591 | CP | графа 44 — номер документа | formalized.invoice_1 |
| 05 | goods[1].g44_docs[3].doc_date | 30.10.2025 | CP | графа 44 — дата документа | formalized.invoice_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[4].doc_code | 04131 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[4].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[4].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | графа 44 — наименование документа | formalized.packing_list_1 |
| 04 | goods[1].g44_docs[4].doc_number | LM-2591 | CP | графа 44 — номер документа | formalized.packing_list_1 |
| 05 | goods[1].g44_docs[4].doc_date | 30.10.2025 | CP | графа 44 — дата документа | formalized.packing_list_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[5].doc_code | 02015 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[5].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[5].doc_name | CMR | CP | графа 44 — наименование документа | formalized.cmr_1 |
| 04 | goods[1].g44_docs[5].doc_number | 00378 | CP | графа 44 — номер документа | formalized.cmr_1 |
| 05 | goods[1].g44_docs[5].doc_date | 20.01.2026 | CP | графа 44 — дата документа | formalized.cmr_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[6].doc_code | 04023 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[6].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[6].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | графа 44 — наименование документа | formalized.payment_order_1 |
| 04 | goods[1].g44_docs[6].doc_number | 1 | CP | графа 44 — номер документа | formalized.payment_order_1 |
| 05 | goods[1].g44_docs[6].doc_date | 13.01.2026 | CP | графа 44 — дата документа | formalized.payment_order_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[7].doc_code | 04023 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[7].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[7].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | графа 44 — наименование документа | formalized.payment_order_2 |
| 04 | goods[1].g44_docs[7].doc_number | 7 | CP | графа 44 — номер документа | formalized.payment_order_2 |
| 05 | goods[1].g44_docs[7].doc_date | 28.11.2025 | CP | графа 44 — дата документа | formalized.payment_order_2 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[8].doc_code | 04031 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[8].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[8].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | графа 44 — наименование документа | formalized.service_invoice_1 |
| 04 | goods[1].g44_docs[8].doc_number | 26-00378-tl | CP | графа 44 — номер документа | formalized.service_invoice_1 |
| 05 | goods[1].g44_docs[8].doc_date | 27.01.2026 | CP | графа 44 — дата документа | formalized.service_invoice_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[9].doc_code | 04111 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[9].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[9].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | графа 44 — наименование документа | formalized.insurance_document_1 |
| 04 | goods[1].g44_docs[9].doc_number | 26-00378-tl/1 | CP | графа 44 — номер документа | formalized.insurance_document_1 |
| 05 | goods[1].g44_docs[9].doc_date | 14.01.2026 | CP | графа 44 — дата документа | formalized.insurance_document_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[10].doc_code | 05999 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[10].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[10].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | графа 44 — наименование документа | formalized.tech_description_1 |
| 04 | goods[1].g44_docs[10].doc_number | Б/Н | CP | графа 44 — номер документа | formalized.tech_description_1 |
| 05 | goods[1].g44_docs[10].doc_date | 30.10.2025 | CP | графа 44 — дата документа | formalized.tech_description_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[11].doc_code | 11001 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[11].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[11].doc_name | ПАСПОРТ | CP | графа 44 — наименование документа | formalized.passport_1 |
| 04 | goods[1].g44_docs[11].doc_number | 63 09 449948 | CP | графа 44 — номер документа | formalized.passport_1 |
| 05 | goods[1].g44_docs[11].doc_date | 11.03.2010 | CP | графа 44 — дата документа | formalized.passport_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[12].doc_code | 11004 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[12].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | графа 44 — наименование документа | formalized.letter_of_attorney_1 |
| 04 | goods[1].g44_docs[12].doc_number | 1 | CP | графа 44 — номер документа | formalized.letter_of_attorney_1 |
| 05 | goods[1].g44_docs[12].doc_date | 01.02.2026 | CP | графа 44 — дата документа | formalized.letter_of_attorney_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[13].doc_code | 04033 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[13].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[13].doc_name | ДОГОВОР ТРАНСПОРТНОЙ ЭКСПЕДИЦИИ | CP | графа 44 — наименование документа | formalized.transport_contract_1 |
| 04 | goods[1].g44_docs[13].doc_number | КООО/26651/М | CP | графа 44 — номер документа | formalized.transport_contract_1 |
| 05 | goods[1].g44_docs[13].doc_date | 13.05.2025 | CP | графа 44 — дата документа | formalized.transport_contract_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[14].doc_code | 04011 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[1].g44_docs[14].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[1].g44_docs[14].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | графа 44 — наименование документа | formalized.egrul_1 |
| 04 | goods[1].g44_docs[14].doc_number | ЮЭ9965-25-106893283 | CP | графа 44 — номер документа | formalized.egrul_1 |
| 05 | goods[1].g44_docs[14].doc_date | 12.03.2020 | CP | графа 44 — дата документа | formalized.egrul_1 |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[2]
- goods._element_num: 2

#### 3.16.1 Графа 31 — описание товаров

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g31.name | Сетки из стекловолокна различных типов и размеров. СМ.ДОПОЛНЕНИЕ | D | графа 31 — описание товара | обобщенное описание + СМ.ДОПОЛНЕНИЕ |
| 02 | goods[2].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | графа 31 — производитель | у всех строк группы один производитель |
| 03 | goods[2].g31.trade_mark | ОТСУТСТВУЕТ | D | графа 31 — товарный знак / ТМ | у всех строк группы ТМ отсутствует |
| 04 | goods[2].places | 27 | D | графа 31 — количество мест по товару | non_formalized.svh_1.goods[1].places (tnved=7019900095) |

- _audit: 4

#### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].item_no | 2 | D | графа 32 — номер товара | порядковый номер |
| 02 | goods[2].tnved_code | 7019900095 | D | графа 33 — код товара | GoodsCode группы |
| 03 | goods[2].tnved.flag_1 | С | D | графа 33 — доп. признак | заглушка С |
| 04 | goods[2].tnved.flag_2 | N | D | графа 33 — доп. признак | нет торговой марки |
| 05 | goods[2].origin_country_code | CN | D | графа 34 — код страны происхождения | OriginCountryCode=156 -> CN |
| 06 | goods[2].gross_weight | 1710.00 | D | графа 35 — вес брутто по товару | non_formalized.svh_1.goods[1].gross_weight_kg |
| 07 | goods[2].preference | ОООО-ОО | D | графа 36 — преференция | отсутствие преференций |
| 08 | goods[2].net_weight | 1614.60 | D | графа 38 — вес нетто по товару | сумма нетто по строкам группы: 491.40+1123.20 |

- _audit: 8

#### 3.16.3 Графы 42–46 — стоимости по товару

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].invoice_cost | 42228.00 | D | графа 42 — цена товара | сумма стоимости по строкам группы: 12852.00+29376.00 |
| 02 | goods[2].customs_value | | pending | графа 45 — таможенная стоимость | не хватает курса USD для пересчета transport_to_border в RUB |
| 03 | goods[2].transport_to_border | | pending | графа 45 — транспорт до границы | не хватает курса USD для пересчета в RUB (доля: (1710/3500)*1404 USD) |

- _audit: 3

#### 3.16.4 Массив: goods[2].txt[]
- _array_audit: 2

#### 3.16.5 Элемент массива: goods[2].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].txt[1].line_1 | АРТ: - 90 рулонов | D | графа 31 — TXT строка 1 | InvoiceGoods[5].GoodsQuantity + MeasureUnitQualifierName |
| 02 | goods[2].txt[1].line_2 | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | D | графа 31 — TXT строка 2 | InvoiceGoods[5].GoodsDescription |

- _item_audit: 2

#### 3.16.5 Элемент массива: goods[2].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].txt[2].line_1 | АРТ: - 180 рулонов | D | графа 31 — TXT строка 1 | InvoiceGoods[6].GoodsQuantity + MeasureUnitQualifierName |
| 02 | goods[2].txt[2].line_2 | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | D | графа 31 — TXT строка 2 | InvoiceGoods[6].GoodsDescription |

- _item_audit: 2

#### 3.16.6 Массив: goods[2].tovg[]
- _array_audit: 2

#### 3.16.7 Элемент массива: goods[2].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].tovg[1].line_no | 1 | D | графа 31 — № строки таблицы | порядковый номер |
| 02 | goods[2].tovg[1].description | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | D | графа 31 — наименование | InvoiceGoods[5].GoodsDescription |
| 03 | goods[2].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | графа 31 — производитель | InvoiceGoods[5].AdditionalGoodsDescription_Manufacturer |
| 04 | goods[2].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | графа 31 — марка/ТМ | InvoiceGoods[5].AdditionalGoodsDescription_TradeMark |
| 05 | goods[2].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | графа 31 — товарный знак | InvoiceGoods[5].AdditionalGoodsDescription_GoodsMark |
| 06 | goods[2].tovg[1].model | NOT APPLICABLE | D | графа 31 — модель/модификация | InvoiceGoods[5].AdditionalGoodsDescription_GoodsModel |
| 07 | goods[2].tovg[1].quantity | 3780 | CP | графа 31 — количество в доп.ед.изм | InvoiceGoods[5].goods_supplementary_quantity |
| 08 | goods[2].tovg[1].unit_code | 055 | D | графа 31 — код ЕИ | M2 -> 055 по cb:unit |
| 09 | goods[2].tovg[1].unit_name | M2 | CP | графа 31 — наименование ЕИ | InvoiceGoods[5].goods_supplementary_uom_name |
| 10 | goods[2].tovg[1].gross_weight | 520.00 | CP | графа 35 — вес брутто по строке | InvoiceGoods[5].GrossWeightQuantity |
| 11 | goods[2].tovg[1].net_weight | 491.40 | CP | графа 38 — вес нетто по строке | InvoiceGoods[5].NetWeightQuantity |
| 12 | goods[2].tovg[1].invoice_cost | 12852.00 | CP | графа 42 — цена по строке | InvoiceGoods[5].TotalCost |

- _item_audit: 12

#### 3.16.7 Элемент массива: goods[2].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].tovg[2].line_no | 2 | D | графа 31 — № строки таблицы | порядковый номер |
| 02 | goods[2].tovg[2].description | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | D | графа 31 — наименование | InvoiceGoods[6].GoodsDescription |
| 03 | goods[2].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | графа 31 — производитель | InvoiceGoods[6].AdditionalGoodsDescription_Manufacturer |
| 04 | goods[2].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | графа 31 — марка/ТМ | InvoiceGoods[6].AdditionalGoodsDescription_TradeMark |
| 05 | goods[2].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | графа 31 — товарный знак | InvoiceGoods[6].AdditionalGoodsDescription_GoodsMark |
| 06 | goods[2].tovg[2].model | NOT APPLICABLE | D | графа 31 — модель/модификация | InvoiceGoods[6].AdditionalGoodsDescription_GoodsModel |
| 07 | goods[2].tovg[2].quantity | 8640 | CP | графа 31 — количество в доп.ед.изм | InvoiceGoods[6].goods_supplementary_quantity |
| 08 | goods[2].tovg[2].unit_code | 055 | D | графа 31 — код ЕИ | M2 -> 055 по cb:unit |
| 09 | goods[2].tovg[2].unit_name | M2 | CP | графа 31 — наименование ЕИ | InvoiceGoods[6].goods_supplementary_uom_name |
| 10 | goods[2].tovg[2].gross_weight | 1190.00 | CP | графа 35 — вес брутто по строке | InvoiceGoods[6].GrossWeightQuantity |
| 11 | goods[2].tovg[2].net_weight | 1123.20 | CP | графа 38 — вес нетто по строке | InvoiceGoods[6].NetWeightQuantity |
| 12 | goods[2].tovg[2].invoice_cost | 29376.00 | CP | графа 42 — цена по строке | InvoiceGoods[6].TotalCost |

- _item_audit: 12

#### 3.16.9 Поле G_44 (текстовое поле в карточке товара)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44.text | СМ.ДОПОЛНЕНИЕ | D | графа 44 — текстовое поле | константа |

- _audit: 1

#### 3.16.10 Массив: goods[2].g44_docs[]
- _array_audit: 14

#### 3.16.11 Элемент массива: goods[2].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[1].doc_code | 03011 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[1].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[1].doc_name | КОНТРАКТ | CP | графа 44 — наименование документа | formalized.contract_1 |
| 04 | goods[2].g44_docs[1].doc_number | LM-2553 | CP | графа 44 — номер документа | formalized.contract_1 |
| 05 | goods[2].g44_docs[1].doc_date | 02.07.2025 | CP | графа 44 — дата документа | formalized.contract_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[2].doc_code | 03012 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[2].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | графа 44 — наименование документа | formalized.supplementary_contract_1 |
| 04 | goods[2].g44_docs[2].doc_number | 1 | CP | графа 44 — номер документа | formalized.supplementary_contract_1 |
| 05 | goods[2].g44_docs[2].doc_date | 25.11.2025 | CP | графа 44 — дата документа | formalized.supplementary_contract_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[3].doc_code | 04021 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[3].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[3].doc_name | ИНВОЙС | CP | графа 44 — наименование документа | formalized.invoice_1 |
| 04 | goods[2].g44_docs[3].doc_number | LM-2591 | CP | графа 44 — номер документа | formalized.invoice_1 |
| 05 | goods[2].g44_docs[3].doc_date | 30.10.2025 | CP | графа 44 — дата документа | formalized.invoice_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[4].doc_code | 04131 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[4].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[4].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | графа 44 — наименование документа | formalized.packing_list_1 |
| 04 | goods[2].g44_docs[4].doc_number | LM-2591 | CP | графа 44 — номер документа | formalized.packing_list_1 |
| 05 | goods[2].g44_docs[4].doc_date | 30.10.2025 | CP | графа 44 — дата документа | formalized.packing_list_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[5].doc_code | 02015 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[5].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[5].doc_name | CMR | CP | графа 44 — наименование документа | formalized.cmr_1 |
| 04 | goods[2].g44_docs[5].doc_number | 00378 | CP | графа 44 — номер документа | formalized.cmr_1 |
| 05 | goods[2].g44_docs[5].doc_date | 20.01.2026 | CP | графа 44 — дата документа | formalized.cmr_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[6].doc_code | 04023 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[6].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[6].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | графа 44 — наименование документа | formalized.payment_order_1 |
| 04 | goods[2].g44_docs[6].doc_number | 1 | CP | графа 44 — номер документа | formalized.payment_order_1 |
| 05 | goods[2].g44_docs[6].doc_date | 13.01.2026 | CP | графа 44 — дата документа | formalized.payment_order_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[7].doc_code | 04023 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[7].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[7].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | графа 44 — наименование документа | formalized.payment_order_2 |
| 04 | goods[2].g44_docs[7].doc_number | 7 | CP | графа 44 — номер документа | formalized.payment_order_2 |
| 05 | goods[2].g44_docs[7].doc_date | 28.11.2025 | CP | графа 44 — дата документа | formalized.payment_order_2 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[8].doc_code | 04031 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[8].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[8].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | графа 44 — наименование документа | formalized.service_invoice_1 |
| 04 | goods[2].g44_docs[8].doc_number | 26-00378-tl | CP | графа 44 — номер документа | formalized.service_invoice_1 |
| 05 | goods[2].g44_docs[8].doc_date | 27.01.2026 | CP | графа 44 — дата документа | formalized.service_invoice_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[9].doc_code | 04111 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[9].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[9].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | графа 44 — наименование документа | formalized.insurance_document_1 |
| 04 | goods[2].g44_docs[9].doc_number | 26-00378-tl/1 | CP | графа 44 — номер документа | formalized.insurance_document_1 |
| 05 | goods[2].g44_docs[9].doc_date | 14.01.2026 | CP | графа 44 — дата документа | formalized.insurance_document_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[10].doc_code | 05999 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[10].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[10].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | графа 44 — наименование документа | formalized.tech_description_1 |
| 04 | goods[2].g44_docs[10].doc_number | Б/Н | CP | графа 44 — номер документа | formalized.tech_description_1 |
| 05 | goods[2].g44_docs[10].doc_date | 30.10.2025 | CP | графа 44 — дата документа | formalized.tech_description_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[11].doc_code | 11001 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[11].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[11].doc_name | ПАСПОРТ | CP | графа 44 — наименование документа | formalized.passport_1 |
| 04 | goods[2].g44_docs[11].doc_number | 63 09 449948 | CP | графа 44 — номер документа | formalized.passport_1 |
| 05 | goods[2].g44_docs[11].doc_date | 11.03.2010 | CP | графа 44 — дата документа | formalized.passport_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[12].doc_code | 11004 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[12].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | графа 44 — наименование документа | formalized.letter_of_attorney_1 |
| 04 | goods[2].g44_docs[12].doc_number | 1 | CP | графа 44 — номер документа | formalized.letter_of_attorney_1 |
| 05 | goods[2].g44_docs[12].doc_date | 01.02.2026 | CP | графа 44 — дата документа | formalized.letter_of_attorney_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[13].doc_code | 04033 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[13].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[13].doc_name | ДОГОВОР ТРАНСПОРТНОЙ ЭКСПЕДИЦИИ | CP | графа 44 — наименование документа | formalized.transport_contract_1 |
| 04 | goods[2].g44_docs[13].doc_number | КООО/26651/М | CP | графа 44 — номер документа | formalized.transport_contract_1 |
| 05 | goods[2].g44_docs[13].doc_date | 13.05.2025 | CP | графа 44 — дата документа | formalized.transport_contract_1 |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[14].doc_code | 04011 | CP | графа 44 — код документа | cb:doc |
| 02 | goods[2].g44_docs[14].kind_code | 0 | CO | графа 44 — признак записи | заглушка 0 |
| 03 | goods[2].g44_docs[14].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | графа 44 — наименование документа | formalized.egrul_1 |
| 04 | goods[2].g44_docs[14].doc_number | ЮЭ9965-25-106893283 | CP | графа 44 — номер документа | formalized.egrul_1 |
| 05 | goods[2].g44_docs[14].doc_date | 12.03.2020 | CP | графа 44 — дата документа | formalized.egrul_1 |

- _item_audit: 5

### 3.17 Теги после товаров и документов (графы 51–54)

#### 3.17.1 Графа 54 — уполномоченное лицо / представитель

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | representative.date | 23.05.2026 | D | графа 54 — дата заполнения/подачи | текущая дата |
| 02 | representative.phone | +7 927-222-0500 | CD | графа 54 — телефон | master_data.representative.phone |
| 03 | representative.email | A.K.ARBUZOVA@YANDEX.RU | CD | графа 54 — e-mail | master_data.representative.email |
| 04 | representative.last_name | АРБУЗОВА | CP | графа 54 — фамилия | formalized.letter_of_attorney_1.EmpoweredPerson_PersonSurname |
| 05 | representative.first_name | АНАСТАСИЯ | CP | графа 54 — имя | formalized.letter_of_attorney_1.EmpoweredPerson_PersonName |
| 06 | representative.middle_name | КОНСТАНТИНОВНА | CP | графа 54 — отчество | formalized.letter_of_attorney_1.EmpoweredPerson_PersonMiddleName |
| 07 | representative.authority_doc_name | ДОВЕРЕННОСТЬ | CP | графа 54 — документ полномочий | formalized.letter_of_attorney_1.DocumentHead_DocumentName |
| 08 | representative.authority_doc_number | 1 | CP | графа 54 — № документа полномочий | formalized.letter_of_attorney_1.DocumentHead_DocumentNumber |
| 09 | representative.authority_doc_date_from | 01.02.2026 | CP | графа 54 — дата начала действия | formalized.letter_of_attorney_1.DocumentHead_DocumentDate |
| 10 | representative.authority_doc_date_to | 31.12.2026 | CP | графа 54 — дата окончания действия | formalized.letter_of_attorney_1.EndDate |
| 11 | representative.position | ПРЕДСТАВИТЕЛЬ | CP | графа 54 — должность/статус | formalized.letter_of_attorney_1.EmpoweredPerson_PersonPost |
| 12 | representative.passport_code | RU01001 | CP | графа 54 — код документа удостоверения личности | formalized.passport_1.CardSeries (константа) |
| 13 | representative.passport_name | ПАСПОРТ | CP | графа 54 — наименование документа | formalized.passport_1.DocumentHead_DocumentName (константа) |
| 14 | representative.passport_number | 449948 | CP | графа 54 — номер паспорта | formalized.passport_1.CardNumber |
| 15 | representative.passport_date | 11.03.2010 | CP | графа 54 — дата выдачи паспорта | formalized.passport_1.CardDate |
| 16 | representative.passport_series | 63 09 | CP | графа 54 — серия паспорта | formalized.passport_1.CardSeries |
| 17 | representative.passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CP | графа 54 — кем выдан | formalized.passport_1.OrganizationName |

- _audit: 17

### Итог:
- `dt_status`: pending

## Часть II: Issues (нерешенные вопросы)

### Для полей:
- `goods[2].customs_value`:
  - `question`: Не удается рассчитать таможенную стоимость товара 2 (7019900095). Требуется курс USD на дату подачи для пересчета transport_to_border (1404.00 USD * доля товара 2) в RUB. В primary.md есть только курс CNY (10.9430).

- `goods[2].transport_to_border`:
  - `question`: Не удается рассчитать долю транспортных расходов до границы для товара 2. Транспорт по счету 26-00378-tl = 1404.00 USD. Доля товара 2 = (1710/3500) * 1404 = 685.54 USD. Для пересчета в RUB требуется курс USD на дату подачи ДТ.

### Для общих вопросов:
- `[Общий]`:
  - `question`: В primary.md отсутствует курс USD. Для расчета таможенной стоимости товара 2 (7019900095) необходимо добавить курс USD на дату подачи ДТ (23.05.2026) в primary.md или предоставить его оператору.
