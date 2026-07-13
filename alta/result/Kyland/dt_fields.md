# Исходные данные для ДТ

## Метаданные:
- `название кейса`: Kyland
- `путь к папке поставки`: alta\source\Kyland\01
- `тип поставки`: 1 ДТ / 7 товаров
- `агрегация ДТ`: без группировки (ТН ВЭД 8517 в списке исключений Пост. 257)
- `источники данных`: primary.md + operator_provided_data.md + master_data.md

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
| 03 | sender.name | Kyland Technology Co., Ltd. | CP | полное наименование отправителя | |
| 04 | sender.region | Beijing | CP | область/район | |
| 05 | sender.city | Beijing | CP | город | |
| 06 | sender.street | Building No.2, Shixing Avenue 30#, Shijingshan District | CP | улица и дом | |

- _audit: 6

### 3.3 Количество товаров и мест (графы 5, 6)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.total_goods_number | 7 | D | количество товарных позиций в ДТ | |
| 02 | shipment.packages_flag | true | D | признак подсчета мест | |
| 03 | shipment.total_packages | 6 | D | общее количество грузовых мест | |

- _audit: 3

### 3.4 Получатель (графа 8)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | consignee.same_as_declarant | true | D | признак "см. графу 14" | |
| 02 | consignee.ogrn | 1087746277740 | CP | ОГРН получателя | |
| 03 | consignee.inn_kpp | 7720609470/772001001 | D | ИНН/КПП получателя | |
| 04 | consignee.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CP | наименование организации | |
| 05 | consignee.country_code | RU | CP | код страны получателя | |
| 06 | consignee.country_name | РОССИЯ | CP | страна получателя | |
| 07 | consignee.postcode | 111675 | CP | почтовый индекс | |
| 08 | consignee.region | ГОРОД МОСКВА | CP | регион получателя | |
| 09 | consignee.city | МОСКВА | CP | населенный пункт | |
| 10 | consignee.street | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CP | улица получателя | |
| 11 | consignee.building | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CP | дом получателя | |
| 12 | consignee.room | 103 | D | помещение/офис | |
| 13 | consignee.phone | +7495 981-62-44 | CP | телефон получателя | |
| 14 | consignee.email | info@symanitron.ru | CP | e-mail получателя | |

- _audit: 14

### 3.5 Финансовое урегулирование (графа 9)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | financial.same_as_declarant | true | D | признак "см. графу 14" | |
| 02 | financial.ogrn | 1087746277740 | CP | ОГРН | |
| 03 | financial.inn_kpp | 7720609470/772001001 | D | ИНН/КПП | |
| 04 | financial.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CP | наименование | |
| 05 | financial.country_code | RU | CP | код страны | |
| 06 | financial.country_name | РОССИЯ | CP | наименование страны | |
| 07 | financial.postcode | 111675 | CP | индекс | |
| 08 | financial.region | ГОРОД МОСКВА | CP | регион | |
| 09 | financial.city | МОСКВА | CP | город | |
| 10 | financial.street | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CP | улица | |
| 11 | financial.building | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CP | дом | |
| 12 | financial.room | 103 | D | помещение | |
| 13 | financial.country_code_alt | RU | CP | дублирующий код страны | |
| 14 | financial.phone | +7495 981-62-44 | CP | телефон | |
| 15 | financial.email | info@symanitron.ru | CP | e-mail | |

- _audit: 15

### 3.6 Торгующая страна (графа 11)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.trade_country_code | CN | CP | код торгующей страны alpha-2 | |

- _audit: 1

### 3.7 Декларант (графа 14)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | declarant.ogrn | 1087746277740 | CP | ОГРН декларанта | |
| 02 | declarant.inn_kpp | 7720609470/772001001 | D | ИНН/КПП декларанта | |
| 03 | declarant.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CP | наименование организации | |
| 04 | declarant.country_code | RU | CP | код страны | |
| 05 | declarant.country_name | РОССИЯ | CP | наименование страны | |
| 06 | declarant.postcode | 111675 | CP | почтовый индекс | |
| 07 | declarant.region | ГОРОД МОСКВА | CP | регион | |
| 08 | declarant.city | МОСКВА | CP | населенный пункт | |
| 09 | declarant.street | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CP | улица | |
| 10 | declarant.building | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CP | дом | |
| 11 | declarant.room | 103 | D | помещение/офис | |
| 12 | declarant.phone | +7495 981-62-44 | CP | телефон | |
| 13 | declarant.email | info@symanitron.ru | CP | e-mail | |

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
| 02 | delivery.place_name | Yichang | D | место поставки | |

- _audit: 2

### 3.10 Транспорт (графы 18, 19, 21)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.vehicles_count | | D | количество транспортных средств | |
| 02 | transport.identification | | D | идентификация ТС | |
| 03 | transport.registration_country_code | | D | код страны регистрации ТС | |
| 04 | transport.container_flag | 0 | CO | признак контейнера | |
| 05 | transport.border_mode | 1 | D | количество активных ТС на границе | |
| 06 | transport.border_id | 3U3721 | CP | номер авиарейса / идентификация ТС на границе | |
| 07 | transport.border_country_code | 00 | D | код страны регистрации ТС на границе | |

- _audit: 7

### 3.11 Валюта и стоимость (графа 22)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.invoice_currency_numeric | 840 | D | цифровой код валюты | |
| 02 | shipment.invoice_currency_alpha | USD | CP | буквенный код валюты | |
| 03 | shipment.invoice_amount | 25255.00 | CP | сумма по счету | |

- _audit: 3

### 3.12 Характер сделки (графа 24)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.transaction_nature | 010 | D | характер сделки | |
| 02 | shipment.transaction_feature | 00 | D | особенность сделки | |

- _audit: 2

### 3.13 Вид транспорта (графы 25, 26)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.border_transport_code | 40 | D | код вида транспорта на границе | |
| 02 | transport.internal_transport_code | 40 | D | код вида транспорта внутри страны | |

- _audit: 2

### 3.14 Таможня на границе (графа 29)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs.border_code | 10005020 | CP | код таможенного органа на границе | |
| 02 | customs.border_name | Т/П АЭРОПОРТ ШЕРЕМЕТЬЕВО (ГРУЗОВОЙ) | CP | наименование таможенного поста на границе | |

- _audit: 2

### 3.15 Местонахождение товаров (графа 30)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | location.document_number | 10005/060917/10048/4 | CP | номер документа СВХ | |

- _audit: 01

### 3.16 Массив: goods[]
- goods._array_audit: 7

#### 3.16.0 Элемент массива: goods[1]
- goods._element_num: 1

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g31.name | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-4GX16GE-L2-L2. Предназначен для проводной передачи и коммутации пакетов данных в промышленных сетях. Порты: 16 RJ45 10/100/1000Base-TX, 4 SFP 100Base-X. Металлический корпус IP40, крепление на DIN-рейку. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[1].g31.manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 03 | goods[1].g31.trade_mark | Kyland | D | товарный знак / ТМ | |
| 04 | goods[1].places | 2 | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].item_no | 1 | D | номер товара | |
| 02 | goods[1].tnved_code | 8517620003 | D | код товара | |
| 03 | goods[1].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[1].tnved.flag_2 | | D | доп. признак | |
| 05 | goods[1].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[1].gross_weight | 32.884 | D | вес брутто по товару | |
| 07 | goods[1].preference | ОООО-ОО | D | преференция | |
| 08 | goods[1].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[1].net_weight | 28.5 | D | вес нетто по товару | |
| 10 | goods[1].supplementary_quantity | 19 | D | количество в доп. единице | |
| 11 | goods[1].supplementary_unit_code | 796 | D | код доп. единицы | |
| 12 | goods[1].supplementary_unit_name | шт | D | наименование доп. единицы | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].invoice_cost | 12977.00 | D | Цена товара | |
| 02 | goods[1].transport_cost | 590.35 | D | доля транспортных расходов | |
| 03 | goods[1].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[1].insurance_cost | 2664.12 | D | доля расходов на страхование | |
| 05 | goods[1].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[1].txt[]
- _array_audit: 1

##### 3.16.5 Элемент массива: goods[1].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[1] | АРТ: - 19 шт Промышленный управляемый коммутатор Ethernet, модель SYM3000A-4GX16GE-L2-L2. Предназначен для проводной передачи и коммутации пакетов данных в промышленных сетях. Порты: 16 RJ45 10/100/1000Base-TX, 4 SFP 100Base-X. Металлический корпус IP40, крепление на DIN-рейку. | D | строка дополнения графы 31 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[1].tovg[]
- _array_audit: 1

##### 3.16.7 Элемент массива: goods[1].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[1].tovg[1].description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-4GX16GE-L2-L2. Предназначен для проводной передачи и коммутации пакетов данных в промышленных сетях. Порты: 16 RJ45 10/100/1000Base-TX, 4 SFP 100Base-X. Металлический корпус IP40, крепление на DIN-рейку. | D | наименование | |
| 03 | goods[1].tovg[1].manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 04 | goods[1].tovg[1].trade_mark | Kyland | D | марка/ТМ | |
| 05 | goods[1].tovg[1].goods_mark | Kyland | D | товарный знак | |
| 06 | goods[1].tovg[1].model | SYM3000A-4GX16GE-L2-L2 | D | модель/модификация | |
| 07 | goods[1].tovg[1].quantity | 19 | D | количество в доп.ед | |
| 08 | goods[1].tovg[1].unit_code | 796 | D | код ЕИ | |
| 09 | goods[1].tovg[1].unit_name | шт | D | наименование ЕИ | |
| 10 | goods[1].tovg[1].gross_weight | 32.884 | D | вес брутто по строке | |
| 11 | goods[1].tovg[1].net_weight | 28.5 | D | вес нетто по строке | |
| 12 | goods[1].tovg[1].invoice_cost | 12977.00 | D | цена по строке | |

- _item_audit: 12

##### 3.16.9 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[1].g44_docs[]
- _array_audit: 16

##### 3.16.11 Элемент массива: goods[1].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[1].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[1].doc_number | Im191018/Kyl | CP | номер документа | |
| 05 | goods[1].g44_docs[1].doc_date | 19.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[2].doc_code | 03012 | CP | код документа | |
| 02 | goods[1].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[2].doc_number | 221211 | CP | номер документа | |
| 05 | goods[1].g44_docs[2].doc_date | 11.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[3].doc_code | 03031 | CP | код документа | |
| 02 | goods[1].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[3].doc_name | УНК | CP | наименование документа | |
| 04 | goods[1].g44_docs[3].doc_number | 18100214110000097211 | CP | номер документа | |
| 05 | goods[1].g44_docs[3].doc_date | 25.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[4].doc_code | 04011 | CP | код документа | |
| 02 | goods[1].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[4].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[1].g44_docs[4].doc_number | ЮЭ9965-19-16744108 | CP | номер документа | |
| 05 | goods[1].g44_docs[4].doc_date | 14.02.2019 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[5].doc_code | 04021 | CP | код документа | |
| 02 | goods[1].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[5].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[1].g44_docs[5].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[1].g44_docs[5].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[6].doc_code | 04131 | CP | код документа | |
| 02 | goods[1].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[6].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[6].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[1].g44_docs[6].doc_date | 23.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[7].doc_code | 02017 | CP | код документа | |
| 02 | goods[1].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[7].doc_name | АВИАНАКЛАДНАЯ | CP | наименование документа | |
| 04 | goods[1].g44_docs[7].doc_number | 876-41176586 | CP | номер документа | |
| 05 | goods[1].g44_docs[7].doc_date | 02.01.2023 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[8].doc_code | 04023 | CP | код документа | |
| 02 | goods[1].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[8].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[8].doc_number | 30 | CP | номер документа | |
| 05 | goods[1].g44_docs[8].doc_date | 15.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[9].doc_code | 04033 | CP | код документа | |
| 02 | goods[1].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[9].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[1].g44_docs[9].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[10].doc_code | 04031 | CP | код документа | |
| 02 | goods[1].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[10].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[10].doc_number | VIG2227802 | CP | номер документа | |
| 05 | goods[1].g44_docs[10].doc_date | 28.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[11].doc_code | 04111 | CP | код документа | |
| 02 | goods[1].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[11].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[11].doc_number | VIG2227611 | CP | номер документа | |
| 05 | goods[1].g44_docs[11].doc_date | 27.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[12].doc_code | 01402 | CP | код документа | |
| 02 | goods[1].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[12].doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CP | наименование документа | |
| 04 | goods[1].g44_docs[12].doc_number | РА01.В.65848/21 | CP | номер документа | |
| 05 | goods[1].g44_docs[12].doc_date | 14.05.2021 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[13].doc_code | 10052 | CP | код документа | |
| 02 | goods[1].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[13].doc_name | НОТИФИКАЦИЯ | CP | наименование документа | |
| 04 | goods[1].g44_docs[13].doc_number | RU0000049917 | CP | номер документа | |
| 05 | goods[1].g44_docs[13].doc_date | 25.09.2020 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[14].doc_code | 05999 | CP | код документа | |
| 02 | goods[1].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[14].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[14].doc_number | БН | CP | номер документа | |
| 05 | goods[1].g44_docs[14].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[15].doc_code | 11004 | CP | код документа | |
| 02 | goods[1].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[15].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[1].g44_docs[15].doc_number | 1 | CP | номер документа | |
| 05 | goods[1].g44_docs[15].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[16].doc_code | 11001 | CP | код документа | |
| 02 | goods[1].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[16].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[16].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[1].g44_docs[16].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[2]
- goods._element_num: 2

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g31.name | Промышленный коммутатор Ethernet с пониженным энергопотреблением, модель SYM3000A-LITE-2GX8T-L3-L3. Предназначен для проводной передачи пакетов данных. Порты: 8 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[2].g31.manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 03 | goods[2].g31.trade_mark | Kyland | D | товарный знак / ТМ | |
| 04 | goods[2].places | 2 | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].item_no | 2 | D | номер товара | |
| 02 | goods[2].tnved_code | 8517620003 | D | код товара | |
| 03 | goods[2].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[2].tnved.flag_2 | | D | доп. признак | |
| 05 | goods[2].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[2].gross_weight | 25.962 | D | вес брутто по товару | |
| 07 | goods[2].preference | ОООО-ОО | D | преференция | |
| 08 | goods[2].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[2].net_weight | 22.5 | D | вес нетто по товару | |
| 10 | goods[2].supplementary_quantity | 15 | D | количество в доп. единице | |
| 11 | goods[2].supplementary_unit_code | 796 | D | код доп. единицы | |
| 12 | goods[2].supplementary_unit_name | шт | D | наименование доп. единицы | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].invoice_cost | 5205.00 | D | Цена товара | |
| 02 | goods[2].transport_cost | 465.73 | D | доля транспортных расходов | |
| 03 | goods[2].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[2].insurance_cost | 1068.79 | D | доля расходов на страхование | |
| 05 | goods[2].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[2].txt[]
- _array_audit: 1

##### 3.16.5 Элемент массива: goods[2].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[1] | АРТ: - 15 шт Промышленный коммутатор Ethernet с пониженным энергопотреблением, модель SYM3000A-LITE-2GX8T-L3-L3. Предназначен для проводной передачи пакетов данных. Порты: 8 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. | D | строка дополнения графы 31 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[2].tovg[]
- _array_audit: 1

##### 3.16.7 Элемент массива: goods[2].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[2].tovg[1].description | Промышленный коммутатор Ethernet с пониженным энергопотреблением, модель SYM3000A-LITE-2GX8T-L3-L3. Предназначен для проводной передачи пакетов данных. Порты: 8 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. | D | наименование | |
| 03 | goods[2].tovg[1].manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 04 | goods[2].tovg[1].trade_mark | Kyland | D | марка/ТМ | |
| 05 | goods[2].tovg[1].goods_mark | Kyland | D | товарный знак | |
| 06 | goods[2].tovg[1].model | SYM3000A-LITE-2GX8T-L3-L3 | D | модель/модификация | |
| 07 | goods[2].tovg[1].quantity | 15 | D | количество в доп.ед | |
| 08 | goods[2].tovg[1].unit_code | 796 | D | код ЕИ | |
| 09 | goods[2].tovg[1].unit_name | шт | D | наименование ЕИ | |
| 10 | goods[2].tovg[1].gross_weight | 25.962 | D | вес брутто по строке | |
| 11 | goods[2].tovg[1].net_weight | 22.5 | D | вес нетто по строке | |
| 12 | goods[2].tovg[1].invoice_cost | 5205.00 | D | цена по строке | |

- _item_audit: 12

##### 3.16.9 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[2].g44_docs[]
- _array_audit: 16

##### 3.16.11 Элемент массива: goods[2].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[2].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[1].doc_number | Im191018/Kyl | CP | номер документа | |
| 05 | goods[2].g44_docs[1].doc_date | 19.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[2].doc_code | 03012 | CP | код документа | |
| 02 | goods[2].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[2].doc_number | 221211 | CP | номер документа | |
| 05 | goods[2].g44_docs[2].doc_date | 11.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[3].doc_code | 03031 | CP | код документа | |
| 02 | goods[2].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[3].doc_name | УНК | CP | наименование документа | |
| 04 | goods[2].g44_docs[3].doc_number | 18100214110000097211 | CP | номер документа | |
| 05 | goods[2].g44_docs[3].doc_date | 25.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[4].doc_code | 04011 | CP | код документа | |
| 02 | goods[2].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[4].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[2].g44_docs[4].doc_number | ЮЭ9965-19-16744108 | CP | номер документа | |
| 05 | goods[2].g44_docs[4].doc_date | 14.02.2019 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[5].doc_code | 04021 | CP | код документа | |
| 02 | goods[2].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[5].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[2].g44_docs[5].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[2].g44_docs[5].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[6].doc_code | 04131 | CP | код документа | |
| 02 | goods[2].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[6].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[6].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[2].g44_docs[6].doc_date | 23.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[7].doc_code | 02017 | CP | код документа | |
| 02 | goods[2].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[7].doc_name | АВИАНАКЛАДНАЯ | CP | наименование документа | |
| 04 | goods[2].g44_docs[7].doc_number | 876-41176586 | CP | номер документа | |
| 05 | goods[2].g44_docs[7].doc_date | 02.01.2023 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[8].doc_code | 04023 | CP | код документа | |
| 02 | goods[2].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[8].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[8].doc_number | 30 | CP | номер документа | |
| 05 | goods[2].g44_docs[8].doc_date | 15.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[9].doc_code | 04033 | CP | код документа | |
| 02 | goods[2].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[9].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[2].g44_docs[9].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[10].doc_code | 04031 | CP | код документа | |
| 02 | goods[2].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[10].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[2].g44_docs[10].doc_number | VIG2227802 | CP | номер документа | |
| 05 | goods[2].g44_docs[10].doc_date | 28.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[11].doc_code | 04111 | CP | код документа | |
| 02 | goods[2].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[11].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[2].g44_docs[11].doc_number | VIG2227611 | CP | номер документа | |
| 05 | goods[2].g44_docs[11].doc_date | 27.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[12].doc_code | 01402 | CP | код документа | |
| 02 | goods[2].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[12].doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CP | наименование документа | |
| 04 | goods[2].g44_docs[12].doc_number | РА03.В.59715/21 | CP | номер документа | |
| 05 | goods[2].g44_docs[12].doc_date | 17.12.2021 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[13].doc_code | 10052 | CP | код документа | |
| 02 | goods[2].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[13].doc_name | НОТИФИКАЦИЯ | CP | наименование документа | |
| 04 | goods[2].g44_docs[13].doc_number | RU0000055090 | CP | номер документа | |
| 05 | goods[2].g44_docs[13].doc_date | 13.12.2021 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[14].doc_code | 05999 | CP | код документа | |
| 02 | goods[2].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[14].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[14].doc_number | БН | CP | номер документа | |
| 05 | goods[2].g44_docs[14].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[15].doc_code | 11004 | CP | код документа | |
| 02 | goods[2].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[15].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[2].g44_docs[15].doc_number | 1 | CP | номер документа | |
| 05 | goods[2].g44_docs[15].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[16].doc_code | 11001 | CP | код документа | |
| 02 | goods[2].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[16].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[16].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[2].g44_docs[16].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[3]
- goods._element_num: 3

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g31.name | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-4SFP8T-L2-L2. Предназначен для передачи данных в локальных сетях. Порты: 8 RJ45 10/100Base-TX, 4 SFP 100Base-X. Металлический корпус, крепление на DIN-рейку. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[3].g31.manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 03 | goods[3].g31.trade_mark | Kyland | D | товарный знак / ТМ | |
| 04 | goods[3].places | | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].item_no | 3 | D | номер товара | |
| 02 | goods[3].tnved_code | 8517620003 | D | код товара | |
| 03 | goods[3].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[3].tnved.flag_2 | | D | доп. признак | |
| 05 | goods[3].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[3].gross_weight | 8.654 | D | вес брутто по товару | |
| 07 | goods[3].preference | ОООО-ОО | D | преференция | |
| 08 | goods[3].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[3].net_weight | 7.5 | D | вес нетто по товару | |
| 10 | goods[3].supplementary_quantity | 5 | D | количество в доп. единице | |
| 11 | goods[3].supplementary_unit_code | 796 | D | код доп. единицы | |
| 12 | goods[3].supplementary_unit_name | шт | D | наименование доп. единицы | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].invoice_cost | 1735.00 | D | Цена товара | |
| 02 | goods[3].transport_cost | 155.29 | D | доля транспортных расходов | |
| 03 | goods[3].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[3].insurance_cost | 356.24 | D | доля расходов на страхование | |
| 05 | goods[3].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[3].txt[]
- _array_audit: 1

