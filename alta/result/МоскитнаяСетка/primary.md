# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 2 товара
- `источники данных:` md + master_keys.md + master_data.md

## 2. formalized:

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\CL на сетку .md
  - `file_name`: CL на сетку .md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | CurrencyRate | | pending | курс валюты | |
| 02 | CurrencyCode | CNY | CD | валюта инвойса | |
| 03 | DocumentCode | 04021 | CD | код вида документа | |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест | |
| 05 | PlacesDescription | pcs | CD | описание мест | |
| 06 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто | copied_from:formalized.cmr.CMRGoodsWeight_GrossWeightQuantity |
| 07 | NetWeightQuantity | 3302.00 | CD | общий вес нетто | copied_from:formalized.packing_list.NetWeightQuantity |
| 08 | GCost | 97260.00 | CD | системное поле Альты | |
| 09 | TotalCost | 97260.00 | CD | итого по инвойсу | |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 1 | CD | числовой код условий | note: стандартный код для EXW |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 14 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 16 | Registration_PrDocumentName | Commercial invoice | CD | наименование документа | |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | master_data.md |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | master_data.md |
| 23 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование покупателя | master_data.md |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | master_data.md |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | master_data.md |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя текст | master_data.md |
| 27 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data.md |
| 28 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data.md |
| 29 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_data.md |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | продавец | |
| 31 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца alpha-2 | |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца текст | |
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город/район продавца | |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом | |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | note: нормализация: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | note: нормализация: consignor=seller |
| 38 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя текст | note: нормализация: consignor=seller |
| 39 | Consignor_Address_Region | Hebei | CD | регион | note: нормализация: consignor=seller |
| 40 | Consignor_Address_City | Shijiazhuang | CD | город/район | note: нормализация: consignor=seller |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | note: нормализация: consignor=seller |
| 42 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 43 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 44 | Consignee_INN | 1650389298 | CD | ИНН | master_data.md |
| 45 | Consignee_KPP | 165001001 | CD | КПП | master_data.md |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | master_data.md |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 48 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | master_data.md |
| 49 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data.md |
| 50 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data.md |
| 51 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_data.md |
| 52 | doc_gr44 | true | CD | служебный признак | |
| 53 | doc_code | 04021 | CD | код документа | |
| 54 | doc_name | ИНВОЙС | CD | наименование документа | |
| 55 | doc_number | LM-2591 | CD | номер документа | |
| 56 | doc_date | 30.10.2025 | CD | дата документа | |

- _audit: 56

#### 3.1.1 Массив: InvoiceGoods[7]
- _array_audit: 7
#### 3.1.1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара | |
| 03 | GoodsQuantity | 60 | CD | кол-во в основной ед. | |
| 04 | goods_supplementary_quantity | 2520 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | м2 | CD | наим. доп.ед. | |
| 06 | MeasureUnitQualifierName | КВАДРАТНЫЙ МЕТР | CD | ед.изм. доп.количества | |
| 07 | GrossWeightQuantity | 855.00 | CD | брутто по строке | copied_from:formalized.packing_list.Goods[1].GrossWeightQuantity |
| 08 | NetWeightQuantity | 806.60 | CD | нетто по строке | copied_from:formalized.packing_list.Goods[1].NetWeightQuantity |
| 09 | Price | 5.85 | CD | цена за единицу | |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.4*30 | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара | |
| 03 | GoodsQuantity | 30 | CD | кол-во в основной ед. | |
| 04 | goods_supplementary_quantity | 1440 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | м2 | CD | наим. доп.ед. | |
| 06 | MeasureUnitQualifierName | КВАДРАТНЫЙ МЕТР | CD | ед.изм. доп.количества | |
| 07 | GrossWeightQuantity | 490.00 | CD | брутто по строке | copied_from:formalized.packing_list.Goods[2].GrossWeightQuantity |
| 08 | NetWeightQuantity | 460.80 | CD | нетто по строке | copied_from:formalized.packing_list.Goods[2].NetWeightQuantity |
| 09 | Price | 5.85 | CD | цена за единицу | |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.6*30 | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2 | CD | описание товара | |
| 03 | GoodsQuantity | 60 | CD | кол-во в основной ед. | |
| 04 | goods_supplementary_quantity | 2520 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | м2 | CD | наим. доп.ед. | |
| 06 | MeasureUnitQualifierName | КВАДРАТНЫЙ МЕТР | CD | ед.изм. доп.количества | |
| 07 | GrossWeightQuantity | 265.00 | CD | брутто по строке | copied_from:formalized.packing_list.Goods[3].GrossWeightQuantity |
| 08 | NetWeightQuantity | 252.00 | CD | нетто по строке | copied_from:formalized.packing_list.Goods[3].NetWeightQuantity |
| 09 | Price | 6.35 | CD | цена за единицу | |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,4*30 | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | |
| 03 | GoodsQuantity | 30 | CD | кол-во в основной ед. | |
| 04 | goods_supplementary_quantity | 1440 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | м2 | CD | наим. доп.ед. | |
| 06 | MeasureUnitQualifierName | КВАДРАТНЫЙ МЕТР | CD | ед.изм. доп.количества | |
| 07 | GrossWeightQuantity | 155.00 | CD | брутто по строке | copied_from:formalized.packing_list.Goods[4].GrossWeightQuantity |
| 08 | NetWeightQuantity | 144.00 | CD | нетто по строке | copied_from:formalized.packing_list.Goods[4].NetWeightQuantity |
| 09 | Price | 6.35 | CD | цена за единицу | |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,6*30 | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара | |
| 03 | GoodsQuantity | 90 | CD | кол-во в основной ед. | |
| 04 | goods_supplementary_quantity | 3780 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | м2 | CD | наим. доп.ед. | |
| 06 | MeasureUnitQualifierName | КВАДРАТНЫЙ МЕТР | CD | ед.изм. доп.количества | |
| 07 | GrossWeightQuantity | 520.00 | CD | брутто по строке | copied_from:formalized.packing_list.Goods[5].GrossWeightQuantity |
| 08 | NetWeightQuantity | 491.40 | CD | нетто по строке | copied_from:formalized.packing_list.Goods[5].NetWeightQuantity |
| 09 | Price | 3.4 | CD | цена за единицу | |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,4*30 | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара | |
| 03 | GoodsQuantity | 180 | CD | кол-во в основной ед. | |
| 04 | goods_supplementary_quantity | 8640 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | м2 | CD | наим. доп.ед. | |
| 06 | MeasureUnitQualifierName | КВАДРАТНЫЙ МЕТР | CD | ед.изм. доп.количества | |
| 07 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | copied_from:formalized.packing_list.Goods[6].GrossWeightQuantity |
| 08 | NetWeightQuantity | 1123.20 | CD | нетто по строке | copied_from:formalized.packing_list.Goods[6].NetWeightQuantity |
| 09 | Price | 3.4 | CD | цена за единицу | |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,6*30 | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### 3.1.1 Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | |
| 03 | GoodsQuantity | 5 | CD | кол-во в основной ед. | |
| 04 | goods_supplementary_quantity | 240 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | м2 | CD | наим. доп.ед. | |
| 06 | MeasureUnitQualifierName | КВАДРАТНЫЙ МЕТР | CD | ед.изм. доп.количества | |
| 07 | GrossWeightQuantity | 25.00 | CD | брутто по строке | copied_from:formalized.packing_list.Goods[7].GrossWeightQuantity |
| 08 | NetWeightQuantity | 24.00 | CD | нетто по строке | copied_from:formalized.packing_list.Goods[7].NetWeightQuantity |
| 09 | Price | 28 | CD | цена за единицу | |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1,6*30 | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 5 | CD | индекс позиции внутри товара | |

- _item_audit: 17

- `doc_status`: confirmed
### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list
  - `xml_target_root`: AltaE2PACK
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\PL на сетку .md
  - `file_name`: PL на сетку .md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто | |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто | |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | |
| 04 | Consignor_ShortName | | pending | краткое наименование | |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя текст | |
| 07 | Consignor_Address_Region | Hebei | CD | регион | |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 11 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | master_data.md |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | master_data.md |
| 14 | Consignee_KPP | 165001001 | CD | КПП | master_data.md |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | master_data.md |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | master_data.md |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data.md |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data.md |
| 20 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_data.md |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 1 | CD | числовой код условий | note: стандартный код для EXW |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наим. контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наим. инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наим. упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CD | № упаковочного | note: используется номер инвойса |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CD | дата упаковочного | note: используется дата инвойса |
| 33 | doc_gr44 | true | CD | служебный признак | |
| 34 | doc_code | 04131 | CD | код документа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | |
| 36 | doc_number | LM-2591 | CD | номер документа | note: используется номер инвойса |
| 37 | doc_date | 30.10.2025 | CD | дата документа | note: используется дата инвойса |

- _audit: 37

#### 3.2.1 Массив: Goods[7]
- _array_audit: 7

#### 3.2.1 Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки | |
| 02 | GoodsQuantity | 60 | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | |
| 05 | PakingQuantity | 60 | CD | кол-во упаковок | |

- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 460.80 | CD | нетто по строке | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | |

- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера . Размер рулона 1,42*0,64*0,22 | CD | описание строки | |
| 02 | GoodsQuantity | 60 | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 252.00 | CD | нетто по строке | |
| 05 | PakingQuantity | 6 | CD | кол-во упаковок | |

- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 144.00 | CD | нетто по строке | |
| 05 | PakingQuantity | 3 | CD | кол-во упаковок | |

- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА " Антимошка " из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание строки | |
| 02 | GoodsQuantity | 90 | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 491.40 | CD | нетто по строке | |
| 05 | PakingQuantity | 9 | CD | кол-во упаковок | |

- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание строки | |
| 02 | GoodsQuantity | 180 | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто по строке | |
| 05 | PakingQuantity | 18 | CD | кол-во упаковок | |

- _item_audit: 5

#### 3.2.1 Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки "Антипыльца " из полиэстера Размер рулона 1,72*0,35*0,31*1 | CD | описание строки | |
| 02 | GoodsQuantity | 5 | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 24.00 | CD | нетто по строке | |
| 05 | PakingQuantity | 1 | CD | кол-во упаковок | |

- _item_audit: 5

#### 3.2.2 Массив: TransportMeans[0]
- _array_audit: 0

- `doc_status`: confirmed

### `document`: CMR
  - `uqi_prefix`: formalized.cmr
  - `xml_target_root`: AltaE3CMR
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\СМР от СВХ.md
  - `file_name`: СМР от СВХ.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | LanguageCode | RU | CD | язык документа | |
| 02 | CMR_Choice | 1 | CD | системный выбор | |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | |
| 05 | RegistrationDocument_Place | Манчжурия | CD | место составления | |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата принятия груза | |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CD | страна принятия груза | |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза текст | |
| 09 | DeliveryPlace_CountryCode | RU | CD | страна доставки | |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки текст | |
| 11 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryPlace |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | условия поставки | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode |
| 13 | GoodsQuantity | 127 | CD | общее число мест | |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто | |
| 15 | CMRTransport_PrimeMoverStateSignID | O157AO774 | CD | гос. номер тягача | |
| 16 | CMRTransport_TrailerStateSignID | BT374974 | CD | гос. номер прицепа | |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование | |
| 18 | Consignor_ShortName | | pending | краткое наименование | |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна alpha-2 | |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна текст | |
| 21 | Consignor_Address_Region | Hebei | CD | регион | |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город/район | |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | |
| 24 | Consignor_Guarantee_OrganizationName | | pending | наименование гаранта | |
| 25 | Consignor_Guarantee_ShortName | | pending | краткое наименование | |
| 26 | Consignor_Guarantee_Address_CountryCode | | pending | страна alpha-2 | |
| 27 | Consignor_Guarantee_Address_CounryName | | pending | страна текст | |
| 28 | Consignor_Guarantee_Address_Region | | pending | регион | |
| 29 | Consignor_Guarantee_Address_City | | pending | город/район | |
| 30 | Consignor_Guarantee_Address_StreetHouse | | pending | улица/дом | |
| 31 | Consignee_NameInf | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование получателя | master_data.md |
| 32 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | master_data.md |
| 33 | Consignee_OGRNID | 1201600020390 | CD | ОГРН | master_data.md |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | master_data.md |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | master_data.md |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | master_data.md |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | master_data.md |
| 39 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data.md |
| 40 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data.md |
| 41 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_data.md |
| 42 | doc_gr44 | true | CD | служебный признак | |
| 43 | doc_code | 02015 | CD | код документа | |
| 44 | doc_name | CMR | CD | наименование документа | |
| 45 | doc_number | 00378 | CD | номер документа | |
| 46 | doc_date | 20.01.2026 | CD | дата документа | |

