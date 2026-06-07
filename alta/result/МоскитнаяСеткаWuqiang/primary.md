# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСеткаWuqiang
- `путь к папке поставки`: alta\source\МоскитнаяСеткаWuqiang
- `direction`: ИМ
- `тип поставки`: 1 ДТ/3 товара
- `источники данных`: md + master_keys.md + master_data.md

## 2. formalized/master_data/non_formalized:

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103.md
- `file_name`: CL 26HL-1103.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | CNY | CD | валюта инвойса | |
| 02 | DocumentCode | 04021 | CD | код вида документа | |
| 03 | PlacesQuantity | 201 | CD | кол-во грузовых мест | |
| 04 | PlacesDescription | рулон | CD | описание мест | |
| 05 | GrossWeightQuantity | 3155 | CD | общий вес брутто | |
| 06 | NetWeightQuantity | 2960 | CD | общий вес нетто | |
| 07 | GCost | 72607.44 | CD | системное поле Альты | |
| 08 | TotalCost | 72607.44 | CD | итого по инвойсу | |
| 09 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий поставки | |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 15 | Registration_PrDocumentName | Commercial invoice / Комерчесский инвойс | CD | наименование документа | |
| 16 | Registration_PrDocumentNumber | 26HL-1103 | CD | номер инвойса | |
| 17 | Registration_PrDocumentDate | 31.03.2026 | CD | дата инвойса | |
| 18 | Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта-ссылки | |
| 19 | Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта-ссылки | |
| 20 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | |
| 21 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | |
| 22 | Buyer_Name | ООО "СКИФ" | CD | наименование покупателя | |
| 23 | Buyer_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | |
| 25 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя, текст | |
| 26 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 27 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 28 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 29 | Seler_Name | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | продавец | |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | |
| 31 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | |
| 32 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | |
| 33 | Seler_PostalAddress_City | Wuqiang, Hengshui | CD | город/район продавца | |
| 34 | Seler_PostalAddress_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом одной строкой | |
| 35 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 37 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | |
| 38 | Consignor_Address_Region | Hebei | CD | регион | |
| 39 | Consignor_Address_City | Wuqiang, Hengshui | CD | город/район | |
| 40 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом одной строкой | |
| 41 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | |
| 42 | Consignee_OGRN | 1201600020390 | CD | ОГРН | |
| 43 | Consignee_INN | 1650389298 | CD | ИНН | |
| 44 | Consignee_KPP | 165001001 | CD | КПП | |
| 45 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | |
| 46 | Consignee_Address_CountryCode | RU | CD | страна | |
| 47 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 48 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 49 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 50 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 51 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 52 | doc_code | 04021 | CD | код документа | |
| 53 | doc_name | ИНВОЙС | CD | наименование документа | |
| 54 | doc_number | 26HL-1103 | CD | номер документа | |
| 55 | doc_date | 31.03.2026 | CD | дата документа | |
- _audit: 55
- `doc_status`: confirmed

#### 3.1.1 Массив: InvoiceGoods[13]
- _array_audit: 13
#### 3.1.1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.4m x 30m Black /Сетчатая ткань из полиэстера с плиссировкой Сетка 16х16, 16 мм. 1.4м х 30м. Черная | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 10 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 420 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 42 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 39 | CD | нетто по строке | |
| 09 | Price | 2.3 | CD | цена за единицу | |
| 10 | TotalCost | 966.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.4m x 30m | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.6m x 30m Black /Сетчатая ткань из полиэстера с плиссировкой Сетка 16х16, 16 мм 1.6м х 30м. Черная | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 10 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 480 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 36.4 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 34 | CD | нетто по строке | |
| 09 | Price | 2.3 | CD | цена за единицу | |
| 10 | TotalCost | 1104.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.6m x 30m | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | European Pleated Mesh 16mm 1.4m x 30m Black /Европейская плиссированная сетка16 мм 1.4м х 30м . Черная | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 10 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 420 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 39 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 36 | CD | нетто по строке | |
| 09 | Price | 2.35 | CD | цена за единицу | |
| 10 | TotalCost | 987.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | European Pleated Mesh 16mm 1.4m x 30m | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | European Pleated Mesh 16mm 1.6m x 30m. Black /Европейская плиссированная сетка16 мм 1.6м х 30м. Черная | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 10 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 480 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 34.4 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 31.5 | CD | нетто по строке | |
| 09 | Price | 2.35 | CD | цена за единицу | |
| 10 | TotalCost | 1128.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | European Pleated Mesh 16mm 1.6m x 30m | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 220g 1.4m x 30m Grey /Сетка от кошек 220 гр "Антикот" 1.4м х 30м. Серая | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 1260 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 303 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 277 | CD | нетто по строке | |
| 09 | Price | 4.7 | CD | цена за единицу | |
| 10 | TotalCost | 5922.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 220g 1.4m x 30m | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Grey /Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Серая | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 50 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 2400 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 800 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 768 | CD | нетто по строке | |
| 09 | Price | 5.8 | CD | цена за единицу | |
| 10 | TotalCost | 13920.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.6m x 30m | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара j | |
- _item_audit: 17
#### 3.1.1 Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.4 m x 30m Grey /Сетка от кошек 320 гр "Антикот" 1.4 м х 30м. Серая | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 50 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 2100 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 710 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 672 | CD | нетто по строке | |
| 09 | Price | 5.8 | CD | цена за единицу | |
| 10 | TotalCost | 12180.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.4 m x 30m | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Black /Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Черная | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 480 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 461 | CD | нетто по строке | |
| 09 | Price | 5.8 | CD | цена за единицу | |
| 10 | TotalCost | 8352.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.6m x 30m | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.4m x 30m Black /Сетка от кошек 320 гр "Антикот" 1.4м х 30м. Черная | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 1260 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 426 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 403 | CD | нетто по строке | |
| 09 | Price | 5.8 | CD | цена за единицу | |
| 10 | TotalCost | 7308.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.4m x 30m | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 5 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*50 M2.Black /Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*50 М2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 10 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 800 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 45.46 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 33 | CD | нетто по строке | |
| 09 | Price | 9.42 | CD | цена за единицу | |
| 10 | TotalCost | 7536.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti Dust Mesh 30g polyester 1,6*50 M2 | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 6 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*30 M2.Black. Roll size: 1.6*100 m2 /Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*100 М2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 800 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 22.74 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 16.5 | CD | нетто по строке | |
| 09 | Price | 9.42 | CD | цена за единицу | |
| 10 | TotalCost | 7536.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti Dust Mesh 30g polyester 1,6*100 M2 | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 7 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7314490000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Mesh 18 Mesh 0.18mm материал SS304 material SS304 Roll size: 1.6*30 m2 Original / сетка 18, размер ячейки 0,18 мм материал SS304. Размер рулона 1,6*30 М2 цвет оригинальный | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 3 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 144 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 51 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 43 | CD | нетто по строке | |
| 09 | Price | 7.46 | CD | цена за единицу | |
| 10 | TotalCost | 1073.86 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Mesh 18 Mesh 0.18mm SS304 1.6*30 m2 | CD | модель/модификация | |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7314490000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Mesh 20 0.17 mm материал SS304 material SS304 Roll size: 1.4*30 m black / сетка17 мм материал SS304. Размер рулона 1,4*30 М2 Черный | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 10 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 420 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 165 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 146 | CD | нетто по строке | |
| 09 | Price | 10.94 | CD | цена за единицу | |
| 10 | TotalCost | 4594.58 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Mesh 20 0.17 mm SS304 1.4*30 m | CD | модель/модификация | |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_2
- `xml_target_root`: AltaE2I
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103-A.md
- `file_name`: CL 26HL-1103-A.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | CNY | CD | валюта инвойса | |
| 02 | DocumentCode | 04021 | CD | код вида документа | |
| 03 | PlacesQuantity | 5 | CD | кол-во грузовых мест | |
| 04 | PlacesDescription | рулон | CD | описание мест | |
| 05 | GrossWeightQuantity | 305 | CD | общий вес брутто | |
| 06 | NetWeightQuantity | 240 | CD | общий вес нетто | |
| 07 | GCost | 15360.00 | CD | системное поле Альты | |
| 08 | TotalCost | 15360.00 | CD | итого по инвойсу | |
| 09 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий поставки | |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 15 | Registration_PrDocumentName | Commercial invoice / Комерчесский инвойс | CD | наименование документа | |
| 16 | Registration_PrDocumentNumber | 26HL-1103-A | CD | номер инвойса | |
| 17 | Registration_PrDocumentDate | 31.03.2026 | CD | дата инвойса | |
| 18 | Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта-ссылки | |
| 19 | Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта-ссылки | |
| 20 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | |
| 21 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | |
| 22 | Buyer_Name | ООО "СКИФ" | CD | наименование покупателя | |
| 23 | Buyer_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | |
| 25 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя, текст | |
| 26 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 27 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 28 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 29 | Seler_Name | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | продавец | |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | |
| 31 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | |
| 32 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | |
| 33 | Seler_PostalAddress_City | Wuqiang, Hengshui | CD | город/район продавца | |
| 34 | Seler_PostalAddress_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом одной строкой | |
| 35 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 37 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | |
| 38 | Consignor_Address_Region | Hebei | CD | регион | |
| 39 | Consignor_Address_City | Wuqiang, Hengshui | CD | город/район | |
| 40 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом одной строкой | |
| 41 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | |
| 42 | Consignee_OGRN | 1201600020390 | CD | ОГРН | |
| 43 | Consignee_INN | 1650389298 | CD | ИНН | |
| 44 | Consignee_KPP | 165001001 | CD | КПП | |
| 45 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | |
| 46 | Consignee_Address_CountryCode | RU | CD | страна | |
| 47 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 48 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 49 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 50 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 51 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 52 | doc_code | 04021 | CD | код документа | |
| 53 | doc_name | ИНВОЙС | CD | наименование документа | |
| 54 | doc_number | 26HL-1103-A | CD | номер документа | |
| 55 | doc_date | 31.03.2026 | CD | дата документа | |
- _audit: 55
- `doc_status`: confirmed

#### 3.1.2 Массив: InvoiceGoods[1]
- _array_audit: 1

#### 3.1.2 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti polent Mesh 100g polyester Black 1,6 м*30 м / Антипыльца 100 г черная 1,6 м*30 м | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 50 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 2400 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м2 | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м2 | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 305 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 240 | CD | нетто по строке | |
| 09 | Price | 6.4 | CD | цена за единицу | |
| 10 | TotalCost | 15360.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti polent Mesh 100g polyester | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара j | |
- _item_audit: 17

### `document`: Packing List
- `uqi_prefix`: formalized.packing_list
- `xml_target_root`: AltaE2PACK
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\PL.md
- `file_name`: PL.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3460 | CD | общий вес брутто | |
| 02 | NetWeightQuantity | 3200 | CD | общий вес нетто | |
| 03 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | |
| 04 | Consignor_ShortName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | краткое наименование | |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | |
| 07 | Consignor_Address_Region | Hebei | CD | регион | |
| 08 | Consignor_Address_City | Wuqiang, Hengshui | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом одной строкой | |
| 10 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | |
| 11 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | |
| 14 | Consignee_KPP | 165001001 | CD | КПП | |
| 15 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | |
| 16 | Consignee_Address_CountryCode | RU | CD | страна | |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 20 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | внутренний числовой код условий | |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наименование контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 26HL-1103 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 31.03.2026 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | 26HL-1103 | CD | № упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 31.03.2026 | CD | дата упаковочного | |
| 33 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 34 | doc_code | 04131 | CD | код документа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | |
| 36 | doc_number | 26HL-1103 | CD | номер документа | |
| 37 | doc_date | 31.03.2026 | CD | дата документа | |
- _audit: 37
- `doc_status`: confirmed

#### 3.2.1 Массив: Goods[14]
- _array_audit: 14

#### 3.2.1 Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.4m x 30m Black / Сетчатая ткань из полиэстера с плиссировкой Сетка 16х16, 16 мм. 1.4м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 42 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 39 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.6m x 30m Black / Сетчатая ткань из полиэстера с плиссировкой Сетка 16х16, 16 мм 1.6м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 36.4 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 34 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | European Pleated Mesh 16mm 1.4m x 30m Black /Европейская плиссированная сетка16 мм 1.4м х 30м . Черная | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 39 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 36 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | European Pleated Mesh 16mm 1.6m x 30m. Black /Европейская плиссированная сетка16 мм 1.6м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 34.4 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 31.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 220g 1.4m x 30m Grey / Сетка от кошек 220 гр "Антикот" 1.4м х 30м. Серая | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 303 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 277 | CD | нетто по строке | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Grey / Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Серая | CD | описание строки | |
| 02 | GoodsQuantity | 50 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 800 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 768 | CD | нетто по строке | |
| 05 | PakingQuantity | 50 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.4 m x 30m Grey / Сетка от кошек 320 гр "Антикот" 1.4 м х 30м. Серая | CD | описание строки | |
| 02 | GoodsQuantity | 50 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 710 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 672 | CD | нетто по строке | |
| 05 | PakingQuantity | 50 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Black / Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 480 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 461 | CD | нетто по строке | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.4m x 30m Black / Сетка от кошек 320 гр "Антикот" 1.4м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 426 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 403 | CD | нетто по строке | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*50 M2.Black /Пылезащитная сетка 30 г "Антипыльца " Черная из полиэстера Размер рулона 1,6*50 М2 | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 45.46 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 33 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*30 M2.Black. Roll size: 1.6*100 m2 /Пылезащитная сетка 30 г "Антипыльца " Черная из полиэстера Размер рулона 1,6*30 М2 | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 22.74 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 16.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 5 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Mesh 18 Mesh 0.18mm материал SS304 material SS304 Roll size: 1.4*30 m2 / сетка 18, размер ячейки 0,18 мм материал SS304. Размер рулона 1,4*30 М2 | CD | описание строки | |
| 02 | GoodsQuantity | 1 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 51 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 43 | CD | нетто по строке | |
| 05 | PakingQuantity | 3 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Mesh 20 0.17 mm материал SS304 material SS304 Roll size: 1.4*30 m / сетка17 мм материал SS304. Размер рулона 1,4*30 М2 | CD | описание строки | |
| 02 | GoodsQuantity | 1 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 165 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 146 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti polent Mesh 100g polyester Black / Антипыльца 100 г черная | CD | описание строки | |
| 02 | GoodsQuantity | 5 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 305 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 240 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок/мест в упаковке | |
- _item_audit: 5

#### 3.2.2 Массив: TransportMeans[2]
- _array_audit: 2

#### 3.2.2 Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | M869OM67 | CD | регистрационный номер | |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 643 | CD | код “национальности” ТС | |
| 04 | MoverIndicator | true | CD | тягач | |
- _item_audit: 4

#### 3.2.2 Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | AM015667 | CD | регистрационный номер | |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 643 | CD | код “национальности” ТС | |
| 04 | MoverIndicator | false | CD | прицеп | |
- _item_audit: 4

### `document`: CMR
- `uqi_prefix`: formalized.cmr
- `xml_target_root`: AltaE3CMR
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\СМР.md
- `file_name`: СМР.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CD | язык документа | |
| 02 | CMR_Choice | 1 | CD | системный выбор/вариант Альты | |
| 03 | RegistrationDocument_RegID | 09225 | CD | номер CMR | |
| 04 | RegistrationDocument_DateInf | 27.05.2026 | CD | дата CMR | |
| 05 | RegistrationDocument_Place | МАНЬЧЖУРИЯ | CD | место составления | |
| 06 | TrakingCargo_TakingCargoDate | 27.05.2026 | CD | дата принятия груза | |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CD | страна принятия груза | |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза, текст | |
| 09 | DeliveryPlace_CountryCode | RU | CD | страна доставки | |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки, текст | |
| 11 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | условия поставки | |
| 13 | GoodsQuantity | 206 | CD | общее число грузовых мест | |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3460 | CD | общий вес брутто | |
| 15 | CMRTransport_PrimeMoverStateSignID | M869OM67 | CD | гос. номер тягача | |
| 16 | CMRTransport_TrailerStateSignID | AM015667 | CD | гос. номер прицепа | |
| 17 | Consignor_NameInf | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD. | CD | наименование отправителя | |
| 18 | Consignor_ShortName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD. | CD | краткое наименование | |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна отправителя | |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна отправителя, текст | |
| 21 | Consignor_Address_Region | HEBEI | CD | регион | |
| 22 | Consignor_Address_City | WUQIANG, HENGSHUI | CD | город/район | |
| 23 | Consignor_Address_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом одной строкой | |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CD | наименование гаранта | |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CD | краткое наименование | |
| 26 | Consignor_Guarantee_Address_CountryCode | ОТСУТСТВУЕТ | CD | страна гаранта | |
| 27 | Consignor_Guarantee_Address_CounryName | ОТСУТСТВУЕТ | CD | страна гаранта, текст | |
| 28 | Consignor_Guarantee_Address_Region | ОТСУТСТВУЕТ | CD | регион | |
| 29 | Consignor_Guarantee_Address_City | ОТСУТСТВУЕТ | CD | город/район | |
| 30 | Consignor_Guarantee_Address_StreetHouse | ОТСУТСТВУЕТ | CD | улица/дом одной строкой | |
| 31 | Consignee_NameInf | ООО "СКИФ" | CD | наименование получателя | |
| 32 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | |
| 33 | Consignee_OGRNID | 1201600020390 | CD | ОГРН | |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна получателя | |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя, текст | |
| 39 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 40 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 41 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 42 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 43 | doc_code | 02015 | CD | код документа | |
| 44 | doc_name | CMR | CD | наименование документа | |
| 45 | doc_number | 09225 | CD | номер документа | |
| 46 | doc_date | 27.05.2026 | CD | дата документа | |
- _audit: 46
- `doc_status`: confirmed

#### 3.3.1 Массив: CMRGoods[1]
- _array_audit: 1

#### 3.3.1 Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | |
| 02 | GoodsDescription | ТОВАР ЗАГРУЖЕН, СОГЛАСНО СПЕЦИФИКАЦИИ К INVOICE № 26HL-1103-A ОТ 31.03.2026; INVOICE № 26HL-1103 ОТ 31.03.2026 | CD | описание груза/товара как в CMR | |
| 03 | PakingQuantity | 206 | CD | кол-во упаковок/мест | |
- _item_audit: 3

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\currency_transfer_5_03.04.2026.md
- `file_name`: currency_transfer_5_03.04.2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 01 | CD | код способа платежа | |
| 03 | PaymentAmount | 72607.44 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции/код | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN MESH. CONTRACT NO.: 26HL-1103, DATE: MARCH 31ST, 2026, INVOICE NO: 26HL-1103, DATE: 31.03.2026 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Семьдесят две тысячи шестьсот семь юаней 44/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 5 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 03.04.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | ООО "СКИФ" | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | |
| 13 | Payer_Bank_BankName | Филиал "Центральный" Банка ВТБ (ПАО), БИК 044525411, Сч. № 40702156216150000051 | CD | реквизиты банка плательщика | |
| 14 | Payee_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SWIFT VTBRCNSHXXX, Адрес: SHANGHAI TOWER, RM. 2503-2505 FLOOR 25, 501 MIDDLE YINCHENG ROAD, PUDONG SHANGHAI, CN | CD | реквизиты банка получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | |
| 17 | PersonName | Дмитрий | CD | имя подписанта | |
| 18 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 19 | doc_code | 04023 | CD | код документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 5 | CD | номер документа | |
| 22 | doc_date | 03.04.2026 | CD | дата документа | |
- _audit: 22
- `doc_status`: confirmed

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\currency_transfer_6_08.04.2026.md
- `file_name`: currency_transfer_6_08.04.2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 01 | CD | код способа платежа | |
| 03 | PaymentAmount | 15360.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции/код | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN MESH. CONTRACT NO.: 26HL-1103, DATE: MARCH 31ST, 2026, INVOICE NO: 26HL-1103-A, DATE: 31.03.2026 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Пятнадцать тысяч триста шестьдесят юаней 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 6 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 08.04.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | ООО "СКИФ" | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | |
| 13 | Payer_Bank_BankName | Филиал "Центральный" Банка ВТБ (ПАО), БИК 044525411, Сч. № 40702156216150000051 | CD | реквизиты банка плательщика | |
| 14 | Payee_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SWIFT VTBRCNSHXXX, Адрес: SHANGHAI TOWER, RM. 2503-2505 FLOOR 25, 501 MIDDLE YINCHENG ROAD, PUDONG SHANGHAI, CN | CD | реквизиты банка получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | |
| 17 | PersonName | Дмитрий | CD | имя подписанта | |
| 18 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 19 | doc_code | 04023 | CD | код документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 6 | CD | номер документа | |
| 22 | doc_date | 08.04.2026 | CD | дата документа | |
- _audit: 22
- `doc_status`: confirmed

### `document`: Service Invoice
- `uqi_prefix`: formalized.service_invoice
- `xml_target_root`: AltaServiceInvoice
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\Счет_№26-09225-tl_от_12-05-2026.md
- `file_name`: Счет_№26-09225-tl_от_12-05-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CD | системный признак документа Альты | |
| 02 | TotalServiceCost | 2610.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого | |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг/перевозчик | |
| 05 | BankName | АО "Альфа-Банк", БИК 044525593, Сч. № 30101810200000000593, Сч. № 40702810001600010931 | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | КООO/26651/М | CD | № договора на услуги/перевозку | |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги/перевозку | |
| 08 | PrDocumentNumber | 26-09225-tl | CD | номер связанного документа | |
| 09 | PrDocumentDate | 07.04.2026 | CD | дата связанного документа | |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | 26-09225-tl | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 12.05.2026 | CD | дата счета | |
| 13 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | |
| 14 | PostalCode | ОТСУТСТВУЕТ | CD | почтовый индекс | |
| 15 | CountryCode | CN | CD | страна | |
| 16 | CounryName | КИТАЙ | CD | страна, текст | |
| 17 | Region | Hebei | CD | регион | |
| 18 | Town | Wuqiang, Hengshui | CD | город/район | |
| 19 | StreetHouse | Haozhuang Industrial Zone | CD | улица/дом одной строкой | |
| 20 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CD | ОГРН | |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | |
| 24 | PostalCode | 423800 | CD | почтовый индекс | |
| 25 | CountryCode | RU | CD | страна | |
| 26 | CounryName | РОССИЯ | CD | страна, текст | |
| 27 | Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 28 | Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 29 | StreetHouse | проезд Хлебный | CD | улица | |
| 30 | House | д. 30 | CD | дом | |
| 31 | Room | офис 211 | CD | офис/кв | |
| 32 | Signature_Choice | 2 | CD | вариант подписи | |
| 33 | IndividualEntrepreneur_PersonSurname | ОТСУТСТВУЕТ | CD | фамилия ИП | |
| 34 | IndividualEntrepreneur_PersonName | ОТСУТСТВУЕТ | CD | имя ИП | |
| 35 | IndividualEntrepreneur_PersonMiddleName | ОТСУТСТВУЕТ | CD | отчество ИП | |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CD | фамилия руководителя | |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л. | CD | имя руководителя | |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А. | CD | отчество руководителя | |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О. | CD | имя бухгалтера | |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А. | CD | отчество бухгалтера | |
| 42 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 43 | doc_code | 04031 | CD | код документа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | |
| 45 | doc_number | 26-09225-tl | CD | номер документа | |
| 46 | doc_date | 12.05.2026 | CD | дата документа | |
| 47 | transport_to_border | 1358.00 | CD | стоимость маршрута до границы | |
| 48 | transport_currency | USD | CD | валюта стоимости | |
- _audit: 48
- `doc_status`: confirmed

#### 3.5.1 Массив: ServiceDescription[2]
- _array_audit: 2

#### 3.5.1 Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООO/26651/М от 13-05-2025 по транспортному заказу №26-09225-tl от 07.04.2026 по маршруту: China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) | CD | наименование/маршрут | |
| 04 | TaxRate | 0% | CD | ставка налога | |
| 05 | TaxSum | 0 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1358.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |
- _item_audit: 7

#### 3.5.1 Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, г. Набережные Челны | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, г. Набережные Челны | CD | наименование/маршрут | |
| 04 | TaxRate | 0% | CD | ставка налога | |
| 05 | TaxSum | 0 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1252.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |
- _item_audit: 7

### `document`: Insurance Services Invoice
- `uqi_prefix`: formalized.insurance_invoice
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\Счет_№26-09225-tl_1_от_11-05-2026.md
- `file_name`: Счет_№26-09225-tl_1_от_11-05-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 11.05.2026 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | 26-09225-tl/1 | CD | номер документа | |
| 05 | TextPara | Возмещение за добровольное страхование груза по договору №КООO/26651/М от 13-05-2025 по заявлению на страхование грузов 26-09225-tl от 11.05.2026 | CD | основной текст/условия | |
| 06 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 07 | doc_code | 04111 | CD | код документа | |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | |
| 09 | doc_number | 26-09225-tl/1 | CD | номер документа | |
| 10 | doc_date | 11.05.2026 | CD | дата документа | |
| 11 | insurance_to_border | 798.04 | CD | стоимость страхования | |
| 12 | insurance_currency | RUB | CD | валюта страхования | |
- _audit: 12
- `doc_status`: confirmed

### `document`: Contract
- `uqi_prefix`: master_data.contract
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 03011 | CD | код документа | |
| 03 | doc_name | КОНТРАКТ | CD | наименование документа | |
| 04 | doc_number | 26HL-1103 | CD | номер документа | |
| 05 | doc_date | 31.03.2026 | CD | дата документа | |
- _audit: 5
- `doc_status`: confirmed

### `document`: EGRUL
- `uqi_prefix`: master_data.egrul
- `path`: alta\master_data.md
- `file_name`: master_data.md

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
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | |
| 14 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 15 | doc_code | 04011 | CD | код документа | |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | |
| 18 | doc_date | 14.07.2025 | CD | дата документа | |
- _audit: 18
- `doc_status`: confirmed

### `document`: Personal Passport
- `uqi_prefix`: master_data.passport
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | |
| 04 | CardSeries | 63 09 | CD | серия | |
| 05 | CardNumber | 449948 | CD | номер | |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | |
| 08 | Phone | +7 927-222-0500 | CD | телефон | |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | |
| 10 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 11 | doc_code | 11001 | CD | код документа | |
| 12 | doc_name | ПАСПОРТ | CD | наименование документа | |
| 13 | doc_number | 63 09 449948 | CD | номер документа | |
| 14 | doc_date | 11.03.2010 | CD | дата документа | |
- _audit: 14
- `doc_status`: confirmed

### `document`: Letter of Attorney
- `uqi_prefix`: master_data.letter_of_attorney
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | |
| 03 | EndDate | 31.12.2026 | CD | действительна до | |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | |
| 05 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 06 | doc_code | 11004 | CD | код документа | |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | |
| 08 | doc_number | 1 | CD | номер документа | |
| 09 | doc_date | 01.02.2026 | CD | дата документа | |
- _audit: 9
- `doc_status`: confirmed

### `document`: Transport Contract
- `uqi_prefix`: master_data.transport_contract
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 04033 | CD | код документа | |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование документа | |
| 04 | doc_number | КООО/26651/М | CD | номер документа | |
| 05 | doc_date | 13.05.2025 | CD | дата документа | |
- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter
- `uqi_prefix`: master_data.exemption_letter
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 09023 | CD | код документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | |
| 05 | doc_date | 20.08.2025 | CD | дата документа | |
- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter (source)
- `uqi_prefix`: master_data.exemption_letter_source
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 09999 | CD | код документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | |
| 05 | doc_date | 20.08.2025 | CD | дата документа | |
- _audit: 5
- `doc_status`: confirmed

### `document`: Goods Description
- `uqi_prefix`: non_formalized.goods_description_1
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103.md
- `file_name`: CL 26HL-1103.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | служебный признак включения в графу 44 | |
- _audit: 1
- `doc_status`: confirmed

#### 5.0.1 Массив: goods[3]
- _array_audit: 3

#### 5.0.1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 6303921000 | CD | ТН ВЭД | |
| 02 | description | Сетка плиссированная из полиэстера (100% полиэстер), нетканая, однотонная, без узора, в рулонах. Предназначена для использования в качестве внутренних штор/занавесей для защиты от насекомых и пыли в жилых и офисных помещениях. | CD | описание товара | |
- _item_audit: 2

#### 5.0.1 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 5804101000 | CD | ТН ВЭД | |
| 02 | description | Сетка защитная (тюль) из полиэстера, однотонная, без узора, нетканая, в рулонах. Предназначена для защиты от насекомых, пыли, пыльцы и домашних животных (сетка 'Антикот', 'Антипыль', 'Антипыльца') в оконных и дверных проемах. | CD | описание товара | |
- _item_audit: 2

#### 5.0.1 Элемент массива: goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 7314490000 | CD | ТН ВЭД | |
| 02 | description | Сетка проволочная тканая из коррозионностойкой (нержавеющей) стали марки SS304, с квадратными ячейками, неоцинкованная, без покрытия пластиком, в рулонах. Предназначена для защиты оконных проемов от насекомых. | CD | описание товара | |
- _item_audit: 2

### `document`: Goods Description
- `uqi_prefix`: non_formalized.goods_description_2
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103-A.md
- `file_name`: CL 26HL-1103-A.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | служебный признак включения в графу 44 | |
- _audit: 1
- `doc_status`: confirmed

#### 5.0.2 Массив: goods[1]
- _array_audit: 1

#### 5.0.2 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 5804101000 | CD | ТН ВЭД | |
| 02 | description | Сетка защитная (тюль) из полиэстера, однотонная, без узора, нетканая, в рулонах. Предназначена для защиты от насекомых, пыли, пыльцы и домашних животных (сетка 'Антикот', 'Антипыль', 'Антипыльца') в оконных и дверных проемах. | CD | описание товара | |
- _item_audit: 2

### `document`: Transit Declaration
- `uqi_prefix`: non_formalized.td
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\Транзитка.md
- `file_name`: Транзитка.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs_post_code | 10719110 | CD | код таможенного органа | |
| 02 | customs_post_name | т/п МАПП Забайкальск | CD | наименование таможенного органа | |
| 03 | transport_reg_number | M869OM67/AM015667 | CD | ТС по ТД | |
| 04 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 05 | doc_code | 09013 | CD | код документа | |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | наименование документа | |
| 07 | doc_number | 10719110/300526/5086483 | CD | номер документа | |
| 08 | doc_date | 30.05.2026 | CD | дата документа | |
- _audit: 8
- `doc_status`: confirmed

### `document`: Storage Report
- `uqi_prefix`: non_formalized.svh
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\ДО_ОТЧЕТ.md
- `file_name`: ДО_ОТЧЕТ.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии/свидетельства СВХ | |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии/свидетельства СВХ | |
| 03 | actual_gross_weight | 3460 | CD | фактический вес по весам | |
| 04 | actual_places | 206 | CD | фактическое количество мест | |
| 05 | transport_reg_number | М 869 ОМ 67 (Прицеп: АМ 0156 67) | CD | номер ТС при въезде | |
| 06 | doc_gr44 | false | CD | служебный признак включения в графу 44 | |
- _audit: 6
- `doc_status`: confirmed

#### 5.2.1 Массив: goods[3]
- _array_audit: 3

#### 5.2.1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара | |
| 02 | places | 197 | CD | кол-во грузовых мест по строке | |
| 03 | gross_weight_kg | 3092.2 | CD | вес брутто по строке | |
| 04 | cost | 78114 | CD | стоимость по строке | |
| 05 | currency_code | CNY | CD | буквенный код валюты | |
- _item_audit: 5

#### 5.2.1 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 6303921000 | CD | код товара | |
| 02 | places | 8 | CD | кол-во грузовых мест по строке | |
| 03 | gross_weight_kg | 151.8 | CD | вес брутто по строке | |
| 04 | cost | 4185 | CD | стоимость по строке | |
| 05 | currency_code | CNY | CD | буквенный код валюты | |
- _item_audit: 5

#### 5.2.1 Элемент массива: goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 7314490000 | CD | код товара | |
| 02 | places | 1 | CD | кол-во грузовых мест по строке | |
| 03 | gross_weight_kg | 216 | CD | вес брутто по строке | |
| 04 | cost | 5668.44 | CD | стоимость по строке | |
| 05 | currency_code | CNY | CD | буквенный код валюты | |
- _item_audit: 5

### `document`: Storage Report Additional Sheet
- `uqi_prefix`: non_formalized.svh_additional_sheet_1
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\ДО_ДОБАВОЧНЫЙ_ЛИСТ.md
- `file_name`: ДО_ДОБАВОЧНЫЙ_ЛИСТ.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 1 | CD | № доп.листа/приложения | |
| 02 | date | 05.06.2026 | CD | дата доп.листа | |
| 03 | actual_gross_weight | 3460 | CD | фактический вес по весам | |
| 04 | actual_places | 206 | CD | фактическое количество мест | |
| 05 | transport_reg_number | М 869 ОМ 67 (Прицеп: АМ 0156 67) | CD | номер ТС при въезде | |
| 06 | svh_address_region | Республика Татарстан | CD | регион СВХ | |
| 07 | svh_address_city | г. Набережные Челны | CD | город/нас.пункт СВХ | |
| 08 | svh_address_street_house | Производственный пр-д, д. 45 | CD | улица/дом СВХ | |
| 09 | svh_customs_code | 10404083 | CD | код таможенного органа в зоне СВХ | |
| 10 | doc_gr44 | false | CD | служебный признак включения в графу 44 | |
- _audit: 10
- `doc_status`: confirmed

### Итого, по файлу:

`total_unreliable_fields`: 0
`primary_status`: confirmed

## 5. Нерешенные вопросы (Issues)

**Для полей:**
- `master_data.passport` / `master_data.letter_of_attorney`
  - `question`: Данные представителя в master_data.md (Арбузова) отличаются от данных в Транзитной декларации (Кузнецов). Использованы данные из master_data.md согласно master_keys.md.
- `master_data.exemption_letter`
  - `question`: Номер отказного письма в master_data.md (24968/МЛ10) отличается от номеров в Транзитной декларации (029407, 790245). Использованы данные из master_data.md.

**Для общих вопросов:**
- `[Общий]`
  - `question`: Нет.

## 6. `unreliable_fields`:
- Нет.
