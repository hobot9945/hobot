# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСеткаWuqiang
- `путь к папке поставки`: alta\source\МоскитнаяСеткаWuqiang\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ/3 товара
- `источники данных`: md + master_data.md

## 2. formalized/master_data/non_formalized:

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: alta\source\МоскитнаяСеткаWuqiang\02\md\25HL-1083_Invoice.md
- `file_name`: 25HL-1083_Invoice.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | CNY | CD | валюта инвойса | |
| 02 | DocumentCode | 04021 | CD | код вида документа | |
| 03 | PlacesQuantity | 83 | CD | кол-во грузовых мест | |
| 04 | PlacesDescription | pcs | CD | описание мест | |
| 05 | GrossWeightQuantity | 3620 | CD | общий вес брутто | |
| 06 | NetWeightQuantity | 3246.6 | CD | общий вес нетто | |
| 07 | GCost | 85214.40 | CD | системное поле стоимости | |
| 08 | TotalCost | 85214.40 | CD | полная стоимость | |
| 09 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | |
| 10 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 11 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 12 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 13 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 14 | Registration_PrDocumentName | Commercial invoice | CD | наименование документа | |
| 15 | Registration_PrDocumentNumber | 25HL-1083 | CD | номер инвойса | |
| 16 | Registration_PrDocumentDate | 14.05.2026 | CD | дата инвойса | |
| 17 | Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта-ссылки | |
| 18 | Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта-ссылки | |
| 19 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | |
| 20 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | |
| 21 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование покупателя | |
| 22 | Buyer_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | |
| 23 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | |
| 24 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя, текст | |
| 25 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 26 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 27 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом | |
| 28 | Seler_Name | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | продавец | |
| 29 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | |
| 30 | Seler_PostalAddress_CounryName | Китай | CD | страна продавца, текст | |
| 31 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | |
| 32 | Seler_PostalAddress_City | Wuqiang, Hengshui | CD | город/район продавца | |
| 33 | Seler_PostalAddress_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом одной строкой | |
| 34 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | |
| 35 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 36 | Consignor_Address_CounryName | Китай | CD | страна грузоотправителя, текст | |
| 37 | Consignor_Address_Region | Hebei | CD | регион | |
| 38 | Consignor_Address_City | Wuqiang, Hengshui | CD | город/район | |
| 39 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом одной строкой | |
| 40 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | |
| 41 | Consignee_OGRN | 1201600020390 | CD | ОГРН | |
| 42 | Consignee_INN | 1650389298 | CD | ИНН | |
| 43 | Consignee_KPP | 165001001 | CD | КПП | |
| 44 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | |
| 45 | Consignee_Address_CountryCode | RU | CD | страна | |
| 46 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 47 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 48 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 49 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом | |
| 50 | doc_gr44 | true | CD | служебный признак | |
| 51 | doc_code | 04021 | CD | код документа | |
| 52 | doc_name | ИНВОЙС | CD | наименование документа | |
| 53 | doc_number | 25HL-1083 | CD | номер инвойса | |
| 54 | doc_date | 14.05.2026 | CD | дата инвойса | |
- _audit: 54

#### formalized.invoice_1 Массив: InvoiceGoods[6]
- _array_audit: 6

#### formalized.invoice_1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Сетка от мошек 22*22 Серая 1,6м*30м | CD | описание товара | |
| 03 | GoodsQuantity | 100 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 4800 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 696 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 624 | CD | нетто по строке | |
| 09 | Price | 163.20 | CD | цена за единицу | |
| 10 | TotalCost | 16320.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | 22*22 Grey 1,6m*30 m | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |
- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Сетка от мошек 22*22 Серая 1,4м*30м | CD | описание товара | |
| 03 | GoodsQuantity | 100 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 4200 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 696 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 624 | CD | нетто по строке | |
| 09 | Price | 142.80 | CD | цена за единицу | |
| 10 | TotalCost | 14280.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | 22*22 Grey 1,4m*30 m | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | |
- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Сетка от мошек 22*22 Черный 1,6м*30м | CD | описание товара | |
| 03 | GoodsQuantity | 100 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 4800 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 608 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 546 | CD | нетто по строке | |
| 09 | Price | 163.20 | CD | цена за единицу | |
| 10 | TotalCost | 16320.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | 22*22 Black 1,6m*30 m | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции внутри товара | |
- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Сетка от мошек 22*22 Черный 1,4м*30м | CD | описание товара | |
| 03 | GoodsQuantity | 100 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 4200 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 608 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 546 | CD | нетто по строке | |
| 09 | Price | 142.80 | CD | цена за единицу | |
| 10 | TotalCost | 14280.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | 22*22 Black 1,4m*30 m | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции внутри товара | |
- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Черная и серебристо-серая | CD | описание товара | |
| 03 | GoodsQuantity | 42 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 2016 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 719 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 645.1 | CD | нетто по строке | |
| 09 | Price | 355.20 | CD | цена за единицу | |
| 10 | TotalCost | 14918.40 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | 320g 1.6m x 30m Black and Silver Grey | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |
- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7314490000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | сетка 17 мм материал SS304. Размер рулона 1,4*30 М2 Черный | CD | описание товара | |
| 03 | GoodsQuantity | 20 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 840 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 293 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 261.5 | CD | нетто по строке | |
| 09 | Price | 454.80 | CD | цена за единицу | |
| 10 | TotalCost | 9096.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | CN | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO.,LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Mesh 20 0.17 mm material SS304 Roll size: 1.4*30 m black | CD | модель/модификация | |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |
- _item_audit: 17
- `doc_status`: confirmed

### `document`: Packing List
- `uqi_prefix`: formalized.packing_list
- `xml_target_root`: AltaE2PACK
- `path`: alta\source\МоскитнаяСеткаWuqiang\02\md\Упаковочный_25HL-1083_1_PackingList.md
- `file_name`: Упаковочный_25HL-1083_1_PackingList.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3620 | CD | общий вес брутто | |
| 02 | NetWeightQuantity | 3246.6 | CD | общий вес нетто | |
| 03 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | |
| 04 | Consignor_ShortName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | краткое наименование | |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 06 | Consignor_Address_CounryName | Китай | CD | страна грузоотправителя, текст | |
| 07 | Consignor_Address_Region | Hebei | CD | регион | |
| 08 | Consignor_Address_City | Wuqiang, Hengshui | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом одной строкой | |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | |
| 11 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | |
| 14 | Consignee_KPP | 165001001 | CD | КПП | |
| 15 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | |
| 16 | Consignee_Address_CountryCode | RU | CD | страна | |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 20 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом одной строкой | |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | | | внутренний числовой код условий | |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наименование контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 25HL-1083 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 14.05.2026 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | 25HL-1083/1 | CD | № упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 14.05.2026 | CD | дата упаковочного | |
| 33 | doc_gr44 | true | CD | служебный признак | |
| 34 | doc_code | 04131 | CD | код документа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | |
| 36 | doc_number | 25HL-1083/1 | CD | номер упаковочного | |
| 37 | doc_date | 14.05.2026 | CD | дата упаковочного | |
- _audit: 37

#### formalized.packing_list Массив: Goods[6]
- _array_audit: 6

#### formalized.packing_list Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-midge mesh 22*22 Grey 1,6m*30 m / Сетка от мошек 22*22 Серая 1,6м*30м | CD | описание строки | |
| 02 | GoodsQuantity | 10 | CD | количество мест в строке | |
| 03 | GrossWeightQuantity | 696 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 624 | CD | нетто по строке | |
| 05 | PakingQuantity | 100 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-midge mesh 22*22 Grey 1,4m*30 m / Сетка от мошек 22*22 Серая 1,4 м*30м | CD | описание строки | |
| 02 | GoodsQuantity | 10 | CD | количество мест в строке | |
| 03 | GrossWeightQuantity | 696 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 624 | CD | нетто по строке | |
| 05 | PakingQuantity | 100 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-midge mesh 22*22 Black 1,6m*30 m / Сетка от мошек 22*22 Черный 1,6м*30м | CD | описание строки | |
| 02 | GoodsQuantity | 10 | CD | количество мест в строке | |
| 03 | GrossWeightQuantity | 608 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 546 | CD | нетто по строке | |
| 05 | PakingQuantity | 100 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-midge mesh 22*22 Black 1,4m*30 m / Сетка от мошек 22*22 Черный 1,4м*30м | CD | описание строки | |
| 02 | GoodsQuantity | 10 | CD | количество мест в строке | |
| 03 | GrossWeightQuantity | 608 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 546 | CD | нетто по строке | |
| 05 | PakingQuantity | 100 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Black and Silver Grey / Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Черная и серебристо-серая | CD | описание строки | |
| 02 | GoodsQuantity | 42 | CD | количество мест в строке | |
| 03 | GrossWeightQuantity | 719 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 645.1 | CD | нетто по строке | |
| 05 | PakingQuantity | 42 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Mesh 20 0.17 mm материал SS304 material SS304 Roll size: 1.4*30 m black / сетка 17 мм материал SS304. Размер рулона 1,4*30 М2 Черный | CD | описание строки | |
| 02 | GoodsQuantity | 1 | CD | количество мест в строке | |
| 03 | GrossWeightQuantity | 293 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 261.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 20 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list Массив: TransportMeans[2]
- _array_audit: 2

#### formalized.packing_list Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | T927MC55 | CD | регистрационный номер | |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 643 | CD | код национальности ТС | |
| 04 | MoverIndicator | true | CD | признак тягача | |
- _item_audit: 4

#### formalized.packing_list Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | AP087055 | CD | регистрационный номер | |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 643 | CD | код национальности ТС | |
| 04 | MoverIndicator | false | CD | признак тягача | |
- _item_audit: 4
- `doc_status`: confirmed

### `document`: CMR
- `uqi_prefix`: formalized.cmr
- `xml_target_root`: AltaE3CMR
- `path`: alta\source\МоскитнаяСеткаWuqiang\02\md\СМР_18614_CMR.md
- `file_name`: СМР_18614_CMR.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CD | язык документа | |
| 02 | CMR_Choice | 1 | CD | системный выбор Альты | |
| 03 | RegistrationDocument_RegID | 18614 | CD | номер CMR | |
| 04 | RegistrationDocument_DateInf | 15.07.2026 | CD | дата CMR | |
| 05 | RegistrationDocument_Place | Маньчжурия | CD | место составления | |
| 06 | TrakingCargo_TakingCargoDate | 15.07.2026 | CD | дата принятия груза | |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CD | страна принятия груза | |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | Китай | CD | страна принятия груза текст | |
| 09 | DeliveryPlace_CountryCode | RU | CD | страна доставки | |
| 10 | DeliveryPlace_CounryName | Россия | CD | страна доставки текст | |
| 11 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 13 | GoodsQuantity | 83 | CD | общее число мест | |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3620 | CD | общий вес брутто | |
| 15 | CMRTransport_PrimeMoverStateSignID | T927MC55 | CD | гос. номер тягача | |
| 16 | CMRTransport_TrailerStateSignID | AP087055 | CD | гос. номер прицепа | |
| 17 | Consignor_NameInf | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | |
| 18 | Consignor_ShortName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | краткое наименование | |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна грузоотправителя | |
| 20 | Consignor_Address_CounryName | Китай | CD | страна грузоотправителя текст | |
| 21 | Consignor_Address_Region | Hebei | CD | регион | |
| 22 | Consignor_Address_City | Wuqiang, Hengshui | CD | город/район | |
| 23 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом | |
| 24 | Consignor_Guarantee_OrganizationName | | | наименование гаранта | |
| 25 | Consignor_Guarantee_ShortName | | | краткое наименование | |
| 26 | Consignor_Guarantee_Address_CountryCode | | | страна | |
| 27 | Consignor_Guarantee_Address_CounryName | | | страна, текст | |
| 28 | Consignor_Guarantee_Address_Region | | | регион | |
| 29 | Consignor_Guarantee_Address_City | | | город/район | |
| 30 | Consignor_Guarantee_Address_StreetHouse | | | улица/дом | |
| 31 | Consignee_NameInf | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | |
| 32 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | |
| 33 | Consignee_OGRNID | 1201600020390 | CD | ОГРН | |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна | |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 39 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 40 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 41 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом | |
| 42 | doc_gr44 | true | CD | служебный признак | |
| 43 | doc_code | 02015 | CD | код документа | |
| 44 | doc_name | CMR | CD | наименование документа | |
| 45 | doc_number | 18614 | CD | номер документа | |
| 46 | doc_date | 15.07.2026 | CD | дата документа | |
- _audit: 46

