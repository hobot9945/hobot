# Первичные данные

## 1. meta:
- `название кейса`: ПриточнаяВентиляция
- `путь к папке поставки`: alta\source\ПриточнаяВентиляция\03\
- `direction`: ИМ
- `тип поставки`: 1 ДТ
- `источники данных:` md + operator_provided_data.md + master_data.md

## 2. formalized/master_data/non_formalized:

### `document`: Contract
  - `uqi_prefix`: master_data.contract
  - `path`: alta\source\ПриточнаяВентиляция\master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 03011 | CD | константа | |
| 03 | doc_name | КОНТРАКТ | CD | константа | |
| 04 | doc_number | 25AZC003 | CD | номер контракта | master_data.md |
| 05 | doc_date | 10.04.2025 | CD | дата контракта | master_data.md |
- _audit: 5
- `doc_status`: confirmed

### `document`: Supplementary Contract
  - `uqi_prefix`: master_data.supplementary_contract_1
  - `path`: alta\source\ПриточнаяВентиляция\master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 03012 | CD | константа | |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | константа | |
| 04 | doc_number | 1 | CD | номер доп. соглашения | master_data.md |
| 05 | doc_date | 18.03.2026 | CD | дата доп. соглашения | master_data.md |
- _audit: 5
- `doc_status`: confirmed

### `document`: EGRUL
  - `uqi_prefix`: master_data.egrul
  - `path`: alta\source\ПриточнаяВентиляция\master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование организации | master_data.md |
| 02 | ShortName | ООО "СКИФ" | CD | краткое наименование | master_data.md |
| 03 | OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 04 | INN | 1650389298 | CD | ИНН | master_data.md |
| 05 | KPP | 165001001 | CD | КПП | master_data.md |
| 06 | Address_PostalCode | 423800 | CD | индекс | master_data.md |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 08 | Address_CounryName | РОССИЯ | CD | страна текст | master_data.md |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data.md |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data.md |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом | master_data.md |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | master_data.md |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | master_data.md |
| 14 | doc_gr44 | true | CD | константа | |
| 15 | doc_code | 04011 | CD | константа | |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | константа | |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | master_data.md |
| 18 | doc_date | 14.07.2025 | CD | дата документа | master_data.md |
- _audit: 18
- `doc_status`: confirmed

### `document`: Personal Passport
  - `uqi_prefix`: master_data.passport
  - `path`: alta\source\ПриточнаяВентиляция\master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | master_data.md |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | master_data.md |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | master_data.md |
| 04 | CardSeries | 63 09 | CD | серия | master_data.md |
| 05 | CardNumber | 449948 | CD | номер | master_data.md |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | master_data.md |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data.md |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_data.md |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | master_data.md |
| 10 | doc_gr44 | true | CD | константа | |
| 11 | doc_code | 11001 | CD | константа | |
| 12 | doc_name | ПАСПОРТ | CD | константа | |
| 13 | doc_number | 63 09 449948 | CD | номер документа | master_data.md |
| 14 | doc_date | 11.03.2010 | CD | дата документа | master_data.md |
- _audit: 14
- `doc_status`: confirmed
### `document`: Letter of Attorney
  - `uqi_prefix`: master_data.letter_of_attorney
  - `path`: alta\source\ПриточнаяВентиляция\master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data.md |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_data.md |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_data.md |
| 05 | doc_gr44 | true | CD | константа | |
| 06 | doc_code | 11004 | CD | константа | |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | константа | |
| 08 | doc_number | 1 | CD | номер документа | master_data.md |
| 09 | doc_date | 01.02.2026 | CD | дата документа | master_data.md |
- _audit: 9
- `doc_status`: confirmed

### `document`: Transport Contract
  - `uqi_prefix`: master_data.transport_contract
  - `path`: alta\source\ПриточнаяВентиляция\master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 04033 | CD | константа | |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | константа | |
| 04 | doc_number | КООО/26651/М | CD | номер документа | master_data.md |
| 05 | doc_date | 13.05.2025 | CD | дата документа | master_data.md |
- _audit: 5
- `doc_status`: confirmed
### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: alta\source\ПриточнаяВентиляция\03\md\Инвойс 26AZ4058.md
  - `file_name`: Инвойс 26AZ4058.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | CNY | CD | валюта инвойса | |
| 02 | DocumentCode | 04021 | CD | код вида документа | |
| 03 | PlacesQuantity | 2 | CD | кол-во грузовых мест | copied_from:formalized.packing_list |
| 04 | PlacesDescription | палеты | CD | описание мест | copied_from:formalized.packing_list |
| 05 | GrossWeightQuantity | 958.000 | CD | общий вес брутто | copied_from:formalized.packing_list |
| 06 | NetWeightQuantity | 906.520 | CD | общий вес нетто | copied_from:formalized.packing_list |
| 07 | GCost | 38106.80 | CD | системная стоимость | |
| 08 | TotalCost | 38106.80 | CD | полная стоимость | |
| 09 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CD | место поставки | |
| 10 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | условия поставки | |
| 11 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 12 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 13 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 14 | Registration_PrDocumentName | ИНВОЙС | CD | наименование документа | |
| 15 | Registration_PrDocumentNumber | 26AZ4058 | CD | номер инвойса | |
| 16 | Registration_PrDocumentDate | 12.05.2026 | CD | дата инвойса | |
| 17 | Contract_PrDocumentNumber | 25AZC003 | CD | номер контракта | |
| 18 | Contract_PrDocumentDate | 10.04.2025 | CD | дата контракта | |
| 19 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | |
| 20 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | |
| 21 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование покупателя | master_data.md |
| 22 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | master_data.md |
| 23 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | master_data.md |
| 24 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя текст | master_data.md |
| 25 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион покупателя | master_data.md |
| 26 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город покупателя | master_data.md |
| 27 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом покупателя | master_data.md |
| 28 | Seler_Name | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | наименование продавца | |
| 29 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | |
| 30 | Seler_PostalAddress_CounryName | CHINA | CD | страна продавца текст | |
| 31 | Seler_PostalAddress_Region | ОТСУТСТВУЕТ | CD | регион продавца | |
| 32 | Seler_PostalAddress_City | Ningbo | CD | город продавца | |
| 33 | Seler_PostalAddress_StreetHouse | D4-109, Liangzhu Culture Park, Haishu District | CD | улица/дом продавца | |
| 34 | Consignor_OrganizationName | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | грузоотправитель | seller=consignor |
| 35 | Consignor_Address_CountryCode | CN | CD | страна отправителя | seller=consignor |
| 36 | Consignor_Address_CounryName | CHINA | CD | страна отправителя текст | seller=consignor |
| 37 | Consignor_Address_Region | ОТСУТСТВУЕТ | CD | регион отправителя | seller=consignor |
| 38 | Consignor_Address_City | Ningbo | CD | город отправителя | seller=consignor |
| 39 | Consignor_Address_StreetHouse | D4-109, Liangzhu Culture Park, Haishu District | CD | улица/дом отправителя | seller=consignor |
| 40 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 41 | Consignee_OGRN | 1201600020390 | CD | ОГРН получателя | master_data.md |
| 42 | Consignee_INN | 1650389298 | CD | ИНН получателя | master_data.md |
| 43 | Consignee_KPP | 165001001 | CD | КПП получателя | master_data.md |
| 44 | Consignee_Address_PostalCode | 423800 | CD | индекс получателя | master_data.md |
| 45 | Consignee_Address_CountryCode | RU | CD | страна получателя | master_data.md |
| 46 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя текст | master_data.md |
| 47 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data.md |
| 48 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data.md |
| 49 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом получателя | master_data.md |
| 50 | doc_gr44 | true | CD | константа | |
| 51 | doc_code | 04021 | CD | константа | |
| 52 | doc_name | ИНВОЙС | CD | константа | |
| 53 | doc_number | 26AZ4058 | CD | номер документа | |
| 54 | doc_date | 12.05.2026 | CD | дата документа | |
- _audit: 54

#### formalized.invoice_1 Массив: InvoiceGoods[5]
- _array_audit: 5

#### formalized.invoice_1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7616910000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Пластиковый воздухозаборник для вентиляции ELC-100 маленький | CD | описание товара | |
| 03 | GoodsQuantity | 990 | CD | кол-во в осн. ед. | |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп. ед. | |
| 05 | goods_supplementary_uom_name | | CD | ед. доп. наименование | |
| 06 | MeasureUnitQualifierName | | CD | ед. доп. классификатор | |
| 07 | GrossWeightQuantity | 131.176 | CD | брутто позиции | copied_from:formalized.packing_list.Goods[1] |
| 08 | NetWeightQuantity | 125.400 | CD | нетто позиции | copied_from:formalized.packing_list.Goods[1] |
| 09 | Price | 5.20 | CD | цена | |
| 10 | TotalCost | 5148.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | ELC-100 small | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | |
- _item_audit: 17
#### formalized.invoice_1 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7616910000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Литой алюминий Вентиляционная решетка ELC-125 маленький | CD | описание товара | |
| 03 | GoodsQuantity | 3996 | CD | кол-во в осн. ед. | |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп. ед. | |
| 05 | goods_supplementary_uom_name | | CD | ед. доп. наименование | |
| 06 | MeasureUnitQualifierName | | CD | ед. доп. классификатор | |
| 07 | GrossWeightQuantity | 643.586 | CD | брутто позиции | copied_from:formalized.packing_list.Goods[2] |
| 08 | NetWeightQuantity | 607.540 | CD | нетто позиции | copied_from:formalized.packing_list.Goods[2] |
| 09 | Price | 6.50 | CD | цена | |
| 10 | TotalCost | 25974.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | ELC-125 small | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции в товаре | |
- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7616910000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Литой алюминий Вентиляционная решетка ELC-150 маленький | CD | описание товара | |
| 03 | GoodsQuantity | 144 | CD | кол-во в осн. ед. | |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп. ед. | |
| 05 | goods_supplementary_uom_name | | CD | ед. доп. наименование | |
| 06 | MeasureUnitQualifierName | | CD | ед. доп. классификатор | |
| 07 | GrossWeightQuantity | 34.283 | CD | брутто позиции | copied_from:formalized.packing_list.Goods[3] |
| 08 | NetWeightQuantity | 32.560 | CD | нетто позиции | copied_from:formalized.packing_list.Goods[3] |
| 09 | Price | 9.05 | CD | цена | |
| 10 | TotalCost | 1303.20 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | ELC-150 small | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции в товаре | |
- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7616910000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Литой алюминий Вентиляционная решетка ELC-200 маленький | CD | описание товара | |
| 03 | GoodsQuantity | 156 | CD | кол-во в осн. ед. | |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп. ед. | |
| 05 | goods_supplementary_uom_name | | CD | ед. доп. наименование | |
| 06 | MeasureUnitQualifierName | | CD | ед. доп. классификатор | |
| 07 | GrossWeightQuantity | 60.993 | CD | брутто позиции | copied_from:formalized.packing_list.Goods[4] |
| 08 | NetWeightQuantity | 57.900 | CD | нетто позиции | copied_from:formalized.packing_list.Goods[4] |
| 09 | Price | 15.60 | CD | цена | |
| 10 | TotalCost | 2433.60 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | ELC-200 small | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции в товаре | |
- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7616910000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Литой алюминий Вентиляционная решетка ELC-250 маленький | CD | описание товара | |
| 03 | GoodsQuantity | 112 | CD | кол-во в осн. ед. | |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп. ед. | |
| 05 | goods_supplementary_uom_name | | CD | ед. доп. наименование | |
| 06 | MeasureUnitQualifierName | | CD | ед. доп. классификатор | |
| 07 | GrossWeightQuantity | 87.961 | CD | брутто позиции | copied_from:formalized.packing_list.Goods[5] |
| 08 | NetWeightQuantity | 83.120 | CD | нетто позиции | copied_from:formalized.packing_list.Goods[5] |
| 09 | Price | 29.00 | CD | цена | |
| 10 | TotalCost | 3248.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | ELC-250small | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 5 | CD | индекс позиции в товаре | |
- _item_audit: 17

- `doc_status`: confirmed
### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list
  - `xml_target_root`: AltaE2PACK
  - `path`: alta\source\ПриточнаяВентиляция\03\md\Упаковочный_решетки.md
  - `file_name`: Упаковочный_решетки.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 958.000 | CD | общий вес брутто | |
| 02 | NetWeightQuantity | 906.520 | CD | общий вес нетто | |
| 03 | Consignor_OrganizationName | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | грузоотправитель | |
| 04 | Consignor_ShortName | ZENTEC | CD | краткое имя отправителя | |
| 05 | Consignor_Address_CountryCode | CN | CD | страна отправителя | |
| 06 | Consignor_Address_CounryName | CHINA | CD | страна отправителя текст | |
| 07 | Consignor_Address_Region | ОТСУТСТВУЕТ | CD | регион отправителя | |
| 08 | Consignor_Address_City | Ningbo | CD | город отправителя | |
| 09 | Consignor_Address_StreetHouse | D4-109, Liangzhu Culture Park, Haishu District | CD | улица/дом отправителя | |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 11 | Consignee_ShortName | ООО "СКИФ" | CD | краткое имя получателя | master_data.md |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН получателя | master_data.md |
| 13 | Consignee_INN | 1650389298 | CD | ИНН получателя | master_data.md |
| 14 | Consignee_KPP | 165001001 | CD | КПП получателя | master_data.md |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс получателя | master_data.md |
| 16 | Consignee_Address_CountryCode | RU | CD | страна получателя | master_data.md |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя текст | master_data.md |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data.md |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data.md |
| 20 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом получателя | master_data.md |
| 21 | DeliveryTerms_DeliveryPlace | Ningbo | CD | место поставки | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | EXW |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | имя контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | 25AZC003 | CO | номер контракта | operator |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 10.04.2025 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | имя инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 26AZ4058 | CO | номер инвойса | operator |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 12.05.2026 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | имя упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | 26AZ4058/1 | CD | номер упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 12.05.2026 | CD | дата упаковочного | |
| 33 | doc_gr44 | true | CD | константа | |
| 34 | doc_code | 04131 | CD | константа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | константа | |
| 36 | doc_number | 26AZ4058/1 | CD | номер документа | |
| 37 | doc_date | 12.05.2026 | CD | дата документа | |
- _audit: 37

#### formalized.packing_list Массив: Goods[5]
- _array_audit: 5

#### formalized.packing_list Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Пластиковый воздухозаборник для вентиляции ELC-100 маленький | CD | описание строки | |
| 02 | GoodsQuantity | 6 | CD | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 131.176 | CD | брутто строки | |
| 04 | NetWeightQuantity | 125.400 | CD | нетто строки | |
| 05 | PakingQuantity | 6 | CD | кол-во упаковок | |
- _item_audit: 5
#### formalized.packing_list Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Литой алюминий Вентиляционная решетка ELC-125 маленький | CD | описание строки | |
| 02 | GoodsQuantity | 37 | CD | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 643.586 | CD | брутто строки | |
| 04 | NetWeightQuantity | 607.540 | CD | нетто строки | |
| 05 | PakingQuantity | 37 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Литой алюминий Вентиляционная решетка ELC-150 маленький | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 34.283 | CD | брутто строки | |
| 04 | NetWeightQuantity | 32.560 | CD | нетто строки | |
| 05 | PakingQuantity | 2 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Литой алюминий Вентиляционная решетка ELC-200 маленький | CD | описание строки | |
| 02 | GoodsQuantity | 3 | CD | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 60.993 | CD | брутто строки | |
| 04 | NetWeightQuantity | 57.900 | CD | нетто строки | |
| 05 | PakingQuantity | 3 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Литой алюминий Вентиляционная решетка ELC-250 маленький | CD | описание строки | |
| 02 | GoodsQuantity | 4 | CD | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 87.961 | CD | брутто строки | |
| 04 | NetWeightQuantity | 83.120 | CD | нетто строки | |
| 05 | PakingQuantity | 4 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Массив: TransportMeans[0]
- _array_audit: 0

- `doc_status`: confirmed
### `document`: CMR
  - `uqi_prefix`: formalized.cmr
  - `xml_target_root`: AltaE3CMR
  - `path`: alta\source\ПриточнаяВентиляция\03\md\СМР.md
  - `file_name`: СМР.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CD | язык документа | |
| 02 | CMR_Choice | 1 | CD | системный выбор | |
| 03 | RegistrationDocument_RegID | 17366 | CD | номер CMR | |
| 04 | RegistrationDocument_DateInf | 07.07.2026 | CD | дата CMR | |
| 05 | RegistrationDocument_Place | МАНЬЧЖУРИЯ | CD | место составления | |
| 06 | TrakingCargo_TakingCargoDate | 07.07.2026 | CD | дата принятия груза | |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CD | страна принятия | |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | CHINA | CD | страна принятия текст | |
| 09 | DeliveryPlace_CountryCode | RU | CD | страна доставки | |
| 10 | DeliveryPlace_CounryName | RUSSIA | CD | страна доставки текст | |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CD | место поставки | |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | условия поставки | |
| 13 | GoodsQuantity | 2 | CD | количество мест | |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 958 | CD | общий вес брутто | |
| 15 | CMRTransport_PrimeMoverStateSignID | С081ХО161 | CD | гос. номер тягача | визуальный перевод в кириллицу |
| 16 | CMRTransport_TrailerStateSignID | ЕУ457623 | CD | гос. номер прицепа | визуальный перевод в кириллицу |
| 17 | Consignor_NameInf | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | отправитель наименование | |
| 18 | Consignor_ShortName | ZENTEC | CD | отправитель краткое имя | |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна отправителя | |
| 20 | Consignor_Address_CounryName | CHINA | CD | страна отправителя текст | |
| 21 | Consignor_Address_Region | ОТСУТСТВУЕТ | CD | регион отправителя | |
| 22 | Consignor_Address_City | Ningbo | CD | город отправителя | |
| 23 | Consignor_Address_StreetHouse | D4-109, Liangzhu Culture Park, Haishu District | CD | улица/дом отправителя | |
| 24 | Consignor_Guarantee_OrganizationName | | CD | наименование гаранта | |
| 25 | Consignor_Guarantee_ShortName | | CD | краткое наименование гаранта | |
| 26 | Consignor_Guarantee_Address_CountryCode | | CD | страна гаранта | |
| 27 | Consignor_Guarantee_Address_CounryName | | CD | страна гаранта текст | |
| 28 | Consignor_Guarantee_Address_Region | | CD | регион гаранта | |
| 29 | Consignor_Guarantee_Address_City | | CD | город гаранта | |
| 30 | Consignor_Guarantee_Address_StreetHouse | | CD | улица/дом гаранта | |
| 31 | Consignee_NameInf | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | получатель наименование | master_data.md |
| 32 | Consignee_ShortName | ООО "СКИФ" | CD | получатель краткое имя | master_data.md |
| 33 | Consignee_OGRNID | 1201600020390 | CD | ОГРН получателя | master_data.md |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН получателя | master_data.md |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП получателя | master_data.md |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс получателя | master_data.md |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна получателя | master_data.md |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя текст | master_data.md |
| 39 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data.md |
| 40 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data.md |
| 41 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом получателя | master_data.md |
| 42 | doc_gr44 | true | CD | константа | |
| 43 | doc_code | 02015 | CD | константа | |
| 44 | doc_name | CMR | CD | константа | |
| 45 | doc_number | 17366 | CD | номер документа | |
| 46 | doc_date | 07.07.2026 | CD | дата документа | |
- _audit: 46

#### formalized.cmr Массив: CMRGoods[1]
- _array_audit: 1

#### formalized.cmr Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер | |
| 02 | GoodsDescription | ЛИТОЙ АЛЮМИНИЙ ВЕНТИЛЯЦИОННАЯ РЕШЕТКА | CD | описание груза | |
| 03 | PakingQuantity | 2 | CD | количество мест | |
- _item_audit: 3

- `doc_status`: confirmed
### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\ПриточнаяВентиляция\03\md\mt103_10.md
  - `file_name`: mt103_10.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 1 | CD | способ платежа | |
| 03 | PaymentAmount | 19053.40 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN ALUMINUM AIR VENT. CONTRACT NO.: 25AZC003, DATE: APR 10, 2025, INVOICE NO: 26AZ4058, DATE: 12.05.2026 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Девятнадцать тысяч пятьдесят три юаня 40 фыней | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 10 | CD | номер платежки | |
| 09 | DocumentReference_PrDocumentDate | 14.05.2026 | CD | дата платежки | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | |
| 13 | Payer_Bank_BankName | Филиал "Центральный" Банка ВТБ (ПАО) | CD | банк плательщика | |
| 14 | Payee_OrganizationName | NINGBO ZENTEC AIR CONDITIONING AND REFRIGERATION CO., LTD | CD | получатель | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия | |
| 17 | PersonName | Дмитрий | CD | имя | |
| 18 | doc_gr44 | true | CD | константа | |
| 19 | doc_code | 04023 | CD | константа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | константа | |
| 21 | doc_number | 10 | CD | номер документа | |
| 22 | doc_date | 14.05.2026 | CD | дата документа | |
- _audit: 22
- `doc_status`: confirmed

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\ПриточнаяВентиляция\03\md\mt103_13.md
  - `file_name`: mt103_13.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 1 | CD | способ платежа | |
| 03 | PaymentAmount | 19053.40 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN ALUMINUM AIR VENT. CONTRACT NO.: 25AZC003, DATE: APR 10, 2025, INVOICE NO: 26AZ4058, DATE: 12.05.2026 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Девятнадцать тысяч пятьдесят три юаня 40 фыней | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 13 | CD | номер платежки | |
| 09 | DocumentReference_PrDocumentDate | 16.06.2026 | CD | дата платежки | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | |
| 13 | Payer_Bank_BankName | Филиал "Центральный" Банка ВТБ (ПАО) | CD | банк плательщика | |
| 14 | Payee_OrganizationName | NINGBO ZENTEC AIR CONDITIONING AND REFRIGERATION CO., LTD | CD | получатель | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия | |
| 17 | PersonName | Дмитрий | CD | имя | |
| 18 | doc_gr44 | true | CD | константа | |
| 19 | doc_code | 04023 | CD | константа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | константа | |
| 21 | doc_number | 13 | CD | номер документа | |
| 22 | doc_date | 16.06.2026 | CD | дата документа | |
- _audit: 22
- `doc_status`: confirmed
### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice
  - `xml_target_root`: AltaServiceInvoice
  - `path`: alta\source\ПриточнаяВентиляция\03\md\Счет_№26-17336-tl_от_23-06-2026 (1).md
  - `file_name`: Счет_№26-17336-tl_от_23-06-2026 (1).md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CD | системный признак | |
| 02 | TotalServiceCost | 1330.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого | |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг | |
| 05 | BankName | АО "Райффайзенбанк" | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги | |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги | |
| 08 | PrDocumentNumber | 26-17336-tl | CD | номер связанного док | |
| 09 | PrDocumentDate | 17.06.2026 | CD | дата связанного док | |
| 10 | Registration_PrDocumentName | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | 26-17336-tl | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 23.06.2026 | CD | дата счета | |
| 13 | Consignor_OrganizationName | BESTWILL SUPPLY CHAIN (NINGBO) CO.,LTD | CD | грузоотправитель | |
| 14 | PostalCode | ОТСУТСТВУЕТ | CD | индекс отправителя | |
| 15 | CountryCode | CN | CD | страна отправителя | |
| 16 | CounryName | CHINA | CD | страна отправителя текст | |
| 17 | Region | ОТСУТСТВУЕТ | CD | регион отправителя | |
| 18 | Town | Ningbo | CD | город отправителя | |
| 19 | StreetHouse | ОТСУТСТВУЕТ | CD | улица отправителя | |
| 20 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CD | ОГРН получателя | master_data.md |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН получателя | master_data.md |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП получателя | master_data.md |
| 24 | PostalCode | 423800 | CD | индекс получателя | master_data.md |
| 25 | CountryCode | RU | CD | страна получателя | master_data.md |
| 26 | CounryName | РОССИЯ | CD | страна получателя текст | master_data.md |
| 27 | Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data.md |
| 28 | Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data.md |
| 29 | StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица получателя | master_data.md |
| 30 | House | 30 | CD | дом получателя | master_data.md |
| 31 | Room | 211 | CD | офис получателя | master_data.md |
| 32 | Signature_Choice | 2 | CD | вариант подписи | |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | |
| 34 | IndividualEntrepreneur_PersonName | | CD | имя ИП | |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CD | фамилия руководителя | |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л. | CD | имя руководителя | |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А. | CD | отчество руководителя | |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О. | CD | имя бухгалтера | |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А. | CD | отчество бухгалтера | |
| 42 | doc_gr44 | true | CD | константа | |
| 43 | doc_code | 04031 | CD | константа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | константа | |
| 45 | doc_number | 26-17336-tl | CD | номер документа | |
| 46 | doc_date | 23.06.2026 | CD | дата документа | |
| 47 | transport_to_border | 692.00 | CD | стоимость до границы | |
| 48 | transport_currency | USD | CD | валюта стоимости до границы | |
- _audit: 48

#### formalized.service_invoice Массив: ServiceDescription[2]
- _array_audit: 2

#### formalized.service_invoice Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-17336-tl от 17.06.2026 по маршруту: China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) | CD | маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 692.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |
- _item_audit: 7

#### formalized.service_invoice Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-17336-tl от 17.06.2026 по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, г. Набережные челны | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | граница РФ (п/п Маньчжурия/Забайкальск) - Россия, г. Набережные челны | CD | маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 638.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |
- _item_audit: 7

- `doc_status`: confirmed
### `document`: Insurance Invoice
  - `uqi_prefix`: formalized.insurance_invoice
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\ПриточнаяВентиляция\03\md\Счет_№26-17336-tl_1_от_22-06-2026.md
  - `file_name`: Счет_№26-17336-tl_1_от_22-06-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 22.06.2026 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | 26-17336-tl/1 | CD | номер документа | |
| 05 | TextPara | link:alta\source\ПриточнаяВентиляция\03\md\Счет_№26-17336-tl_1_от_22-06-2026.md | CD | ссылка на источник | |
| 06 | doc_gr44 | true | CD | константа | |
| 07 | doc_code | 04111 | CD | константа | |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | константа | |
| 09 | doc_number | 26-17336-tl/1 | CD | номер документа | |
| 10 | doc_date | 22.06.2026 | CD | дата документа | |
| 11 | insurance_to_border | 341.79 | CD | стоимость страхования | |
| 12 | insurance_currency | RUB | CD | валюта страхования | |
- _audit: 12
- `doc_status`: confirmed

### `document`: Tech Description
  - `uqi_prefix`: formalized.tech_description
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\ПриточнаяВентиляция\03\md\техничка_РЕШЕТКА_АЛ.md
  - `file_name`: техничка_РЕШЕТКА_АЛ.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКИЕ ХАРАКТЕРИСТИКИ | CD | наименование техописания | |
| 03 | DocumentHead_DocumentDate | 12.05.2026 | CD | дата техописания | дата инвойса |
| 04 | DocumentHead_DocumentNumber | БН | CD | номер техописания | |
| 05 | TextPara | link:alta\source\ПриточнаяВентиляция\03\md\техничка_РЕШЕТКА_АЛ.md | CD | ссылка на источник | |
| 06 | doc_gr44 | true | CD | константа | |
| 07 | doc_code | 05999 | CD | константа | |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | константа | |
| 09 | doc_number | БН | CD | номер документа | |
| 10 | doc_date | 12.05.2026 | CD | дата документа | |
- _audit: 10
- `doc_status`: confirmed

### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.td
  - `path`: alta\source\ПриточнаяВентиляция\03\md\Транзитка_10719110_100726_5116051_reg.md
  - `file_name`: Транзитка_10719110_100726_5116051_reg.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs_post_code | 10719110 | CD | код таможни | MAPП Забайкальск |
| 02 | customs_post_name | т/п МАПП Забайкальск | CD | наименование таможни | |
| 03 | transport_reg_number | С081ХО161/ЕУ457623 | CD | номера ТС | |
| 04 | doc_gr44 | true | CD | константа | |
| 05 | doc_code | 09013 | CD | константа | |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | константа | |
| 07 | doc_number | 10719110/100726/5116051 | CD | номер документа | |
| 08 | doc_date | 10.07.2026 | CD | дата документа | |
- _audit: 8
- `doc_status`: confirmed
### `document`: Goods Description
  - `uqi_prefix`: non_formalized.goods_description_1
  - `path`: alta\source\ПриточнаяВентиляция\03\md\техничка_РЕШЕТКА_АЛ.md
  - `file_name`: техничка_РЕШЕТКА_АЛ.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | константа | |
- _audit: 1

#### non_formalized.goods_description_1 Массив: goods[1]
- _array_audit: 1

#### non_formalized.goods_description_1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 7616910000 | CD | код ТН ВЭД | |
| 02 | description | ВЕНТИЛЯЦИОННЫЕ РЕШЕТКИ КРУГЛЫЕ АЛЮМИНИЕВЫЕ С ФЛАНЦЕМ, С НАКЛОННЫМИ ЖАЛЮЗИ, ИЗГОТОВЛЕНЫ ИЗ АЛЮМИНИЕВОГО СПЛАВА МЕТОДОМ ЛИТЬЯ ПОД ДАВЛЕНИЕМ, ПРЕДНАЗНАЧЕНЫ ДЛЯ ИСПОЛЬЗОВАНИЯ В СИСТЕМАХ ВЕНТИЛЯЦИИ, КОНДИЦИОНИРОВАНИЯ И ОТОПЛЕНИЯ ЗДАНИЙ. | CD | описание товара | |
- _item_audit: 2

- `doc_status`: confirmed
### Итого, по файлу:

`total_unreliable_fields`: 0
`primary_status`: confirmed

## 5. Нерешенные вопросы (Issues)

**Для общих вопросов:**
- `[Общий]`
  - `question`: [WARNING] КРИТИЧЕСКИЙ ВЕС ТАРЫ (КАРАУЛ!) - Общий вес нетто по данным поставщика (906.520 кг) превышает расчетный чистый вес оборудования по техническим характеристикам (820.496 кг) более чем на 8% (отклонение составляет 10.43%). Возможен избыточный вес индивидуальной упаковки либо расхождение в техническом описании, однако данные нетто из упаковочного листа подтверждены и используются напрямую.
- `[Общий]`
  - `question`: Уникальный номер контракта (УНК) отсутствует в мастер-данных и первичных документах. Декларация формируется без указания УНК.
- `[Общий]`
  - `question`: Отчет СВХ (ДО-1/ДО-2) отсутствует в комплекте документов поставки. Документ non_formalized.svh не генерировался. Адрес СВХ и номер лицензии извлечены из CMR для дальнейшего заполнения графы 30.

## 6. `unreliable_fields`:

