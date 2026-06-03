# Исходные данные для ДТ

## Метаданные:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `тип поставки`: 1 ДТ / 2 товара
- `агрегация ДТ`: группировка по коду ТН ВЭД
- `источники данных`: primary.md + operator_provided_data.md

## Часть I: Поля ДТ

### 3.1 Заголовок декларации (графа 1)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | declaration.direction | ИМ | CP | направление декларации | meta.direction |
| 02 | declaration.procedure | 40 | CO | код таможенной процедуры | operator:declaration.procedure |
| 03 | declaration.form | ЭД | D | форма подачи декларации | constant |

- _audit: 3

### 3.2 Отправитель (графа 2)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | sender.country_name | КИТАЙ | CP | страна отправителя, текст | formalized.invoice_1.Seler_PostalAddress_CounryName |
| 02 | sender.country_code | CN | CP | код страны отправителя alpha-2 | formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 03 | sender.name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CP | наименование отправителя | formalized.invoice_1.Seler_Name |
| 04 | sender.region | HEBEI | CP | регион отправителя | formalized.invoice_1.Seler_PostalAddress_Region |
| 05 | sender.city | SHIJIAZHUANG | CP | город отправителя | formalized.invoice_1.Seler_PostalAddress_City |
| 06 | sender.street | No. 5 Gaodong street | CP | улица/дом отправителя | formalized.invoice_1.Seler_PostalAddress_StreetHouse |

- _audit: 6

### 3.3 Количество товаров и мест (графы 5, 6)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.total_goods_number | 2 | D | количество товаров в ДТ | размер массива goods |
| 02 | shipment.packages_flag | true | D | признак подсчета мест | constant |
| 03 | shipment.total_packages | 127 | D | общее количество грузовых мест | non_formalized.svh.actual_places |

- _audit: 3

### 3.4 Получатель (графа 8)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | consignee.same_as_declarant | true | D | признак 'см. графу 14' | constant |
| 02 | consignee.ogrn | 1201600020390 | D | ОГРН получателя | копируется из графы 14 |
| 03 | consignee.inn_kpp | 1650389298/165001001 | D | ИНН/КПП получателя | копируется из графы 14 |
| 04 | consignee.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование получателя | копируется из графы 14 |
| 05 | consignee.country_code | RU | D | код страны получателя | копируется из графы 14 |
| 06 | consignee.country_name | РОССИЯ | D | страна получателя, текст | копируется из графы 14 |
| 07 | consignee.postcode | 423800 | D | почтовый индекс получателя | копируется из графы 14 |
| 08 | consignee.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион получателя | копируется из графы 14 |
| 09 | consignee.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | город получателя | копируется из графы 14 |
| 10 | consignee.street | ПРОЕЗД ХЛЕБНЫЙ | D | улица получателя | копируется из графы 14 |
| 11 | consignee.building | Д. 30 | D | дом получателя | копируется из графы 14 |
| 12 | consignee.room | ОФИС 211 | D | помещение/офис получателя | копируется из графы 14 |
| 13 | consignee.phone | +7 (843) 207 18 90 | D | телефон получателя | копируется из графы 14 |
| 14 | consignee.email | PROM_TAT@MAIL.RU | D | e-mail получателя | копируется из графы 14 |

- _audit: 14

### 3.5 Финансовое урегулирование (графа 9)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | financial.same_as_declarant | true | D | признак 'см. графу 14' | constant |
| 02 | financial.ogrn | 1201600020390 | D | ОГРН | копируется из графы 14 |
| 03 | financial.inn_kpp | 1650389298/165001001 | D | ИНН/КПП | копируется из графы 14 |
| 04 | financial.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование | копируется из графы 14 |
| 05 | financial.country_code | RU | D | код страны | копируется из графы 14 |
| 06 | financial.country_name | РОССИЯ | D | страна, текст | копируется из графы 14 |
| 07 | financial.postcode | 423800 | D | индекс | копируется из графы 14 |
| 08 | financial.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион | копируется из графы 14 |
| 09 | financial.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | город | копируется из графы 14 |
| 10 | financial.street | ПРОЕЗД ХЛЕБНЫЙ | D | улица | копируется из графы 14 |
| 11 | financial.building | Д. 30 | D | дом | копируется из графы 14 |
| 12 | financial.room | ОФИС 211 | D | помещение | копируется из графы 14 |
| 13 | financial.country_code_alt | RU | D | дублирующий код страны | копируется из declarant.country_code |
| 14 | financial.phone | +7 (843) 207 18 90 | D | телефон | копируется из графы 14 |
| 15 | financial.email | PROM_TAT@MAIL.RU | D | e-mail | копируется из графы 14 |

- _audit: 15

### 3.6 Торгующая страна (графа 11)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.trade_country_code | CN | CP | код торгующей страны | formalized.invoice_1.DeliveryTerms_TradingCountryCode |

- _audit: 1

### 3.7 Декларант (графа 14)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | declarant.ogrn | 1201600020390 | CP | ОГРН декларанта | master_data.egrul.OGRN |
| 02 | declarant.inn_kpp | 1650389298/165001001 | D | ИНН/КПП декларанта | master_data.egrul.INN + '/' + master_data.egrul.KPP |
| 03 | declarant.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CP | наименование декларанта | master_data.egrul.OrganizationName |
| 04 | declarant.country_code | RU | CP | код страны декларанта | master_data.egrul.Address_CountryCode |
| 05 | declarant.country_name | РОССИЯ | CP | страна декларанта, текст | master_data.egrul.Address_CounryName |
| 06 | declarant.postcode | 423800 | CP | почтовый индекс декларанта | master_data.egrul.Address_PostalCode |
| 07 | declarant.region | РЕСПУБЛИКА ТАТАРСТАН | CP | регион декларанта | master_data.egrul.Address_Region |
| 08 | declarant.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | город декларанта | master_data.egrul.Address_City |
| 09 | declarant.street | ПРОЕЗД ХЛЕБНЫЙ | D | улица декларанта | извлечено из master_data.egrul.Address_StreetHouse |
| 10 | declarant.building | Д. 30 | D | дом декларанта | извлечено из master_data.egrul.Address_StreetHouse |
| 11 | declarant.room | ОФИС 211 | D | помещение/офис декларанта | извлечено из master_data.egrul.Address_StreetHouse |
| 12 | declarant.phone | +7 (843) 207 18 90 | CP | телефон декларанта | master_data.egrul.Phone |
| 13 | declarant.email | PROM_TAT@MAIL.RU | CP | e-mail декларанта | master_data.egrul.Email |

- _audit: 13

### 3.8 Страны (графы 15, 16, 17)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.dispatch_country_code | CN | CP | код страны отправления | formalized.invoice_1.DeliveryTerms_DispatchCountryCode |
| 02 | shipment.destination_country_code | RU | CP | код страны назначения | formalized.invoice_1.DeliveryTerms_DestinationCountryCode |
| 03 | shipment.dispatch_country_name | КИТАЙ | D | страна отправления, текст | cb:country |
| 04 | shipment.destination_country_name | РОССИЯ | D | страна назначения, текст | cb:country |
| 05 | shipment.origin_country_code | CN | D | код страны происхождения | cb:country |
| 06 | shipment.origin_country_name | КИТАЙ | D | страна происхождения, текст | cb:country |

- _audit: 6

### 3.9 Условия поставки (графа 20)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | delivery.terms_code | EXW | D | условия поставки | formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode |
| 02 | delivery.place_name | HEBEI | D | место поставки | formalized.invoice_1.DeliveryTerms_DeliveryPlace |

- _audit: 2

### 3.10 Транспорт (графы 18, 19, 21)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.vehicles_count | 2 | D | количество ТС | размер массива TransportMeans |
| 02 | transport.identification | O157AO774/BT374974 | D | идентификация ТС | номера ТС через '/' |
| 03 | transport.registration_country_code | RU | D | код страны регистрации ТС | cb:country |
| 04 | transport.container_flag | 0 | CO | признак контейнера | constant |
| 05 | transport.border_mode | 1 | D | код активного ТС на границе | автоперевозка |

- _audit: 5

### 3.11 Валюта и стоимость (графа 22)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.invoice_currency_numeric | 156 | D | цифровой код валюты | cb:currency_okv |
| 02 | shipment.invoice_currency_alpha | CNY | CP | буквенный код валюты | formalized.invoice_1.CurrencyCode |
| 03 | shipment.invoice_amount | 97260.00 | CP | сумма по счету | formalized.invoice_1.TotalCost |

- _audit: 3

### 3.12 Вид транспорта (графы 25, 26)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.border_transport_code | 31 | D | код вида транспорта на границе | автодорожный состав |
| 02 | transport.internal_transport_code | 31 | D | код вида транспорта внутри страны | автодорожный состав |

- _audit: 2

### 3.13 Таможня на границе (графа 29)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs.border_code | 10719110 | CP | код таможенного органа на границе | non_formalized.td.customs_post_code |
| 02 | customs.border_name | ОТСУТСТВУЕТ | CP | наименование таможенного поста | non_formalized.td.customs_post_name |

- _audit: 2

### 3.14 Местонахождение товаров (графа 30)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | location.type | 11 | D | тип места нахождения товаров | constant |
| 02 | location.document_kind | 2 | D | вид документа СВХ | constant |
| 03 | location.document_number | 10404/141210/10092/5 | CP | номер документа СВХ | non_formalized.svh.warehouse_license_number |
| 04 | location.document_date | 18.09.2025 | CP | дата документа СВХ | non_formalized.svh.warehouse_license_date |
| 05 | location.address.country_code | RU | D | код страны местонахождения | constant |
| 06 | location.address.region | РЕСПУБЛИКА ТАТАРСТАН | CP | регион СВХ | non_formalized.svh_additional_sheet_1.svh_address_region |
| 07 | location.address.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | город СВХ | non_formalized.svh_additional_sheet_1.svh_address_city |
| 08 | location.address.street | Производственный пр-д, д. 45 | CP | улица/дом СВХ | non_formalized.svh_additional_sheet_1.svh_address_street_house |
| 09 | location.customs_code | 10404083 | CP | код таможенного органа СВХ | non_formalized.svh_additional_sheet_1.svh_customs_code |

- _audit: 9

### 3.15 Массив: goods[]
- goods._array_audit: 2

#### 3.15.0 Элемент массива: goods[1]
- goods._element_num: 1

#### 3.15.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g31.name | СЕТКА МОСКИТНАЯ ИЗ ПОЛИЭСТЕРА, ПЛЕТЕНАЯ, В РУЛОНАХ, ПРЕДНАЗНАЧЕНА ДЛЯ ЗАЩИТЫ ОТ НАСЕКОМЫХ, ПЫЛИ И ПЫЛЬЦЫ (МОДЕЛИ АНТИКОТ, АНТИПЫЛЬЦА, ТРЕХСЛОЙНАЯ АНТИПЫЛЬЦА). МЕТОД ИЗГОТОВЛЕНИЯ - ПЛЕТЕНИЕ. СМ.ДОПОЛНЕНИЕ | D | описание товара | non_formalized.goods_description_1.goods[1].description |
| 02 | goods[1].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | formalized.invoice_1.goods_all.manufacturer |
| 03 | goods[1].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | formalized.invoice_1.goods_all.trade_mark |
| 04 | goods[1].places | 100 | D | количество мест по товару | non_formalized.svh.goods_2.places |

- _audit: 4

#### 3.15.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].item_no | 1 | D | номер товара | порядковый номер товара в ДТ |
| 02 | goods[1].tnved_code | 5804101000 | D | код товара | ТН ВЭД |
| 03 | goods[1].tnved.flag_1 | С | D | доп. признак | constant |
| 04 | goods[1].tnved.flag_2 | N | D | доп. признак | нет торговой марки |
| 05 | goods[1].origin_country_code | CN | D | код страны происхождения | cb:country |
| 06 | goods[1].gross_weight | 1790 | D | вес брутто по товару | non_formalized.svh.goods_2.gross_weight_kg |
| 07 | goods[1].preference | ОООО-ОО | D | преференция | constant |
| 08 | goods[1].procedure_code | 4000000 | D | код процедуры по товару | declaration.procedure + '00' + '000' |
| 09 | goods[1].net_weight | 1687.40 | D | вес нетто по товару | сумма нетто по строкам группы |

- _audit: 9

#### 3.15.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].transport_cost | 718.05 | D | доля транспортных расходов | пропорционально весу брутто |
| 02 | goods[1].transport_currency | USD | CP | валюта транспортных расходов | formalized.service_invoice.transport_currency |
| 03 | goods[1].insurance_cost | 515.14 | D | доля расходов на страхование | пропорционально фактурной стоимости |
| 04 | goods[1].insurance_currency | RUB | CP | валюта страхования | formalized.insurance_invoice.insurance_currency |

- _audit: 4

#### 3.15.4 Массив: goods[1].txt[]
- _array_audit: 5

#### 3.15.5 Элемент массива: goods[1].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[1].line_1 | АРТ: - 2520 м2 | D | графа 31 — TXT строка 1 | АРТ: - кол-во ед |
| 02 | goods[1].txt[1].line_2 | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | D | графа 31 — TXT строка 2 | наименование |

- _item_audit: 2

#### 3.15.5 Элемент массива: goods[1].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[2].line_1 | АРТ: - 1440 м2 | D | графа 31 — TXT строка 1 | АРТ: - кол-во ед |
| 02 | goods[1].txt[2].line_2 | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | D | графа 31 — TXT строка 2 | наименование |

- _item_audit: 2

#### 3.15.5 Элемент массива: goods[1].txt[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[3].line_1 | АРТ: - 2520 м2 | D | графа 31 — TXT строка 1 | АРТ: - кол-во ед |
| 02 | goods[1].txt[3].line_2 | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы «Антипыльца» из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | D | графа 31 — TXT строка 2 | наименование |

- _item_audit: 2

#### 3.15.5 Элемент массива: goods[1].txt[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[4].line_1 | АРТ: - 1440 м2 | D | графа 31 — TXT строка 1 | АРТ: - кол-во ед |
| 02 | goods[1].txt[4].line_2 | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | D | графа 31 — TXT строка 2 | наименование |

- _item_audit: 2

#### 3.15.5 Элемент массива: goods[1].txt[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[5].line_1 | АРТ: - 240 м2 | D | графа 31 — TXT строка 1 | АРТ: - кол-во ед |
| 02 | goods[1].txt[5].line_2 | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 | D | графа 31 — TXT строка 2 | наименование |

- _item_audit: 2

#### 3.15.6 Массив: goods[1].tovg[]
- _array_audit: 5

#### 3.15.7 Элемент массива: goods[1].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[1].line_no | 1 | D | № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[1].description | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | D | наименование | copied_from:formalized.invoice_1.goods_1.GoodsDescription |
| 03 | goods[1].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | formalized.invoice_1.goods_all.manufacturer |
| 04 | goods[1].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | formalized.invoice_1.goods_all.trade_mark |
| 05 | goods[1].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | formalized.invoice_1.goods_all.goods_mark |
| 06 | goods[1].tovg[1].model | NOT APPLICABLE | D | модель/модификация | formalized.invoice_1.goods_all.model |
| 07 | goods[1].tovg[1].quantity | 2520 | D | количество в доп.ед.изм | formalized.invoice_1.goods_1.goods_supplementary_quantity |
| 08 | goods[1].tovg[1].unit_code | 055 | D | код ЕИ | cb:unit |
| 09 | goods[1].tovg[1].unit_name | м2 | D | наименование ЕИ | formalized.invoice_1.goods_1.goods_supplementary_uom_name |
| 10 | goods[1].tovg[1].gross_weight | 855.00 | D | вес брутто по строке | formalized.invoice_1.goods_1.gross_weight |
| 11 | goods[1].tovg[1].net_weight | 806.60 | D | вес нетто по строке | formalized.invoice_1.goods_1.net_weight |
| 12 | goods[1].tovg[1].invoice_cost | 14742.00 | D | цена по строке | formalized.invoice_1.goods_1.TotalCost |

- _item_audit: 12

#### 3.15.7 Элемент массива: goods[1].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[2].line_no | 2 | D | № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[2].description | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | D | наименование | copied_from:formalized.invoice_1.goods_2.GoodsDescription |
| 03 | goods[1].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | formalized.invoice_1.goods_all.manufacturer |
| 04 | goods[1].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | formalized.invoice_1.goods_all.trade_mark |
| 05 | goods[1].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | formalized.invoice_1.goods_all.goods_mark |
| 06 | goods[1].tovg[2].model | NOT APPLICABLE | D | модель/модификация | formalized.invoice_1.goods_all.model |
| 07 | goods[1].tovg[2].quantity | 1440 | D | количество в доп.ед.изм | formalized.invoice_1.goods_2.goods_supplementary_quantity |
| 08 | goods[1].tovg[2].unit_code | 055 | D | код ЕИ | cb:unit |
| 09 | goods[1].tovg[2].unit_name | м2 | D | наименование ЕИ | formalized.invoice_1.goods_2.goods_supplementary_uom_name |
| 10 | goods[1].tovg[2].gross_weight | 490.00 | D | вес брутто по строке | formalized.invoice_1.goods_2.gross_weight |
| 11 | goods[1].tovg[2].net_weight | 460.80 | D | вес нетто по строке | formalized.invoice_1.goods_2.net_weight |
| 12 | goods[1].tovg[2].invoice_cost | 8424.00 | D | цена по строке | formalized.invoice_1.goods_2.TotalCost |

- _item_audit: 12

#### 3.15.7 Элемент массива: goods[1].tovg[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[3].line_no | 3 | D | № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[3].description | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы «Антипыльца» из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | D | наименование | copied_from:formalized.invoice_1.goods_3.GoodsDescription |
| 03 | goods[1].tovg[3].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | formalized.invoice_1.goods_all.manufacturer |
| 04 | goods[1].tovg[3].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | formalized.invoice_1.goods_all.trade_mark |
| 05 | goods[1].tovg[3].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | formalized.invoice_1.goods_all.goods_mark |
| 06 | goods[1].tovg[3].model | NOT APPLICABLE | D | модель/модификация | formalized.invoice_1.goods_all.model |
| 07 | goods[1].tovg[3].quantity | 2520 | D | количество в доп.ед.изм | formalized.invoice_1.goods_3.goods_supplementary_quantity |
| 08 | goods[1].tovg[3].unit_code | 055 | D | код ЕИ | cb:unit |
| 09 | goods[1].tovg[3].unit_name | м2 | D | наименование ЕИ | formalized.invoice_1.goods_3.goods_supplementary_uom_name |
| 10 | goods[1].tovg[3].gross_weight | 265.00 | D | вес брутто по строке | formalized.invoice_1.goods_3.gross_weight |
| 11 | goods[1].tovg[3].net_weight | 252.00 | D | вес нетто по строке | formalized.invoice_1.goods_3.net_weight |
| 12 | goods[1].tovg[3].invoice_cost | 16002.00 | D | цена по строке | formalized.invoice_1.goods_3.TotalCost |

- _item_audit: 12

#### 3.15.7 Элемент массива: goods[1].tovg[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[4].line_no | 4 | D | № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[4].description | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | D | наименование | copied_from:formalized.invoice_1.goods_4.GoodsDescription |
| 03 | goods[1].tovg[4].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | formalized.invoice_1.goods_all.manufacturer |
| 04 | goods[1].tovg[4].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | formalized.invoice_1.goods_all.trade_mark |
| 05 | goods[1].tovg[4].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | formalized.invoice_1.goods_all.goods_mark |
| 06 | goods[1].tovg[4].model | NOT APPLICABLE | D | модель/модификация | formalized.invoice_1.goods_all.model |
| 07 | goods[1].tovg[4].quantity | 1440 | D | количество в доп.ед.изм | formalized.invoice_1.goods_4.goods_supplementary_quantity |
| 08 | goods[1].tovg[4].unit_code | 055 | D | код ЕИ | cb:unit |
| 09 | goods[1].tovg[4].unit_name | м2 | D | наименование ЕИ | formalized.invoice_1.goods_4.goods_supplementary_uom_name |
| 10 | goods[1].tovg[4].gross_weight | 155.00 | D | вес брутто по строке | formalized.invoice_1.goods_4.gross_weight |
| 11 | goods[1].tovg[4].net_weight | 144.00 | D | вес нетто по строке | formalized.invoice_1.goods_4.net_weight |
| 12 | goods[1].tovg[4].invoice_cost | 9144.00 | D | цена по строке | formalized.invoice_1.goods_4.TotalCost |

- _item_audit: 12

#### 3.15.7 Элемент массива: goods[1].tovg[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[5].line_no | 5 | D | № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[5].description | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 | D | наименование | copied_from:formalized.invoice_1.goods_7.GoodsDescription |
| 03 | goods[1].tovg[5].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | formalized.invoice_1.goods_all.manufacturer |
| 04 | goods[1].tovg[5].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | formalized.invoice_1.goods_all.trade_mark |
| 05 | goods[1].tovg[5].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | formalized.invoice_1.goods_all.goods_mark |
| 06 | goods[1].tovg[5].model | NOT APPLICABLE | D | модель/модификация | formalized.invoice_1.goods_all.model |
| 07 | goods[1].tovg[5].quantity | 240 | D | количество в доп.ед.изм | formalized.invoice_1.goods_7.goods_supplementary_quantity |
| 08 | goods[1].tovg[5].unit_code | 055 | D | код ЕИ | cb:unit |
| 09 | goods[1].tovg[5].unit_name | м2 | D | наименование ЕИ | formalized.invoice_1.goods_7.goods_supplementary_uom_name |
| 10 | goods[1].tovg[5].gross_weight | 25.00 | D | вес брутто по строке | formalized.invoice_1.goods_7.gross_weight |
| 11 | goods[1].tovg[5].net_weight | 24.00 | D | вес нетто по строке | formalized.invoice_1.goods_7.net_weight |
| 12 | goods[1].tovg[5].invoice_cost | 6720.00 | D | цена по строке | formalized.invoice_1.goods_7.TotalCost |

- _item_audit: 12

#### 3.15.9 Поле G_44 (текстовое поле в карточке товара)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

#### 3.15.10 Массив: goods[1].g44_docs[]
- _array_audit: 17

#### 3.15.11 Элемент массива: goods[1].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[1].doc_code | 04021 | CP | код документа | formalized.invoice_1.doc_code |
| 02 | goods[1].g44_docs[1].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[1].doc_name | ИНВОЙС | CP | наименование документа | formalized.invoice_1.doc_name |
| 04 | goods[1].g44_docs[1].doc_number | LM-2591 | CP | номер документа | formalized.invoice_1.doc_number |
| 05 | goods[1].g44_docs[1].doc_date | 2025-10-30 | CP | дата документа | formalized.invoice_1.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[2].doc_code | 04131 | CP | код документа | formalized.packing_list.doc_code |
| 02 | goods[1].g44_docs[2].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[2].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | formalized.packing_list.doc_name |
| 04 | goods[1].g44_docs[2].doc_number | LM-2591 | CP | номер документа | formalized.packing_list.doc_number |
| 05 | goods[1].g44_docs[2].doc_date | 2025-10-30 | CP | дата документа | formalized.packing_list.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[3].doc_code | 02015 | CP | код документа | formalized.cmr.doc_code |
| 02 | goods[1].g44_docs[3].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[3].doc_name | CMR | CP | наименование документа | formalized.cmr.doc_name |
| 04 | goods[1].g44_docs[3].doc_number | 00378 | CP | номер документа | formalized.cmr.doc_number |
| 05 | goods[1].g44_docs[3].doc_date | 2026-01-20 | CP | дата документа | formalized.cmr.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[4].doc_code | 04023 | CP | код документа | formalized.payment_order_1.doc_code |
| 02 | goods[1].g44_docs[4].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[4].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | formalized.payment_order_1.doc_name |
| 04 | goods[1].g44_docs[4].doc_number | 1 | CP | номер документа | formalized.payment_order_1.doc_number |
| 05 | goods[1].g44_docs[4].doc_date | 2026-01-13 | CP | дата документа | formalized.payment_order_1.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[5].doc_code | 04023 | CP | код документа | formalized.payment_order_2.doc_code |
| 02 | goods[1].g44_docs[5].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[5].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | formalized.payment_order_2.doc_name |
| 04 | goods[1].g44_docs[5].doc_number | 7 | CP | номер документа | formalized.payment_order_2.doc_number |
| 05 | goods[1].g44_docs[5].doc_date | 2025-11-28 | CP | дата документа | formalized.payment_order_2.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[6].doc_code | 04031 | CP | код документа | formalized.service_invoice.doc_code |
| 02 | goods[1].g44_docs[6].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[6].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | formalized.service_invoice.doc_name |
| 04 | goods[1].g44_docs[6].doc_number | 26-00378-tl | CP | номер документа | formalized.service_invoice.doc_number |
| 05 | goods[1].g44_docs[6].doc_date | 2026-01-27 | CP | дата документа | formalized.service_invoice.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[7].doc_code | 04111 | CP | код документа | formalized.insurance_invoice.doc_code |
| 02 | goods[1].g44_docs[7].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[7].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | formalized.insurance_invoice.doc_name |
| 04 | goods[1].g44_docs[7].doc_number | 26-00378-tl/1 | CP | номер документа | formalized.insurance_invoice.doc_number |
| 05 | goods[1].g44_docs[7].doc_date | 2026-01-14 | CP | дата документа | formalized.insurance_invoice.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[8].doc_code | 05999 | CP | код документа | formalized.tech_description.doc_code |
| 02 | goods[1].g44_docs[8].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[8].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | formalized.tech_description.doc_name |
| 04 | goods[1].g44_docs[8].doc_number | Б/Н | CP | номер документа | formalized.tech_description.doc_number |
| 05 | goods[1].g44_docs[8].doc_date | 2025-10-30 | CP | дата документа | formalized.tech_description.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[9].doc_code | 03011 | CP | код документа | master_data.contract.doc_code |
| 02 | goods[1].g44_docs[9].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[9].doc_name | КОНТРАКТ | CP | наименование документа | master_data.contract.doc_name |
| 04 | goods[1].g44_docs[9].doc_number | LM-2553 | CP | номер документа | master_data.contract.doc_number |
| 05 | goods[1].g44_docs[9].doc_date | 2025-07-02 | CP | дата документа | master_data.contract.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[10].doc_code | 03012 | CP | код документа | master_data.supplementary_contract_1.doc_code |
| 02 | goods[1].g44_docs[10].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[10].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | master_data.supplementary_contract_1.doc_name |
| 04 | goods[1].g44_docs[10].doc_number | 1 | CP | номер документа | master_data.supplementary_contract_1.doc_number |
| 05 | goods[1].g44_docs[10].doc_date | 2025-11-25 | CP | дата документа | master_data.supplementary_contract_1.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[11].doc_code | 04011 | CP | код документа | master_data.egrul.doc_code |
| 02 | goods[1].g44_docs[11].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[11].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | master_data.egrul.doc_name |
| 04 | goods[1].g44_docs[11].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | master_data.egrul.doc_number |
| 05 | goods[1].g44_docs[11].doc_date | 2025-07-14 | CP | дата документа | master_data.egrul.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[12].doc_code | 11001 | CP | код документа | master_data.passport.doc_code |
| 02 | goods[1].g44_docs[12].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[12].doc_name | ПАСПОРТ | CP | наименование документа | master_data.passport.doc_name |
| 04 | goods[1].g44_docs[12].doc_number | 63 09 449948 | CP | номер документа | master_data.passport.doc_number |
| 05 | goods[1].g44_docs[12].doc_date | 2010-03-11 | CP | дата документа | master_data.passport.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[13].doc_code | 11004 | CP | код документа | master_data.letter_of_attorney.doc_code |
| 02 | goods[1].g44_docs[13].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[13].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | master_data.letter_of_attorney.doc_name |
| 04 | goods[1].g44_docs[13].doc_number | 1 | CP | номер документа | master_data.letter_of_attorney.doc_number |
| 05 | goods[1].g44_docs[13].doc_date | 2026-02-01 | CP | дата документа | master_data.letter_of_attorney.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[14].doc_code | 04033 | CP | код документа | master_data.transport_contract.doc_code |
| 02 | goods[1].g44_docs[14].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[14].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | master_data.transport_contract.doc_name |
| 04 | goods[1].g44_docs[14].doc_number | КООО/26651/М | CP | номер документа | master_data.transport_contract.doc_number |
| 05 | goods[1].g44_docs[14].doc_date | 2025-05-13 | CP | дата документа | master_data.transport_contract.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[15].doc_code | 09023 | CP | код документа | master_data.exemption_letter.doc_code |
| 02 | goods[1].g44_docs[15].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter.doc_name |
| 04 | goods[1].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter.doc_number |
| 05 | goods[1].g44_docs[15].doc_date | 2025-08-20 | CP | дата документа | master_data.exemption_letter.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[16].doc_code | 09999 | CP | код документа | master_data.exemption_letter_source.doc_code |
| 02 | goods[1].g44_docs[16].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[16].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_source.doc_name |
| 04 | goods[1].g44_docs[16].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_source.doc_number |
| 05 | goods[1].g44_docs[16].doc_date | 2025-08-20 | CP | дата документа | master_data.exemption_letter_source.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[1].g44_docs[17]
- _element_num: 17

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[17].doc_code | 09013 | CP | код документа | non_formalized.td.doc_code |
| 02 | goods[1].g44_docs[17].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[1].g44_docs[17].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | non_formalized.td.doc_name |
| 04 | goods[1].g44_docs[17].doc_number | 10719110/240126/5011363 | CP | номер документа | non_formalized.td.doc_number |
| 05 | goods[1].g44_docs[17].doc_date | 2026-01-24 | CP | дата документа | non_formalized.td.doc_date |

- _item_audit: 5

#### 3.15.0 Элемент массива: goods[2]
- goods._element_num: 2

#### 3.15.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g31.name | СЕТКА МОСКИТНАЯ ИЗ СТЕКЛОВОЛОКНА, ПЛЕТЕНАЯ, В РУЛОНАХ, ПРЕДНАЗНАЧЕНА ДЛЯ ЗАЩИТЫ ОТ МЕЛЬЧАЙШИХ НАСЕКОМЫХ И МОШЕК (МОДЕЛЬ АНТИМОШКА). МЕТОД ИЗГОТОВЛЕНИЯ - ПЛЕТЕНИЕ. СМ.ДОПОЛНЕНИЕ | D | описание товара | non_formalized.goods_description_1.goods[2].description |
| 02 | goods[2].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | formalized.invoice_1.goods_all.manufacturer |
| 03 | goods[2].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | formalized.invoice_1.goods_all.trade_mark |
| 04 | goods[2].places | 27 | D | количество мест по товару | non_formalized.svh.goods_1.places |

- _audit: 4

#### 3.15.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].item_no | 2 | D | номер товара | порядковый номер товара в ДТ |
| 02 | goods[2].tnved_code | 7019900095 | D | код товара | ТН ВЭД |
| 03 | goods[2].tnved.flag_1 | С | D | доп. признак | constant |
| 04 | goods[2].tnved.flag_2 | N | D | доп. признак | нет торговой марки |
| 05 | goods[2].origin_country_code | CN | D | код страны происхождения | cb:country |
| 06 | goods[2].gross_weight | 1710 | D | вес брутто по товару | non_formalized.svh.goods_1.gross_weight_kg |
| 07 | goods[2].preference | ОООО-ОО | D | преференция | constant |
| 08 | goods[2].procedure_code | 4000000 | D | код процедуры по товару | declaration.procedure + '00' + '000' |
| 09 | goods[2].net_weight | 1614.60 | D | вес нетто по товару | сумма нетто по строкам группы |

- _audit: 9

#### 3.15.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].transport_cost | 685.95 | D | доля транспортных расходов | пропорционально весу брутто |
| 02 | goods[2].transport_currency | USD | CP | валюта транспортных расходов | formalized.service_invoice.transport_currency |
| 03 | goods[2].insurance_cost | 395.20 | D | доля расходов на страхование | пропорционально фактурной стоимости |
| 04 | goods[2].insurance_currency | RUB | CP | валюта страхования | formalized.insurance_invoice.insurance_currency |

- _audit: 4

#### 3.15.4 Массив: goods[2].txt[]
- _array_audit: 2

#### 3.15.5 Элемент массива: goods[2].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[1].line_1 | АРТ: - 3780 м2 | D | графа 31 — TXT строка 1 | АРТ: - кол-во ед |
| 02 | goods[2].txt[1].line_2 | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,4*30 M2 | D | графа 31 — TXT строка 2 | наименование |

- _item_audit: 2

#### 3.15.5 Элемент массива: goods[2].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[2].line_1 | АРТ: - 8640 м2 | D | графа 31 — TXT строка 1 | АРТ: - кол-во ед |
| 02 | goods[2].txt[2].line_2 | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,6*30 M2 | D | графа 31 — TXT строка 2 | наименование |

- _item_audit: 2

#### 3.15.6 Массив: goods[2].tovg[]
- _array_audit: 2

#### 3.15.7 Элемент массива: goods[2].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[1].line_no | 1 | D | № строки таблицы | порядковый номер |
| 02 | goods[2].tovg[1].description | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,4*30 M2 | D | наименование | copied_from:formalized.invoice_1.goods_5.GoodsDescription |
| 03 | goods[2].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | formalized.invoice_1.goods_all.manufacturer |
| 04 | goods[2].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | formalized.invoice_1.goods_all.trade_mark |
| 05 | goods[2].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | formalized.invoice_1.goods_all.goods_mark |
| 06 | goods[2].tovg[1].model | NOT APPLICABLE | D | модель/модификация | formalized.invoice_1.goods_all.model |
| 07 | goods[2].tovg[1].quantity | 3780 | D | количество в доп.ед.изм | formalized.invoice_1.goods_5.goods_supplementary_quantity |
| 08 | goods[2].tovg[1].unit_code | 055 | D | код ЕИ | cb:unit |
| 09 | goods[2].tovg[1].unit_name | м2 | D | наименование ЕИ | formalized.invoice_1.goods_5.goods_supplementary_uom_name |
| 10 | goods[2].tovg[1].gross_weight | 520.00 | D | вес брутто по строке | formalized.invoice_1.goods_5.gross_weight |
| 11 | goods[2].tovg[1].net_weight | 491.40 | D | вес нетто по строке | formalized.invoice_1.goods_5.net_weight |
| 12 | goods[2].tovg[1].invoice_cost | 12852.00 | D | цена по строке | formalized.invoice_1.goods_5.TotalCost |

- _item_audit: 12

#### 3.15.7 Элемент массива: goods[2].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[2].line_no | 2 | D | № строки таблицы | порядковый номер |
| 02 | goods[2].tovg[2].description | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,6*30 M2 | D | наименование | copied_from:formalized.invoice_1.goods_6.GoodsDescription |
| 03 | goods[2].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | formalized.invoice_1.goods_all.manufacturer |
| 04 | goods[2].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | formalized.invoice_1.goods_all.trade_mark |
| 05 | goods[2].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | formalized.invoice_1.goods_all.goods_mark |
| 06 | goods[2].tovg[2].model | NOT APPLICABLE | D | модель/модификация | formalized.invoice_1.goods_all.model |
| 07 | goods[2].tovg[2].quantity | 8640 | D | количество в доп.ед.изм | formalized.invoice_1.goods_6.goods_supplementary_quantity |
| 08 | goods[2].tovg[2].unit_code | 055 | D | код ЕИ | cb:unit |
| 09 | goods[2].tovg[2].unit_name | м2 | D | наименование ЕИ | formalized.invoice_1.goods_6.goods_supplementary_uom_name |
| 10 | goods[2].tovg[2].gross_weight | 1190.00 | D | вес брутто по строке | formalized.invoice_1.goods_6.gross_weight |
| 11 | goods[2].tovg[2].net_weight | 1123.20 | D | вес нетто по строке | formalized.invoice_1.goods_6.net_weight |
| 12 | goods[2].tovg[2].invoice_cost | 29376.00 | D | цена по строке | formalized.invoice_1.goods_6.TotalCost |

- _item_audit: 12

#### 3.15.9 Поле G_44 (текстовое поле в карточке товара)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

#### 3.15.10 Массив: goods[2].g44_docs[]
- _array_audit: 17

#### 3.15.11 Элемент массива: goods[2].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[1].doc_code | 04021 | CP | код документа | formalized.invoice_1.doc_code |
| 02 | goods[2].g44_docs[1].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[1].doc_name | ИНВОЙС | CP | наименование документа | formalized.invoice_1.doc_name |
| 04 | goods[2].g44_docs[1].doc_number | LM-2591 | CP | номер документа | formalized.invoice_1.doc_number |
| 05 | goods[2].g44_docs[1].doc_date | 2025-10-30 | CP | дата документа | formalized.invoice_1.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[2].doc_code | 04131 | CP | код документа | formalized.packing_list.doc_code |
| 02 | goods[2].g44_docs[2].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[2].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | formalized.packing_list.doc_name |
| 04 | goods[2].g44_docs[2].doc_number | LM-2591 | CP | номер документа | formalized.packing_list.doc_number |
| 05 | goods[2].g44_docs[2].doc_date | 2025-10-30 | CP | дата документа | formalized.packing_list.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[3].doc_code | 02015 | CP | код документа | formalized.cmr.doc_code |
| 02 | goods[2].g44_docs[3].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[3].doc_name | CMR | CP | наименование документа | formalized.cmr.doc_name |
| 04 | goods[2].g44_docs[3].doc_number | 00378 | CP | номер документа | formalized.cmr.doc_number |
| 05 | goods[2].g44_docs[3].doc_date | 2026-01-20 | CP | дата документа | formalized.cmr.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[4].doc_code | 04023 | CP | код документа | formalized.payment_order_1.doc_code |
| 02 | goods[2].g44_docs[4].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[4].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | formalized.payment_order_1.doc_name |
| 04 | goods[2].g44_docs[4].doc_number | 1 | CP | номер документа | formalized.payment_order_1.doc_number |
| 05 | goods[2].g44_docs[4].doc_date | 2026-01-13 | CP | дата документа | formalized.payment_order_1.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[5].doc_code | 04023 | CP | код документа | formalized.payment_order_2.doc_code |
| 02 | goods[2].g44_docs[5].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[5].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | formalized.payment_order_2.doc_name |
| 04 | goods[2].g44_docs[5].doc_number | 7 | CP | номер документа | formalized.payment_order_2.doc_number |
| 05 | goods[2].g44_docs[5].doc_date | 2025-11-28 | CP | дата документа | formalized.payment_order_2.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[6].doc_code | 04031 | CP | код документа | formalized.service_invoice.doc_code |
| 02 | goods[2].g44_docs[6].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[6].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | formalized.service_invoice.doc_name |
| 04 | goods[2].g44_docs[6].doc_number | 26-00378-tl | CP | номер документа | formalized.service_invoice.doc_number |
| 05 | goods[2].g44_docs[6].doc_date | 2026-01-27 | CP | дата документа | formalized.service_invoice.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[7].doc_code | 04111 | CP | код документа | formalized.insurance_invoice.doc_code |
| 02 | goods[2].g44_docs[7].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[7].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | formalized.insurance_invoice.doc_name |
| 04 | goods[2].g44_docs[7].doc_number | 26-00378-tl/1 | CP | номер документа | formalized.insurance_invoice.doc_number |
| 05 | goods[2].g44_docs[7].doc_date | 2026-01-14 | CP | дата документа | formalized.insurance_invoice.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[8].doc_code | 05999 | CP | код документа | formalized.tech_description.doc_code |
| 02 | goods[2].g44_docs[8].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[8].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | formalized.tech_description.doc_name |
| 04 | goods[2].g44_docs[8].doc_number | Б/Н | CP | номер документа | formalized.tech_description.doc_number |
| 05 | goods[2].g44_docs[8].doc_date | 2025-10-30 | CP | дата документа | formalized.tech_description.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[9].doc_code | 03011 | CP | код документа | master_data.contract.doc_code |
| 02 | goods[2].g44_docs[9].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[9].doc_name | КОНТРАКТ | CP | наименование документа | master_data.contract.doc_name |
| 04 | goods[2].g44_docs[9].doc_number | LM-2553 | CP | номер документа | master_data.contract.doc_number |
| 05 | goods[2].g44_docs[9].doc_date | 2025-07-02 | CP | дата документа | master_data.contract.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[10].doc_code | 03012 | CP | код документа | master_data.supplementary_contract_1.doc_code |
| 02 | goods[2].g44_docs[10].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[10].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | master_data.supplementary_contract_1.doc_name |
| 04 | goods[2].g44_docs[10].doc_number | 1 | CP | номер документа | master_data.supplementary_contract_1.doc_number |
| 05 | goods[2].g44_docs[10].doc_date | 2025-11-25 | CP | дата документа | master_data.supplementary_contract_1.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[11].doc_code | 04011 | CP | код документа | master_data.egrul.doc_code |
| 02 | goods[2].g44_docs[11].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[11].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | master_data.egrul.doc_name |
| 04 | goods[2].g44_docs[11].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | master_data.egrul.doc_number |
| 05 | goods[2].g44_docs[11].doc_date | 2025-07-14 | CP | дата документа | master_data.egrul.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[12].doc_code | 11001 | CP | код документа | master_data.passport.doc_code |
| 02 | goods[2].g44_docs[12].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[12].doc_name | ПАСПОРТ | CP | наименование документа | master_data.passport.doc_name |
| 04 | goods[2].g44_docs[12].doc_number | 63 09 449948 | CP | номер документа | master_data.passport.doc_number |
| 05 | goods[2].g44_docs[12].doc_date | 2010-03-11 | CP | дата документа | master_data.passport.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[13].doc_code | 11004 | CP | код документа | master_data.letter_of_attorney.doc_code |
| 02 | goods[2].g44_docs[13].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[13].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | master_data.letter_of_attorney.doc_name |
| 04 | goods[2].g44_docs[13].doc_number | 1 | CP | номер документа | master_data.letter_of_attorney.doc_number |
| 05 | goods[2].g44_docs[13].doc_date | 2026-02-01 | CP | дата документа | master_data.letter_of_attorney.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[14].doc_code | 04033 | CP | код документа | master_data.transport_contract.doc_code |
| 02 | goods[2].g44_docs[14].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[14].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | master_data.transport_contract.doc_name |
| 04 | goods[2].g44_docs[14].doc_number | КООО/26651/М | CP | номер документа | master_data.transport_contract.doc_number |
| 05 | goods[2].g44_docs[14].doc_date | 2025-05-13 | CP | дата документа | master_data.transport_contract.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[15].doc_code | 09023 | CP | код документа | master_data.exemption_letter.doc_code |
| 02 | goods[2].g44_docs[15].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter.doc_name |
| 04 | goods[2].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter.doc_number |
| 05 | goods[2].g44_docs[15].doc_date | 2025-08-20 | CP | дата документа | master_data.exemption_letter.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[16].doc_code | 09999 | CP | код документа | master_data.exemption_letter_source.doc_code |
| 02 | goods[2].g44_docs[16].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[16].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_source.doc_name |
| 04 | goods[2].g44_docs[16].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_source.doc_number |
| 05 | goods[2].g44_docs[16].doc_date | 2025-08-20 | CP | дата документа | master_data.exemption_letter_source.doc_date |

- _item_audit: 5

#### 3.15.11 Элемент массива: goods[2].g44_docs[17]
- _element_num: 17

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[17].doc_code | 09013 | CP | код документа | non_formalized.td.doc_code |
| 02 | goods[2].g44_docs[17].kind_code | 0 | CO | признак записи | constant |
| 03 | goods[2].g44_docs[17].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | non_formalized.td.doc_name |
| 04 | goods[2].g44_docs[17].doc_number | 10719110/240126/5011363 | CP | номер документа | non_formalized.td.doc_number |
| 05 | goods[2].g44_docs[17].doc_date | 2026-01-24 | CP | дата документа | non_formalized.td.doc_date |

- _item_audit: 5

### 3.16 Теги после товаров и документов (графы 51–54)

### 3.16.1 Графа 54 — уполномоченное лицо / представитель

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | representative.date | 2026-06-03 | D | дата заполнения/подачи | текущая дата |
| 02 | representative.phone | +7 927-222-0500 | CP | телефон | master_data.passport.Phone |
| 03 | representative.email | A.K.ARBUZOVA@YANDEX.RU | CP | e-mail | master_data.passport.Email |
| 04 | representative.last_name | АРБУЗОВА | CP | фамилия | master_data.passport.PersonSurname |
| 05 | representative.first_name | АНАСТАСИЯ | CP | имя | master_data.passport.PersonName |
| 06 | representative.middle_name | КОНСТАНТИНОВНА | CP | отчество | master_data.passport.PersonMiddleName |
| 07 | representative.authority_doc_name | ДОВЕРЕННОСТЬ | CP | документ полномочий | master_data.letter_of_attorney.doc_name |
| 08 | representative.authority_doc_number | 1 | CP | № документа полномочий | master_data.letter_of_attorney.DocumentNumber |
| 09 | representative.authority_doc_date_from | 2026-02-01 | CP | дата начала действия | master_data.letter_of_attorney.DocumentDate |
| 10 | representative.authority_doc_date_to | 2026-12-31 | CP | дата окончания действия | master_data.letter_of_attorney.EndDate |
| 11 | representative.position | УПОЛНОМОЧЕННОЕ ЛИЦО | CP | должность/статус | master_data.letter_of_attorney.EmpoweredPost |
| 12 | representative.passport_code | RU01001 | CP | код документа личности | constant |
| 13 | representative.passport_name | ПАСПРФ | CP | наименование документа | constant |
| 14 | representative.passport_number | 449948 | CP | номер паспорта | master_data.passport.CardNumber |
| 15 | representative.passport_date | 2010-03-11 | CP | дата выдачи паспорта | master_data.passport.CardDate |
| 16 | representative.passport_series | 63 09 | CP | серия паспорта | master_data.passport.CardSeries |
| 17 | representative.passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CP | кем выдан | master_data.passport.OrganizationName |

- _audit: 17

### Итог:
- `dt_status`: confirmed

### Часть II: Issues (нерешенные вопросы)

*Нет нерешенных вопросов. Все поля успешно заполнены подтвержденными значениями.*
