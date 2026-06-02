# Исходные данные для ДТ

## Метаданные:
- `название кейса`: ЗапчастиТермометров
- `путь к папке поставки`: alta\source\ЗапчастиТермометров
- `тип поставки`: 1 ДТ / 5 товаров
- `агрегация ДТ`: группировка по ТН ВЭД
- `источники данных:` primary.md + operator_provided_data

## Часть I: Поля ДТ

### 3.1 Заголовок декларации (графа 1)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | declaration.direction | ИМ | CD | направление декларации | meta.direction |
| 02 | declaration.procedure | 40 | CO | код таможенной процедуры | cb:procedure |
| 03 | declaration.form | ЭД | D | форма подачи декларации | константа |

- _audit: 3

### 3.2 Отправитель (графа 2)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | sender.country_name | КИТАЙ | CP | наименование страны | copied_from:formalized.invoice_1.Seler_PostalAddress_CounryName |
| 02 | sender.country_code | CN | CP | код страны alpha-2 | copied_from:formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 03 | sender.name | BESTWILL SUPPLY CHAIN (NINGBO) CO.,LTD | CP | наименование отправителя | copied_from:formalized.invoice_1.Seler_Name |
| 04 | sender.region | Yinzhou district | CP | область/район | copied_from:formalized.invoice_1.Seler_PostalAddress_Region |
| 05 | sender.city | Ningbo | CP | город | copied_from:formalized.invoice_1.Seler_PostalAddress_City |
| 06 | sender.street | TIANTONG SOUTH ROAD | CP | улица и дом | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse |

- _audit: 6

### 3.3 Количество товаров и мест (графы 5, 6)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.total_goods_number | 5 | D | количество товарных позиций | размер массива goods |
| 02 | shipment.packages_flag | true | D | признак подсчёта мест | константа |
| 03 | shipment.total_packages | 8 | D | общее количество грузовых мест | по приоритету: packing_list.PlacesQuantity |

- _audit: 3

### 3.4 Получатель (графа 8)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | consignee.same_as_declarant | true | D | признак «см. графу 14» | константа |
| 02 | consignee.ogrn | 1201600020390 | D | ОГРН получателя | см. графу 14 |
| 03 | consignee.inn_kpp | 1650389298/165001001 | D | ИНН/КПП получателя | см. графу 14 |
| 04 | consignee.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование организации | см. графу 14 |
| 05 | consignee.country_code | RU | D | код страны alpha-2 | см. графу 14 |
| 06 | consignee.country_name | РОССИЯ | D | страна, наименование | см. графу 14 |
| 07 | consignee.postcode | 423800 | D | почтовый индекс | см. графу 14 |
| 08 | consignee.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион | см. графу 14 |
| 09 | consignee.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | населённый пункт | см. графу 14 |
| 10 | consignee.street | ПРОЕЗД ХЛЕБНЫЙ | D | улица | см. графу 14 |
| 11 | consignee.building | Д. 30 | D | дом | см. графу 14 |
| 12 | consignee.room | ОФИС 211 | D | помещение/офис | см. графу 14 |
| 13 | consignee.phone | +7 (843) 207 18 90 | D | телефон | см. графу 14 |
| 14 | consignee.email | PROM_TAT@MAIL.RU | D | e-mail | см. графу 14 |

- _audit: 14

### 3.5 Финансовое урегулирование (графа 9)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | financial.same_as_declarant | true | D | признак «см. графу 14» | константа |
| 02 | financial.ogrn | 1201600020390 | D | ОГРН | см. графу 14 |
| 03 | financial.inn_kpp | 1650389298/165001001 | D | ИНН/КПП | см. графу 14 |
| 04 | financial.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование | см. графу 14 |
| 05 | financial.country_code | RU | D | код страны | см. графу 14 |
| 06 | financial.country_name | РОССИЯ | D | наименование страны | см. графу 14 |
| 07 | financial.postcode | 423800 | D | индекс | см. графу 14 |
| 08 | financial.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион | см. графу 14 |
| 09 | financial.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | город | см. графу 14 |
| 10 | financial.street | ПРОЕЗД ХЛЕБНЫЙ | D | улица | см. графу 14 |
| 11 | financial.building | Д. 30 | D | дом | см. графу 14 |
| 12 | financial.room | ОФИС 211 | D | помещение | см. графу 14 |
| 13 | financial.country_code_alt | RU | D | дублирующий код страны | declarant.country_code |
| 14 | financial.phone | +7 (843) 207 18 90 | D | телефон | см. графу 14 |
| 15 | financial.email | PROM_TAT@MAIL.RU | D | e-mail | см. графу 14 |

- _audit: 15

### 3.6 Торгующая страна (графа 11)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.trade_country_code | CN | CP | код торгующей страны | copied_from:formalized.invoice_1.DeliveryTerms_TradingCountryCode |

- _audit: 1

### 3.7 Декларант (графа 14)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | declarant.ogrn | 1201600020390 | CP | ОГРН декларанта | master_data.egrul_1.OGRN |
| 02 | declarant.inn_kpp | 1650389298/165001001 | D | ИНН/КПП декларанта | master_data.egrul_1.INN + "/" + master_data.egrul_1.KPP |
| 03 | declarant.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CP | наименование организации | master_data.egrul_1.OrganizationName |
| 04 | declarant.country_code | RU | CP | код страны | master_data.egrul_1.Address_CountryCode |
| 05 | declarant.country_name | РОССИЯ | CP | наименование страны | master_data.egrul_1.Address_CounryName |
| 06 | declarant.postcode | 423800 | CP | почтовый индекс | master_data.egrul_1.Address_PostalCode |
| 07 | declarant.region | РЕСПУБЛИКА ТАТАРСТАН | CP | регион | master_data.egrul_1.Address_Region |
| 08 | declarant.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | населённый пункт | master_data.egrul_1.Address_City |
| 09 | declarant.street | ПРОЕЗД ХЛЕБНЫЙ | CP | улица | извлечено из master_data.egrul_1.Address_StreetHouse |
| 10 | declarant.building | Д. 30 | CP | дом | извлечено из master_data.egrul_1.Address_StreetHouse |
| 11 | declarant.room | ОФИС 211 | D | помещение/офис | извлечено из master_data.egrul_1.Address_StreetHouse |
| 12 | declarant.phone | +7 (843) 207 18 90 | CP | телефон | master_data.egrul_1.Phone |
| 13 | declarant.email | PROM_TAT@MAIL.RU | CP | e-mail | master_data.egrul_1.Email |

- _audit: 13

### 3.8 Страны (графы 15, 16, 17)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.dispatch_country_code | CN | CP | код страны отправления | copied_from:formalized.invoice_1.DeliveryTerms_DispatchCountryCode |
| 02 | shipment.destination_country_code | RU | CP | код страны назначения | copied_from:formalized.invoice_1.DeliveryTerms_DestinationCountryCode |
| 03 | shipment.dispatch_country_name | КИТАЙ | D | страна отправления, текст | cb:country |
| 04 | shipment.destination_country_name | РОССИЯ | D | страна назначения, текст | cb:country |
| 05 | shipment.origin_country_code | CN | D | код страны происхождения | у всех товаров один OriginCountryCode |
| 06 | shipment.origin_country_name | КИТАЙ | D | страна происхождения, текст | cb:country |

- _audit: 6

### 3.9 Условия поставки (графа 20)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | delivery.terms_code | EXW | D | условия поставки | по приоритету: formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode |
| 02 | delivery.place_name | Ningbo | D | место поставки | по приоритету: formalized.invoice_1.DeliveryTerms_DeliveryPlace |

- _audit: 2

### 3.10 Транспорт (графы 18, 19, 21)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | transport.vehicles_count | 2 | D | количество транспортных средств | число блоков TransportMeans в упаковочном |
| 02 | transport.identification | M862EY67/AM710667 | D | идентификация ТС | join номеров тягача и прицепа |
| 03 | transport.registration_country_code | RU | D | код страны регистрации ТС | по умолчанию RU для номеров РФ |
| 04 | transport.container_flag | 0 | CO | признак контейнера | константа |
| 05 | transport.border_mode | 1 | D | код активного ТС на границе | автоперевозка |

- _audit: 5

### 3.11 Валюта и стоимость (графа 22)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.invoice_currency_numeric | 156 | D | цифровой код валюты | CNY -> 156 по cb:currency_okv |
| 02 | shipment.invoice_currency_alpha | CNY | CP | буквенный код валюты | formalized.invoice_1.CurrencyCode |
| 03 | shipment.invoice_amount | 49500.00 | CP | сумма по счёту | formalized.invoice_1.TotalCost |

- _audit: 3

### 3.12 Курс валюты (графа 23)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | shipment.currency_rate | | CP | курс валюты к рублю | оставить пустым по operator_provided_data.md |

- _audit: 1

### 3.13 Вид транспорта (графы 25, 26)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | transport.border_transport_code | 31 | D | код вида транспорта на границе | автосостав |
| 02 | transport.internal_transport_code | 31 | D | код вида транспорта внутри страны | автосостав |

- _audit: 2

### 3.14 Таможня на границе (графа 29)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | customs.border_code | | CP | код таможенного органа | отсутствует в поставке |
| 02 | customs.border_name | | CP | наименование таможенного поста | отсутствует в поставке |

- _audit: 2

### 3.15 Местонахождение товаров (графа 30)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | location.type | 11 | D | тип места нахождения товаров | СВХ |
| 02 | location.document_kind | 2 | D | вид документа | лицензия |
| 03 | location.document_number | | CP | номер документа СВХ | отсутствует в поставке |
| 04 | location.document_date | | CP | дата документа СВХ | отсутствует в поставке |
| 05 | location.address.country_code | RU | D | код страны местонахождения | константа |
| 06 | location.address.region | | CP | регион | отсутствует в поставке |
| 07 | location.address.city | | CP | город | отсутствует в поставке |
| 08 | location.address.street | | CP | улица и дом | отсутствует в поставке |
| 09 | location.customs_code | | CP | код таможенного органа СВХ | отсутствует в поставке |

- _audit: 09

### 3.16 Массив: goods[]
- goods._array_audit: 5

#### 3.16.0 Элемент массива: goods[1]
- goods._element_num: 1

#### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g31.name | ЖИДКОКРИСТАЛЛИЧЕСКИЙ ЭКРАН (LCD SCREEN) DL0151-RL0 42X31ММ, СМ.ДОПОЛНЕНИЕ | D | описание товара | по данным tovg |
| 02 | goods[1].g31.manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | по данным tovg |
| 03 | goods[1].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | по данным tovg |
| 04 | goods[1].places | 2 | D | количество мест по товару | по данным tovg |

- _audit: 4

#### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].item_no | 1 | D | номер товара | порядковый номер |
| 02 | goods[1].tnved_code | 8524910056 | D | код товара | ТН ВЭД |
| 03 | goods[1].tnved.flag_1 | С | D | доп. признак | константа |
| 04 | goods[1].tnved.flag_2 | N | D | доп. признак | константа |
| 05 | goods[1].origin_country_code | CN | D | код страны происхождения | cb:country |
| 06 | goods[1].gross_weight | 27.00 | D | вес брутто по товару | по данным tovg |
| 07 | goods[1].preference | ОООО-ОО | D | преференция | константа |
| 08 | goods[1].net_weight | 26.45 | D | вес нетто по товару | по данным tovg |

- _audit: 8

#### 3.16.3 Графы 42–46 — стоимости по товару

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].invoice_cost | 13200.00 | D | цена товара | по данным tovg |
| 02 | goods[1].customs_value | | D | таможенная стоимость | курс пустой |
| 03 | goods[1].transport_to_border | | D | транспорт до границы | курс пустой |

- _audit: 3

#### 3.16.4 Массив: goods[1].txt[]
- _array_audit: 1

#### 3.16.5 Элемент массива: goods[1].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].txt[1].line_1 | АРТ: - 5000 шт | D | TXT строка 1 | по данным tovg |
| 02 | goods[1].txt[1].line_2 | LCD screen DL0151-RL0 42x31mm Экран LCD DL0151-RL0 42x31мм | D | TXT строка 2 | по данным tovg |

- _item_audit: 2

#### 3.16.6 Массив: goods[1].tovg[]
- _array_audit: 1

#### 3.16.7 Элемент массива: goods[1].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].tovg[1].line_no | 1 | D | № строки таблицы | порядковый номер |
| 02 | goods[1].tovg[1].description | LCD screen DL0151-RL0 42x31mm Экран LCD DL0151-RL0 42x31мм | D | наименование | copied_from:invoice.InvoiceGoods[1].GoodsDescription (где dt_item_index == 1 и dt_tovg_index == 1) |
| 03 | goods[1].tovg[1].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | copied_from:invoice.InvoiceGoods[1].AdditionalGoodsDescription_Manufacturer (где dt_item_index == 1 и dt_tovg_index == 1) |
| 04 | goods[1].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | copied_from:invoice.InvoiceGoods[1].AdditionalGoodsDescription_TradeMark (где dt_item_index == 1 и dt_tovg_index == 1) |
| 05 | goods[1].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | copied_from:invoice.InvoiceGoods[1].AdditionalGoodsDescription_GoodsMark (где dt_item_index == 1 и dt_tovg_index == 1) |
| 06 | goods[1].tovg[1].model | DL0151-RL0 | D | модель/модификация | copied_from:invoice.InvoiceGoods[1].AdditionalGoodsDescription_GoodsModel (где dt_item_index == 1 и dt_tovg_index == 1) |
| 07 | goods[1].tovg[1].quantity | 5000 | CP | количество в доп.ед.изм | copied_from:invoice.InvoiceGoods[1].goods_supplementary_quantity (где dt_item_index == 1 и dt_tovg_index == 1) |
| 08 | goods[1].tovg[1].unit_code | 796 | D | код ЕИ | cb:unit |
| 09 | goods[1].tovg[1].unit_name | шт | CP | наименование ЕИ | copied_from:invoice.InvoiceGoods[1].goods_supplementary_uom_name (где dt_item_index == 1 и dt_tovg_index == 1) |
| 10 | goods[1].tovg[1].gross_weight | 27.00 | CP | вес брутто по строке | copied_from:invoice.InvoiceGoods[1].GrossWeightQuantity (где dt_item_index == 1 и dt_tovg_index == 1) |
| 11 | goods[1].tovg[1].net_weight | 26.45 | CP | вес нетто по строке | copied_from:invoice.InvoiceGoods[1].NetWeightQuantity (где dt_item_index == 1 и dt_tovg_index == 1) |
| 12 | goods[1].tovg[1].invoice_cost | 13200.00 | CP | цена по строке | copied_from:invoice.InvoiceGoods[1].TotalCost (где dt_item_index == 1 и dt_tovg_index == 1) |

- _item_audit: 12

#### 3.16.9 Поле G_44 — текстовое поле

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | константа |

- _audit: 1

#### 3.16.10 Массив: goods[1].g44_docs[]
- _array_audit: 15

#### 3.16.11 Элемент массива: goods[1].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[1].doc_code | 04021 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[1].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[1].doc_name | ИНВОЙС | CP | наименование документа | formalized.invoice_1.doc_name |
| 04 | goods[1].g44_docs[1].doc_number | HNKY260226 | CP | номер документа | formalized.invoice_1.doc_number |
| 05 | goods[1].g44_docs[1].doc_date | 26.02.2026 | CP | дата документа | formalized.invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[2].doc_code | 04131 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[2].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[2].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | formalized.packing_list_1.doc_name |
| 04 | goods[1].g44_docs[2].doc_number | HNKY260226 | CP | номер документа | formalized.packing_list_1.doc_number |
| 05 | goods[1].g44_docs[2].doc_date | 26.02.2026 | CP | дата документа | formalized.packing_list_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[3].doc_code | 02015 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[3].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[3].doc_name | CMR | CP | наименование документа | formalized.cmr_1.doc_name |
| 04 | goods[1].g44_docs[3].doc_number | 09886 | CP | номер документа | formalized.cmr_1.doc_number |
| 05 | goods[1].g44_docs[3].doc_date | 13.05.2026 | CP | дата документа | formalized.cmr_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[4].doc_code | 04023 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[4].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[4].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | formalized.payment_order_1.doc_name |
| 04 | goods[1].g44_docs[4].doc_number | 2 | CP | номер документа | formalized.payment_order_1.doc_number |
| 05 | goods[1].g44_docs[4].doc_date | 27.02.2026 | CP | дата документа | formalized.payment_order_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[5].doc_code | 04031 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[5].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[5].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | formalized.service_invoice_1.doc_name |
| 04 | goods[1].g44_docs[5].doc_number | 26-09886-tl | CP | номер документа | formalized.service_invoice_1.doc_number |
| 05 | goods[1].g44_docs[5].doc_date | 12.05.2026 | CP | дата документа | formalized.service_invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[6].doc_code | 04111 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[6].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[6].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | formalized.insurance_document_1.doc_name |
| 04 | goods[1].g44_docs[6].doc_number | 26-09886-tl/1 | CP | номер документа | formalized.insurance_document_1.doc_number |
| 05 | goods[1].g44_docs[6].doc_date | 05.05.2026 | CP | дата документа | formalized.insurance_document_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[7].doc_code | 05999 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[7].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[7].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | formalized.tech_description_1.doc_name |
| 04 | goods[1].g44_docs[7].doc_number | БН | CP | номер документа | formalized.tech_description_1.doc_number |
| 05 | goods[1].g44_docs[7].doc_date | 26.02.2026 | CP | дата документа | formalized.tech_description_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[8].doc_code | 03011 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[8].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[8].doc_name | КОНТРАКТ | CP | наименование документа | formalized.contract.doc_name |
| 04 | goods[1].g44_docs[8].doc_number | HNKY250929 | CP | номер документа | formalized.contract.doc_number |
| 05 | goods[1].g44_docs[8].doc_date | 29.09.2025 | CP | дата документа | formalized.contract.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[9].doc_code | 03012 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[9].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[9].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | formalized.supplementary_contract_1.doc_name |
| 04 | goods[1].g44_docs[9].doc_number | 1 | CP | номер документа | formalized.supplementary_contract_1.doc_number |
| 05 | goods[1].g44_docs[9].doc_date | 26.02.2026 | CP | дата документа | formalized.supplementary_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[10].doc_code | 04011 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[10].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | master_data.egrul_1.doc_name |
| 04 | goods[1].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | master_data.egrul_1.doc_number |
| 05 | goods[1].g44_docs[10].doc_date | 14.07.2025 | CP | дата документа | master_data.egrul_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[11].doc_code | 11001 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[11].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[11].doc_name | ПАСПОРТ | CP | наименование документа | master_data.passport_1.doc_name |
| 04 | goods[1].g44_docs[11].doc_number | 63 09 449948 | CP | номер документа | master_data.passport_1.doc_number |
| 05 | goods[1].g44_docs[11].doc_date | 11.03.2010 | CP | дата документа | master_data.passport_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[12].doc_code | 11004 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[12].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | master_data.letter_of_attorney_1.doc_name |
| 04 | goods[1].g44_docs[12].doc_number | 1 | CP | номер документа | master_data.letter_of_attorney_1.doc_number |
| 05 | goods[1].g44_docs[12].doc_date | 01.02.2026 | CP | дата документа | master_data.letter_of_attorney_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[13].doc_code | 04033 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[13].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[13].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | master_data.transport_contract_1.doc_name |
| 04 | goods[1].g44_docs[13].doc_number | КООО/26651/М | CP | номер документа | master_data.transport_contract_1.doc_number |
| 05 | goods[1].g44_docs[13].doc_date | 13.05.2025 | CP | дата документа | master_data.transport_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[14].doc_code | 09023 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[14].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[14].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_1.doc_name |
| 04 | goods[1].g44_docs[14].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_1.doc_number |
| 05 | goods[1].g44_docs[14].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[1].g44_docs[15].doc_code | 09999 | CP | код документа | cb:doc |
| 02 | goods[1].g44_docs[15].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[1].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_source_1.doc_name |
| 04 | goods[1].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_source_1.doc_number |
| 05 | goods[1].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_source_1.doc_date |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[2]
- goods._element_num: 2

#### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g31.name | ПЛАТА С ДАТЧИКОМ, БАТАРЕЕЙ, ПРОВОДОМ И ЩУПОМ, СМ.ДОПОЛНЕНИЕ | D | описание товара | по данным tovg |
| 02 | goods[2].g31.manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | по данным tovg |
| 03 | goods[2].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | по данным tovg |
| 04 | goods[2].places | 5 | D | количество мест по товару | по данным tovg |

- _audit: 4

#### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].item_no | 2 | D | номер товара | порядковый номер |
| 02 | goods[2].tnved_code | 9025900008 | D | код товара | ТН ВЭД |
| 03 | goods[2].tnved.flag_1 | С | D | доп. признак | константа |
| 04 | goods[2].tnved.flag_2 | N | D | доп. признак | константа |
| 05 | goods[2].origin_country_code | CN | D | код страны происхождения | cb:country |
| 06 | goods[2].gross_weight | 38.80 | D | вес брутто по товару | по данным tovg |
| 07 | goods[2].preference | ОООО-ОО | D | преференция | константа |
| 08 | goods[2].net_weight | 38.30 | D | вес нетто по товару | по данным tovg |

- _audit: 8

#### 3.16.3 Графы 42–46 — стоимости по товару

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].invoice_cost | 30000.00 | D | цена товара | по данным tovg |
| 02 | goods[2].customs_value | | D | таможенная стоимость | курс пустой |
| 03 | goods[2].transport_to_border | | D | транспорт до границы | курс пустой |

- _audit: 3

#### 3.16.4 Массив: goods[2].txt[]
- _array_audit: 1

#### 3.16.5 Элемент массива: goods[2].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].txt[1].line_1 | АРТ: - | D | TXT строка 1 | по данным tovg |
| 02 | goods[2].txt[1].line_2 | Board with a secsor and wire 300 mm length, batary and probe 35 mm*40 mm / Плата с датчиком 35 мм*40 мм, батареей и проводом длиной 300 мм и щупом Plata s datchikom i provodom dlinoy | D | TXT строка 2 | по данным tovg |

- _item_audit: 2

#### 3.16.6 Массив: goods[2].tovg[]
- _array_audit: 1

