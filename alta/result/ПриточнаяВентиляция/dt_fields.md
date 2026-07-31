# Исходные данные для ДТ

## Метаданные:
- `название кейса`: ПриточнаяВентиляция
- `путь к папке поставки`: alta\source\ПриточнаяВентиляция\03\
- `тип поставки`: 1 ДТ / 1 товар
- `агрегация ДТ`: 5 позиций инвойса сгруппированы в 1 товар ТН ВЭД 7616910000
- `источники данных:` primary.md + operator_provided_data.md + master_data.md

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
| 01 | sender.country_name | CHINA | CP | страна отправителя текст | |
| 02 | sender.country_code | CN | CP | код страны отправителя | |
| 03 | sender.name | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CP | наименование отправителя | |
| 04 | sender.region | ОТСУТСТВУЕТ | CP | регион отправителя | |
| 05 | sender.city | Ningbo | CP | город отправителя | |
| 06 | sender.street | D4-109, Liangzhu Culture Park, Haishu District | CP | улица/дом отправителя | |
- _audit: 6

### 3.3 Количество товаров и мест (графы 5, 6)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.total_goods_number | 1 | D | количество товаров | |
| 02 | shipment.packages_flag | true | D | признак подсчета мест | |
| 03 | shipment.total_packages | 2 | D | общее количество мест | 2 палеты из CMR/ТД |
- _audit: 3

### 3.4 Получатель (графа 8)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | consignee.same_as_declarant | true | D | признак см. графу 14 | |
| 02 | consignee.ogrn | 1201600020390 | D | ОГРН получателя | |
| 03 | consignee.inn_kpp | 1650389298/165001001 | D | ИНН/КПП получателя | |
| 04 | consignee.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование получателя | |
| 05 | consignee.country_code | RU | D | код страны получателя | |
| 06 | consignee.country_name | РОССИЯ | D | страна получателя текст | |
| 07 | consignee.postcode | 423800 | D | индекс получателя | |
| 08 | consignee.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион получателя | |
| 09 | consignee.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | город получателя | |
| 10 | consignee.street | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | D | улица получателя | |
| 11 | consignee.building | 30 | D | дом получателя | |
| 12 | consignee.room | 211 | D | офис получателя | |
| 13 | consignee.phone | +7 (843) 207 18 90 | D | телефон получателя | |
| 14 | consignee.email | PROM_TAT@MAIL.RU | D | e-mail получателя | |
- _audit: 14

### 3.5 Финансовое урегулирование (графа 9)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | financial.same_as_declarant | true | D | признак см. графу 14 | |
| 02 | financial.ogrn | 1201600020390 | D | ОГРН финансового регулирования | |
| 03 | financial.inn_kpp | 1650389298/165001001 | D | ИНН/КПП финансового регулирования | |
| 04 | financial.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | D | наименование лица | |
| 05 | financial.country_code | RU | D | код страны лица | |
| 06 | financial.country_name | РОССИЯ | D | наименование страны лица | |
| 07 | financial.postcode | 423800 | D | индекс лица | |
| 08 | financial.region | РЕСПУБЛИКА ТАТАРСТАН | D | регион лица | |
| 09 | financial.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | D | город лица | |
| 10 | financial.street | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | D | улица лица | |
| 11 | financial.building | 30 | D | дом лица | |
| 12 | financial.room | 211 | D | офис лица | |
| 13 | financial.country_code_alt | RU | D | дублирующий код страны | |
| 14 | financial.phone | +7 (843) 207 18 90 | D | телефон лица | |
| 15 | financial.email | PROM_TAT@MAIL.RU | D | e-mail лица | |
- _audit: 15

### 3.6 Торгующая страна (графа 11)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.trade_country_code | CN | CP | код торгующей страны | |
- _audit: 1

### 3.7 Декларант (графа 14)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | declarant.ogrn | 1201600020390 | CP | ОГРН декларанта | |
| 02 | declarant.inn_kpp | 1650389298/165001001 | D | ИНН/КПП декларанта | |
| 03 | declarant.name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CP | наименование декларанта | |
| 04 | declarant.country_code | RU | CP | код страны декларанта | |
| 05 | declarant.country_name | РОССИЯ | CP | страна декларанта текст | |
| 06 | declarant.postcode | 423800 | CP | индекс декларанта | |
| 07 | declarant.region | РЕСПУБЛИКА ТАТАРСТАН | CP | регион декларанта | |
| 08 | declarant.city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CP | город декларанта | |
| 09 | declarant.street | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CP | улица декларанта | |
| 10 | declarant.building | 30 | D | дом декларанта | |
| 11 | declarant.room | 211 | D | офис декларанта | |
| 12 | declarant.phone | +7 (843) 207 18 90 | CP | телефон декларанта | |
| 13 | declarant.email | PROM_TAT@MAIL.RU | CP | e-mail декларанта | |
- _audit: 13
### 3.8 Страны (графы 15, 16, 17)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.dispatch_country_code | CN | CP | код страны отправления | |
| 02 | shipment.destination_country_code | RU | CP | код страны назначения | |
| 03 | shipment.dispatch_country_name | КИТАЙ | D | страна отправления текст | |
| 04 | shipment.destination_country_name | РОССИЯ | D | страна назначения текст | |
| 05 | shipment.origin_country_code | CN | D | код страны происхождения | |
| 06 | shipment.origin_country_name | КИТАЙ | D | страна происхождения текст | |
- _audit: 6

### 3.9 Условия поставки (графа 20)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | delivery.terms_code | EXW | D | условия поставки | |
| 02 | delivery.place_name | Ningbo | D | место поставки | |
- _audit: 2

### 3.10 Транспорт (графы 18, 19, 21)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.vehicles_count | 1 | D | количество ТС | |
| 02 | transport.identification | С081ХО161/ЕУ457623 | D | гос. номера ТС | |
| 03 | transport.registration_country_code | RU | D | код страны регистрации ТС | |
| 04 | transport.container_flag | 0 | CO | признак контейнера | |
| 05 | transport.border_mode | 1 | D | количество ТС на границе | |
| 06 | transport.border_id | | D | номера ТС на границе | |
| 07 | transport.border_country_code | | D | код страны регистрации ТС на границе | |
- _audit: 7

### 3.11 Валюта и стоимость (графа 22)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.invoice_currency_numeric | 156 | D | цифровой код валюты | |
| 02 | shipment.invoice_currency_alpha | CNY | CP | буквенный код валюты | |
| 03 | shipment.invoice_amount | 38106.80 | CP | сумма по счету | |
- _audit: 3

### 3.12 Характер сделки (графа 24)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | shipment.transaction_nature | 010 | D | характер сделки | |
| 02 | shipment.transaction_feature | 06 | D | особенность сделки | без УНК |
- _audit: 2

### 3.13 Вид транспорта (графы 25, 26)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | transport.border_transport_code | 31 | D | код транспорта на границе | |
| 02 | transport.internal_transport_code | 31 | D | код внутреннего транспорта | |
- _audit: 2

### 3.14 Таможня на границе (графа 29)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs.border_code | 10719110 | CP | код таможни на границе | |
| 02 | customs.border_name | т/п МАПП Забайкальск | CP | наименование таможни на границе | |
- _audit: 2

### 3.15 Местонахождение товаров (графа 30)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | location.document_number | 10404/141210/10092/5 | CP | номер документа СВХ | |
- _audit: 1
### 3.16 Массив: goods[]
- goods._array_audit: 1

#### 3.16.0 Элемент массива: goods[1]
- goods._element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g31.name | ВЕНТИЛЯЦИОННЫЕ РЕШЕТКИ КРУГЛЫЕ АЛЮМИНИЕВЫЕ С ФЛАНЦЕМ, С НАКЛОННЫМИ ЖАЛЮЗИ, РАЗМЕРЫ 150Х150 ММ, ГЛУБИНА 19 ММ, ИЗГОТОВЛЕНЫ ИЗ АЛЮМИНИЕВОГО СПЛАВА МЕТОДОМ ЛИТЬЯ ПОД ДАВЛЕНИЕМ, ПРЕДНАЗНАЧЕНЫ ДЛЯ ИСПОЛЬЗОВАНИЯ В СИСТЕМАХ ВЕНТИЛЯЦИИ, КОНДИЦИОНИРОВАНИЯ И ОТОПЛЕНИЯ ЗДАНИЙ. СМ.ДОПОЛНЕНИЕ | D | описание товара | |
| 02 | goods[1].g31.manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | |
| 03 | goods[1].g31.trade_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 04 | goods[1].places | 2 | D | количество мест по товару | |
- _audit: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].item_no | 1 | D | номер товара в ДТ | |
| 02 | goods[1].tnved_code | 7616910000 | D | код ТН ВЭД товара | |
| 03 | goods[1].tnved.flag_1 | С | D | доп. признак (сертификация) | |
| 04 | goods[1].tnved.flag_2 | N | D | доп. признак (ТМ) | |
| 05 | goods[1].origin_country_code | CN | D | код страны происхождения | |
| 06 | goods[1].gross_weight | 958.000 | D | вес брутто по товару | |
| 07 | goods[1].preference | ОООО-ОО | D | преференция | |
| 08 | goods[1].procedure_code | 4000000 | D | код процедуры по товару | |
| 09 | goods[1].net_weight | 906.520 | D | вес нетто по товару | |
| 10 | goods[1].supplementary_quantity | | D | кол-во в доп. ед. | |
| 11 | goods[1].supplementary_unit_code | | D | код доп. ед. | |
| 12 | goods[1].supplementary_unit_name | | D | наименование доп. ед. | |
- _audit: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].invoice_cost | 38106.80 | D | цена товара | |
| 02 | goods[1].transport_cost | 692.00 | D | доля транспортных расходов | |
| 03 | goods[1].transport_currency | USD | CP | валюта транспортных расходов | |
| 04 | goods[1].insurance_cost | 341.79 | D | доля расходов на страхование | |
| 05 | goods[1].insurance_currency | RUB | CP | валюта страхования | |
- _audit: 5
#### 3.16.4 Массив: goods[1].txt[]
- _array_audit: 5

#### 3.16.5 Элемент массива: goods[1].txt[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[1] | АРТ: -  Пластиковый воздухозаборник для вентиляции ELC-100 маленький | D | текстовое описание позиции | |
- _item_audit: 1

#### 3.16.5 Элемент массива: goods[1].txt[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[2] | АРТ: -  Литой алюминий Вентиляционная решетка ELC-125 маленький | D | текстовое описание позиции | |
- _item_audit: 1

#### 3.16.5 Элемент массива: goods[1].txt[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[3] | АРТ: -  Литой алюминий Вентиляционная решетка ELC-150 маленький | D | текстовое описание позиции | |
- _item_audit: 1

#### 3.16.5 Элемент массива: goods[1].txt[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[4] | АРТ: -  Литой алюминий Вентиляционная решетка ELC-200 маленький | D | текстовое описание позиции | |
- _item_audit: 1

#### 3.16.5 Элемент массива: goods[1].txt[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].txt[5] | АРТ: -  Литой алюминий Вентиляционная решетка ELC-250 маленький | D | текстовое описание позиции | |
- _item_audit: 1
#### 3.16.6 Массив: goods[1].tovg[]
- _array_audit: 5

#### 3.16.7 Элемент массива: goods[1].tovg[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[1].line_no | 1 | D | номер строки таблицы | |
| 02 | goods[1].tovg[1].description | Пластиковый воздухозаборник для вентиляции ELC-100 маленький | D | наименование | |
| 03 | goods[1].tovg[1].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | |
| 04 | goods[1].tovg[1].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[1].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[1].model | ELC-100 small | D | модель | |
| 07 | goods[1].tovg[1].quantity | | CP | кол-во в доп. ед. | |
| 08 | goods[1].tovg[1].unit_code | | D | код ЕИ | |
| 09 | goods[1].tovg[1].unit_name | | CP | наименование ЕИ | |
| 10 | goods[1].tovg[1].gross_weight | 131.176 | CP | брутто по строке | |
| 11 | goods[1].tovg[1].net_weight | 125.400 | CP | нетто по строке | |
| 12 | goods[1].tovg[1].invoice_cost | 5148.00 | CP | цена по строке | |
- _item_audit: 12

#### 3.16.7 Элемент массива: goods[1].tovg[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[2].line_no | 2 | D | номер строки таблицы | |
| 02 | goods[1].tovg[2].description | Литой алюминий Вентиляционная решетка ELC-125 маленький | D | наименование | |
| 03 | goods[1].tovg[2].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | |
| 04 | goods[1].tovg[2].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[2].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[2].model | ELC-125 small | D | модель | |
| 07 | goods[1].tovg[2].quantity | | CP | кол-во в доп. ед. | |
| 08 | goods[1].tovg[2].unit_code | | D | код ЕИ | |
| 09 | goods[1].tovg[2].unit_name | | CP | наименование ЕИ | |
| 10 | goods[1].tovg[2].gross_weight | 643.586 | CP | брутто по строке | |
| 11 | goods[1].tovg[2].net_weight | 607.540 | CP | нетто по строке | |
| 12 | goods[1].tovg[2].invoice_cost | 25974.00 | CP | цена по строке | |
- _item_audit: 12

#### 3.16.7 Элемент массива: goods[1].tovg[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[3].line_no | 3 | D | номер строки таблицы | |
| 02 | goods[1].tovg[3].description | Литой алюминий Вентиляционная решетка ELC-150 маленький | D | наименование | |
| 03 | goods[1].tovg[3].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | |
| 04 | goods[1].tovg[3].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[3].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[3].model | ELC-150 small | D | модель | |
| 07 | goods[1].tovg[3].quantity | | CP | кол-во в доп. ед. | |
| 08 | goods[1].tovg[3].unit_code | | D | код ЕИ | |
| 09 | goods[1].tovg[3].unit_name | | CP | наименование ЕИ | |
| 10 | goods[1].tovg[3].gross_weight | 34.283 | CP | брутто по строке | |
| 11 | goods[1].tovg[3].net_weight | 32.560 | CP | нетто по строке | |
| 12 | goods[1].tovg[3].invoice_cost | 1303.20 | CP | цена по строке | |
- _item_audit: 12

#### 3.16.7 Элемент массива: goods[1].tovg[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[4].line_no | 4 | D | номер строки таблицы | |
| 02 | goods[1].tovg[4].description | Литой алюминий Вентиляционная решетка ELC-200 маленький | D | наименование | |
| 03 | goods[1].tovg[4].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | |
| 04 | goods[1].tovg[4].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[4].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[4].model | ELC-200 small | D | модель | |
| 07 | goods[1].tovg[4].quantity | | CP | кол-во в доп. ед. | |
| 08 | goods[1].tovg[4].unit_code | | D | код ЕИ | |
| 09 | goods[1].tovg[4].unit_name | | CP | наименование ЕИ | |
| 10 | goods[1].tovg[4].gross_weight | 60.993 | CP | брутто по строке | |
| 11 | goods[1].tovg[4].net_weight | 57.900 | CP | нетто по строке | |
| 12 | goods[1].tovg[4].invoice_cost | 2433.60 | CP | цена по строке | |
- _item_audit: 12

#### 3.16.7 Элемент массива: goods[1].tovg[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].tovg[5].line_no | 5 | D | номер строки таблицы | |
| 02 | goods[1].tovg[5].description | Литой алюминий Вентиляционная решетка ELC-250 маленький | D | наименование | |
| 03 | goods[1].tovg[5].manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | D | производитель | |
| 04 | goods[1].tovg[5].trade_mark | ОТСУТСТВУЕТ | D | марка/ТМ | |
| 05 | goods[1].tovg[5].goods_mark | ОТСУТСТВУЕТ | D | товарный знак | |
| 06 | goods[1].tovg[5].model | ELC-250small | D | модель | |
| 07 | goods[1].tovg[5].quantity | | CP | кол-во в доп. ед. | |
| 08 | goods[1].tovg[5].unit_code | | D | код ЕИ | |
| 09 | goods[1].tovg[5].unit_name | | CP | наименование ЕИ | |
| 10 | goods[1].tovg[5].gross_weight | 87.961 | CP | брутто по строке | |
| 11 | goods[1].tovg[5].net_weight | 83.120 | CP | нетто по строке | |
| 12 | goods[1].tovg[5].invoice_cost | 3248.00 | CP | цена по строке | |
- _item_audit: 12
### 3.16.8 Поле G_44

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44.text | СМ.ДОПОЛНЕНИЕ | D | текстовое поле графы 44 | |
- _audit: 1

#### 3.16.10 Массив: goods[1].g44_docs[]
- _array_audit: 15

#### 3.16.11 Элемент массива: goods[1].g44_docs[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[1].doc_code | 03011 | CP | код документа | |
| 02 | goods[1].g44_docs[1].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[1].doc_name | КОНТРАКТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[1].doc_number | 25AZC003 | CP | номер документа | |
| 05 | goods[1].g44_docs[1].doc_date | 10.04.2025 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[2].doc_code | 03012 | CP | код документа | |
| 02 | goods[1].g44_docs[2].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[2].doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[2].doc_number | 1 | CP | номер документа | |
| 05 | goods[1].g44_docs[2].doc_date | 18.03.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[3].doc_code | 04011 | CP | код документа | |
| 02 | goods[1].g44_docs[3].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[3].doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CP | наименование документа | |
| 04 | goods[1].g44_docs[3].doc_number | ЮЭ9965-25-106893283 | CP | номер документа | |
| 05 | goods[1].g44_docs[3].doc_date | 14.07.2025 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[4].doc_code | 11001 | CP | код документа | |
| 02 | goods[1].g44_docs[4].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[4].doc_name | ПАСПОРТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[4].doc_number | 63 09 449948 | CP | номер документа | |
| 05 | goods[1].g44_docs[4].doc_date | 11.03.2010 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[5].doc_code | 11004 | CP | код документа | |
| 02 | goods[1].g44_docs[5].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[5].doc_name | ДОВЕРЕННОСТЬ | CP | наименование документа | |
| 04 | goods[1].g44_docs[5].doc_number | 1 | CP | номер документа | |
| 05 | goods[1].g44_docs[5].doc_date | 01.02.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[6].doc_code | 04033 | CP | код документа | |
| 02 | goods[1].g44_docs[6].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[6].doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[6].doc_number | КООО/26651/М | CP | номер документа | |
| 05 | goods[1].g44_docs[6].doc_date | 13.05.2025 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[7].doc_code | 04021 | CP | код документа | |
| 02 | goods[1].g44_docs[7].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[7].doc_name | ИНВОЙС | CP | наименование документа | |
| 04 | goods[1].g44_docs[7].doc_number | 26AZ4058 | CP | номер документа | |
| 05 | goods[1].g44_docs[7].doc_date | 12.05.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[8].doc_code | 04131 | CP | код документа | |
| 02 | goods[1].g44_docs[8].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[8].doc_name | УПАКОВОЧНЫЙ ЛИСТ | CP | наименование документа | |
| 04 | goods[1].g44_docs[8].doc_number | 26AZ4058/1 | CP | номер документа | |
| 05 | goods[1].g44_docs[8].doc_date | 12.05.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[9].doc_code | 02015 | CP | код документа | |
| 02 | goods[1].g44_docs[9].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[9].doc_name | CMR | CP | наименование документа | |
| 04 | goods[1].g44_docs[9].doc_number | 17366 | CP | номер документа | |
| 05 | goods[1].g44_docs[9].doc_date | 07.07.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[10].doc_code | 04023 | CP | код документа | |
| 02 | goods[1].g44_docs[10].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[10].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[10].doc_number | 10 | CP | номер документа | |
| 05 | goods[1].g44_docs[10].doc_date | 14.05.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[11].doc_code | 04023 | CP | код документа | |
| 02 | goods[1].g44_docs[11].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[11].doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[11].doc_number | 13 | CP | номер документа | |
| 05 | goods[1].g44_docs[11].doc_date | 16.06.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[12].doc_code | 04031 | CP | код документа | |
| 02 | goods[1].g44_docs[12].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[12].doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[12].doc_number | 26-17336-tl | CP | номер документа | |
| 05 | goods[1].g44_docs[12].doc_date | 23.06.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[13].doc_code | 04111 | CP | код документа | |
| 02 | goods[1].g44_docs[13].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[13].doc_name | СЧЕТ ЗА СТРАХОВКУ | CP | наименование документа | |
| 04 | goods[1].g44_docs[13].doc_number | 26-17336-tl/1 | CP | номер документа | |
| 05 | goods[1].g44_docs[13].doc_date | 22.06.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[14].doc_code | 05999 | CP | код документа | |
| 02 | goods[1].g44_docs[14].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[14].doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CP | наименование документа | |
| 04 | goods[1].g44_docs[14].doc_number | БН | CP | номер документа | |
| 05 | goods[1].g44_docs[14].doc_date | 12.05.2026 | CP | дата документа | |
- _item_audit: 5

#### 3.16.11 Элемент массива: goods[1].g44_docs[15]
- _element_num: 15

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | goods[1].g44_docs[15].doc_code | 09013 | CP | код документа | |
| 02 | goods[1].g44_docs[15].kind_code | 0 | CO | признак записи | |
| 03 | goods[1].g44_docs[15].doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CP | наименование документа | |
| 04 | goods[1].g44_docs[15].doc_number | 10719110/100726/5116051 | CP | номер документа | |
| 05 | goods[1].g44_docs[15].doc_date | 10.07.2026 | CP | дата документа | |
- _item_audit: 5

- `doc_status`: confirmed
### 3.17 Теги после товаров (графы 51–54)

#### 3.17.1 Графа 54 — уполномоченное лицо / представитель

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | representative.date | 21.07.2026 | D | дата заполнения/подачи | текущая дата |
| 02 | representative.phone | +7 927-222-0500 | CP | телефон представителя | |
| 03 | representative.email | A.K.ARBUZOVA@YANDEX.RU | CP | e-mail представителя | |
| 04 | representative.last_name | АРБУЗОВА | CP | фамилия представителя | |
| 05 | representative.first_name | АНАСТАСИЯ | CP | имя представителя | |
| 06 | representative.middle_name | КОНСТАНТИНОВНА | CP | отчество представителя | |
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

### Часть II: Issues (нерешенные вопросы)

Для общих вопросов:
- `[Общий]`:
  - `question`: [WARNING] КРИТИЧЕСКИЙ ВЕС ТАРЫ (КАРАУЛ!) - Общий вес нетто по данным поставщика (906.520 кг) превышает чистый физический вес оборудования (820.496 кг) более чем на 8% (отклонение 10.43%). Данные нетто из PL перенесены напрямую.
- `[Общий]`:
  - `question`: Уникальный номер контракта (УНК) отсутствует в мастер-данных и первичных документах. Декларация формируется без указания УНК.
- `[Общий]`:
  - `question`: Отчет СВХ (ДО-1/ДО-2) отсутствует в комплекте документов поставки. Документ non_formalized.svh не генерировался. Номер лицензии СВХ (10404/141210/10092/5) извлечен из CMR.

## 6. `unreliable_fields`:

