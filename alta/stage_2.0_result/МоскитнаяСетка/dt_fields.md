# Исходные данные для ДТ

## Метаданные:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `тип поставки`: 1 ДТ / 7 позиций инвойса / 2 товара ДТ
- `агрегация ДТ`: 2 товара по кодам ТН ВЭД
- `источники данных`: primary.md + operator_provided_data

## Часть I: Поля ДТ

### 3.1 Заголовок декларации (графа 1)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | declaration.direction | ИМ | CP | направление декларации | |
| 02 | declaration.procedure | 40 | CO | код таможенной процедуры | |
| 03 | declaration.form | ЭД | D | форма подачи декларации | |

- _audit: 3

### 3.2 Отправитель (графа 2)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | sender.country_name | China | CP | название страны | |
| 02 | sender.country_code | CN | CP | код страны alpha-2 | |
| 03 | sender.name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CP | наименование отправителя | |
| 04 | sender.region | Hebei | CP | область/район | |
| 05 | sender.city | Shijiazhuang | CP | город | |
| 06 | sender.street | No. 5 Gaodong street | CP | улица и дом | |

- _audit: 6

### 3.3 Количество товаров и мест (графы 5, 6)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.total_goods_number | 2 | D | количество товарных позиций | |
| 02 | shipment.packages_flag | true | D | признак подсчёта мест | |
| 03 | shipment.total_packages | 127 | D | общее количество грузовых мест | |

- _audit: 3

### 3.4 Получатель (графа 8)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | consignee.same_as_declarant | true | D | признак «см. графу 14» | |
| 02 | consignee.ogrn | 1201600020390 | D | ОГРН получателя | |
| 03 | consignee.inn_kpp | 1650389298/165001001 | D | ИНН/КПП получателя | |
| 04 | consignee.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование организации | |
| 05 | consignee.country_code | RU | D | код страны alpha-2 | |
| 06 | consignee.country_name | РОССИЯ | D | страна, наименование | |
| 07 | consignee.postcode | 423800 | D | почтовый индекс | |
| 08 | consignee.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион | |
| 09 | consignee.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | населённый пункт | |
| 10 | consignee.street | ПРОЕЗД ХЛЕБНЫЙ | D | улица | |
| 11 | consignee.building | Д. 30 | D | дом | |
| 12 | consignee.room | ОФИС 211 | D | помещение/офис | |
| 13 | consignee.phone | +7 (843) 207 18 90 | D | телефон | |
| 14 | consignee.email | PROM_TAT@MAIL.RU | D | e-mail | |

- _audit: 14

### 3.5 Финансовое урегулирование (графа 9)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | financial.same_as_declarant | true | D | признак «см. графу 14» | |
| 02 | financial.ogrn | 1201600020390 | D | ОГРН | |
| 03 | financial.inn_kpp | 1650389298/165001001 | D | ИНН/КПП | |
| 04 | financial.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование | |
| 05 | financial.country_code | RU | D | код страны | |
| 06 | financial.country_name | РОССИЯ | D | наименование страны | |
| 07 | financial.postcode | 423800 | D | индекс | |
| 08 | financial.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион | |
| 09 | financial.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | город | |
| 10 | financial.street | ПРОЕЗД ХЛЕБНЫЙ | D | улица | |
| 11 | financial.building | Д. 30 | D | дом | |
| 12 | financial.room | ОФИС 211 | D | помещение | |
| 13 | financial.country_code_alt | RU | D | дублирующий код страны | |
| 14 | financial.phone | +7 (843) 207 18 90 | D | телефон | |
| 15 | financial.email | PROM_TAT@MAIL.RU | D | e-mail | |

- _audit: 15

### 3.6 Торгующая страна (графа 11)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.trade_country_code | CN | CP | код торгующей страны alpha-2 | |

- _audit: 1

### 3.7 Декларант (графа 14)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | declarant.ogrn | 1201600020390 | CP | ОГРН декларанта | |
| 02 | declarant.inn_kpp | 1650389298/165001001 | D | ИНН/КПП через "/" | |
| 03 | declarant.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CP | наименование организации | |
| 04 | declarant.country_code | RU | CP | код страны | |
| 05 | declarant.country_name | РОССИЯ | CP | наименование страны | |
| 06 | declarant.postcode | 423800 | CP | почтовый индекс | |
| 07 | declarant.region | РЕСПУБЛИКА ТАТАРСТАН | CP | регион | |
| 08 | declarant.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | населённый пункт | |
| 09 | declarant.street | ПРОЕЗД ХЛЕБНЫЙ | CP | улица | |
| 10 | declarant.building | Д. 30 | CP | дом | |
| 11 | declarant.room | ОФИС 211 | D | помещение/офис | |
| 12 | declarant.phone | +7 (843) 207 18 90 | CP | телефон | |
| 13 | declarant.email | PROM_TAT@MAIL.RU | CP | e-mail | |

- _audit: 13

### 3.8 Страны (графы 15, 16, 17)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.dispatch_country_code | CN | CP | код страны отправления alpha-2 | |
| 02 | shipment.destination_country_code | RU | CP | код страны назначения alpha-2 | |
| 03 | shipment.dispatch_country_name | КИТАЙ | D | страна отправления, текст | |
| 04 | shipment.destination_country_name | РОССИЯ | D | страна назначения, текст | |
| 05 | shipment.origin_country_code | CN | D | код страны происхождения alpha-2 | |
| 06 | shipment.origin_country_name | КИТАЙ | D | страна происхождения, текст | |

- _audit: 6

### 3.9 Условия поставки (графа 20)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | delivery.terms_code | EXW | D | условия поставки | |
| 02 | delivery.place_name | HEBEI | D | место поставки | |

- _audit: 2

### 3.10 Транспорт (графы 18, 19, 21)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.vehicles_count | 2 | D | количество транспортных средств | |
| 02 | transport.identification | О157АО774/ВТ374974 | D | идентификация ТС | |
| 03 | transport.registration_country_code | 00 | D | код страны регистрации ТС | |
| 04 | transport.container_flag | 0 | CO | признак контейнера | |
| 05 | transport.border_mode | 1 | D | код активного ТС на границе | |

- _audit: 5

### 3.11 Валюта и стоимость (графа 22)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.invoice_currency_numeric | 156 | D | цифровой код валюты | |
| 02 | shipment.invoice_currency_alpha | CNY | CP | буквенный код валюты | |
| 03 | shipment.invoice_amount | 97260.00 | CP | сумма по счёту | |

- _audit: 3

### 3.12 Курс валюты (графа 23)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.currency_rate | 10.9430 | CP | курс валюты к рублю | |

- _audit: 1

### 3.13 Вид транспорта (графы 25, 26)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.border_transport_code | 31 | D | код вида транспорта на границе | |
| 02 | transport.internal_transport_code | 31 | D | код вида транспорта внутри страны | |

- _audit: 2

### 3.14 Таможня на границе (графа 29)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs.border_code | 10719110 | CP | код таможенного органа на границе | |
| 02 | customs.border_name | ОТСУТСТВУЕТ | CP | наименование таможенного поста | |

- _audit: 2

### 3.15 Местонахождение товаров (графа 30)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | location.type | 11 | D | тип места нахождения товаров | |
| 02 | location.document_kind | 2 | D | вид документа СВХ | |
| 03 | location.document_number | 10404/141210/10092/5 | CP | номер документа СВХ | |
| 04 | location.document_date | 18.09.2025 | CP | дата документа СВХ | |
| 05 | location.address.country_code | RU | D | код страны местонахождения | |
| 06 | location.address.region | РЕСПУБЛИКА ТАТАРСТАН | CP | регион | |
| 07 | location.address.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | город | |
| 08 | location.address.street | Производственный пр-д, д. 45 | CP | улица и дом | |
| 09 | location.customs_code | 10404083 | CP | код таможенного органа СВХ | |

- _audit: 9

### 3.16 Массив: goods[]
- goods._array_audit: 2

#### 3.16.0 Элемент массива: goods[1]
- goods._element_num: 1

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g31.name | СЕТКА МОСКИТНАЯ ИЗ ПОЛИЭСТЕРА, СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[1].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | |
| 03 | goods[1].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | |
| 04 | goods[1].places | 100 | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].item_no | 1 | D | номер товара | |
| 02 | goods[1].tnved_code | 5804101000 | D | код товара | |
| 03 | goods[1].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[1].tnved.flag_2 | N | D | доп. признак | |
| 05 | goods[1].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[1].gross_weight | 1790.00 | D | вес брутто по товару | |
| 07 | goods[1].preference | ОООО-ОО | D | преференция | |
| 08 | goods[1].net_weight | 1687.40 | D | вес нетто по товару | |

- _audit: 8

##### 3.16.3 Графы 42–46 — стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].invoice_cost | 55032.00 | D | цена товара | |
| 02 | goods[1].customs_value | 660173.93 | D | таможенная стоимость | |
| 03 | goods[1].transport_to_border | 57443.66 | D | доля расходов на транспорт | |

- _audit: 3

##### 3.16.4 Массив: goods[1].txt[]
- _array_audit: 5

###### 3.16.5 Элемент массива: goods[1].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[1].line_1 | АРТ: - 2520 M2 | D | строка 1 (арт/кол-во/ед) | |
| 02 | goods[1].txt[1].line_2 | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | D | строка 2 (наименование) | |

- _item_audit: 2

###### 3.16.5 Элемент массива: goods[1].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[2].line_1 | АРТ: - 1440 M2 | D | строка 1 (арт/кол-во/ед) | |
| 02 | goods[1].txt[2].line_2 | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | D | строка 2 (наименование) | |

- _item_audit: 2

###### 3.16.5 Элемент массива: goods[1].txt[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[3].line_1 | АРТ: - 2520 M2 | D | строка 1 (арт/кол-во/ед) | |
| 02 | goods[1].txt[3].line_2 | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | D | строка 2 (наименование) | |

- _item_audit: 2

###### 3.16.5 Элемент массива: goods[1].txt[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[4].line_1 | АРТ: - 1440 M2 | D | строка 1 (арт/кол-во/ед) | |
| 02 | goods[1].txt[4].line_2 | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | D | строка 2 (наименование) | |

- _item_audit: 2

###### 3.16.5 Элемент массива: goods[1].txt[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[5].line_1 | АРТ: - 240 M2 | D | строка 1 (арт/кол-во/ед) | |
| 02 | goods[1].txt[5].line_2 | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | D | строка 2 (наименование) | |

- _item_audit: 2

##### 3.16.6 Массив: goods[1].tovg[]
- _array_audit: 5

###### 3.16.7 Элемент массива: goods[1].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[1].tovg[1].description | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | D | наименование | |
| 03 | goods[1].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | производитель | |
| 04 | goods[1].tovg[1].trade_mark | ОТСУТСТВУЕТ | CP | марка/ТМ | |
| 05 | goods[1].tovg[1].goods_mark | ОТСУТСТВУЕТ | CP | товарный знак | |
| 06 | goods[1].tovg[1].model | Roll size 1.4*30 | CP | модель/модификация | |
| 07 | goods[1].tovg[1].quantity | 2520 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[1].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[1].unit_name | M2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[1].gross_weight | 855.00 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[1].net_weight | 806.60 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[1].invoice_cost | 14742.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[1].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[2].line_no | 2 | D | № строки таблицы | |
| 02 | goods[1].tovg[2].description | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | D | наименование | |
| 03 | goods[1].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | производитель | |
| 04 | goods[1].tovg[2].trade_mark | ОТСУТСТВУЕТ | CP | марка/ТМ | |
| 05 | goods[1].tovg[2].goods_mark | ОТСУТСТВУЕТ | CP | товарный знак | |
| 06 | goods[1].tovg[2].model | Roll size 1.6*30 | CP | модель/модификация | |
| 07 | goods[1].tovg[2].quantity | 1440 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[2].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[2].unit_name | M2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[2].gross_weight | 490.00 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[2].net_weight | 460.80 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[2].invoice_cost | 8424.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[1].tovg[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[3].line_no | 3 | D | № строки таблицы | |
| 02 | goods[1].tovg[3].description | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | D | наименование | |
| 03 | goods[1].tovg[3].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | производитель | |
| 04 | goods[1].tovg[3].trade_mark | ОТСУТСТВУЕТ | CP | марка/ТМ | |
| 05 | goods[1].tovg[3].goods_mark | ОТСУТСТВУЕТ | CP | товарный знак | |
| 06 | goods[1].tovg[3].model | Roll size 1,4*30 | CP | модель/модификация | |
| 07 | goods[1].tovg[3].quantity | 2520 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[3].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[3].unit_name | M2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[3].gross_weight | 265.00 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[3].net_weight | 252.00 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[3].invoice_cost | 16002.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[1].tovg[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[4].line_no | 4 | D | № строки таблицы | |
| 02 | goods[1].tovg[4].description | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | D | наименование | |
| 03 | goods[1].tovg[4].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | производитель | |
| 04 | goods[1].tovg[4].trade_mark | ОТСУТСТВУЕТ | CP | марка/ТМ | |
| 05 | goods[1].tovg[4].goods_mark | ОТСУТСТВУЕТ | CP | товарный знак | |
| 06 | goods[1].tovg[4].model | Roll size 1,6*30 | CP | модель/модификация | |
| 07 | goods[1].tovg[4].quantity | 1440 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[4].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[4].unit_name | M2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[4].gross_weight | 155.00 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[4].net_weight | 144.00 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[4].invoice_cost | 9144.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[1].tovg[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[5].line_no | 5 | D | № строки таблицы | |
| 02 | goods[1].tovg[5].description | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | D | наименование | |
| 03 | goods[1].tovg[5].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | производитель | |
| 04 | goods[1].tovg[5].trade_mark | ОТСУТСТВУЕТ | CP | марка/ТМ | |
| 05 | goods[1].tovg[5].goods_mark | ОТСУТСТВУЕТ | CP | товарный знак | |
| 06 | goods[1].tovg[5].model | Roll size 1,6*30 | CP | модель/модификация | |
| 07 | goods[1].tovg[5].quantity | 240 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[5].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[5].unit_name | M2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[5].gross_weight | 25.00 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[5].net_weight | 24.00 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[5].invoice_cost | 6720.00 | CP | цена по строке | |

- _item_audit: 12

##### 3.16.9 Поле G_44 (текстовое поле в карточке товара)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[1].g44_docs[]
- _array_audit: 17

###### 3.16.11 Элемент массива: goods[1].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[1].doc_code | 04021 | CP | код документа | |
| 02 | goods[1].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[1].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[1].g44_docs[1].doc_number | LM-2591 | CP | номер документа | |
| 05 | goods[1].g44_docs[1].doc_date | 30.10.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[2].doc_code | 04131 | CP | код документа | |
| 02 | goods[1].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[2].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[2].doc_number | LM-2591 | CP | номер документа | |
| 05 | goods[1].g44_docs[2].doc_date | 30.10.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[3].doc_code | 02015 | CP | код документа | |
| 02 | goods[1].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[3].doc_name | CMR | CP | наименование документа | |
| 04 | goods[1].g44_docs[3].doc_number | 00378 | CP | номер документа | |
| 05 | goods[1].g44_docs[3].doc_date | 20.01.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[4].doc_code | 04023 | CP | код документа | |
| 02 | goods[1].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[4].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[4].doc_number | 1 | CP | номер документа | |
| 05 | goods[1].g44_docs[4].doc_date | 13.01.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[5].doc_code | 04023 | CP | код документа | |
| 02 | goods[1].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[5].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[5].doc_number | 7 | CP | номер документа | |
| 05 | goods[1].g44_docs[5].doc_date | 28.11.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[6].doc_code | 04031 | CP | код документа | |
| 02 | goods[1].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[6].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[6].doc_number | 26-00378-tl | CP | номер документа | |
| 05 | goods[1].g44_docs[6].doc_date | 27.01.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[7].doc_code | 04111 | CP | код документа | |
| 02 | goods[1].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[7].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[7].doc_number | 26-00378-tl/1 | CP | номер документа | |
| 05 | goods[1].g44_docs[7].doc_date | 14.01.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[8].doc_code | 05999 | CP | код документа | |
| 02 | goods[1].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[8].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[8].doc_number | Б/Н | CO | номер документа | |
| 05 | goods[1].g44_docs[8].doc_date | 30.10.2025 | CO | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[9].doc_code | 03011 | CP | код документа | |
| 02 | goods[1].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[9].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[9].doc_number | LM-2553 | CP | номер документа | |
| 05 | goods[1].g44_docs[9].doc_date | 02.07.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[10].doc_code | 03012 | CP | код документа | |
| 02 | goods[1].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[10].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[10].doc_number | 1 | CP | номер документа | |
| 05 | goods[1].g44_docs[10].doc_date | 25.11.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[11].doc_code | 04011 | CP | код документа | |
| 02 | goods[1].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[11].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[1].g44_docs[11].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | |
| 05 | goods[1].g44_docs[11].doc_date | 14.07.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[12].doc_code | 11001 | CP | код документа | |
| 02 | goods[1].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[12].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[12].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[1].g44_docs[12].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[13].doc_code | 11004 | CP | код документа | |
| 02 | goods[1].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[13].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[1].g44_docs[13].doc_number | 1 | CP | номер документа | |
| 05 | goods[1].g44_docs[13].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[14].doc_code | 04033 | CP | код документа | |
| 02 | goods[1].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[14].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[14].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[1].g44_docs[14].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[15].doc_code | 09023 | CP | код документа | |
| 02 | goods[1].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[1].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[1].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[16].doc_code | 09999 | CP | код документа | |
| 02 | goods[1].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[16].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[1].g44_docs[16].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[1].g44_docs[16].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[17]
- _element_num: 17

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[17].doc_code | 09013 | CP | код документа | |
| 02 | goods[1].g44_docs[17].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[17].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | |
| 04 | goods[1].g44_docs[17].doc_number | 10719110/240126/5011363 | CO | номер документа | |
| 05 | goods[1].g44_docs[17].doc_date | 24.01.2026 | CO | дата документа | |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[2]
- goods._element_num: 2

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g31.name | СЕТКА МОСКИТНАЯ ИЗ СТЕКЛОВОЛОКНА, СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[2].g31.manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | D | производитель | |
| 03 | goods[2].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | |
| 04 | goods[2].places | 27 | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].item_no | 2 | D | номер товара | |
| 02 | goods[2].tnved_code | 7019900095 | D | код товара | |
| 03 | goods[2].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[2].tnved.flag_2 | N | D | доп. признак | |
| 05 | goods[2].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[2].gross_weight | 1710.00 | D | вес брутто по товару | |
| 07 | goods[2].preference | ОООО-ОО | D | преференция | |
| 08 | goods[2].net_weight | 1614.60 | D | вес нетто по товару | |

- _audit: 8

##### 3.16.3 Графы 42–46 — стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].invoice_cost | 42228.00 | D | цена товара | |
| 02 | goods[2].customs_value | 517372.59 | D | таможенная стоимость | |
| 03 | goods[2].transport_to_border | 54876.34 | D | доля расходов на транспорт | |

- _audit: 3

##### 3.16.4 Массив: goods[2].txt[]
- _array_audit: 2

###### 3.16.5 Элемент массива: goods[2].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[1].line_1 | АРТ: - 3780 M2 | D | строка 1 (арт/кол-во/ед) | |
| 02 | goods[2].txt[1].line_2 | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | D | строка 2 (наименование) | |

- _item_audit: 2

###### 3.16.5 Элемент массива: goods[2].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[2].line_1 | АРТ: - 8640 M2 | D | строка 1 (арт/кол-во/ед) | |
| 02 | goods[2].txt[2].line_2 | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | D | строка 2 (наименование) | |

- _item_audit: 2

##### 3.16.6 Массив: goods[2].tovg[]
- _array_audit: 2

###### 3.16.7 Элемент массива: goods[2].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[2].tovg[1].description | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | D | наименование | |
| 03 | goods[2].tovg[1].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | производитель | |
| 04 | goods[2].tovg[1].trade_mark | ОТСУТСТВУЕТ | CP | марка/ТМ | |
| 05 | goods[2].tovg[1].goods_mark | ОТСУТСТВУЕТ | CP | товарный знак | |
| 06 | goods[2].tovg[1].model | Roll size 1,4*30 | CP | модель/модификация | |
| 07 | goods[2].tovg[1].quantity | 3780 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[1].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[1].unit_name | M2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[1].gross_weight | 520.00 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[1].net_weight | 491.40 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[1].invoice_cost | 12852.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[2].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[2].line_no | 2 | D | № строки таблицы | |
| 02 | goods[2].tovg[2].description | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | D | наименование | |
| 03 | goods[2].tovg[2].manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CP | производитель | |
| 04 | goods[2].tovg[2].trade_mark | ОТСУТСТВУЕТ | CP | марка/ТМ | |
| 05 | goods[2].tovg[2].goods_mark | ОТСУТСТВУЕТ | CP | товарный знак | |
| 06 | goods[2].tovg[2].model | Roll size 1,6*30 | CP | модель/модификация | |
| 07 | goods[2].tovg[2].quantity | 8640 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[2].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[2].unit_name | M2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[2].gross_weight | 1190.00 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[2].net_weight | 1123.20 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[2].invoice_cost | 29376.00 | CP | цена по строке | |

- _item_audit: 12

##### 3.16.9 Поле G_44 (текстовое поле в карточке товара)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[2].g44_docs[]
- _array_audit: 17

###### 3.16.11 Элемент массива: goods[2].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[1].doc_code | 04021 | CP | код документа | |
| 02 | goods[2].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[1].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[2].g44_docs[1].doc_number | LM-2591 | CP | номер документа | |
| 05 | goods[2].g44_docs[1].doc_date | 30.10.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[2].doc_code | 04131 | CP | код документа | |
| 02 | goods[2].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[2].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[2].doc_number | LM-2591 | CP | номер документа | |
| 05 | goods[2].g44_docs[2].doc_date | 30.10.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[3].doc_code | 02015 | CP | код документа | |
| 02 | goods[2].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[3].doc_name | CMR | CP | наименование документа | |
| 04 | goods[2].g44_docs[3].doc_number | 00378 | CP | номер документа | |
| 05 | goods[2].g44_docs[3].doc_date | 20.01.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[4].doc_code | 04023 | CP | код документа | |
| 02 | goods[2].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[4].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[4].doc_number | 1 | CP | номер документа | |
| 05 | goods[2].g44_docs[4].doc_date | 13.01.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[5].doc_code | 04023 | CP | код документа | |
| 02 | goods[2].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[5].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[5].doc_number | 7 | CP | номер документа | |
| 05 | goods[2].g44_docs[5].doc_date | 28.11.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[6].doc_code | 04031 | CP | код документа | |
| 02 | goods[2].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[6].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[2].g44_docs[6].doc_number | 26-00378-tl | CP | номер документа | |
| 05 | goods[2].g44_docs[6].doc_date | 27.01.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[7].doc_code | 04111 | CP | код документа | |
| 02 | goods[2].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[7].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[2].g44_docs[7].doc_number | 26-00378-tl/1 | CP | номер документа | |
| 05 | goods[2].g44_docs[7].doc_date | 14.01.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[8].doc_code | 05999 | CP | код документа | |
| 02 | goods[2].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[8].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[8].doc_number | Б/Н | CO | номер документа | |
| 05 | goods[2].g44_docs[8].doc_date | 30.10.2025 | CO | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[9].doc_code | 03011 | CP | код документа | |
| 02 | goods[2].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[9].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[9].doc_number | LM-2553 | CP | номер документа | |
| 05 | goods[2].g44_docs[9].doc_date | 02.07.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[10].doc_code | 03012 | CP | код документа | |
| 02 | goods[2].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[10].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[10].doc_number | 1 | CP | номер документа | |
| 05 | goods[2].g44_docs[10].doc_date | 25.11.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[11].doc_code | 04011 | CP | код документа | |
| 02 | goods[2].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[11].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[2].g44_docs[11].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | |
| 05 | goods[2].g44_docs[11].doc_date | 14.07.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[12].doc_code | 11001 | CP | код документа | |
| 02 | goods[2].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[12].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[12].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[2].g44_docs[12].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[13].doc_code | 11004 | CP | код документа | |
| 02 | goods[2].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[13].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[2].g44_docs[13].doc_number | 1 | CP | номер документа | |
| 05 | goods[2].g44_docs[13].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[14].doc_code | 04033 | CP | код документа | |
| 02 | goods[2].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[14].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[14].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[2].g44_docs[14].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[15].doc_code | 09023 | CP | код документа | |
| 02 | goods[2].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[2].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[2].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[16].doc_code | 09999 | CP | код документа | |
| 02 | goods[2].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[16].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[2].g44_docs[16].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[2].g44_docs[16].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[17]
- _element_num: 17

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[17].doc_code | 09013 | CP | код документа | |
| 02 | goods[2].g44_docs[17].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[17].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | |
| 04 | goods[2].g44_docs[17].doc_number | 10719110/240126/5011363 | CO | номер документа | |
| 05 | goods[2].g44_docs[17].doc_date | 24.01.2026 | CO | дата документа | |

- _item_audit: 5

### 3.17 Теги после товаров и документов (графы 51–54)

#### 3.17.1 Графа 54 — уполномоченное лицо / представитель

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | representative.date | 30.05.2026 | D | дата заполнения/подачи | |
| 02 | representative.phone | +7 927-222-0500 | CP | телефон | |
| 03 | representative.email | A.K.ARBUZOVA@YANDEX.RU | CP | e-mail | |
| 04 | representative.last_name | АРБУЗОВА | CP | фамилия | |
| 05 | representative.first_name | АНАСТАСИЯ | CP | имя | |
| 06 | representative.middle_name | КОНСТАНТИНОВНА | CP | отчество | |
| 07 | representative.authority_doc_name | ДОВЕРЕННОСТЬ | CP | документ полномочий | |
| 08 | representative.authority_doc_number | 1 | CP | № документа полномочий | |
| 09 | representative.authority_doc_date_from | 01.02.2026 | CP | дата начала действия | |
| 10 | representative.authority_doc_date_to | 31.12.2026 | CP | дата окончания действия | |
| 11 | representative.position | УПОЛНОМОЧЕННОЕ ЛИЦО | CP | должность/статус | |
| 12 | representative.passport_code | RU01001 | CP | код документа личности | |
| 13 | representative.passport_name | ПАСПРФ | CP | наименование документа | |
| 14 | representative.passport_number | 449948 | CP | номер паспорта | |
| 15 | representative.passport_date | 11.03.2010 | CP | дата выдачи паспорта | |
| 16 | representative.passport_series | 63 09 | CP | серия паспорта | |
| 17 | representative.passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CP | кем выдан | |

- _audit: 17

### Итог:
- `dt_status`: confirmed

### Часть II: Issues (нерешенные вопросы)
- Нет.