#### 3.16.7 Элемент массива: goods[2].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].tovg[1].line_no | 1 | D | № строки таблицы | порядковый номер |
| 02 | goods[2].tovg[1].description | Board with a secsor and wire 300 mm length, batary and probe 35 mm*40 mm / Плата с датчиком 35 мм*40 мм, батареей и проводом длиной 300 мм и щупом Plata s datchikom i provodom dlinoy | D | наименование | copied_from:invoice.InvoiceGoods[2].GoodsDescription (где dt_item_index == 2 и dt_tovg_index == 1) |
| 03 | goods[2].tovg[1].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | copied_from:invoice.InvoiceGoods[2].AdditionalGoodsDescription_Manufacturer (где dt_item_index == 2 и dt_tovg_index == 1) |
| 04 | goods[2].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | copied_from:invoice.InvoiceGoods[2].AdditionalGoodsDescription_TradeMark (где dt_item_index == 2 и dt_tovg_index == 1) |
| 05 | goods[2].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | copied_from:invoice.InvoiceGoods[2].AdditionalGoodsDescription_GoodsMark (где dt_item_index == 2 и dt_tovg_index == 1) |
| 06 | goods[2].tovg[1].model | | CO | модель/модификация | operator:confirmed (оставить пустым) |
| 07 | goods[2].tovg[1].quantity | | CO | количество в доп.ед.изм | operator:confirmed (not_applicable) |
| 08 | goods[2].tovg[1].unit_code | | CO | код ЕИ | operator:confirmed (not_applicable) |
| 09 | goods[2].tovg[1].unit_name | | CO | наименование ЕИ | operator:confirmed (not_applicable) |
| 10 | goods[2].tovg[1].gross_weight | 38.80 | CP | вес брутто по строке | copied_from:invoice.InvoiceGoods[2].GrossWeightQuantity (где dt_item_index == 2 и dt_tovg_index == 1) |
| 11 | goods[2].tovg[1].net_weight | 38.30 | CP | вес нетто по строке | copied_from:invoice.InvoiceGoods[2].NetWeightQuantity (где dt_item_index == 2 и dt_tovg_index == 1) |
| 12 | goods[2].tovg[1].invoice_cost | 30000.00 | CP | цена по строке | copied_from:invoice.InvoiceGoods[2].TotalCost (где dt_item_index == 2 и dt_tovg_index == 1) |

- _item_audit: 12

#### 3.16.9 Поле G_44 — текстовое поле

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | константа |

- _audit: 1

#### 3.16.10 Массив: goods[2].g44_docs[]
- _array_audit: 15

#### 3.16.11 Элемент массива: goods[2].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[1].doc_code | 04021 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[1].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[1].doc_name | ИНВОЙС | CP | наименование документа | formalized.invoice_1.doc_name |
| 04 | goods[2].g44_docs[1].doc_number | HNKY260226 | CP | номер документа | formalized.invoice_1.doc_number |
| 05 | goods[2].g44_docs[1].doc_date | 26.02.2026 | CP | дата документа | formalized.invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[2].doc_code | 04131 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[2].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[2].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | formalized.packing_list_1.doc_name |
| 04 | goods[2].g44_docs[2].doc_number | HNKY260226 | CP | номер документа | formalized.packing_list_1.doc_number |
| 05 | goods[2].g44_docs[2].doc_date | 26.02.2026 | CP | дата документа | formalized.packing_list_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[3].doc_code | 02015 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[3].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[3].doc_name | CMR | CP | наименование документа | formalized.cmr_1.doc_name |
| 04 | goods[2].g44_docs[3].doc_number | 09886 | CP | номер документа | formalized.cmr_1.doc_number |
| 05 | goods[2].g44_docs[3].doc_date | 13.05.2026 | CP | дата документа | formalized.cmr_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[4].doc_code | 04023 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[4].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[4].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | formalized.payment_order_1.doc_name |
| 04 | goods[2].g44_docs[4].doc_number | 2 | CP | номер документа | formalized.payment_order_1.doc_number |
| 05 | goods[2].g44_docs[4].doc_date | 27.02.2026 | CP | дата документа | formalized.payment_order_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[5].doc_code | 04031 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[5].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[5].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | formalized.service_invoice_1.doc_name |
| 04 | goods[2].g44_docs[5].doc_number | 26-09886-tl | CP | номер документа | formalized.service_invoice_1.doc_number |
| 05 | goods[2].g44_docs[5].doc_date | 12.05.2026 | CP | дата документа | formalized.service_invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[6].doc_code | 04111 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[6].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[6].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | formalized.insurance_document_1.doc_name |
| 04 | goods[2].g44_docs[6].doc_number | 26-09886-tl/1 | CP | номер документа | formalized.insurance_document_1.doc_number |
| 05 | goods[2].g44_docs[6].doc_date | 05.05.2026 | CP | дата документа | formalized.insurance_document_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[7].doc_code | 05999 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[7].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[7].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | formalized.tech_description_1.doc_name |
| 04 | goods[2].g44_docs[7].doc_number | БН | CP | номер документа | formalized.tech_description_1.doc_number |
| 05 | goods[2].g44_docs[7].doc_date | 26.02.2026 | CP | дата документа | formalized.tech_description_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[8].doc_code | 03011 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[8].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[8].doc_name | КОНТРАКТ | CP | наименование документа | formalized.contract.doc_name |
| 04 | goods[2].g44_docs[8].doc_number | HNKY250929 | CP | номер документа | formalized.contract.doc_number |
| 05 | goods[2].g44_docs[8].doc_date | 29.09.2025 | CP | дата документа | formalized.contract.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[9].doc_code | 03012 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[9].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[9].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | formalized.supplementary_contract_1.doc_name |
| 04 | goods[2].g44_docs[9].doc_number | 1 | CP | номер документа | formalized.supplementary_contract_1.doc_number |
| 05 | goods[2].g44_docs[9].doc_date | 26.02.2026 | CP | дата документа | formalized.supplementary_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[10].doc_code | 04011 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[10].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | master_data.egrul_1.doc_name |
| 04 | goods[2].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | master_data.egrul_1.doc_number |
| 05 | goods[2].g44_docs[10].doc_date | 14.07.2025 | CP | дата документа | master_data.egrul_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[11].doc_code | 11001 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[11].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[11].doc_name | ПАСПОРТ | CP | наименование документа | master_data.passport_1.doc_name |
| 04 | goods[2].g44_docs[11].doc_number | 63 09 449948 | CP | номер документа | master_data.passport_1.doc_number |
| 05 | goods[2].g44_docs[11].doc_date | 11.03.2010 | CP | дата документа | master_data.passport_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[12].doc_code | 11004 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[12].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | master_data.letter_of_attorney_1.doc_name |
| 04 | goods[2].g44_docs[12].doc_number | 1 | CP | номер документа | master_data.letter_of_attorney_1.doc_number |
| 05 | goods[2].g44_docs[12].doc_date | 01.02.2026 | CP | дата документа | master_data.letter_of_attorney_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[13].doc_code | 04033 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[13].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[13].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | master_data.transport_contract_1.doc_name |
| 04 | goods[2].g44_docs[13].doc_number | КООО/26651/М | CP | номер документа | master_data.transport_contract_1.doc_number |
| 05 | goods[2].g44_docs[13].doc_date | 13.05.2025 | CP | дата документа | master_data.transport_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[14].doc_code | 09023 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[14].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[14].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_1.doc_name |
| 04 | goods[2].g44_docs[14].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_1.doc_number |
| 05 | goods[2].g44_docs[14].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[2].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[2].g44_docs[15].doc_code | 09999 | CP | код документа | cb:doc |
| 02 | goods[2].g44_docs[15].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[2].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_source_1.doc_name |
| 04 | goods[2].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_source_1.doc_number |
| 05 | goods[2].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_source_1.doc_date |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[3]
- goods._element_num: 3

#### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g31.name | ЗОНД С ДВУСТОРОННИМ СКОТЧЕМ ИЗ НЕРЖАВЕЮЩЕЙ СТАЛИ 31*13ММ, СМ.ДОПОЛНЕНИЕ | D | описание товара | по данным tovg |
| 02 | goods[3].g31.manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | по данным tovg |
| 03 | goods[3].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | по данным tovg |
| 04 | goods[3].places | | CO | количество мест по товару | operator:confirmed (оставить пустым) |

- _audit: 4

#### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].item_no | 3 | D | номер товара | порядковый номер |
| 02 | goods[3].tnved_code | 7326909409 | D | код товара | ТН ВЭД |
| 03 | goods[3].tnved.flag_1 | С | D | доп. признак | константа |
| 04 | goods[3].tnved.flag_2 | N | D | доп. признак | константа |
| 05 | goods[3].origin_country_code | CN | D | код страны происхождения | cb:country |
| 06 | goods[3].gross_weight | 1.60 | D | вес брутто по товару | по данным tovg |
| 07 | goods[3].preference | ОООО-ОО | D | преференция | константа |
| 08 | goods[3].net_weight | 1.50 | D | вес нетто по товару | по данным tovg |

- _audit: 8

#### 3.16.3 Графы 42–46 — стоимости по товару

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].invoice_cost | 3000.00 | D | цена товара | по данным tovg |
| 02 | goods[3].customs_value | | D | таможенная стоимость | курс пустой |
| 03 | goods[3].transport_to_border | | D | транспорт до границы | курс пустой |

- _audit: 3

#### 3.16.4 Массив: goods[3].txt[]
- _array_audit: 1

#### 3.16.5 Элемент массива: goods[3].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].txt[1].line_1 | АРТ: - | D | TXT строка 1 | по данным tovg |
| 02 | goods[3].txt[1].line_2 | Probe with double-sided tape made of stainless steel 31*13 mm/Зонд с двусторонним скотчем из нержавеющей стали 31*13мм | D | TXT строка 2 | по данным tovg |

- _item_audit: 2

#### 3.16.6 Массив: goods[3].tovg[]
- _array_audit: 1

#### 3.16.7 Элемент массива: goods[3].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].tovg[1].line_no | 1 | D | № строки таблицы | порядковый номер |
| 02 | goods[3].tovg[1].description | Probe with double-sided tape made of stainless steel 31*13 mm/Зонд с двусторонним скотчем из нержавеющей стали 31*13мм | D | наименование | copied_from:invoice.InvoiceGoods[3].GoodsDescription (где dt_item_index == 3 и dt_tovg_index == 1) |
| 03 | goods[3].tovg[1].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | copied_from:invoice.InvoiceGoods[3].AdditionalGoodsDescription_Manufacturer (где dt_item_index == 3 и dt_tovg_index == 1) |
| 04 | goods[3].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | copied_from:invoice.InvoiceGoods[3].AdditionalGoodsDescription_TradeMark (где dt_item_index == 3 и dt_tovg_index == 1) |
| 05 | goods[3].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | copied_from:invoice.InvoiceGoods[3].AdditionalGoodsDescription_GoodsMark (где dt_item_index == 3 и dt_tovg_index == 1) |
| 06 | goods[3].tovg[1].model | | CO | модель/модификация | operator:confirmed (оставить пустым) |
| 07 | goods[3].tovg[1].quantity | | CO | количество в доп.ед.изм | operator:confirmed (not_applicable) |
| 08 | goods[3].tovg[1].unit_code | | CO | код ЕИ | operator:confirmed (not_applicable) |
| 09 | goods[3].tovg[1].unit_name | | CO | наименование ЕИ | operator:confirmed (not_applicable) |
| 10 | goods[3].tovg[1].gross_weight | 1.60 | CP | вес брутто по строке | copied_from:invoice.InvoiceGoods[3].GrossWeightQuantity (где dt_item_index == 3 и dt_tovg_index == 1) |
| 11 | goods[3].tovg[1].net_weight | 1.50 | CP | вес нетто по строке | copied_from:invoice.InvoiceGoods[3].NetWeightQuantity (где dt_item_index == 3 и dt_tovg_index == 1) |
| 12 | goods[3].tovg[1].invoice_cost | 3000.00 | CP | цена по строке | copied_from:invoice.InvoiceGoods[3].TotalCost (где dt_item_index == 3 и dt_tovg_index == 1) |

- _item_audit: 12

#### 3.16.9 Поле G_44 — текстовое поле

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | константа |

- _audit: 1

#### 3.16.10 Массив: goods[3].g44_docs[]
- _array_audit: 15

#### 3.16.11 Элемент массива: goods[3].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[1].doc_code | 04021 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[1].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[1].doc_name | ИНВОЙС | CP | наименование документа | formalized.invoice_1.doc_name |
| 04 | goods[3].g44_docs[1].doc_number | HNKY260226 | CP | номер документа | formalized.invoice_1.doc_number |
| 05 | goods[3].g44_docs[1].doc_date | 26.02.2026 | CP | дата документа | formalized.invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[2].doc_code | 04131 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[2].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[2].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | formalized.packing_list_1.doc_name |
| 04 | goods[3].g44_docs[2].doc_number | HNKY260226 | CP | номер документа | formalized.packing_list_1.doc_number |
| 05 | goods[3].g44_docs[2].doc_date | 26.02.2026 | CP | дата документа | formalized.packing_list_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[3].doc_code | 02015 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[3].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[3].doc_name | CMR | CP | наименование документа | formalized.cmr_1.doc_name |
| 04 | goods[3].g44_docs[3].doc_number | 09886 | CP | номер документа | formalized.cmr_1.doc_number |
| 05 | goods[3].g44_docs[3].doc_date | 13.05.2026 | CP | дата документа | formalized.cmr_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[4].doc_code | 04023 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[4].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[4].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | formalized.payment_order_1.doc_name |
| 04 | goods[3].g44_docs[4].doc_number | 2 | CP | номер документа | formalized.payment_order_1.doc_number |
| 05 | goods[3].g44_docs[4].doc_date | 27.02.2026 | CP | дата документа | formalized.payment_order_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[5].doc_code | 04031 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[5].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[5].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | formalized.service_invoice_1.doc_name |
| 04 | goods[3].g44_docs[5].doc_number | 26-09886-tl | CP | номер документа | formalized.service_invoice_1.doc_number |
| 05 | goods[3].g44_docs[5].doc_date | 12.05.2026 | CP | дата документа | formalized.service_invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[6].doc_code | 04111 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[6].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[6].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | formalized.insurance_document_1.doc_name |
| 04 | goods[3].g44_docs[6].doc_number | 26-09886-tl/1 | CP | номер документа | formalized.insurance_document_1.doc_number |
| 05 | goods[3].g44_docs[6].doc_date | 05.05.2026 | CP | дата документа | formalized.insurance_document_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[7].doc_code | 05999 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[7].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[7].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | formalized.tech_description_1.doc_name |
| 04 | goods[3].g44_docs[7].doc_number | БН | CP | номер документа | formalized.tech_description_1.doc_number |
| 05 | goods[3].g44_docs[7].doc_date | 26.02.2026 | CP | дата документа | formalized.tech_description_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[8].doc_code | 03011 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[8].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[8].doc_name | КОНТРАКТ | CP | наименование документа | formalized.contract.doc_name |
| 04 | goods[3].g44_docs[8].doc_number | HNKY250929 | CP | номер документа | formalized.contract.doc_number |
| 05 | goods[3].g44_docs[8].doc_date | 29.09.2025 | CP | дата документа | formalized.contract.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[9].doc_code | 03012 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[9].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[9].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | formalized.supplementary_contract_1.doc_name |
| 04 | goods[3].g44_docs[9].doc_number | 1 | CP | номер документа | formalized.supplementary_contract_1.doc_number |
| 05 | goods[3].g44_docs[9].doc_date | 26.02.2026 | CP | дата документа | formalized.supplementary_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[10].doc_code | 04011 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[10].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | master_data.egrul_1.doc_name |
| 04 | goods[3].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | master_data.egrul_1.doc_number |
| 05 | goods[3].g44_docs[10].doc_date | 14.07.2025 | CP | дата документа | master_data.egrul_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[11].doc_code | 11001 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[11].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[11].doc_name | ПАСПОРТ | CP | наименование документа | master_data.passport_1.doc_name |
| 04 | goods[3].g44_docs[11].doc_number | 63 09 449948 | CP | номер документа | master_data.passport_1.doc_number |
| 05 | goods[3].g44_docs[11].doc_date | 11.03.2010 | CP | дата документа | master_data.passport_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[12].doc_code | 11004 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[12].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | master_data.letter_of_attorney_1.doc_name |
| 04 | goods[3].g44_docs[12].doc_number | 1 | CP | номер документа | master_data.letter_of_attorney_1.doc_number |
| 05 | goods[3].g44_docs[12].doc_date | 01.02.2026 | CP | дата документа | master_data.letter_of_attorney_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[13].doc_code | 04033 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[13].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[13].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | master_data.transport_contract_1.doc_name |
| 04 | goods[3].g44_docs[13].doc_number | КООО/26651/М | CP | номер документа | master_data.transport_contract_1.doc_number |
| 05 | goods[3].g44_docs[13].doc_date | 13.05.2025 | CP | дата документа | master_data.transport_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[14].doc_code | 09023 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[14].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[14].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_1.doc_name |
| 04 | goods[3].g44_docs[14].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_1.doc_number |
| 05 | goods[3].g44_docs[14].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[3].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[3].g44_docs[15].doc_code | 09999 | CP | код документа | cb:doc |
| 02 | goods[3].g44_docs[15].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[3].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_source_1.doc_name |
| 04 | goods[3].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_source_1.doc_number |
| 05 | goods[3].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_source_1.doc_date |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[4]
- goods._element_num: 4

#### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g31.name | РОЗОВЫЕ ПОЛОСКИ ИЗ АНТИСТАТИЧЕСКОГО ПЕНОПОЛИЭТИЛЕНА 32ММ*42ММ*2ММ, СМ.ДОПОЛНЕНИЕ | D | описание товара | по данным tovg |
| 02 | goods[4].g31.manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | по данным tovg |
| 03 | goods[4].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | по данным tovg |
| 04 | goods[4].places | 1 | D | количество мест по товару | по данным tovg |

- _audit: 4

#### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].item_no | 4 | D | номер товара | порядковый номер |
| 02 | goods[4].tnved_code | 3926909709 | D | код товара | ТН ВЭД |
| 03 | goods[4].tnved.flag_1 | С | D | доп. признак | константа |
| 04 | goods[4].tnved.flag_2 | N | D | доп. признак | константа |
| 05 | goods[4].origin_country_code | CN | D | код страны происхождения | cb:country |
| 06 | goods[4].gross_weight | 4.60 | D | вес брутто по товару | по данным tovg |
| 07 | goods[4].preference | ОООО-ОО | D | преференция | константа |
| 08 | goods[4].net_weight | 4.40 | D | вес нетто по товару | по данным tovg |

- _audit: 8

#### 3.16.3 Графы 42–46 — стоимости по товару

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].invoice_cost | 2000.00 | D | цена товара | по данным tovg |
| 02 | goods[4].customs_value | | D | таможенная стоимость | курс пустой |
| 03 | goods[4].transport_to_border | | D | транспорт до границы | курс пустой |

- _audit: 3

#### 3.16.4 Массив: goods[4].txt[]
- _array_audit: 1

#### 3.16.5 Элемент массива: goods[4].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].txt[1].line_1 | АРТ: - | D | TXT строка 1 | по данным tovg |
| 02 | goods[4].txt[1].line_2 | Pink strips of antistatic polyethylene foam 32mm*42mm*2mm/ розовые полоски из антистатического пенополиэтилена 32мм*42мм*2мм | D | TXT строка 2 | по данным tovg |

- _item_audit: 2

#### 3.16.6 Массив: goods[4].tovg[]
- _array_audit: 1

#### 3.16.7 Элемент массива: goods[4].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].tovg[1].line_no | 1 | D | № строки таблицы | порядковый номер |
| 02 | goods[4].tovg[1].description | Pink strips of antistatic polyethylene foam 32mm*42mm*2mm/ розовые полоски из антистатического пенополиэтилена 32мм*42мм*2мм | D | наименование | copied_from:invoice.InvoiceGoods[4].GoodsDescription (где dt_item_index == 4 и dt_tovg_index == 1) |
| 03 | goods[4].tovg[1].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | copied_from:invoice.InvoiceGoods[4].AdditionalGoodsDescription_Manufacturer (где dt_item_index == 4 и dt_tovg_index == 1) |
| 04 | goods[4].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | copied_from:invoice.InvoiceGoods[4].AdditionalGoodsDescription_TradeMark (где dt_item_index == 4 и dt_tovg_index == 1) |
| 05 | goods[4].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | copied_from:invoice.InvoiceGoods[4].AdditionalGoodsDescription_GoodsMark (где dt_item_index == 4 и dt_tovg_index == 1) |
| 06 | goods[4].tovg[1].model | | CO | модель/модификация | operator:confirmed (оставить пустым) |
| 07 | goods[4].tovg[1].quantity | | CO | количество в доп.ед.изм | operator:confirmed (not_applicable) |
| 08 | goods[4].tovg[1].unit_code | | CO | код ЕИ | operator:confirmed (not_applicable) |
| 09 | goods[4].tovg[1].unit_name | | CO | наименование ЕИ | operator:confirmed (not_applicable) |
| 10 | goods[4].tovg[1].gross_weight | 4.60 | CP | вес брутто по строке | copied_from:invoice.InvoiceGoods[4].GrossWeightQuantity (где dt_item_index == 4 и dt_tovg_index == 1) |
| 11 | goods[4].tovg[1].net_weight | 4.40 | CP | вес нетто по строке | copied_from:invoice.InvoiceGoods[4].NetWeightQuantity (где dt_item_index == 4 и dt_tovg_index == 1) |
| 12 | goods[4].tovg[1].invoice_cost | 2000.00 | CP | цена по строке | copied_from:invoice.InvoiceGoods[4].TotalCost (где dt_item_index == 4 и dt_tovg_index == 1) |

- _item_audit: 12

#### 3.16.9 Поле G_44 — текстовое поле

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | константа |

- _audit: 1

#### 3.16.10 Массив: goods[4].g44_docs[]
- _array_audit: 15

#### 3.16.11 Элемент массива: goods[4].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[1].doc_code | 04021 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[1].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[1].doc_name | ИНВОЙС | CP | наименование документа | formalized.invoice_1.doc_name |
| 04 | goods[4].g44_docs[1].doc_number | HNKY260226 | CP | номер документа | formalized.invoice_1.doc_number |
| 05 | goods[4].g44_docs[1].doc_date | 26.02.2026 | CP | дата документа | formalized.invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[2].doc_code | 04131 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[2].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[2].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | formalized.packing_list_1.doc_name |
| 04 | goods[4].g44_docs[2].doc_number | HNKY260226 | CP | номер документа | formalized.packing_list_1.doc_number |
| 05 | goods[4].g44_docs[2].doc_date | 26.02.2026 | CP | дата документа | formalized.packing_list_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[3].doc_code | 02015 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[3].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[3].doc_name | CMR | CP | наименование документа | formalized.cmr_1.doc_name |
| 04 | goods[4].g44_docs[3].doc_number | 09886 | CP | номер документа | formalized.cmr_1.doc_number |
| 05 | goods[4].g44_docs[3].doc_date | 13.05.2026 | CP | дата документа | formalized.cmr_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[4].doc_code | 04023 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[4].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[4].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | formalized.payment_order_1.doc_name |
| 04 | goods[4].g44_docs[4].doc_number | 2 | CP | номер документа | formalized.payment_order_1.doc_number |
| 05 | goods[4].g44_docs[4].doc_date | 27.02.2026 | CP | дата документа | formalized.payment_order_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[5].doc_code | 04031 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[5].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[5].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | formalized.service_invoice_1.doc_name |
| 04 | goods[4].g44_docs[5].doc_number | 26-09886-tl | CP | номер документа | formalized.service_invoice_1.doc_number |
| 05 | goods[4].g44_docs[5].doc_date | 12.05.2026 | CP | дата документа | formalized.service_invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[6].doc_code | 04111 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[6].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[6].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | formalized.insurance_document_1.doc_name |
| 04 | goods[4].g44_docs[6].doc_number | 26-09886-tl/1 | CP | номер документа | formalized.insurance_document_1.doc_number |
| 05 | goods[4].g44_docs[6].doc_date | 05.05.2026 | CP | дата документа | formalized.insurance_document_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[7].doc_code | 05999 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[7].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[7].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | formalized.tech_description_1.doc_name |
| 04 | goods[4].g44_docs[7].doc_number | БН | CP | номер документа | formalized.tech_description_1.doc_number |
| 05 | goods[4].g44_docs[7].doc_date | 26.02.2026 | CP | дата документа | formalized.tech_description_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[8].doc_code | 03011 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[8].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[8].doc_name | КОНТРАКТ | CP | наименование документа | formalized.contract.doc_name |
| 04 | goods[4].g44_docs[8].doc_number | HNKY250929 | CP | номер документа | formalized.contract.doc_number |
| 05 | goods[4].g44_docs[8].doc_date | 29.09.2025 | CP | дата документа | formalized.contract.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[9].doc_code | 03012 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[9].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[9].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | formalized.supplementary_contract_1.doc_name |
| 04 | goods[4].g44_docs[9].doc_number | 1 | CP | номер документа | formalized.supplementary_contract_1.doc_number |
| 05 | goods[4].g44_docs[9].doc_date | 26.02.2026 | CP | дата документа | formalized.supplementary_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[10].doc_code | 04011 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[10].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | master_data.egrul_1.doc_name |
| 04 | goods[4].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | master_data.egrul_1.doc_number |
| 05 | goods[4].g44_docs[10].doc_date | 14.07.2025 | CP | дата документа | master_data.egrul_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[11].doc_code | 11001 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[11].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[11].doc_name | ПАСПОРТ | CP | наименование документа | master_data.passport_1.doc_name |
| 04 | goods[4].g44_docs[11].doc_number | 63 09 449948 | CP | номер документа | master_data.passport_1.doc_number |
| 05 | goods[4].g44_docs[11].doc_date | 11.03.2010 | CP | дата документа | master_data.passport_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[12].doc_code | 11004 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[12].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | master_data.letter_of_attorney_1.doc_name |
| 04 | goods[4].g44_docs[12].doc_number | 1 | CP | номер документа | master_data.letter_of_attorney_1.doc_number |
| 05 | goods[4].g44_docs[12].doc_date | 01.02.2026 | CP | дата документа | master_data.letter_of_attorney_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[13].doc_code | 04033 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[13].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[13].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | master_data.transport_contract_1.doc_name |
| 04 | goods[4].g44_docs[13].doc_number | КООО/26651/М | CP | номер документа | master_data.transport_contract_1.doc_number |
| 05 | goods[4].g44_docs[13].doc_date | 13.05.2025 | CP | дата документа | master_data.transport_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[14].doc_code | 09023 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[14].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[14].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_1.doc_name |
| 04 | goods[4].g44_docs[14].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_1.doc_number |
| 05 | goods[4].g44_docs[14].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[4].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[4].g44_docs[15].doc_code | 09999 | CP | код документа | cb:doc |
| 02 | goods[4].g44_docs[15].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[4].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_source_1.doc_name |
| 04 | goods[4].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_source_1.doc_number |
| 05 | goods[4].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_source_1.doc_date |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[5]
- goods._element_num: 5

#### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g31.name | САМОРЕЗ ИЗ ОЦИНКОВАННОЙ СТАЛИ 0,4*1,5 ММ, СМ.ДОПОЛНЕНИЕ | D | описание товара | по данным tovg |
| 02 | goods[5].g31.manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | по данным tovg |
| 03 | goods[5].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | по данным tovg |
| 04 | goods[5].places | | CO | количество мест по товару | operator:confirmed (оставить пустым) |

- _audit: 4

#### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].item_no | 5 | D | номер товара | порядковый номер |
| 02 | goods[5].tnved_code | 7318149100 | D | код товара | ТН ВЭД |
| 03 | goods[5].tnved.flag_1 | С | D | доп. признак | константа |
| 04 | goods[5].tnved.flag_2 | N | D | доп. признак | константа |
| 05 | goods[5].origin_country_code | CN | D | код страны происхождения | cb:country |
| 06 | goods[5].gross_weight | 6.00 | D | вес брутто по товару | по данным tovg |
| 07 | goods[5].preference | ОООО-ОО | D | преференция | константа |
| 08 | goods[5].net_weight | 5.80 | D | вес нетто по товару | по данным tovg |

- _audit: 8

#### 3.16.3 Графы 42–46 — стоимости по товару

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].invoice_cost | 1300.00 | D | цена товара | по данным tovg |
| 02 | goods[5].customs_value | | D | таможенная стоимость | курс пустой |
| 03 | goods[5].transport_to_border | | D | транспорт до границы | курс пустой |

- _audit: 3

#### 3.16.4 Массив: goods[5].txt[]
- _array_audit: 1

#### 3.16.5 Элемент массива: goods[5].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].txt[1].line_1 | АРТ: - | D | TXT строка 1 | по данным tovg |
| 02 | goods[5].txt[1].line_2 | Self-tapping screw made of galvanized steel 0,4*1,5 мм / Саморез из оцинкованной стали 0,4*1,5 мм | D | TXT строка 2 | по данным tovg |

- _item_audit: 2

#### 3.16.6 Массив: goods[5].tovg[]
- _array_audit: 1

#### 3.16.7 Элемент массива: goods[5].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].tovg[1].line_no | 1 | D | № строки таблицы | порядковый номер |
| 02 | goods[5].tovg[1].description | Self-tapping screw made of galvanized steel 0,4*1,5 мм / Саморез из оцинкованной стали 0,4*1,5 мм | D | наименование | copied_from:invoice.InvoiceGoods[5].GoodsDescription (где dt_item_index == 5 и dt_tovg_index == 1) |
| 03 | goods[5].tovg[1].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | copied_from:invoice.InvoiceGoods[5].AdditionalGoodsDescription_Manufacturer (где dt_item_index == 5 и dt_tovg_index == 1) |
| 04 | goods[5].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | copied_from:invoice.InvoiceGoods[5].AdditionalGoodsDescription_TradeMark (где dt_item_index == 5 и dt_tovg_index == 1) |
| 05 | goods[5].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | copied_from:invoice.InvoiceGoods[5].AdditionalGoodsDescription_GoodsMark (где dt_item_index == 5 и dt_tovg_index == 1) |
| 06 | goods[5].tovg[1].model | | CO | модель/модификация | operator:confirmed (оставить пустым) |
| 07 | goods[5].tovg[1].quantity | | CO | количество в доп.ед.изм | operator:confirmed (not_applicable) |
| 08 | goods[5].tovg[1].unit_code | | CO | код ЕИ | operator:confirmed (not_applicable) |
| 09 | goods[5].tovg[1].unit_name | | CO | наименование ЕИ | operator:confirmed (not_applicable) |
| 10 | goods[5].tovg[1].gross_weight | 6.00 | CP | вес брутто по строке | copied_from:invoice.InvoiceGoods[5].GrossWeightQuantity (где dt_item_index == 5 и dt_tovg_index == 1) |
| 11 | goods[5].tovg[1].net_weight | 5.80 | CP | вес нетто по строке | copied_from:invoice.InvoiceGoods[5].NetWeightQuantity (где dt_item_index == 5 и dt_tovg_index == 1) |
| 12 | goods[5].tovg[1].invoice_cost | 1300.00 | CP | цена по строке | copied_from:invoice.InvoiceGoods[5].TotalCost (где dt_item_index == 5 и dt_tovg_index == 1) |

- _item_audit: 12

#### 3.16.9 Поле G_44 — текстовое поле

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | константа |

- _audit: 1

#### 3.16.10 Массив: goods[5].g44_docs[]
- _array_audit: 15

#### 3.16.11 Элемент массива: goods[5].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[1].doc_code | 04021 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[1].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[1].doc_name | ИНВОЙС | CP | наименование документа | formalized.invoice_1.doc_name |
| 04 | goods[5].g44_docs[1].doc_number | HNKY260226 | CP | номер документа | formalized.invoice_1.doc_number |
| 05 | goods[5].g44_docs[1].doc_date | 26.02.2026 | CP | дата документа | formalized.invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[2].doc_code | 04131 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[2].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[2].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | formalized.packing_list_1.doc_name |
| 04 | goods[5].g44_docs[2].doc_number | HNKY260226 | CP | номер документа | formalized.packing_list_1.doc_number |
| 05 | goods[5].g44_docs[2].doc_date | 26.02.2026 | CP | дата документа | formalized.packing_list_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[3].doc_code | 02015 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[3].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[3].doc_name | CMR | CP | наименование документа | formalized.cmr_1.doc_name |
| 04 | goods[5].g44_docs[3].doc_number | 09886 | CP | номер документа | formalized.cmr_1.doc_number |
| 05 | goods[5].g44_docs[3].doc_date | 13.05.2026 | CP | дата документа | formalized.cmr_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[4].doc_code | 04023 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[4].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[4].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | formalized.payment_order_1.doc_name |
| 04 | goods[5].g44_docs[4].doc_number | 2 | CP | номер документа | formalized.payment_order_1.doc_number |
| 05 | goods[5].g44_docs[4].doc_date | 27.02.2026 | CP | дата документа | formalized.payment_order_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[5].doc_code | 04031 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[5].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[5].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | formalized.service_invoice_1.doc_name |
| 04 | goods[5].g44_docs[5].doc_number | 26-09886-tl | CP | номер документа | formalized.service_invoice_1.doc_number |
| 05 | goods[5].g44_docs[5].doc_date | 12.05.2026 | CP | дата документа | formalized.service_invoice_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[6].doc_code | 04111 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[6].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[6].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | formalized.insurance_document_1.doc_name |
| 04 | goods[5].g44_docs[6].doc_number | 26-09886-tl/1 | CP | номер документа | formalized.insurance_document_1.doc_number |
| 05 | goods[5].g44_docs[6].doc_date | 05.05.2026 | CP | дата документа | formalized.insurance_document_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[7].doc_code | 05999 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[7].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[7].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | formalized.tech_description_1.doc_name |
| 04 | goods[5].g44_docs[7].doc_number | БН | CP | номер документа | formalized.tech_description_1.doc_number |
| 05 | goods[5].g44_docs[7].doc_date | 26.02.2026 | CP | дата документа | formalized.tech_description_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[8].doc_code | 03011 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[8].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[8].doc_name | КОНТРАКТ | CP | наименование документа | formalized.contract.doc_name |
| 04 | goods[5].g44_docs[8].doc_number | HNKY250929 | CP | номер документа | formalized.contract.doc_number |
| 05 | goods[5].g44_docs[8].doc_date | 29.09.2025 | CP | дата документа | formalized.contract.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[9].doc_code | 03012 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[9].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[9].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | formalized.supplementary_contract_1.doc_name |
| 04 | goods[5].g44_docs[9].doc_number | 1 | CP | номер документа | formalized.supplementary_contract_1.doc_number |
| 05 | goods[5].g44_docs[9].doc_date | 26.02.2026 | CP | дата документа | formalized.supplementary_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[10].doc_code | 04011 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[10].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | master_data.egrul_1.doc_name |
| 04 | goods[5].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | master_data.egrul_1.doc_number |
| 05 | goods[5].g44_docs[10].doc_date | 14.07.2025 | CP | дата документа | master_data.egrul_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[11].doc_code | 11001 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[11].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[11].doc_name | ПАСПОРТ | CP | наименование документа | master_data.passport_1.doc_name |
| 04 | goods[5].g44_docs[11].doc_number | 63 09 449948 | CP | номер документа | master_data.passport_1.doc_number |
| 05 | goods[5].g44_docs[11].doc_date | 11.03.2010 | CP | дата документа | master_data.passport_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[12].doc_code | 11004 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[12].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | master_data.letter_of_attorney_1.doc_name |
| 04 | goods[5].g44_docs[12].doc_number | 1 | CP | номер документа | master_data.letter_of_attorney_1.doc_number |
| 05 | goods[5].g44_docs[12].doc_date | 01.02.2026 | CP | дата документа | master_data.letter_of_attorney_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[13].doc_code | 04033 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[13].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[13].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | master_data.transport_contract_1.doc_name |
| 04 | goods[5].g44_docs[13].doc_number | КООО/26651/М | CP | номер документа | master_data.transport_contract_1.doc_number |
| 05 | goods[5].g44_docs[13].doc_date | 13.05.2025 | CP | дата документа | master_data.transport_contract_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[14].doc_code | 09023 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[14].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[14].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_1.doc_name |
| 04 | goods[5].g44_docs[14].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_1.doc_number |
| 05 | goods[5].g44_docs[14].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_1.doc_date |

- _item_audit: 5

#### 3.16.11 Элемент массива: goods[5].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | goods[5].g44_docs[15].doc_code | 09999 | CP | код документа | cb:doc |
| 02 | goods[5].g44_docs[15].kind_code | 0 | CO | признак записи | константа |
| 03 | goods[5].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | master_data.exemption_letter_source_1.doc_name |
| 04 | goods[5].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | master_data.exemption_letter_source_1.doc_number |
| 05 | goods[5].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | master_data.exemption_letter_source_1.doc_date |

- _item_audit: 5

### 3.17 Теги после товаров и документов (графы 51–54)

#### 3.17.1 Графа 54 — уполномоченное лицо / представитель

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | representative.date | 30.05.2026 | D | дата заполнения/подачи | текущая дата |
| 02 | representative.phone | +7 927-222-0500 | CP | телефон | master_data.passport_1.Phone |
| 03 | representative.email | A.K.ARBUZOVA@YANDEX.RU | CP | e-mail | master_data.passport_1.Email |
| 04 | representative.last_name | АРБУЗОВА | CP | фамилия | master_data.passport_1.PersonSurname |
| 05 | representative.first_name | АНАСТАСИЯ | CP | имя | master_data.passport_1.PersonName |
| 06 | representative.middle_name | КОНСТАНТИНОВНА | CP | отчество | master_data.passport_1.PersonMiddleName |
| 07 | representative.authority_doc_name | ДОВЕРЕННОСТЬ | CP | документ полномочий | master_data.letter_of_attorney_1.doc_name |
| 08 | representative.authority_doc_number | 1 | CP | № документа полномочий | master_data.letter_of_attorney_1.DocumentNumber |
| 09 | representative.authority_doc_date_from | 01.02.2026 | CP | дата начала действия | master_data.letter_of_attorney_1.DocumentDate |
| 10 | representative.authority_doc_date_to | 31.12.2026 | CP | дата окончания действия | master_data.letter_of_attorney_1.EndDate |
| 11 | representative.position | УПОЛНОМОЧЕННОЕ ЛИЦО | CP | должность/статус | master_data.letter_of_attorney_1.EmpoweredPost |
| 12 | representative.passport_code | RU01001 | CP | код документа личности | константа |
| 13 | representative.passport_name | ПАСРФ | CP | наименование документа | константа |
| 14 | representative.passport_number | 449948 | CP | номер паспорта | master_data.passport_1.CardNumber |
| 15 | representative.passport_date | 11.03.2010 | CP | дата выдачи паспорта | master_data.passport_1.CardDate |
| 16 | representative.passport_series | 63 09 | CP | серия паспорта | master_data.passport_1.CardSeries |
| 17 | representative.passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CP | кем выдан | master_data.passport_1.OrganizationName |

- _audit: 17

### Итог:
- `dt_status`: confirmed

### Часть II: Issues (нерешенные вопросы)

**Для полей:**
- Нет.

**Для общих вопросов:**
- `[Общий]`:
  - `question`: Нет.
