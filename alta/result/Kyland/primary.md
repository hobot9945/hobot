# Первичные данные

## 1. meta:
- `название кейса`: Kyland
- `путь к папке поставки`: alta\source\Kyland\01
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 7 товаров
- `источники данных`: md + operator_provided_data.md + master_data.md

## 2. formalized:

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: alta\source\Kyland\01\md\CI, PL final_Invoice.md
- `file_name`: CI, PL final_Invoice.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | USD | CD | валюта инвойса | |
| 02 | DocumentCode | 04021 | CD | код вида документа | |
| 03 | PlacesQuantity | 6 | CD | кол-во грузовых мест | copied_from:formalized.packing_list.PlacesQuantity |
| 04 | PlacesDescription | КОРОБКА | CD | описание мест | copied_from:formalized.packing_list |
| 05 | GrossWeightQuantity | 90 | CD | общий вес брутто | copied_from:formalized.packing_list.GrossWeightQuantity |
| 06 | NetWeightQuantity | 78 | CD | общий вес нетто | copied_from:formalized.packing_list.NetWeightQuantity |
| 07 | GCost | 25255.00 | CD | системное поле стоимости | |
| 08 | TotalCost | 25255.00 | CD | полная стоимость | |
| 09 | DeliveryTerms_DeliveryPlace | Yichang | CD | место поставки по Incoterms | |
| 10 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 55 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | |
| 11 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 12 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 13 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 14 | Registration_PrDocumentName | ИНВОЙС | CD | наименование документа | |
| 15 | Registration_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | номер инвойса | |
| 16 | Registration_PrDocumentDate | 12.12.2022 | CO | дата инвойса | operator:дата инвойса |
| 17 | Contract_PrDocumentNumber | Im191018/Kyl | CD | № контракта-ссылки | master_data.md |
| 18 | Contract_PrDocumentDate | 19.10.2018 | CD | дата контракта-ссылки | master_data.md |
| 19 | Buyer_CompanyID | 7720609470 | CD | ИНН покупателя | master_data.md |
| 20 | Buyer_KPPCode | 772001001 | CD | КПП покупателя | master_data.md |
| 21 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | наименование покупателя | master_data.md |
| 22 | Buyer_PostalAddress_PostalCode | 111675 | CD | почтовый индекс | master_data.md |
| 23 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | master_data.md |
| 24 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя, текст | master_data.md |
| 25 | Buyer_PostalAddress_Region | ГОРОД МОСКВА | CD | регион | master_data.md |
| 26 | Buyer_PostalAddress_City | МОСКВА | CD | город | master_data.md |
| 27 | Buyer_PostalAddress_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис одной строкой | master_data.md |
| 28 | Seler_Name | Kyland Technology Co., Ltd. | CD | продавец | |
| 29 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | |
| 30 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | |
| 31 | Seler_PostalAddress_Region | Beijing | CD | регион продавца | |
| 32 | Seler_PostalAddress_City | Beijing | CD | город/район продавца | |
| 33 | Seler_PostalAddress_StreetHouse | Building No.2, Shixing Avenue 30#, Shijingshan District | CD | улица/дом одной строкой | |
| 34 | Consignor_OrganizationName | Kyland Technology Co., Ltd. | CD | грузоотправитель | copied_from:formalized.invoice_1.Seler_Name |
| 35 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | copied_from:formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 36 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | copied_from:formalized.invoice_1.Seler_PostalAddress_CounryName |
| 37 | Consignor_Address_Region | Beijing | CD | регион | copied_from:formalized.invoice_1.Seler_PostalAddress_Region |
| 38 | Consignor_Address_City | Beijing | CD | город/район | copied_from:formalized.invoice_1.Seler_PostalAddress_City |
| 39 | Consignor_Address_StreetHouse | Building No.2, Shixing Avenue 30#, Shijingshan District | CD | улица/дом одной строкой | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 40 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | грузополучатель | master_data.md |
| 41 | Consignee_OGRN | 1087746277740 | CD | ОГРН | master_data.md |
| 42 | Consignee_INN | 7720609470 | CD | ИНН | master_data.md |
| 43 | Consignee_KPP | 772001001 | CD | КПП | master_data.md |
| 44 | Consignee_Address_PostalCode | 117246 | CD | почтовый индекс | |
| 45 | Consignee_Address_CountryCode | RU | CD | страна | |
| 46 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 47 | Consignee_Address_Region | Москва | CD | регион | |
| 48 | Consignee_Address_City | Москва | CD | город | |
| 49 | Consignee_Address_StreetHouse | Научный проезд, д. 8, стр. 1, оф. 429 | CD | улица/дом/офис одной строкой | |
| 50 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 51 | doc_code | 04021 | CD | код документа | |
| 52 | doc_name | ИНВОЙС | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | |
| 53 | doc_number | 1000059769 /60285 /60389 /60491 | CD | номер инвойса | |
| 54 | doc_date | 12.12.2022 | CO | дата инвойса | operator:дата инвойса |

- _audit: 54

#### Массив: InvoiceGoods[7]
- _array_audit: 7

#### Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator:коды ТН ВЭД |
| 02 | GoodsDescription | Промышленный коммутатор Ethernet | CD | описание товара | |
| 03 | GoodsQuantity | 19 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | | | кол-во в доп. ед. изм. | |
| 05 | goods_supplementary_uom_name | | | наим. доп. ед. изм. | |
| 06 | MeasureUnitQualifierName | | | ед. изм. доп. кол-ва | |
| 07 | GrossWeightQuantity | 32.884 | CD | брутто по строке | weight_rules:распределено |
| 08 | NetWeightQuantity | 28.5 | CD | нетто по строке | |
| 09 | Price | 683.00 | CD | цена за единицу | |
| 10 | TotalCost | 12977.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | Kyland | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-4GX16GE-L2-L2 | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator:коды ТН ВЭД |
| 02 | GoodsDescription | Промышленный коммутатор Ethernet | CD | описание товара | |
| 03 | GoodsQuantity | 15 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | | | кол-во в доп. ед. изм. | |
| 05 | goods_supplementary_uom_name | | | наим. доп. ед. изм. | |
| 06 | MeasureUnitQualifierName | | | ед. изм. доп. кол-ва | |
| 07 | GrossWeightQuantity | 25.962 | CD | брутто по строке | weight_rules:распределено |
| 08 | NetWeightQuantity | 22.5 | CD | нетто по строке | |
| 09 | Price | 347.00 | CD | цена за единицу | |
| 10 | TotalCost | 5205.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | Kyland | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-LITE-2GX8T-L3-L3 | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator:коды ТН ВЭД |
| 02 | GoodsDescription | Промышленный коммутатор Ethernet | CD | описание товара | |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | | | кол-во в доп. ед. изм. | |
| 05 | goods_supplementary_uom_name | | | наим. доп. ед. изм. | |
| 06 | MeasureUnitQualifierName | | | ед. изм. доп. кол-ва | |
| 07 | GrossWeightQuantity | 8.654 | CD | брутто по строке | weight_rules:распределено |
| 08 | NetWeightQuantity | 7.5 | CD | нетто по строке | |
| 09 | Price | 347.00 | CD | цена за единицу | |
| 10 | TotalCost | 1735.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | Kyland | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-4SFP8T-L2-L2 | CD | модель/модификация | |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator:коды ТН ВЭД |
| 02 | GoodsDescription | Промышленный коммутатор Ethernet | CD | описание товара | |
| 03 | GoodsQuantity | 1 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | | | кол-во в доп. ед. изм. | |
| 05 | goods_supplementary_uom_name | | | наим. доп. ед. изм. | |
| 06 | MeasureUnitQualifierName | | | ед. изм. доп. кол-ва | |
| 07 | GrossWeightQuantity | 1.731 | CD | брутто по строке | weight_rules:распределено |
| 08 | NetWeightQuantity | 1.5 | CD | нетто по строке | |
| 09 | Price | 215.00 | CD | цена за единицу | |
| 10 | TotalCost | 215.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | Kyland | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-LITE-8T-L3-L3 | CD | модель/модификация | |
| 16 | dt_item_index | 4 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator:коды ТН ВЭД |
| 02 | GoodsDescription | Промышленный коммутатор Ethernet | CD | описание товара | |
| 03 | GoodsQuantity | 1 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | | | кол-во в доп. ед. изм. | |
| 05 | goods_supplementary_uom_name | | | наим. доп. ед. изм. | |
| 06 | MeasureUnitQualifierName | | | ед. изм. доп. кол-ва | |
| 07 | GrossWeightQuantity | 1.731 | CD | брутто по строке | weight_rules:распределено |
| 08 | NetWeightQuantity | 1.5 | CD | нетто по строке | |
| 09 | Price | 604.00 | CD | цена за единицу | |
| 10 | TotalCost | 604.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | Kyland | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX16GE-L2-L2 | CD | модель/модификация | |
| 16 | dt_item_index | 5 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator:коды ТН ВЭД |
| 02 | GoodsDescription | Промышленный коммутатор Ethernet | CD | описание товара | |
| 03 | GoodsQuantity | 8 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | | | кол-во в доп. ед. изм. | |
| 05 | goods_supplementary_uom_name | | | наим. доп. ед. изм. | |
| 06 | MeasureUnitQualifierName | | | ед. изм. доп. кол-ва | |
| 07 | GrossWeightQuantity | 13.846 | CD | брутто по строке | weight_rules:распределено |
| 08 | NetWeightQuantity | 12 | CD | нетто по строке | |
| 09 | Price | 368.00 | CD | цена за единицу | |
| 10 | TotalCost | 2944.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | Kyland | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX8T-L2-L2 | CD | модель/модификация | |
| 16 | dt_item_index | 6 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator:коды ТН ВЭД |
| 02 | GoodsDescription | Промышленный коммутатор Ethernet | CD | описание товара | |
| 03 | GoodsQuantity | 3 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | | | кол-во в доп. ед. изм. | |
| 05 | goods_supplementary_uom_name | | | наим. доп. ед. изм. | |
| 06 | MeasureUnitQualifierName | | | ед. изм. доп. кол-ва | |
| 07 | GrossWeightQuantity | 5.192 | CD | брутто по строке | weight_rules:распределено |
| 08 | NetWeightQuantity | 4.5 | CD | нетто по строке | |
| 09 | Price | 525.00 | CD | цена за единицу | |
| 10 | TotalCost | 1575.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | Kyland | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX16T-L2-L2 | CD | модель/модификация | |
| 16 | dt_item_index | 7 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

- doc_status: confirmed
### `document`: Packing List
- `uqi_prefix`: formalized.packing_list
- `xml_target_root`: AltaE2PACK
- `path`: alta\source\Kyland\01\md\CI, PL final_PackingList.md
- `file_name`: CI, PL final_PackingList.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 90 | CD | общий вес брутто | |
| 02 | NetWeightQuantity | 78 | CD | общий вес нетто | |
| 03 | Consignor_OrganizationName | Kyland Technology Co., Ltd. | CD | грузоотправитель | |
| 04 | Consignor_ShortName | Kyland Technology Co., Ltd. | CD | краткое наименование | |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | |
| 07 | Consignor_Address_Region | Beijing | CD | регион | |
| 08 | Consignor_Address_City | Beijing | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | Building No.2, Shixing Avenue 30#, Shijingshan District | CD | улица/дом одной строкой | |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | грузополучатель | master_data.md |
| 11 | Consignee_ShortName | ООО "СИМАНИТРОН" | CD | краткое наименование | master_data.md |
| 12 | Consignee_OGRN | 1087746277740 | CD | ОГРН | master_data.md |
| 13 | Consignee_INN | 7720609470 | CD | ИНН | master_data.md |
| 14 | Consignee_KPP | 772001001 | CD | КПП | master_data.md |
| 15 | Consignee_Address_PostalCode | 117246 | CD | почтовый индекс | |
| 16 | Consignee_Address_CountryCode | RU | CD | страна | |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 18 | Consignee_Address_Region | Москва | CD | регион | |
| 19 | Consignee_Address_City | Москва | CD | город | |
| 20 | Consignee_Address_StreetHouse | Научный проезд, д. 8, стр. 1, оф. 429 | CD | улица/дом/офис одной строкой | |
| 21 | DeliveryTerms_DeliveryPlace | Yichang | CD | место поставки по Incoterms | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | КОНТРАКТ | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | Im191018/Kyl | CD | № контракта | master_data.md |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 19.10.2018 | CD | дата контракта | master_data.md |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИВОЙС | CD | ИВОЙС | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 12.12.2022 | CO | дата инвойса | operator:дата инвойса |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | УПАКОВОЧНЫЙ ЛИСТ | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | № упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 23.12.2022 | CD | дата упаковочного | |
| 33 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 34 | doc_code | 04131 | CD | код документа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | |
| 36 | doc_number | 1000059769 /60285 /60389 /60491 | CD | номер документа | |
| 37 | doc_date | 23.12.2022 | CD | дата документа | |

- _audit: 37

#### Массив: Goods[7]
- _array_audit: 7

#### Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-4GX16GE-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 32.884 | CD | брутто по строке | weight_rules:распределено |
| 04 | NetWeightQuantity | 28.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 19 | CD | кол-во упаковок | |

- _item_audit: 5

#### Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-LITE-2GX8T-L3-L3 | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 25.962 | CD | брутто по строке | weight_rules:распределено |
| 04 | NetWeightQuantity | 22.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 15 | CD | кол-во упаковок | |

- _item_audit: 5

#### Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX16T-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 5.192 | CD | брутто по строке | weight_rules:распределено |
| 04 | NetWeightQuantity | 4.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 3 | CD | кол-во упаковок | |

- _item_audit: 5

#### Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-4SFP8T-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | | | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 8.654 | CD | брутто по строке | weight_rules:распределено |
| 04 | NetWeightQuantity | 7.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 5 | CD | кол-во упаковок | |

- _item_audit: 5

#### Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-LITE-8T-L3-L3 | CD | описание строки | |
| 02 | GoodsQuantity | | | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 1.731 | CD | брутто по строке | weight_rules:распределено |
| 04 | NetWeightQuantity | 1.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 1 | CD | кол-во упаковок | |

- _item_audit: 5

#### Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX16GE-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | | | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 1.731 | CD | брутто по строке | weight_rules:распределено |
| 04 | NetWeightQuantity | 1.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 1 | CD | кол-во упаковок | |

- _item_audit: 5

#### Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX8T-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | | | кол-во мест в строке | |
| 03 | GrossWeightQuantity | 13.846 | CD | брутто по строке | weight_rules:распределено |
| 04 | NetWeightQuantity | 12 | CD | нетто по строке | |
| 05 | PakingQuantity | 8 | CD | кол-во упаковок | |

- _item_audit: 5

#### Массив: TransportMeans[1]
- _array_audit: 1

#### Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | 3U3721 | CD | регистрационный номер | copied_from:formalized.awb.flight_number |
| 02 | ModeCode | 40 | CD | код вида транспорта | copied_from:formalized.awb |
| 03 | NationalityCode | 000 | CD | код национальности ТС | |
| 04 | MoverIndicator | true | CD | признак тягача | |

- _item_audit: 4

- doc_status: confirmed

### `document`: Air Waybill
- `uqi_prefix`: formalized.awb
- `xml_target_root`: AltaE3AWB
- `path`: alta\source\Kyland\01\md\АвиаНакладная.md
- `file_name`: АвиаНакладная.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | AgreedValuation | N.V.D | CD | объявленная ценность | |
| 02 | AgreedValuationCurrencyCode | CNY | CD | код валюты ценности | |
| 03 | Registration_AirlineIATACode | 876 | CD | IATA-код авиакомпании | |
| 04 | Registration_DocumentNumber | 41176586 | CD | номер авианакладной | |
| 05 | Registration_DateInf | 2023-01-02 | CD | дата выпуска | |
| 06 | Consignor_NameInf | KYLAND TECHNOLOGY CO., LTD. | CD | грузоотправитель | |
| 07 | Consignor_ShortName | KYLAND TECHNOLOGY CO., LTD. | CD | краткое наименование грузоотправителя | |
| 08 | Consignor_PostalAddress_CountryCode | CN | CD | код страны отправителя | |
| 09 | Consignor_Address_CounryName | КИТАЙ | CD | страна отправителя, текст | |
| 10 | Consignor_Address_City | Beijing | CD | город отправителя | |
| 11 | Consignor_Address_StreetHouse | Building No.2, Shixing Avenue 30#, Shijingshan District | CD | улица, дом отправителя | |
| 12 | Consignee_NameInf | SYMANITRON LTD | CD | грузополучатель | |
| 13 | Consignee_ShortName | SYMANITRON LTD | CD | краткое наименование грузополучателя | |
| 14 | Consignee_OGRNID | 1087746277740 | CD | ОГРН получателя | master_data.md |
| 15 | Consignee_INNID | 7720609470 | CD | ИНН получателя | |
| 16 | Consignee_KPPCode | 772001001 | CD | КПП получателя | master_data.md |
| 17 | Consignee_PostalAddress_PostalCode | 111675 | CD | индекс получателя | |
| 18 | Consignee_PostalAddres_CountryCode | RU | CD | код страны получателя | |
| 19 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя, текст | |
| 20 | Consignee_Address_Region | Moscow | CD | регион получателя | |
| 21 | Consignee_Address_City | Moscow | CD | город получателя | |
| 22 | Consignee_Address_StreetHouse | Ap.103. 17 Rudnevka str. | CD | улица, дом, офис получателя | |
| 23 | GoodsMovement | | | сведения о движении товара | |
| 24 | HandlingInfo | NOTIFY: SAME AS CNEE | CD | информация по обработке груза | |
| 25 | IssueInfo_OrganizationName | CHINA SICHUAN AIRLINES | CD | авиакомпания | |
| 26 | IssueInfo_Address_CountryCode | CN | CD | код страны авиакомпании | |
| 27 | IssueInfo_Address_CounryName | КИТАЙ | CD | страна авиакомпании, текст | |
| 28 | AWBGoodsInfo_TotalPlacesQuantity | 6 | CD | общее количество мест | |
| 29 | AWBGoodsInfo_WeightUnitQualifierCode | K | CD | код единицы измерения веса | |
| 30 | AWBGoodsInfo_GrossWeightQuantity | 90 | CD | общий вес брутто | |
| 31 | flight_number | 3U3721 | CD | номер рейса | |
| 32 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 33 | doc_code | 02017 | CD | код документа | |
| 34 | doc_name | АВИАНАКЛАДНАЯ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | |
| 35 | doc_number | 876-41176586 | CD | номер накладной | |
| 36 | doc_date | 02.01.2023 | CD | дата накладной | |
| 37 | customs_post_code | 10005020 | CO | код таможни на границе | operator:таможня |
| 38 | customs_post_name | Т/П АЭРОПОРТ ШЕРЕМЕТЬЕВО (ГРУЗОВОЙ) | CO | наименование таможни | operator:таможня |
| 39 | warehouse_license_number | 10005/060917/10048/4 | CO | номер лицензии СВХ | operator:СВХ |

- _audit: 39

#### Массив: AWBGoodsInfo_AWBGoods[1]
- _array_audit: 1

#### Элемент массива: AWBGoodsInfo_AWBGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PlacesQuantity | 6 | CD | количество мест | |
| 02 | WeightUnitQualifierCode | K | CD | единица измерения веса | |
| 03 | GrossWeightQuantity | 90 | CD | вес брутто | |
| 04 | CommodityItemNum | 1 | CD | порядковый номер позиции | |
| 05 | GoodsCommodityCode | 8517620003 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.InvoiceGoods[1].GoodsCode |
| 06 | FactPlacesQuantity | 6 | CD | фактическое количество мест | |
| 07 | GoodsDescription | INDUSTRIAL ETHERNET SWITCH | CD | описание груза | |

- _item_audit: 7

- doc_status: confirmed

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\Kyland\01\md\ПЛАТЕЖКА.md
- `file_name`: ПЛАТЕЖКА.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 1 | CD | способ платежа | |
| 03 | PaymentAmount | 25255.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | SALE AND PURCHASE CONTRACT lm191018/Kyl date 19/10/18 Invoice 1000059769 /60285 /60389 /60491 date 12/12/22 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Двадцать пять тысяч двести пятьдесят пять долларов США 00 центов | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 30 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 15.12.2022 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | ООО "СИМАНИТРОН" | CD | плательщик | master_data.md |
| 11 | Payer_INN | 7720609470 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 772001001 | CD | КПП плательщика | master_data.md |
| 13 | Payer_Bank_BankName | АО ЮниКредит Банк, р/с 06037583USDCOCA101 | CD | реквизиты банка | |
| 14 | Payee_OrganizationName | Kyland Technology Co., Ltd., р/с 77012025000038811 | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | BANK OF NINGBO, SWIFT BKNBCN2NBEI | CD | банк получателя | |
| 16 | PersonSurname | ОТСУТСТВУЕТ | CD | фамилия подписанта | |
| 17 | PersonName | ОТСУТСТВУЕТ | CD | имя подписанта | |
| 18 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 19 | doc_code | 04023 | CD | код документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | |
| 21 | doc_number | 30 | CD | номер платежного поручения | |
| 22 | doc_date | 15.12.2022 | CD | дата платежного поручения | |

- _audit: 22

- doc_status: confirmed

### `document`: Service Invoice
- `uqi_prefix`: formalized.service_invoice
- `xml_target_root`: AltaServiceInvoice
- `path`: alta\source\Kyland\01\md\Счет на оплату № VIG2227802 от 28.12.2022.md
- `file_name`: Счет на оплату № VIG2227802 от 28.12.2022.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CD | системный признак документа | |
| 02 | TotalServiceCost | 1615.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого | |
| 04 | ServiceProvider_Name | ООО "ВиАйДжи Кастомс" | CD | перевозчик | |
| 05 | BankName | СМОЛЕНСКОЕ ОТДЕЛЕНИЕ N8609 ПАО СБЕРБАНК г. Смоленск, БИК 046614632, Сч. № 30101810000000000632, Сч. № 40702810959000016812 | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | 279 | CD | № договора на услуги | |
| 07 | ContractDetails_PrDocumentDate | 16.07.2019 | CD | дата договора на услуги | |
| 08 | PrDocumentNumber | BDE000168 | CD | номер связанного документа | |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CD | дата связанного документа | |
| 10 | Registration_PrDocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | VIG2227802 | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 28.12.2022 | CD | дата счета | |
| 13 | Consignor_OrganizationName | ОТСУТСТВУЕТ | CD | грузоотправитель | |
| 14 | PostalCode | | | почтовый индекс грузоотправителя | |
| 15 | CountryCode | | | страна грузоотправителя | |
| 16 | CounryName | | | страна грузоотправителя, текст | |
| 17 | Region | | | регион грузоотправителя | |
| 18 | Town | | | город грузоотправителя | |
| 19 | StreetHouse | | | улица/дом грузоотправителя | |
| 20 | Consignee_OrganizationName | ООО "СИМАНИТРОН" | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1087746277740 | CD | ОГРН грузополучателя | master_data.md |
| 22 | Consignee_RFOrganizationFeatures_INN | 7720609470 | CD | ИНН грузополучателя | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 772001001 | CD | КПП грузополучателя | |
| 24 | PostalCode | 111675 | CD | индекс грузополучателя | |
| 25 | CountryCode | RU | CD | страна грузополучателя | |
| 26 | CounryName | РОССИЯ | CD | страна грузополучателя, текст | |
| 27 | Region | Москва г | CD | регион грузополучателя | |
| 28 | Town | Москва г | CD | город грузополучателя | |
| 29 | StreetHouse | Руднёвка ул | CD | улица грузополучателя | |
| 30 | House | дом 17 | CD | дом грузополучателя | |
| 31 | Room | 103 | CD | офис грузополучателя | |
| 32 | Signature_Choice | 2 | CD | вариант подписи | |
| 33 | IndividualEntrepreneur_PersonSurname | | | фамилия ИП | |
| 34 | IndividualEntrepreneur_PersonName | | | имя ИП | |
| 35 | IndividualEntrepreneur_PersonMiddleName | | | отчество ИП | |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Левчук | CD | фамилия руководителя | |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Е | CD | имя руководителя | |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А | CD | отчество руководителя | |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Левчук | CD | фамилия бухгалтера | |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | Е | CD | имя бухгалтера | |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А | CD | отчество бухгалтера | |
| 42 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 43 | doc_code | 04031 | CD | код документа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | |
| 45 | doc_number | VIG2227802 | CD | номер счета | |
| 46 | doc_date | 28.12.2022 | CD | дата счета | |
| 47 | transport_to_border | 1615.00 | CD | стоимость перевозки до границы | |
| 48 | transport_currency | USD | CD | валюта стоимости перевозки | |

- _audit: 48

#### Массив: ServiceDescription[1]
- _array_audit: 1

#### Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Организация перевозки груза по маршруту: Йичанг, КИТАЙ - Аэропорт Шереметьево - Авианакладная № 876-41176586 | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | Йичанг, КИТАЙ - Аэропорт Шереметьево | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1615.00 | CD | стоимость услуги | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости услуги | |

- _item_audit: 7

- doc_status: confirmed

### `document`: Insurance Invoice
- `uqi_prefix`: formalized.insurance_invoice
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\Счет на оплату № 27611 от 27 декабря 2022 г.md
- `file_name`: Счет на оплату № 27611 от 27 декабря 2022 г.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 27.12.2022 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | VIG2227611 | CD | номер документа | |
| 05 | TextPara | link:alta\source\Kyland\01\md\Счет на оплату № 27611 от 27 декабря 2022 г.md | CD | ссылка на файл-источник | |
| 06 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 07 | doc_code | 04111 | CD | код документа | |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | |
| 09 | doc_number | VIG2227611 | CD | номер документа | |
| 10 | doc_date | 27.12.2022 | CD | дата документа | |
| 11 | insurance_to_border | 5186.02 | CD | стоимость страхования | |
| 12 | insurance_currency | RUB | CD | валюта страхования | |

- _audit: 12

- doc_status: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\Kyland\01\md\TechDescription.md
- `file_name`: TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | Техническое описание груза Kyland | CD | наименование техописания | |
| 03 | DocumentHead_DocumentDate | 12.12.2022 | CD | дата техописания | copied_from:formalized.invoice_1.Registration_PrDocumentDate |
| 04 | DocumentHead_DocumentNumber | БН | CD | номер техописания | |
| 05 | TextPara | link:alta\source\Kyland\01\md\TechDescription.md | CD | ссылка на файл-источник | |
| 06 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 07 | doc_code | 05999 | CD | код документа | |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | |
| 09 | doc_number | БН | CD | номер документа | |
| 10 | doc_date | 12.12.2022 | CD | дата документа | copied_from:formalized.invoice_1.Registration_PrDocumentDate |

- _audit: 10

- doc_status: confirmed

## 3. master_data:

### `document`: Contract
- `uqi_prefix`: master_data.contract
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 02 | doc_code | 03011 | CD | код документа | master_data.md |
| 03 | doc_name | КОНТРАКТ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | master_data.md |
| 04 | doc_number | Im191018/Kyl | CD | номер контракта | master_data.md |
| 05 | doc_date | 19.10.2018 | CD | дата контракта | master_data.md |

- _audit: 5

- doc_status: confirmed

### `document`: Supplementary Contract
- `uqi_prefix`: master_data.supplementary_contract_1
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 02 | doc_code | 03012 | CD | код документа | master_data.md |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | master_data.md |
| 04 | doc_number | 221211 | CD | номер доп. соглашения | master_data.md |
| 05 | doc_date | 11.12.2022 | CD | дата доп. соглашения | master_data.md |

- _audit: 5

- doc_status: confirmed

### `document`: UNK
- `uqi_prefix`: master_data.unk
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 02 | doc_code | 03031 | CD | код документа | master_data.md |
| 03 | doc_name | УНК | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | master_data.md |
| 04 | doc_number | 18100214110000097211 | CD | номер УНК | master_data.md |
| 05 | doc_date | 25.10.2018 | CD | дата УНК | master_data.md |

- _audit: 5

- doc_status: confirmed

### `document`: EGRUL
- `uqi_prefix`: master_data.egrul
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | наименование организации | master_data.md |
| 02 | ShortName | ООО "СИМАНИТРОН" | CD | краткое наименование | master_data.md |
| 03 | OGRN | 1087746277740 | CD | ОГРН | master_data.md |
| 04 | INN | 7720609470 | CD | ИНН | master_data.md |
| 05 | KPP | 772001001 | CD | КПП | master_data.md |
| 06 | Address_PostalCode | 111675 | CD | индекс | master_data.md |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 08 | Address_CounryName | РОССИЯ | CD | страна, текст | master_data.md |
| 09 | Address_Region | ГОРОД МОСКВА | CD | регион | master_data.md |
| 10 | Address_City | МОСКВА | CD | город | master_data.md |
| 11 | Address_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data.md |
| 12 | Phone | +7495 981-62-44 | CD | телефон | master_data.md |
| 13 | Email | info@symanitron.ru | CD | e-mail | master_data.md |
| 14 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 15 | doc_code | 04011 | CD | код документа | master_data.md |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | master_data.md |
| 17 | doc_number | ЮЭ9965-19-16744108 | CD | номер выписки | master_data.md |
| 18 | doc_date | 14.02.2019 | CD | дата выписки | master_data.md |

- _audit: 18

- doc_status: confirmed

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
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | master_data.md |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data.md |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_data.md |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | master_data.md |
| 10 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 11 | doc_code | 11001 | CD | код документа | master_data.md |
| 12 | doc_name | ПАСПОРТ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | master_data.md |
| 13 | doc_number | 63 09 449948 | CD | серия и номер | master_data.md |
| 14 | doc_date | 11.03.2010 | CD | дата выдачи | master_data.md |

- _audit: 14

- doc_status: confirmed

### `document`: Letter of Attorney
- `uqi_prefix`: master_data.letter_of_attorney
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data.md |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_data.md |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | должность | master_data.md |
| 05 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 06 | doc_code | 11004 | CD | код документа | master_data.md |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | master_data.md |
| 08 | doc_number | 1 | CD | номер доверенности | master_data.md |
| 09 | doc_date | 01.02.2026 | CD | дата доверенности | master_data.md |

- _audit: 9

- doc_status: confirmed

### `document`: Transport Contract
- `uqi_prefix`: master_data.transport_contract
- `path`: alta\source\Kyland\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 02 | doc_code | 04033 | CD | код документа | master_data.md |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | НАИМЕНОВАНИЕ ДОКУМЕНТА | master_data.md |
| 04 | doc_number | КООО/26651/М | CD | номер договора | master_data.md |
| 05 | doc_date | 13.05.2025 | CD | дата договора | master_data.md |

- _audit: 5

- doc_status: confirmed

## 4. non_formalized:

### `document`: Goods Description
- `uqi_prefix`: non_formalized.goods_description_1
- `path`: alta\source\Kyland\01\md\TechDescription.md
- `file_name`: TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | признак включения в графу 44 | |

- _audit: 1

#### Массив: goods[7]
- _array_audit: 7

#### Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CO | ТН ВЭД | operator:коды ТН ВЭД |
| 02 | description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-4GX16GE-L2-L2. Предназначен для проводной передачи и коммутации пакетов данных в промышленных сетях. Порты: 16 RJ45 10/100/1000Base-TX, 4 SFP 100Base-X. Металлический корпус IP40, крепление на DIN-рейку. | CD | описание для графы 31 | |

- _item_audit: 2

#### Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CO | ТН ВЭД | operator:коды ТН ВЭД |
| 02 | description | Промышленный коммутатор Ethernet с пониженным энергопотреблением, модель SYM3000A-LITE-2GX8T-L3-L3. Предназначен для проводной передачи пакетов данных. Порты: 8 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. | CD | описание для графы 31 | |

- _item_audit: 2

#### Элемент массива: goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CO | ТН ВЭД | operator:коды ТН ВЭД |
| 02 | description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX16T-L2-L2. Предназначен для передачи и коммутации пакетов данных. Порты: 16 RJ45 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. | CD | описание для графы 31 | |

- _item_audit: 2

#### Элемент массива: goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CO | ТН ВЭД | operator:коды ТН ВЭД |
| 02 | description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-4SFP8T-L2-L2. Предназначен для передачи данных в локальных сетях. Порты: 8 RJ45 10/100Base-TX, 4 SFP 100Base-X. Металлический корпус, крепление на DIN-рейку. | CD | описание для графы 31 | |

- _item_audit: 2

#### Элемент массива: goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CO | ТН ВЭД | operator:коды ТН ВЭД |
| 02 | description | Промышленный коммутатор Ethernet с пониженным энергопотреблением, модель SYM3000A-LITE-8T-L3-L3. Предназначен для проводной передачи данных. Порты: 8 10/100Base-TX. Металлический корпус, крепление на DIN-рейку. | CD | описание для графы 31 | |

- _item_audit: 2

#### Элемент массива: goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CO | ТН ВЭД | operator:коды ТН ВЭД |
| 02 | description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX16GE-L2-L2. Предназначен для коммутации пакетов данных. Порты: 16 RJ45 10/100/1000Base-TX, 2 SFP 100/1000Base-X. Металлический корпус IP40, крепление на DIN-рейку. | CD | описание для графы 31 | |

- _item_audit: 2

#### Элемент массива: goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CO | ТН ВЭД | operator:коды ТН ВЭД |
| 02 | description | Промышленный управляемый коммутатор Ethernet, модель SYM3000A-2GX8T-L2-L2. Предназначен для передачи пакетов данных. Порты: 8 RJ45 10/100Base-TX, 2 SFP 100/1000Base-X. Металлический корпус, крепление на DIN-рейку. | CD | описание для графы 31 | |

- _item_audit: 2

- doc_status: confirmed

### `document`: Regulatory Docs
- `uqi_prefix`: non_formalized.regulatory_docs
- `path`: alta\source\Kyland\01\md\RegulatoryDocs.md
- `file_name`: RegulatoryDocs.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в графу 44 | |

- _audit: 1

#### Массив: invoice_goods[7]
- _array_audit: 7

#### Элемент массива: invoice_goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | model | SYM3000A-4GX16GE-L2-L2 | CD | модель товара | |
| 02 | eac_doc_code | 01402 | CD | код ДС | |
| 03 | eac_doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим. ДС | |
| 04 | eac_doc_number | РА01.В.65848/21 | CD | номер ДС | |
| 05 | eac_doc_date | 14.05.2021 | CD | дата ДС | |
| 06 | eac_date_start | 14.05.2021 | CD | дата начала ДС | |
| 07 | eac_date_end | 12.05.2026 | CD | дата окончания ДС | |
| 08 | ntf_doc_code | 10052 | CD | код нотификации | |
| 09 | ntf_doc_name | НОТИФИКАЦИЯ | CD | наим. нотификации | |
| 10 | ntf_doc_number | RU0000049917 | CD | номер нотификации | |
| 11 | ntf_doc_date | 25.09.2020 | CD | дата нотификации | |
| 12 | ntf_date_start | 25.09.2020 | CD | дата начала нотификации | |
| 13 | ntf_date_end | 31.12.2030 | CD | дата окончания нотификации | |
| 14 | co_doc_code | 06013 | CD | код СО | |
| 15 | co_doc_name | СЕРТИФИКАТ ПРОИСХОЖДЕНИЯ | CD | наим. СО | |
| 16 | co_doc_number | ОТСУТСТВУЕТ | CD | номер СО | |
| 17 | co_doc_date | ОТСУТСТВУЕТ | CD | дата СО | |
| 18 | co_date_start | ОТСУТСТВУЕТ | CD | дата начала СО | |
| 19 | co_date_end | ОТСУТСТВУЕТ | CD | дата окончания СО | |

- _item_audit: 19

#### Элемент массива: invoice_goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | model | SYM3000A-LITE-2GX8T-L3-L3 | CD | модель товара | |
| 02 | eac_doc_code | 01402 | CD | код ДС | |
| 03 | eac_doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим. ДС | |
| 04 | eac_doc_number | РА03.В.59715/21 | CD | номер ДС | |
| 05 | eac_doc_date | 17.12.2021 | CD | дата ДС | |
| 06 | eac_date_start | 17.12.2021 | CD | дата начала ДС | |
| 07 | eac_date_end | 15.12.2026 | CD | дата окончания ДС | |
| 08 | ntf_doc_code | 10052 | CD | код нотификации | |
| 09 | ntf_doc_name | НОТИФИКАЦИЯ | CD | наим. нотификации | |
| 10 | ntf_doc_number | RU0000055090 | CD | номер нотификации | |
| 11 | ntf_doc_date | 13.12.2021 | CD | дата нотификации | |
| 12 | ntf_date_start | 13.12.2021 | CD | дата начала нотификации | |
| 13 | ntf_date_end | 31.12.2031 | CD | дата окончания нотификации | |
| 14 | co_doc_code | 06013 | CD | код СО | |
| 15 | co_doc_name | СЕРТИФИКАТ ПРОИСХОЖДЕНИЯ | CD | наим. СО | |
| 16 | co_doc_number | ОТСУТСТВУЕТ | CD | номер СО | |
| 17 | co_doc_date | ОТСУТСТВУЕТ | CD | дата СО | |
| 18 | co_date_start | ОТСУТСТВУЕТ | CD | дата начала СО | |
| 19 | co_date_end | ОТСУТСТВУЕТ | CD | дата окончания СО | |

- _item_audit: 19

#### Элемент массива: invoice_goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | model | SYM3000A-2GX16T-L2-L2 | CD | модель товара | |
| 02 | eac_doc_code | 01402 | CD | код ДС | |
| 03 | eac_doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим. ДС | |
| 04 | eac_doc_number | РА01.В.65848/21 | CD | номер ДС | |
| 05 | eac_doc_date | 14.05.2021 | CD | дата ДС | |
| 06 | eac_date_start | 14.05.2021 | CD | дата начала ДС | |
| 07 | eac_date_end | 12.05.2026 | CD | дата окончания ДС | |
| 08 | ntf_doc_code | 10052 | CD | код нотификации | |
| 09 | ntf_doc_name | НОТИФИКАЦИЯ | CD | наим. нотификации | |
| 10 | ntf_doc_number | RU0000049917 | CD | номер нотификации | |
| 11 | ntf_doc_date | 25.09.2020 | CD | дата нотификации | |
| 12 | ntf_date_start | 25.09.2020 | CD | дата начала нотификации | |
| 13 | ntf_date_end | 31.12.2030 | CD | дата окончания нотификации | |
| 14 | co_doc_code | 06013 | CD | код СО | |
| 15 | co_doc_name | СЕРТИФИКАТ ПРОИСХОЖДЕНИЯ | CD | наим. СО | |
| 16 | co_doc_number | ОТСУТСТВУЕТ | CD | номер СО | |
| 17 | co_doc_date | ОТСУТСТВУЕТ | CD | дата СО | |
| 18 | co_date_start | ОТСУТСТВУЕТ | CD | дата начала СО | |
| 19 | co_date_end | ОТСУТСТВУЕТ | CD | дата окончания СО | |

- _item_audit: 19

#### Элемент массива: invoice_goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | model | SYM3000A-4SFP8T-L2-L2 | CD | модель товара | |
| 02 | eac_doc_code | 01402 | CD | код ДС | |
| 03 | eac_doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим. ДС | |
| 04 | eac_doc_number | РА01.В.65848/21 | CD | номер ДС | |
| 05 | eac_doc_date | 14.05.2021 | CD | дата ДС | |
| 06 | eac_date_start | 14.05.2021 | CD | дата начала ДС | |
| 07 | eac_date_end | 12.05.2026 | CD | дата окончания ДС | |
| 08 | ntf_doc_code | 10052 | CD | код нотификации | |
| 09 | ntf_doc_name | НОТИФИКАЦИЯ | CD | наим. нотификации | |
| 10 | ntf_doc_number | RU0000049917 | CD | номер нотификации | |
| 11 | ntf_doc_date | 25.09.2020 | CD | дата нотификации | |
| 12 | ntf_date_start | 25.09.2020 | CD | дата начала нотификации | |
| 13 | ntf_date_end | 31.12.2030 | CD | дата окончания нотификации | |
| 14 | co_doc_code | 06013 | CD | код СО | |
| 15 | co_doc_name | СЕРТИФИКАТ ПРОИСХОЖДЕНИЯ | CD | наим. СО | |
| 16 | co_doc_number | ОТСУТСТВУЕТ | CD | номер СО | |
| 17 | co_doc_date | ОТСУТСТВУЕТ | CD | дата СО | |
| 18 | co_date_start | ОТСУТСТВУЕТ | CD | дата начала СО | |
| 19 | co_date_end | ОТСУТСТВУЕТ | CD | дата окончания СО | |

- _item_audit: 19

#### Элемент массива: invoice_goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | model | SYM3000A-LITE-8T-L3-L3 | CD | модель товара | |
| 02 | eac_doc_code | 01402 | CD | код ДС | |
| 03 | eac_doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим. ДС | |
| 04 | eac_doc_number | РА03.В.59715/21 | CD | номер ДС | |
| 05 | eac_doc_date | 17.12.2021 | CD | дата ДС | |
| 06 | eac_date_start | 17.12.2021 | CD | дата начала ДС | |
| 07 | eac_date_end | 15.12.2026 | CD | дата окончания ДС | |
| 08 | ntf_doc_code | 10052 | CD | код нотификации | |
| 09 | ntf_doc_name | НОТИФИКАЦИЯ | CD | наим. нотификации | |
| 10 | ntf_doc_number | RU0000055090 | CD | номер нотификации | |
| 11 | ntf_doc_date | 13.12.2021 | CD | дата нотификации | |
| 12 | ntf_date_start | 13.12.2021 | CD | дата начала нотификации | |
| 13 | ntf_date_end | 31.12.2031 | CD | дата окончания нотификации | |
| 14 | co_doc_code | 06013 | CD | код СО | |
| 15 | co_doc_name | СЕРТИФИКАТ ПРОИСХОЖДЕНИЯ | CD | наим. СО | |
| 16 | co_doc_number | ОТСУТСТВУЕТ | CD | номер СО | |
| 17 | co_doc_date | ОТСУТСТВУЕТ | CD | дата СО | |
| 18 | co_date_start | ОТСУТСТВУЕТ | CD | дата начала СО | |
| 19 | co_date_end | ОТСУТСТВУЕТ | CD | дата окончания СО | |

- _item_audit: 19

#### Элемент массива: invoice_goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | model | SYM3000A-2GX16GE-L2-L2 | CD | модель товара | |
| 02 | eac_doc_code | 01402 | CD | код ДС | |
| 03 | eac_doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим. ДС | |
| 04 | eac_doc_number | РА01.В.65848/21 | CD | номер ДС | |
| 05 | eac_doc_date | 14.05.2021 | CD | дата ДС | |
| 06 | eac_date_start | 14.05.2021 | CD | дата начала ДС | |
| 07 | eac_date_end | 12.05.2026 | CD | дата окончания ДС | |
| 08 | ntf_doc_code | 10052 | CD | код нотификации | |
| 09 | ntf_doc_name | НОТИФИКАЦИЯ | CD | наим. нотификации | |
| 10 | ntf_doc_number | RU0000049917 | CD | номер нотификации | |
| 11 | ntf_doc_date | 25.09.2020 | CD | дата нотификации | |
| 12 | ntf_date_start | 25.09.2020 | CD | дата начала нотификации | |
| 13 | ntf_date_end | 31.12.2030 | CD | дата окончания нотификации | |
| 14 | co_doc_code | 06013 | CD | код СО | |
| 15 | co_doc_name | СЕРТИФИКАТ ПРОИСХОЖДЕНИЯ | CD | наим. СО | |
| 16 | co_doc_number | ОТСУТСТВУЕТ | CD | номер СО | |
| 17 | co_doc_date | ОТСУТСТВУЕТ | CD | дата СО | |
| 18 | co_date_start | ОТСУТСТВУЕТ | CD | дата начала СО | |
| 19 | co_date_end | ОТСУТСТВУЕТ | CD | дата окончания СО | |

- _item_audit: 19

#### Элемент массива: invoice_goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | model | SYM3000A-2GX8T-L2-L2 | CD | модель товара | |
| 02 | eac_doc_code | 01402 | CD | код ДС | |
| 03 | eac_doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | наим. ДС | |
| 04 | eac_doc_number | РА01.В.65848/21 | CD | номер ДС | |
| 05 | eac_doc_date | 14.05.2021 | CD | дата ДС | |
| 06 | eac_date_start | 14.05.2021 | CD | дата начала ДС | |
| 07 | eac_date_end | 12.05.2026 | CD | дата окончания ДС | |
| 08 | ntf_doc_code | 10052 | CD | код нотификации | |
| 09 | ntf_doc_name | НОТИФИКАЦИЯ | CD | наим. нотификации | |
| 10 | ntf_doc_number | RU0000049917 | CD | номер нотификации | |
| 11 | ntf_doc_date | 25.09.2020 | CD | дата нотификации | |
| 12 | ntf_date_start | 25.09.2020 | CD | дата начала нотификации | |
| 13 | ntf_date_end | 31.12.2030 | CD | дата окончания нотификации | |
| 14 | co_doc_code | 06013 | CD | код СО | |
| 15 | co_doc_name | СЕРТИФИКАТ ПРОИСХОЖДЕНИЯ | CD | наим. СО | |
| 16 | co_doc_number | ОТСУТСТВУЕТ | CD | номер СО | |
| 17 | co_doc_date | ОТСУТСТВУЕТ | CD | дата СО | |
| 18 | co_date_start | ОТСУТСТВУЕТ | CD | дата начала СО | |
| 19 | co_date_end | ОТСУТСТВУЕТ | CD | дата окончания СО | |

- _item_audit: 19

- doc_status: confirmed

### Итого, по файлу:
`total_unreliable_fields`: 0
`primary_status`: confirmed

## 5. Нерешенные вопросы (Issues)

**Для полей:**

**Для общих вопросов:**
- `[Общий]`
  - `question`: вопросов нет.

## 6. unreliable_fields:
