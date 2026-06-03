# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 7 товаров
- `источники данных`: md + operator_provided_data.md + master_keys.md + master_data.md

## 2. formalized/master_data/non_formalized:

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\CL на сетку .pdf
- `file_name`: CL на сетку .pdf
- `note`: Основной коммерческий инвойс поставки

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты | operator:formalized.invoice_1.exchange_rate |
| 02 | CurrencyCode | CNY | CO | валюта инвойса | operator:formalized.invoice_1.currency_code |
| 03 | DocumentCode | 04021 | CD | код вида документа | constant |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест | copied_from:formalized.invoice_1.PlacesQuantity (CL на сетку .pdf) |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator:formalized.invoice_1.places_description |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто | operator:formalized.invoice_1.total_gross_weight |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто | operator:formalized.invoice_1.total_net_weight |
| 08 | GCost | 97260.00 | CO | системное поле Альты | operator:formalized.invoice_1.gcost |
| 09 | TotalCost | 97260.00 | CO | итого по инвойсу | operator:formalized.invoice_1.total_cost |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryPlace (CL на сетку .pdf) |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator:formalized.invoice_1.delivery_terms_numeric |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode (CL на сетку .pdf) |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления | operator:formalized.invoice_1.dispatch_country_code |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна | operator:formalized.invoice_1.trading_country_code |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения | operator:formalized.invoice_1.destination_country_code |
| 16 | Registration_PrDocumentName | ИНВОЙС | CD | наименование документа | constant |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentNumber (CL на сетку .pdf) |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentDate (CL на сетку .pdf) |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | copied_from:formalized.invoice_1.Contract_PrDocumentNumber (CL на сетку .pdf) |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | copied_from:formalized.invoice_1.Contract_PrDocumentDate (CL на сетку .pdf) |
| 21 | Buyer_CompanyID | 1650389298 | CO | ИНН покупателя | allow_cross_doc_master_data_to_contract_invoice |
| 22 | Buyer_KPPCode | 165001001 | CO | КПП покупателя | allow_cross_doc_master_data_to_contract_invoice |
| 23 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO | наименование покупателя | allow_cross_doc_master_data_to_contract_invoice |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CO | индекс покупателя | allow_cross_doc_master_data_to_contract_invoice |
| 25 | Buyer_PostalAddress_CountryCode | RU | CO | страна покупателя alpha-2 | allow_cross_doc_master_data_to_contract_invoice |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CO | страна покупателя, текст | allow_cross_doc_master_data_to_contract_invoice |
| 27 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион | allow_cross_doc_master_data_to_contract_invoice |
| 28 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город | allow_cross_doc_master_data_to_contract_invoice |
| 29 | Buyer_PostalAddress_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator:formalized.contract_1.russian_person_address_line |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | продавец | copied_from:formalized.invoice_1.Seler_Name (CL на сетку .pdf) |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | operator:formalized.invoice_1.seller_country_code_alpha2 |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | cb:country |
| 33 | Seler_PostalAddress_Region | HEBEI | CD | регион продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_Region (CL на сетку .pdf) |
| 34 | Seler_PostalAddress_City | SHIJIAZHUANG | CD | город/район продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_City (CL на сетку .pdf) |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse (CL на сетку .pdf) |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | normalisation: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | normalisation: consignor=seller |
| 38 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | normalisation: consignor=seller |
| 39 | Consignor_Address_Region | HEBEI | CD | регион | normalisation: consignor=seller |
| 40 | Consignor_Address_City | SHIJIAZHUANG | CD | город/район | normalisation: consignor=seller |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | normalisation: consignor=seller |
| 42 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO | грузополучатель | allow_cross_doc_master_data_to_contract_invoice |
| 43 | Consignee_OGRN | 1201600020390 | CO | ОГРН | allow_cross_doc_master_data_to_contract_invoice |
| 44 | Consignee_INN | 1650389298 | CO | ИНН | allow_cross_doc_master_data_to_contract_invoice |
| 45 | Consignee_KPP | 165001001 | CO | КПП | allow_cross_doc_master_data_to_contract_invoice |
| 46 | Consignee_Address_PostalCode | 423800 | CO | индекс | allow_cross_doc_master_data_to_contract_invoice |
| 47 | Consignee_Address_CountryCode | RU | CO | страна alpha-2 | allow_cross_doc_master_data_to_contract_invoice |
| 48 | Consignee_Address_CounryName | РОССИЯ | CO | страна, текст | allow_cross_doc_master_data_to_contract_invoice |
| 49 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион | allow_cross_doc_master_data_to_contract_invoice |
| 50 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город | allow_cross_doc_master_data_to_contract_invoice |
| 51 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator:formalized.contract_1.russian_person_address_line |
| 52 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 53 | doc_code | 04021 | CD | код документа | constant |
| 54 | doc_name | ИНВОЙС | CD | наименование документа | constant |
| 55 | doc_number | LM-2591 | CD | номер документа | copied_from:formalized.invoice_1.Registration_PrDocumentNumber (CL на сетку .pdf) |
| 56 | doc_date | 30.10.2025 | CD | дата документа | copied_from:formalized.invoice_1.Registration_PrDocumentDate (CL на сетку .pdf) |

- _audit: 56
- `doc_status`: confirmed

#### Массив: InvoiceGoods[7]
- _array_audit: 7

#### Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.goods_1.GoodsCode (CL на сетку .pdf) |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара как в инвойсе | copied_from:formalized.invoice_1.goods_1.GoodsDescription (CL на сетку .pdf) |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | copied_from:formalized.invoice_1.goods_1.GoodsQuantity (CL на сетку .pdf) |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | copied_from:formalized.invoice_1.goods_1.goods_supplementary_quantity (CL на сетку .pdf) |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_1.gross_weight |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator:formalized.invoice_1.goods_1.net_weight |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from:formalized.invoice_1.goods_1.Price (CL на сетку .pdf) |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.goods_1.TotalCost (CL на сетку .pdf) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группировка по коду ТН ВЭД |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | группировка по коду ТН ВЭД |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.goods_2.GoodsCode (CL на сетку .pdf) |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара как в инвойсе | copied_from:formalized.invoice_1.goods_2.GoodsDescription (CL на сетку .pdf) |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | copied_from:formalized.invoice_1.goods_2.GoodsQuantity (CL на сетку .pdf) |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | copied_from:formalized.invoice_1.goods_2.goods_supplementary_quantity (CL на сетку .pdf) |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_2.gross_weight |
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator:formalized.invoice_1.goods_2.net_weight |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from:formalized.invoice_1.goods_2.Price (CL на сетку .pdf) |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.goods_2.TotalCost (CL на сетку .pdf) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группировка по коду ТН ВЭД |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | группировка по коду ТН ВЭД |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.goods_3.GoodsCode (CL на сетку .pdf) |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы «Антипыльца» из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | CD | описание товара как в инвойсе | copied_from:formalized.invoice_1.goods_3.GoodsDescription (CL на сетку .pdf) |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | copied_from:formalized.invoice_1.goods_3.GoodsQuantity (CL на сетку .pdf) |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | copied_from:formalized.invoice_1.goods_3.goods_supplementary_quantity (CL на сетку .pdf) |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_3.gross_weight |
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator:formalized.invoice_1.goods_3.net_weight |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from:formalized.invoice_1.goods_3.Price (CL на сетку .pdf) |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.goods_3.TotalCost (CL на сетку .pdf) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группировка по коду ТН ВЭД |
| 17 | dt_tovg_index | 3 | CD | индекс позиции внутри товара | группировка по коду ТН ВЭД |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.goods_4.GoodsCode (CL на сетку .pdf) |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CD | описание товара как в инвойсе | copied_from:formalized.invoice_1.goods_4.GoodsDescription (CL на сетку .pdf) |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | copied_from:formalized.invoice_1.goods_4.GoodsQuantity (CL на сетку .pdf) |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | copied_from:formalized.invoice_1.goods_4.goods_supplementary_quantity (CL на сетку .pdf) |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_4.gross_weight |
| 08 | NetWeightQuantity | 144.00 | CO | нетто по строке | operator:formalized.invoice_1.goods_4.net_weight |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from:formalized.invoice_1.goods_4.Price (CL на сетку .pdf) |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.goods_4.TotalCost (CL на сетку .pdf) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группировка по коду ТН ВЭД |
| 17 | dt_tovg_index | 4 | CD | индекс позиции внутри товара | группировка по коду ТН ВЭД |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.goods_5.GoodsCode (CL на сетку .pdf) |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара как в инвойсе | copied_from:formalized.invoice_1.goods_5.GoodsDescription (CL на сетку .pdf) |
| 03 | GoodsQuantity | 90 | CD | кол-во по строке инвойса | copied_from:formalized.invoice_1.goods_5.GoodsQuantity (CL на сетку .pdf) |
| 04 | goods_supplementary_quantity | 3780 | CD | количество в доп.ед.изм | copied_from:formalized.invoice_1.goods_5.goods_supplementary_quantity (CL на сетку .pdf) |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_5.gross_weight |
| 08 | NetWeightQuantity | 491.40 | CO | нетто по строке | operator:formalized.invoice_1.goods_5.net_weight |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from:formalized.invoice_1.goods_5.Price (CL на сетку .pdf) |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.goods_5.TotalCost (CL на сетку .pdf) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | группировка по коду ТН ВЭД |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | группировка по коду ТН ВЭД |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.goods_6.GoodsCode (CL на сетку .pdf) |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара как в инвойсе | copied_from:formalized.invoice_1.goods_6.GoodsDescription (CL на сетку .pdf) |
| 03 | GoodsQuantity | 180 | CD | кол-во по строке инвойса | copied_from:formalized.invoice_1.goods_6.GoodsQuantity (CL на сетку .pdf) |
| 04 | goods_supplementary_quantity | 8640 | CD | количество в доп.ед.изм | copied_from:formalized.invoice_1.goods_6.goods_supplementary_quantity (CL на сетку .pdf) |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_6.gross_weight |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто по строке | operator:formalized.invoice_1.goods_6.net_weight |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from:formalized.invoice_1.goods_6.Price (CL на сетку .pdf) |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.goods_6.TotalCost (CL на сетку .pdf) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | группировка по коду ТН ВЭД |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | группировка по коду ТН ВЭД |

- _item_audit: 17

#### Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.goods_7.GoodsCode (CL на сетку .pdf) |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара как в инвойсе | copied_from:formalized.invoice_1.goods_7.GoodsDescription (CL на сетку .pdf) |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке инвойса | copied_from:formalized.invoice_1.goods_7.GoodsQuantity (CL на сетку .pdf) |
| 04 | goods_supplementary_quantity | 240 | CD | количество в доп.ед.изм | copied_from:formalized.invoice_1.goods_7.goods_supplementary_quantity (CL на сетку .pdf) |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_7.gross_weight |
| 08 | NetWeightQuantity | 24.00 | CO | нетто по строке | operator:formalized.invoice_1.goods_7.net_weight |
| 09 | Price | 28 | CD | цена за единицу | copied_from:formalized.invoice_1.goods_7.Price (CL на сетку .pdf) |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.goods_7.TotalCost (CL на сетку .pdf) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группировка по коду ТН ВЭД |
| 17 | dt_tovg_index | 5 | CD | индекс позиции внутри товара | группировка по коду ТН ВЭД |

- _item_audit: 17

### `document`: Packing List
- `uqi_prefix`: formalized.packing_list
- `xml_target_root`: AltaE2PACK
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\PL на сетку .pdf
- `file_name`: PL на сетку .pdf
- `note`: Упаковочный лист поставки

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто по упаковочному | operator:formalized.invoice_1.total_gross_weight |
| 02 | NetWeightQuantity | 3302.00 | CO | общий вес нетто по упаковочному | operator:formalized.invoice_1.total_net_weight |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | copied_from:formalized.invoice_1.Seler_Name (CL на сетку .pdf) |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | краткое наименование | operator:formalized.packing_list_1.consignor_shortname_equals_full |
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | operator:formalized.packing_list_1.consignor_country_code_alpha2 |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | cb:country |
| 07 | Consignor_Address_Region | HEBEI | CD | регион | copied_from:formalized.invoice_1.Seler_PostalAddress_Region (CL на сетку .pdf) |
| 08 | Consignor_Address_City | SHIJIAZHUANG | CD | город/район | copied_from:formalized.invoice_1.Seler_PostalAddress_City (CL на сетку .pdf) |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse (CL на сетку .pdf) |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO | грузополучатель | allow_cross_doc_master_data_to_contract_invoice |
| 11 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator:formalized.packing_list_1.consignee_shortname_equals_full |
| 12 | Consignee_OGRN | 1201600020390 | CO | ОГРН | allow_cross_doc_master_data_to_contract_invoice |
| 13 | Consignee_INN | 1650389298 | CO | ИНН | allow_cross_doc_master_data_to_contract_invoice |
| 14 | Consignee_KPP | 165001001 | CO | КПП | allow_cross_doc_master_data_to_contract_invoice |
| 15 | Consignee_Address_PostalCode | 423800 | CO | индекс | allow_cross_doc_master_data_to_contract_invoice |
| 16 | Consignee_Address_CountryCode | RU | CO | страна alpha-2 | allow_cross_doc_master_data_to_contract_invoice |
| 17 | Consignee_Address_CounryName | РОССИЯ | CO | страна, текст | allow_cross_doc_master_data_to_contract_invoice |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион | allow_cross_doc_master_data_to_contract_invoice |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город | allow_cross_doc_master_data_to_contract_invoice |
| 20 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator:formalized.contract_1.russian_person_address_line |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryPlace (CL на сетку .pdf) |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | внутренний числовой код условий | operator:formalized.invoice_1.delivery_terms_numeric |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode (CL на сетку .pdf) |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | наименование контракта | operator:formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentName |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | copied_from:formalized.invoice_1.Contract_PrDocumentNumber (CL на сетку .pdf) |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | copied_from:formalized.invoice_1.Contract_PrDocumentDate (CL на сетку .pdf) |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | constant |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentNumber (CL на сетку .pdf) |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentDate (CL на сетку .pdf) |
| 30 | DeliveryTerms_Registration_PrDocumentName | Упаковочный лист | CO | наименование упаковочного | operator:formalized.packing_list_1.registration_doc_name |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | operator:formalized.packing_list_1.registration_doc_number |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | operator:formalized.packing_list_1.registration_doc_date |
| 33 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 34 | doc_code | 04131 | CD | код документа | constant |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | constant |
| 36 | doc_number | LM-2591 | CO | номер документа | operator:formalized.packing_list_1.registration_doc_number |
| 37 | doc_date | 30.10.2025 | CO | дата документа | operator:formalized.packing_list_1.registration_doc_date |

- _audit: 37
- `doc_status`: confirmed

#### Массив: Goods[7]
- _array_audit: 7

#### Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки как в документе | copied_from:formalized.packing_list_1.goods_1.GoodsDescription (PL на сетку .pdf) |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц в строке | copied_from:formalized.packing_list_1.goods_1.GoodsQuantity (PL на сетку .pdf) |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.goods_1.GrossWeightQuantity (PL на сетку .pdf) |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | copied_from:formalized.packing_list_1.goods_1.NetWeightQuantity (PL на сетку .pdf) |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок/мест в упаковке | operator:formalized.packing_list_1.goods_1.paking_quantity |

- _item_audit: 5

#### Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки как в документе | copied_from:formalized.packing_list_1.goods_2.GoodsDescription (PL на сетку .pdf) |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц в строке | copied_from:formalized.packing_list_1.goods_2.GoodsQuantity (PL на сетку .pdf) |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.goods_2.GrossWeightQuantity (PL на сетку .pdf) |
| 04 | NetWeightQuantity | 460.80 | CD | нетто по строке | copied_from:formalized.packing_list_1.goods_2.NetWeightQuantity (PL на сетку .pdf) |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок/мест в упаковке | operator:formalized.packing_list_1.goods_2.paking_quantity |

- _item_audit: 5

#### Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы «Антипыльца» из полиэстера . Размер рулона 1,42*0,64*0,22 | CD | описание строки как в документе | copied_from:formalized.packing_list_1.goods_3.GoodsDescription (PL на сетку .pdf) |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц в строке | copied_from:formalized.packing_list_1.goods_3.GoodsQuantity (PL на сетку .pdf) |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.goods_3.GrossWeightQuantity (PL на сетку .pdf) |
| 04 | NetWeightQuantity | 252.00 | CD | нетто по строке | copied_from:formalized.packing_list_1.goods_3.NetWeightQuantity (PL на сетку .pdf) |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок/мест в упаковке | operator:formalized.packing_list_1.goods_3.paking_quantity |

- _item_audit: 5

#### Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы «Антипыльца» из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание строки как в документе | copied_from:formalized.packing_list_1.goods_4.GoodsDescription (PL на сетку .pdf) |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц в строке | copied_from:formalized.packing_list_1.goods_4.GoodsQuantity (PL на сетку .pdf) |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.goods_4.GrossWeightQuantity (PL на сетку .pdf) |
| 04 | NetWeightQuantity | 144.00 | CD | нетто по строке | copied_from:formalized.packing_list_1.goods_4.NetWeightQuantity (PL на сетку .pdf) |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок/мест в упаковке | operator:formalized.packing_list_1.goods_4.paking_quantity |

- _item_audit: 5

#### Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание строки как в документе | copied_from:formalized.packing_list_1.goods_5.GoodsDescription (PL на сетку .pdf) |
| 02 | GoodsQuantity | 90 | CD | количество мест/грузовых единиц в строке | copied_from:formalized.packing_list_1.goods_5.GoodsQuantity (PL на сетку .pdf) |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.goods_5.GrossWeightQuantity (PL на сетку .pdf) |
| 04 | NetWeightQuantity | 491.40 | CD | нетто по строке | copied_from:formalized.packing_list_1.goods_5.NetWeightQuantity (PL на сетку .pdf) |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок/мест в упаковке | operator:formalized.packing_list_1.goods_5.paking_quantity |

- _item_audit: 5

#### Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание строки как в документе | copied_from:formalized.packing_list_1.goods_6.GoodsDescription (PL на сетку .pdf) |
| 02 | GoodsQuantity | 180 | CD | количество мест/грузовых единиц в строке | copied_from:formalized.packing_list_1.goods_6.GoodsQuantity (PL на сетку .pdf) |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.goods_6.GrossWeightQuantity (PL на сетку .pdf) |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто по строке | copied_from:formalized.packing_list_1.goods_6.NetWeightQuantity (PL на сетку .pdf) |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок/мест в упаковке | operator:formalized.packing_list_1.goods_6.paking_quantity |

- _item_audit: 5

#### Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки «Антипыльца» из полиэстера Размер рулона 1,72*0,35*0,31*1 | CD | описание строки как в документе | copied_from:formalized.packing_list_1.goods_7.GoodsDescription (PL на сетку .pdf) |
| 02 | GoodsQuantity | 5 | CD | количество мест/грузовых единиц в строке | copied_from:formalized.packing_list_1.goods_7.GoodsQuantity (PL на сетку .pdf) |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.goods_7.GrossWeightQuantity (PL на сетку .pdf) |
| 04 | NetWeightQuantity | 24.00 | CD | нетто по строке | copied_from:formalized.packing_list_1.goods_7.NetWeightQuantity (PL на сетку .pdf) |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок/мест в упаковке | operator:formalized.packing_list_1.goods_7.paking_quantity |

- _item_audit: 5

#### Массив: TransportMeans[2]
- _array_audit: 2

#### Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | O157AO774 | CO | регистрационный номер | operator:formalized.packing_list_1.transport_1.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator:formalized.packing_list_1.transport_1.mode_code |
| 03 | NationalityCode | 000 | CO | код национальности ТС | operator:formalized.packing_list_1.transport_1.nationality_code |
| 04 | MoverIndicator | true | CO | признак тягача | operator:formalized.packing_list_1.transport_1.mover_indicator |

- _item_audit: 4

#### Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | BT374974 | CO | регистрационный номер | operator:formalized.packing_list_1.transport_2.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator:formalized.packing_list_1.transport_2.mode_code |
| 03 | NationalityCode | 000 | CO | код национальности ТС | operator:formalized.packing_list_1.transport_2.nationality_code |
| 04 | MoverIndicator | false | CO | признак тягача | operator:formalized.packing_list_1.transport_2.mover_indicator |

- _item_audit: 4

### `document`: CMR
- `uqi_prefix`: formalized.cmr
- `xml_target_root`: AltaE3CMR
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\СМР от СВХ.pdf
- `file_name`: СМР от СВХ.pdf
- `note`: Международная товарно-транспортная накладная

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CO | язык документа | operator:formalized.cmr_1.language_code |
| 02 | CMR_Choice | 1 | CO | вариант Альты | operator:formalized.cmr_1.cmr_choice |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | copied_from:formalized.cmr_1.RegistrationDocument_RegID (СМР от СВХ.pdf) |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | copied_from:formalized.cmr_1.TrakingCargo_TakingCargoDate (СМР от СВХ.pdf) |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления | operator:formalized.cmr_1.registration_place |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата принятия груза | copied_from:formalized.cmr_1.TrakingCargo_TakingCargoDate (СМР от СВХ.pdf) |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2 | operator:formalized.cmr_1.taking_cargo_country_code_alpha2 |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза, текст | cb:country |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | operator:formalized.cmr_1.delivery_country_code_alpha2 |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки, текст | cb:country |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки по Incoterms | operator:formalized.cmr_1.delivery_terms_place |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator:formalized.cmr_1.delivery_terms_string |
| 13 | GoodsQuantity | 127 | CD | общее число грузовых мест | copied_from:formalized.cmr_1.GoodsQuantity (СМР от СВХ.pdf) |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто | copied_from:formalized.cmr_1.CMRGoodsWeight_GrossWeightQuantity (СМР от СВХ.pdf) |
| 15 | CMRTransport_PrimeMoverStateSignID | O157AO774 | CD | гос. номер тягача | copied_from:formalized.cmr_1.CMRTransport_PrimeMoverStateSignID (СМР от СВХ.pdf) |
| 16 | CMRTransport_TrailerStateSignID | BT374974 | CD | гос. номер прицепа | copied_from:formalized.cmr_1.CMRTransport_TrailerStateSignID (СМР от СВХ.pdf) |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование отправителя | copied_from:formalized.cmr_1.Consignor_NameInf (СМР от СВХ.pdf) |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator:formalized.cmr_1.consignor_shortname_equals_full |
| 19 | Consignor_PostalAddress_CountryCode | CN | CO | страна alpha-2 | operator:formalized.cmr_1.consignor_country_code_alpha2 |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна, текст | cb:country |
| 21 | Consignor_Address_Region | Hebei | CD | регион | copied_from:formalized.cmr_1.Consignor_Address_Region (СМР от СВХ.pdf) |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from:formalized.cmr_1.Consignor_Address_City (СМР от СВХ.pdf) |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from:formalized.cmr_1.Consignor_Address_StreetHouse (СМР от СВХ.pdf) |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | наименование гаранта | operator:formalized.cmr_1.consignor_guarantee_all |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое наименование | operator:formalized.cmr_1.consignor_guarantee_all |
| 26 | Consignor_Guarantee_Address_CountryCode | ОТСУТСТВУЕТ | CO | страна alpha-2 | operator:formalized.cmr_1.consignor_guarantee_all |
| 27 | Consignor_Guarantee_Address_CounryName | ОТСУТСТВУЕТ | CO | страна, текст | operator:formalized.cmr_1.consignor_guarantee_all |
| 28 | Consignor_Guarantee_Address_Region | ОТСУТСТВУЕТ | CO | регион | operator:formalized.cmr_1.consignor_guarantee_all |
| 29 | Consignor_Guarantee_Address_City | ОТСУТСТВУЕТ | CO | город/район | operator:formalized.cmr_1.consignor_guarantee_all |
| 30 | Consignor_Guarantee_Address_StreetHouse | ОТСУТСТВУЕТ | CO | улица/дом одной строкой | operator:formalized.cmr_1.consignor_guarantee_all |
| 31 | Consignee_NameInf | ООО «Скиф» | CD | наименование получателя | copied_from:formalized.cmr_1.Consignee_NameInf (СМР от СВХ.pdf) |
| 32 | Consignee_ShortName | ООО «Скиф» | CO | краткое наименование | operator:formalized.cmr_1.consignee_shortname_equals_full |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | operator:formalized.cmr_1.consignee_ogrn_from_master_data |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | copied_from:formalized.cmr_1.Consignee_INNID (СМР от СВХ.pdf) |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | copied_from:formalized.cmr_1.Consignee_KPPCode (СМР от СВХ.pdf) |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | copied_from:formalized.cmr_1.Consignee_PostalAddress_PostalCode (СМР от СВХ.pdf) |
| 37 | Consignee_PostalAddress_CountryCode | RU | CO | страна alpha-2 | allow_cross_doc_master_data_to_contract_invoice |
| 38 | Consignee_Address_CounryName | РОССИЯ | CO | страна, текст | allow_cross_doc_master_data_to_contract_invoice |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | copied_from:formalized.cmr_1.Consignee_Address_Region (СМР от СВХ.pdf) |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | copied_from:formalized.cmr_1.Consignee_Address_City (СМР от СВХ.pdf) |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом/офис одной строкой | copied_from:formalized.cmr_1.Consignee_Address_StreetHouse (СМР от СВХ.pdf) |
| 42 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 43 | doc_code | 02015 | CD | код документа | constant |
| 44 | doc_name | CMR | CD | наименование документа | constant |
| 45 | doc_number | 00378 | CD | номер документа | copied_from:formalized.cmr_1.RegistrationDocument_RegID (СМР от СВХ.pdf) |
| 46 | doc_date | 20.01.2026 | CD | дата документа | copied_from:formalized.cmr_1.TrakingCargo_TakingCargoDate (СМР от СВХ.pdf) |

- _audit: 46
- `doc_status`: confirmed

#### Массив: CMRGoods[1]
- _array_audit: 1

#### Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | авто-нумерация единственной строки |
| 02 | GoodsDescription | МОСКИТНАЯ СЕТКА | CD | описание груза/товара | derived from invoice/TD |
| 03 | PakingQuantity | 127 | CO | кол-во упаковок/мест | operator:formalized.cmr_1.goods_1.packing_quantity |

- _item_audit: 3

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\платежки\currency_transfer_1_13.01.2026.pdf
- `file_name`: currency_transfer_1_13.01.2026.pdf
- `note`: Заявление на перевод № 1 (предоплата)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | constant |
| 02 | PaymentModeCode | 0 | CO | код способа платежа | operator:formalized.payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | copied_from:formalized.payment_order_1.PaymentAmount (currency_transfer_1_13.01.2026.pdf) |
| 04 | TransactionKind | 01 | CO | вид операции | operator:formalized.payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator:formalized.payment_order_all.priority |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from:formalized.payment_order_1.Purpose (currency_transfer_1_13.01.2026.pdf) |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | copied_from:formalized.payment_order_1.ValueSpelledOut (currency_transfer_1_13.01.2026.pdf) |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber (currency_transfer_1_13.01.2026.pdf) |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate (currency_transfer_1_13.01.2026.pdf) |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | copied_from:formalized.payment_order_1.Payer_OrganizationName (currency_transfer_1_13.01.2026.pdf) |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from:formalized.payment_order_1.Payer_INN (currency_transfer_1_13.01.2026.pdf) |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator:formalized.payment_order_1.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ 'ЦЕНТРАЛЬНЫЙ' БАНКА ВТБ (ПАО)) | CD | реквизиты банка плательщика | copied_from:formalized.payment_order_1.Payer_Bank_BankName (currency_transfer_1_13.01.2026.pdf) |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | copied_from:formalized.payment_order_1.Payee_OrganizationName (currency_transfer_1_13.01.2026.pdf) |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | реквизиты банка получателя | copied_from:formalized.payment_order_1.Payee_Bank_BankName (currency_transfer_1_13.01.2026.pdf) |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator:formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator:formalized.payment_order_all.payer_sign.name |
| 18 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 19 | doc_code | 04023 | CD | код документа | constant |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | constant |
| 21 | doc_number | 1 | CD | номер документа | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber (currency_transfer_1_13.01.2026.pdf) |
| 22 | doc_date | 13.01.2026 | CD | дата документа | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate (currency_transfer_1_13.01.2026.pdf) |

- _audit: 22
- `doc_status`: confirmed

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\платежки\currency_transfer_7_28.11.2025.pdf
- `file_name`: currency_transfer_7_28.11.2025.pdf
- `note`: Заявление на перевод № 7 (предоплата)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | constant |
| 02 | PaymentModeCode | 0 | CO | код способа платежа | operator:formalized.payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | copied_from:formalized.payment_order_2.PaymentAmount (currency_transfer_7_28.11.2025.pdf) |
| 04 | TransactionKind | 01 | CO | вид операции | operator:formalized.payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator:formalized.payment_order_all.priority |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from:formalized.payment_order_2.Purpose (currency_transfer_7_28.11.2025.pdf) |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | copied_from:formalized.payment_order_2.ValueSpelledOut (currency_transfer_7_28.11.2025.pdf) |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | copied_from:formalized.payment_order_2.DocumentReference_PrDocumentNumber (currency_transfer_7_28.11.2025.pdf) |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | copied_from:formalized.payment_order_2.DocumentReference_PrDocumentDate (currency_transfer_7_28.11.2025.pdf) |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | copied_from:formalized.payment_order_2.Payer_OrganizationName (currency_transfer_7_28.11.2025.pdf) |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from:formalized.payment_order_2.Payer_INN (currency_transfer_7_28.11.2025.pdf) |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator:formalized.payment_order_2.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ 'ЦЕНТРАЛЬНЫЙ' БАНКА ВТБ (ПАО)) | CD | реквизиты банка плательщика | copied_from:formalized.payment_order_2.Payer_Bank_BankName (currency_transfer_7_28.11.2025.pdf) |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | copied_from:formalized.payment_order_2.Payee_OrganizationName (currency_transfer_7_28.11.2025.pdf) |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | реквизиты банка получателя | copied_from:formalized.payment_order_2.Payee_Bank_BankName (currency_transfer_7_28.11.2025.pdf) |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator:formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator:formalized.payment_order_all.payer_sign.name |
| 18 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 19 | doc_code | 04023 | CD | код документа | constant |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | constant |
| 21 | doc_number | 7 | CD | номер документа | copied_from:formalized.payment_order_2.DocumentReference_PrDocumentNumber (currency_transfer_7_28.11.2025.pdf) |
| 22 | doc_date | 28.11.2025 | CD | дата документа | copied_from:formalized.payment_order_2.DocumentReference_PrDocumentDate (currency_transfer_7_28.11.2025.pdf) |

- _audit: 22
- `doc_status`: confirmed

### `document`: Service Invoice
- `uqi_prefix`: formalized.service_invoice
- `xml_target_root`: AltaServiceInvoice
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\Счет_№26-00378-tl_от_27-01-2026.pdf
- `file_name`: Счет_№26-00378-tl_от_27-01-2026.pdf
- `note`: Счет за перевозку

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак документа Альты | operator:formalized.service_invoice_1.document_sign |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | copied_from:formalized.service_invoice_1.TotalServiceCost (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 03 | Currency | USD | CD | валюта итого | copied_from:formalized.service_invoice_1.Currency (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг/перевозчик | copied_from:formalized.service_invoice_1.ServiceProvider_Name (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 05 | BankName | АО "Райффайзенбанк"; БИК 044525700; Сч. № 30101810200000000700; Сч. № 40702810400000233463 | CD | банк исполнителя | copied_from:formalized.service_invoice_1.BankName (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 06 | ContractDetails_PrDocumentNumber | №КООО/26651/М | CD | № договора на услуги/перевозку | copied_from:formalized.service_invoice_1.ContractDetails_PrDocumentNumber (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 07 | ContractDetails_PrDocumentDate | 13-05-2025 | CD | дата договора на услуги/перевозку | copied_from:formalized.service_invoice_1.ContractDetails_PrDocumentDate (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 08 | PrDocumentNumber | ОТСУТСТВУЕТ | CO | связанный документ/заказ: номер | operator:formalized.service_invoice_1.payment_document_number |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CO | связанный документ/заказ: дата | operator:formalized.service_invoice_1.payment_document_date |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | copied_from:formalized.service_invoice_1.Registration_PrDocumentName (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | copied_from:formalized.service_invoice_1.Registration_PrDocumentNumber (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | copied_from:formalized.service_invoice_1.Registration_PrDocumentDate (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | operator:formalized.service_invoice_1.consignor_decision |
| 14 | PostalCode | ОТСУТСТВУЕТ | CO | индекс | operator:formalized.service_invoice_1.consignor_postalcode_empty_ok |
| 15 | CountryCode | CN | CO | страна alpha-2 | service_invoice_1.consignor_address_from_seller |
| 16 | CounryName | КИТАЙ | CO | страна, текст | service_invoice_1.consignor_address_from_seller |
| 17 | Region | HEBEI | CO | регион | service_invoice_1.consignor_address_from_seller |
| 18 | Town | SHIJIAZHUANG | CO | город/район | service_invoice_1.consignor_address_from_seller |
| 19 | StreetHouse | No. 5 Gaodong street | CO | улица/дом одной строкой | service_invoice_1.consignor_address_from_seller |
| 20 | Consignee_OrganizationName | ООО "Скиф" (ООО "СКИФ") | CD | грузополучатель | copied_from:formalized.service_invoice_1.Consignee_OrganizationName (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator:formalized.service_invoice_1.consignee_ogrn_from_master_data |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | copied_from:formalized.service_invoice_1.Consignee_RFOrganizationFeatures_INN (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | copied_from:formalized.service_invoice_1.Consignee_RFOrganizationFeatures_KPP (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 24 | PostalCode | 423800 | CD | индекс | copied_from:formalized.service_invoice_1.PostalCode (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 25 | CountryCode | RU | CO | страна alpha-2 | allow_cross_doc_master_data_to_contract_invoice |
| 26 | CounryName | Россия | CD | страна, текст | copied_from:formalized.service_invoice_1.CounryName (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 27 | Region | Республика Татарстан | CD | регион | copied_from:formalized.service_invoice_1.Region (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 28 | Town | Набережные Челны | CD | город | copied_from:formalized.service_invoice_1.Town (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 29 | StreetHouse | проезд Хлебный | CD | улица | copied_from:formalized.service_invoice_1.StreetHouse (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 30 | House | 30 | CO | дом | operator:formalized.service_invoice_1.consignee_house |
| 31 | Room | 211 | CO | офис/кв | operator:formalized.service_invoice_1.consignee_room |
| 32 | Signature_Choice | 2 | CD | вариант подписи | copied_from:formalized.service_invoice_1.Signature_Choice (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 33 | IndividualEntrepreneur_PersonSurname | ОТСУТСТВУЕТ | CD | фамилия ИП | constant |
| 34 | IndividualEntrepreneur_PersonName | ОТСУТСТВУЕТ | CD | имя ИП | constant |
| 35 | IndividualEntrepreneur_PersonMiddleName | ОТСУТСТВУЕТ | CD | отчество ИП | constant |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климовин | CD | фамилия руководителя | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonSurname (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CD | имя руководителя | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonName (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | ОТСУТСТВУЕТ | CD | отчество руководителя | constant |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CD | имя бухгалтера | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonName (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | ОТСУТСТВУЕТ | CD | отчество бухгалтера | constant |
| 42 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 43 | doc_code | 04031 | CD | код документа | constant |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | constant |
| 45 | doc_number | 26-00378-tl | CD | номер документа | copied_from:formalized.service_invoice_1.Registration_PrDocumentNumber (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 46 | doc_date | 27.01.2026 | CD | дата документа | copied_from:formalized.service_invoice_1.Registration_PrDocumentDate (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 47 | transport_to_border | 1404.00 | CO | стоимость до границы | operator:formalized.service_invoice_1.transport_to_border |
| 48 | transport_currency | USD | CO | валюта стоимости | operator:formalized.service_invoice_1.transport_currency |

- _audit: 48
- `doc_status`: confirmed

#### Массив: ServiceDescription[2]
- _array_audit: 2

#### Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) - перевозка автотранспортом | CD | описание услуги | copied_from:formalized.service_invoice_1.service_1.GoodsDescription (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:formalized.service_invoice_1.service_1.CurrencyCode (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 03 | ServiceName | China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) | CD | наименование/маршрут | copied_from:formalized.service_invoice_1.service_1.ServiceName (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from:formalized.service_invoice_1.service_1.TaxRate (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:formalized.service_invoice_1.service_1.TaxSum (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | copied_from:formalized.service_invoice_1.service_1.ServiceCost_Amount (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | copied_from:formalized.service_invoice_1.service_1.ServiceCost_Currency (Счет_№26-00378-tl_от_27-01-2026.pdf) |

- _item_audit: 7

#### Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | copied_from:formalized.service_invoice_1.service_2.GoodsDescription (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:formalized.service_invoice_1.service_2.CurrencyCode (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 03 | ServiceName | граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | наименование/маршрут | copied_from:formalized.service_invoice_1.service_2.ServiceName (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from:formalized.service_invoice_1.service_2.TaxRate (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:formalized.service_invoice_1.service_2.TaxSum (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | copied_from:formalized.service_invoice_1.service_2.ServiceCost_Amount (Счет_№26-00378-tl_от_27-01-2026.pdf) |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | copied_from:formalized.service_invoice_1.service_2.ServiceCost_Currency (Счет_№26-00378-tl_от_27-01-2026.pdf) |

- _item_audit: 7

### `document`: Insurance Services Invoice
- `uqi_prefix`: formalized.insurance_invoice
- `xml_target_root`: AltaFreeDoc
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\Счет_№26-00378-tl_1_от_14-01-2026.pdf
- `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.pdf
- `note`: Счет за страхование

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | constant |
| 02 | DocumentHead_DocumentName | Счет на оплату №26-00378-tl/1 от 14.01.2026 г. | CD | наименование документа | copied_from:formalized.insurance_invoice_1.DocumentHead_DocumentName (Счет_№26-00378-tl_1_от_14-01-2026.pdf) |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | copied_from:formalized.insurance_invoice_1.DocumentHead_DocumentDate (Счет_№26-00378-tl_1_от_14-01-2026.pdf) |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | copied_from:formalized.insurance_invoice_1.DocumentHead_DocumentNumber (Счет_№26-00378-tl_1_от_14-01-2026.pdf) |
| 05 | TextPara | link:HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | основной текст/условия | operator:formalized.insurance_document_1.textpara_storage |
| 06 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 07 | doc_code | 04111 | CD | код документа | constant |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | constant |
| 09 | doc_number | 26-00378-tl/1 | CD | номер документа | copied_from:formalized.insurance_invoice_1.DocumentHead_DocumentNumber (Счет_№26-00378-tl_1_от_14-01-2026.pdf) |
| 10 | doc_date | 14.01.2026 | CD | дата документа | copied_from:formalized.insurance_invoice_1.DocumentHead_DocumentDate (Счет_№26-00378-tl_1_от_14-01-2026.pdf) |
| 11 | insurance_to_border | 910.34 | CO | стоимость страхования | operator:formalized.insurance_document_1.insurance_to_border |
| 12 | insurance_currency | RUB | CO | валюта страхования | operator:formalized.insurance_document_1.insurance_currency |

- _audit: 12
- `doc_status`: confirmed

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description
- `xml_target_root`: AltaFreeDoc
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\техничка Антикот, антипыльца антимошка .pdf
- `file_name`: техничка Антикот, антипыльца антимошка .pdf
- `note`: Техническое описание товаров

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | constant |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование техописания | copied_from:formalized.tech_description_1.DocumentHead_DocumentName (техничка Антикот, антипыльца антимошка .pdf) |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата техописания | operator:formalized.tech_description_1.date |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания | operator:formalized.tech_description_1.number |
| 05 | TextPara | link:HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md | CD | технический текст | rule 3.7.5: link to md source |
| 06 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 07 | doc_code | 05999 | CD | код документа | constant |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | constant |
| 09 | doc_number | Б/Н | CO | номер документа | operator:formalized.tech_description_1.number |
| 10 | doc_date | 30.10.2025 | CO | дата документа | operator:formalized.tech_description_1.date |

- _audit: 10
- `doc_status`: confirmed

### `document`: Contract
- `uqi_prefix`: master_data.contract
- `path`: alta\master_data.md
- `file_name`: master_data.md
- `note`: Контракт купли-продажи

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 02 | doc_code | 03011 | CD | код документа | constant |
| 03 | doc_name | КОНТРАКТ | CD | наименование документа | constant |
| 04 | doc_number | LM-2553 | CD | номер документа | master_keys.md |
| 05 | doc_date | 02.07.2025 | CD | дата документа | master_keys.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Supplementary Contract
- `uqi_prefix`: master_data.supplementary_contract_1
- `path`: alta\master_data.md
- `file_name`: master_data.md
- `note`: Дополнительное соглашение № 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 02 | doc_code | 03012 | CD | код документа | constant |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наименование документа | constant |
| 04 | doc_number | 1 | CD | номер документа | master_keys.md |
| 05 | doc_date | 25.11.2025 | CD | дата документа | master_keys.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: EGRUL
- `uqi_prefix`: master_data.egrul
- `path`: alta\master_data.md
- `file_name`: master_data.md
- `note`: Выписка из ЕГРЮЛ декларанта

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование организации | master_data.md |
| 02 | ShortName | ООО "СКИФ" | CD | краткое наименование | master_data.md |
| 03 | OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 04 | INN | 1650389298 | CD | ИНН | master_data.md |
| 05 | KPP | 165001001 | CD | КПП | master_data.md |
| 06 | Address_PostalCode | 423800 | CD | индекс | master_data.md |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 08 | Address_CounryName | РОССИЯ | CD | страна, текст | master_data.md |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data.md |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data.md |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | master_data.md |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | master_data.md |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | master_data.md |
| 14 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 15 | doc_code | 04011 | CD | код документа | constant |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | constant |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | master_keys.md |
| 18 | doc_date | 14.07.2025 | CD | дата документа | master_keys.md |

- _audit: 18
- `doc_status`: confirmed

### `document`: Personal Passport
- `uqi_prefix`: master_data.passport
- `path`: alta\master_data.md
- `file_name`: master_data.md
- `note`: Паспорт представителя

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
| 10 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 11 | doc_code | 11001 | CD | код документа | constant |
| 12 | doc_name | ПАСПОРТ | CD | наименование документа | constant |
| 13 | doc_number | 63 09 449948 | CD | номер документа | master_keys.md |
| 14 | doc_date | 11.03.2010 | CD | дата документа | master_keys.md |

- _audit: 14
- `doc_status`: confirmed

### `document`: Letter of Attorney
- `uqi_prefix`: master_data.letter_of_attorney
- `path`: alta\master_data.md
- `file_name`: master_data.md
- `note`: Доверенность представителя

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_keys.md |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_keys.md |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_data.md |
| 05 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 06 | doc_code | 11004 | CD | код документа | constant |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | constant |
| 08 | doc_number | 1 | CD | номер документа | master_keys.md |
| 09 | doc_date | 01.02.2026 | CD | дата документа | master_keys.md |

- _audit: 9
- `doc_status`: confirmed

### `document`: Transport Contract
- `uqi_prefix`: master_data.transport_contract
- `path`: alta\master_data.md
- `file_name`: master_data.md
- `note`: Договор транспортной экспедиции

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 02 | doc_code | 04033 | CD | код документа | constant |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование документа | constant |
| 04 | doc_number | КООО/26651/М | CD | номер документа | master_keys.md |
| 05 | doc_date | 13.05.2025 | CD | дата документа | master_keys.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter
- `uqi_prefix`: master_data.exemption_letter
- `path`: alta\master_data.md
- `file_name`: master_data.md
- `note`: Отказное письмо

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 02 | doc_code | 09023 | CD | код документа | constant |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | constant |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | master_keys.md |
| 05 | doc_date | 20.08.2025 | CD | дата документа | master_keys.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter (source)
- `uqi_prefix`: master_data.exemption_letter_source
- `path`: alta\master_data.md
- `file_name`: master_data.md
- `note`: Отказное письмо (источник)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 02 | doc_code | 09999 | CD | код документа | constant |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | constant |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | master_keys.md |
| 05 | doc_date | 20.08.2025 | CD | дата документа | master_keys.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Goods Description
- `uqi_prefix`: non_formalized.goods_description_1
- `note`: Служебный документ описания товаров для графы 31

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | служебный признак включения в графу 44 | constant |

- _audit: 1
- `doc_status`: confirmed

#### Массив: goods[2]
- _array_audit: 2

#### Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 5804101000 | CD | ТН ВЭД | группировка по коду ТН ВЭД |
| 02 | description | СЕТКА МОСКИТНАЯ ИЗ ПОЛИЭСТЕРА, ПЛЕТЕНАЯ, В РУЛОНАХ, ПРЕДНАЗНАЧЕНА ДЛЯ ЗАЩИТЫ ОТ НАСЕКОМЫХ, ПЫЛИ И ПЫЛЬЦЫ (МОДЕЛИ АНТИКОТ, АНТИПЫЛЬЦА, ТРЕХСЛОЙНАЯ АНТИПЫЛЬЦА). МЕТОД ИЗГОТОВЛЕНИЯ - ПЛЕТЕНИЕ. | CD | описание товара | rule 7.6: composed description |

- _item_audit: 2

#### Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 7019900095 | CD | ТН ВЭД | группировка по коду ТН ВЭД |
| 02 | description | СЕТКА МОСКИТНАЯ ИЗ СТЕКЛОВОЛОКНА, ПЛЕТЕНАЯ, В РУЛОНАХ, ПРЕДНАЗНАЧЕНА ДЛЯ ЗАЩИТЫ ОТ МЕЛЬЧАЙШИХ НАСЕКОМЫХ И МОШЕК (МОДЕЛЬ АНТИМОШКА). МЕТОД ИЗГОТОВЛЕНИЯ - ПЛЕТЕНИЕ. | CD | описание товара | rule 7.6: composed description |

- _item_audit: 2

### `document`: Transit Declaration
- `uqi_prefix`: non_formalized.td
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\ТД 10719110_240126_5011363_reg00378тд.pdf
- `file_name`: ТД 10719110_240126_5011363_reg00378тд.pdf
- `note`: Транзитная декларация

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs_post_code | 10719110 | CD | код таможенного органа | copied_from:non_formalized.td.customs_post_code (ТД 10719110_240126_5011363_reg00378тд.pdf) |
| 02 | customs_post_name | ОТСУТСТВУЕТ | CD | наименование таможенного органа | не указано в тексте ТД |
| 03 | transport_reg_number | O157AO774/BT374974 | CD | ТС по ТД | copied_from:non_formalized.td.transport_reg_number (ТД 10719110_240126_5011363_reg00378тд.pdf) |
| 04 | doc_gr44 | true | CD | служебный признак включения в графу 44 | constant |
| 05 | doc_code | 09013 | CD | код документа | constant |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | наименование документа | constant |
| 07 | doc_number | 10719110/240126/5011363 | CO | номер документа | operator:non_formalized.transit_declaration_1.number |
| 08 | doc_date | 24.01.2026 | CO | дата документа | operator:non_formalized.transit_declaration_1.date |

- _audit: 8
- `doc_status`: confirmed

### `document`: Storage Report
- `uqi_prefix`: non_formalized.svh
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\ДО 14431420260204161621.jpg
- `file_name`: ДО 14431420260204161621.jpg
- `note`: Отчет СВХ (ДО-1)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии/свидетельства СВХ | copied_from:non_formalized.svh.warehouse_license_number (ДО 14431420260204161621.jpg) |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии/свидетельства СВХ | copied_from:non_formalized.svh.warehouse_license_date (ДО 14431420260204161621.jpg) |
| 03 | actual_gross_weight | 3500 | CD | фактический вес по весам | operator:non_formalized.svh_1.actual_totals_from_svh_additional_sheet |
| 04 | actual_places | 127 | CD | фактическое количество мест | operator:non_formalized.svh_1.actual_totals_from_svh_additional_sheet |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС при въезде | copied_from:non_formalized.svh.transport_reg_number (ДО 14431420260204161621.jpg) |
| 06 | doc_gr44 | false | CD | служебный признак включения в графу 44 | constant |

- _audit: 6
- `doc_status`: confirmed

#### Массив: goods[2]
- _array_audit: 2

#### Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 7019900095 | CD | код товара | copied_from:non_formalized.svh.goods_1.tnved (ДО 14431420260204161621.jpg) |
| 02 | places | 27 | CD | кол-во грузовых мест по строке | copied_from:non_formalized.svh.goods_1.places (ДО 14431420260204161621.jpg) |
| 03 | gross_weight_kg | 1710 | CD | вес брутто по строке | copied_from:non_formalized.svh.goods_1.gross_weight_kg (ДО 14431420260204161621.jpg) |
| 04 | cost | 42228 | CD | стоимость по строке | copied_from:non_formalized.svh.goods_1.cost (ДО 14431420260204161621.jpg) |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from:non_formalized.svh.goods_1.currency_code (ДО 14431420260204161621.jpg) |

- _item_audit: 5

#### Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара | copied_from:non_formalized.svh.goods_2.tnved (ДО 14431420260204161621.jpg) |
| 02 | places | 100 | CD | кол-во грузовых мест по строке | copied_from:non_formalized.svh.goods_2.places (ДО 14431420260204161621.jpg) |
| 03 | gross_weight_kg | 1790 | CD | вес брутто по строке | copied_from:non_formalized.svh.goods_2.gross_weight_kg (ДО 14431420260204161621.jpg) |
| 04 | cost | 55032 | CD | стоимость по строке | copied_from:non_formalized.svh.goods_2.cost (ДО 14431420260204161621.jpg) |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from:non_formalized.svh.goods_2.currency_code (ДО 14431420260204161621.jpg) |

- _item_audit: 5

### `document`: Storage Report Additional Sheet
- `uqi_prefix`: non_formalized.svh_additional_sheet_1
- `path`: HEBEI LANGMAI IMPORT AND EXPORT\02\ДО доп 14431520260204161645.jpg
- `file_name`: ДО доп 14431520260204161645.jpg
- `note`: Добавочный лист к отчету СВХ

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 1 | CD | № доп.листа/приложения | copied_from:non_formalized.svh_additional_sheet_1.number (ДО доп 14431520260204161645.jpg) |
| 02 | date | 03.02.2026 | CD | дата доп.листа | copied_from:non_formalized.svh.date (ДО 14431420260204161621.jpg) |
| 03 | actual_gross_weight | 3500 | CD | фактический вес по весам | copied_from:non_formalized.svh_additional_sheet_1.actual_gross_weight (ДО доп 14431520260204161645.jpg) |
| 04 | actual_places | 127 | CD | фактическое количество мест | copied_from:non_formalized.svh_additional_sheet_1.actual_places (ДО доп 14431520260204161645.jpg) |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС при въезде | copied_from:non_formalized.svh.transport_reg_number (ДО 14431420260204161621.jpg) |
| 06 | svh_address_region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион СВХ | operator:non_formalized.svh_additional_sheet_1.svh_address_region |
| 07 | svh_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город/нас.пункт СВХ | operator:non_formalized.svh_additional_sheet_1.svh_address_city |
| 08 | svh_address_street_house | Производственный пр-д, д. 45 | CO | улица/дом СВХ одной строкой | operator:non_formalized.svh_additional_sheet_1.svh_address_street_house |
| 09 | svh_customs_code | 10404083 | CO | код таможенного органа в зоне СВХ | operator:non_formalized.svh_additional_sheet_1.svh_customs_code |
| 10 | doc_gr44 | false | CD | служебный признак включения в графу 44 | constant |

- _audit: 10
- `doc_status`: confirmed

### Итогo, по файлу:

`total_unreliable_fields`: 0
`primary_status`: confirmed

## 5. Нерешенные вопросы (Issues)

*Нет нерешенных вопросов. Все спорные моменты и недостающие данные успешно урегулированы решениями оператора.*

## 6. `unreliable_fields`:
*Нет недостоверно распознанных полей.*
