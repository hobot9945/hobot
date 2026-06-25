# Первичные данные

## 1. meta:
- `название кейса`: Kyland
- `путь к папке поставки`: alta\source\Kyland\01
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 7 товаров
- `источники данных`: md + master_data.md + предыдущая ДТ 10005030/040123/3000837

## 2. formalized:

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: alta\source\Kyland\01\md\CI, PL final_Invoice.md
- `file_name`: CI, PL final_Invoice.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | USD | CD | валюта инвойса | copied_from:formalized.invoice_1.CurrencyCode |
| 02 | DocumentCode | 04021 | CD | код вида документа | константа |
| 03 | PlacesQuantity | 6 | CD | кол-во грузовых мест | copied_from:formalized.packing_list.TotalCartons |
| 04 | PlacesDescription | Cartons | CD | описание мест | copied_from:formalized.packing_list.PlacesDescription |
| 05 | GrossWeightQuantity | 90 | CD | общий вес брутто | copied_from:formalized.packing_list.GrossWeightQuantity |
| 06 | NetWeightQuantity | 78 | CD | общий вес нетто | copied_from:formalized.packing_list.NetWeightQuantity |
| 07 | GCost | 25255.00 | CD | системное поле стоимости | copied_from:formalized.invoice_1.TotalCost |
| 08 | TotalCost | 25255.00 | CD | итого по инвойсу | copied_from:formalized.invoice_1.TotalCost |
| 09 | DeliveryTerms_DeliveryPlace | Yichang | CD | место поставки по Incoterms | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryPlace |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий поставки | cb:delivery_tems |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | copied_from:formalized.invoice_1.DeliveryTerms_DispatchCountryCode |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | copied_from:formalized.invoice_1.DeliveryTerms_TradingCountryCode |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | copied_from:formalized.invoice_1.DeliveryTerms_DestinationCountryCode |
| 15 | Registration_PrDocumentName | COMMERCIAL INVOICE | CD | наименование документа | copied_from:formalized.invoice_1.Registration_PrDocumentName |
| 16 | Registration_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | номер инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentNumber |
| 17 | Registration_PrDocumentDate | 2022-12-12 | CD | дата инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentDate |
| 18 | Contract_PrDocumentNumber | lm191018/Kyl | CD | № контракта-ссылки | copied_from:formalized.invoice_1.Contract_PrDocumentNumber |
| 19 | Contract_PrDocumentDate | 2018-10-19 | CD | дата контракта-ссылки | copied_from:formalized.invoice_1.Contract_PrDocumentDate |
| 20 | Buyer_CompanyID | 7720609470 | CD | ИНН покупателя | master_data:master_data.md |
| 21 | Buyer_KPPCode | 772001001 | CD | КПП покупателя | master_data:master_data.md |
| 22 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | наименование покупателя | master_data:master_data.md |
| 23 | Buyer_PostalAddress_PostalCode | 111675 | CD | почтовый индекс | master_data:master_data.md |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | master_data:master_data.md |
| 25 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя текст | master_data:master_data.md |
| 26 | Buyer_PostalAddress_Region | ГОРОД МОСКВА | CD | регион | master_data:master_data.md |
| 27 | Buyer_PostalAddress_City | МОСКВА | CD | город | master_data:master_data.md |
| 28 | Buyer_PostalAddress_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data:master_data.md |
| 29 | Seler_Name | Kyland Technology Co., Ltd. | CD | продавец | copied_from:formalized.invoice_1.Seler_Name |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | cb:country |
| 31 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца текст | cb:country |
| 32 | Seler_PostalAddress_Region | Beijing | CD | регион продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_Region |
| 33 | Seler_PostalAddress_City | Beijing | CD | город/район продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_City |
| 34 | Seler_PostalAddress_StreetHouse | Building No.2, Shixing Avenue 30# Shijingshan District | CD | улица/дом продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 35 | Consignor_OrganizationName | Kyland Technology Co., Ltd. | CD | грузоотправитель | note: нормализация: consignor=seller |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | note: нормализация: consignor=seller |
| 37 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя текст | note: нормализация: consignor=seller |
| 38 | Consignor_Address_Region | Beijing | CD | регион | note: нормализация: consignor=seller |
| 39 | Consignor_Address_City | Beijing | CD | город/район | note: нормализация: consignor=seller |
| 40 | Consignor_Address_StreetHouse | Building No.2, Shixing Avenue 30# Shijingshan District | CD | улица/дом | note: нормализация: consignor=seller |
| 41 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | грузополучатель | master_data:master_data.md |
| 42 | Consignee_OGRN | 1087746277740 | CD | ОГРН | master_data:master_data.md |
| 43 | Consignee_INN | 7720609470 | CD | ИНН | master_data:master_data.md |
| 44 | Consignee_KPP | 772001001 | CD | КПП | master_data:master_data.md |
| 45 | Consignee_Address_PostalCode | 111675 | CD | почтовый индекс | master_data:master_data.md |
| 46 | Consignee_Address_CountryCode | RU | CD | страна | master_data:master_data.md |
| 47 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | master_data:master_data.md |
| 48 | Consignee_Address_Region | ГОРОД МОСКВА | CD | регион | master_data:master_data.md |
| 49 | Consignee_Address_City | МОСКВА | CD | город | master_data:master_data.md |
| 50 | Consignee_Address_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data:master_data.md |
| 51 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 52 | doc_code | 04021 | CD | код документа Гр44 | константа |
| 53 | doc_name | ИНВОЙС | CD | наим документа Гр44 | константа |
| 54 | doc_number | 1000059769 /60285 /60389 /60491 | CD | номер документа Гр44 | copied_from:formalized.invoice_1.Registration_PrDocumentNumber |
| 55 | doc_date | 2022-12-12 | CD | дата документа Гр44 | copied_from:formalized.invoice_1.Registration_PrDocumentDate |
- _audit: 55

#### InvoiceGoods Массив: InvoiceGoods[7]
- _array_audit: 7

#### Element: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[1].GoodsDescription |
| 03 | GoodsQuantity | 19 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[1].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 45.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 08 | NetWeightQuantity | 38.000 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 09 | Price | 683.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[1].Price |
| 10 | TotalCost | 12977.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[1].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-4GX16GE-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[2].GoodsDescription |
| 03 | GoodsQuantity | 15 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[2].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 15.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 08 | NetWeightQuantity | 13.500 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 09 | Price | 347.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[2].Price |
| 10 | TotalCost | 5205.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[2].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[2].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[2].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-LITE-2GX8T-L3-L3 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[2].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[3].GoodsDescription |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[3].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 8.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 08 | NetWeightQuantity | 7.500 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 09 | Price | 347.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[3].Price |
| 10 | TotalCost | 1735.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[3].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[3].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[3].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-4SFP8T-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[3].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[4].GoodsDescription |
| 03 | GoodsQuantity | 1 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[4].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 1.500 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 08 | NetWeightQuantity | 1.250 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 09 | Price | 215.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[4].Price |
| 10 | TotalCost | 215.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[4].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[4].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[4].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-LITE-8T-L3-L3 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[4].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 4 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[5].GoodsDescription |
| 03 | GoodsQuantity | 1 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[5].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 1.500 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 08 | NetWeightQuantity | 1.250 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 09 | Price | 604.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[5].Price |
| 10 | TotalCost | 604.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[5].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[5].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[5].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX16GE-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[5].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 5 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[6].GoodsDescription |
| 03 | GoodsQuantity | 8 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[6].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 14.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 08 | NetWeightQuantity | 12.000 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 09 | Price | 368.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[6].Price |
| 10 | TotalCost | 2944.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[6].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[6].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[6].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX8T-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[6].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 6 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[7].GoodsDescription |
| 03 | GoodsQuantity | 3 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[7].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 5.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 08 | NetWeightQuantity | 4.500 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 09 | Price | 525.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[7].Price |
| 10 | TotalCost | 1575.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[7].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[7].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[7].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX16T-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[7].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 7 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

- `doc_status`: confirmed

### `document`: Packing List
- `uqi_prefix`: formalized.packing_list
- `xml_target_root`: AltaE2PACK
- `path`: alta\source\Kyland\01\md\CI, PL final_PackingList.md
- `file_name`: CI, PL final_PackingList.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 90 | CD | общий вес брутто | copied_from:formalized.packing_list.GrossWeightQuantity |
| 02 | NetWeightQuantity | 78 | CD | общий вес нетто | copied_from:formalized.packing_list.NetWeightQuantity |
| 03 | Consignor_OrganizationName | Kyland Technology Co., Ltd. | CD | грузоотправитель | copied_from:formalized.packing_list.Consignor_OrganizationName |
| 04 | Consignor_ShortName | Kyland | CD | краткое наим грузоотправителя | copied_from:formalized.packing_list.Consignor_ShortName |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | cb:country |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя текст | cb:country |
| 07 | Consignor_Address_Region | Beijing | CD | регион | copied_from:formalized.packing_list.Consignor_Address_Region |
| 08 | Consignor_Address_City | Beijing | CD | город/район | copied_from:formalized.packing_list.Consignor_Address_City |
| 09 | Consignor_Address_StreetHouse | Building No.2, Shixing Avenue 30# Shijingshan District | CD | улица/дом | copied_from:formalized.packing_list.Consignor_Address_StreetHouse |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | грузополучатель | master_data:master_data.md |
| 11 | Consignee_ShortName | ООО "СИМАНИТРОН" | CD | краткое наим грузополучателя | master_data:master_data.md |
| 12 | Consignee_OGRN | 1087746277740 | CD | ОГРН | master_data:master_data.md |
| 13 | Consignee_INN | 7720609470 | CD | ИНН | master_data:master_data.md |
| 14 | Consignee_KPP | 772001001 | CD | КПП | master_data:master_data.md |
| 15 | Consignee_Address_PostalCode | 111675 | CD | почтовый индекс | master_data:master_data.md |
| 16 | Consignee_Address_CountryCode | RU | CD | страна | master_data:master_data.md |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | master_data:master_data.md |
| 18 | Consignee_Address_Region | ГОРОД МОСКВА | CD | регион | master_data:master_data.md |
| 19 | Consignee_Address_City | МОСКВА | CD | город | master_data:master_data.md |
| 20 | Consignee_Address_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data:master_data.md |
| 21 | DeliveryTerms_DeliveryPlace | Yichang | CD | место поставки по Incoterms | copied_from:formalized.packing_list.DeliveryTerms_DeliveryPlace |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | cb:delivery_tems |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from:formalized.packing_list.DeliveryTerms_DeliveryTermsStringCode |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наим контракта | константа |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | lm191018/Kyl | CD | № контракта | copied_from:formalized.packing_list.DeliveryTerms_Contract_PrDocumentNumber |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 2018-10-19 | CD | дата контракта | copied_from:formalized.packing_list.DeliveryTerms_Contract_PrDocumentDate |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наим инвойса | константа |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | № инвойса | copied_from:formalized.packing_list.DeliveryTerms_Invoice_PrDocumentNumber |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 2022-12-23 | CD | дата инвойса | copied_from:formalized.packing_list.DeliveryTerms_Invoice_PrDocumentDate |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наим упаковочного | константа |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | № упаковочного | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentNumber |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 2022-12-23 | CD | дата упаковочного | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentDate |
| 33 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 34 | doc_code | 04131 | CD | код документа Гр44 | константа |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наим документа Гр44 | константа |
| 36 | doc_number | 1000059769 /60285 /60389 /60491 | CD | номер документа Гр44 | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentNumber |
| 37 | doc_date | 2022-12-23 | CD | дата документа Гр44 | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentDate |
- _audit: 37