#### formalized.cmr Массив: CMRGoods[1]
- _array_audit: 1

#### formalized.cmr Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер | |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № 25HL-1083 from 14.05.2026 | CD | описание груза | |
| 03 | PakingQuantity | 83 | CD | кол-во упаковок | |
- _item_audit: 3
- `doc_status`: confirmed

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\МоскитнаяСеткаWuqiang\02\md\currency_transfer_9_14.05.2026 (1)_PaymentOrder.md
- `file_name`: currency_transfer_9_14.05.2026 (1)_PaymentOrder.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 1 | CD | системный код способа платежа | |
| 03 | PaymentAmount | 85214.40 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | системный вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN MESH. CONTRACT NO.: 26HL-1103, DATE: MARCH 31ST, 2026, INVOICE NO: 25HL-1083, DATE: 14.05.2026 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Восемьдесят пять тысяч двести четырнадцать юаней 40 фыней | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 9 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 14.05.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | |
| 13 | Payer_Bank_BankName | Филиал "Центральный" Банка ВТБ (ПАО) р/с 40702156216150000051 | CD | банк плательщика | |
| 14 | Payee_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD, HAOZHUANG INDUSTRIAL ZONE, WUQIANG, HENGSHUI, HEBEI, CHINA, HENGSHUI, CN, a/c 40807156200610025308 | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SHANGHAI TOWER, RM. 2503-2505 FLOOR 25, 501 MIDDLE YINCHENG ROAD, PUDONG SHANGHAI, CN, SWIFT: VTBRCNSHXXX, code: CN767290000018 | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | |
| 17 | PersonName | Дмитрий | CD | имя подписанта | |
| 18 | doc_gr44 | true | CD | служебный признак | |
| 19 | doc_code | 04023 | CD | код документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 9 | CD | номер платежного поручения | |
| 22 | doc_date | 14.05.2026 | CD | дата платежного поручения | |
- _audit: 22
- `doc_status`: confirmed