##### 3.16.5 Элемент массива: goods[3].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].txt[1] | АРТ: - 5 шт Промышленный управляемый коммутатор Ethernet, модель SYM3000A-4SFP8T-L2-L2. Предназначен для передачи данных в локальных сетях. Порты: 8 RJ45 10/100Base-TX, 4 SFP 100Base-X. Металлический корпус, крепление на DIN-рейку. | D | строка дополнения графы 31 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[3].tovg[]
- _array_audit: 1

##### 3.16.7 Элемент массива: goods[3].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[3].tovg[1].description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-4SFP8T-L2-L2. Предназначен для передачи данных в локальных сетях. Порты: 8 RJ45 10/100Base-TX, 4 SFP 100Base-X. Металлический корпус, крепление на DIN-рейку. | D | наименование | |
| 03 | goods[3].tovg[1].manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 04 | goods[3].tovg[1].trade_mark | Kyland | D | марка/ТМ | |
| 05 | goods[3].tovg[1].goods_mark | Kyland | D | товарный знак | |
| 06 | goods[3].tovg[1].model | SYM3000A-4SFP8T-L2-L2 | D | модель/модификация | |
| 07 | goods[3].tovg[1].quantity | 5 | D | количество в доп.ед | |
| 08 | goods[3].tovg[1].unit_code | 796 | D | код ЕИ | |
| 09 | goods[3].tovg[1].unit_name | шт | D | наименование ЕИ | |
| 10 | goods[3].tovg[1].gross_weight | 8.654 | D | вес брутто по строке | |
| 11 | goods[3].tovg[1].net_weight | 7.5 | D | вес нетто по строке | |
| 12 | goods[3].tovg[1].invoice_cost | 1735.00 | D | цена по строке | |

- _item_audit: 12

##### 3.16.9 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[3].g44_docs[]
- _array_audit: 16

##### 3.16.11 Элемент массива: goods[3].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[3].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[3].g44_docs[1].doc_number | Im191018/Kyl | CP | номер документа | |
| 05 | goods[3].g44_docs[1].doc_date | 19.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[2].doc_code | 03012 | CP | код документа | |
| 02 | goods[3].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[2].doc_number | 221211 | CP | номер документа | |
| 05 | goods[3].g44_docs[2].doc_date | 11.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[3].doc_code | 03031 | CP | код документа | |
| 02 | goods[3].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[3].doc_name | УНК | CP | наименование документа | |
| 04 | goods[3].g44_docs[3].doc_number | 18100214110000097211 | CP | номер документа | |
| 05 | goods[3].g44_docs[3].doc_date | 25.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[4].doc_code | 04011 | CP | код документа | |
| 02 | goods[3].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[4].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[3].g44_docs[4].doc_number | ЮЭ9965-19-16744108 | CP | номер документа | |
| 05 | goods[3].g44_docs[4].doc_date | 14.02.2019 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[5].doc_code | 04021 | CP | код документа | |
| 02 | goods[3].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[5].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[3].g44_docs[5].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[3].g44_docs[5].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[6].doc_code | 04131 | CP | код документа | |
| 02 | goods[3].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[6].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[3].g44_docs[6].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[3].g44_docs[6].doc_date | 23.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[7].doc_code | 02017 | CP | код документа | |
| 02 | goods[3].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[7].doc_name | АВИАНАКЛАДНАЯ | CP | наименование документа | |
| 04 | goods[3].g44_docs[7].doc_number | 876-41176586 | CP | номер документа | |
| 05 | goods[3].g44_docs[7].doc_date | 02.01.2023 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[8].doc_code | 04023 | CP | код документа | |
| 02 | goods[3].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[8].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[8].doc_number | 30 | CP | номер документа | |
| 05 | goods[3].g44_docs[8].doc_date | 15.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[9].doc_code | 04033 | CP | код документа | |
| 02 | goods[3].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[9].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[3].g44_docs[9].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[10].doc_code | 04031 | CP | код документа | |
| 02 | goods[3].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[10].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[3].g44_docs[10].doc_number | VIG2227802 | CP | номер документа | |
| 05 | goods[3].g44_docs[10].doc_date | 28.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[11].doc_code | 04111 | CP | код документа | |
| 02 | goods[3].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[11].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[3].g44_docs[11].doc_number | VIG2227611 | CP | номер документа | |
| 05 | goods[3].g44_docs[11].doc_date | 27.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[12].doc_code | 01402 | CP | код документа | |
| 02 | goods[3].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[12].doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CP | наименование документа | |
| 04 | goods[3].g44_docs[12].doc_number | РА01.В.65848/21 | CP | номер документа | |
| 05 | goods[3].g44_docs[12].doc_date | 14.05.2021 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[13].doc_code | 10052 | CP | код документа | |
| 02 | goods[3].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[13].doc_name | НОТИФИКАЦИЯ | CP | наименование документа | |
| 04 | goods[3].g44_docs[13].doc_number | RU0000049917 | CP | номер документа | |
| 05 | goods[3].g44_docs[13].doc_date | 25.09.2020 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[14].doc_code | 05999 | CP | код документа | |
| 02 | goods[3].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[14].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[14].doc_number | БН | CP | номер документа | |
| 05 | goods[3].g44_docs[14].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[15].doc_code | 11004 | CP | код документа | |
| 02 | goods[3].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[15].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[3].g44_docs[15].doc_number | 1 | CP | номер документа | |
| 05 | goods[3].g44_docs[15].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[16].doc_code | 11001 | CP | код документа | |
| 02 | goods[3].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[16].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[3].g44_docs[16].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[3].g44_docs[16].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[4]
- goods._element_num: 4

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g31.name | Промышленный коммутатор Ethernet с пониженным энергопотреблением, модель SYM3000A-LITE-8T-L3-L3. Предназначен для проводной передачи данных. Порты: 8 10/100Base-TX. Металлический корпус, крепление на DIN-рейку. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[4].g31.manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 03 | goods[4].g31.trade_mark | Kyland | D | товарный знак / ТМ | |
| 04 | goods[4].places | | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].item_no | 4 | D | номер товара | |
| 02 | goods[4].tnved_code | 8517620003 | D | код товара | |
| 03 | goods[4].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[4].tnved.flag_2 | | D | доп. признак | |
| 05 | goods[4].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[4].gross_weight | 1.731 | D | вес брутто по товару | |
| 07 | goods[4].preference | ОООО-ОО | D | преференция | |
| 08 | goods[4].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[4].net_weight | 1.5 | D | вес нетто по товару | |
| 10 | goods[4].supplementary_quantity | 1 | D | количество в доп. единице | |
| 11 | goods[4].supplementary_unit_code | 796 | D | код доп. единицы | |
| 12 | goods[4].supplementary_unit_name | шт | D | наименование доп. единицы | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].invoice_cost | 215.00 | D | Цена товара | |
| 02 | goods[4].transport_cost | 31.04 | D | доля транспортных расходов | |
| 03 | goods[4].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[4].insurance_cost | 44.15 | D | доля расходов на страхование | |
| 05 | goods[4].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[4].txt[]
- _array_audit: 1

##### 3.16.5 Элемент массива: goods[4].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].txt[1] | АРТ: - 1 шт Промышленный коммутатор Ethernet с пониженным энергопотреблением, модель SYM3000A-LITE-8T-L3-L3. Предназначен для проводной передачи данных. Порты: 8 10/100Base-TX. Металлический корпус, крепление на DIN-рейку. | D | строка дополнения графы 31 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[4].tovg[]
- _array_audit: 1

##### 3.16.7 Элемент массива: goods[4].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[4].tovg[1].description | Промышленный коммутатор Ethernet с пониженным энергопотреблением, модель SYM3000A-LITE-8T-L3-L3. Предназначен для проводной передачи данных. Порты: 8 10/100Base-TX. Металлический корпус, крепление на DIN-рейку. | D | наименование | |
| 03 | goods[4].tovg[1].manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 04 | goods[4].tovg[1].trade_mark | Kyland | D | марка/ТМ | |
| 05 | goods[4].tovg[1].goods_mark | Kyland | D | товарный знак | |
| 06 | goods[4].tovg[1].model | SYM3000A-LITE-8T-L3-L3 | D | модель/модификация | |
| 07 | goods[4].tovg[1].quantity | 1 | D | количество в доп.ед | |
| 08 | goods[4].tovg[1].unit_code | 796 | D | код ЕИ | |
| 09 | goods[4].tovg[1].unit_name | шт | D | наименование ЕИ | |
| 10 | goods[4].tovg[1].gross_weight | 1.731 | D | вес брутто по строке | |
| 11 | goods[4].tovg[1].net_weight | 1.5 | D | вес нетто по строке | |
| 12 | goods[4].tovg[1].invoice_cost | 215.00 | D | цена по строке | |

- _item_audit: 12

##### 3.16.9 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[4].g44_docs[]
- _array_audit: 16

##### 3.16.11 Элемент массива: goods[4].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[4].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[4].g44_docs[1].doc_number | Im191018/Kyl | CP | номер документа | |
| 05 | goods[4].g44_docs[1].doc_date | 19.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[2].doc_code | 03012 | CP | код документа | |
| 02 | goods[4].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[4].g44_docs[2].doc_number | 221211 | CP | номер документа | |
| 05 | goods[4].g44_docs[2].doc_date | 11.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[3].doc_code | 03031 | CP | код документа | |
| 02 | goods[4].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[3].doc_name | УНК | CP | наименование документа | |
| 04 | goods[4].g44_docs[3].doc_number | 18100214110000097211 | CP | номер документа | |
| 05 | goods[4].g44_docs[3].doc_date | 25.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[4].doc_code | 04011 | CP | код документа | |
| 02 | goods[4].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[4].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[4].g44_docs[4].doc_number | ЮЭ9965-19-16744108 | CP | номер документа | |
| 05 | goods[4].g44_docs[4].doc_date | 14.02.2019 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[5].doc_code | 04021 | CP | код документа | |
| 02 | goods[4].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[5].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[4].g44_docs[5].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[4].g44_docs[5].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[6].doc_code | 04131 | CP | код документа | |
| 02 | goods[4].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[6].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[4].g44_docs[6].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[4].g44_docs[6].doc_date | 23.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[7].doc_code | 02017 | CP | код документа | |
| 02 | goods[4].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[7].doc_name | АВИАНАКЛАДНАЯ | CP | наименование документа | |
| 04 | goods[4].g44_docs[7].doc_number | 876-41176586 | CP | номер документа | |
| 05 | goods[4].g44_docs[7].doc_date | 02.01.2023 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[8].doc_code | 04023 | CP | код документа | |
| 02 | goods[4].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[8].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[4].g44_docs[8].doc_number | 30 | CP | номер документа | |
| 05 | goods[4].g44_docs[8].doc_date | 15.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[9].doc_code | 04033 | CP | код документа | |
| 02 | goods[4].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[4].g44_docs[9].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[4].g44_docs[9].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[10].doc_code | 04031 | CP | код документа | |
| 02 | goods[4].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[10].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[4].g44_docs[10].doc_number | VIG2227802 | CP | номер документа | |
| 05 | goods[4].g44_docs[10].doc_date | 28.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[11].doc_code | 04111 | CP | код документа | |
| 02 | goods[4].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[11].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[4].g44_docs[11].doc_number | VIG2227611 | CP | номер документа | |
| 05 | goods[4].g44_docs[11].doc_date | 27.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[12].doc_code | 01402 | CP | код документа | |
| 02 | goods[4].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[12].doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CP | наименование документа | |
| 04 | goods[4].g44_docs[12].doc_number | РА03.В.59715/21 | CP | номер документа | |
| 05 | goods[4].g44_docs[12].doc_date | 17.12.2021 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[13].doc_code | 10052 | CP | код документа | |
| 02 | goods[4].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[13].doc_name | НОТИФИКАЦИЯ | CP | наименование документа | |
| 04 | goods[4].g44_docs[13].doc_number | RU0000055090 | CP | номер документа | |
| 05 | goods[4].g44_docs[13].doc_date | 13.12.2021 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[14].doc_code | 05999 | CP | код документа | |
| 02 | goods[4].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[14].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[4].g44_docs[14].doc_number | БН | CP | номер документа | |
| 05 | goods[4].g44_docs[14].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[15].doc_code | 11004 | CP | код документа | |
| 02 | goods[4].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[15].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[4].g44_docs[15].doc_number | 1 | CP | номер документа | |
| 05 | goods[4].g44_docs[15].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[4].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[4].g44_docs[16].doc_code | 11001 | CP | код документа | |
| 02 | goods[4].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[4].g44_docs[16].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[4].g44_docs[16].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[4].g44_docs[16].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[5]
- goods._element_num: 5

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g31.name | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX16GE-L2-L2. Предназначен для коммутации пакетов данных. Порты: 16 RJ45 10/100/1000Base-TX, 2 SFP 100/1000Base-X. Металлический корпус IP40, крепление на DIN-рейку. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[5].g31.manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 03 | goods[5].g31.trade_mark | Kyland | D | товарный знак / ТМ | |
| 04 | goods[5].places | | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].item_no | 5 | D | номер товара | |
| 02 | goods[5].tnved_code | 8517620003 | D | код товара | |
| 03 | goods[5].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[5].tnved.flag_2 | | D | доп. признак | |
| 05 | goods[5].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[5].gross_weight | 1.731 | D | вес брутто по товару | |
| 07 | goods[5].preference | ОООО-ОО | D | преференция | |
| 08 | goods[5].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[5].net_weight | 1.5 | D | вес нетто по товару | |
| 10 | goods[5].supplementary_quantity | 1 | D | количество в доп. единице | |
| 11 | goods[5].supplementary_unit_code | 796 | D | код доп. единицы | |
| 12 | goods[5].supplementary_unit_name | шт | D | наименование доп. единицы | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].invoice_cost | 604.00 | D | Цена товара | |
| 02 | goods[5].transport_cost | 31.04 | D | доля транспортных расходов | |
| 03 | goods[5].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[5].insurance_cost | 124.02 | D | доля расходов на страхование | |
| 05 | goods[5].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[5].txt[]
- _array_audit: 1

##### 3.16.5 Элемент массива: goods[5].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].txt[1] | АРТ: - 1 шт Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX16GE-L2-L2. Предназначен для коммутации пакетов данных. Порты: 16 RJ45 10/100/1000Base-TX, 2 SFP 100/1000Base-X. Металлический корпус IP40, крепление на DIN-рейку. | D | строка дополнения графы 31 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[5].tovg[]
- _array_audit: 1

##### 3.16.7 Элемент массива: goods[5].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[5].tovg[1].description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX16GE-L2-L2. Предназначен для коммутации пакетов данных. Порты: 16 RJ45 10/100/1000Base-TX, 2 SFP 100/1000Base-X. Металлический корпус IP40, крепление на DIN-рейку. | D | наименование | |
| 03 | goods[5].tovg[1].manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 04 | goods[5].tovg[1].trade_mark | Kyland | D | марка/ТМ | |
| 05 | goods[5].tovg[1].goods_mark | Kyland | D | товарный знак | |
| 06 | goods[5].tovg[1].model | SYM3000A-2GX16GE-L2-L2 | D | модель/модификация | |
| 07 | goods[5].tovg[1].quantity | 1 | D | количество в доп.ед | |
| 08 | goods[5].tovg[1].unit_code | 796 | D | код ЕИ | |
| 09 | goods[5].tovg[1].unit_name | шт | D | наименование ЕИ | |
| 10 | goods[5].tovg[1].gross_weight | 1.731 | D | вес брутто по строке | |
| 11 | goods[5].tovg[1].net_weight | 1.5 | D | вес нетто по строке | |
| 12 | goods[5].tovg[1].invoice_cost | 604.00 | D | цена по строке | |

- _item_audit: 12

##### 3.16.9 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[5].g44_docs[]
- _array_audit: 16

##### 3.16.11 Элемент массива: goods[5].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[5].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[5].g44_docs[1].doc_number | Im191018/Kyl | CP | номер документа | |
| 05 | goods[5].g44_docs[1].doc_date | 19.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[2].doc_code | 03012 | CP | код документа | |
| 02 | goods[5].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[5].g44_docs[2].doc_number | 221211 | CP | номер документа | |
| 05 | goods[5].g44_docs[2].doc_date | 11.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[3].doc_code | 03031 | CP | код документа | |
| 02 | goods[5].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[3].doc_name | УНК | CP | наименование документа | |
| 04 | goods[5].g44_docs[3].doc_number | 18100214110000097211 | CP | номер документа | |
| 05 | goods[5].g44_docs[3].doc_date | 25.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[4].doc_code | 04011 | CP | код документа | |
| 02 | goods[5].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[4].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[5].g44_docs[4].doc_number | ЮЭ9965-19-16744108 | CP | номер документа | |
| 05 | goods[5].g44_docs[4].doc_date | 14.02.2019 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[5].doc_code | 04021 | CP | код документа | |
| 02 | goods[5].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[5].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[5].g44_docs[5].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[5].g44_docs[5].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[6].doc_code | 04131 | CP | код документа | |
| 02 | goods[5].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[6].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[5].g44_docs[6].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[5].g44_docs[6].doc_date | 23.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[7].doc_code | 02017 | CP | код документа | |
| 02 | goods[5].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[7].doc_name | АВИАНАКЛАДНАЯ | CP | наименование документа | |
| 04 | goods[5].g44_docs[7].doc_number | 876-41176586 | CP | номер документа | |
| 05 | goods[5].g44_docs[7].doc_date | 02.01.2023 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[8].doc_code | 04023 | CP | код документа | |
| 02 | goods[5].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[8].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[5].g44_docs[8].doc_number | 30 | CP | номер документа | |
| 05 | goods[5].g44_docs[8].doc_date | 15.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[9].doc_code | 04033 | CP | код документа | |
| 02 | goods[5].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[5].g44_docs[9].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[5].g44_docs[9].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[10].doc_code | 04031 | CP | код документа | |
| 02 | goods[5].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[10].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[5].g44_docs[10].doc_number | VIG2227802 | CP | номер документа | |
| 05 | goods[5].g44_docs[10].doc_date | 28.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[11].doc_code | 04111 | CP | код документа | |
| 02 | goods[5].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[11].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[5].g44_docs[11].doc_number | VIG2227611 | CP | номер документа | |
| 05 | goods[5].g44_docs[11].doc_date | 27.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[12].doc_code | 01402 | CP | код документа | |
| 02 | goods[5].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[12].doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CP | наименование документа | |
| 04 | goods[5].g44_docs[12].doc_number | РА01.В.65848/21 | CP | номер документа | |
| 05 | goods[5].g44_docs[12].doc_date | 14.05.2021 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[13].doc_code | 10052 | CP | код документа | |
| 02 | goods[5].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[13].doc_name | НОТИФИКАЦИЯ | CP | наименование документа | |
| 04 | goods[5].g44_docs[13].doc_number | RU0000049917 | CP | номер документа | |
| 05 | goods[5].g44_docs[13].doc_date | 25.09.2020 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[14].doc_code | 05999 | CP | код документа | |
| 02 | goods[5].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[14].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[5].g44_docs[14].doc_number | БН | CP | номер документа | |
| 05 | goods[5].g44_docs[14].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[15].doc_code | 11004 | CP | код документа | |
| 02 | goods[5].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[15].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[5].g44_docs[15].doc_number | 1 | CP | номер документа | |
| 05 | goods[5].g44_docs[15].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[5].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[5].g44_docs[16].doc_code | 11001 | CP | код документа | |
| 02 | goods[5].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[5].g44_docs[16].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[5].g44_docs[16].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[5].g44_docs[16].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[6]
- goods._element_num: 6

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g31.name | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX8T-L2-L2. Предназначен для передачи пакетов данных. Порты: 8 RJ45 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[6].g31.manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 03 | goods[6].g31.trade_mark | Kyland | D | товарный знак / ТМ | |
| 04 | goods[6].places | | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].item_no | 6 | D | номер товара | |
| 02 | goods[6].tnved_code | 8517620003 | D | код товара | |
| 03 | goods[6].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[6].tnved.flag_2 | | D | доп. признак | |
| 05 | goods[6].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[6].gross_weight | 13.846 | D | вес брутто по товару | |
| 07 | goods[6].preference | ОООО-ОО | D | преференция | |
| 08 | goods[6].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[6].net_weight | 12 | D | вес нетто по товару | |
| 10 | goods[6].supplementary_quantity | 8 | D | количество в доп. единице | |
| 11 | goods[6].supplementary_unit_code | 796 | D | код доп. единицы | |
| 12 | goods[6].supplementary_unit_name | шт | D | наименование доп. единицы | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].invoice_cost | 2944.00 | D | Цена товара | |
| 02 | goods[6].transport_cost | 248.46 | D | доля транспортных расходов | |
| 03 | goods[6].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[6].insurance_cost | 604.54 | D | доля расходов на страхование | |
| 05 | goods[6].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[6].txt[]
- _array_audit: 1

##### 3.16.5 Элемент массива: goods[6].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].txt[1] | АРТ: - 8 шт Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX8T-L2-L2. Предназначен для передачи пакетов данных. Порты: 8 RJ45 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. | D | строка дополнения графы 31 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[6].tovg[]
- _array_audit: 1

##### 3.16.7 Элемент массива: goods[6].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[6].tovg[1].description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX8T-L2-L2. Предназначен для передачи пакетов данных. Порты: 8 RJ45 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. | D | наименование | |
| 03 | goods[6].tovg[1].manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 04 | goods[6].tovg[1].trade_mark | Kyland | D | марка/ТМ | |
| 05 | goods[6].tovg[1].goods_mark | Kyland | D | товарный знак | |
| 06 | goods[6].tovg[1].model | SYM3000A-2GX8T-L2-L2 | D | модель/модификация | |
| 07 | goods[6].tovg[1].quantity | 8 | D | количество в доп.ед | |
| 08 | goods[6].tovg[1].unit_code | 796 | D | код ЕИ | |
| 09 | goods[6].tovg[1].unit_name | шт | D | наименование ЕИ | |
| 10 | goods[6].tovg[1].gross_weight | 13.846 | D | вес брутто по строке | |
| 11 | goods[6].tovg[1].net_weight | 12 | D | вес нетто по строке | |
| 12 | goods[6].tovg[1].invoice_cost | 2944.00 | D | цена по строке | |

- _item_audit: 12

##### 3.16.9 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[6].g44_docs[]
- _array_audit: 16

##### 3.16.11 Элемент массива: goods[6].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[6].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[6].g44_docs[1].doc_number | Im191018/Kyl | CP | номер документа | |
| 05 | goods[6].g44_docs[1].doc_date | 19.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[2].doc_code | 03012 | CP | код документа | |
| 02 | goods[6].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[6].g44_docs[2].doc_number | 221211 | CP | номер документа | |
| 05 | goods[6].g44_docs[2].doc_date | 11.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[3].doc_code | 03031 | CP | код документа | |
| 02 | goods[6].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[3].doc_name | УНК | CP | наименование документа | |
| 04 | goods[6].g44_docs[3].doc_number | 18100214110000097211 | CP | номер документа | |
| 05 | goods[6].g44_docs[3].doc_date | 25.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[4].doc_code | 04011 | CP | код документа | |
| 02 | goods[6].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[4].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[6].g44_docs[4].doc_number | ЮЭ9965-19-16744108 | CP | номер документа | |
| 05 | goods[6].g44_docs[4].doc_date | 14.02.2019 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[5].doc_code | 04021 | CP | код документа | |
| 02 | goods[6].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[5].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[6].g44_docs[5].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[6].g44_docs[5].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[6].doc_code | 04131 | CP | код документа | |
| 02 | goods[6].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[6].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[6].g44_docs[6].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[6].g44_docs[6].doc_date | 23.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[7].doc_code | 02017 | CP | код документа | |
| 02 | goods[6].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[7].doc_name | АВИАНАКЛАДНАЯ | CP | наименование документа | |
| 04 | goods[6].g44_docs[7].doc_number | 876-41176586 | CP | номер документа | |
| 05 | goods[6].g44_docs[7].doc_date | 02.01.2023 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[8].doc_code | 04023 | CP | код документа | |
| 02 | goods[6].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[8].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[6].g44_docs[8].doc_number | 30 | CP | номер документа | |
| 05 | goods[6].g44_docs[8].doc_date | 15.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[9].doc_code | 04033 | CP | код документа | |
| 02 | goods[6].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[6].g44_docs[9].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[6].g44_docs[9].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[10].doc_code | 04031 | CP | код документа | |
| 02 | goods[6].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[10].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[6].g44_docs[10].doc_number | VIG2227802 | CP | номер документа | |
| 05 | goods[6].g44_docs[10].doc_date | 28.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[11].doc_code | 04111 | CP | код документа | |
| 02 | goods[6].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[11].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[6].g44_docs[11].doc_number | VIG2227611 | CP | номер документа | |
| 05 | goods[6].g44_docs[11].doc_date | 27.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[12].doc_code | 01402 | CP | код документа | |
| 02 | goods[6].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[12].doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CP | наименование документа | |
| 04 | goods[6].g44_docs[12].doc_number | РА01.В.65848/21 | CP | номер документа | |
| 05 | goods[6].g44_docs[12].doc_date | 14.05.2021 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[13].doc_code | 10052 | CP | код документа | |
| 02 | goods[6].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[13].doc_name | НОТИФИКАЦИЯ | CP | наименование документа | |
| 04 | goods[6].g44_docs[13].doc_number | RU0000049917 | CP | номер документа | |
| 05 | goods[6].g44_docs[13].doc_date | 25.09.2020 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[14].doc_code | 05999 | CP | код документа | |
| 02 | goods[6].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[14].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[6].g44_docs[14].doc_number | БН | CP | номер документа | |
| 05 | goods[6].g44_docs[14].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[15].doc_code | 11004 | CP | код документа | |
| 02 | goods[6].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[15].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[6].g44_docs[15].doc_number | 1 | CP | номер документа | |
| 05 | goods[6].g44_docs[15].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[6].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[6].g44_docs[16].doc_code | 11001 | CP | код документа | |
| 02 | goods[6].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[6].g44_docs[16].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[6].g44_docs[16].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[6].g44_docs[16].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

#### 3.16.0 Элемент массива: goods[7]
- goods._element_num: 7

##### 3.16.1 Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g31.name | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX16T-L2-L2. Предназначен для передачи и коммутации пакетов данных. Порты: 16 RJ45 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[7].g31.manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 03 | goods[7].g31.trade_mark | Kyland | D | товарный знак / ТМ | |
| 04 | goods[7].places | 2 | D | количество мест по товару | |

- _audit: 4

##### 3.16.2 Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].item_no | 7 | D | номер товара | |
| 02 | goods[7].tnved_code | 8517620003 | D | код товара | |
| 03 | goods[7].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[7].tnved.flag_2 | | D | доп. признак | |
| 05 | goods[7].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[7].gross_weight | 5.192 | D | вес брутто по товару | |
| 07 | goods[7].preference | ОООО-ОО | D | преференция | |
| 08 | goods[7].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[7].net_weight | 4.5 | D | вес нетто по товару | |
| 10 | goods[7].supplementary_quantity | 3 | D | количество в доп. единице | |
| 11 | goods[7].supplementary_unit_code | 796 | D | код доп. единицы | |
| 12 | goods[7].supplementary_unit_name | шт | D | наименование доп. единицы | |

- _audit: 12

##### 3.16.3 Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].invoice_cost | 1575.00 | D | Цена товара | |
| 02 | goods[7].transport_cost | 93.17 | D | доля транспортных расходов | |
| 03 | goods[7].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[7].insurance_cost | 323.37 | D | доля расходов на страхование | |
| 05 | goods[7].insurance_currency | RUB | CP | валюта страхования | |

- _audit: 5

##### 3.16.4 Массив: goods[7].txt[]
- _array_audit: 1

##### 3.16.5 Элемент массива: goods[7].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].txt[1] | АРТ: - 3 шт Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX16T-L2-L2. Предназначен для передачи и коммутации пакетов данных. Порты: 16 RJ45 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. | D | строка дополнения графы 31 | |

- _item_audit: 1

##### 3.16.6 Массив: goods[7].tovg[]
- _array_audit: 1

##### 3.16.7 Элемент массива: goods[7].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[7].tovg[1].description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX16T-L2-L2. Предназначен для передачи и коммутации пакетов данных. Порты: 16 RJ45 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. | D | наименование | |
| 03 | goods[7].tovg[1].manufacturer | Kyland Technology Co., Ltd. | D | производитель | |
| 04 | goods[7].tovg[1].trade_mark | Kyland | D | марка/ТМ | |
| 05 | goods[7].tovg[1].goods_mark | Kyland | D | товарный знак | |
| 06 | goods[7].tovg[1].model | SYM3000A-2GX16T-L2-L2 | D | модель/модификация | |
| 07 | goods[7].tovg[1].quantity | 3 | D | количество в доп.ед | |
| 08 | goods[7].tovg[1].unit_code | 796 | D | код ЕИ | |
| 09 | goods[7].tovg[1].unit_name | шт | D | наименование ЕИ | |
| 10 | goods[7].tovg[1].gross_weight | 5.192 | D | вес брутто по строке | |
| 11 | goods[7].tovg[1].net_weight | 4.5 | D | вес нетто по строке | |
| 12 | goods[7].tovg[1].invoice_cost | 1575.00 | D | цена по строке | |

- _item_audit: 12

##### 3.16.9 Графа 44 — представляемые документы (G_44)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |

- _audit: 1

##### 3.16.10 Массив: goods[7].g44_docs[]
- _array_audit: 16

##### 3.16.11 Элемент массива: goods[7].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[7].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[7].g44_docs[1].doc_number | Im191018/Kyl | CP | номер документа | |
| 05 | goods[7].g44_docs[1].doc_date | 19.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[2].doc_code | 03012 | CP | код документа | |
| 02 | goods[7].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[7].g44_docs[2].doc_number | 221211 | CP | номер документа | |
| 05 | goods[7].g44_docs[2].doc_date | 11.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[3].doc_code | 03031 | CP | код документа | |
| 02 | goods[7].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[3].doc_name | УНК | CP | наименование документа | |
| 04 | goods[7].g44_docs[3].doc_number | 18100214110000097211 | CP | номер документа | |
| 05 | goods[7].g44_docs[3].doc_date | 25.10.2018 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[4].doc_code | 04011 | CP | код документа | |
| 02 | goods[7].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[4].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[7].g44_docs[4].doc_number | ЮЭ9965-19-16744108 | CP | номер документа | |
| 05 | goods[7].g44_docs[4].doc_date | 14.02.2019 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[5].doc_code | 04021 | CP | код документа | |
| 02 | goods[7].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[5].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[7].g44_docs[5].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[7].g44_docs[5].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[6].doc_code | 04131 | CP | код документа | |
| 02 | goods[7].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[6].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[7].g44_docs[6].doc_number | 1000059769 /60285 /60389 /60491 | CP | номер документа | |
| 05 | goods[7].g44_docs[6].doc_date | 23.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[7].doc_code | 02017 | CP | код документа | |
| 02 | goods[7].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[7].doc_name | АВИАНАКЛАДНАЯ | CP | наименование документа | |
| 04 | goods[7].g44_docs[7].doc_number | 876-41176586 | CP | номер документа | |
| 05 | goods[7].g44_docs[7].doc_date | 02.01.2023 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[8].doc_code | 04023 | CP | код документа | |
| 02 | goods[7].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[8].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[7].g44_docs[8].doc_number | 30 | CP | номер документа | |
| 05 | goods[7].g44_docs[8].doc_date | 15.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[9].doc_code | 04033 | CP | код документа | |
| 02 | goods[7].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[9].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[7].g44_docs[9].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[7].g44_docs[9].doc_date | 13.05.2025 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[10].doc_code | 04031 | CP | код документа | |
| 02 | goods[7].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[10].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[7].g44_docs[10].doc_number | VIG2227802 | CP | номер документа | |
| 05 | goods[7].g44_docs[10].doc_date | 28.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[11].doc_code | 04111 | CP | код документа | |
| 02 | goods[7].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[11].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[7].g44_docs[11].doc_number | VIG2227611 | CP | номер документа | |
| 05 | goods[7].g44_docs[11].doc_date | 27.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[12].doc_code | 01402 | CP | код документа | |
| 02 | goods[7].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[12].doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CP | наименование документа | |
| 04 | goods[7].g44_docs[12].doc_number | РА01.В.65848/21 | CP | номер документа | |
| 05 | goods[7].g44_docs[12].doc_date | 14.05.2021 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[13].doc_code | 10052 | CP | код документа | |
| 02 | goods[7].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[13].doc_name | НОТИФИКАЦИЯ | CP | наименование документа | |
| 04 | goods[7].g44_docs[13].doc_number | RU0000049917 | CP | номер документа | |
| 05 | goods[7].g44_docs[13].doc_date | 25.09.2020 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[14].doc_code | 05999 | CP | код документа | |
| 02 | goods[7].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[14].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[7].g44_docs[14].doc_number | БН | CP | номер документа | |
| 05 | goods[7].g44_docs[14].doc_date | 12.12.2022 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[15].doc_code | 11004 | CP | код документа | |
| 02 | goods[7].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[15].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[7].g44_docs[15].doc_number | 1 | CP | номер документа | |
| 05 | goods[7].g44_docs[15].doc_date | 01.02.2026 | CP | дата документа | |

- _item_audit: 5

##### 3.16.11 Элемент массива: goods[7].g44_docs[16]
- _element_num: 16

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[7].g44_docs[16].doc_code | 11001 | CP | код документа | |
| 02 | goods[7].g44_docs[16].kind_code | 0 | CO | признак записи | |
| 03 | goods[7].g44_docs[16].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[7].g44_docs[16].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[7].g44_docs[16].doc_date | 11.03.2010 | CP | дата документа | |

- _item_audit: 5

### 3.17 Теги после товаров и документов (графы 51–54)

#### 3.17.1 Графа 54 — уполномоченное лицо / представитель

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | representative.date | 30.06.2026 | D | дата заполнения/подачи | |
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
| 13 | representative.passport_name | ПАСРФ | CO | наименование документа | |
| 14 | representative.passport_number | 449948 | CP | номер паспорта | |
| 15 | representative.passport_date | 11.03.2010 | CP | дата выдачи паспорта | |
| 16 | representative.passport_series | 63 09 | CP | серия паспорта | |
| 17 | representative.passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CP | кем выдан | |

- _audit: 17

### Итог:
- `dt_status`: confirmed

## Часть II: Issues (нерешенные вопросы)

**Для полей:**

**Для общих вопросов:**
- `[Общий]`:
  - `question`: вопросов нет.
