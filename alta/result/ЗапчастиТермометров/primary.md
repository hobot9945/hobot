# Первичные данные

## 1. meta:
- `название кейса`: ЗапчастиТермометров
- `путь к папке поставки`: alta\source\ЗапчастиТермометров
- `direction`: ИМ
- `тип поставки`: 1 ДТ
- `источники данных:` md + master_data

## 2. formalized:

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: alta\source\ЗапчастиТермометров\md\Инвойс HNKY260226 с печатью .md
  - `file_name`: Инвойс HNKY260226 с печатью .md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | CurrencyRate | | CO | курс валюты | operator:confirmed |
| 02 | CurrencyCode | CNY | CD | валюта инвойса | copied_from:formalized.invoice_1.CurrencyCode (Инвойс HNKY260226 с печатью .md) |
| 03 | DocumentCode | 04021 | CD | код вида документа | константа |
| 04 | PlacesQuantity | 8 | CD | кол-во грузовых мест | copied_from:formalized.packing_list_1.PlacesQuantity (Упаковочный .md) |
| 05 | PlacesDescription | КАРТОННАЯ КОРОБКА | CD | описание мест | copied_from:formalized.packing_list_1 (Упаковочный .md) |
| 06 | GrossWeightQuantity | 78.00 | CD | общий вес брутто | copied_from:formalized.packing_list_1.GrossWeightQuantity (Упаковочный .md) |
| 07 | NetWeightQuantity | 76.45 | CD | общий вес нетто | copied_from:formalized.packing_list_1.NetWeightQuantity (Упаковочный .md) |
| 08 | GCost | 49500.00 | CD | системное поле стоимости | derived: равен TotalCost |
| 09 | TotalCost | 49500.00 | CD | итого по инвойсу | copied_from:formalized.invoice_1.TotalCost (Инвойс HNKY260226 с печатью .md) |
| 10 | DeliveryTerms_DeliveryPlace | Ningbo | CD | место поставки | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryPlace (Инвойс HNKY260226 с печатью .md) |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | | CO | числовой код условий | operator:confirmed |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode (Инвойс HNKY260226 с печатью .md) |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | copied_from:formalized.invoice_1.DeliveryTerms_DispatchCountryCode (Инвойс HNKY260226 с печатью .md) |
| 14 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | copied_from:formalized.invoice_1.DeliveryTerms_TradingCountryCode (Инвойс HNKY260226 с печатью .md) |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | copied_from:formalized.invoice_1.DeliveryTerms_DestinationCountryCode (Инвойс HNKY260226 с печатью .md) |
| 16 | Registration_PrDocumentName | КОММЕРЧЕСКИЙ ИНВОЙС | CD | наименование документа | copied_from:formalized.invoice_1.Registration_PrDocumentName (Инвойс HNKY260226 с печатью .md) |
| 17 | Registration_PrDocumentNumber | HNKY260226 | CD | номер инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentNumber (Инвойс HNKY260226 с печатью .md) |
| 18 | Registration_PrDocumentDate | 26.02.2026 | CD | дата инвойса | copied_from:formalized.invoice_1.Registration_PrDocumentDate (Инвойс HNKY260226 с печатью .md) |
| 19 | Contract_PrDocumentNumber | HNKY250929 | CD | № контракта-ссылки | copied_from:formalized.invoice_1.Contract_PrDocumentNumber (Инвойс HNKY260226 с печатью .md) |
| 20 | Contract_PrDocumentDate | 29.09.2025 | CD | дата контракта-ссылки | copied_from:formalized.invoice_1.Contract_PrDocumentDate (Инвойс HNKY260226 с печатью .md) |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | master_data:master_proto.md (declarant.inn) |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | master_data:master_proto.md (declarant.kpp) |
| 23 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование покупателя | master_data:master_proto.md (declarant.organization_name) |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | master_data:master_proto.md (declarant.postal_code) |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя (код) | master_data:master_proto.md (declarant.country_code) |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя (текст) | master_data:master_proto.md (declarant.country_name) |
| 27 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион покупателя | master_data:master_proto.md (declarant.region) |
| 28 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город покупателя | master_data:master_proto.md (declarant.city) |
| 29 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом покупателя | master_data:master_proto.md (declarant.street_house) |
| 30 | Seler_Name | BESTWILL SUPPLY CHAIN (NINGBO) CO.,LTD | CD | продавец | copied_from:formalized.invoice_1.Seler_Name (Инвойс HNKY260226 с печатью .md) |
| 31 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца (код) | copied_from:formalized.invoice_1.Seler_PostalAddress_CountryCode (Инвойс HNKY260226 с печатью .md) |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца (текст) | copied_from:formalized.invoice_1.Seler_PostalAddress_CounryName (Инвойс HNKY260226 с печатью .md) |
| 33 | Seler_PostalAddress_Region | Yinzhou district | CD | регион продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_Region (Инвойс HNKY260226 с печатью .md) |
| 34 | Seler_PostalAddress_City | Ningbo | CD | город продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_City (Инвойс HNKY260226 с печатью .md) |
| 35 | Seler_PostalAddress_StreetHouse | TIANTONG SOUTH ROAD | CD | улица/дом продавца | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse (Инвойс HNKY260226 с печатью .md) |
| 36 | Consignor_OrganizationName | BESTWILL SUPPLY CHAIN (NINGBO) CO.,LTD | CD | грузоотправитель | нормализация: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя (код) | нормализация: consignor=seller |
| 38 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя (текст) | нормализация: consignor=seller |
| 39 | Consignor_Address_Region | Yinzhou district | CD | регион грузоотправителя | нормализация: consignor=seller |
| 40 | Consignor_Address_City | Ningbo | CD | город грузоотправителя | нормализация: consignor=seller |
| 41 | Consignor_Address_StreetHouse | TIANTONG SOUTH ROAD | CD | улица/дом грузоотправителя | нормализация: consignor=seller |
| 42 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data:master_proto.md (declarant.organization_name) |
| 43 | Consignee_OGRN | 1201600020390 | CD | ОГРН грузополучателя | master_data:master_proto.md (declarant.ogrn) |
| 44 | Consignee_INN | 1650389298 | CD | ИНН грузополучателя | master_data:master_proto.md (declarant.inn) |
| 45 | Consignee_KPP | 165001001 | CD | КПП грузополучателя | master_data:master_proto.md (declarant.kpp) |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс грузополучателя | master_data:master_proto.md (declarant.postal_code) |
| 47 | Consignee_Address_CountryCode | RU | CD | страна грузополучателя (код) | master_data:master_proto.md (declarant.country_code) |
| 48 | Consignee_Address_CounryName | РОССИЯ | CD | страна грузополучателя (текст) | master_data:master_proto.md (declarant.country_name) |
| 49 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион грузополучателя | master_data:master_proto.md (declarant.region) |
| 50 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город грузополучателя | master_data:master_proto.md (declarant.city) |
| 51 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом грузополучателя | master_data:master_proto.md (declarant.street_house) |
| 52 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 53 | kind_code | 0 | CD | код вида подачи | константа |
| 54 | doc_code | 04021 | CD | код документа | константа |
| 55 | doc_name | ИНВОЙС | CD | наим. документа | константа |
| 56 | doc_number | HNKY260226 | CD | номер для гр. 44 | copied_from:formalized.invoice_1.Registration_PrDocumentNumber |
| 57 | doc_date | 26.02.2026 | CD | дата для гр. 44 | copied_from:formalized.invoice_1.Registration_PrDocumentDate |

- _audit: 57
- `doc_status`: confirmed

#### 3.3 Массив: InvoiceGoods[5]
- _array_audit: 5

#### 3.3 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 8524910056 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.InvoiceGoods[1].GoodsCode (Инвойс HNKY260226 с печатью .md) |
| 02 | GoodsDescription | LCD screen DL0151-RL0 42x31mm Экран LCD DL0151-RL0 42x31мм | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[1].GoodsDescription (Инвойс HNKY260226 с печатью .md) |
| 03 | GoodsQuantity | 5000 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[1].GoodsQuantity (Инвойс HNKY260226 с печатью .md) |
| 04 | goods_supplementary_quantity | 5000 | CD | кол-во в доп. ед. | copied_from:formalized.invoice_1.InvoiceGoods[1].GoodsQuantity (Инвойс HNKY260226 с печатью .md) |
| 05 | goods_supplementary_uom_name | шт | CD | наим. доп. ед. | cb:unit (код 796) |
| 06 | MeasureUnitQualifierName | ШТУКА | CD | ед. изм. доп. кол-ва | cb:unit (код 796) |
| 07 | GrossWeightQuantity | 27.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[1].GrossWeightQuantity (Упаковочный .md) |
| 08 | NetWeightQuantity | 26.45 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[1].NetWeightQuantity (Упаковочный .md) |
| 09 | Price | 2.64 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[1].Price (Инвойс HNKY260226 с печатью .md) |
| 10 | TotalCost | 13200.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[1].TotalCost (Инвойс HNKY260226 с печатью .md) |
| 11 | OriginCountryCode | CN | CD | код страны происх. | copied_from:formalized.invoice_1.InvoiceGoods[1].OriginCountryCode (Инвойс HNKY260226 с печатью .md) |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_Manufacturer (Инвойс HNKY260226 с печатью .md) |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_TradeMark (Инвойс HNKY260226 с печатью .md) |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_GoodsMark (Инвойс HNKY260226 с печатью .md) |
| 15 | AdditionalGoodsDescription_GoodsModel | DL0151-RL0 | CD | модель/модификация | copied_from:formalized.invoice_1.InvoiceGoods[1].AdditionalGoodsDescription_GoodsModel (Инвойс HNKY260226 с печатью .md) |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | группировка по ТН ВЭД (уникальный код 8524910056) |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | первая позиция в группе |

- _item_audit: 17

#### 3.3 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 9025900008 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.InvoiceGoods[2].GoodsCode (Инвойс HNKY260226 с печатью .md) |
| 02 | GoodsDescription | Board with a secsor and wire 300 mm length, batary and probe 35 mm*40 mm / Плата с датчиком 35 мм*40 мм, батареей и проводом длиной 300 мм и щупом Plata s datchikom i provodom dlinoy | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[2].GoodsDescription (Инвойс HNKY260226 с печатью .md) |
| 03 | GoodsQuantity | 5000 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[2].GoodsQuantity (Инвойс HNKY260226 с печатью .md) |
| 04 | goods_supplementary_quantity | | CO | кол-во в доп. ед. | operator:confirmed (not_applicable) |
| 05 | goods_supplementary_uom_name | | CO | наим. доп. ед. | operator:confirmed (not_applicable) |
| 06 | MeasureUnitQualifierName | | CO | ед. изм. доп. кол-ва | operator:confirmed (not_applicable) |
| 07 | GrossWeightQuantity | 38.80 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[2].GrossWeightQuantity (Упаковочный .md) |
| 08 | NetWeightQuantity | 38.30 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[2].NetWeightQuantity (Упаковочный .md) |
| 09 | Price | 6.00 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[2].Price (Инвойс HNKY260226 с печатью .md) |
| 10 | TotalCost | 30000.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[2].TotalCost (Инвойс HNKY260226 с печатью .md) |
| 11 | OriginCountryCode | CN | CD | код страны происх. | copied_from:formalized.invoice_1.InvoiceGoods[2].OriginCountryCode (Инвойс HNKY260226 с печатью .md) |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[2].AdditionalGoodsDescription_Manufacturer (Инвойс HNKY260226 с печатью .md) |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[2].AdditionalGoodsDescription_TradeMark (Инвойс HNKY260226 с печатью .md) |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | copied_from:formalized.invoice_1.InvoiceGoods[2].AdditionalGoodsDescription_GoodsMark (Инвойс HNKY260226 с печатью .md) |
| 15 | AdditionalGoodsDescription_GoodsModel | | CO | модель/модификация | operator:confirmed (оставить пустым) |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | группировка по ТН ВЭД (уникальный код 9025900008) |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | первая позиция в группе |

- _item_audit: 17

#### 3.3 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7326909409 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.InvoiceGoods[3].GoodsCode (Инвойс HNKY260226 с печатью .md) |
| 02 | GoodsDescription | Probe with double-sided tape made of stainless steel 31*13 mm/Зонд с двусторонним скотчем из нержавеющей стали 31*13мм | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[3].GoodsDescription (Инвойс HNKY260226 с печатью .md) |
| 03 | GoodsQuantity | 5000 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[3].GoodsQuantity (Инвойс HNKY260226 с печатью .md) |
| 04 | goods_supplementary_quantity | | CO | кол-во в доп. ед. | operator:confirmed (not_applicable) |
| 05 | goods_supplementary_uom_name | | CO | наим. доп. ед. | operator:confirmed (not_applicable) |
| 06 | MeasureUnitQualifierName | | CO | ед. изм. доп. кол-ва | operator:confirmed (not_applicable) |
| 07 | GrossWeightQuantity | 1.60 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[3].GrossWeightQuantity (Упаковочный .md) |
| 08 | NetWeightQuantity | 1.50 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[3].NetWeightQuantity (Упаковочный .md) |
| 09 | Price | 0.60 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[3].Price (Инвойс HNKY260226 с печатью .md) |
| 10 | TotalCost | 3000.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[3].TotalCost (Инвойс HNKY260226 с печатью .md) |
| 11 | OriginCountryCode | CN | CD | код страны происх. | copied_from:formalized.invoice_1.InvoiceGoods[3].OriginCountryCode (Инвойс HNKY260226 с печатью .md) |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[3].AdditionalGoodsDescription_Manufacturer (Инвойс HNKY260226 с печатью .md) |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[3].AdditionalGoodsDescription_TradeMark (Инвойс HNKY260226 с печатью .md) |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | copied_from:formalized.invoice_1.InvoiceGoods[3].AdditionalGoodsDescription_GoodsMark (Инвойс HNKY260226 с печатью .md) |
| 15 | AdditionalGoodsDescription_GoodsModel | | CO | модель/модификация | operator:confirmed (оставить пустым) |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | группировка по ТН ВЭД (уникальный код 7326909409) |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | первая позиция в группе |

- _item_audit: 17

#### 3.3 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 3926909709 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.InvoiceGoods[4].GoodsCode (Инвойс HNKY260226 с печатью .md) |
| 02 | GoodsDescription | Pink strips of antistatic polyethylene foam 32mm*42mm*2mm/ розовые полоски из антистатического пенополиэтилена 32мм*42мм*2мм | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[4].GoodsDescription (Инвойс HNKY260226 с печатью .md) |
| 03 | GoodsQuantity | 10000 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[4].GoodsQuantity (Инвойс HNKY260226 с печатью .md) |
| 04 | goods_supplementary_quantity | | CO | кол-во в доп. ед. | operator:confirmed (not_applicable) |
| 05 | goods_supplementary_uom_name | | CO | наим. доп. ед. | operator:confirmed (not_applicable) |
| 06 | MeasureUnitQualifierName | | CO | ед. изм. доп. кол-ва | operator:confirmed (not_applicable) |
| 07 | GrossWeightQuantity | 4.60 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[4].GrossWeightQuantity (Упаковочный .md) |
| 08 | NetWeightQuantity | 4.40 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[4].NetWeightQuantity (Упаковочный .md) |
| 09 | Price | 0.20 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[4].Price (Инвойс HNKY260226 с печатью .md) |
| 10 | TotalCost | 2000.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[4].TotalCost (Инвойс HNKY260226 с печатью .md) |
| 11 | OriginCountryCode | CN | CD | код страны происх. | copied_from:formalized.invoice_1.InvoiceGoods[4].OriginCountryCode (Инвойс HNKY260226 с печатью .md) |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[4].AdditionalGoodsDescription_Manufacturer (Инвойс HNKY260226 с печатью .md) |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[4].AdditionalGoodsDescription_TradeMark (Инвойс HNKY260226 с печатью .md) |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | copied_from:formalized.invoice_1.InvoiceGoods[4].AdditionalGoodsDescription_GoodsMark (Инвойс HNKY260226 с печатью .md) |
| 15 | AdditionalGoodsDescription_GoodsModel | | CO | модель/модификация | operator:confirmed (оставить пустым) |
| 16 | dt_item_index | 4 | CD | индекс товара ДТ | группировка по ТН ВЭД (уникальный код 3926909709) |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | первая позиция в группе |

- _item_audit: 17

#### 3.3 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7318149100 | CD | код ТН ВЭД | copied_from:formalized.invoice_1.InvoiceGoods[5].GoodsCode (Инвойс HNKY260226 с печатью .md) |
| 02 | GoodsDescription | Self-tapping screw made of galvanized steel 0,4*1,5 мм / Саморез из оцинкованной стали 0,4*1,5 мм | CD | описание товара | copied_from:formalized.invoice_1.InvoiceGoods[5].GoodsDescription (Инвойс HNKY260226 с печатью .md) |
| 03 | GoodsQuantity | 20000 | CD | кол-во по строке | copied_from:formalized.invoice_1.InvoiceGoods[5].GoodsQuantity (Инвойс HNKY260226 с печатью .md) |
| 04 | goods_supplementary_quantity | | CO | кол-во в доп. ед. | operator:confirmed (not_applicable) |
| 05 | goods_supplementary_uom_name | | CO | наим. доп. ед. | operator:confirmed (not_applicable) |
| 06 | MeasureUnitQualifierName | | CO | ед. изм. доп. кол-ва | operator:confirmed (not_applicable) |
| 07 | GrossWeightQuantity | 6.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[5].GrossWeightQuantity (Упаковочный .md) |
| 08 | NetWeightQuantity | 5.80 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[5].NetWeightQuantity (Упаковочный .md) |
| 09 | Price | 0.07 | CD | цена за единицу | copied_from:formalized.invoice_1.InvoiceGoods[5].Price (Инвойс HNKY260226 с печатью .md) |
| 10 | TotalCost | 1300.00 | CD | стоимость по строке | copied_from:formalized.invoice_1.InvoiceGoods[5].TotalCost (Инвойс HNKY260226 с печатью .md) |
| 11 | OriginCountryCode | CN | CD | код страны происх. | copied_from:formalized.invoice_1.InvoiceGoods[5].OriginCountryCode (Инвойс HNKY260226 с печатью .md) |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | copied_from:formalized.invoice_1.InvoiceGoods[5].AdditionalGoodsDescription_Manufacturer (Инвойс HNKY260226 с печатью .md) |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | copied_from:formalized.invoice_1.InvoiceGoods[5].AdditionalGoodsDescription_TradeMark (Инвойс HNKY260226 с печатью .md) |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | copied_from:formalized.invoice_1.InvoiceGoods[5].AdditionalGoodsDescription_GoodsMark (Инвойс HNKY260226 с печатью .md) |
| 15 | AdditionalGoodsDescription_GoodsModel | | CO | модель/модификация | operator:confirmed (оставить пустым) |
| 16 | dt_item_index | 5 | CD | индекс товара ДТ | группировка по ТН ВЭД (уникальный код 7318149100) |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | первая позиция в группе |

- _item_audit: 17

### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list_1
  - `xml_target_root`: AltaE2PACK
  - `path`: alta\source\ЗапчастиТермометров\md\Упаковочный .md
  - `file_name`: Упаковочный .md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GrossWeightQuantity | 78.00 | CD | общий вес брутто | copied_from:formalized.packing_list_1.GrossWeightQuantity (Упаковочный .md) |
| 02 | NetWeightQuantity | 76.45 | CD | общий вес нетто | copied_from:formalized.packing_list_1.NetWeightQuantity (Упаковочный .md) |
| 03 | Consignor_OrganizationName | BESTWILL SUPPLY CHAIN (NINGBO) CO.,LTD | CD | грузоотправитель | copied_from:formalized.packing_list_1.Consignor_OrganizationName (Упаковочный .md) |
| 04 | Consignor_ShortName | BESTWILL SUPPLY CHAIN (NINGBO) CO.,LTD | CD | краткое наим. отправителя | copied_from:formalized.packing_list_1.Consignor_ShortName (Упаковочный .md) |
| 05 | Consignor_Address_CountryCode | CN | CD | страна отправителя (код) | copied_from:formalized.packing_list_1.Consignor_Address_CountryCode (Упаковочный .md) |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна отправителя (текст) | copied_from:formalized.packing_list_1.Consignor_Address_CounryName (Упаковочный .md) |
| 07 | Consignor_Address_Region | Yinzhou district | CD | регион отправителя | copied_from:formalized.packing_list_1.Consignor_Address_Region (Упаковочный .md) |
| 08 | Consignor_Address_City | Ningbo | CD | город отправителя | copied_from:formalized.packing_list_1.Consignor_Address_City (Упаковочный .md) |
| 09 | Consignor_Address_StreetHouse | TIANTONG SOUTH ROAD | CD | улица/дом отправителя | copied_from:formalized.packing_list_1.Consignor_Address_StreetHouse (Упаковочный .md) |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data:master_proto.md (declarant.organization_name) |
| 11 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наим. получателя | master_data:master_proto.md (declarant.short_name) |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН получателя | master_data:master_proto.md (declarant.ogrn) |
| 13 | Consignee_INN | 1650389298 | CD | ИНН получателя | master_data:master_proto.md (declarant.inn) |
| 14 | Consignee_KPP | 165001001 | CD | КПП получателя | master_data:master_proto.md (declarant.kpp) |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс получателя | master_data:master_proto.md (declarant.postal_code) |
| 16 | Consignee_Address_CountryCode | RU | CD | страна получателя (код) | master_data:master_proto.md (declarant.country_code) |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя (текст) | master_data:master_proto.md (declarant.country_name) |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data:master_proto.md (declarant.region) |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data:master_proto.md (declarant.city) |
| 20 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом получателя | master_data:master_proto.md (declarant.street_house) |
| 21 | DeliveryTerms_DeliveryPlace | Ningbo | CD | место поставки | copied_from:formalized.packing_list_1.DeliveryTerms_DeliveryPlace (Упаковочный .md) |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | | CO | числовой код условий | operator:confirmed |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from:formalized.packing_list_1.DeliveryTerms_DeliveryTermsStringCode (Упаковочный .md) |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наим. контракта | константа |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | HNKY250929 | CD | № контракта | copied_from:formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentNumber (Упаковочный .md) |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 29.09.2025 | CD | дата контракта | copied_from:formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentDate (Упаковочный .md) |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наим. инвойса | константа |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | HNKY260226 | CO | № инвойса | operator_provided_data.md (опечатка в оригинале) |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 26.02.2026 | CO | дата инвойса | operator_provided_data.md (опечатка в оригинале) |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наим. упаковочного | константа |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | HNKY260226-1 | CD | № упаковочного | copied_from:formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentNumber (Упаковочный .md) |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 06.02.2026 | CD | дата упаковочного | copied_from:formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentDate (Упаковочный .md) |
| 33 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 34 | doc_code | 04131 | CD | код документа | константа |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наим. документа | константа |
| 36 | doc_number | HNKY260226 | CO | номер для гр. 44 | operator_provided_data.md |
| 37 | doc_date | 26.02.2026 | CO | дата для гр. 44 | operator_provided_data.md |

- _audit: 37
- `doc_status`: confirmed

#### 3.4 Массив: Goods[5]
- _array_audit: 5

#### 3.4 Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | LCD screen 42x31mm Экран LCD 42x31мм | CD | описание груза | copied_from:formalized.packing_list_1.Goods[1].GoodsDescription (Упаковочный .md) |
| 02 | GoodsQuantity | 2 | CD | кол-во мест | copied_from:formalized.packing_list_1.Goods[1].GoodsQuantity (Упаковочный .md) |
| 03 | GrossWeightQuantity | 27.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[1].GrossWeightQuantity (Упаковочный .md) |
| 04 | NetWeightQuantity | 26.45 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[1].NetWeightQuantity (Упаковочный .md) |
| 05 | PakingQuantity | 5000 | CD | кол-во в упаковке | copied_from:formalized.packing_list_1.Goods[1].PakingQuantity (Упаковочный .md) |

- _item_audit: 5

#### 3.4 Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Board with a secsor and wire 300 mm length, batary and probe 35 mm*40 mm / Плата с датчиком 35 мм*40 мм, батареей и проводом длиной 300 мм и щупом | CD | описание груза | copied_from:formalized.packing_list_1.Goods[2].GoodsDescription (Упаковочный .md) |
| 02 | GoodsQuantity | 5 | CD | кол-во мест | copied_from:formalized.packing_list_1.Goods[2].GoodsQuantity (Упаковочный .md) |
| 03 | GrossWeightQuantity | 38.80 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[2].GrossWeightQuantity (Упаковочный .md) |
| 04 | NetWeightQuantity | 38.30 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[2].NetWeightQuantity (Упаковочный .md) |
| 05 | PakingQuantity | 5000 | CD | кол-во в упаковке | copied_from:formalized.packing_list_1.Goods[2].PakingQuantity (Упаковочный .md) |

- _item_audit: 5

#### 3.4 Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Probe with double-sided tape made of stainless steel 31*13 mm/Зонд с двусторонним скотчем из нержавеющей стали 31*13мм | CD | описание груза | copied_from:formalized.packing_list_1.Goods[3].GoodsDescription (Упаковочный .md) |
| 02 | GoodsQuantity | | CO | кол-во мест | operator:confirmed (оставить пустым) |
| 03 | GrossWeightQuantity | 1.60 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[3].GrossWeightQuantity (Упаковочный .md) |
| 04 | NetWeightQuantity | 1.50 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[3].NetWeightQuantity (Упаковочный .md) |
| 05 | PakingQuantity | 5000 | CD | кол-во в упаковке | copied_from:formalized.packing_list_1.Goods[3].PakingQuantity (Упаковочный .md) |

- _item_audit: 5

#### 3.4 Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Pink strips of antistatic polyethylene foam 32mm*42mm*2mm/ розовые полоски из антистатического пенополиэтилена 32мм*42мм*2мм | CD | описание груза | copied_from:formalized.packing_list_1.Goods[4].GoodsDescription (Упаковочный .md) |
| 02 | GoodsQuantity | 1 | CD | кол-во мест | copied_from:formalized.packing_list_1.Goods[4].GoodsQuantity (Упаковочный .md) |
| 03 | GrossWeightQuantity | 4.60 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[4].GrossWeightQuantity (Упаковочный .md) |
| 04 | NetWeightQuantity | 4.40 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[4].NetWeightQuantity (Упаковочный .md) |
| 05 | PakingQuantity | 10000 | CD | кол-во в упаковке | copied_from:formalized.packing_list_1.Goods[4].PakingQuantity (Упаковочный .md) |

- _item_audit: 5

#### 3.4 Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Self-tapping screw made of galvanized steel 0,004*0,0015 мм / Саморез из оцинкованной стали 0,004*0,0015 мм | CD | описание груза | copied_from:formalized.packing_list_1.Goods[5].GoodsDescription (Упаковочный .md) |
| 02 | GoodsQuantity | | CO | кол-во мест | operator:confirmed (оставить пустым) |
| 03 | GrossWeightQuantity | 6.00 | CD | брутто по строке | copied_from:formalized.packing_list_1.Goods[5].GrossWeightQuantity (Упаковочный .md) |
| 04 | NetWeightQuantity | 5.80 | CD | нетто по строке | copied_from:formalized.packing_list_1.Goods[5].NetWeightQuantity (Упаковочный .md) |
| 05 | PakingQuantity | 20000 | CD | кол-во в упаковке | copied_from:formalized.packing_list_1.Goods[5].PakingQuantity (Упаковочный .md) |

- _item_audit: 5

#### 3.4 Массив: TransportMeans[2]
- _array_audit: 2

#### 3.4 Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | Number | M862EY67 | CD | гос. номер ТС | copied_from:formalized.cmr_1.CMRTransport_PrimeMoverStateSignID (СМР.md) |
| 02 | ModeCode | 31 | CD | код вида транспорта | константа: автосостав |
| 03 | NationalityCode | 000 | CD | код национальности ТС | константа |
| 04 | MoverIndicator | true | CD | признак тягача | константа |

- _item_audit: 4

#### 3.4 Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | Number | AM710667 | CD | гос. номер ТС | copied_from:formalized.cmr_1.CMRTransport_TrailerStateSignID (СМР.md) |
| 02 | ModeCode | 31 | CD | код вида транспорта | константа: автосостав |
| 03 | NationalityCode | 000 | CD | код национальности ТС | константа |
| 04 | MoverIndicator | false | CD | признак тягача | константа |

- _item_audit: 4

### `document`: CMR
  - `uqi_prefix`: formalized.cmr_1
  - `xml_target_root`: AltaE3CMR
  - `path`: alta\source\ЗапчастиТермометров\md\СМР.md
  - `file_name`: СМР.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | LanguageCode | RU | CD | язык документа | константа |
| 02 | CMR_Choice | 1 | CD | системный выбор | константа |
| 03 | RegistrationDocument_RegID | 09886 | CD | номер CMR | copied_from:formalized.cmr_1.RegistrationDocument_RegID (СМР.md) |
| 04 | RegistrationDocument_DateInf | 13.05.2026 | CD | дата CMR | copied_from:formalized.cmr_1.RegistrationDocument_DateInf (СМР.md) |
| 05 | RegistrationDocument_Place | Маньчжурия | CD | место составления | copied_from:formalized.cmr_1.RegistrationDocument_Place (СМР.md) |
| 06 | TrakingCargo_TakingCargoDate | 13.05.2026 | CD | дата принятия груза | copied_from:formalized.cmr_1.TrakingCargo_TakingCargoDate (СМР.md) |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CD | страна принятия (код) | copied_from:formalized.cmr_1.TrakingCargo_TakingCargoPlace_CountryCode (СМР.md) |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия (текст) | copied_from:formalized.cmr_1.TrakingCargo_TakingCargoPlace_CounryName (СМР.md) |
| 09 | DeliveryPlace_CountryCode | RU | CD | страна доставки (код) | copied_from:formalized.cmr_1.DeliveryPlace_CountryCode (СМР.md) |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки (текст) | copied_from:formalized.cmr_1.DeliveryPlace_CounryName (СМР.md) |
| 11 | DeliveryTerms_DeliveryPlace | | CO | место поставки | operator:confirmed (not_present) |
| 12 | DeliveryTerms_DeliveryTermsStringCode | | CO | условия поставки | operator:confirmed (not_present) |
| 13 | GoodsQuantity | 8 | CD | общее кол-во мест | copied_from:formalized.cmr_1.GoodsQuantity (СМР.md) |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 78.00 | CD | общий вес брутто | copied_from:formalized.cmr_1.CMRGoodsWeight_GrossWeightQuantity (СМР.md) |
| 15 | CMRTransport_PrimeMoverStateSignID | M862EY67 | CD | гос. номер тягача | copied_from:formalized.cmr_1.CMRTransport_PrimeMoverStateSignID (СМР.md) |
| 16 | CMRTransport_TrailerStateSignID | AM710667 | CD | гос. номер прицепа | copied_from:formalized.cmr_1.CMRTransport_TrailerStateSignID (СМР.md) |
| 17 | Consignor_NameInf | BESTWILL SUPPLY CHAIN (NINGBO) CO.,LTD | CD | отправитель | copied_from:formalized.cmr_1.Consignor_NameInf (СМР.md) |
| 18 | Consignor_ShortName | BESTWILL SUPPLY CHAIN (NINGBO) CO.,LTD | CD | краткое наим. отправителя | copied_from:formalized.cmr_1.Consignor_ShortName (СМР.md) |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна отправителя (код) | copied_from:formalized.cmr_1.Consignor_PostalAddress_CountryCode (СМР.md) |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна отправителя (текст) | copied_from:formalized.cmr_1.Consignor_Address_CounryName (СМР.md) |
| 21 | Consignor_Address_Region | Yinzhou district | CD | регион отправителя | copied_from:formalized.cmr_1.Consignor_Address_Region (СМР.md) |
| 22 | Consignor_Address_City | Ningbo | CD | город отправителя | copied_from:formalized.cmr_1.Consignor_Address_City (СМР.md) |
| 23 | Consignor_Address_StreetHouse | TIANTONG SOUTH ROAD | CD | улица/дом отправителя | copied_from:formalized.cmr_1.Consignor_Address_StreetHouse (СМР.md) |
| 24 | Consignor_Guarantee_OrganizationName | | CO | наим. гаранта | operator:confirmed (not_present) |
| 25 | Consignor_Guarantee_ShortName | | CO | краткое наим. гаранта | operator:confirmed (not_present) |
| 26 | Consignor_Guarantee_Address_CountryCode | | CO | страна гаранта (код) | operator:confirmed (not_present) |
| 27 | Consignor_Guarantee_Address_CounryName | | CO | страна гаранта (текст) | operator:confirmed (not_present) |
| 28 | Consignor_Guarantee_Address_Region | | CO | регион гаранта | operator:confirmed (not_present) |
| 29 | Consignor_Guarantee_Address_City | | CO | город гаранта | operator:confirmed (not_present) |
| 30 | Consignor_Guarantee_Address_StreetHouse | | CO | улица/дом гаранта | operator:confirmed (not_present) |
| 31 | Consignee_NameInf | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | получатель | master_data:master_proto.md (declarant.organization_name) |
| 32 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наим. получателя | master_data:master_proto.md (declarant.short_name) |
| 33 | Consignee_OGRNID | 1201600020390 | CD | ОГРН получателя | master_data:master_proto.md (declarant.ogrn) |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН получателя | master_data:master_proto.md (declarant.inn) |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП получателя | master_data:master_proto.md (declarant.kpp) |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс получателя | master_data:master_proto.md (declarant.postal_code) |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна получателя (код) | master_data:master_proto.md (declarant.country_code) |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя (текст) | master_data:master_proto.md (declarant.country_name) |
| 39 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data:master_proto.md (declarant.region) |
| 40 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data:master_proto.md (declarant.city) |
| 41 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом получателя | master_data:master_proto.md (declarant.street_house) |
| 42 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 43 | doc_code | 02015 | CD | код документа | константа |
| 44 | doc_name | CMR | CD | наим. документа | константа |
| 45 | doc_number | 09886 | CD | номер для гр. 44 | copied_from:formalized.cmr_1.RegistrationDocument_RegID |
| 46 | doc_date | 13.05.2026 | CD | дата для гр. 44 | copied_from:formalized.cmr_1.RegistrationDocument_DateInf |

- _audit: 46
- `doc_status`: confirmed

#### 3.5 Массив: CMRGoods[1]
- _array_audit: 1

#### 3.5 Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsNumeric | 1 | CD | порядковый номер | авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № HNKY260226 from 26.02.2026 | CD | описание груза | copied_from:formalized.cmr_1.GoodsDescription (СМР.md) |
| 03 | PakingQuantity | 8 | CD | кол-во упаковок | copied_from:formalized.cmr_1.Goods[1].PakingQuantity (СМР.md) |

- _item_audit: 3

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\ЗапчастиТермометров\md\mt103_2.md
  - `file_name`: mt103_2.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CD | код вида документа | константа |
| 02 | PaymentModeCode | | CO | код способа платежа | operator:confirmed (not_present) |
| 03 | PaymentAmount | 15000.00 | CD | сумма платежа | copied_from:formalized.payment_order_1.PaymentAmount (mt103_2.md) |
| 04 | TransactionKind | 01 | CD | вид операции | константа |
| 05 | Priority | . | CD | очередность | константа |
| 06 | Purpose | PAYMENT FOR THE CONTRACT NO.: HNKY250929, DATE: SEPTEMBER 29, 2025, INVOICE NO: HNKY260226 DATE: FEBRUARY, 26, 2026 | CD | назначение платежа | copied_from:formalized.payment_order_1.Purpose (mt103_2.md) |
| 07 | ValueSpelledOut | Пятнадцать тысяч юаней 00/100 | CD | сумма прописью | copied_from:formalized.payment_order_1.ValueSpelledOut (mt103_2.md) |
| 08 | DocumentReference_PrDocumentNumber | 2 | CD | номер платежного поручения | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber (mt103_2.md) |
| 09 | DocumentReference_PrDocumentDate | 27.02.2026 | CD | дата платежного поручения | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate (mt103_2.md) |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | copied_from:formalized.payment_order_1.Payer_OrganizationName (mt103_2.md) |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from:formalized.payment_order_1.Payer_INN (mt103_2.md) |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | master_data:master_proto.md (declarant.kpp) |
| 13 | Payer_Bank_BankName | ВТБ, Сч. № 40702156216150000051 | CD | реквизиты банка плательщика | copied_from:formalized.payment_order_1.Payer_Bank_BankName (mt103_2.md) |
| 14 | Payee_OrganizationName | BESTWILL SUPPLY CHAIN (NINGBO) CO.,LTD, NO.666 TIANTONG SOUTH ROAD, YINZHOU DISTRICT NINGBO, CHINA 315100 NINGBO, CN | CD | получатель платежа | copied_from:formalized.payment_order_1.Payee_OrganizationName (mt103_2.md) |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SWIFT: VTBRCNSHXXX, Account: 40807156600610035383 | CD | реквизиты банка получателя | copied_from:formalized.payment_order_1.Payee_Bank_BankName (mt103_2.md) |
| 16 | PersonSurname | САРАНОВ | CD | фамилия подписанта | copied_from:formalized.payment_order_1.PersonSurname (mt103_2.md) |
| 17 | PersonName | ДМИТРИЙ | CD | имя подписанта | copied_from:formalized.payment_order_1.PersonName (mt103_2.md) |
| 18 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 19 | doc_code | 04023 | CD | код документа | константа |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наим. документа | константа |
| 21 | doc_number | 2 | CD | номер для гр. 44 | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber |
| 22 | doc_date | 27.02.2026 | CD | дата для гр. 44 | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate |

- _audit: 22
- `doc_status`: confirmed

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice_1
  - `xml_target_root`: AltaServiceInvoice
  - `path`: alta\source\ЗапчастиТермометров\md\Счет_№26-09886-tl_от_12-05-2026.md
  - `file_name`: Счет_№26-09886-tl_от_12-05-2026.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentSign | 1 | CD | признак документа | константа |
| 02 | TotalServiceCost | 485.00 | CD | итого по услугам | copied_from:formalized.service_invoice_1.TotalServiceCost (Счет_№26-09886-tl_от_12-05-2026.md) |
| 03 | Currency | USD | CD | валюта итого | copied_from:formalized.service_invoice_1.Currency (Счет_№26-09886-tl_от_12-05-2026.md) |
| 04 | ServiceProvider_Name | ООО "Трансимпериал" | CD | исполнитель услуг | copied_from:formalized.service_invoice_1.ServiceProvider_Name (Счет_№26-09886-tl_от_12-05-2026.md) |
| 05 | BankName | АО "Альфа-Банк", БИК 044525593, Сч. № 407021810001600010931, Корр. Сч. 30101810200000000593 | CD | банк исполнителя | copied_from:formalized.service_invoice_1.BankName (Счет_№26-09886-tl_от_12-05-2026.md) |
| 06 | ContractDetails_PrDocumentNumber | КОО0/26651/М | CD | № договора на услуги | copied_from:formalized.service_invoice_1.ContractDetails_PrDocumentNumber (Счет_№26-09886-tl_от_12-05-2026.md) |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги | copied_from:formalized.service_invoice_1.ContractDetails_PrDocumentDate (Счет_№26-09886-tl_от_12-05-2026.md) |
| 08 | PrDocumentNumber | 26-09886-tl | CD | номер связанного заказа | copied_from:formalized.service_invoice_1.PaymentDocument_PrDocumentNumber (Счет_№26-09886-tl_от_12-05-2026.md) |
| 09 | PrDocumentDate | 13.04.2026 | CD | дата связанного заказа | copied_from:formalized.service_invoice_1.PaymentDocument_PrDocumentDate (Счет_№26-09886-tl_от_12-05-2026.md) |
| 10 | Registration_PrDocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование счета | copied_from:formalized.service_invoice_1.Registration_PrDocumentName (Счет_№26-09886-tl_от_12-05-2026.md) |
| 11 | Registration_PrDocumentNumber | 26-09886-tl | CD | номер счета | copied_from:formalized.service_invoice_1.Registration_PrDocumentNumber (Счет_№26-09886-tl_от_12-05-2026.md) |
| 12 | Registration_PrDocumentDate | 12.05.2026 | CD | дата счета | copied_from:formalized.service_invoice_1.Registration_PrDocumentDate (Счет_№26-09886-tl_от_12-05-2026.md) |
| 13 | Consignor_OrganizationName | | CO | грузоотправитель | operator:confirmed (not_present) |
| 14 | PostalCode | | CO | индекс отправителя | operator:confirmed (not_present) |
| 15 | CountryCode | | CO | страна отправителя (код) | operator:confirmed (not_present) |
| 16 | CounryName | | CO | страна отправителя (текст) | operator:confirmed (not_present) |
| 17 | Region | | CO | регион отправителя | operator:confirmed (not_present) |
| 18 | Town | | CO | город отправителя | operator:confirmed (not_present) |
| 19 | StreetHouse | | CO | улица/дом отправителя | operator:confirmed (not_present) |
| 20 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data:master_proto.md (declarant.organization_name) |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CD | ОГРН получателя | master_data:master_proto.md (declarant.ogrn) |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН получателя | master_data:master_proto.md (declarant.inn) |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП получателя | master_data:master_proto.md (declarant.kpp) |
| 24 | PostalCode | 423800 | CD | индекс получателя | master_data:master_proto.md (declarant.postal_code) |
| 25 | CountryCode | RU | CD | страна получателя (код) | master_data:master_proto.md (declarant.country_code) |
| 26 | CounryName | РОССИЯ | CD | страна получателя (текст) | master_data:master_proto.md (declarant.country_name) |
| 27 | Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data:master_proto.md (declarant.region) |
| 28 | Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data:master_proto.md (declarant.city) |
| 29 | StreetHouse | ПРОЕЗД ХЛЕБНЫЙ | CD | улица получателя | master_data:master_proto.md (declarant.street_house) |
| 30 | House | Д. 30 | CD | дом получателя | master_data:master_proto.md (declarant.street_house) |
| 31 | Room | ОФИС 211 | CD | офис получателя | master_data:master_proto.md (declarant.street_house) |
| 32 | Signature_Choice | 2 | CD | вариант подписи | константа |
| 33 | IndividualEntrepreneur_PersonSurname | | CO | фамилия ИП | operator:confirmed (not_applicable) |
| 34 | IndividualEntrepreneur_PersonName | | CO | имя ИП | operator:confirmed (not_applicable) |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CO | отчество ИП | operator:confirmed (not_applicable) |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | КЛИМОВИЧ | CD | фамилия руководителя | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonSurname (Счет_№26-09886-tl_от_12-05-2026.md) |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л. | CD | имя руководителя | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonName (Счет_№26-09886-tl_от_12-05-2026.md) |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А. | CD | отчество руководителя | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonMiddleName (Счет_№26-09886-tl_от_12-05-2026.md) |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | ЛЕХНО | CD | фамилия бухгалтера | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname (Счет_№26-09886-tl_от_12-05-2026.md) |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О. | CD | имя бухгалтера | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonName (Счет_№26-09886-tl_от_12-05-2026.md) |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А. | CD | отчество бухгалтера | copied_from:formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName (Счет_№26-09886-tl_от_12-05-2026.md) |
| 42 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 43 | doc_code | 04031 | CD | код документа | константа |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наим. документа | константа |
| 45 | doc_number | 26-09886-tl | CD | номер для гр. 44 | copied_from:formalized.service_invoice_1.Registration_PrDocumentNumber |
| 46 | doc_date | 12.05.2026 | CD | дата для гр. 44 | copied_from:formalized.service_invoice_1.Registration_PrDocumentDate |
| 47 | transport_to_border | 252.00 | CD | стоимость до границы | copied_from:formalized.service_invoice_1.transport_to_border (Счет_№26-09886-tl_от_12-05-2026.md) |
| 48 | transport_currency | USD | CD | валюта стоимости | copied_from:formalized.service_invoice_1.transport_currency (Счет_№26-09886-tl_от_12-05-2026.md) |

- _audit: 48
- `doc_status`: confirmed

#### 3.7 Массив: ServiceDescription[2]
- _array_audit: 2

#### 3.7 Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КОО0/26651/М от 13-05-2025 по транспортному заказу № 26-09886-tl от 13.04.2026 по маршруту: China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | copied_from:formalized.service_invoice_1.ServiceDescription[1].GoodsDescription (Счет_№26-09886-tl_от_12-05-2026.md) |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:formalized.service_invoice_1.ServiceDescription[1].CurrencyCode (Счет_№26-09886-tl_от_12-05-2026.md) |
| 03 | ServiceName | China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) | CD | наименование/маршрут | copied_from:formalized.service_invoice_1.ServiceDescription[1].ServiceName (Счет_№26-09886-tl_от_12-05-2026.md) |
| 04 | TaxRate | НДС 0% | CD | ставка налога | copied_from:formalized.service_invoice_1.ServiceDescription[1].TaxRate (Счет_№26-09886-tl_от_12-05-2026.md) |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:formalized.service_invoice_1.ServiceDescription[1].TaxSum (Счет_№26-09886-tl_от_12-05-2026.md) |
| 06 | ServiceCost_Amount | 252.00 | CD | стоимость строки | copied_from:formalized.service_invoice_1.ServiceDescription[1].ServiceCost_Amount (Счет_№26-09886-tl_от_12-05-2026.md) |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | copied_from:formalized.service_invoice_1.ServiceDescription[1].ServiceCost_Currency (Счет_№26-09886-tl_от_12-05-2026.md) |

- _item_audit: 7

#### 3.7 Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, г. Набережные Челны | CD | описание услуги | copied_from:formalized.service_invoice_1.ServiceDescription[2].GoodsDescription (Счет_№26-09886-tl_от_12-05-2026.md) |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:formalized.service_invoice_1.ServiceDescription[2].CurrencyCode (Счет_№26-09886-tl_от_12-05-2026.md) |
| 03 | ServiceName | граница РФ (п/п Маньчжурия/Забайкальск) - Россия, г. Набережные Челны | CD | наименование/маршрут | copied_from:formalized.service_invoice_1.ServiceDescription[2].ServiceName (Счет_№26-09886-tl_от_12-05-2026.md) |
| 04 | TaxRate | НДС 0% | CD | ставка налога | copied_from:formalized.service_invoice_1.ServiceDescription[2].TaxRate (Счет_№26-09886-tl_от_12-05-2026.md) |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:formalized.service_invoice_1.ServiceDescription[2].TaxSum (Счет_№26-09886-tl_от_12-05-2026.md) |
| 06 | ServiceCost_Amount | 233.00 | CD | стоимость строки | copied_from:formalized.service_invoice_1.ServiceDescription[2].ServiceCost_Amount (Счет_№26-09886-tl_от_12-05-2026.md) |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | copied_from:formalized.service_invoice_1.ServiceDescription[2].ServiceCost_Currency (Счет_№26-09886-tl_от_12-05-2026.md) |

- _item_audit: 7

### `document`: Insurance Services Invoice
  - `uqi_prefix`: formalized.insurance_document_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\ЗапчастиТермометров\md\Счет_№26-09886-tl_1_от_05-05-2026.md
  - `file_name`: Счет_№26-09886-tl_1_от_05-05-2026.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04111 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование документа | copied_from:formalized.insurance_document_1.DocumentHead_DocumentName (Счет_№26-09886-tl_1_от_05-05-2026.md) |
| 03 | DocumentHead_DocumentDate | 05.05.2026 | CD | дата документа | copied_from:formalized.insurance_document_1.DocumentHead_DocumentDate (Счет_№26-09886-tl_1_от_05-05-2026.md) |
| 04 | DocumentHead_DocumentNumber | 26-09886-tl/1 | CD | номер документа | copied_from:formalized.insurance_document_1.DocumentHead_DocumentNumber (Счет_№26-09886-tl_1_от_05-05-2026.md) |
| 05 | TextPara | link:alta\source\ЗапчастиТермометров\md\Счет_№26-09886-tl_1_от_05-05-2026.md | CD | основной текст | link на файл-источник |
| 06 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 07 | doc_code | 04111 | CD | код документа | константа |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наим. документа | константа |
| 09 | doc_number | 26-09886-tl/1 | CD | номер для гр. 44 | copied_from:formalized.insurance_document_1.DocumentHead_DocumentNumber |
| 10 | doc_date | 05.05.2026 | CD | дата для гр. 44 | copied_from:formalized.insurance_document_1.DocumentHead_DocumentDate |
| 11 | insurance_to_border | 453.40 | CD | стоимость страхования | copied_from:formalized.insurance_document_1.insurance_to_border (Счет_№26-09886-tl_1_от_05-05-2026.md) |
| 12 | insurance_currency | RUB | CD | валюта страхования | copied_from:formalized.insurance_document_1.insurance_currency (Счет_№26-09886-tl_1_от_05-05-2026.md) |

- _audit: 12
- `doc_status`: confirmed

### `document`: Tech Description
  - `uqi_prefix`: formalized.tech_description_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\ЗапчастиТермометров\md\тех описание .md
  - `file_name`: тех описание .md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКИЕ ХАРАКТЕРИСТИКИ | CD | наименование техописания | copied_from:formalized.tech_description_1.DocumentHead_DocumentName (тех описание .md) |
| 03 | DocumentHead_DocumentDate | 26.02.2026 | CO | дата техописания | operator_provided_data.md (привязка к инвойсу) |
| 04 | DocumentHead_DocumentNumber | БН | CO | номер техописания | operator_provided_data.md |
| 05 | TextPara | link:alta\source\ЗапчастиТермометров\md\тех описание .md | CD | технический текст | link на файл-источник |
| 06 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 07 | doc_code | 05999 | CD | код документа | константа |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наим. документа | константа |
| 09 | doc_number | БН | CO | номер для гр. 44 | operator_provided_data.md |
| 10 | doc_date | 26.02.2026 | CO | дата для гр. 44 | operator_provided_data.md |

- _audit: 10
- `doc_status`: confirmed

### `document`: Contract
  - `uqi_prefix`: master_data.contract
  - `path`: alta\master_data\master_proto.md
  - `file_name`: master_proto.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 02 | doc_code | 03011 | CD | код документа | константа |
| 03 | doc_name | КОНТРАКТ | CD | наим. документа | константа |
| 04 | doc_number | HNKY250929 | CD | номер для гр. 44 | master_data:master_proto.md (contract.doc_number) |
| 05 | doc_date | 29.09.2025 | CD | дата для гр. 44 | master_data:master_proto.md (contract.doc_date) |

- _audit: 5
- `doc_status`: confirmed

### `document`: Supplementary Contract
  - `uqi_prefix`: master_data.supplementary_contract_1
  - `path`: alta\master_data\master_proto.md
  - `file_name`: master_proto.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 02 | doc_code | 03012 | CD | код документа | константа |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наим. документа | константа |
| 04 | doc_number | 1 | CD | номер для гр. 44 | master_data:master_proto.md (supplementary_contract_1.doc_number) |
| 05 | doc_date | 26.02.2026 | CD | дата для гр. 44 | master_data:master_proto.md (supplementary_contract_1.doc_date) |

- _audit: 5
- `doc_status`: confirmed

## 3. master_data:

### `document`: EGRUL
  - `uqi_prefix`: master_data.egrul_1
  - `path`: alta\master_data\master_proto.md
  - `file_name`: master_proto.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование организации | master_data:master_proto.md (declarant.organization_name) |
| 02 | ShortName | ООО "СКИФ" | CD | краткое наименование | master_data:master_proto.md (declarant.short_name) |
| 03 | OGRN | 1201600020390 | CD | ОГРН | master_data:master_proto.md (declarant.ogrn) |
| 04 | INN | 1650389298 | CD | ИНН | master_data:master_proto.md (declarant.inn) |
| 05 | KPP | 165001001 | CD | КПП | master_data:master_proto.md (declarant.kpp) |
| 06 | Address_PostalCode | 423800 | CD | индекс | master_data:master_proto.md (declarant.postal_code) |
| 07 | Address_CountryCode | RU | CD | страна (код) | master_data:master_proto.md (declarant.country_code) |
| 08 | Address_CounryName | РОССИЯ | CD | страна (текст) | master_data:master_proto.md (declarant.country_name) |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data:master_proto.md (declarant.region) |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data:master_proto.md (declarant.city) |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом | master_data:master_proto.md (declarant.street_house) |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | master_data:master_proto.md (declarant.phone) |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | master_data:master_proto.md (declarant.email) |
| 14 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 15 | doc_code | 04011 | CD | код документа | константа |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наим. документа | константа |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер для гр. 44 | master_data:master_proto.md (egrul.doc_number) |
| 18 | doc_date | 14.07.2025 | CD | дата для гр. 44 | master_data:master_proto.md (egrul.doc_date) |

- _audit: 18
- `doc_status`: confirmed

### `document`: Personal Passport
  - `uqi_prefix`: master_data.passport_1
  - `path`: alta\master_data\master_proto.md
  - `file_name`: master_proto.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | master_data:master_proto.md (representative.surname) |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | master_data:master_proto.md (representative.name) |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | master_data:master_proto.md (representative.middle_name) |
| 04 | CardSeries | 63 09 | CD | серия паспорта | master_data:master_proto.md (representative.passport_series) |
| 05 | CardNumber | 449948 | CD | номер паспорта | master_data:master_proto.md (representative.passport_number) |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | master_data:master_proto.md (representative.passport_date) |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data:master_proto.md (representative.passport_org) |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_data:master_proto.md (representative.phone) |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | master_data:master_proto.md (representative.email) |
| 10 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 11 | doc_code | 11001 | CD | код документа | константа |
| 12 | doc_name | ПАСПОРТ | CD | наим. документа | константа |
| 13 | doc_number | 63 09 449948 | CD | номер для гр. 44 | master_data:master_proto.md (passport.doc_number) |
| 14 | doc_date | 11.03.2010 | CD | дата для гр. 44 | master_data:master_proto.md (passport.doc_date) |

- _audit: 14
- `doc_status`: confirmed

### `document`: Letter of Attorney
  - `uqi_prefix`: master_data.letter_of_attorney_1
  - `path`: alta\master_data\master_proto.md
  - `file_name`: master_proto.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data:master_proto.md (letter_of_attorney.doc_number) |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_data:master_proto.md (letter_of_attorney.doc_date) |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_data:master_proto.md (letter_of_attorney.end_date) |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_data:master_proto.md (letter_of_attorney.empowered_post) |
| 05 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 06 | doc_code | 11004 | CD | код документа | константа |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наим. документа | константа |
| 08 | doc_number | 1 | CD | номер для гр. 44 | master_data:master_proto.md (letter_of_attorney.doc_number) |
| 09 | doc_date | 01.02.2026 | CD | дата для гр. 44 | master_data:master_proto.md (letter_of_attorney.doc_date) |

- _audit: 9
- `doc_status`: confirmed

### `document`: Transport Contract
  - `uqi_prefix`: master_data.transport_contract_1
  - `path`: alta\master_data\master_proto.md
  - `file_name`: master_proto.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 02 | doc_code | 04033 | CD | код документа | константа |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наим. документа | константа |
| 04 | doc_number | КООО/26651/М | CD | номер для гр. 44 | master_data:master_proto.md (transport_contract.doc_number) |
| 05 | doc_date | 13.05.2025 | CD | дата для гр. 44 | master_data:master_proto.md (transport_contract.doc_date) |

- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter
  - `uqi_prefix`: master_data.exemption_letter_1
  - `path`: alta\master_data\master_proto.md
  - `file_name`: master_proto.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 02 | doc_code | 09023 | CD | код документа | константа |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наим. документа | константа |
| 04 | doc_number | 24968/МЛ10 | CD | номер для гр. 44 | master_data:master_proto.md (exemption_letter.doc_number) |
| 05 | doc_date | 20.08.2025 | CD | дата для гр. 44 | master_data:master_proto.md (exemption_letter.doc_date) |

- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter (source)
  - `uqi_prefix`: master_data.exemption_letter_source_1
  - `path`: alta\master_data\master_proto.md
  - `file_name`: master_proto.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак гр. 44 | константа |
| 02 | doc_code | 09999 | CD | код документа | константа |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наим. документа | константа |
| 04 | doc_number | 24968/МЛ10 | CD | номер для гр. 44 | master_data:master_proto.md (exemption_letter_source.doc_number) |
| 05 | doc_date | 20.08.2025 | CD | дата для гр. 44 | master_data:master_proto.md (exemption_letter_source.doc_date) |

- _audit: 5
- `doc_status`: confirmed

## 4. non_formalized:
(не применимо, документы отсутствуют в поставке)

### Итогo, по файлу:

`total_unreliable_fields`: 0
`formalization_status`: confirmed

## 5. Нерешенные вопросы (Issues)

**Для полей:**
- Нет.

**Для общих вопросов:**
- Нет.

## 6. `unreliable_fields`:
- Нет