#### Goods Массив: Goods[7]
- _array_audit: 7

#### Element: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-4GX16GE-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[1].GoodsDescription |
| 02 | GoodsQuantity | 2 | CD | кол-во мест | copied_from:formalized.packing_list.Goods[1].GoodsQuantity |
| 03 | GrossWeightQuantity | 45.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 04 | NetWeightQuantity | 38.000 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 05 | PakingQuantity | 19 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[1].PakingQuantity |
- _item_audit: 5

#### Element: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-LITE-2GX8T-L3-L3 | CD | описание строки | copied_from:formalized.packing_list.Goods[2].GoodsDescription |
| 02 | GoodsQuantity | 2 | CD | кол-во мест | copied_from:formalized.packing_list.Goods[2].GoodsQuantity |
| 03 | GrossWeightQuantity | 15.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 04 | NetWeightQuantity | 13.500 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 05 | PakingQuantity | 15 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[2].PakingQuantity |
- _item_audit: 5

#### Element: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-4SFP8T-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[3].GoodsDescription |
| 02 | GoodsQuantity | 0 | CD | кол-во мест | note: упаковано совместно |
| 03 | GrossWeightQuantity | 8.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 04 | NetWeightQuantity | 7.500 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 05 | PakingQuantity | 5 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[3].PakingQuantity |
- _item_audit: 5

#### Element: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-LITE-8T-L3-L3 | CD | описание строки | copied_from:formalized.packing_list.Goods[4].GoodsDescription |
| 02 | GoodsQuantity | 0 | CD | кол-во мест | note: упаковано совместно |
| 03 | GrossWeightQuantity | 1.500 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 04 | NetWeightQuantity | 1.250 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 05 | PakingQuantity | 1 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[4].PakingQuantity |
- _item_audit: 5

#### Element: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX16GE-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[5].GoodsDescription |
| 02 | GoodsQuantity | 2 | CD | кол-во мест | copied_from:formalized.packing_list.Goods[5].GoodsQuantity |
| 03 | GrossWeightQuantity | 1.500 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 04 | NetWeightQuantity | 1.250 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 05 | PakingQuantity | 1 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[5].PakingQuantity |
- _item_audit: 5

#### Element: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX8T-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[6].GoodsDescription |
| 02 | GoodsQuantity | 0 | CD | кол-во мест | note: упаковано совместно |
| 03 | GrossWeightQuantity | 14.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 04 | NetWeightQuantity | 12.000 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 05 | PakingQuantity | 8 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[6].PakingQuantity |
- _item_audit: 5

#### Element: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX16T-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[7].GoodsDescription |
| 02 | GoodsQuantity | 0 | CD | кол-во мест | note: упаковано совместно |
| 03 | GrossWeightQuantity | 5.000 | CD | брутто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 04 | NetWeightQuantity | 4.500 | CD | нетто по строке | copied_from:ДТ_10005030/040123/3000837 |
| 05 | PakingQuantity | 3 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[7].PakingQuantity |
- _item_audit: 5

#### TransportMeans Массив: TransportMeans[1]
- _array_audit: 1

#### Element: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | ОТСУТСТВУЕТ | CD | рег номер ТС | авиадоставка |
| 02 | ModeCode | 40 | CD | код вида транспорта | авиаперевозка |
| 03 | NationalityCode | ОТСУТСТВУЕТ | CD | код национальности ТС | авиадоставка |
| 04 | MoverIndicator | ОТСУТСТВУЕТ | CD | признак тягач/прицеп | авиадоставка |
- _item_audit: 4

- `doc_status`: confirmed

### `document`: Air Waybill
- `uqi_prefix`: formalized.awb
- `xml_target_root`: AltaE3AWB
- `path`: alta\source\Kyland\01\md\АвиаНакладная.md
- `file_name`: АвиаНакладная.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | AgreedValuation | N.V.D. | CD | объявленная ценность | copied_from:formalized.awb.AgreedValuation |
| 02 | AgreedValuationCurrencyCode | CNY | CD | код валюты ценности | copied_from:formalized.awb.AgreedValuationCurrencyCode |
| 03 | Registration_AirlineIATACode | 876 | CD | IATA-код авиакомпании | copied_from:formalized.awb.Registration_AirlineIATACode |
| 04 | Registration_DocumentNumber | 41176586 | CD | номер авианакладной | copied_from:formalized.awb.Registration_DocumentNumber |
| 05 | Registration_DateInf | 2023-01-02 | CD | дата выпуска | copied_from:formalized.awb.Registration_DateInf |
| 06 | Consignor_NameInf | KYLAND TECHNOLOGY CO., LTD. | CD | грузоотправитель | copied_from:formalized.awb.Consignor_NameInf |
| 07 | Consignor_ShortName | KYLAND | CD | краткое наим грузоотправителя | copied_from:formalized.awb.Consignor_ShortName |
| 08 | Consignor_PostalAddress_CountryCode | CN | CD | код страны отправителя | cb:country |
| 09 | Consignor_Address_CounryName | КИТАЙ | CD | страна отправителя текст | cb:country |
| 10 | Consignor_Address_City | BEIJING | CD | город отправителя | copied_from:formalized.awb.Consignor_Address_City |
| 11 | Consignor_Address_StreetHouse | OFFICE ADDRESS: BUILDING NO.2, SHIXING AVENUE 30#, SHIJINGSHAN DISTRICT | CD | улица/дом отправителя | copied_from:formalized.awb.Consignor_Address_StreetHouse |
| 12 | Consignee_NameInf | SYMANITRON LTD | CD | грузополучатель | copied_from:formalized.awb.Consignee_NameInf |
| 13 | Consignee_ShortName | SYMANITRON | CD | краткое наим грузополучателя | copied_from:formalized.awb.Consignee_ShortName |
| 14 | Consignee_OGRNID | 1087746277740 | CD | ОГРН получателя | master_data:master_data.md |
| 15 | Consignee_INNID | 7720609470 | CD | ИНН получателя | copied_from:formalized.awb.Consignee_INNID |
| 16 | Consignee_KPPCode | 772001001 | CD | КПП получателя | master_data:master_data.md |
| 17 | Consignee_PostalAddress_PostalCode | 111675 | CD | индекс получателя | copied_from:formalized.awb.Consignee_PostalAddress_PostalCode |
| 18 | Consignee_PostalAddres_CountryCode | RU | CD | код страны получателя | copied_from:formalized.awb.Consignee_PostalAddres_CountryCode |
| 19 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя текст | cb:country |
| 20 | Consignee_Address_Region | ГОРОД МОСКВА | CD | регион получателя | master_data:master_data.md |
| 21 | Consignee_Address_City | MOSCOW | CD | город получателя | copied_from:formalized.awb.Consignee_Address_City |
| 22 | Consignee_Address_StreetHouse | AP.103. 17 RUDNEVKA STR. | CD | улица/дом получателя | copied_from:formalized.awb.Consignee_Address_StreetHouse |
| 23 | GoodsMovement | CHENGDU - SVO1 3U - MOSCOW | CD | сведения о движении | copied_from:formalized.awb.GoodsMovement |
| 24 | HandlingInfo | NOTIFY: SAME AS CNEE | CD | информация по обработке | copied_from:formalized.awb.HandlingInfo |
| 25 | IssueInfo_OrganizationName | CHINA SICHUAN AIRLINES | CD | авиакомпания | copied_from:formalized.awb.IssueInfo_OrganizationName |
| 26 | IssueInfo_Address_CountryCode | CN | CD | код страны авиакомпании | cb:country |
| 27 | IssueInfo_Address_CounryName | КИТАЙ | CD | страна авиакомпании текст | cb:country |
| 28 | AWBGoodsInfo_TotalPlacesQuantity | 6 | CD | общее количество мест | copied_from:formalized.awb.AWBGoodsInfo_TotalPlacesQuantity |
| 29 | AWBGoodsInfo_WeightUnitQualifierCode | K | CD | код единицы веса | константа |
| 30 | AWBGoodsInfo_GrossWeightQuantity | 90 | CD | общий вес брутто | copied_from:formalized.awb.AWBGoodsInfo_GrossWeightQuantity |
| 31 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 32 | doc_code | 02017 | CD | код документа Гр44 | константа |
| 33 | doc_name | АВИАНАКЛАДНАЯ | CD | наим документа Гр44 | константа |
| 34 | doc_number | 876-41176586 | CD | номер в формате IATA-DocNum | copied_from:formalized.awb.number |
| 35 | doc_date | 2023-01-02 | CD | дата документа Гр44 | copied_from:formalized.awb.Registration_DateInf |
- _audit: 35

#### AWBGoodsInfo_AWBGoods Массив: AWBGoodsInfo_AWBGoods[1]
- _array_audit: 1

#### Element: AWBGoodsInfo_AWBGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PlacesQuantity | 6 | CD | количество мест | copied_from:formalized.awb.AWBGoodsInfo_AWBGoods[1].PlacesQuantity |
| 02 | WeightUnitQualifierCode | K | CD | единица измерения веса | константа |
| 03 | GrossWeightQuantity | 90 | CD | вес брутто позиции | copied_from:formalized.awb.AWBGoodsInfo_AWBGoods[1].GrossWeightQuantity |
| 04 | CommodityItemNum | 1 | CD | порядковый номер позиции | константа |
| 05 | GoodsCommodityCode | 8517620003 | CD | код ТН ВЭД позиции | copied_from:ДТ_10005030/040123/3000837 |
| 06 | FactPlacesQuantity | 6 | CD | фактическое кол-во мест | copied_from:formalized.awb.AWBGoodsInfo_AWBGoods[1].FactPlacesQuantity |
| 07 | GoodsDescription | INDUSTRIAL ETHERNET SWITCH DIMS(cm): 62*52*33*5 37*33*23*1 VOL: 0.56CBM | CD | наименование и описание груза | copied_from:formalized.awb.AWBGoodsInfo_AWBGoods[1].GoodsDescription |
- _item_audit: 7

- `doc_status`: confirmed

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\Kyland\01\md\ПЛАТЕЖКА.md
- `file_name`: ПЛАТЕЖКА.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | константа |
| 02 | PaymentModeCode | 1 | CD | код способа платежа | константа |
| 03 | PaymentAmount | 25255.00 | CD | сумма платежа | copied_from:formalized.payment_order_1.PaymentAmount |
| 04 | TransactionKind | 01 | CD | вид операции | константа |
| 05 | Priority | . | CD | очередность | константа |
| 06 | Purpose | SALE AND PURCHASE CONTRACT lm191018/Kyl date 19/10/18 Invoice 1000059769 /60285 /60389 /60491 date 12/12/22 | CD | назначение платежа | copied_from:formalized.payment_order_1.Purpose |
| 07 | ValueSpelledOut | Двадцать пять тысяч двести пятьдесят пять долларов США 00 центов | CD | сумма прописью | copied_from:formalized.payment_order_1.ValueSpelledOut |
| 08 | DocumentReference_PrDocumentNumber | 30 | CD | номер платежного поручения | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber |
| 09 | DocumentReference_PrDocumentDate | 2022-12-15 | CD | дата платежного поручения | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate |
| 10 | Payer_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | плательщик | master_data:master_data.md |
| 11 | Payer_INN | 7720609470 | CD | ИНН плательщика | copied_from:formalized.payment_order_1.Payer_INN |
| 12 | Payer_KPP | 772001001 | CD | КПП плательщика | master_data:master_data.md |
| 13 | Payer_Bank_BankName | АО ЮниКредит Банк, Номер счета 06037583USDCOCA101 | CD | банк плательщика | copied_from:formalized.payment_order_1.Payer_Bank_BankName |
| 14 | Payee_OrganizationName | Kyland Technology Co., Ltd. | CD | получатель платежа | copied_from:formalized.payment_order_1.Payee_OrganizationName |
| 15 | Payee_Bank_BankName | BANK OF NINGBO, SWIFT BKNBCN2NBEI | CD | банк получателя | copied_from:formalized.payment_order_1.Payee_Bank_BankName |
| 16 | PersonSurname | ОТСУТСТВУЕТ | CD | фамилия подписанта | электронный перевод |
| 17 | PersonName | ОТСУТСТВУЕТ | CD | имя подписанта | электронный перевод |
| 18 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 19 | doc_code | 04023 | CD | код документа Гр44 | константа |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наим документа Гр44 | константа |
| 21 | doc_number | 30 | CD | номер документа Гр44 | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber |
| 22 | doc_date | 2022-12-15 | CD | дата документа Гр44 | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate |
- _audit: 22

- `doc_status`: confirmed

### `document`: Service Invoice
- `uqi_prefix`: formalized.service_invoice
- `xml_target_root`: AltaServiceInvoice
- `path`: alta\source\Kyland\01\md\Счет на оплату № VIG2227802 от 28.12.2022.md
- `file_name`: Счет на оплату № VIG2227802 от 28.12.2022.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CD | признак документа | константа |
| 02 | TotalServiceCost | 1615.00 | CD | итого по услугам | copied_from:formalized.service_invoice.TotalServiceCost |
| 03 | Currency | USD | CD | валюта итого | copied_from:formalized.service_invoice.Currency |
| 04 | ServiceProvider_Name | ООО "ВиАйДжи Кастомс" | CD | исполнитель услуг | copied_from:formalized.service_invoice.ServiceProvider_Name |
| 05 | BankName | СМОЛЕНСКОЕ ОТДЕЛЕНИЕ N8609 ПАО СБЕРБАНК г. Смоленск, БИК 046614632, Сч. № 30101810000000000632, Сч. № 40702810959000016812 | CD | банк исполнителя | copied_from:formalized.service_invoice.BankName |
| 06 | ContractDetails_PrDocumentNumber | 279 | CD | № договора на услуги | copied_from:formalized.service_invoice.ContractDetails_PrDocumentNumber |
| 07 | ContractDetails_PrDocumentDate | 2019-07-16 | CD | дата договора на услуги | copied_from:formalized.service_invoice.ContractDetails_PrDocumentDate |
| 08 | PrDocumentNumber | BDE000168 | CD | номер заказа | copied_from:formalized.service_invoice.PrDocumentNumber |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CD | дата заказа | константа |
| 10 | Registration_PrDocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование счета | copied_from:formalized.service_invoice.Registration_PrDocumentName |
| 11 | Registration_PrDocumentNumber | VIG2227802 | CD | номер счета | copied_from:formalized.service_invoice.Registration_PrDocumentNumber |
| 12 | Registration_PrDocumentDate | 2022-12-28 | CD | дата счета | copied_from:formalized.service_invoice.Registration_PrDocumentDate |
| 13 | Consignor_OrganizationName | ОТСУТСТВУЕТ | CD | грузоотправитель | константа |
| 14 | PostalCode | ОТСУТСТВУЕТ | CD | почтовый индекс | константа |
| 15 | CountryCode | ОТСУТСТВУЕТ | CD | страна | константа |
| 16 | CounryName | ОТСУТСТВУЕТ | CD | страна текст | константа |
| 17 | Region | ОТСУТСТВУЕТ | CD | регион | константа |
| 18 | Town | ОТСУТСТВУЕТ | CD | город/район | константа |
| 19 | StreetHouse | ОТСУТСТВУЕТ | CD | улица/дом | константа |
| 20 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | грузополучатель | master_data:master_data.md |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1087746277740 | CD | ОГРН | master_data:master_data.md |
| 22 | Consignee_RFOrganizationFeatures_INN | 7720609470 | CD | ИНН | master_data:master_data.md |
| 23 | Consignee_RFOrganizationFeatures_KPP | 772001001 | CD | КПП | master_data:master_data.md |
| 24 | PostalCode | 111675 | CD | почтовый индекс | master_data:master_data.md |
| 25 | CountryCode | RU | CD | страна | master_data:master_data.md |
| 26 | CounryName | РОССИЯ | CD | страна текст | master_data:master_data.md |
| 27 | Region | ГОРОД МОСКВА | CD | регион | master_data:master_data.md |
| 28 | Town | МОСКВА | CD | город | master_data:master_data.md |
| 29 | StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица | master_data:master_data.md |
| 30 | House | 17 | CD | дом | master_data:master_data.md |
| 31 | Room | 103 | CD | офис/кв | master_data:master_data.md |
| 32 | Signature_Choice | 2 | CD | вариант подписи | константа |
| 33 | IndividualEntrepreneur_PersonSurname | ОТСУТСТВУЕТ | CD | фамилия ИП | константа |
| 34 | IndividualEntrepreneur_PersonName | ОТСУТСТВУЕТ | CD | имя ИП | константа |
| 35 | IndividualEntrepreneur_PersonMiddleName | ОТСУТСТВУЕТ | CD | отчество ИП | константа |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Минакова | CD | фамилия руководителя | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_Director_PersonSurname |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Н. | CD | имя руководителя | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_Director_PersonName |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | В. | CD | отчество руководителя | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_Director_PersonMiddleName |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Емец | CD | фамилия бухгалтера | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | М. | CD | имя бухгалтера | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_ChiefAccountant_PersonName |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | В. | CD | отчество бухгалтера | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName |
| 42 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 43 | doc_code | 04031 | CD | код документа Гр44 | константа |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наим документа Гр44 | константа |
| 45 | doc_number | VIG2227802 | CD | номер документа Гр44 | copied_from:formalized.service_invoice.Registration_PrDocumentNumber |
| 46 | doc_date | 2022-12-28 | CD | дата документа Гр44 | copied_from:formalized.service_invoice.Registration_PrDocumentDate |
| 47 | transport_to_border | 1615.00 | CD | стоимость до границы | copied_from:formalized.service_invoice.transport_to_border |
| 48 | transport_currency | USD | CD | валюта стоимости | copied_from:formalized.service_invoice.transport_currency |
- _audit: 48

#### ServiceDescription Массив: ServiceDescription[1]
- _array_audit: 1

#### Element: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Организация перевозки груза по маршруту: Йичанг, КИТАЙ - Аэропорт Шереметьево - Авианакладная № 876-41176586 | CD | описание услуги | copied_from:formalized.service_invoice.ServiceDescription[1].GoodsDescription |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:formalized.service_invoice.ServiceDescription[1].CurrencyCode |
| 03 | ServiceName | Йичанг, КИТАЙ - Аэропорт Шереметьево | CD | наименование/маршрут | copied_from:formalized.service_invoice.ServiceDescription[1].ServiceName |
| 04 | TaxRate | 0 | CD | ставка налога | copied_from:formalized.service_invoice.ServiceDescription[1].TaxRate |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:formalized.service_invoice.ServiceDescription[1].TaxSum |
| 06 | ServiceCost_Amount | 1615.00 | CD | стоимость строки | copied_from:formalized.service_invoice.ServiceDescription[1].ServiceCost_Amount |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | copied_from:formalized.service_invoice.ServiceDescription[1].ServiceCost_Currency |
- _item_audit: 7

- `doc_status`: confirmed

### `document`: Insurance Invoice
- `uqi_prefix`: formalized.insurance_invoice
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Счет на оплату № 27611 от 27 декабря 2022 г.md
- `file_name`: Счет на оплату № 27611 от 27 декабря 2022 г.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование документа | copied_from:formalized.insurance_invoice.DocumentHead_DocumentName |
| 03 | DocumentHead_DocumentDate | 2022-12-27 | CD | дата документа | copied_from:formalized.insurance_invoice.DocumentHead_DocumentDate |
| 04 | DocumentHead_DocumentNumber | VIG2227611 | CD | номер документа | copied_from:formalized.insurance_invoice.DocumentHead_DocumentNumber |
| 05 | TextPara | link:alta\source\Kyland\01\md\Счет на оплату № 27611 от 27 декабря 2022 г.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 04111 | CD | код документа Гр44 | константа |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наим документа Гр44 | константа |
| 09 | doc_number | VIG2227611 | CD | номер документа Гр44 | copied_from:formalized.insurance_invoice.DocumentHead_DocumentNumber |
| 10 | doc_date | 2022-12-27 | CD | дата документа Гр44 | copied_from:formalized.insurance_invoice.DocumentHead_DocumentDate |
| 11 | insurance_to_border | 5186.02 | CD | стоимость страхования | copied_from:formalized.insurance_invoice.insurance_to_border |
| 12 | insurance_currency | RUB | CD | валюта страхования | copied_from:formalized.insurance_invoice.insurance_currency |
- _audit: 12

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_1
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-2GX16GE-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-2GX16GE-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-2GX16GE-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_2
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-2GX16T-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-2GX16T-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-2GX16T-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_3
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-2GX8T-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-2GX8T-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-2GX8T-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_4
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-4GX16GE-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-4GX16GE-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-4GX16GE-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_5
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-4SFP8T-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-4SFP8T-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-4SFP8T-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_6
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-LITE-2GX8T-L3-L3_TechDescription.md
- `file_name`: Kyland SYM3000A-LITE-2GX8T-L3-L3_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-LITE-2GX8T-L3-L3_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_7
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-LITE-8T-L3-L3_TechDescription.md
- `file_name`: Kyland SYM3000A-LITE-8T-L3-L3_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-LITE-8T-L3-L3_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

## 3. master_data:

### `document`: Contract
- `uqi_prefix`: master_data.contract
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 03011 | CD | код документа Гр44 | константа |
| 03 | doc_name | КОНТРАКТ | CD | наим документа Гр44 | константа |
| 04 | doc_number | Im191018/Kyl | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2018-10-19 | CD | дата документа Гр44 | master_data.md |
- _audit: 5

- `doc_status`: confirmed

### `document`: Supplementary Contract
- `uqi_prefix`: master_data.supplementary_contract_1
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 03012 | CD | код документа Гр44 | константа |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наим документа Гр44 | константа |
| 04 | doc_number | 221211 | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2022-12-11 | CD | дата документа Гр44 | master_data.md |
- _audit: 5

- `doc_status`: confirmed

### `document`: UNK
- `uqi_prefix`: master_data.unk
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 03031 | CD | код документа Гр44 | константа |
| 03 | doc_name | УНК | CD | наим документа Гр44 | константа |
| 04 | doc_number | 18100214110000097211 | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2018-10-25 | CD | дата документа Гр44 | master_data.md |
- _audit: 5

- `doc_status`: confirmed

### `document`: EGRUL
- `uqi_prefix`: master_data.egrul
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | наим организации | master_data.md |
| 02 | ShortName | ООО "СИМАНИТРОН" | CD | краткое наим | master_data.md |
| 03 | OGRN | 1087746277740 | CD | ОГРН | master_data.md |
| 04 | INN | 7720609470 | CD | ИНН | master_data.md |
| 05 | KPP | 772001001 | CD | КПП | master_data.md |
| 06 | Address_PostalCode | 111675 | CD | индекс | master_data.md |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 08 | Address_CounryName | РОССИЯ | CD | страна текст | master_data.md |
| 09 | Address_Region | ГОРОД МОСКВА | CD | регион | master_data.md |
| 10 | Address_City | МОСКВА | CD | город | master_data.md |
| 11 | Address_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data.md |
| 12 | Phone | +7495 981-62-44 | CD | телефон | master_data.md |
| 13 | Email | info@symanitron.ru | CD | e-mail | master_data.md |
| 14 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 15 | doc_code | 04011 | CD | код документа Гр44 | константа |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наим документа Гр44 | константа |
| 17 | doc_number | ЮЭ9965-19-16744108 | CD | номер документа Гр44 | master_data.md |
| 18 | doc_date | 2019-02-14 | CD | дата документа Гр44 | master_data.md |
- _audit: 18

- `doc_status`: confirmed

### `document`: Personal Passport
- `uqi_prefix`: master_data.passport
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | master_data.md |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | master_data.md |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | master_data.md |
| 04 | CardSeries | 63 09 | CD | серия паспорта | master_data.md |
| 05 | CardNumber | 449948 | CD | номер паспорта | master_data.md |
| 06 | CardDate | 2010-03-11 | CD | дата выдачи | master_data.md |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data.md |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_data.md |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | master_data.md |
| 10 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 11 | doc_code | 11001 | CD | код документа Гр44 | константа |
| 12 | doc_name | ПАСПОРТ | CD | наим документа Гр44 | константа |
| 13 | doc_number | 63 09 449948 | CD | номер документа Гр44 | master_data.md |
| 14 | doc_date | 2010-03-11 | CD | дата документа Гр44 | master_data.md |
- _audit: 14

- `doc_status`: confirmed

### `document`: Letter of Attorney
- `uqi_prefix`: master_data.letter_of_attorney
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data.md |
| 02 | DocumentDate | 2026-02-01 | CD | дата доверенности | master_data.md |
| 03 | EndDate | 2026-12-31 | CD | действительна до | master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_data.md |
| 05 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 06 | doc_code | 11004 | CD | код документа Гр44 | константа |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наим документа Гр44 | константа |
| 08 | doc_number | 1 | CD | номер документа Гр44 | master_data.md |
| 09 | doc_date | 2026-02-01 | CD | дата документа Гр44 | master_data.md |
- _audit: 9

- `doc_status`: confirmed

### `document`: Transport Contract
- `uqi_prefix`: master_data.transport_contract
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 04033 | CD | код документа Гр44 | константа |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наим документа Гр44 | константа |
| 04 | doc_number | КООО/26651/М | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2025-05-13 | CD | дата документа Гр44 | master_data.md |
- _audit: 5

- `doc_status`: confirmed

### `document`: Conformity Document
- `uqi_prefix`: master_data.conformity_document_1
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 01402 | CD | код документа Гр44 | константа |
| 03 | doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим документа Гр44 | константа |
| 04 | doc_number | ЕАЭС N RU Д-CN.РА01.В.65848/21 | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2021-05-14 | CD | дата документа Гр44 | master_data.md |
| 06 | date_start | 2021-05-14 | CD | дата начала действия | master_data.md |
| 07 | date_end | 2026-05-12 | CD | дата окончания действия | master_data.md |
- _audit: 7

- `doc_status`: confirmed

### `document`: Conformity Document
- `uqi_prefix`: master_data.conformity_document_2
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 01402 | CD | код документа Гр44 | константа |
| 03 | doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим документа Гр44 | константа |
| 04 | doc_number | ЕАЭС N RU Д-CN.РА03.В.59715/21 | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2021-12-17 | CD | дата документа Гр44 | master_data.md |
| 06 | date_start | 2021-12-17 | CD | дата начала действия | master_data.md |
| 07 | date_end | 2026-12-15 | CD | дата окончания действия | master_data.md |
- _audit: 7

- `doc_status`: confirmed

## 4. non_formalized:

### `document`: Goods Description
- `uqi_prefix`: non_formalized.goods_description_1
- `path`: alta\source\Kyland\01\md\CI, PL final_Invoice.md
- `file_name`: CI, PL final_Invoice.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | служебный признак Гр44 | константа |
- _audit: 1

#### goods Массив: goods[7]
- _array_audit: 7

#### Element: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:ДТ_10005030/040123/3000837 |
- _item_audit: 2

#### Element: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ С ПОНИЖЕННЫМ ПОТРЕБЛЕНИЕМ ПИТАНИЯ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:ДТ_10005030/040123/3000837 |
- _item_audit: 2

#### Element: goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:ДТ_10005030/040123/3000837 |
- _item_audit: 2

#### Element: goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | description | ПРОМЫШЛЕННЫЙ КОММУТАТОР С ПОНИЖЕННЫМ ПОТРЕБЛЕНИЕМ ПИТАНИЯ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:ДТ_10005030/040123/3000837 |
- _item_audit: 2

#### Element: goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | description | ПРОМЫШЛЕННЫЙ КОММУТАТОР ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:ДТ_10005030/040123/3000837 |
- _item_audit: 2

#### Element: goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:ДТ_10005030/040123/3000837 |
- _item_audit: 2

#### Element: goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | copied_from:ДТ_10005030/040123/3000837 |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:ДТ_10005030/040123/3000837 |
- _item_audit: 2

- `doc_status`: confirmed

### `document`: Storage Report
- `uqi_prefix`: non_formalized.svh
- `path`: alta\source\Kyland\01\md\АвиаНакладная.md
- `file_name`: АвиаНакладная.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10005/060917/10048/2 | CD | номер лицензии СВХ | copied_from:ДТ_10005030/040123/3000837 |
| 02 | actual_gross_weight | 90 | CD | фактический вес брутто | copied_from:formalized.awb.AWBGoodsInfo_GrossWeightQuantity |
| 03 | actual_places | 6 | CD | фактическое кол-во мест | copied_from:formalized.awb.AWBGoodsInfo_TotalPlacesQuantity |
| 04 | transport_reg_number | ОТСУТСТВУЕТ | CD | номер ТС при въезде | авиадоставка |
| 05 | doc_gr44 | false | CD | служебный признак Гр44 | константа |
| 06 | doc_code | 10061 | CD | код документа Гр44 | константа |
- _audit: 6

#### goods Массив: goods[1]
- _array_audit: 1

#### Element: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 8517620003 | CD | код товара | copied_from:ДТ_10005030/040123/3000837 |
| 02 | places | 6 | CD | кол-во мест по строке | авиадоставка, ДО-1 отсутствует |
| 03 | gross_weight_kg | 90 | CD | вес брутто по строке | авиадоставка, ДО-1 отсутствует |
| 04 | cost | 25255.00 | CD | стоимость по строке | авиадоставка, ДО-1 отсутствует |
| 05 | currency_code | USD | CD | код валюты | авиадоставка, ДО-1 отсутствует |
- _item_audit: 5

- `doc_status`: confirmed

### Итогo, по файлу:

`total_unreliable_fields`: 0
`primary_status`: confirmed

## 5. Нерешенные вопросы (Issues)

## 6. `unreliable_fields`:
# Первичные данные

## 1. meta:
- `название кейса`: Kyland
- `путь к папке поставки`: alta\source\Kyland\01
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 7 товаров
- `источники данных`: md + master_data.md + расчет по dt_rules.md

## 2. formalized:

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: alta\source\Kyland\01\md\CI, PL final_Invoice.md
- `file_name`: CI, PL final_Invoice.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | USD | CD | валюта инвойса | copied_from:formalized.invoice_1.CurrencyCode |
| 02 | DocumentCode | 04021 | CD | код вида документа | константа |
| 03 | PlacesQuantity | 6 | CD | кол-во грузовых мест | copied_from:formalized.packing_list.TotalCartons |
| 04 | PlacesDescription | Cartons | CD | описание мест | copied_from:formalized.packing_list.PlacesDescription |
| 05 | GrossWeightQuantity | 90 | CD | общий вес брутто | copied_from:formalized.packing_list.GrossWeightQuantity |
| 06 | NetWeightQuantity | 78 | CD | общий вес нетто | copied_from:formalized.packing_list.NetWeightQuantity |
| 07 | GCost | 25255.00 | CD | системное поле стоимости | copied_from:formalized.invoice_1.TotalCost |
| 08 | TotalCost | 25255.00 | CD | итого по инвойсу | copied_from:formalized.invoice_1.TotalCost |
| 09 | DeliveryTerms_DeliveryPlace | Yichang | CD | место поставки по Incoterms | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryPlace |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий поставки | cb:delivery_tems |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | copied_from:formalized.invoice_1.DeliveryTerms_DispatchCountryCode |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | copied_from:formalized.invoice_1.DeliveryTerms_TradingCountryCode |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | copied_from:formalized.invoice_1.DeliveryTerms_DestinationCountryCode |
| 15 | Registration_PrDocumentName | COMMERCIAL INVOICE | CD | наименование документа | copied_from:formalized.invoice_1.Registration_PrDocumentName |
| 16 | Registration_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | номер инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentNumber |
| 17 | Registration_PrDocumentDate | 2022-12-12 | CD | дата инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentDate |
| 18 | Contract_PrDocumentNumber | lm191018/Kyl | CD | № контракта-ссылки | copied_from:formalized.invoice_1.Contract_PrDocumentNumber |
| 19 | Contract_PrDocumentDate | 2018-10-19 | CD | дата контракта-ссылки | copied_from:formalized.invoice_1.Contract_PrDocumentDate |
| 20 | Buyer_CompanyID | 7720609470 | CD | ИНН покупателя | master_data:master_data.md |
| 21 | Buyer_KPPCode | 772001001 | CD | КПП покупателя | master_data:master_data.md |
| 22 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | наименование покупателя | master_data:master_data.md |
| 23 | Buyer_PostalAddress_PostalCode | 111675 | CD | почтовый индекс | master_data:master_data.md |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | master_data:master_data.md |
| 25 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя текст | master_data:master_data.md |
| 26 | Buyer_PostalAddress_Region | ГОРОД МОСКВА | CD | регион | master_data:master_data.md |
| 27 | Buyer_PostalAddress_City | МОСКВА | CD | город | master_data:master_data.md |
| 28 | Buyer_PostalAddress_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data:master_data.md |
| 29 | Seler_Name | Kyland Technology Co., Ltd. | CD | продавец | copied_from:formalized.invoice_1.Seler_Name |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | cb:country |
| 31 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца текст | cb:country |
| 32 | Seler_PostalAddress_Region | Beijing | CD | регион продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_Region |
| 33 | Seler_PostalAddress_City | Beijing | CD | город/район продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_City |
| 34 | Seler_PostalAddress_StreetHouse | Building No.2, Shixing Avenue 30# Shijingshan District | CD | улица/дом продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 35 | Consignor_OrganizationName | Kyland Technology Co., Ltd. | CD | грузоотправитель | note: нормализация: consignor=seller |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | note: нормализация: consignor=seller |
| 37 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя текст | note: нормализация: consignor=seller |
| 38 | Consignor_Address_Region | Beijing | CD | регион | note: нормализация: consignor=seller |
| 39 | Consignor_Address_City | Beijing | CD | город/район | note: нормализация: consignor=seller |
| 40 | Consignor_Address_StreetHouse | Building No.2, Shixing Avenue 30# Shijingshan District | CD | улица/дом | note: нормализация: consignor=seller |
| 41 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | грузополучатель | master_data:master_data.md |
| 42 | Consignee_OGRN | 1087746277740 | CD | ОГРН | master_data:master_data.md |
| 43 | Consignee_INN | 7720609470 | CD | ИНН | master_data:master_data.md |
| 44 | Consignee_KPP | 772001001 | CD | КПП | master_data:master_data.md |
| 45 | Consignee_Address_PostalCode | 111675 | CD | почтовый индекс | master_data:master_data.md |
| 46 | Consignee_Address_CountryCode | RU | CD | страна | master_data:master_data.md |
| 47 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | master_data:master_data.md |
| 48 | Consignee_Address_Region | ГОРОД МОСКВА | CD | регион | master_data:master_data.md |
| 49 | Consignee_Address_City | МОСКВА | CD | город | master_data:master_data.md |
| 50 | Consignee_Address_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data:master_data.md |
| 51 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 52 | doc_code | 04021 | CD | код документа Гр44 | константа |
| 53 | doc_name | ИНВОЙС | CD | наим документа Гр44 | константа |
| 54 | doc_number | 1000059769 /60285 /60389 /60491 | CD | номер документа Гр44 | copied_from:formalized.invoice_1.Registration_PrDocumentNumber |
| 55 | doc_date | 2022-12-12 | CD | дата документа Гр44 | copied_from:formalized.invoice_1.Registration_PrDocumentDate |
- _audit: 55

#### InvoiceGoods Массив: InvoiceGoods[7]
- _array_audit: 7

#### Element: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | константа |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[1].GoodsDescription |
| 03 | GoodsQuantity | 19 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[1].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 44.240 | CD | брутто по строке | calculated:dt_rules.md (балансировка -1г) |
| 08 | NetWeightQuantity | 38.342 | CD | нетто по строке | calculated:dt_rules.md |
| 09 | Price | 683.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[1].Price |
| 10 | TotalCost | 12977.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[1].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-4GX16GE-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | константа |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[2].GoodsDescription |
| 03 | GoodsQuantity | 15 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[2].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 15.441 | CD | брутто по строке | calculated:dt_rules.md |
| 08 | NetWeightQuantity | 13.382 | CD | нетто по строке | calculated:dt_rules.md |
| 09 | Price | 347.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[2].Price |
| 10 | TotalCost | 5205.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[2].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[2].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[2].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-LITE-2GX8T-L3-L3 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[2].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | константа |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[3].GoodsDescription |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[3].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 9.192 | CD | брутто по строке | calculated:dt_rules.md |
| 08 | NetWeightQuantity | 7.966 | CD | нетто по строке | calculated:dt_rules.md |
| 09 | Price | 347.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[3].Price |
| 10 | TotalCost | 1735.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[3].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[3].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[3].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-4SFP8T-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[3].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | константа |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[4].GoodsDescription |
| 03 | GoodsQuantity | 1 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[4].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 1.029 | CD | брутто по строке | calculated:dt_rules.md |
| 08 | NetWeightQuantity | 0.892 | CD | нетто по строке | calculated:dt_rules.md |
| 09 | Price | 215.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[4].Price |
| 10 | TotalCost | 215.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[4].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[4].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[4].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-LITE-8T-L3-L3 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[4].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 4 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | константа |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[5].GoodsDescription |
| 03 | GoodsQuantity | 1 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[5].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 2.329 | CD | брутто по строке | calculated:dt_rules.md |
| 08 | NetWeightQuantity | 2.018 | CD | нетто по строке | calculated:dt_rules.md |
| 09 | Price | 604.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[5].Price |
| 10 | TotalCost | 604.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[5].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[5].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[5].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX16GE-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[5].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 5 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | константа |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[6].GoodsDescription |
| 03 | GoodsQuantity | 8 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[6].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 10.784 | CD | брутто по строке | calculated:dt_rules.md |
| 08 | NetWeightQuantity | 9.346 | CD | нетто по строке | calculated:dt_rules.md |
| 09 | Price | 368.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[6].Price |
| 10 | TotalCost | 2944.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[6].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[6].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[6].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX8T-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[6].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 6 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

#### Element: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CD | код ТН ВЭД | константа |
| 02 | GoodsDescription | Industrial Ethernet Switch | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[7].GoodsDescription |
| 03 | GoodsQuantity | 3 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[7].GoodsQuantity |
| 04 | goods_supplementary_quantity | | CD | кол-во в доп ед | ОТСУТСТВУЕТ |
| 05 | goods_supplementary_uom_name | | CD | наим доп ед | ОТСУТСТВУЕТ |
| 06 | MeasureUnitQualifierName | | CD | ед изм доп кол-ва | ОТСУТСТВУЕТ |
| 07 | GrossWeightQuantity | 6.985 | CD | брутто по строке | calculated:dt_rules.md |
| 08 | NetWeightQuantity | 6.054 | CD | нетто по строке | calculated:dt_rules.md |
| 09 | Price | 525.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[7].Price |
| 10 | TotalCost | 1575.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[7].TotalCost |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[7].AdditionalGoodsDescription_Manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[7].AdditionalGoodsDescription_TradeMark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | константа |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX16T-L2-L2 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[7].AdditionalGoodsDescription_GoodsModel |
| 16 | dt_item_index | 7 | CD | индекс товара ДТ | авто-группировка |
| 17 | dt_tovg_index | 1 | CD | индекс позиции в товаре | авто-группировка |
- _item_audit: 17

- `doc_status`: confirmed

### `document`: Packing List
- `uqi_prefix`: formalized.packing_list
- `xml_target_root`: AltaE2PACK
- `path`: alta\source\Kyland\01\md\CI, PL final_PackingList.md
- `file_name`: CI, PL final_PackingList.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 90 | CD | общий вес брутто | copied_from:formalized.packing_list.GrossWeightQuantity |
| 02 | NetWeightQuantity | 78 | CD | общий вес нетто | copied_from:formalized.packing_list.NetWeightQuantity |
| 03 | Consignor_OrganizationName | Kyland Technology Co., Ltd. | CD | грузоотправитель | copied_from:formalized.packing_list.Consignor_OrganizationName |
| 04 | Consignor_ShortName | Kyland | CD | краткое наим грузоотправителя | copied_from:formalized.packing_list.Consignor_ShortName |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | cb:country |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя текст | cb:country |
| 07 | Consignor_Address_Region | Beijing | CD | регион | copied_from:formalized.packing_list.Consignor_Address_Region |
| 08 | Consignor_Address_City | Beijing | CD | город/район | copied_from:formalized.packing_list.Consignor_Address_City |
| 09 | Consignor_Address_StreetHouse | Building No.2, Shixing Avenue 30# Shijingshan District | CD | улица/дом | copied_from:formalized.packing_list.Consignor_Address_StreetHouse |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | грузополучатель | master_data:master_data.md |
| 11 | Consignee_ShortName | ООО "СИМАНИТРОН" | CD | краткое наим грузополучателя | master_data:master_data.md |
| 12 | Consignee_OGRN | 1087746277740 | CD | ОГРН | master_data:master_data.md |
| 13 | Consignee_INN | 7720609470 | CD | ИНН | master_data:master_data.md |
| 14 | Consignee_KPP | 772001001 | CD | КПП | master_data:master_data.md |
| 15 | Consignee_Address_PostalCode | 111675 | CD | почтовый индекс | master_data:master_data.md |
| 16 | Consignee_Address_CountryCode | RU | CD | страна | master_data:master_data.md |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | master_data:master_data.md |
| 18 | Consignee_Address_Region | ГОРОД МОСКВА | CD | регион | master_data:master_data.md |
| 19 | Consignee_Address_City | МОСКВА | CD | город | master_data:master_data.md |
| 20 | Consignee_Address_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data:master_data.md |
| 21 | DeliveryTerms_DeliveryPlace | Yichang | CD | место поставки по Incoterms | copied_from:formalized.packing_list.DeliveryTerms_DeliveryPlace |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | cb:delivery_tems |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from:formalized.packing_list.DeliveryTerms_DeliveryTermsStringCode |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наим контракта | константа |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | lm191018/Kyl | CD | № контракта | copied_from:formalized.packing_list.DeliveryTerms_Contract_PrDocumentNumber |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 2018-10-19 | CD | дата контракта | copied_from:formalized.packing_list.DeliveryTerms_Contract_PrDocumentDate |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наим инвойса | константа |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | № инвойса | copied_from:formalized.packing_list.DeliveryTerms_Invoice_PrDocumentNumber |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 2022-12-23 | CD | дата инвойса | copied_from:formalized.packing_list.DeliveryTerms_Invoice_PrDocumentDate |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наим упаковочного | константа |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | № упаковочного | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentNumber |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 2022-12-23 | CD | дата упаковочного | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentDate |
| 33 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 34 | doc_code | 04131 | CD | код документа Гр44 | константа |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наим документа Гр44 | константа |
| 36 | doc_number | 1000059769 /60285 /60389 /60491 | CD | номер документа Гр44 | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentNumber |
| 37 | doc_date | 2022-12-23 | CD | дата документа Гр44 | copied_from:formalized.packing_list.DeliveryTerms_Registration_PrDocumentDate |
- _audit: 37

#### Goods Массив: Goods[7]
- _array_audit: 7

#### Element: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-4GX16GE-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[1].GoodsDescription |
| 02 | GoodsQuantity | 2 | CD | кол-во мест | copied_from:formalized.packing_list.Goods[1].GoodsQuantity |
| 03 | GrossWeightQuantity | 44.240 | CD | брутто по строке | calculated:dt_rules.md (балансировка -1г) |
| 04 | NetWeightQuantity | 38.342 | CD | нетто по строке | calculated:dt_rules.md |
| 05 | PakingQuantity | 19 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[1].PakingQuantity |
- _item_audit: 5

#### Element: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-LITE-2GX8T-L3-L3 | CD | описание строки | copied_from:formalized.packing_list.Goods[2].GoodsDescription |
| 02 | GoodsQuantity | 2 | CD | кол-во мест | copied_from:formalized.packing_list.Goods[2].GoodsQuantity |
| 03 | GrossWeightQuantity | 15.441 | CD | брутто по строке | calculated:dt_rules.md |
| 04 | NetWeightQuantity | 13.382 | CD | нетто по строке | calculated:dt_rules.md |
| 05 | PakingQuantity | 15 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[2].PakingQuantity |
- _item_audit: 5

#### Element: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-4SFP8T-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[3].GoodsDescription |
| 02 | GoodsQuantity | 0 | CD | кол-во мест | note: упаковано совместно |
| 03 | GrossWeightQuantity | 9.192 | CD | брутто по строке | calculated:dt_rules.md |
| 04 | NetWeightQuantity | 7.966 | CD | нетто по строке | calculated:dt_rules.md |
| 05 | PakingQuantity | 5 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[3].PakingQuantity |
- _item_audit: 5

#### Element: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-LITE-8T-L3-L3 | CD | описание строки | copied_from:formalized.packing_list.Goods[4].GoodsDescription |
| 02 | GoodsQuantity | 0 | CD | кол-во мест | note: упаковано совместно |
| 03 | GrossWeightQuantity | 1.029 | CD | брутто по строке | calculated:dt_rules.md |
| 04 | NetWeightQuantity | 0.892 | CD | нетто по строке | calculated:dt_rules.md |
| 05 | PakingQuantity | 1 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[4].PakingQuantity |
- _item_audit: 5

#### Element: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX16GE-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[5].GoodsDescription |
| 02 | GoodsQuantity | 2 | CD | кол-во мест | copied_from:formalized.packing_list.Goods[5].GoodsQuantity |
| 03 | GrossWeightQuantity | 2.329 | CD | брутто по строке | calculated:dt_rules.md |
| 04 | NetWeightQuantity | 2.018 | CD | нетто по строке | calculated:dt_rules.md |
| 05 | PakingQuantity | 1 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[5].PakingQuantity |
- _item_audit: 5

#### Element: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX8T-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[6].GoodsDescription |
| 02 | GoodsQuantity | 0 | CD | кол-во мест | note: упаковано совместно |
| 03 | GrossWeightQuantity | 10.784 | CD | брутто по строке | calculated:dt_rules.md |
| 04 | NetWeightQuantity | 9.346 | CD | нетто по строке | calculated:dt_rules.md |
| 05 | PakingQuantity | 8 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[6].PakingQuantity |
- _item_audit: 5

#### Element: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX16T-L2-L2 | CD | описание строки | copied_from:formalized.packing_list.Goods[7].GoodsDescription |
| 02 | GoodsQuantity | 0 | CD | кол-во мест | note: упаковано совместно |
| 03 | GrossWeightQuantity | 6.985 | CD | брутто по строке | calculated:dt_rules.md |
| 04 | NetWeightQuantity | 6.054 | CD | нетто по строке | calculated:dt_rules.md |
| 05 | PakingQuantity | 3 | CD | кол-во в упаковке | copied_from:formalized.packing_list.Goods[7].PakingQuantity |
- _item_audit: 5

#### TransportMeans Массив: TransportMeans[1]
- _array_audit: 1

#### Element: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | ОТСУТСТВУЕТ | CD | рег номер ТС | авиадоставка |
| 02 | ModeCode | 40 | CD | код вида транспорта | авиаперевозка |
| 03 | NationalityCode | ОТСУТСТВУЕТ | CD | код национальности ТС | авиадоставка |
| 04 | MoverIndicator | ОТСУТСТВУЕТ | CD | признак тягач/прицеп | авиадоставка |
- _item_audit: 4

- `doc_status`: confirmed

### `document`: Air Waybill
- `uqi_prefix`: formalized.awb
- `xml_target_root`: AltaE3AWB
- `path`: alta\source\Kyland\01\md\АвиаНакладная.md
- `file_name`: АвиаНакладная.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | AgreedValuation | N.V.D. | CD | объявленная ценность | copied_from:formalized.awb.AgreedValuation |
| 02 | AgreedValuationCurrencyCode | CNY | CD | код валюты ценности | copied_from:formalized.awb.AgreedValuationCurrencyCode |
| 03 | Registration_AirlineIATACode | 876 | CD | IATA-код авиакомпании | copied_from:formalized.awb.Registration_AirlineIATACode |
| 04 | Registration_DocumentNumber | 41176586 | CD | номер авианакладной | copied_from:formalized.awb.Registration_DocumentNumber |
| 05 | Registration_DateInf | 2023-01-02 | CD | дата выпуска | copied_from:formalized.awb.Registration_DateInf |
| 06 | Consignor_NameInf | KYLAND TECHNOLOGY CO., LTD. | CD | грузоотправитель | copied_from:formalized.awb.Consignor_NameInf |
| 07 | Consignor_ShortName | KYLAND | CD | краткое наим грузоотправителя | copied_from:formalized.awb.Consignor_ShortName |
| 08 | Consignor_PostalAddress_CountryCode | CN | CD | код страны отправителя | cb:country |
| 09 | Consignor_Address_CounryName | КИТАЙ | CD | страна отправителя текст | cb:country |
| 10 | Consignor_Address_City | BEIJING | CD | город отправителя | copied_from:formalized.awb.Consignor_Address_City |
| 11 | Consignor_Address_StreetHouse | OFFICE ADDRESS: BUILDING NO.2, SHIXING AVENUE 30#, SHIJINGSHAN DISTRICT | CD | улица/дом отправителя | copied_from:formalized.awb.Consignor_Address_StreetHouse |
| 12 | Consignee_NameInf | SYMANITRON LTD | CD | грузополучатель | copied_from:formalized.awb.Consignee_NameInf |
| 13 | Consignee_ShortName | SYMANITRON | CD | краткое наим грузополучателя | copied_from:formalized.awb.Consignee_ShortName |
| 14 | Consignee_OGRNID | 1087746277740 | CD | ОГРН получателя | master_data:master_data.md |
| 15 | Consignee_INNID | 7720609470 | CD | ИНН получателя | copied_from:formalized.awb.Consignee_INNID |
| 16 | Consignee_KPPCode | 772001001 | CD | КПП получателя | master_data:master_data.md |
| 17 | Consignee_PostalAddress_PostalCode | 111675 | CD | индекс получателя | copied_from:formalized.awb.Consignee_PostalAddress_PostalCode |
| 18 | Consignee_PostalAddres_CountryCode | RU | CD | код страны получателя | copied_from:formalized.awb.Consignee_PostalAddres_CountryCode |
| 19 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя текст | cb:country |
| 20 | Consignee_Address_Region | ГОРОД МОСКВА | CD | регион получателя | master_data:master_data.md |
| 21 | Consignee_Address_City | MOSCOW | CD | город получателя | copied_from:formalized.awb.Consignee_Address_City |
| 22 | Consignee_Address_StreetHouse | AP.103. 17 RUDNEVKA STR. | CD | улица/дом получателя | copied_from:formalized.awb.Consignee_Address_StreetHouse |
| 23 | GoodsMovement | CHENGDU - SVO1 3U - MOSCOW | CD | сведения о движении | copied_from:formalized.awb.GoodsMovement |
| 24 | HandlingInfo | NOTIFY: SAME AS CNEE | CD | информация по обработке | copied_from:formalized.awb.HandlingInfo |
| 25 | IssueInfo_OrganizationName | CHINA SICHUAN AIRLINES | CD | авиакомпания | copied_from:formalized.awb.IssueInfo_OrganizationName |
| 26 | IssueInfo_Address_CountryCode | CN | CD | код страны авиакомпании | cb:country |
| 27 | IssueInfo_Address_CounryName | КИТАЙ | CD | страна авиакомпании текст | cb:country |
| 28 | AWBGoodsInfo_TotalPlacesQuantity | 6 | CD | общее количество мест | copied_from:formalized.awb.AWBGoodsInfo_TotalPlacesQuantity |
| 29 | AWBGoodsInfo_WeightUnitQualifierCode | K | CD | код единицы веса | константа |
| 30 | AWBGoodsInfo_GrossWeightQuantity | 90 | CD | общий вес брутто | copied_from:formalized.awb.AWBGoodsInfo_GrossWeightQuantity |
| 31 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 32 | doc_code | 02017 | CD | код документа Гр44 | константа |
| 33 | doc_name | АВИАНАКЛАДНАЯ | CD | наим документа Гр44 | константа |
| 34 | doc_number | 876-41176586 | CD | номер в формате IATA-DocNum | copied_from:formalized.awb.number |
| 35 | doc_date | 2023-01-02 | CD | дата документа Гр44 | copied_from:formalized.awb.Registration_DateInf |
- _audit: 35

#### AWBGoodsInfo_AWBGoods Массив: AWBGoodsInfo_AWBGoods[1]
- _array_audit: 1

#### Element: AWBGoodsInfo_AWBGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PlacesQuantity | 6 | CD | количество мест | copied_from:formalized.awb.AWBGoodsInfo_AWBGoods[1].PlacesQuantity |
| 02 | WeightUnitQualifierCode | K | CD | единица измерения веса | константа |
| 03 | GrossWeightQuantity | 90 | CD | вес брутто позиции | copied_from:formalized.awb.AWBGoodsInfo_AWBGoods[1].GrossWeightQuantity |
| 04 | CommodityItemNum | 1 | CD | порядковый номер позиции | константа |
| 05 | GoodsCommodityCode | 8517620003 | CD | код ТН ВЭД позиции | cb:doc |
| 06 | FactPlacesQuantity | 6 | CD | фактическое кол-во мест | copied_from:formalized.awb.AWBGoodsInfo_AWBGoods[1].FactPlacesQuantity |
| 07 | GoodsDescription | INDUSTRIAL ETHERNET SWITCH DIMS(cm): 62*52*33*5 37*33*23*1 VOL: 0.56CBM | CD | наименование и описание груза | copied_from:formalized.awb.AWBGoodsInfo_AWBGoods[1].GoodsDescription |
- _item_audit: 7

- `doc_status`: confirmed

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\Kyland\01\md\ПЛАТЕЖКА.md
- `file_name`: ПЛАТЕЖКА.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | константа |
| 02 | PaymentModeCode | 1 | CD | код способа платежа | константа |
| 03 | PaymentAmount | 25255.00 | CD | сумма платежа | copied_from:formalized.payment_order_1.PaymentAmount |
| 04 | TransactionKind | 01 | CD | вид операции | константа |
| 05 | Priority | . | CD | очередность | константа |
| 06 | Purpose | SALE AND PURCHASE CONTRACT lm191018/Kyl date 19/10/18 Invoice 1000059769 /60285 /60389 /60491 date 12/12/22 | CD | назначение платежа | copied_from:formalized.payment_order_1.Purpose |
| 07 | ValueSpelledOut | Двадцать пять тысяч двести пятьдесят пять долларов США 00 центов | CD | сумма прописью | copied_from:formalized.payment_order_1.ValueSpelledOut |
| 08 | DocumentReference_PrDocumentNumber | 30 | CD | номер платежного поручения | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber |
| 09 | DocumentReference_PrDocumentDate | 2022-12-15 | CD | дата платежного поручения | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate |
| 10 | Payer_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | плательщик | master_data:master_data.md |
| 11 | Payer_INN | 7720609470 | CD | ИНН плательщика | copied_from:formalized.payment_order_1.Payer_INN |
| 12 | Payer_KPP | 772001001 | CD | КПП плательщика | master_data:master_data.md |
| 13 | Payer_Bank_BankName | АО ЮниКредит Банк, Номер счета 06037583USDCOCA101 | CD | банк плательщика | copied_from:formalized.payment_order_1.Payer_Bank_BankName |
| 14 | Payee_OrganizationName | Kyland Technology Co., Ltd. | CD | получатель платежа | copied_from:formalized.payment_order_1.Payee_OrganizationName |
| 15 | Payee_Bank_BankName | BANK OF NINGBO, SWIFT BKNBCN2NBEI | CD | банк получателя | copied_from:formalized.payment_order_1.Payee_Bank_BankName |
| 16 | PersonSurname | ОТСУТСТВУЕТ | CD | фамилия подписанта | электронный перевод |
| 17 | PersonName | ОТСУТСТВУЕТ | CD | имя подписанта | электронный перевод |
| 18 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 19 | doc_code | 04023 | CD | код документа Гр44 | константа |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наим документа Гр44 | константа |
| 21 | doc_number | 30 | CD | номер документа Гр44 | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber |
| 22 | doc_date | 2022-12-15 | CD | дата документа Гр44 | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate |
- _audit: 22

- `doc_status`: confirmed

### `document`: Service Invoice
- `uqi_prefix`: formalized.service_invoice
- `xml_target_root`: AltaServiceInvoice
- `path`: alta\source\Kyland\01\md\Счет на оплату № VIG2227802 от 28.12.2022.md
- `file_name`: Счет на оплату № VIG2227802 от 28.12.2022.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CD | признак документа | константа |
| 02 | TotalServiceCost | 1615.00 | CD | итого по услугам | copied_from:formalized.service_invoice.TotalServiceCost |
| 03 | Currency | USD | CD | валюта итого | copied_from:formalized.service_invoice.Currency |
| 04 | ServiceProvider_Name | ООО "ВиАйДжи Кастомс" | CD | исполнитель услуг | copied_from:formalized.service_invoice.ServiceProvider_Name |
| 05 | BankName | СМОЛЕНСКОЕ ОТДЕЛЕНИЕ N8609 ПАО СБЕРБАНК г. Смоленск, БИК 046614632, Сч. № 30101810000000000632, Сч. № 40702810959000016812 | CD | банк исполнителя | copied_from:formalized.service_invoice.BankName |
| 06 | ContractDetails_PrDocumentNumber | 279 | CD | № договора на услуги | copied_from:formalized.service_invoice.ContractDetails_PrDocumentNumber |
| 07 | ContractDetails_PrDocumentDate | 2019-07-16 | CD | дата договора на услуги | copied_from:formalized.service_invoice.ContractDetails_PrDocumentDate |
| 08 | PrDocumentNumber | BDE000168 | CD | номер заказа | copied_from:formalized.service_invoice.PrDocumentNumber |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CD | дата заказа | константа |
| 10 | Registration_PrDocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование счета | copied_from:formalized.service_invoice.Registration_PrDocumentName |
| 11 | Registration_PrDocumentNumber | VIG2227802 | CD | номер счета | copied_from:formalized.service_invoice.Registration_PrDocumentNumber |
| 12 | Registration_PrDocumentDate | 2022-12-28 | CD | дата счета | copied_from:formalized.service_invoice.Registration_PrDocumentDate |
| 13 | Consignor_OrganizationName | ОТСУТСТВУЕТ | CD | грузоотправитель | константа |
| 14 | PostalCode | ОТСУТСТВУЕТ | CD | почтовый индекс | константа |
| 15 | CountryCode | ОТСУТСТВУЕТ | CD | страна | константа |
| 16 | CounryName | ОТСУТСТВУЕТ | CD | страна текст | константа |
| 17 | Region | ОТСУТСТВУЕТ | CD | регион | константа |
| 18 | Town | ОТСУТСТВУЕТ | CD | город/район | константа |
| 19 | StreetHouse | ОТСУТСТВУЕТ | CD | улица/дом | константа |
| 20 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | грузополучатель | master_data:master_data.md |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1087746277740 | CD | ОГРН | master_data:master_data.md |
| 22 | Consignee_RFOrganizationFeatures_INN | 7720609470 | CD | ИНН | master_data:master_data.md |
| 23 | Consignee_RFOrganizationFeatures_KPP | 772001001 | CD | КПП | master_data:master_data.md |
| 24 | PostalCode | 111675 | CD | почтовый индекс | master_data:master_data.md |
| 25 | CountryCode | RU | CD | страна | master_data:master_data.md |
| 26 | CounryName | РОССИЯ | CD | страна текст | master_data:master_data.md |
| 27 | Region | ГОРОД МОСКВА | CD | регион | master_data:master_data.md |
| 28 | Town | МОСКВА | CD | город | master_data:master_data.md |
| 29 | StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица | master_data:master_data.md |
| 30 | House | 17 | CD | дом | master_data:master_data.md |
| 31 | Room | 103 | CD | офис/кв | master_data:master_data.md |
| 32 | Signature_Choice | 2 | CD | вариант подписи | константа |
| 33 | IndividualEntrepreneur_PersonSurname | ОТСУТСТВУЕТ | CD | фамилия ИП | константа |
| 34 | IndividualEntrepreneur_PersonName | ОТСУТСТВУЕТ | CD | имя ИП | константа |
| 35 | IndividualEntrepreneur_PersonMiddleName | ОТСУТСТВУЕТ | CD | отчество ИП | константа |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Минакова | CD | фамилия руководителя | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_Director_PersonSurname |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Н. | CD | имя руководителя | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_Director_PersonName |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | В. | CD | отчество руководителя | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_Director_PersonMiddleName |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Емец | CD | фамилия бухгалтера | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | М. | CD | имя бухгалтера | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_ChiefAccountant_PersonName |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | В. | CD | отчество бухгалтера | copied_from:formalized.service_invoice.SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName |
| 42 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 43 | doc_code | 04031 | CD | код документа Гр44 | константа |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наим документа Гр44 | константа |
| 45 | doc_number | VIG2227802 | CD | номер документа Гр44 | copied_from:formalized.service_invoice.Registration_PrDocumentNumber |
| 46 | doc_date | 2022-12-28 | CD | дата документа Гр44 | copied_from:formalized.service_invoice.Registration_PrDocumentDate |
| 47 | transport_to_border | 1615.00 | CD | стоимость до границы | copied_from:formalized.service_invoice.transport_to_border |
| 48 | transport_currency | USD | CD | валюта стоимости | copied_from:formalized.service_invoice.transport_currency |
- _audit: 48

#### ServiceDescription Массив: ServiceDescription[1]
- _array_audit: 1

#### Element: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Организация перевозки груза по маршруту: Йичанг, КИТАЙ - Аэропорт Шереметьево - Авианакладная № 876-41176586 | CD | описание услуги | copied_from:formalized.service_invoice.ServiceDescription[1].GoodsDescription |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:formalized.service_invoice.ServiceDescription[1].CurrencyCode |
| 03 | ServiceName | Йичанг, КИТАЙ - Аэропорт Шеремевето | CD | наименование/маршрут | copied_from:formalized.service_invoice.ServiceDescription[1].ServiceName |
| 04 | TaxRate | 0 | CD | ставка налога | copied_from:formalized.service_invoice.ServiceDescription[1].TaxRate |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:formalized.service_invoice.ServiceDescription[1].TaxSum |
| 06 | ServiceCost_Amount | 1615.00 | CD | стоимость строки | copied_from:formalized.service_invoice.ServiceDescription[1].ServiceCost_Amount |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | copied_from:formalized.service_invoice.ServiceDescription[1].ServiceCost_Currency |
- _item_audit: 7

- `doc_status`: confirmed

### `document`: Insurance Invoice
- `uqi_prefix`: formalized.insurance_invoice
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Счет на оплату № 27611 от 27 декабря 2022 г.md
- `file_name`: Счет на оплату № 27611 от 27 декабря 2022 г.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование документа | copied_from:formalized.insurance_invoice.DocumentHead_DocumentName |
| 03 | DocumentHead_DocumentDate | 2022-12-27 | CD | дата документа | copied_from:formalized.insurance_invoice.DocumentHead_DocumentDate |
| 04 | DocumentHead_DocumentNumber | VIG2227611 | CD | номер документа | copied_from:formalized.insurance_invoice.DocumentHead_DocumentNumber |
| 05 | TextPara | link:alta\source\Kyland\01\md\Счет на оплату № 27611 от 27 декабря 2022 г.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 04111 | CD | код документа Гр44 | константа |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наим документа Гр44 | константа |
| 09 | doc_number | VIG2227611 | CD | номер документа Гр44 | copied_from:formalized.insurance_invoice.DocumentHead_DocumentNumber |
| 10 | doc_date | 2022-12-27 | CD | дата документа Гр44 | copied_from:formalized.insurance_invoice.DocumentHead_DocumentDate |
| 11 | insurance_to_border | 5186.02 | CD | стоимость страхования | copied_from:formalized.insurance_invoice.insurance_to_border |
| 12 | insurance_currency | RUB | CD | валюта страхования | copied_from:formalized.insurance_invoice.insurance_currency |
- _audit: 12

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_1
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-2GX16GE-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-2GX16GE-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-2GX16GE-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_2
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-2GX16T-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-2GX16T-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-2GX16T-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_3
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-2GX8T-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-2GX8T-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-2GX8T-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_4
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-4GX16GE-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-4GX16GE-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-4GX16GE-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_5
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-4SFP8T-L2-L2_TechDescription.md
- `file_name`: Kyland SYM3000A-4SFP8T-L2-L2_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-4SFP8T-L2-L2_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_6
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-LITE-2GX8T-L3-L3_TechDescription.md
- `file_name`: Kyland SYM3000A-LITE-2GX8T-L3-L3_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-LITE-2GX8T-L3-L3_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description_7
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Kyland SYM3000A-LITE-8T-L3-L3_TechDescription.md
- `file_name`: Kyland SYM3000A-LITE-8T-L3-L3_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование техописания | константа |
| 03 | DocumentHead_DocumentDate | 2023-01-03 | CD | дата техописания | copied_from:ДТ_10005030/040123/3000837 |
| 04 | DocumentHead_DocumentNumber | 117СИ0301 | CD | номер техописания | copied_from:ДТ_10005030/040123/3000837 |
| 05 | TextPara | link:alta\source\Kyland\01\md\Kyland SYM3000A-LITE-8T-L3-L3_TechDescription.md | CD | ссылка на файл-источник | константа |
| 06 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 07 | doc_code | 05999 | CD | код документа Гр44 | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим документа Гр44 | константа |
| 09 | doc_number | 117СИ0301 | CD | номер документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
| 10 | doc_date | 2023-01-03 | CD | дата документа Гр44 | copied_from:ДТ_10005030/040123/3000837 |
- _audit: 10

- `doc_status`: confirmed

## 3. master_data:

### `document`: Contract
- `uqi_prefix`: master_data.contract
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 03011 | CD | код документа Гр44 | константа |
| 03 | doc_name | КОНТРАКТ | CD | наим документа Гр44 | константа |
| 04 | doc_number | Im191018/Kyl | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2018-10-19 | CD | дата документа Гр44 | master_data.md |
- _audit: 5

- `doc_status`: confirmed

### `document`: Supplementary Contract
- `uqi_prefix`: master_data.supplementary_contract_1
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 03012 | CD | код документа Гр44 | константа |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наим документа Гр44 | константа |
| 04 | doc_number | 221211 | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2022-12-11 | CD | дата документа Гр44 | master_data.md |
- _audit: 5

- `doc_status`: confirmed

### `document`: UNK
- `uqi_prefix`: master_data.unk
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 03031 | CD | код документа Гр44 | константа |
| 03 | doc_name | УНК | CD | наим документа Гр44 | константа |
| 04 | doc_number | 18100214110000097211 | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2018-10-25 | CD | дата документа Гр44 | master_data.md |
- _audit: 5

- `doc_status`: confirmed

### `document`: EGRUL
- `uqi_prefix`: master_data.egrul
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | наим организации | master_data.md |
| 02 | ShortName | ООО "СИМАНИТРОН" | CD | краткое наим | master_data.md |
| 03 | OGRN | 1087746277740 | CD | ОГРН | master_data.md |
| 04 | INN | 7720609470 | CD | ИНН | master_data.md |
| 05 | KPP | 772001001 | CD | КПП | master_data.md |
| 06 | Address_PostalCode | 111675 | CD | индекс | master_data.md |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 08 | Address_CounryName | РОССИЯ | CD | страна текст | master_data.md |
| 09 | Address_Region | ГОРОД МОСКВА | CD | регион | master_data.md |
| 10 | Address_City | МОСКВА | CD | город | master_data.md |
| 11 | Address_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data.md |
| 12 | Phone | +7495 981-62-44 | CD | телефон | master_data.md |
| 13 | Email | info@symanitron.ru | CD | e-mail | master_data.md |
| 14 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 15 | doc_code | 04011 | CD | код документа Гр44 | константа |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наим документа Гр44 | константа |
| 17 | doc_number | ЮЭ9965-19-16744108 | CD | номер документа Гр44 | master_data.md |
| 18 | doc_date | 2019-02-14 | CD | дата документа Гр44 | master_data.md |
- _audit: 18

- `doc_status`: confirmed

### `document`: Personal Passport
- `uqi_prefix`: master_data.passport
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | master_data.md |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | master_data.md |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | master_data.md |
| 04 | CardSeries | 63 09 | CD | серия паспорта | master_data.md |
| 05 | CardNumber | 449948 | CD | номер паспорта | master_data.md |
| 06 | CardDate | 2010-03-11 | CD | дата выдачи | master_data.md |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data.md |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_data.md |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | master_data.md |
| 10 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 11 | doc_code | 11001 | CD | код документа Гр44 | константа |
| 12 | doc_name | ПАСПОРТ | CD | наим документа Гр44 | константа |
| 13 | doc_number | 63 09 449948 | CD | номер документа Гр44 | master_data.md |
| 14 | doc_date | 2010-03-11 | CD | дата документа Гр44 | master_data.md |
- _audit: 14

- `doc_status`: confirmed

### `document`: Letter of Attorney
- `uqi_prefix`: master_data.letter_of_attorney
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data.md |
| 02 | DocumentDate | 2026-02-01 | CD | дата доверенности | master_data.md |
| 03 | EndDate | 2026-12-31 | CD | действительна до | master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_data.md |
| 05 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 06 | doc_code | 11004 | CD | код документа Гр44 | константа |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наим документа Гр44 | константа |
| 08 | doc_number | 1 | CD | номер документа Гр44 | master_data.md |
| 09 | doc_date | 2026-02-01 | CD | дата документа Гр44 | master_data.md |
- _audit: 9

- `doc_status`: confirmed

### `document`: Transport Contract
- `uqi_prefix`: master_data.transport_contract
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 04033 | CD | код документа Гр44 | константа |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наим документа Гр44 | константа |
| 04 | doc_number | КООО/26651/М | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2025-05-13 | CD | дата документа Гр44 | master_data.md |
- _audit: 5

- `doc_status`: confirmed

### `document`: Conformity Document
- `uqi_prefix`: master_data.conformity_document_1
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 01402 | CD | код документа Гр44 | константа |
| 03 | doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим документа Гр44 | константа |
| 04 | doc_number | ЕАЭС N RU Д-CN.РА01.В.65848/21 | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2021-05-14 | CD | дата документа Гр44 | master_data.md |
| 06 | date_start | 2021-05-14 | CD | дата начала действия | master_data.md |
| 07 | date_end | 2026-05-12 | CD | дата окончания действия | master_data.md |
- _audit: 7

- `doc_status`: confirmed

### `document`: Conformity Document
- `uqi_prefix`: master_data.conformity_document_2
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак Гр44 | константа |
| 02 | doc_code | 01402 | CD | код документа Гр44 | константа |
| 03 | doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим документа Гр44 | константа |
| 04 | doc_number | ЕАЭС N RU Д-CN.РА03.В.59715/21 | CD | номер документа Гр44 | master_data.md |
| 05 | doc_date | 2021-12-17 | CD | дата документа Гр44 | master_data.md |
| 06 | date_start | 2021-12-17 | CD | дата начала действия | master_data.md |
| 07 | date_end | 2026-12-15 | CD | дата окончания действия | master_data.md |
- _audit: 7

- `doc_status`: confirmed

## 4. non_formalized:

### `document`: Goods Description
- `uqi_prefix`: non_formalized.goods_description_1
- `path`: alta\source\Kyland\01\md\CI, PL final_Invoice.md
- `file_name`: CI, PL final_Invoice.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | служебный признак Гр44 | константа |
- _audit: 1

#### goods Массив: goods[7]
- _array_audit: 7

#### Element: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | cb:doc |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:техописание |
- _item_audit: 2

#### Element: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | cb:doc |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ С ПОНИЖЕННЫМ ПОТРЕБЛЕНИЕМ ПИТАНИЯ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:техописание |
- _item_audit: 2

#### Element: goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | cb:doc |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:техописание |
- _item_audit: 2

#### Element: goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | cb:doc |
| 02 | description | ПРОМЫШЛЕННЫЙ КОММУТАТОР С ПОНИЖЕННЫМ ПОТРЕБЛЕНИЕМ ПИТАНИЯ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:техописание |
- _item_audit: 2

#### Element: goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | cb:doc |
| 02 | description | ПРОМЫШЛЕННЫЙ КОММУТАТОР ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:техописание |
- _item_audit: 2

#### Element: goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | cb:doc |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:техописание |
- _item_audit: 2

#### Element: goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CD | ТН ВЭД | cb:doc |
| 02 | description | ПРОМЫШЛЕННЫЕ КОММУТАТОРЫ ДЛЯ УПРАВЛЕНИЯ В СЕТЯХ ETHERNET, НЕ ЛОМ ЭЛЕКТРООБОРУДОВАНИЯ, НЕ ПРЕДНАЗНАЧЕНЫ ДЛЯ НЕГЛАСНОГО ПОЛУЧЕНИЯ ИНФОРМАЦИИ, КОМПЛЕКТУЮЩИЕ ДЛЯ СЕРВЕРОВ И СЕРВЕРНОГО ОБОРУДОВАНИЯ, ГРАЖДАНСКОГО ПРИМЕНЕНИЯ. ПОСТАВЛЯЮТСЯ С КОНСОЛЬНЫМ КАБЕЛЕМ. ПРОИЗВ.: KYLAND TECHNOLOGY CO., LTD., (TM) KYLAND | CD | описание товара | copied_from:техописание |
- _item_audit: 2

- `doc_status`: confirmed

### `document`: Storage Report
- `uqi_prefix`: non_formalized.svh
- `path`: alta\source\Kyland\01\md\АвиаНакладная.md
- `file_name`: АвиаНакладная.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10005/060917/10048/2 | CD | номер лицензии СВХ | copied_from:справочник СВХ |
| 02 | actual_gross_weight | 90 | CD | фактический вес брутто | copied_from:formalized.awb.AWBGoodsInfo_GrossWeightQuantity |
| 03 | actual_places | 6 | CD | фактическое кол-во мест | copied_from:formalized.awb.AWBGoodsInfo_TotalPlacesQuantity |
| 04 | transport_reg_number | ОТСУТСТВУЕТ | CD | номер ТС при въезде | авиадоставка |
| 05 | doc_gr44 | false | CD | служебный признак Гр44 | константа |
| 06 | doc_code | 10061 | CD | код документа Гр44 | константа |
- _audit: 6

#### goods Массив: goods[1]
- _array_audit: 1

#### Element: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 8517620003 | CD | код товара | cb:doc |
| 02 | places | 6 | CD | кол-во мест по строке | авиадоставка, ДО-1 отсутствует |
| 03 | gross_weight_kg | 90 | CD | вес брутто по строке | авиадоставка, ДО-1 отсутствует |
| 04 | cost | 25255.00 | CD | стоимость по строке | авиадоставка, ДО-1 отсутствует |
| 05 | currency_code | USD | CD | код валюты | авиадоставка, ДО-1 отсутствует |
- _item_audit: 5

- `doc_status`: confirmed

### Итогo, по файлу:

`total_unreliable_fields`: 0
`primary_status`: confirmed

[WARNING] КРИТИЧЕСКИЙ ВЕС ТАРЫ (КАРАУЛ!): Вес нетто поставщика (78.000 кг) превышает чистый вес приборов по техничкам (73.440 кг) более чем на 5% (разница 6.2%). Веса брутто и нетто по товарам пересчитаны и сбалансированы по правилам dt_rules.md.

## 5. Нерешенные вопросы (Issues)

## 6. `unreliable_fields`:
