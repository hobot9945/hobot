# Исходные данные для ДТ

## Метаданные:
- `название кейса`: МоскитнаяСеткаWuqiang
- `путь к папке поставки`: alta\source\МоскитнаяСеткаWuqiang\01
- `тип поставки`: 1 ДТ/14 товаров
- `агрегация ДТ`: определяется правилами stage 2.0
- `источники данных:` primary.md + master_data.md + operator_provided_data.md

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
| 01 | sender.country_name | КИТАЙ | CP | текстовое название страны | |
| 02 | sender.country_code | CN | CP | код страны alpha-2 | |
| 03 | sender.name | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CP | полное наименование отправителя | |
| 04 | sender.region | Hebei | CP | область/район | |
| 05 | sender.city | Wuqiang, Hengshui | CP | город | |
| 06 | sender.street | Haozhuang Industrial Zone | CP | улица и дом | |

- _audit: 6

### 3.3 Количество товаров и мест (графы 5, 6)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.total_goods_number | 3 | D | количество товарных позиций в ДТ | |
| 02 | shipment.packages_flag | true | D | признак подсчёта мест | |
| 03 | shipment.total_packages | 206 | D | общее количество грузовых мест | |

- _audit: 3

### 3.4 Получатель (графа 8)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | consignee.same_as_declarant | true | D | признак «см. графу 14» | |
| 02 | consignee.ogrn | 1201600020390 | D | ОГРН получателя | |
| 03 | consignee.inn_kpp | 1650389298/165001001 | D | ИНН/КПП получателя | |
| 04 | consignee.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ \"СКИФ\" | D | наименование организации | |
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
| 04 | financial.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ \"СКИФ\" | D | наименование | |
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
| 02 | declarant.inn_kpp | 1650389298/165001001 | D | ИНН/КПП декларанта | |
| 03 | declarant.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ \"СКИФ\" | CP | наименование организации | |
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
| 02 | transport.identification | M869OM67/AM015667 | D | идентификация ТС | |
| 03 | transport.registration_country_code | RU | D | код страны регистрации ТС | |
| 04 | transport.container_flag | 0 | CO | признак контейнера | |
| 05 | transport.border_mode | 1 | D | код active-ТС на границе | |

- _audit: 5

### 3.11 Валюта и стоимость (графа 22)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.invoice_currency_numeric | 156 | D | цифровой код валюты | |
| 02 | shipment.invoice_currency_alpha | CNY | CP | буквенный код валюты | |
| 03 | shipment.invoice_amount | 87967.44 | CP | сумма по счёту | |

- _audit: 3

### 3.12 Характер сделки (графа 24)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.transaction_nature | 010 | D | характер сделки | |
| 02 | shipment.transaction_feature | 06 | D | особенность сделки | |

- _audit: 2

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
| 02 | customs.border_name | МАПП Забайкальск | CP | наименование таможенного поста | |

- _audit: 2

### 3.15 Местонахождение товаров (графа 30)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | location.document_number | 10404/141210/10092/5 | CP | номер документа СВХ | |

- _audit: 1

### 3.16 Массив: goods[]
- goods._array_audit: 3

#### 3.16.0 Элемент массива: goods[1]
- goods._element_num: 1

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g31.name | СЕТЧАТАЯ ТКАНЬ (ШТОРЫ) ИЗ ПОЛИЭСТЕРА (100% СИНТЕТИЧЕСКИЕ ВОЛОКНА) С ПЛИССИРОВКОЙ, ГОФРИРОВАННАЯ (СЕТКА-ГАРМОШКА), ДЛЯ ИСПОЛЬЗОВАНИЯ В МОСКИТНЫХ СИСТЕМАХ ОКН/ДВЕРЕЙ ДЛЯ ЗАЩИТЫ ОТ НАСЕКОМЫХ. МЕТОД ИЗГОТОВЛЕНИЯ: ПЛЕТЕНИЕ. РАЗМЕРЫ: 1.4*30 М, 1.6*30 М. ТМ: ОТСУТСТВУЕТ. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[1].g31.manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 03 | goods[1].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | |
| 04 | goods[1].places | 8 | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].item_no | 1 | D | номер товара | |
| 02 | goods[1].tnved_code | 6303921000 | D | код товара | |
| 03 | goods[1].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[1].tnved.flag_2 | N | D | доп. признак | |
| 05 | goods[1].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[1].gross_weight | 151.8 | D | вес брутто по товару | |
| 07 | goods[1].preference | ОООО-ОО | D | преференция | |
| 08 | goods[1].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[1].net_weight | 140.5 | D | вес нетто по товару | |
| 10 | goods[1].supplementary_quantity | 1800 | D | количество в дополнительной единице измерения | |
| 11 | goods[1].supplementary_unit_code | 055 | D | код дополнительной единицы измерения | |
| 12 | goods[1].supplementary_unit_name | м2 | D | наименование дополнительной единицы измерения | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].invoice_cost | 4185.00 | D | Цена товара | |
| 02 | goods[1].transport_cost | 59.58 | D | доля транспортных расходов до границы ЕАЭС по товару | |
| 03 | goods[1].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[1].insurance_cost | 37.97 | D | доля расходов на страхование по товару | |
| 05 | goods[1].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[1].txt[]
- _array_audit: 4

###### 3.16.5 Элемент массива: goods[1].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[1] | АРТ: - 420 м2 Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм. 1.4m x 30m. Черная | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[1].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[2] | АРТ: - 480 м2 Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм 1.6m x 30m. Черная | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[1].txt[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[3] | АРТ: - 420 м2 Европейская плиссированная сетка16 мм 1.4m x 30m . Черная | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[1].txt[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[4] | АРТ: - 480 м2 Европейская плиссированная сетка16 мм 1.6m x 30m. Черная | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[1].tovg[j]
- _array_audit: 4

###### 3.16.7 Элемент массива: goods[1].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[1].tovg[1].description | Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм. 1.4m x 30m. Черная | D | наименование | |
| 03 | goods[1].tovg[1].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[1].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[1].model | ПЛИССЕ 1.4*30 | D | модель/модификация | |
| 07 | goods[1].tovg[1].quantity | 420 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[1].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[1].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[1].gross_weight | 42 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[1].net_weight | 39 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[1].invoice_cost | 966.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[1].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[2].line_no | 2 | D | № строки таблицы | |
| 02 | goods[1].tovg[2].description | Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм 1.6m x 30m. Черная | D | наименование | |
| 03 | goods[1].tovg[2].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[1].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[2].model | ПЛИССЕ 1.6*30 | D | модель/модификация | |
| 07 | goods[1].tovg[2].quantity | 480 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[2].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[2].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[2].gross_weight | 36.4 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[2].net_weight | 34 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[2].invoice_cost | 1104.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[1].tovg[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[3].line_no | 3 | D | № строки таблицы | |
| 02 | goods[1].tovg[3].description | Европейская плиссированная сетка16 мм 1.4m x 30m . Черная | D | наименование | |
| 03 | goods[1].tovg[3].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[1].tovg[3].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[3].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[3].model | ЕВРОПЕЙСКАЯ ПЛИССЕ 1.4*30 | D | модель/модификация | |
| 07 | goods[1].tovg[3].quantity | 420 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[3].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[3].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[3].gross_weight | 39 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[3].net_weight | 36 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[3].invoice_cost | 987.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[1].tovg[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[4].line_no | 4 | D | № строки таблицы | |
| 02 | goods[1].tovg[4].description | Европейская плиссированная сетка16 мм 1.6m x 30m. Черная | D | наименование | |
| 03 | goods[1].tovg[4].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[1].tovg[4].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[4].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[4].model | ЕВРОПЕЙСКАЯ ПЛИССЕ 1.6*30 | D | модель/модификация | |
| 07 | goods[1].tovg[4].quantity | 480 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[4].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[4].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[4].gross_weight | 34.4 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[4].net_weight | 31.5 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[4].invoice_cost | 1128.00 | CP | цена по строке | |

- _item_audit: 12

##### 3.16.8 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[1].g44_docs[k]
- _array_audit: 15

###### 3.16.11 Элемент массива: goods[1].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[1].doc_code | 02015 | CP | код документа | |
| 02 | goods[1].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[1].doc_name | CMR | CP | наименование документа | |
| 04 | goods[1].g44_docs[1].doc_number | 09225 | CP | номер документа | |
| 05 | goods[1].g44_docs[1].doc_date | 27.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[2].doc_code | 03011 | CP | код документа | |
| 02 | goods[1].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[2].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[2].doc_number | 26HL-1103 | CP | номер документа | |
| 05 | goods[1].g44_docs[2].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[3].doc_code | 04011 | CP | код документа | |
| 02 | goods[1].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[3].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[1].g44_docs[3].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | |
| 05 | goods[1].g44_docs[3].doc_date | 14.07.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[4].doc_code | 04021 | CP | код документа | |
| 02 | goods[1].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[4].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[1].g44_docs[4].doc_number | 26HL-1103 | CP | номер документа | |
| 05 | goods[1].g44_docs[4].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[5].doc_code | 04023 | CP | код документа | |
| 02 | goods[1].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[5].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[5].doc_number | 5 | CP | номер документа | |
| 05 | goods[1].g44_docs[5].doc_date | 03.04.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[6].doc_code | 04031 | CP | код документа | |
| 02 | goods[1].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[6].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[6].doc_number | 26-09225-tl | CP | номер документа | |
| 05 | goods[1].g44_docs[6].doc_date | 12.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[7].doc_code | 04033 | CP | код документа | |
| 02 | goods[1].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[7].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[7].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[1].g44_docs[7].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[8].doc_code | 04111 | CP | код документа | |
| 02 | goods[1].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[8].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[8].doc_number | 26-09225-tl/1 | CP | номер документа | |
| 05 | goods[1].g44_docs[8].doc_date | 11.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[9].doc_code | 04131 | CP | код документа | |
| 02 | goods[1].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[9].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[9].doc_number | Б/Н | CO | номер документа | |
| 05 | goods[1].g44_docs[9].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[10].doc_code | 05999 | CP | код документа | |
| 02 | goods[1].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[10].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[10].doc_number | 31032026 | CP | номер документа | |
| 05 | goods[1].g44_docs[10].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[11].doc_code | 09013 | CP | код документа | |
| 02 | goods[1].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[11].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | |
| 04 | goods[1].g44_docs[11].doc_number | 10719110/300526/5086483 | CP | номер документа | |
| 05 | goods[1].g44_docs[11].doc_date | 30.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[12].doc_code | 09023 | CP | код документа | |
| 02 | goods[1].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[12].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[1].g44_docs[12].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[1].g44_docs[12].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[13].doc_code | 09999 | CP | код документа | |
| 02 | goods[1].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[13].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[1].g44_docs[13].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[1].g44_docs[13].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[14].doc_code | 11001 | CP | код документа | |
| 02 | goods[1].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[14].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[14].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[1].g44_docs[14].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[1].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[15].doc_code | 11004 | CP | код документа | |
| 02 | goods[1].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[15].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[1].g44_docs[15].doc_number | 1 | CP | номер документа | |
| 05 | goods[1].g44_docs[15].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[2]
- goods._element_num: 2

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g31.name | СЕТКА МОСКИТНАЯ РУЛОННАЯ ИЗ ПОЛИЭСТЕРА (100% ПОЛИЭФИРНЫЕ ВОЛОКНА ИЛИ С ПОКРЫТИЕМ ПВХ), ПЛЕТЕНАЯ, ДЛЯ ЗАЩИТЫ ОТ НАСЕКОМЫХ, ПЫЛИ, ПЫЛЬЦЫ, С ПОВЫШЕННОЙ ПРОЧНОСТЬЮ ОТ КОГТЕЙ ЖИВОТНЫХ (АНТИКОТ) ИЛИ МЕЛКОЙ ЯЧЕЙКОЙ (АНТИПЫЛЬ), ДЛЯ ОКН И ДВЕРЕЙ. РАЗМЕРЫ РУЛОНОВ: 1.4*30 М, 1.6*30 М, 1.6*50 М, 1.6*100 М. ТМ: ОТСУТСТВУЕТ. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[2].g31.manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 03 | goods[2].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | |
| 04 | goods[2].places | 197 | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].item_no | 2 | D | номер товара | |
| 02 | goods[2].tnved_code | 5804101000 | D | код товара | |
| 03 | goods[2].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[2].tnved.flag_2 | N | D | доп. признак | |
| 05 | goods[2].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[2].gross_weight | 3092.2 | D | вес брутто по товару | |
| 07 | goods[2].preference | ОООО-ОО | D | преференция | |
| 08 | goods[2].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[2].net_weight | 2870.5 | D | вес нетто по товару | |
| 10 | goods[2].supplementary_quantity | 12460 | D | количество в дополнительной единице измерения | |
| 11 | goods[2].supplementary_unit_code | 055 | D | код дополнительной единицы измерения | |
| 12 | goods[2].supplementary_unit_name | м2 | D | наименование дополнительной единицы измерения | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].invoice_cost | 78114.00 | D | Цена товара | |
| 02 | goods[2].transport_cost | 1213.64 | D | доля транспортных расходов до границы ЕАЭС по товару | |
| 03 | goods[2].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[2].insurance_cost | 708.65 | D | доля расходов на страхование по товару | |
| 05 | goods[2].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[2].txt[]
- _array_audit: 8

###### 3.16.5 Элемент массива: goods[2].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[1] | АРТ: - 1260 м2 Сетка от кошек 220 гр "Антикот" 1.4m x 30m. Серая | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[2].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[2] | АРТ: - 2400 м2 Сетка от кошек 320 гр "Антикот" 1.6m x 30m. Серая | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[2].txt[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[3] | АРТ: - 2100 м2 Сетка от кошек 320 гр "Антикот" 1.4 m x 30m. Серая | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[2].txt[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[4] | АРТ: - 1440 м2 Сетка от кошек 320 гр "Антикот" 1.6m x 30m. Черная | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[2].txt[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[5] | АРТ: - 1260 м2 Сетка от кошек 320 гр "Антикот" 1.4m x 30m. Черная | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[2].txt[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[6] | АРТ: - 800 м2 Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*50 М2 | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[2].txt[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[7] | АРТ: - 800 м2 Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*100 М2 | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[2].txt[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[8] | АРТ: - 2400 м2 Антипыльца 100 г черная 1,6 м*30 м | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[2].tovg[j]
- _array_audit: 8

###### 3.16.7 Элемент массива: goods[2].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[2].tovg[1].description | Сетка от кошек 220 гр "Антикот" 1.4m x 30m. Серая | D | наименование | |
| 03 | goods[2].tovg[1].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[2].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[2].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[2].tovg[1].model | АНТИКОТ 220Г 1.4*30 | D | модель/модификация | |
| 07 | goods[2].tovg[1].quantity | 1260 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[1].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[1].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[1].gross_weight | 303 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[1].net_weight | 277 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[1].invoice_cost | 5922.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[2].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[2].line_no | 2 | D | № строки таблицы | |
| 02 | goods[2].tovg[2].description | Сетка от кошек 320 гр "Антикот" 1.6m x 30m. Серая | D | наименование | |
| 03 | goods[2].tovg[2].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[2].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[2].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[2].tovg[2].model | АНТИКОТ 320Г 1.6*30 СЕРАЯ | D | модель/модификация | |
| 07 | goods[2].tovg[2].quantity | 2400 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[2].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[2].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[2].gross_weight | 800 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[2].net_weight | 768 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[2].invoice_cost | 13920.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[2].tovg[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[3].line_no | 3 | D | № строки таблицы | |
| 02 | goods[2].tovg[3].description | Сетка от кошек 320 гр "Антикот" 1.4 m x 30m. Серая | D | наименование | |
| 03 | goods[2].tovg[3].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[2].tovg[3].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[2].tovg[3].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[2].tovg[3].model | АНТИКОТ 320Г 1.4*30 СЕРАЯ | D | модель/модификация | |
| 07 | goods[2].tovg[3].quantity | 2100 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[3].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[3].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[3].gross_weight | 710 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[3].net_weight | 672 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[3].invoice_cost | 12180.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[2].tovg[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[4].line_no | 4 | D | № строки таблицы | |
| 02 | goods[2].tovg[4].description | Сетка от кошек 320 гр "Антикот" 1.6m x 30m. Черная | D | наименование | |
| 03 | goods[2].tovg[4].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[2].tovg[4].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[2].tovg[4].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[2].tovg[4].model | АНТИКОТ 320Г 1.6*30 ЧЕРНАЯ | D | модель/модификация | |
| 07 | goods[2].tovg[4].quantity | 1440 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[4].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[4].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[4].gross_weight | 480 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[4].net_weight | 461 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[4].invoice_cost | 8352.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[2].tovg[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[5].line_no | 5 | D | № строки таблицы | |
| 02 | goods[2].tovg[5].description | Сетка от кошек 320 гр "Антикот" 1.4m x 30m. Черная | D | наименование | |
| 03 | goods[2].tovg[5].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[2].tovg[5].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[2].tovg[5].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[2].tovg[5].model | АНТИКОТ 320Г 1.4*30 ЧЕРНАЯ | D | модель/модификация | |
| 07 | goods[2].tovg[5].quantity | 1260 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[5].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[5].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[5].gross_weight | 426 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[5].net_weight | 403 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[5].invoice_cost | 7308.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[2].tovg[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[6].line_no | 6 | D | № строки таблицы | |
| 02 | goods[2].tovg[6].description | Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*50 М2 | D | наименование | |
| 03 | goods[2].tovg[6].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[2].tovg[6].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[2].tovg[6].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[2].tovg[6].model | АНТИПЫЛЬ 30Г 1.6*50 | D | модель/модификация | |
| 07 | goods[2].tovg[6].quantity | 800 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[6].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[6].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[6].gross_weight | 45.46 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[6].net_weight | 33 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[6].invoice_cost | 7536.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[2].tovg[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[7].line_no | 7 | D | № строки таблицы | |
| 02 | goods[2].tovg[7].description | Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*100 М2 | D | наименование | |
| 03 | goods[2].tovg[7].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[2].tovg[7].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[2].tovg[7].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[2].tovg[7].model | АНТИПЫЛЬ 30Г 1.6*100 | D | модель/модификация | |
| 07 | goods[2].tovg[7].quantity | 800 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[7].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[7].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[7].gross_weight | 22.74 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[7].net_weight | 16.5 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[7].invoice_cost | 7536.00 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[2].tovg[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[8].line_no | 8 | D | № строки таблицы | |
| 02 | goods[2].tovg[8].description | Антипыльца 100 г черная 1,6 м*30 м | D | наименование | |
| 03 | goods[2].tovg[8].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[2].tovg[8].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[2].tovg[8].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[2].tovg[8].model | АНТИПЫЛЬЦА 1.6*30 | D | модель/модификация | |
| 07 | goods[2].tovg[8].quantity | 2400 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[8].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[8].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[8].gross_weight | 305 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[8].net_weight | 240 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[8].invoice_cost | 15360.00 | CP | цена по строке | |

- _item_audit: 12

##### 3.16.8 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[2].g44_docs[k]
- _array_audit: 17

###### 3.16.11 Элемент массива: goods[2].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[1].doc_code | 02015 | CP | код документа | |
| 02 | goods[2].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[1].doc_name | CMR | CP | наименование документа | |
| 04 | goods[2].g44_docs[1].doc_number | 09225 | CP | номер документа | |
| 05 | goods[2].g44_docs[1].doc_date | 27.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[2].doc_code | 03011 | CP | код документа | |
| 02 | goods[2].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[2].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[2].doc_number | 26HL-1103 | CP | номер документа | |
| 05 | goods[2].g44_docs[2].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[3].doc_code | 04011 | CP | код документа | |
| 02 | goods[2].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[3].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[2].g44_docs[3].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | |
| 05 | goods[2].g44_docs[3].doc_date | 14.07.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[4].doc_code | 04021 | CP | код документа | |
| 02 | goods[2].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[4].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[2].g44_docs[4].doc_number | 26HL-1103-A | CP | номер документа | |
| 05 | goods[2].g44_docs[4].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[5].doc_code | 04021 | CP | код документа | |
| 02 | goods[2].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[5].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[2].g44_docs[5].doc_number | 26HL-1103 | CP | номер документа | |
| 05 | goods[2].g44_docs[5].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[6].doc_code | 04023 | CP | код документа | |
| 02 | goods[2].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[6].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[6].doc_number | 5 | CP | номер документа | |
| 05 | goods[2].g44_docs[6].doc_date | 03.04.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[7].doc_code | 04023 | CP | код документа | |
| 02 | goods[2].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[7].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[7].doc_number | 6 | CP | номер документа | |
| 05 | goods[2].g44_docs[7].doc_date | 08.04.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[8].doc_code | 04031 | CP | код документа | |
| 02 | goods[2].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[8].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[2].g44_docs[8].doc_number | 26-09225-tl | CP | номер документа | |
| 05 | goods[2].g44_docs[8].doc_date | 12.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[9].doc_code | 04033 | CP | код документа | |
| 02 | goods[2].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[9].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[2].g44_docs[9].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[10].doc_code | 04111 | CP | код документа | |
| 02 | goods[2].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[10].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[2].g44_docs[10].doc_number | 26-09225-tl/1 | CP | номер документа | |
| 05 | goods[2].g44_docs[10].doc_date | 11.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[11].doc_code | 04131 | CP | код документа | |
| 02 | goods[2].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[11].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[11].doc_number | Б/Н | CO | номер документа | |
| 05 | goods[2].g44_docs[11].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[12].doc_code | 05999 | CP | код документа | |
| 02 | goods[2].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[12].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[12].doc_number | 31032026 | CP | номер документа | |
| 05 | goods[2].g44_docs[12].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[13].doc_code | 09013 | CP | код документа | |
| 02 | goods[2].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[13].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | |
| 04 | goods[2].g44_docs[13].doc_number | 10719110/300526/5086483 | CP | номер документа | |
| 05 | goods[2].g44_docs[13].doc_date | 30.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[14].doc_code | 09023 | CP | код документа | |
| 02 | goods[2].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[14].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[2].g44_docs[14].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[2].g44_docs[14].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[15].doc_code | 09999 | CP | код документа | |
| 02 | goods[2].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[2].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[2].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[16].doc_code | 11001 | CP | код документа | |
| 02 | goods[2].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[16].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[16].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[2].g44_docs[16].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[2].g44_docs[17]
- _element_num: 17

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[17].doc_code | 11004 | CP | код документа | |
| 02 | goods[2].g44_docs[17].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[17].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[2].g44_docs[17].doc_number | 1 | CP | номер документа | |
| 05 | goods[2].g44_docs[17].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[3]
- goods._element_num: 3

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g31.name | СЕТКА МОСКИТНАЯ МЕТАЛЛИЧЕСКАЯ ИЗ ПРОВОЛОКИ ИЗ НЕРЖАВЕЮЩЕЙ СТАЛИ 304 (ЖЕЛЕЗО/СТАЛЬ), ПЕРЕПЛЕТЕННАЯ, СВЕРХПРОЧНАЯ, ДЛЯ ОКН И ДВЕРЕЙ, ЗАЩИТА ОТ НАСЕКОМЫХ И ГРЫЗУНОВ. ТОЛЩИНА 0.23 ММ, РАЗМЕРЫ РУЛОНА: 1.6*30 М, 1.4*30 М. ТМ: ОТСУТСТВУЕТ. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[3].g31.manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 03 | goods[3].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | |
| 04 | goods[3].places | 1 | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].item_no | 3 | D | номер товара | |
| 02 | goods[3].tnved_code | 7314490000 | D | код товара | |
| 03 | goods[3].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[3].tnved.flag_2 | N | D | доп. признак | |
| 05 | goods[3].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[3].gross_weight | 216 | D | вес брутто по товару | |
| 07 | goods[3].preference | ОООО-ОО | D | преференция | |
| 08 | goods[3].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[3].net_weight | 189 | D | вес нетто по товару | |
| 10 | goods[3].supplementary_quantity | 564 | D | количество в дополнительной единице измерения | |
| 11 | goods[3].supplementary_unit_code | 055 | D | код дополнительной единицы измерения | |
| 12 | goods[3].supplementary_unit_name | м2 | D | наименование дополнительной единицы измерения | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].invoice_cost | 5668.44 | D | Цена товара | |
| 02 | goods[3].transport_cost | 84.78 | D | доля транспортных расходов до границы ЕАЭС по товару | |
| 03 | goods[3].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[3].insurance_cost | 51.42 | D | доля расходов на страхование по товару | |
| 05 | goods[3].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[3].txt[]
- _array_audit: 2

###### 3.16.5 Элемент массива: goods[3].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].txt[1] | АРТ: - 144 м2 сетка 18, размер ячейки 0,18 мм материал SS304. Размер рулона 1,6*30 М2 цвет оригинальный | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

###### 3.16.5 Элемент массива: goods[3].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].txt[2] | АРТ: - 420 м2 сетка17 мм материал SS304. Размер рулона 1,4*30 М2 Черный | D | графа 31 — TXT строка 1 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[3].tovg[j]
- _array_audit: 2

###### 3.16.7 Элемент массива: goods[3].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[3].tovg[1].description | сетка 18, размер ячейки 0,18 мм материал SS304. Размер рулона 1,6*30 М2 цвет оригинальный | D | наименование | |
| 03 | goods[3].tovg[1].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[3].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[3].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[3].tovg[1].model | НЕРЖАВЕЙКА 18 ЯЧЕЙКА 0.18ММ 1.6*30 | D | модель/модификация | |
| 07 | goods[3].tovg[1].quantity | 144 | CP | количество в доп.ед.изм | |
| 08 | goods[3].tovg[1].unit_code | 055 | D | код ЕИ | |
| 09 | goods[3].tovg[1].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[3].tovg[1].gross_weight | 51 | CP | вес брутто по строке | |
| 11 | goods[3].tovg[1].net_weight | 43 | CP | вес нетто по строке | |
| 12 | goods[3].tovg[1].invoice_cost | 1073.86 | CP | цена по строке | |

- _item_audit: 12

###### 3.16.7 Элемент массива: goods[3].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].tovg[2].line_no | 2 | D | № строки таблицы | |
| 02 | goods[3].tovg[2].description | сетка17 мм материал SS304. Размер рулона 1,4*30 М2 Черный | D | наименование | |
| 03 | goods[3].tovg[2].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | D | производитель | |
| 04 | goods[3].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[3].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[3].tovg[2].model | НЕРЖАВЕЙКА 20 ЯЧЕЙКА 0.17ММ 1.4*30 | D | модель/модификация | |
| 07 | goods[3].tovg[2].quantity | 420 | CP | количество в доп.ед.изм | |
| 08 | goods[3].tovg[2].unit_code | 055 | D | код ЕИ | |
| 09 | goods[3].tovg[2].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[3].tovg[2].gross_weight | 165 | CP | вес брутто по строке | |
| 11 | goods[3].tovg[2].net_weight | 146 | CP | вес нетто по строке | |
| 12 | goods[3].tovg[2].invoice_cost | 4594.58 | CP | цена по строке | |

- _item_audit: 12

##### 3.16.8 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[3].g44_docs[k]
- _array_audit: 15

###### 3.16.11 Элемент массива: goods[3].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[1].doc_code | 02015 | CP | код документа | |
| 02 | goods[3].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[1].doc_name | CMR | CP | наименование документа | |
| 04 | goods[3].g44_docs[1].doc_number | 09225 | CP | номер документа | |
| 05 | goods[3].g44_docs[1].doc_date | 27.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[2].doc_code | 03011 | CP | код документа | |
| 02 | goods[3].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[2].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[3].g44_docs[2].doc_number | 26HL-1103 | CP | номер документа | |
| 05 | goods[3].g44_docs[2].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[3].doc_code | 04011 | CP | код документа | |
| 02 | goods[3].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[3].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[3].g44_docs[3].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | |
| 05 | goods[3].g44_docs[3].doc_date | 14.07.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[4].doc_code | 04021 | CP | код документа | |
| 02 | goods[3].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[4].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[3].g44_docs[4].doc_number | 26HL-1103 | CP | номер документа | |
| 05 | goods[3].g44_docs[4].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[5].doc_code | 04023 | CP | код документа | |
| 02 | goods[3].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[5].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[5].doc_number | 5 | CP | номер документа | |
| 05 | goods[3].g44_docs[5].doc_date | 03.04.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[6].doc_code | 04031 | CP | код документа | |
| 02 | goods[3].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[6].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[3].g44_docs[6].doc_number | 26-09225-tl | CP | номер документа | |
| 05 | goods[3].g44_docs[6].doc_date | 12.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[7].doc_code | 04033 | CP | код документа | |
| 02 | goods[3].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[7].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[7].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[3].g44_docs[7].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[8].doc_code | 04111 | CP | код документа | |
| 02 | goods[3].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[8].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[3].g44_docs[8].doc_number | 26-09225-tl/1 | CP | номер документа | |
| 05 | goods[3].g44_docs[8].doc_date | 11.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[9].doc_code | 04131 | CP | код документа | |
| 02 | goods[3].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[9].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[3].g44_docs[9].doc_number | Б/Н | CO | номер документа | |
| 05 | goods[3].g44_docs[9].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[10].doc_code | 05999 | CP | код документа | |
| 02 | goods[3].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[10].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[10].doc_number | 31032026 | CP | номер документа | |
| 05 | goods[3].g44_docs[10].doc_date | 31.03.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[11].doc_code | 09013 | CP | код документа | |
| 02 | goods[3].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[11].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | |
| 04 | goods[3].g44_docs[11].doc_number | 10719110/300526/5086483 | CP | номер документа | |
| 05 | goods[3].g44_docs[11].doc_date | 30.05.2026 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[12].doc_code | 09023 | CP | код документа | |
| 02 | goods[3].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[12].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[3].g44_docs[12].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[3].g44_docs[12].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[13].doc_code | 09999 | CP | код документа | |
| 02 | goods[3].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[13].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[3].g44_docs[13].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[3].g44_docs[13].doc_date | 20.08.2025 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[14].doc_code | 11001 | CP | код документа | |
| 02 | goods[3].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[14].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[3].g44_docs[14].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[3].g44_docs[14].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

###### 3.16.11 Элемент массива: goods[3].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[15].doc_code | 11004 | CP | код документа | |
| 02 | goods[3].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[15].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[3].g44_docs[15].doc_number | 1 | CP | номер документа | |
| 05 | goods[3].g44_docs[15].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

### 3.17 Теги после товаров и документов (графы 51–54)

#### 3.17.1 Графа 54 — уполномоченное лицо / представитель

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | representative.date | 16.06.2026 | D | дата заполнения/подачи | |
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
| 12 | representative.passport_code | RU01001 | CO | код документа удостоверения личности | |
| 13 | representative.passport_name | ПАСПОРТ | CO | наименование документа | |
| 14 | representative.passport_number | 449948 | CP | номер паспорта | |
| 15 | representative.passport_date | 11.03.2010 | CP | дата выдачи паспорта | |
| 16 | representative.passport_series | 63 09 | CP | серия паспорта | |
| 17 | representative.passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CP | кем выдан | |

- _audit: 17

### Итог:
- `dt_status`: confirmed

### Часть II: Issues (нерешенные вопросы)

(Все вопросы успешно разрешены на базе предоставленных оператором ответов и кросс-дока)
