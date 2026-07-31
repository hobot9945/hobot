# Исходные данные для ДТ

## Метаданные:
- `название кейса`: МоскитнаяСеткаWuqiang
- `путь к папке поставки`: alta\source\МоскитнаяСеткаWuqiang\02
- `тип поставки`: 1 ДТ/3 товара
- `агрегация ДТ`: сгруппировано по кодам ТН ВЭД (разрешено)
- `источники данных`: primary.md + operator_provided_data.md

## Часть I: Поля ДТ

### Заголовок декларации (графа 1)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | declaration.direction | ИМ | CP | направление декларации | |
| 02 | declaration.procedure | 40 | CO | код таможенной процедуры | |
| 03 | declaration.form | ЭД | D | форма подачи декларации | |
- _audit: 3

### Отправитель (графа 2)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | sender.country_name | Китай | CP | текстовое название страны | |
| 02 | sender.country_code | CN | CP | код страны alpha-2 | |
| 03 | sender.name | Wuqiang County Huili Fiberglass Co.,Ltd. | CP | полное наименование отправителя | |
| 04 | sender.region | Hebei | CP | область/район | |
| 05 | sender.city | Wuqiang, Hengshui | CP | город | |
| 06 | sender.street | Haozhuang Industrial Zone | CP | улица и дом | |
- _audit: 6

### Количество товаров и мест (графы 5, 6)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.total_goods_number | 3 | D | количество товарных позиций в ДТ | |
| 02 | shipment.packages_flag | true | D | признак подсчёта мест | |
| 03 | shipment.total_packages | 83 | D | общее количество грузовых мест | |
- _audit: 3

### Получатель (графа 8)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | consignee.same_as_declarant | true | D | признак «см. графу 14» | |
| 02 | consignee.ogrn | 1201600020390 | D | ОГРН получателя | |
| 03 | consignee.inn_kpp | 1650389298/165001001 | D | ИНН/КПП получателя | |
| 04 | consignee.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование организации | |
| 05 | consignee.country_code | RU | D | код страны получателя | |
| 06 | consignee.country_name | РОССИЯ | D | страна получателя, текст | |
| 07 | consignee.postcode | 423800 | D | почтовый индекс | |
| 08 | consignee.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион получателя | |
| 09 | consignee.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | город получателя | |
| 10 | consignee.street | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | D | улица получателя | |
| 11 | consignee.building | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | D | дом получателя | |
| 12 | consignee.room | ОФИС 211 | D | помещение/офис | |
| 13 | consignee.phone | +7 (843) 207 18 90 | D | телефон получателя | |
| 14 | consignee.email | PROM_TAT@MAIL.RU | D | e-mail получателя | |
- _audit: 14

### Финансовое урегулирование (графа 9)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | financial.same_as_declarant | true | D | признак «см. графу 14» | |
| 02 | financial.ogrn | 1201600020390 | D | ОГРН контрагента | |
| 03 | financial.inn_kpp | 1650389298/165001001 | D | ИНН/КПП контрагента | |
| 04 | financial.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование организации | |
| 05 | financial.country_code | RU | D | код страны контрагента | |
| 06 | financial.country_name | РОССИЯ | D | страна контрагента, текст | |
| 07 | financial.postcode | 423800 | D | индекс контрагента | |
| 08 | financial.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион контрагента | |
| 09 | financial.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | город контрагента | |
| 10 | financial.street | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | D | улица контрагента | |
| 11 | financial.building | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | D | дом контрагента | |
| 12 | financial.room | ОФИС 211 | D | помещение контрагента | |
| 13 | financial.country_code_alt | RU | D | дублирующий код страны | |
| 14 | financial.phone | +7 (843) 207 18 90 | D | телефон контрагента | |
| 15 | financial.email | PROM_TAT@MAIL.RU | D | e-mail контрагента | |
- _audit: 15

### Торгующая страна (графа 11)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.trade_country_code | CN | CP | код торгующей страны alpha-2 | |
- _audit: 1

### Декларант (графа 14)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | declarant.ogrn | 1201600020390 | CP | ОГРН декларанта | |
| 02 | declarant.inn_kpp | 1650389298/165001001 | D | ИНН/КПП декларанта | |
| 03 | declarant.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CP | наименование декларанта | |
| 04 | declarant.country_code | RU | CP | код страны декларанта | |
| 05 | declarant.country_name | РОССИЯ | CP | наименование страны | |
| 06 | declarant.postcode | 423800 | CP | почтовый индекс | |
| 07 | declarant.region | РЕСПУБЛИКА ТАТАРСТАН | CP | регион | |
| 08 | declarant.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | населённый пункт | |
| 09 | declarant.street | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CP | улица | |
| 10 | declarant.building | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CP | дом | |
| 11 | declarant.room | ОФИС 211 | D | помещение/офис | |
| 12 | declarant.phone | +7 (843) 207 18 90 | CP | телефон | |
| 13 | declarant.email | PROM_TAT@MAIL.RU | CP | e-mail | |
- _audit: 13

### Страны (графы 15, 16, 17)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.dispatch_country_code | CN | CP | код страны отправления alpha-2 | |
| 02 | shipment.destination_country_code | RU | CP | код страны назначения alpha-2 | |
| 03 | shipment.dispatch_country_name | КИТАЙ | D | страна отправления, текст | |
| 04 | shipment.destination_country_name | РОССИЯ | D | страна назначения, текст | |
| 05 | shipment.origin_country_code | CN | D | код страны происхождения alpha-2 | |
| 06 | shipment.origin_country_name | КИТАЙ | D | страна происхождения, текст | |
- _audit: 6

### Условия поставки (графа 20)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | delivery.terms_code | EXW | D | условия поставки | |
| 02 | delivery.place_name | HEBEI | D | место поставки | |
- _audit: 2

### Транспорт (графы 18, 19, 21)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.vehicles_count | 2 | D | количество транспортных средств | |
| 02 | transport.identification | T927MC55/AP087055 | D | идентификация ТС | |
| 03 | transport.registration_country_code | RU | D | код страны регистрации ТС | |
| 04 | transport.container_flag | 0 | CO | признак контейнера | |
| 05 | transport.border_mode | 1 | D | активные ТС на границе | |
| 06 | transport.border_id | | D | номер ТС на границе | |
| 07 | transport.border_country_code | | D | код страны регистрации ТС на границе | |
- _audit: 7

### Валюта и стоимость (графа 22)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.invoice_currency_numeric | 156 | D | цифровой код валюты | |
| 02 | shipment.invoice_currency_alpha | CNY | CP | буквенный код валюты | |
| 03 | shipment.invoice_amount | 85214.40 | CP | сумма по счёту | |
- _audit: 3

### Характер сделки (графа 24)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.transaction_nature | 010 | D | характер сделки | |
| 02 | shipment.transaction_feature | 06 | D | особенность сделки | |
- _audit: 2

### Вид транспорта (графы 25, 26)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.border_transport_code | 31 | D | код вида транспорта на границе | |
| 02 | transport.internal_transport_code | 31 | D | код вида транспорта внутри страны | |
- _audit: 2

### Таможня на границе (графа 29)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs.border_code | 10719110 | CP | код таможенного органа | |
| 02 | customs.border_name | т/п МАПП Забайкальск | CP | наименование таможенного поста | |
- _audit: 2

### Местонахождение товаров (графа 30)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | location.document_number | 10404/141210/10092/6 | CO | номер документа СВХ | |
- _audit: 1

### 3.16 Массив: goods[]
- goods._array_audit: 3

#### 3.16.0 Элемент массива: goods[1]
- goods._element_num: 1

##### Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g31.name | Сетка москитная «Антимошка» из стекловолокна. Изготовлена из прочных стекловолоконных текстильных нитей полотняного переплетения с мелкими ячейками (плотность 22*22 нити на дюйм) в рулонах. Применяется для защиты от проникновения мельчайших насекомых через оконные проемы. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[1].g31.manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | D | производитель | |
| 03 | goods[1].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | |
| 04 | goods[1].places | 40 | D | количество мест по товару | |
- _audit: 4

##### Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].item_no | 1 | D | номер товара | |
| 02 | goods[1].tnved_code | 7019900095 | D | код товара | |
| 03 | goods[1].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[1].tnved.flag_2 | N | D | доп. признак | |
| 05 | goods[1].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[1].gross_weight | 2608 | D | вес брутто по товару | |
| 07 | goods[1].preference | ОООО-ОО | D | преференция | |
| 08 | goods[1].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[1].net_weight | 2340 | D | вес нетто по товару | |
| 10 | goods[1].supplementary_quantity | 18000 | D | количество в дополнительной единице | |
| 11 | goods[1].supplementary_unit_code | 055 | D | код дополнительной единицы | |
| 12 | goods[1].supplementary_unit_name | м2 | D | наименование дополнительной единицы | |
- _audit: 12

##### Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].invoice_cost | 61200.00 | D | Цена товара | |
| 02 | goods[1].transport_cost | 1112.06 | D | доля транспортных расходов | |
| 03 | goods[1].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[1].insurance_cost | 582.35 | D | доля расходов на страхование | |
| 05 | goods[1].insurance_currency | RUB | CP | валюта страхования | |
- _audit: 5

##### 3.16.4 Массив: goods[1].txt[]
- _array_audit: 4

##### 3.16.5 Элемент массива: goods[1].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[1] | АРТ: - 4800 м2 Сетка от мошек 22*22 Серая 1,6м*30м | D | графа 31 — TXT строка 1 | |
- _item_audit: 1

##### 3.16.5 Элемент массива: goods[1].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[2] | АРТ: - 4200 м2 Сетка от мошек 22*22 Серая 1,4м*30м | D | графа 31 — TXT строка 2 | |
- _item_audit: 1

##### 3.16.5 Элемент массива: goods[1].txt[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[3] | АРТ: - 4800 м2 Сетка от мошек 22*22 Черный 1,6м*30м | D | графа 31 — TXT строка 3 | |
- _item_audit: 1

##### 3.16.5 Элемент массива: goods[1].txt[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[4] | АРТ: - 4200 м2 Сетка от мошек 22*22 Черный 1,4м*30м | D | графа 31 — TXT строка 4 | |
- _item_audit: 1

##### 3.16.6 Массив: goods[1].tovg[]
- _array_audit: 4

##### 3.16.7 Элемент массива: goods[1].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[1].tovg[1].description | Сетка от мошек 22*22 Серая 1,6м*30м | D | графа 31 — наименование | |
| 03 | goods[1].tovg[1].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | D | производитель | |
| 04 | goods[1].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[1].model | 22*22 Grey 1,6m*30 m | D | модель/модификация | |
| 07 | goods[1].tovg[1].quantity | 4800 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[1].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[1].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[1].gross_weight | 696 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[1].net_weight | 624 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[1].invoice_cost | 16320.00 | CP | цена по строке | |
- _item_audit: 12

##### 3.16.7 Элемент массива: goods[1].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[2].line_no | 2 | D | № строки таблицы | |
| 02 | goods[1].tovg[2].description | Сетка от мошек 22*22 Серая 1,4м*30м | D | графа 31 — наименование | |
| 03 | goods[1].tovg[2].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | D | производитель | |
| 04 | goods[1].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[2].model | 22*22 Grey 1,4m*30 m | D | модель/модификация | |
| 07 | goods[1].tovg[2].quantity | 4200 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[2].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[2].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[2].gross_weight | 696 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[2].net_weight | 624 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[2].invoice_cost | 14280.00 | CP | цена по строке | |
- _item_audit: 12

##### 3.16.7 Элемент массива: goods[1].tovg[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[3].line_no | 3 | D | № строки таблицы | |
| 02 | goods[1].tovg[3].description | Сетка от мошек 22*22 Черный 1,6м*30м | D | графа 31 — наименование | |
| 03 | goods[1].tovg[3].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | D | производитель | |
| 04 | goods[1].tovg[3].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[3].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[3].model | 22*22 Black 1,6m*30 m | D | модель/модификация | |
| 07 | goods[1].tovg[3].quantity | 4800 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[3].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[3].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[3].gross_weight | 608 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[3].net_weight | 546 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[3].invoice_cost | 16320.00 | CP | цена по строке | |
- _item_audit: 12

##### 3.16.7 Элемент массива: goods[1].tovg[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[4].line_no | 4 | D | № строки таблицы | |
| 02 | goods[1].tovg[4].description | Сетка от мошек 22*22 Черный 1,4м*30м | D | графа 31 — наименование | |
| 03 | goods[1].tovg[4].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | D | производитель | |
| 04 | goods[1].tovg[4].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[4].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[4].model | 22*22 Black 1,4m*30 m | D | модель/модификация | |
| 07 | goods[1].tovg[4].quantity | 4200 | CP | количество в доп.ед.изм | |
| 08 | goods[1].tovg[4].unit_code | 055 | D | код ЕИ | |
| 09 | goods[1].tovg[4].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[1].tovg[4].gross_weight | 608 | CP | вес брутто по строке | |
| 11 | goods[1].tovg[4].net_weight | 546 | CP | вес нетто по строке | |
| 12 | goods[1].tovg[4].invoice_cost | 14280.00 | CP | цена по строке | |
- _item_audit: 12

##### Графа 44 — представляемые документы

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |
- _audit: 1

##### 3.16.10 Массив: goods[1].g44_docs[]
- _array_audit: 15

##### 3.16.11 Элемент массива: goods[1].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[1].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[1].doc_number | 26HL-1103 | CP | номер документа | |
| 05 | goods[1].g44_docs[1].doc_date | 31.03.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[2].doc_code | 04021 | CP | код документа | |
| 02 | goods[1].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[2].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[1].g44_docs[2].doc_number | 25HL-1083 | CP | номер документа | |
| 05 | goods[1].g44_docs[2].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[3].doc_code | 04131 | CP | код документа | |
| 02 | goods[1].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[3].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[3].doc_number | 25HL-1083/1 | CP | номер документа | |
| 05 | goods[1].g44_docs[3].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[4].doc_code | 02015 | CP | код документа | |
| 02 | goods[1].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[4].doc_name | CMR | CP | наименование документа | |
| 04 | goods[1].g44_docs[4].doc_number | 18614 | CP | номер документа | |
| 05 | goods[1].g44_docs[4].doc_date | 15.07.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[5].doc_code | 09013 | CP | код документа | |
| 02 | goods[1].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[5].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | |
| 04 | goods[1].g44_docs[5].doc_number | 10719110/180726/5122642 | CP | номер документа | |
| 05 | goods[1].g44_docs[5].doc_date | 18.07.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[6].doc_code | 04023 | CP | код документа | |
| 02 | goods[1].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[6].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[6].doc_number | 9 | CP | номер документа | |
| 05 | goods[1].g44_docs[6].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[7].doc_code | 04031 | CP | код документа | |
| 02 | goods[1].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[7].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[7].doc_number | 26-18614-tl | CP | номер документа | |
| 05 | goods[1].g44_docs[7].doc_date | 15.07.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[8].doc_code | 04111 | CP | код документа | |
| 02 | goods[1].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[8].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[8].doc_number | 26-18614-tl/1 | CP | номер документа | |
| 05 | goods[1].g44_docs[8].doc_date | 30.06.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[9].doc_code | 05999 | CP | код документа | |
| 02 | goods[1].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[9].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[9].doc_number | БН | CP | номер документа | |
| 05 | goods[1].g44_docs[9].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[10].doc_code | 04011 | CP | код документа | |
| 02 | goods[1].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[1].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | |
| 05 | goods[1].g44_docs[10].doc_date | 14.07.2025 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[11].doc_code | 11001 | CP | код документа | |
| 02 | goods[1].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[11].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[11].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[1].g44_docs[11].doc_date | 11.03.2010 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[12].doc_code | 11004 | CP | код документа | |
| 02 | goods[1].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[1].g44_docs[12].doc_number | 1 | CP | номер документа | |
| 05 | goods[1].g44_docs[12].doc_date | 01.02.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[13].doc_code | 04033 | CP | код документа | |
| 02 | goods[1].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[13].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[13].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[1].g44_docs[13].doc_date | 13.05.2025 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[14].doc_code | 09023 | CP | код документа | |
| 02 | goods[1].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[14].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[1].g44_docs[14].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[1].g44_docs[14].doc_date | 20.08.2025 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[1].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[15].doc_code | 09999 | CP | код документа | |
| 02 | goods[1].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[1].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[1].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | |
- _item_audit: 5

#### 3.16.0 Элемент массива: goods[2]
- goods._element_num: 2

##### Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g31.name | Сетка москитная «Антикот» из полиэстера. Представляет собой одноцветное сетчатое полотно без узора в рулонах, изготовленное из усиленных полиэфирных волокон, покрытых ПВХ (плотность основы 14, утка 11 нитей на дюйм), устойчивое к когтям животных. Предназначено для установки в оконные проемы. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[2].g31.manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | D | производитель | |
| 03 | goods[2].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | |
| 04 | goods[2].places | 42 | D | количество мест по товару | |
- _audit: 4

##### Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].item_no | 2 | D | номер товара | |
| 02 | goods[2].tnved_code | 5804101000 | D | код товара | |
| 03 | goods[2].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[2].tnved.flag_2 | N | D | доп. признак | |
| 05 | goods[2].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[2].gross_weight | 719 | D | вес брутто по товару | |
| 07 | goods[2].preference | ОООО-ОО | D | преференция | |
| 08 | goods[2].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[2].net_weight | 645.1 | D | вес нетто по товару | |
| 10 | goods[2].supplementary_quantity | 2016 | D | количество в дополнительной единице | |
| 11 | goods[2].supplementary_unit_code | 055 | D | код дополнительной единицы | |
| 12 | goods[2].supplementary_unit_name | м2 | D | наименование дополнительной единицы | |
- _audit: 12

##### Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].invoice_cost | 14918.40 | D | Цена товара | |
| 02 | goods[2].transport_cost | 306.66 | D | доля транспортных расходов | |
| 03 | goods[2].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[2].insurance_cost | 141.97 | D | доля расходов на страхование | |
| 05 | goods[2].insurance_currency | RUB | CP | валюта страхования | |
- _audit: 5

##### 3.16.4 Массив: goods[2].txt[]
- _array_audit: 1

##### 3.16.5 Элемент массива: goods[2].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].txt[1] | АРТ: - 2016 м2 Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Черная и серебристо-серая | D | графа 31 — TXT строка 1 | |
- _item_audit: 1

##### 3.16.6 Массив: goods[2].tovg[]
- _array_audit: 1

##### 3.16.7 Элемент массива: goods[2].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[2].tovg[1].description | Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Черная и серебристо-серая | D | графа 31 — наименование | |
| 03 | goods[2].tovg[1].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | D | производитель | |
| 04 | goods[2].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[2].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[2].tovg[1].model | 320g 1.6m x 30m Black and Silver Grey | D | модель/модификация | |
| 07 | goods[2].tovg[1].quantity | 2016 | CP | количество в доп.ед.изм | |
| 08 | goods[2].tovg[1].unit_code | 055 | D | код ЕИ | |
| 09 | goods[2].tovg[1].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[2].tovg[1].gross_weight | 719 | CP | вес брутто по строке | |
| 11 | goods[2].tovg[1].net_weight | 645.1 | CP | вес нетто по строке | |
| 12 | goods[2].tovg[1].invoice_cost | 14918.40 | CP | цена по строке | |
- _item_audit: 12

##### Графа 44 — представляемые документы

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |
- _audit: 1

##### 3.16.10 Массив: goods[2].g44_docs[]
- _array_audit: 15

##### 3.16.11 Элемент массива: goods[2].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[2].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[1].doc_number | 26HL-1103 | CP | номер документа | |
| 05 | goods[2].g44_docs[1].doc_date | 31.03.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[2].doc_code | 04021 | CP | код документа | |
| 02 | goods[2].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[2].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[2].g44_docs[2].doc_number | 25HL-1083 | CP | номер документа | |
| 05 | goods[2].g44_docs[2].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[3].doc_code | 04131 | CP | код документа | |
| 02 | goods[2].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[3].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[3].doc_number | 25HL-1083/1 | CP | номер документа | |
| 05 | goods[2].g44_docs[3].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[4].doc_code | 02015 | CP | код документа | |
| 02 | goods[2].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[4].doc_name | CMR | CP | наименование документа | |
| 04 | goods[2].g44_docs[4].doc_number | 18614 | CP | номер документа | |
| 05 | goods[2].g44_docs[4].doc_date | 15.07.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[5].doc_code | 09013 | CP | код документа | |
| 02 | goods[2].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[5].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | |
| 04 | goods[2].g44_docs[5].doc_number | 10719110/180726/5122642 | CP | номер документа | |
| 05 | goods[2].g44_docs[5].doc_date | 18.07.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[6].doc_code | 04023 | CP | код документа | |
| 02 | goods[2].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[6].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[6].doc_number | 9 | CP | номер документа | |
| 05 | goods[2].g44_docs[6].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[7].doc_code | 04031 | CP | код документа | |
| 02 | goods[2].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[7].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[2].g44_docs[7].doc_number | 26-18614-tl | CP | номер документа | |
| 05 | goods[2].g44_docs[7].doc_date | 15.07.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[8].doc_code | 04111 | CP | код документа | |
| 02 | goods[2].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[8].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[2].g44_docs[8].doc_number | 26-18614-tl/1 | CP | номер документа | |
| 05 | goods[2].g44_docs[8].doc_date | 30.06.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[9].doc_code | 05999 | CP | код документа | |
| 02 | goods[2].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[9].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[9].doc_number | БН | CP | номер документа | |
| 05 | goods[2].g44_docs[9].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[10].doc_code | 04011 | CP | код документа | |
| 02 | goods[2].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[2].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | |
| 05 | goods[2].g44_docs[10].doc_date | 14.07.2025 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[11].doc_code | 11001 | CP | код документа | |
| 02 | goods[2].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[11].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[2].g44_docs[11].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[2].g44_docs[11].doc_date | 11.03.2010 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[12].doc_code | 11004 | CP | код документа | |
| 02 | goods[2].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[2].g44_docs[12].doc_number | 1 | CP | номер документа | |
| 05 | goods[2].g44_docs[12].doc_date | 01.02.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[13].doc_code | 04033 | CP | код документа | |
| 02 | goods[2].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[13].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[2].g44_docs[13].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[2].g44_docs[13].doc_date | 13.05.2025 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[14].doc_code | 09023 | CP | код документа | |
| 02 | goods[2].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[14].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[2].g44_docs[14].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[2].g44_docs[14].doc_date | 20.08.2025 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[2].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[2].g44_docs[15].doc_code | 09999 | CP | код документа | |
| 02 | goods[2].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[2].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[2].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[2].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | |
- _item_audit: 5

#### 3.16.0 Элемент массива: goods[3]
- goods._element_num: 3

##### Графа 31 — описание товаров (G_31)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g31.name | Сетка металлическая из нержавеющей стали. Изготовлена путем переплетения стальной проволоки марки SS304 (толщина 0.17-0.23 мм, плотность 18*18 нитей на дюйм) в рулонах. Сверхпрочное полотно, устойчивое к коррозии и механическим повреждениям. Предназначено для защиты от грызунов и насекомых. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[3].g31.manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | D | производитель | |
| 03 | goods[3].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак / ТМ | |
| 04 | goods[3].places | 1 | D | количество мест по товару | |
- _audit: 4

##### Графы 32–38 — код товара, страна, веса, процедура

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].item_no | 3 | D | номер товара | |
| 02 | goods[3].tnved_code | 7314490000 | D | код товара | |
| 03 | goods[3].tnved.flag_1 | С | D | доп. признак | |
| 04 | goods[3].tnved.flag_2 | N | D | доп. признак | |
| 05 | goods[3].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[3].gross_weight | 293 | D | вес брутто по товару | |
| 07 | goods[3].preference | ОООО-ОО | D | преференция | |
| 08 | goods[3].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[3].net_weight | 261.5 | D | вес нетто по товару | |
| 10 | goods[3].supplementary_quantity | 840 | D | количество в дополнительной единице | |
| 11 | goods[3].supplementary_unit_code | 055 | D | код дополнительной единицы | |
| 12 | goods[3].supplementary_unit_name | м2 | D | наименование дополнительной единицы | |
- _audit: 12

##### Графы 42–46 — исходные данные стоимости по товару

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].invoice_cost | 9096.00 | D | Цена товара | |
| 02 | goods[3].transport_cost | 125.28 | D | доля транспортных расходов | |
| 03 | goods[3].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[3].insurance_cost | 86.42 | D | доля расходов на страхование | |
| 05 | goods[3].insurance_currency | RUB | CP | валюта страхования | |
- _audit: 5

##### 3.16.4 Массив: goods[3].txt[]
- _array_audit: 1

##### 3.16.5 Элемент массива: goods[3].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].txt[1] | АРТ: - 840 м2 сетка 17 мм материал SS304. Размер рулона 1,4*30 М2 Черный | D | графа 31 — TXT строка 1 | |
- _item_audit: 1

##### 3.16.6 Массив: goods[3].tovg[]
- _array_audit: 1

##### 3.16.7 Элемент массива: goods[3].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].tovg[1].line_no | 1 | D | № строки таблицы | |
| 02 | goods[3].tovg[1].description | сетка 17 мм материал SS304. Размер рулона 1,4*30 М2 Черный | D | графа 31 — наименование | |
| 03 | goods[3].tovg[1].manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | D | производитель | |
| 04 | goods[3].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[3].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[3].tovg[1].model | Mesh 20 0.17 mm material SS304 Roll size: 1.4*30 m black | D | модель/модификация | |
| 07 | goods[3].tovg[1].quantity | 840 | CP | количество в доп.ед.изм | |
| 08 | goods[3].tovg[1].unit_code | 055 | D | код ЕИ | |
| 09 | goods[3].tovg[1].unit_name | м2 | CP | наименование ЕИ | |
| 10 | goods[3].tovg[1].gross_weight | 293 | CP | вес брутто по строке | |
| 11 | goods[3].tovg[1].net_weight | 261.5 | CP | вес нетто по строке | |
| 12 | goods[3].tovg[1].invoice_cost | 9096.00 | CP | цена по строке | |
- _item_audit: 12

##### Графа 44 — представляемые документы

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле | |
- _audit: 1

##### 3.16.10 Массив: goods[3].g44_docs[]
- _array_audit: 15

##### 3.16.11 Элемент массива: goods[3].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[3].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[3].g44_docs[1].doc_number | 26HL-1103 | CP | номер документа | |
| 05 | goods[3].g44_docs[1].doc_date | 31.03.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[2].doc_code | 04021 | CP | код документа | |
| 02 | goods[3].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[2].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[3].g44_docs[2].doc_number | 25HL-1083 | CP | номер документа | |
| 05 | goods[3].g44_docs[2].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[3].doc_code | 04131 | CP | код документа | |
| 02 | goods[3].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[3].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[3].g44_docs[3].doc_number | 25HL-1083/1 | CP | номер документа | |
| 05 | goods[3].g44_docs[3].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[4].doc_code | 02015 | CP | код документа | |
| 02 | goods[3].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[4].doc_name | CMR | CP | наименование документа | |
| 04 | goods[3].g44_docs[4].doc_number | 18614 | CP | номер документа | |
| 05 | goods[3].g44_docs[4].doc_date | 15.07.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[5].doc_code | 09013 | CP | код документа | |
| 02 | goods[3].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[5].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | |
| 04 | goods[3].g44_docs[5].doc_number | 10719110/180726/5122642 | CP | номер документа | |
| 05 | goods[3].g44_docs[5].doc_date | 18.07.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[6].doc_code | 04023 | CP | код документа | |
| 02 | goods[3].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[6].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[6].doc_number | 9 | CP | номер документа | |
| 05 | goods[3].g44_docs[6].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[7].doc_code | 04031 | CP | код документа | |
| 02 | goods[3].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[7].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[3].g44_docs[7].doc_number | 26-18614-tl | CP | номер документа | |
| 05 | goods[3].g44_docs[7].doc_date | 15.07.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[8].doc_code | 04111 | CP | код документа | |
| 02 | goods[3].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[8].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[3].g44_docs[8].doc_number | 26-18614-tl/1 | CP | номер документа | |
| 05 | goods[3].g44_docs[8].doc_date | 30.06.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[9].doc_code | 05999 | CP | код документа | |
| 02 | goods[3].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[9].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[9].doc_number | БН | CP | номер документа | |
| 05 | goods[3].g44_docs[9].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[10].doc_code | 04011 | CP | код документа | |
| 02 | goods[3].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[10].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[3].g44_docs[10].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | |
| 05 | goods[3].g44_docs[10].doc_date | 14.07.2025 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[11].doc_code | 11001 | CP | код документа | |
| 02 | goods[3].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[11].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[3].g44_docs[11].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[3].g44_docs[11].doc_date | 11.03.2010 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[12].doc_code | 11004 | CP | код документа | |
| 02 | goods[3].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[12].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[3].g44_docs[12].doc_number | 1 | CP | номер документа | |
| 05 | goods[3].g44_docs[12].doc_date | 01.02.2026 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[13].doc_code | 04033 | CP | код документа | |
| 02 | goods[3].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[13].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[3].g44_docs[13].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[3].g44_docs[13].doc_date | 13.05.2025 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[14].doc_code | 09023 | CP | код документа | |
| 02 | goods[3].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[14].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[3].g44_docs[14].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[3].g44_docs[14].doc_date | 20.08.2025 | CP | дата документа | |
- _item_audit: 5

##### 3.16.11 Элемент массива: goods[3].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[3].g44_docs[15].doc_code | 09999 | CP | код документа | |
| 02 | goods[3].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[3].g44_docs[15].doc_name | ОТКАЗНОЕ ПИСЬМО | CP | наименование документа | |
| 04 | goods[3].g44_docs[15].doc_number | 24968/МЛ10 | CP | номер документа | |
| 05 | goods[3].g44_docs[15].doc_date | 20.08.2025 | CP | дата документа | |
- _item_audit: 5

### Теги после товаров и документов (графы 51–54)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | representative.date | 2026-07-29 | D | дата заполнения/подачи ДТ | |
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
| 12 | representative.passport_code | RU01001 | CP | код удостоверения личности | |
| 13 | representative.passport_name | ПАСРФ | CP | наименование документа | |
| 14 | representative.passport_number | 449948 | CP | номер паспорта | |
| 15 | representative.passport_date | 11.03.2010 | CP | дата выдачи паспорта | |
| 16 | representative.passport_series | 63 09 | CP | серия паспорта | |
| 17 | representative.passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CP | кем выдан | |
- _audit: 17

### Итог:
- `dt_status`: confirmed

## Часть II: Issues (нерешенные вопросы)

**Для полей:**
Нет.

**Для общих вопросов:**
- `[Общий]`
  - `question`: Отчет СВХ (ДО-1 / ДО-2) отсутствует в первичных документах. Данные по весам и местам были сопоставлены кросс-док по Инвойсу, Упаковочному листу и CMR, однако при поступлении ДО-1/ДО-2 потребуется провести повторную сверку.