- _audit: 46

#### 3.3.1 Массив: CMRGoods[1]
- _array_audit: 1

#### 3.3.1 Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsNumeric | 1 | CD | порядковый номер | note: авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 | CD | описание груза | note: исключение CMRGoodsDescription — источник non_formalized.svh_1 |
| 03 | PakingQuantity | 127 | CD | кол-во упаковок | |

- _item_audit: 3

- `doc_status`: confirmed
### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\платежки\currency_transfer_1_13.01.2026.md
  - `file_name`: currency_transfer_1_13.01.2026.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | | pending | системный код способа платежа | |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | master_data.md |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО)) | CD | реквизиты банка | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | реквизиты банка получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия | |
| 17 | PersonName | Дмитрий | CD | имя | |
| 18 | doc_gr44 | true | CD | служебный признак | |
| 19 | doc_code | 04023 | CD | код документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 1 | CD | номер документа | |
| 22 | doc_date | 13.01.2026 | CD | дата документа | |

- _audit: 22

- `doc_status`: confirmed

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\платежки\currency_transfer_7_28.11.2025.md
  - `file_name`: currency_transfer_7_28.11.2025.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | | pending | системный код способа платежа | |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | master_data.md |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО)) | CD | реквизиты банка | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | реквизиты банка получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия | |
| 17 | PersonName | Дмитрий | CD | имя | |
| 18 | doc_gr44 | true | CD | служебный признак | |
| 19 | doc_code | 04023 | CD | код документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 7 | CD | номер документа | |
| 22 | doc_date | 28.11.2025 | CD | дата документа | |

- _audit: 22

- `doc_status`: confirmed

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice
  - `xml_target_root`: AltaServiceInvoice
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_от_27-01-2026.md
  - `file_name`: Счет_№26-00378-tl_от_27-01-2026.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentSign | 1 | CD | системный признак | |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого | |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг | |
| 05 | BankName | АО "Райффайзенбанк"; БИК 044525700; Сч. № 30101810200000000700; Сч. № 40702810400000233463 | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | №КООО/26651/М | CD | № договора на услуги | |
| 07 | ContractDetails_PrDocumentDate | 13-05-2025 | CD | дата договора на услуги | |
| 08 | PrDocumentNumber | 26-00378-tl | CD | номер заказа | |
| 09 | PrDocumentDate | 12.01.2026 | CD | дата заказа | |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | copied_from:formalized.invoice_1.Seler_Name |
| 14 | PostalCode | | pending | индекс | |
| 15 | CountryCode | CN | CD | страна alpha-2 | copied_from:formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 16 | CounryName | КИТАЙ | CD | страна текст | copied_from:formalized.invoice_1.Seler_PostalAddress_CounryName |
| 17 | Region | Hebei | CD | регион | copied_from:formalized.invoice_1.Seler_PostalAddress_Region |
| 18 | Town | Shijiazhuang | CD | город/район | copied_from:formalized.invoice_1.Seler_PostalAddress_City |
| 19 | StreetHouse | No. 5 Gaodong street | CD | улица/дом | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 20 | Consignee_OrganizationName | ООО "Скиф" (ООО "СКИФ") | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | |
| 24 | PostalCode | 423800 | CD | индекс | |
| 25 | CountryCode | RU | CD | страна alpha-2 | |
| 26 | CounryName | РОССИЯ | CD | страна текст | |
| 27 | Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 28 | Town | Набережные Челны | CD | город | |
| 29 | StreetHouse | проезд Хлебный | CD | улица | |
| 30 | House | 30 | CD | дом | |
| 31 | Room | 211 | CD | офис/кв | |
| 32 | Signature_Choice | 2 | CD | вариант подписи | |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | |
| 34 | IndividualEntrepreneur_PersonName | | CD | имя ИП | |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климовин | CD | фамилия руководителя | |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CD | имя руководителя | |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | | pending | отчество руководителя | |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CD | имя бухгалтера | |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | | pending | отчество бухгалтера | |
| 42 | doc_gr44 | true | CD | служебный признак | |
| 43 | doc_code | 04031 | CD | код документа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | |
| 45 | doc_number | 26-00378-tl | CD | номер документа | |
| 46 | doc_date | 27.01.2026 | CD | дата документа | |
| 47 | transport_to_border | 1404.00 | CD | до границы | |
| 48 | transport_currency | USD | CD | валюта страхования | |

- _audit: 48

#### 3.5.1 Массив: ServiceDescription[2]
- _array_audit: 2

#### 3.5.1 Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) - перевозка автотранспортом | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) | CD | наименование/маршрут | |
| 04 | TaxRate | 0% | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | |

- _item_audit: 7

#### 3.5.1 Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | наименование/маршрут | |
| 04 | TaxRate | 0% | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | |

- _item_audit: 7

- `doc_status`: confirmed
### `document`: Insurance Services Invoice
  - `uqi_prefix`: formalized.insurance_invoice
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md
  - `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04111 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | Счет на оплату №26-00378-tl/1 от 14.01.2026 г. | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md | CD | основной текст | |
| 06 | doc_gr44 | true | CD | служебный признак | |
| 07 | doc_code | 04111 | CD | код документа | |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | |
| 09 | doc_number | 26-00378-tl/1 | CD | номер документа | |
| 10 | doc_date | 14.01.2026 | CD | дата документа | |
| 11 | insurance_to_border | 910.34 | CD | стоимость страхования | |
| 12 | insurance_currency | RUB | CD | валюта страхования | |

- _audit: 12

- `doc_status`: confirmed

### `document`: Tech Description
  - `uqi_prefix`: formalized.tech_description
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md
  - `file_name`: техничка Антикот, антипыльца антимошка .md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 05999 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | Технические характеристики — Сетки из полиэстера 5804101000 / Сетки из стекловолокна 7019900095 | CD | наименование техописания | |
| 03 | DocumentHead_DocumentDate | | pending | дата техописания | |
| 04 | DocumentHead_DocumentNumber | | pending | номер техописания | |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md | CD | технический текст | |
| 06 | doc_gr44 | true | CD | служебный признак | |
| 07 | doc_code | 05999 | CD | код документа | |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | |
| 09 | doc_number | | pending | номер документа | |
| 10 | doc_date | | pending | дата документа | |

- _audit: 10

- `doc_status`: confirmed

## 3. master_data:

### `document`: Contract
  - `uqi_prefix`: master_data.contract

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 03011 | CD | код документа | |
| 03 | doc_name | КОНТРАКТ | CD | наименование документа | |
| 04 | doc_number | LM-2553 | CD | номер документа | master_data.md |
| 05 | doc_date | 02.07.2025 | CD | дата документа | master_data.md |

- _audit: 5

- `doc_status`: confirmed

### `document`: Supplementary Contract
  - `uqi_prefix`: master_data.supplementary_contract_1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 03012 | CD | код документа | |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наименование документа | |
| 04 | doc_number | 1 | CD | номер документа | master_data.md |
| 05 | doc_date | 25.11.2025 | CD | дата документа | master_data.md |

- _audit: 5

- `doc_status`: confirmed

### `document`: EGRUL
  - `uqi_prefix`: master_data.egrul

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
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
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_data.md |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | master_data.md |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | master_data.md |
| 14 | doc_gr44 | true | CD | служебный признак | |
| 15 | doc_code | 04011 | CD | код документа | |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | master_data.md |
| 18 | doc_date | 14.07.2025 | CD | дата документа | master_data.md |

- _audit: 18

- `doc_status`: confirmed

### `document`: Personal Passport
  - `uqi_prefix`: master_data.passport

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | master_data.md |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | master_data.md |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | master_data.md |
| 04 | CardSeries | 63 09 | CD | серия | master_data.md |
| 05 | CardNumber | 449948 | CD | номер | master_data.md |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | master_data.md |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data.md |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_data.md |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | master_data.md |
| 10 | doc_gr44 | true | CD | служебный признак | |
| 11 | doc_code | 11001 | CD | код документа | |
| 12 | doc_name | ПАСПОРТ | CD | наименование документа | |
| 13 | doc_number | 63 09 449948 | CD | номер документа | master_data.md |
| 14 | doc_date | 11.03.2010 | CD | дата документа | master_data.md |

- _audit: 14

- `doc_status`: confirmed

### `document`: Letter of Attorney
  - `uqi_prefix`: master_data.letter_of_attorney

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data.md |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_data.md |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_data.md |
| 05 | doc_gr44 | true | CD | служебный признак | |
| 06 | doc_code | 11004 | CD | код документа | |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | |
| 08 | doc_number | 1 | CD | номер документа | master_data.md |
| 09 | doc_date | 01.02.2026 | CD | дата документа | master_data.md |

- _audit: 9

- `doc_status`: confirmed

### `document`: Transport Contract
  - `uqi_prefix`: master_data.transport_contract

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 04033 | CD | код документа | |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование документа | |
| 04 | doc_number | КООО/26651/М | CD | номер документа | master_data.md |
| 05 | doc_date | 13.05.2025 | CD | дата документа | master_data.md |

- _audit: 5

- `doc_status`: confirmed

### `document`: Exemption Letter
  - `uqi_prefix`: master_data.exemption_letter

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 09023 | CD | код документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | master_data.md |
| 05 | doc_date | 20.08.2025 | CD | дата документа | master_data.md |

- _audit: 5

- `doc_status`: confirmed

### `document`: Exemption Letter (source)
  - `uqi_prefix`: master_data.exemption_letter_source

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак | |
| 02 | doc_code | 09999 | CD | код документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | master_data.md |
| 05 | doc_date | 20.08.2025 | CD | дата документа | master_data.md |

- _audit: 5

- `doc_status`: confirmed
## 4. non_formalized:

### `document`: Goods Description
  - `uqi_prefix`: non_formalized.goods_description_1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | false | CD | служебный признак | |

- _audit: 1

#### 5.0.1 Массив: goods[2]
- _array_audit: 2

#### 5.0.1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tn_ved | 5804101000 | CD | ТН ВЭД | |
| 02 | description | Тюль и прочие сетчатые полотна, одноцветные, без узора, из полиэстера. Предназначены для защиты от насекомых и пыли (москитные сетки «Антикот», «Антипыльца»). Изготовлены методом плетения. | CD | описание | note: расшифровка ТН ВЭД + тех. описание |

- _item_audit: 2

#### 5.0.1 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tn_ved | 7019900095 | CD | ТН ВЭД | |
| 02 | description | Изделия из стекловолокна (текстильных волокон), прочие. Сетка «Антимошка» из стекловолокна, изготовлена методом плетения. Предназначена для защиты от мелких насекомых. | CD | описание | note: расшифровка ТН ВЭД + тех. описание |

- _item_audit: 2

- `doc_status`: confirmed

### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.td
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ТД 10719110_240126_5011363_reg00378тд.md
  - `file_name`: ТД 10719110_240126_5011363_reg00378тд.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | customs_post_code | 10719110 | CD | код таможенного органа | |
| 02 | customs_post_name | Таможенный орган отправления | CD | наим. таможенного органа | |
| 03 | transport_reg_number | O157AO774/BT374974 | CD | ТС по ТД | |
| 04 | doc_gr44 | true | CD | служебный признак | |
| 05 | doc_code | 09013 | CD | код документа | |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | наименование документа | |
| 07 | doc_number | 10719110/240126/5011363 | CD | номер документа | |
| 08 | doc_date | 24.01.2026 | CD | дата документа | |

- _audit: 8

- `doc_status`: confirmed

### `document`: Storage Report
  - `uqi_prefix`: non_formalized.svh
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО 14431420260204161621.md
  - `file_name`: ДО 14431420260204161621.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии СВХ | |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии СВХ | |
| 03 | actual_gross_weight | 3500 | CD | фактический вес брутто | copied_from:non_formalized.svh_additional_sheet_1.actual_gross_weight |
| 04 | actual_places | 127 | CD | фактическое кол-во мест | copied_from:non_formalized.svh_additional_sheet_1.actual_places |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС | |
| 06 | doc_gr44 | false | CD | служебный признак | |

- _audit: 6

#### 5.2.1 Массив: goods[2]
- _array_audit: 2

#### 5.2.1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tnved | 7019900095 | CD | код товара | |
| 02 | places | 27 | CD | кол-во мест | |
| 03 | gross_weight_kg | 1710 | CD | вес брутто | |
| 04 | cost | 42228 | CD | стоимость | |
| 05 | currency_code | CNY | CD | код валюты | |

- _item_audit: 5

#### 5.2.1 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tnved | 5804101000 | CD | код товара | |
| 02 | places | 100 | CD | кол-во мест | |
| 03 | gross_weight_kg | 1790 | CD | вес брутто | |
| 04 | cost | 55032 | CD | стоимость | |
| 05 | currency_code | CNY | CD | код валюты | |

- _item_audit: 5

- `doc_status`: confirmed

### `document`: Storage Report Additional Sheet
  - `uqi_prefix`: non_formalized.svh_additional_sheet_1
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО доп 14431520260204161645.md
  - `file_name`: ДО доп 14431520260204161645.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | number | 1 | CD | № доп.листа | |
| 02 | date | 03.02.2026 | CD | дата доп.листа | copied_from:non_formalized.svh.date |
| 03 | actual_gross_weight | 3500 | CD | фактический вес брутто | |
| 04 | actual_places | 127 | CD | фактическое кол-во мест | |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС | copied_from:non_formalized.svh.transport_reg_number |
| 06 | svh_address_region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион СВХ | master_data.md |
| 07 | svh_address_city | Набережные Челны | CD | город СВХ | master_data.md |
| 08 | svh_address_street_house | Производственный пр-д, д. 45 | CD | улица/дом СВХ | copied_from:formalized.cmr.SenderInstructions |
| 09 | svh_customs_code | 10404083 | CD | код таможни СВХ | copied_from:formalized.cmr.SenderInstructions |
| 10 | doc_gr44 | false | CD | служебный признак | |

- _audit: 10

- `doc_status`: confirmed

### Итогo, по файлу:

`total_unreliable_fields`: 0
`primary_status`: confirmed

## 5. Нерешенные вопросы (Issues)

**Для полей:**
- `formalized.invoice_1.CurrencyRate` 
  - `question`: Курс валюты (RMB) не указан в первичке. Будет определен на дату подачи ДТ.
- `formalized.packing_list.Consignor_ShortName` / `formalized.cmr.Consignor_ShortName` 
  - `question`: Краткое наименование отправителя отсутствует.
- `formalized.cmr.Consignor_Guarantee_*` 
  - `question`: Данные о гаранте в CMR отсутствуют (не критично для данного типа перевозки).
- `formalized.payment_order_*.PaymentModeCode` 
  - `question`: Системный код способа платежа не определен.
- `formalized.service_invoice.PostalCode` (Consignor)
  - `question`: Индекс отправителя в счете за перевозку не указан.
- `formalized.service_invoice.*MiddleName` 
  - `question`: Отчества подписантов в счете за перевозку отсутствуют.
- `formalized.tech_description.*` 
  - `question`: Техническое описание не имеет внутреннего номера и даты.
## 6. `unreliable_fields`:
- Нет.