### `document`: Insurance Bill
- `uqi_prefix`: formalized.insurance_invoice
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\МоскитнаяСеткаWuqiang\02\md\Счет_№26-18614-tl_1_от_30-06-2026_ServiceInvoice.md
- `file_name`: Счет_№26-18614-tl_1_от_30-06-2026_ServiceInvoice.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 30.06.2026 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | 26-18614-tl/1 | CD | номер документа | |
| 05 | TextPara | link:alta\source\МоскитнаяСеткаWuqiang\02\md\Счет_№26-18614-tl_1_от_30-06-2026_ServiceInvoice.md | CD | ссылка на файл-источник | |
| 06 | doc_gr44 | true | CD | служебный признак | |
| 07 | doc_code | 04111 | CD | код документа | |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | |
| 09 | doc_number | 26-18614-tl/1 | CD | номер документа | |
| 10 | doc_date | 30.06.2026 | CD | дата документа | |
| 11 | insurance_to_border | 810.74 | CD | стоимость страхования | |
| 12 | insurance_currency | RUB | CD | валюта страхования | |
- _audit: 12
- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\МоскитнаяСеткаWuqiang\02\md\техничка_АНТИМОШКА_АНТИКОТ_НЕРЖАВЕЙКА_TechDescription.md
- `file_name`: техничка_АНТИМОШКА_АНТИКОТ_НЕРЖАВЕЙКА_TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование техописания | |
| 03 | DocumentHead_DocumentDate | 14.05.2026 | CD | дата техописания | |
| 04 | DocumentHead_DocumentNumber | БН | CD | номер техописания | |
| 05 | TextPara | link:alta\source\МоскитнаяСеткаWuqiang\02\md\техничка_АНТИМОШКА_АНТИКОТ_НЕРЖАВЕЙКА_TechDescription.md | CD | ссылка на файл-источник | |
| 06 | doc_gr44 | true | CD | служебный признак | |
| 07 | doc_code | 05999 | CD | код документа | |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | |
| 09 | doc_number | БН | CD | номер документа | |
| 10 | doc_date | 14.05.2026 | CD | дата документа | |
- _audit: 10
- `doc_status`: confirmed

### `document`: Service Invoice
- `uqi_prefix`: formalized.service_invoice
- `xml_target_root`: AltaServiceInvoice
- `path`: alta\source\МоскитнаяСеткаWuqiang\02\md\Счет_№26-18614-tl_от_15-07-2026_ServiceInvoice.md
- `file_name`: Счет_№26-18614-tl_от_15-07-2026_ServiceInvoice.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CD | системный признак документа | |
| 02 | TotalServiceCost | 2970.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого | |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг/перевозчик | |
| 05 | BankName | АО "Райффайзенбанк", БИК: 044525700, Сч. №: 30101810200000000700, ИНН: 7743772602, КПП: 771401001, Сч. №: 40702810400000233463 | CD | реквизиты банка | |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги | |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги | |
| 08 | PrDocumentNumber | 26-18614-tl | CD | номер связанного заказа | |
| 09 | PrDocumentDate | 29.06.2026 | CD | дата связанного заказа | |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | 26-18614-tl | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 15.07.2026 | CD | дата счета | |
| 13 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | |
| 14 | PostalCode | | | почтовый индекс грузоотправителя | |
| 15 | CountryCode | CN | CD | страна грузоотправителя | |
| 16 | CounryName | Китай | CD | страна грузоотправителя текст | |
| 17 | Region | Hebei | CD | регион грузоотправителя | |
| 18 | Town | Wuqiang | CD | город грузоотправителя | |
| 19 | StreetHouse | Haozhuang Industrial Zone | CD | улица/дом грузоотправителя | |
| 20 | Consignee_OrganizationName | Общество с ограниченной ответственностью "СКиФ" (ООО "СКиФ") | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CD | ОГРН грузополучателя | |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН грузополучателя | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП грузополучателя | |
| 24 | PostalCode | 423800 | CD | почтовый индекс грузополучателя | |
| 25 | CountryCode | RU | CD | страна грузополучателя | |
| 26 | CounryName | РОССИЯ | CD | страна грузополучателя текст | |
| 27 | Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион грузополучателя | |
| 28 | Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город грузополучателя | |
| 29 | StreetHouse | ПРОЕЗД ХЛЕБНЫЙ | CD | улица грузополучателя | |
| 30 | House | Д. 30 | CD | дом грузополучателя | |
| 31 | Room | ОФИС 211 | CD | офис грузополучателя | |
| 32 | Signature_Choice | 2 | CD | вариант подписи | |
| 33 | IndividualEntrepreneur_PersonSurname | | | фамилия ИП | |
| 34 | IndividualEntrepreneur_PersonName | | | имя ИП | |
| 35 | IndividualEntrepreneur_PersonMiddleName | | | отчество ИП | |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CD | фамилия руководителя | |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л. | CD | имя руководителя | |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А. | CD | отчество руководителя | |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О. | CD | имя бухгалтера | |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А. | CD | отчество бухгалтера | |
| 42 | doc_gr44 | true | CD | служебный признак | |
| 43 | doc_code | 04031 | CD | код документа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | |
| 45 | doc_number | 26-18614-tl | CD | номер документа | |
| 46 | doc_date | 15.07.2026 | CD | дата документа | |
| 47 | transport_to_border | 1544.00 | CD | стоимость маршрута до границы | |
| 48 | transport_currency | USD | CD | валюта стоимости | |
- _audit: 48

#### formalized.service_invoice Массив: ServiceDescription[2]
- _array_audit: 2

#### formalized.service_invoice Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-18614-tl от 29.06.2026 по маршруту: China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1544.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |
- _item_audit: 7

#### formalized.service_invoice Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, г. Набережные Челны | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | граница РФ (п/п Маньчжурия/Забайкальск) - Россия, г. Набережные Челны | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1426.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |
- _item_audit: 7
- `doc_status`: confirmed

### `document`: Contract
- `uqi_prefix`: master_data.contract

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 03011 | CD | код документа | |
| 03 | doc_name | КОНТРАКТ | CD | наименование документа | |
| 04 | doc_number | 26HL-1103 | CD | номер документа | |
| 05 | doc_date | 31.03.2026 | CD | дата документа | |
- _audit: 5
- `doc_status`: confirmed

### `document`: UNK
- `uqi_prefix`: master_data.unk

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 03031 | CD | код документа | |
| 03 | doc_name | УНК | CD | наименование документа | |
| 04 | doc_number | ОТСУТСТВУЕТ | CO | номер документа | operator: unk not required |
| 05 | doc_date | ОТСУТСТВУЕТ | CO | дата документа | operator: unk not required |
- _audit: 5
- `doc_status`: confirmed

### `document`: EGRUL
- `uqi_prefix`: master_data.egrul

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование организации | |
| 02 | ShortName | ООО "СКИФ" | CD | краткое наименование | |
| 03 | OGRN | 1201600020390 | CD | ОГРН | |
| 04 | INN | 1650389298 | CD | ИНН | |
| 05 | KPP | 165001001 | CD | КПП | |
| 06 | Address_PostalCode | 423800 | CD | индекс | |
| 07 | Address_CountryCode | RU | CD | страна | |
| 08 | Address_CounryName | РОССИЯ | CD | страна, текст | |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом | |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | |
| 14 | doc_gr44 | true | CD | служебный признак | |
| 15 | doc_code | 04011 | CD | код документа | |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | |
| 18 | doc_date | 14.07.2025 | CD | дата документа | |
- _audit: 18
- `doc_status`: confirmed

### `document`: Personal Passport
- `uqi_prefix`: master_data.passport

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | |
| 04 | CardSeries | 63 09 | CD | серия паспорта | |
| 05 | CardNumber | 449948 | CD | номер паспорта | |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | |
| 08 | Phone | +7 927-222-0500 | CD | телефон | |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | |
| 10 | doc_gr44 | true | CD | служебный признак | |
| 11 | doc_code | 11001 | CD | код документа | |
| 12 | doc_name | ПАСПОРТ | CD | наименование документа | |
| 13 | doc_number | 63 09 449948 | CD | номер документа | |
| 14 | doc_date | 11.03.2010 | CD | дата документа | |
- _audit: 14
- `doc_status`: confirmed

### `document`: Letter of Attorney
- `uqi_prefix`: master_data.letter_of_attorney

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | |
| 03 | EndDate | 31.12.2026 | CD | действительна до | |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | должность | |
| 05 | doc_gr44 | true | CD | служебный признак | |
| 06 | doc_code | 11004 | CD | код документа | |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | |
| 08 | doc_number | 1 | CD | номер документа | |
| 09 | doc_date | 01.02.2026 | CD | дата документа | |
- _audit: 9
- `doc_status`: confirmed

