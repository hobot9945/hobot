# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 7 позиций инвойса / 2 товара ДТ
- `источники данных`: md + operator_provided_data + master_keys + master_proto

## 2. formalized:

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\CL на сетку .md
  - `file_name`: CL на сетку .md

| num | field | value | status | description | note |
|---|---|---|--------|---|---|
| 01 | CurrencyRate | 10.9430 | CO     | курс валюты | operator:invoice_1.exchange_rate |
| 02 | CurrencyCode | CNY | CO     | валюта инвойса | operator:invoice_1.currency_code |
| 03 | DocumentCode | 04021 | CD     | код вида документа | константа |
| 04 | PlacesQuantity | 127 | CD     | кол-во грузовых мест | copied_from:md\CL на сетку .md |
| 05 | PlacesDescription | Поддон | CO     | описание мест | operator:invoice_1.places_description |
| 06 | GrossWeightQuantity | 3500.00 | CO     | общий вес брутто | operator:invoice_1.total_gross_weight |
| 07 | NetWeightQuantity | 3302.00 | CO     | общий вес нетто | operator:invoice_1.total_net_weight |
| 08 | GCost | 97260.00 | CO     | системное поле Альты | operator:invoice_1.gcost |
| 09 | TotalCost | 97260.00 | CO     | итого по инвойсу | operator:invoice_1.total_cost |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD     | место поставки | copied_from:md\CL на сетку .md |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO     | числовой код условий | operator:invoice_1.delivery_terms_numeric |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO     | строковый код условий | operator:invoice_1.delivery_terms_string |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO     | страна отправления | operator:invoice_1.dispatch_country_code |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO     | торгующая страна | operator:invoice_1.trading_country_code |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO     | страна назначения | operator:invoice_1.destination_country_code |
| 16 | Registration_PrDocumentName | Commercial invoice / Комерческий инвойс | CD     | наименование документа | copied_from:md\CL на сетку .md |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD     | номер инвойса | copied_from:md\CL на сетку .md |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD     | дата инвойса | copied_from:md\CL на сетку .md |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD     | № контракта-ссылки | copied_from:md\CL на сетку .md |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD     | дата контракта-ссылки | copied_from:md\CL на сетку .md |
| 21 | Buyer_CompanyID | 1650389298 | CD     | ИНН покупателя | master_proto.md (declarant.inn) |
| 22 | Buyer_KPPCode | 165001001 | CD     | КПП покупателя | master_proto.md (declarant.kpp) |
| 23 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD     | наименование покупателя | master_proto.md (declarant.organization_name) |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD     | индекс покупателя | master_proto.md (declarant.postal_code) |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD     | страна покупателя | master_proto.md (declarant.country_code) |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CD     | страна покупателя, текст | master_proto.md (declarant.country_name) |
| 27 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD     | регион | master_proto.md (declarant.region) |
| 28 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD     | город | master_proto.md (declarant.city) |
| 29 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD     | улица/дом/офис | master_proto.md (declarant.street_house) |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD     | продавец | copied_from:md\CL на сетку .md |
| 31 | Seler_PostalAddress_CountryCode | CN | CO     | страна продавца | operator:invoice_1.seller_country_code_alpha2 |
| 32 | Seler_PostalAddress_CounryName | China | CD     | страна продавца, текст | copied_from:md\CL на сетку .md |
| 33 | Seler_PostalAddress_Region | Hebei | CD     | регион продавца | copied_from:md\CL на сетку .md |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD     | город/район продавца | copied_from:md\CL на сетку .md |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD     | улица/дом | copied_from:md\CL на сетку .md |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO     | грузоотправитель | operator:invoice_1.consignor_equals_seller |
| 37 | Consignor_Address_CountryCode | CN | CO     | страна грузоотправителя | operator:invoice_1.seller_country_code_alpha2 |
| 38 | Consignor_Address_CounryName | China | CO     | страна грузоотправителя, текст | copied_from:md\CL на сетку .md |
| 39 | Consignor_Address_Region | Hebei | CO     | регион | copied_from:md\CL на сетку .md |
| 40 | Consignor_Address_City | Shijiazhuang | CO     | город/район | copied_from:md\CL на сетку .md |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CO     | улица/дом | copied_from:md\CL на сетку .md |
| 42 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO     | грузополучатель | operator:invoice_1.consignee_equals_buyer |
| 43 | Consignee_OGRN | 1201600020390 | CD     | ОГРН | master_proto.md (declarant.ogrn) |
| 44 | Consignee_INN | 1650389298 | CD     | ИНН | master_proto.md (declarant.inn) |
| 45 | Consignee_KPP | 165001001 | CD     | КПП | master_proto.md (declarant.kpp) |
| 46 | Consignee_Address_PostalCode | 423800 | CD     | индекс | master_proto.md (declarant.postal_code) |
| 47 | Consignee_Address_CountryCode | RU | CD     | страна | master_proto.md (declarant.country_code) |
| 48 | Consignee_Address_CounryName | РОССИЯ | CD     | страна, текст | master_proto.md (declarant.country_name) |
| 49 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD     | регион | master_proto.md (declarant.region) |
| 50 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD     | город | master_proto.md (declarant.city) |
| 51 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD     | улица/дом/офис | master_proto.md (declarant.street_house) |
| 52 | doc_gr44 | true | CD     | признак включения в гр.44 | константа |
| 53 | doc_code | 04021 | CD     | код документа | константа |
| 54 | doc_name | ИНВОЙС | CD     | наименование документа | константа |
| 55 | doc_number | LM-2591 | CD     | номер документа | copied_from:formalized.invoice_1.Registration_PrDocumentNumber |
| 56 | doc_date | 30.10.2025 | CD     | дата документа | copied_from:formalized.invoice_1.Registration_PrDocumentDate |
- _audit: 56

#### 3.1.1 Массив: InvoiceGoods[7]
- _array_audit: 7

#### 3.1.1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:md\CL на сетку .md |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара | copied_from:md\CL на сетку .md |
| 03 | GoodsQuantity | 60 | CD | кол-во по инвойсу | copied_from:md\CL на сетку .md |
| 04 | goods_supplementary_quantity | 2520 | CD | кол-во в доп.ед.изм | copied_from:md\CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наим. доп.ед.изм | copied_from:md\CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм доп.количества | нормализация по cb:unit |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator:invoice_1.goods_1.gross_weight |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator:invoice_1.goods_1.net_weight |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from:md\CL на сетку .md |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | copied_from:md\CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator:invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | operator:invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.4*30 | CD | модель/модификация | извлечено из GoodsDescription |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группа ТН ВЭД 5804101000 |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | порядковый номер в группе |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:md\CL на сетку .md |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара | copied_from:md\CL на сетку .md |
| 03 | GoodsQuantity | 30 | CD | кол-во по инвойсу | copied_from:md\CL на сетку .md |
| 04 | goods_supplementary_quantity | 1440 | CD | кол-во в доп.ед.изм | copied_from:md\CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наим. доп.ед.изм | copied_from:md\CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм доп.количества | нормализация по cb:unit |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator:invoice_1.goods_2.gross_weight |
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator:invoice_1.goods_2.net_weight |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from:md\CL на сетку .md |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | copied_from:md\CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator:invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | operator:invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.6*30 | CD | модель/модификация | извлечено из GoodsDescription |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группа ТН ВЭД 5804101000 |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | порядковый номер в группе |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:md\CL на сетку .md |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | CD | описание товара | copied_from:md\CL на сетку .md |
| 03 | GoodsQuantity | 60 | CD | кол-во по инвойсу | copied_from:md\CL на сетку .md |
| 04 | goods_supplementary_quantity | 2520 | CD | кол-во в доп.ед.изм | copied_from:md\CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наим. доп.ед.изм | copied_from:md\CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм доп.количества | нормализация по cb:unit |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator:invoice_1.goods_3.gross_weight |
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator:invoice_1.goods_3.net_weight |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from:md\CL на сетку .md |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | copied_from:md\CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator:invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | operator:invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,4*30 | CD | модель/модификация | извлечено из GoodsDescription |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группа ТН ВЭД 5804101000 |
| 17 | dt_tovg_index | 3 | CD | индекс позиции внутри товара | порядковый номер в группе |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:md\CL на сетку .md |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CD | описание товара | copied_from:md\CL на сетку .md |
| 03 | GoodsQuantity | 30 | CD | кол-во по инвойсу | copied_from:md\CL на сетку .md |
| 04 | goods_supplementary_quantity | 1440 | CD | кол-во в доп.ед.изм | copied_from:md\CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наим. доп.ед.изм | copied_from:md\CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм доп.количества | нормализация по cb:unit |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто по строке | operator:invoice_1.goods_4.gross_weight |
| 08 | NetWeightQuantity | 144.00 | CO | нетто по строке | operator:invoice_1.goods_4.net_weight |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from:md\CL на сетку .md |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | copied_from:md\CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator:invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | operator:invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,6*30 | CD | модель/модификация | извлечено из GoodsDescription |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группа ТН ВЭД 5804101000 |
| 17 | dt_tovg_index | 4 | CD | индекс позиции внутри товара | порядковый номер в группе |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from:md\CL на сетку .md |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара | copied_from:md\CL на сетку .md |
| 03 | GoodsQuantity | 90 | CD | кол-во по инвойсу | copied_from:md\CL на сетку .md |
| 04 | goods_supplementary_quantity | 3780 | CD | кол-во в доп.ед.изм | copied_from:md\CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наим. доп.ед.изм | copied_from:md\CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм доп.количества | нормализация по cb:unit |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто по строке | operator:invoice_1.goods_5.gross_weight |
| 08 | NetWeightQuantity | 491.40 | CO | нетто по строке | operator:invoice_1.goods_5.net_weight |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from:md\CL на сетку .md |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | copied_from:md\CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator:invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | operator:invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,4*30 | CD | модель/модификация | извлечено из GoodsDescription |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | группа ТН ВЭД 7019900095 |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | порядковый номер в группе |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from:md\CL на сетку .md |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара | copied_from:md\CL на сетку .md |
| 03 | GoodsQuantity | 180 | CD | кол-во по инвойсу | copied_from:md\CL на сетку .md |
| 04 | goods_supplementary_quantity | 8640 | CD | кол-во в доп.ед.изм | copied_from:md\CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наим. доп.ед.изм | copied_from:md\CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм доп.количества | нормализация по cb:unit |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто по строке | operator:invoice_1.goods_6.gross_weight |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто по строке | operator:invoice_1.goods_6.net_weight |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from:md\CL на сетку .md |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | copied_from:md\CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator:invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | operator:invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,6*30 | CD | модель/модификация | извлечено из GoodsDescription |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | группа ТН ВЭД 7019900095 |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | порядковый номер в группе |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:md\CL на сетку .md |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | copied_from:md\CL на сетку .md |
| 03 | GoodsQuantity | 5 | CD | кол-во по инвойсу | copied_from:md\CL на сетку .md |
| 04 | goods_supplementary_quantity | 240 | CD | кол-во в доп.ед.изм | copied_from:md\CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наим. доп.ед.изм | copied_from:md\CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм доп.количества | нормализация по cb:unit |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто по строке | operator:invoice_1.goods_7.gross_weight |
| 08 | NetWeightQuantity | 24.00 | CO | нетто по строке | operator:invoice_1.goods_7.net_weight |
| 09 | Price | 28 | CD | цена за единицу | copied_from:md\CL на сетку .md |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | copied_from:md\CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator:invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | operator:invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,6*30 | CD | модель/модификация | извлечено из GoodsDescription |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группа ТН ВЭД 5804101000 |
| 17 | dt_tovg_index | 5 | CD | индекс позиции внутри товара | порядковый номер в группе |
- _item_audit: 17
### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list
  - `xml_target_root`: AltaE2PACK
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\PL на сетку .md
  - `file_name`: PL на сетку .md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто | operator:invoice_1.total_gross_weight |
| 02 | NetWeightQuantity | 3302.00 | CO | общий вес нетто | operator:invoice_1.total_net_weight |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | operator:packing_list_1.consignor_shortname_equals_full |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | краткое наименование | operator:packing_list_1.consignor_shortname_equals_full |
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя | operator:packing_list_1.consignor_country_code_alpha2 |
| 06 | Consignor_Address_CounryName | China | CD | страна грузоотправителя, текст | copied_from:md\CL на сетку .md |
| 07 | Consignor_Address_Region | Hebei | CD | регион | copied_from:md\CL на сетку .md |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from:md\CL на сетку .md |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | copied_from:md\CL на сетку .md |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_proto.md (declarant.organization_name) |
| 11 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | master_proto.md (declarant.short_name) |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_proto.md (declarant.ogrn) |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | master_proto.md (declarant.inn) |
| 14 | Consignee_KPP | 165001001 | CD | КПП | master_proto.md (declarant.kpp) |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | master_proto.md (declarant.postal_code) |
| 16 | Consignee_Address_CountryCode | RU | CD | страна | master_proto.md (declarant.country_code) |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | master_proto.md (declarant.country_name) |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_proto.md (declarant.region) |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_proto.md (declarant.city) |
| 20 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_proto.md (declarant.street_house) |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | copied_from:md\PL на сетку .md |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий | operator:invoice_1.delivery_terms_numeric |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator:invoice_1.delivery_terms_string |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | КОНТРАКТ | operator:packing_list_1.DeliveryTerms_Contract_PrDocumentName |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | copied_from:md\PL на сетку .md |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | copied_from:md\PL на сетку .md |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | ИВОЙС | константа |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | copied_from:md\PL на сетку .md |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | copied_from:md\PL на сетку .md |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CO | УПАКОВОЧНЫЙ ЛИСТ | operator:packing_list_1.registration_doc_name |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | operator:packing_list_1.registration_doc_number |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | operator:packing_list_1.registration_doc_date |
| 33 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 34 | doc_code | 04131 | CD | код документа | константа |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | константа |
| 36 | doc_number | LM-2591 | CD | номер документа | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentNumber |
| 37 | doc_date | 30.10.2025 | CD | дата документа | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentDate |
- _audit: 37

#### 3.2.1 Массив: Goods[7]
- _array_audit: 7

#### 3.2.1 Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки | copied_from:md\PL на сетку .md |
| 02 | GoodsQuantity | 60 | CD | кол-во мест | copied_from:md\PL на сетку .md |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | copied_from:md\PL на сетку .md |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | copied_from:md\PL на сетку .md |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок | operator:packing_list_1.goods_1.paking_quantity |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки | copied_from:md\PL на сетку .md |
| 02 | GoodsQuantity | 30 | CD | кол-во мест | copied_from:md\PL на сетку .md |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто по строке | copied_from:md\PL на сетку .md |
| 04 | NetWeightQuantity | 460.80 | CD | нетто по строке | copied_from:md\PL на сетку .md |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок | operator:packing_list_1.goods_2.paking_quantity |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера . Размер рулона 1,42*0,64*0,22 | CD | описание строки | copied_from:md\PL на сетку .md |
| 02 | GoodsQuantity | 60 | CD | кол-во мест | copied_from:md\PL на сетку .md |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто по строке | copied_from:md\PL на сетку .md |
| 04 | NetWeightQuantity | 252.00 | CD | нетто по строке | copied_from:md\PL на сетку .md |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок | operator:packing_list_1.goods_3.paking_quantity |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание строки | copied_from:md\PL на сетку .md |
| 02 | GoodsQuantity | 30 | CD | кол-во мест | copied_from:md\PL на сетку .md |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто по строке | copied_from:md\PL на сетку .md |
| 04 | NetWeightQuantity | 144.00 | CD | нетто по строке | copied_from:md\PL на сетку .md |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок | operator:packing_list_1.goods_4.paking_quantity |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА " Антимошка " из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание строки | copied_from:md\PL на сетку .md |
| 02 | GoodsQuantity | 90 | CD | кол-во мест | copied_from:md\PL на сетку .md |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто по строке | copied_from:md\PL на сетку .md |
| 04 | NetWeightQuantity | 491.40 | CD | нетто по строке | copied_from:md\PL на сетку .md |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок | operator:packing_list_1.goods_5.paking_quantity |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание строки | copied_from:md\PL на сетку .md |
| 02 | GoodsQuantity | 180 | CD | кол-во мест | copied_from:md\PL на сетку .md |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | copied_from:md\PL на сетку .md |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто по строке | copied_from:md\PL на сетку .md |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок | operator:packing_list_1.goods_6.paking_quantity |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки "Антипыльца " из полиэстера Размер рулона 1,72*0,35*0,31*1 | CD | описание строки | copied_from:md\PL на сетку .md |
| 02 | GoodsQuantity | 5 | CD | кол-во мест | copied_from:md\PL на сетку .md |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто по строке | copied_from:md\PL на сетку .md |
| 04 | NetWeightQuantity | 24.00 | CD | нетто по строке | copied_from:md\PL на сетку .md |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок | operator:packing_list_1.goods_7.paking_quantity |
- _item_audit: 5

#### 3.2.2 Массив: TransportMeans[2]
- _array_audit: 2

#### 3.2.2 Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | О157АО774 | CO | рег. номер | operator:packing_list_1.transport_1.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator:packing_list_1.transport_1.mode_code |
| 03 | NationalityCode | 000 | CO | код национальности | operator:packing_list_1.transport_1.nationality_code |
| 04 | MoverIndicator | true | CO | признак тягача | operator:packing_list_1.transport_1.mover_indicator |
- _item_audit: 4

#### 3.2.2 Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | ВТ374974 | CO | рег. номер | operator:packing_list_1.transport_2.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator:packing_list_1.transport_2.mode_code |
| 03 | NationalityCode | 000 | CO | код национальности | operator:packing_list_1.transport_2.nationality_code |
| 04 | MoverIndicator | false | CO | признак тягача | operator:packing_list_1.transport_2.mover_indicator |
- _item_audit: 4
### `document`: CMR
  - `uqi_prefix`: formalized.cmr
  - `xml_target_root`: AltaE3CMR
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\СМР от СВХ.md
  - `file_name`: СМР от СВХ.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CO | язык документа | operator:cmr_1.language_code |
| 02 | CMR_Choice | 1 | CO | системный выбор Альты | operator:cmr_1.cmr_choice |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | copied_from:md\СМР от СВХ.md |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | copied_from:md\СМР от СВХ.md (п.4) |
| 05 | RegistrationDocument_Place | Манчжурия | CO | место составления | operator:cmr_1.registration_place |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата принятия груза | copied_from:md\СМР от СВХ.md |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза | operator:cmr_1.taking_cargo_country_code_alpha2 |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | Китай | CD | страна принятия груза, текст | copied_from:md\СМР от СВХ.md |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки | operator:cmr_1.delivery_country_code_alpha2 |
| 10 | DeliveryPlace_CounryName | Россия | CD | страна доставки, текст | copied_from:md\СМР от СВХ.md |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки | operator:cmr_1.delivery_terms_place |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator:cmr_1.delivery_terms_string |
| 13 | GoodsQuantity | 127 | CD | общее кол-во мест | copied_from:md\СМР от СВХ.md |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто | copied_from:md\СМР от СВХ.md |
| 15 | CMRTransport_PrimeMoverStateSignID | O157AO774 | CD | гос. номер тягача | copied_from:md\СМР от СВХ.md |
| 16 | CMRTransport_TrailerStateSignID | BT374974 | CD | гос. номер прицепа | copied_from:md\СМР от СВХ.md |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование отправителя | copied_from:md\СМР от СВХ.md |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator:cmr_1.consignor_shortname_equals_full |
| 19 | Consignor_PostalAddress_CountryCode | CN | CO | страна отправителя | operator:packing_list_1.consignor_country_code_alpha2 |
| 20 | Consignor_Address_CounryName | China | CD | страна, текст | copied_from:md\СМР от СВХ.md |
| 21 | Consignor_Address_Region | Hebei | CD | регион | copied_from:md\СМР от СВХ.md |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from:md\СМР от СВХ.md |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | copied_from:md\СМР от СВХ.md |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | наименование гаранта | operator:cmr_1.consignor_guarantee_all |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое наименование | operator:cmr_1.consignor_guarantee_all |
| 26 | Consignor_Guarantee_Address_CountryCode | | CO | страна | operator:cmr_1.consignor_guarantee_all |
| 27 | Consignor_Guarantee_Address_CounryName | | CO | страна, текст | operator:cmr_1.consignor_guarantee_all |
| 28 | Consignor_Guarantee_Address_Region | | CO | регион | operator:cmr_1.consignor_guarantee_all |
| 29 | Consignor_Guarantee_Address_City | | CO | город/район | operator:cmr_1.consignor_guarantee_all |
| 30 | Consignor_Guarantee_Address_StreetHouse | | CO | улица/дом | operator:cmr_1.consignor_guarantee_all |
| 31 | Consignee_NameInf | ООО «Скиф» | CD | наименование получателя | copied_from:md\СМР от СВХ.md |
| 32 | Consignee_ShortName | ООО «Скиф» | CO | краткое наименование | operator:cmr_1.consignee_shortname_equals_full |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | operator:cmr_1.consignee_ogrn_from_master_data |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | copied_from:md\СМР от СВХ.md |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | copied_from:md\СМР от СВХ.md |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | copied_from:md\СМР от СВХ.md |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна | copied_from:md\СМР от СВХ.md |
| 38 | Consignee_Address_CounryName | Россия | CD | страна, текст | copied_from:md\СМР от СВХ.md |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | copied_from:md\СМР от СВХ.md |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | copied_from:md\СМР от СВХ.md |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом/офис | copied_from:md\СМР от СВХ.md |
| 42 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 43 | doc_code | 02015 | CD | код документа | константа |
| 44 | doc_name | CMR | CD | наименование документа | константа |
| 45 | doc_number | 00378 | CD | номер документа | copied_from:formalized.cmr.RegistrationDocument_RegID |
| 46 | doc_date | 20.01.2026 | CD | дата документа | copied_from:formalized.cmr.RegistrationDocument_DateInf |
- _audit: 46

#### 3.3.1 Массив: CMRGoods[1]
- _array_audit: 1

#### 3.3.1 Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер | авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 | CD | описание груза | исключение CMRGoodsDescription — источник non_formalized.svh_1 |
| 03 | PakingQuantity | 127 | CO | кол-во упаковок | operator:cmr_1.goods_1.packing_quantity |
- _item_audit: 3
### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_1_13.01.2026.md
  - `file_name`: currency_transfer_1_13.01.2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator:payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | код способа платежа | operator:payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 63.219,00 | CD | сумма платежа | copied_from:md\currency_transfer_1_13.01.2026.md |
| 04 | TransactionKind | 01 | CO | вид операции | operator:payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator:payment_order_all.priority |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from:md\currency_transfer_1_13.01.2026.md |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | copied_from:md\currency_transfer_1_13.01.2026.md |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | copied_from:md\currency_transfer_1_13.01.2026.md |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | copied_from:md\currency_transfer_1_13.01.2026.md |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | copied_from:md\currency_transfer_1_13.01.2026.md |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from:md\currency_transfer_1_13.01.2026.md |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator:payment_order_1.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО)) | CD | банк плательщика | copied_from:md\currency_transfer_1_13.01.2026.md |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | copied_from:md\currency_transfer_1_13.01.2026.md |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | банк получателя | copied_from:md\currency_transfer_1_13.01.2026.md |
| 16 | PersonSurname | Саранов | CO | фамилия | operator:payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя | operator:payment_order_all.payer_sign.name |
| 18 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 19 | doc_code | 04023 | CD | код документа | константа |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | константа |
| 21 | doc_number | 1 | CD | номер документа | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber |
| 22 | doc_date | 13.01.2026 | CD | дата документа | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate |
- _audit: 22

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_7_28.11.2025.md
  - `file_name`: currency_transfer_7_28.11.2025.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator:payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | код способа платежа | operator:payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 34.041,00 | CD | сумма платежа | copied_from:md\currency_transfer_7_28.11.2025.md |
| 04 | TransactionKind | 01 | CO | вид операции | operator:payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator:payment_order_all.priority |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from:md\currency_transfer_7_28.11.2025.md |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | copied_from:md\currency_transfer_7_28.11.2025.md |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | copied_from:md\currency_transfer_7_28.11.2025.md |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | copied_from:md\currency_transfer_7_28.11.2025.md |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | copied_from:md\currency_transfer_7_28.11.2025.md |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from:md\currency_transfer_7_28.11.2025.md |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator:payment_order_2.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО)) | CD | банк плательщика | copied_from:md\currency_transfer_7_28.11.2025.md |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | copied_from:md\currency_transfer_7_28.11.2025.md |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | банк получателя | copied_from:md\currency_transfer_7_28.11.2025.md |
| 16 | PersonSurname | Саранов | CO | фамилия | operator:payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя | operator:payment_order_all.payer_sign.name |
| 18 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 19 | doc_code | 04023 | CD | код документа | константа |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | константа |
| 21 | doc_number | 7 | CD | номер документа | copied_from:formalized.payment_order_2.DocumentReference_PrDocumentNumber |
| 22 | doc_date | 28.11.2025 | CD | дата документа | copied_from:formalized.payment_order_2.DocumentReference_PrDocumentDate |
- _audit: 22
### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice
  - `xml_target_root`: AltaServiceInvoice
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_от_27-01-2026.md
  - `file_name`: Счет_№26-00378-tl_от_27-01-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак документа | operator:service_invoice_1.document_sign |
| 02 | TotalServiceCost | 2700,00 | CD | итого по услугам | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 03 | Currency | USD | CD | валюта итого | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 05 | BankName | АО "Райффайзенбанк"; БИК 044525700; Сч. № 30101810200000000700; Сч. № 40702810400000233463 | CD | банк исполнителя | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 06 | ContractDetails_PrDocumentNumber | №КООО/26651/М | CD | № договора на услуги | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 07 | ContractDetails_PrDocumentDate | 13-05-2025 | CD | дата договора на услуги | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 08 | PrDocumentNumber | ОТСУТСТВУЕТ | CO | номер заказа | operator:service_invoice_1.payment_document_number |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CO | дата заказа | operator:service_invoice_1.payment_document_date |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | operator:service_invoice_1.consignor_decision |
| 14 | PostalCode | | CO | индекс | operator:service_invoice_1.consignor_postalcode_empty_ok |
| 15 | CountryCode | CN | CO | страна alpha-2 | operator:invoice_1.seller_country_code_alpha2 |
| 16 | CounryName | China | CO | страна, текст | copied_from:md\CL на сетку .md |
| 17 | Region | Hebei | CO | регион | copied_from:md\CL на сетку .md |
| 18 | Town | Shijiazhuang | CO | город/район | copied_from:md\CL на сетку .md |
| 19 | StreetHouse | No. 5 Gaodong street | CO | улица/дом | copied_from:md\CL на сетку .md |
| 20 | Consignee_OrganizationName | ООО "Скиф" (ООО "СКИФ") | CD | грузополучатель | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator:service_invoice_1.consignee_ogrn_from_master_data |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 24 | PostalCode | 423800 | CD | индекс | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 25 | CountryCode | RU | CD | страна alpha-2 | master_proto.md (declarant.country_code) |
| 26 | CounryName | Россия | CD | страна, текст | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 27 | Region | Республика Татарстан | CD | регион | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 28 | Town | Набережные Челны | CD | город | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 29 | StreetHouse | проезд Хлебный | CD | улица | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 30 | House | 30 | CO | дом | operator:service_invoice_1.consignee_house |
| 31 | Room | 211 | CO | офис/кв | operator:service_invoice_1.consignee_room |
| 32 | Signature_Choice | 2 | CD | вариант подписи | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | |
| 34 | IndividualEntrepreneur_PersonName | | CD | имя ИП | |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климовин | CD | фамилия руководителя | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л. | CD | имя руководителя | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А. | CD | отчество руководителя | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О. | CD | имя бухгалтера | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А. | CD | отчество бухгалтера | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 42 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 43 | doc_code | 04031 | CD | код документа | константа |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | константа |
| 45 | doc_number | 26-00378-tl | CD | номер документа | copied_from:formalized.service_invoice.Registration_PrDocumentNumber |
| 46 | doc_date | 27.01.2026 | CD | дата документа | copied_from:formalized.service_invoice.Registration_PrDocumentDate |
| 47 | transport_to_border | 1404.00 | CO | стоимость до границы | operator:service_invoice_1.transport_to_border |
| 48 | transport_currency | USD | CO | валюта перевозки | operator:service_invoice_1.transport_to_border |
- _audit: 48

#### 3.5.1 Массив: ServiceDescription[2]
- _array_audit: 2

#### 3.5.1 Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) - перевозка автотранспортом | CD | описание услуги | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 03 | ServiceName | China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) | CD | наименование/маршрут | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 05 | TaxSum | 0,00 | CD | сумма налога | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 06 | ServiceCost_Amount | 1404,00 | CD | стоимость строки | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
- _item_audit: 7

#### 3.5.1 Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 03 | ServiceName | граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | наименование/маршрут | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 05 | TaxSum | 0,00 | CD | сумма налога | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 06 | ServiceCost_Amount | 1296,00 | CD | стоимость строки | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md |
- _item_audit: 7

### `document`: Insurance Document
  - `uqi_prefix`: formalized.insurance_document
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md
  - `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | Счет на оплату №26-00378-tl/1 от 14.01.2026 г. | CD | наименование документа | copied_from:md\Счет_№26-00378-tl_1_от_14-01-2026.md |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | copied_from:md\Счет_№26-00378-tl_1_от_14-01-2026.md |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | copied_from:md\Счет_№26-00378-tl_1_от_14-01-2026.md |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | основной текст | operator:insurance_document_1.textpara_storage |
| 06 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 07 | doc_code | 04111 | CD | код документа | константа |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | константа |
| 09 | doc_number | 26-00378-tl/1 | CD | номер документа | copied_from:formalized.insurance_document.DocumentHead_DocumentNumber |
| 10 | doc_date | 14.01.2026 | CD | дата документа | copied_from:formalized.insurance_document.DocumentHead_DocumentDate |
| 11 | insurance_to_border | 910.34 | CO | стоимость страхования | operator:insurance_document_1.insurance_to_border |
| 12 | insurance_currency | RUB | CO | валюта страхования | operator:insurance_document_1.insurance_to_border |
- _audit: 12
### `document`: Tech Description
  - `uqi_prefix`: formalized.tech_description
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md
  - `file_name`: техничка Антикот, антипыльца антимошка .md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | Технические характеристики — Сетки из полиэстера 5804101000 / Сетки из стекловолокна 7019900095 | CD | наименование техописания | copied_from:md\техничка Антикот, антипыльца антимошка .md |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата техописания | operator:tech_description_1.date |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания | operator:tech_description_1.number |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md | CD | технический текст | copied_from:md\техничка Антикот, антипыльца антимошка .md |
| 06 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 07 | doc_code | 05999 | CD | код документа | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | константа |
| 09 | doc_number | Б/Н | CO | номер документа | operator:tech_description_1.number |
| 10 | doc_date | 30.10.2025 | CO | дата документа | operator:tech_description_1.date |
- _audit: 10

## 3. master_data:

### `document`: Contract
  - `uqi_prefix`: master_data.contract

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 02 | doc_code | 03011 | CD | код документа | константа |
| 03 | doc_name | КОНТРАКТ | CD | наименование документа | константа |
| 04 | doc_number | LM-2553 | CD | номер документа | master_proto.md (contract.doc_number) |
| 05 | doc_date | 02.07.2025 | CD | дата документа | master_proto.md (contract.doc_date) |
- _audit: 5

### `document`: Supplementary Contract
  - `uqi_prefix`: master_data.supplementary_contract_1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 02 | doc_code | 03012 | CD | код документа | константа |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наименование документа | константа |
| 04 | doc_number | 1 | CD | номер документа | master_proto.md (supplementary_contract_1.doc_number) |
| 05 | doc_date | 25.11.2025 | CD | дата документа | master_proto.md (supplementary_contract_1.doc_date) |
- _audit: 5

### `document`: EGRUL
  - `uqi_prefix`: master_data.egrul

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование организации | master_proto.md (declarant.organization_name) |
| 02 | ShortName | ООО "СКИФ" | CD | краткое наименование | master_proto.md (declarant.short_name) |
| 03 | OGRN | 1201600020390 | CD | ОГРН | master_proto.md (declarant.ogrn) |
| 04 | INN | 1650389298 | CD | ИНН | master_proto.md (declarant.inn) |
| 05 | KPP | 165001001 | CD | КПП | master_proto.md (declarant.kpp) |
| 06 | Address_PostalCode | 423800 | CD | индекс | master_proto.md (declarant.postal_code) |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | master_proto.md (declarant.country_code) |
| 08 | Address_CounryName | РОССИЯ | CD | страна, текст | master_proto.md (declarant.country_name) |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_proto.md (declarant.region) |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_proto.md (declarant.city) |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_proto.md (declarant.street_house) |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | master_proto.md (declarant.phone) |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | master_proto.md (declarant.email) |
| 14 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 15 | doc_code | 04011 | CD | код документа | константа |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | константа |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | master_proto.md (egrul.doc_number) |
| 18 | doc_date | 14.07.2025 | CD | дата документа | master_proto.md (egrul.doc_date) |
- _audit: 18

### `document`: Personal Passport
  - `uqi_prefix`: master_data.passport

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | master_proto.md (representative.surname) |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | master_proto.md (representative.name) |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | master_proto.md (representative.middle_name) |
| 04 | CardSeries | 63 09 | CD | серия | master_proto.md (representative.passport_series) |
| 05 | CardNumber | 449948 | CD | номер | master_proto.md (representative.passport_number) |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | master_proto.md (representative.passport_date) |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_proto.md (representative.passport_org) |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_proto.md (representative.phone) |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | master_proto.md (representative.email) |
| 10 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 11 | doc_code | 11001 | CD | код документа | константа |
| 12 | doc_name | ПАСПОРТ | CD | наименование документа | константа |
| 13 | doc_number | 63 09 449948 | CD | номер документа | master_proto.md (passport.doc_number) |
| 14 | doc_date | 11.03.2010 | CD | дата документа | master_proto.md (passport.doc_date) |
- _audit: 14

### `document`: Letter of Attorney
  - `uqi_prefix`: master_data.letter_of_attorney

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_proto.md (letter_of_attorney.doc_number) |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_proto.md (letter_of_attorney.doc_date) |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_proto.md (letter_of_attorney.end_date) |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_proto.md (letter_of_attorney.empowered_post) |
| 05 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 06 | doc_code | 11004 | CD | код документа | константа |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | константа |
| 08 | doc_number | 1 | CD | номер документа | master_proto.md (letter_of_attorney.doc_number) |
| 09 | doc_date | 01.02.2026 | CD | дата документа | master_proto.md (letter_of_attorney.doc_date) |
- _audit: 9

### `document`: Transport Contract
  - `uqi_prefix`: master_data.transport_contract

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 02 | doc_code | 04033 | CD | код документа | константа |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование документа | константа |
| 04 | doc_number | КООО/26651/М | CD | номер документа | master_proto.md (transport_contract.doc_number) |
| 05 | doc_date | 13.05.2025 | CD | дата документа | master_proto.md (transport_contract.doc_date) |
- _audit: 5

### `document`: Exemption Letter
  - `uqi_prefix`: master_data.exemption_letter

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 02 | doc_code | 09023 | CD | код документа | константа |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | константа |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | master_proto.md (exemption_letter.doc_number) |
| 05 | doc_date | 20.08.2025 | CD | дата документа | master_proto.md (exemption_letter.doc_date) |
- _audit: 5

### `document`: Exemption Letter (source)
  - `uqi_prefix`: master_data.exemption_letter_source

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 02 | doc_code | 09999 | CD | код документа | константа |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | константа |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | master_proto.md (exemption_letter_source.doc_number) |
| 05 | doc_date | 20.08.2025 | CD | дата документа | master_proto.md (exemption_letter_source.doc_date) |
- _audit: 5
## 4. non_formalized:

### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.td
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ТД 10719110_240126_5011363_reg00378тд.md
  - `file_name`: ТД 10719110_240126_5011363_reg00378тд.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs_post_code | 10719110 | CD | код таможенного органа | copied_from:md\ТД 10719110_240126_5011363_reg00378тд.md |
| 02 | customs_post_name | ОТСУТСТВУЕТ | CD | наим. таможенного органа | copied_from:md\ТД 10719110_240126_5011363_reg00378тд.md |
| 03 | transport_reg_number | O157AO774/BT374974 | CD | ТС по ТД | copied_from:md\ТД 10719110_240126_5011363_reg00378тд.md |
| 04 | doc_gr44 | true | CD | признак включения в гр.44 | константа |
| 05 | doc_code | 09013 | CD | код документа | константа |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | наименование документа | константа |
| 07 | doc_number | 10719110/240126/5011363 | CO | номер документа | operator:transit_declaration_1.number |
| 08 | doc_date | 24.01.2026 | CO | дата документа | operator:transit_declaration_1.date |
- _audit: 8

### `document`: Storage Report
  - `uqi_prefix`: non_formalized.svh
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО 14431420260204161621.md
  - `file_name`: ДО 14431420260204161621.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии СВХ | copied_from:md\ДО 14431420260204161621.md |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии СВХ | copied_from:md\ДО 14431420260204161621.md |
| 03 | actual_gross_weight | 3500 | CO | фактический вес брутто | operator:svh_1.actual_totals_from_svh_additional_sheet |
| 04 | actual_places | 127 | CO | фактическое кол-во мест | operator:svh_1.actual_totals_from_svh_additional_sheet |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС при въезде | copied_from:md\ДО 14431420260204161621.md |
| 06 | doc_gr44 | false | CD | признак включения в гр.44 | константа |
- _audit: 6

#### 4.2.1 Массив: goods[2]
- _array_audit: 2

#### 4.2.1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 7019900095 | CD | код товара | copied_from:md\ДО 14431420260204161621.md |
| 02 | places | 27 | CD | кол-во мест по строке | copied_from:md\ДО 14431420260204161621.md |
| 03 | gross_weight_kg | 1710 | CD | вес брутто по строке | copied_from:md\ДО 14431420260204161621.md |
| 04 | cost | 42228 | CD | стоимость по строке | copied_from:md\ДО 14431420260204161621.md |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from:md\ДО 14431420260204161621.md |
- _item_audit: 5

#### 4.2.1 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара | copied_from:md\ДО 14431420260204161621.md |
| 02 | places | 100 | CD | кол-во мест по строке | copied_from:md\ДО 14431420260204161621.md |
| 03 | gross_weight_kg | 1790 | CD | вес брутто по строке | copied_from:md\ДО 14431420260204161621.md |
| 04 | cost | 55032 | CD | стоимость по строке | copied_from:md\ДО 14431420260204161621.md |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from:md\ДО 14431420260204161621.md |
- _item_audit: 5

### `document`: Storage Report Additional Sheet
  - `uqi_prefix`: non_formalized.svh_additional_sheet_1
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО доп 14431520260204161645.md
  - `file_name`: ДО доп 14431520260204161645.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 1 | CD | № доп.листа | copied_from:md\ДО доп 14431520260204161645.md |
| 02 | date | 03.02.2026 | CO | дата доп.листа | operator:svh_1.date |
| 03 | actual_gross_weight | 3500 | CD | фактический вес брутто | copied_from:md\ДО доп 14431520260204161645.md |
| 04 | actual_places | 127 | CD | фактическое кол-во мест | copied_from:md\ДО доп 14431520260204161645.md |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CO | номер ТС при въезде | copied_from:md\ДО 14431420260204161621.md |
| 06 | svh_address_region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион СВХ | operator:svh_additional_sheet_1.svh_address_region |
| 07 | svh_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город/нас.пункт СВХ | operator:svh_additional_sheet_1.svh_address_city |
| 08 | svh_address_street_house | Производственный пр-д, д. 45 | CO | улица/дом СВХ | operator:svh_additional_sheet_1.svh_address_street_house |
| 09 | svh_customs_code | 10404083 | CO | код таможенного органа | operator:svh_additional_sheet_1.svh_customs_code |
| 10 | doc_gr44 | false | CD | признак включения в гр.44 | константа |
- _audit: 10
### Итогo, по файлу:

`total_unreliable_fields`: 0
`primary_status`: confirmed

## 5. Нерешенные вопросы (Issues)

**Для полей:**
- Нет.

**Для общих вопросов:**
- Нет.

## 6. `unreliable_fields`:
- Нет.