### `document`: Transport Contract
- `uqi_prefix`: master_data.transport_contract

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 04033 | CD | код документа | |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование документа | |
| 04 | doc_number | КООО/26651/М | CD | номер документа | |
| 05 | doc_date | 13.05.2025 | CD | дата документа | |
- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter
- `uqi_prefix`: master_data.exemption_letter

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 09023 | CD | код документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | |
| 05 | doc_date | 20.08.2025 | CD | дата документа | |
- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter (source)
- `uqi_prefix`: master_data.exemption_letter_source

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 09999 | CD | код документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | |
| 05 | doc_date | 20.08.2025 | CD | дата документа | |
- _audit: 5
- `doc_status`: confirmed

### `document`: Goods Descriptions
- `uqi_prefix`: non_formalized.goods_description_1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | служебный признак | |
- _audit: 1

#### non_formalized.goods_description_1 Массив: goods[3]
- _array_audit: 3

#### non_formalized.goods_description_1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 7019900095 | CD | ТН ВЭД | |
| 02 | description | Сетка москитная «Антимошка» из стекловолокна. Изготовлена из прочных стекловолоконных текстильных нитей полотняного переплетения с мелкими ячейками (плотность 22*22 нити на дюйм) в рулонах. Применяется для защиты от проникновения мельчайших насекомых через оконные проемы. | CD | описание для графы 31 | |
- _item_audit: 2

#### non_formalized.goods_description_1 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 5804101000 | CD | ТН ВЭД | |
| 02 | description | Сетка москитная «Антикот» из полиэстера. Представляет собой одноцветное сетчатое полотно без узора в рулонах, изготовленное из усиленных полиэфирных волокон, покрытых ПВХ (плотность основы 14, утка 11 нитей на дюйм), устойчивое к когтям животных. Предназначено для установки в оконные проемы. | CD | описание для графы 31 | |
- _item_audit: 2

#### non_formalized.goods_description_1 Элемент массива: goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 7314490000 | CD | ТН ВЭД | |
| 02 | description | Сетка металлическая из нержавеющей стали. Изготовлена путем переплетения стальной проволоки марки SS304 (толщина 0.17-0.23 мм, плотность 18*18 нитей на дюйм) в рулонах. Сверхпрочное полотно, устойчивое к коррозии и механическим повреждениям. Предназначено для защиты от грызунов и насекомых. | CD | описание для графы 31 | |
- _item_audit: 2
- `doc_status`: confirmed

### `document`: Transit Declaration
- `uqi_prefix`: non_formalized.td
- `path`: alta\source\МоскитнаяСеткаWuqiang\02\md\Транзитка_10719110_180726_5122642_reg_TransitDeclaration.md
- `file_name`: Транзитка_10719110_180726_5122642_reg_TransitDeclaration.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs_post_code | 10719110 | CD | код таможенного органа | |
| 02 | customs_post_name | т/п МАПП Забайкальск | CD | наименование таможенного органа | |
| 03 | transport_reg_number | T927MC55/AP087055 | CD | ТС по ТД | |
| 04 | doc_gr44 | true | CD | служебный признак | |
| 05 | doc_code | 09013 | CD | код документа | |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | наименование документа | |
| 07 | doc_number | 10719110/180726/5122642 | CD | номер документа | |
| 08 | doc_date | 18.07.2026 | CD | дата документа | |
- _audit: 8
- `doc_status`: confirmed

## 4. Итоги по файлу:

`total_unreliable_fields`: 0
`primary_status`: pending

## 5. Нерешенные вопросы (Issues)

**Для полей:**
Нет.

**Для общих вопросов:**
- `[Общий]`
  - `question`: Отчет СВХ (ДО-1 / ДО-2) отсутствует в первичных документах. Данные по весам и местам были сопоставлены кросс-док по Инвойсу, Упаковочному листу и CMR, однако при поступлении ДО-1/ДО-2 потребуется провести повторную сверку.

## 6. `unreliable_fields`:
Нет.
