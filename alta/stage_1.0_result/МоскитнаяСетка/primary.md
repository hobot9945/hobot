## meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 7 товаров
- `источники данных`: md + operator_provided_data + stable_source (xml)

## formalized:

### `document`: Contract
  - `uqi_prefix`: formalized.contract_1
  - `xml_target_root`: AltaE2CONT
  - `path`: md\SALES CONTRACT NoLM-2553.md
  - `file_name`: SALES CONTRACT NoLM-2553.pdf

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 03011 | CD | код вида документа для графы 44 | derived |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта; графа 44: G44/G442 | |
| 03 | ContractRegistration_PrDocumentDate | 02.07.2025 | CD | дата контракта; графа 44: G44/G443 | |
| 04 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта | operator:decisions_from_chat |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator |
| 06 | ContractTerms_LastDate | 31.12.2026 | CD | срок действия/исполнения | |
| 07 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки / Incoterms | operator |
| 08 | ContractTerms_ContractText | link:md\SALES CONTRACT NoLM-2553.md | CD | текст контракта | |
| 09 | ContractTerms_DealSign | 1 | CO | системный признак Альты | operator |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта | |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 | operator |
| 12 | ForeignPerson_Address_CounryName | Китай | CD | страна продавца, текст | |
| 13 | ForeignPerson_Address_Region | HEBEI | CD | регион/область продавца | |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца | |
| 15 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District | CD | улица/дом продавца одной строкой | |
| 16 | RussianPerson_OrganizationName | ООО «СКИФ» | CD | покупатель/сторона контракта | |
| 17 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН покупателя | operator:allow_cross_doc_master_data |
| 18 | RussianPerson_INN | 1650389298 | CO | ИНН покупателя | operator:allow_cross_doc_master_data |
| 19 | RussianPerson_KPP | 165001001 | CO | КПП покупателя | operator:allow_cross_doc_master_data |
| 20 | RussianPerson_Address_PostalCode | 423800 | CO | индекс покупателя | operator:allow_cross_doc_master_data |
| 21 | RussianPerson_Address_CountryCode | RU | CO | страна покупателя alpha-2 | operator:allow_cross_doc_master_data |
| 22 | RussianPerson_Address_CounryName | РОССИЯ | CO | страна покупателя, текст | operator:allow_cross_doc_master_data |
| 23 | RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион покупателя | operator:allow_cross_doc_master_data |
| 24 | RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город покупателя | operator:allow_cross_doc_master_data |
| 25 | RussianPerson_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | CO | улица/дом/офис одной строкой | operator:allow_cross_doc_master_data |

#### Итого, по документу:
- `doc_fields`: 25 из 25
- `doc_formalization_status`: confirmed

### `document`: Supplementary Contract
  - `uqi_prefix`: formalized.supplementary_contract_1
  - `xml_target_root`: AltaSupplementaryContract
  - `path`: md\1 Supplementary agreement to the contract.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения | |
| 02 | IssueDate | 25.11.2025 | CD | дата доп. соглашения | |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта | |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator |
| 05 | ContractDescription_LastDate | 31.12.2026 | CO | новый срок действия/исполнения | operator |
| 06 | ContractDescription_ContractText | link:md\1 Supplementary agreement to the contract.md | CD | текст доп. соглашения | |
| 07 | ContractDescription_DealSign | 1 | CO | системный признак Альты | operator |
| 08 | ContractDescription_StockCategorySign | 0 | CO | системный признак Альты | operator |
| 09 | ContractDescription_BuyerLimitationSign | 0 | CO | системный признак Альты | operator |
| 10 | ContractDescription_InsuranceSign | 0 | CO | системный признак Альты | operator |
| 11 | RussianPerson_OrganizationName | ООО «СКИФ» | CD | российская сторона; покупатель | |
| 12 | RussianPerson_ShortName | ООО «СКИФ» | CD | краткое наименование | |
| 13 | RussianPerson_OGRN | 1201600020390 | CD | ОГРН | |
| 14 | RussianPerson_INN | 1650389298 | CD | ИНН | |
| 15 | RussianPerson_KPP | 165001001 | CD | КПП | |
| 16 | RussianPerson_Address_PostalCode | 423800 | CD | индекс | |
| 17 | RussianPerson_Address_CountryCode | RU | CD | страна alpha-2 | |
| 18 | RussianPerson_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 19 | RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 20 | RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 21 | RussianPerson_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | CD | улица/дом одной строкой | |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | иностранная сторона; продавец | |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator |
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 | operator |
| 25 | ForeignPerson_Address_CounryName | КИТАЙ | CO | страна, текст | operator:supplementary_contract_1.foreign_person_address_from_contract |
| 26 | ForeignPerson_Address_Region | HEBEI | CO | регион | operator:supplementary_contract_1.foreign_person_address_from_contract |
| 27 | ForeignPerson_Address_City | Shijiazhuang | CO | город/район | operator:supplementary_contract_1.foreign_person_address_from_contract |
| 28 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street | CO | улица/дом одной строкой | operator:supplementary_contract_1.foreign_person_address_from_contract |
| 29 | PersonSurname | Li | CO | фамилия подписанта | operator |
| 30 | PersonName | Jing | CO | имя подписанта | operator |
| 31 | PersonMiddleName | | CO | отчество подписанта | operator |

#### Итого, по документу:
- `doc_fields`: 31 из 31
- `doc_formalization_status`: confirmed

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: md\CL на сетку .md
  - `file_name`: CL на сетку .pdf

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты | operator |
| 02 | CurrencyCode | CNY | CO | валюта инвойса ISO 4217 alpha-3 | operator |
| 03 | DocumentCode | 04021 | CD | код вида документа для графы 44 | derived |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест по инвойсу | |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто по инвойсу | operator |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто по инвойсу | operator |
| 08 | GCost | 97260.00 | CO | системное поле Альты | operator |
| 09 | TotalCost | 97260.00 | CO | итого по инвойсу | operator |
| 10 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CD | место поставки по Incoterms | |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | operator |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | operator |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | operator |
| 16 | Registration_PrDocumentName | Commercial invoice | CD | наименование документа | |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | |
| 23 | Buyer_Name | ООО «СКИФ» | CD | наименование покупателя | |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CO | индекс покупателя | operator |
| 25 | Buyer_PostalAddress_CountryCode | RU | CO | страна покупателя alpha-2 | operator |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CO | страна покупателя, текст | operator |
| 27 | Buyer_PostalAddress_Region | Республика Татарстан | CO | регион | operator |
| 28 | Buyer_PostalAddress_City | Набережные Челны | CO | город | operator |
| 29 | Buyer_PostalAddress_StreetHouse | проезд Хлебный, дом 30, офис 211 | CO | улица/дом/офис одной строкой | operator |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец | |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | operator |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | |
| 33 | Seler_PostalAddress_Region | HEBEI | CD | регион продавца | |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город/район продавца | |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | грузоотправитель | operator |
| 37 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | operator |
| 38 | Consignor_Address_CounryName | КИТАЙ | CO | страна грузоотправителя, текст | operator |
| 39 | Consignor_Address_Region | HEBEI | CO | регион | operator |
| 40 | Consignor_Address_City | Shijiazhuang | CO | город/район | operator |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CO | улица/дом одной строкой | operator |
| 42 | Consignee_OrganizationName | ООО «СКИФ» | CO | грузополучатель | operator |
| 43 | Consignee_OGRN | 1201600020390 | CO | ОГРН | operator |
| 44 | Consignee_INN | 1650389298 | CO | ИНН | operator |
| 45 | Consignee_KPP | 165001001 | CO | КПП | operator |
| 46 | Consignee_Address_PostalCode | 423800 | CO | индекс | operator |
| 47 | Consignee_Address_CountryCode | RU | CO | страна alpha-2 | operator |
| 48 | Consignee_Address_CounryName | РОССИЯ | CO | страна, текст | operator |
| 49 | Consignee_Address_Region | Республика Татарстан | CO | регион | operator |
| 50 | Consignee_Address_City | Набережные Челны | CO | город | operator |
| 51 | Consignee_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CO | улица/дом/офис одной строкой | operator |
| 52 | InvoiceGoods | | | Массив товаров | |

#### InvoiceGoods_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп. количества для ДТ | |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator |
| 09 | Price | 5.85 | CD | цена за единицу | |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_2
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 * 30 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп. количества для ДТ | |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator |
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator |
| 09 | Price | 5.85 | CD | цена за единицу | |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_3
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп. количества для ДТ | |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator |
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator |
| 09 | Price | 6.35 | CD | цена за единицу | |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_4
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп. количества для ДТ | |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто по строке | operator |
| 08 | NetWeightQuantity | 144.00 | CO | нетто по строке | operator |
| 09 | Price | 6.35 | CD | цена за единицу | |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_5
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 90 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 3780 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп. количества для ДТ | |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто по строке | operator |
| 08 | NetWeightQuantity | 491.40 | CO | нетто по строке | operator |
| 09 | Price | 3.4 | CD | цена за единицу | |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_6
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 180 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 8640 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп. количества для ДТ | |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто по строке | operator |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто по строке | operator |
| 09 | Price | 3.4 | CD | цена за единицу | |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_7
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 240 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп. количества для ДТ | |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто по строке | operator |
| 08 | NetWeightQuantity | 24.00 | CO | нетто по строке | operator |
| 09 | Price | 28 | CD | цена за единицу | |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### Итого, по массиву InvoiceGoods:
- `array_elements`: 7
- `item_fields`: всего полей 105 из 105
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 52 из 52
- `doc_formalization_status`: confirmed


### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list_1
  - `xml_target_root`: AltaE2PACK
  - `path`: md\PL на сетку .md
  - `file_name`: PL на сетку .pdf

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто по упаковочному | |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто по упаковочному | |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | грузоотправитель | |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator |
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | operator |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | |
| 07 | Consignor_Address_Region | HEBEI | CD | регион | |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | |
| 10 | Consignee_OrganizationName | ООО «СКИФ» | CD | грузополучатель | |
| 11 | Consignee_ShortName | ООО «СКИФ» | CO | краткое наименование | operator |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data:EGRUL |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | master_data:EGRUL |
| 14 | Consignee_KPP | 165001001 | CD | КПП | master_data:EGRUL |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | master_data:EGRUL |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | master_data:EGRUL |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | master_data:EGRUL |
| 18 | Consignee_Address_Region | Республика Татарстан | CD | регион | master_data:EGRUL |
| 19 | Consignee_Address_City | Набережные Челны | CD | город | master_data:EGRUL |
| 20 | Consignee_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом/офис | master_data:EGRUL |
| 21 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CD | место поставки | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числ. код условий | operator |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | стр. код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | наим. контракта | operator |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | Commercial invoice | CD | наим. инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | Packing list | CD | наим. упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CD | № упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CD | дата упаковочного | |
| 33 | registration_doc_name | Упаковочный лист | CO | наим. документа | operator |
| 34 | registration_doc_number | LM-2591 | CO | номер документа | operator |
| 35 | registration_doc_date | 30.10.2025 | CO | дата документа | operator |
| 36 | Goods | | | грузовые строки (массив) | |
| 37 | TransportMeans | | | транспорт (массив) | |

#### Goods_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антикот 1,4*0,16*0,16 | CD | описание строки | |
| 02 | GoodsQuantity | 60 | CD | количество мест | |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто | |
| 04 | NetWeightQuantity | 806.60 | CD | нетто | |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок/мест | operator |

#### Goods_2
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антикот 1,6*0,16*0,16 | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест | |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто | |
| 04 | NetWeightQuantity | 460.80 | CD | нетто | |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок/мест | operator |

#### Goods_3
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH 1,42*0,64*0,22 | CD | описание строки | |
| 02 | GoodsQuantity | 6 | CD | количество мест | |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто | |
| 04 | NetWeightQuantity | 252.00 | CD | нетто | |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок/мест | operator |

#### Goods_4
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH 1,62*0,64*0,23 | CD | описание строки | |
| 02 | GoodsQuantity | 3 | CD | количество мест | |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто | |
| 04 | NetWeightQuantity | 144.00 | CD | нетто | |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок/мест | operator |

#### Goods_5
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS 1,42*0,55*0,18 | CD | описание строки | |
| 02 | GoodsQuantity | 9 | CD | количество мест | |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто | |
| 04 | NetWeightQuantity | 491.40 | CD | нетто | |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок/мест | operator |

#### Goods_6
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS 1,62*0,55*18 | CD | описание строки | |
| 02 | GoodsQuantity | 18 | CD | количество мест | |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто | |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто | |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок/мест | operator |

#### Goods_7
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | GRID WITH 3 LAYER 1,72*0,35*0,31*1 | CD | описание строки | |
| 02 | GoodsQuantity | 1 | CD | количество мест | |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто | |
| 04 | NetWeightQuantity | 24.00 | CD | нетто | |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок/мест | operator |

#### Итого, по массиву Goods:
- `array_elements`: 7
- `item_fields`: всего полей 35 из 35
- `array_status`: confirmed

#### TransportMeans_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | О157АО774 | CO | рег. номер | operator |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator |
| 03 | NationalityCode | 000 | CO | код национальности | operator |
| 04 | MoverIndicator | true | CO | тягач | operator |

#### TransportMeans_2
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | ВТ374974 | CO | рег. номер | operator |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator |
| 03 | NationalityCode | 000 | CO | код национальности | operator |
| 04 | MoverIndicator | false | CO | прицеп | operator |

#### Итого, по массиву TransportMeans:
- `array_elements`: 2
- `item_fields`: всего полей 8 из 8
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 37 из 37
- `doc_formalization_status`: confirmed

### `document`: CMR
  - `uqi_prefix`: formalized.cmr_1
  - `xml_target_root`: AltaE3CMR
  - `path`: md\СМР от СВХ.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CO | язык документа | operator |
| 02 | CMR_Choice | 1 | CO | системный выбор | operator |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления | operator |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата принятия груза | |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза | operator |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза | |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки | operator |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки | |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки | operator |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator |
| 13 | GoodsQuantity | 127 | CD | общее кол-во мест | |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто | |
| 15 | CMRTransport_PrimeMoverStateSignID | O157AO774 | CD | тягач | |
| 16 | CMRTransport_TrailerStateSignID | BT374974 | CD | прицеп | |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | отправитель | |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator |
| 19 | Consignor_PostalAddress_CountryCode | CN | CO | страна | operator |
| 20 | Consignor_Address_CounryName | КИТАЙ | CO | страна, текст | operator |
| 21 | Consignor_Address_Region | HEBEI | CO | регион | operator |
| 22 | Consignor_Address_City | Shijiazhuang | CO | город | operator |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CO | улица/дом | operator |
| 31 | Consignee_NameInf | ООО «Скиф» | CD | получатель | |
| 32 | Consignee_ShortName | ООО «Скиф» | CO | краткое наименование | operator |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | operator |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна | |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | |
| 41 | Consignee_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом | |
| 42 | CMRGoods | | | массив строк | |

#### CMRGoods_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | номер строки | авто |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CD | описание груза | из CMR |
| 03 | PakingQuantity | 127 | CO | кол-во мест | operator |

#### Итого, по массиву CMRGoods:
- `array_elements`: 1
- `item_fields`: 3 из 3
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 42 из 42
- `doc_formalization_status`: confirmed

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: md\currency_transfer_1_13.01.2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator |
| 02 | PaymentModeCode | 0 | CO | код способа платежа | operator |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator |
| 05 | Priority | 5 | CO | очередность | operator |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator |
| 13 | Payer_Bank_BankName | VTB Bank | CD | банк плательщика | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CO | фамилия | operator |
| 17 | PersonName | Дмитрий | CO | имя | operator |

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: md\currency_transfer_7_28.11.2025.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator |
| 02 | PaymentModeCode | 0 | CO | код способа платежа | operator |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator |
| 05 | Priority | 5 | CO | очередность | operator |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator |
| 13 | Payer_Bank_BankName | VTB Bank | CD | банк плательщика | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CO | фамилия | operator |
| 17 | PersonName | Дмитрий | CO | имя | operator |

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice_1
  - `xml_target_root`: AltaServiceInvoice
  - `path`: md\Счет_№26-00378-tl_1_от_14-01-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак | operator |
| 02 | TotalServiceCost | 910.34 | CD | итого по услугам | |
| 03 | Currency | RUB | CD | валюта | |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель | |
| 05 | BankName | АО "Райффайзенбанк" | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора | |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора | |
| 08 | PrDocumentNumber | 26-00378-tl | CD | номер заказа | |
| 09 | PrDocumentDate | 14.01.2026 | CD | дата заказа | copied_from:md\Счет_№26-00378-tl_1_от_14-01-2026.md (стр.43) |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наим. | |
| 11 | Registration_PrDocumentNumber | 26-00378-tl/1 | CD | номер | |
| 12 | Registration_PrDocumentDate | 14.01.2026 | CD | дата | |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | грузоотправитель | operator |
| 20 | Consignee_OrganizationName | ООО "Скиф" | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | |
| 24 | PostalCode | 423800 | CD | индекс | |
| 25 | CountryCode | RU | CD | страна | |
| 26 | CounryName | РОССИЯ | CD | страна, текст | |
| 27 | Region | Республика Татарстан | CD | регион | |
| 28 | Town | Набережные Челны | CD | город | |
| 29 | StreetHouse | проезд Хлебный | CD | улица | |
| 30 | House | 30 | CO | дом | operator |
| 31 | Room | 211 | CO | офис | operator |
| 32 | Signature_Choice | 2 | CD | вариант подписи | derived: director/chief_accountant fields present; IE fields absent in md |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | N/A (Signature_Choice=2) |
| 34 | IndividualEntrepreneur_PersonName | | CD | имя ИП | N/A (Signature_Choice=2) |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | N/A (Signature_Choice=2) |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CO | директор фамилия | operator |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CO | директор инициалы | operator |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | | CO | директор отчество | operator |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CO | бухгалтер фамилия | operator |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CO | бухгалтер инициалы | operator |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | | CO | бухгалтер отчество | operator |
| 42 | ServiceDescription | | | массив услуг | |

#### ServiceDescription_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Возмещение за добровольное страхование груза | CD | описание | |
| 02 | CurrencyCode | RUB | CD | валюта | |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator |
| 04 | TaxRate | 0 | CD | ставка | |
| 05 | TaxSum | 0 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 910.34 | CD | стоимость | |
| 07 | ServiceCost_Currency | RUB | CD | валюта | |

#### Итого, по массиву ServiceDescription:
- `array_elements`: 1
- `item_fields`: 7 из 7
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 42 из 42
- `doc_formalization_status`: confirmed

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice_2
  - `xml_target_root`: AltaServiceInvoice
  - `path`: md\Счет_№26-00378-tl_от_27-01-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак | operator |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | |
| 03 | Currency | RUB | CO | валюта | operator:confirmed_rub_for_service_invoice_2 |
| 04 | ServiceProvider_Name | ТРАНСИМПЕРИАЛ | CD | исполнитель | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md (стр.9-12) |
| 05 | BankName | | pending | банк исполнителя | в md не указано |
| 06 | ContractDetails_PrDocumentNumber | K000/26651/М | CD | № договора | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md (стр.15) |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md (стр.15) |
| 08 | PrDocumentNumber | 26-00378-tl | CD | номер заказа | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md (стр.15) |
| 09 | PrDocumentDate | 12.01.2026 | CD | дата заказа | copied_from:md\Счет_№26-00378-tl_от_27-01-2026.md (стр.15) |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наим. | |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер | copied_from:title |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата | copied_from:title |
| 13 | Consignor_OrganizationName | | pending | грузоотправитель | в md не указано |
| 20 | Consignee_OrganizationName | | pending | грузополучатель | в md не указано |
| 21 | Consignee_RFOrganizationFeatures_OGRN | | pending | ОГРН | |
| 22 | Consignee_RFOrganizationFeatures_INN | | pending | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | | pending | КПП | |
| 24 | PostalCode | | pending | индекс | |
| 25 | CountryCode | | pending | страна | |
| 26 | CounryName | | pending | страна, текст | |
| 27 | Region | | pending | регион | |
| 28 | Town | | pending | город | |
| 29 | StreetHouse | | pending | улица | |
| 30 | House | | pending | дом | |
| 31 | Room | | pending | офис | |
| 32 | Signature_Choice | 2 | CD | вариант подписи | derived: IE fields absent; director/chief accountant not present in md |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | N/A (Signature_Choice=2) |
| 34 | IndividualEntrepreneur_PersonName | | CD | имя ИП | N/A (Signature_Choice=2) |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | N/A (Signature_Choice=2) |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | | pending | директор фамилия | в md не указано |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | | pending | директор инициалы | |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | | pending | директор отчество | |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | | pending | бухгалтер фамилия | |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | | pending | бухгалтер инициалы | |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | | pending | бухгалтер отчество | |
| 42 | ServiceDescription | | | массив услуг | |

#### ServiceDescription_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №K000/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск) Перевозка автотранспортом | CD | описание | md стр.15 |
| 02 | CurrencyCode | RUB | CO | валюта | operator:confirmed_rub_for_service_invoice_2 |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator |
| 04 | TaxRate | 0 | CD | ставка | from md: НДС 0% |
| 05 | TaxSum | 0 | CD | сумма налога | from md: НДС 0% |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость | md стр.15 |
| 07 | ServiceCost_Currency | RUB | CO | валюта | operator:confirmed_rub_for_service_invoice_2 |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### ServiceDescription_2
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание | md стр.16 |
| 02 | CurrencyCode | RUB | CO | валюта | operator:confirmed_rub_for_service_invoice_2 |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator |
| 04 | TaxRate | 0 | CD | ставка | from md: НДС 0% |
| 05 | TaxSum | 0 | CD | сумма налога | from md: НДС 0% |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость | md стр.16 |
| 07 | ServiceCost_Currency | RUB | CO | валюта | operator:confirmed_rub_for_service_invoice_2 |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### Итого, по массиву ServiceDescription:
- `array_elements`: 2
- `item_fields`: всего полей 14 из 14
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 42 из 42
- `doc_formalization_status`: confirmed

### `document`: TechDescription
  - `uqi_prefix`: formalized.tech_description_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\техничка Антикот, антипыльца антимошка .md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код документа | derived |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование | |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата | operator |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер | operator |
| 05 | TextPara | link:md\техничка Антикот, антипыльца антимошка .md | CD | текст | |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

### `document`: EGRUL
  - `uqi_prefix`: formalized.egrul_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04011 | CD | код | derived |
| 02 | DocumentHead_DocumentName | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование | |
| 03 | DocumentHead_DocumentDate | 14.07.2025 | CD | дата | |
| 04 | DocumentHead_DocumentNumber | ЮЭ9965-25-106893283 | CD | номер | |
| 05 | TextPara | link:alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml | CD | текст | |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

## non_formalized:

### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.transit_declaration_1
  - `path`: md\ТД 10719110_240126_5011363_reg00378тд.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 10719110/240126/5011363 | CO | номер | operator |
| 02 | date | 24.01.2026 | CO | дата | operator |
| 03 | customs_post_code | 10404083 | CD | код поста | |
| 04 | customs_post_name | Таможенный пост Набережночелнинский | CD | наименование поста | |
| 05 | transport_reg_number | O157AO774/BT374974 | CD | ТС | |

#### Итого, по документу:
- `doc_fields`: 5 из 5

### `document`: Transport Contract
  - `uqi_prefix`: formalized.transport_contract_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\stable_source\FreeDoc_КООО_26651_М.xml
  - `file_name`: FreeDoc_КООО_26651_М.xml

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04033 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование договора | |
| 03 | DocumentHead_DocumentDate | 13.05.2025 | CD | дата договора | |
| 04 | DocumentHead_DocumentNumber | КООО/26651/М | CD | номер договора | |
| 05 | TextPara | link:alta\stable_source\FreeDoc_КООО_26651_М.xml | CD | текст договора | |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

### `document`: Personal Passport
  - `uqi_prefix`: formalized.passport_1
  - `xml_target_root`: AltaPassport
  - `path`: alta\stable_source\Passport_63_09_449948.xml
  - `file_name`: Passport_63_09_449948.xml

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 11001 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | ПАСПОРТ | CD | наименование документа | derived |
| 03 | DocumentHead_DocumentDate | 2010-03-11 | CD | дата документа | copied_from:CardDate |
| 04 | DocumentHead_DocumentNumber | 63 09 449948 | CD | номер документа | derived |
| 05 | CardSeries | 63 09 | CD | серия | |
| 06 | CardNumber | 449948 | CD | номер | |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | |
| 08 | CardDate | 2010-03-11 | CD | дата выдачи | |
| 09 | PersonInfo_PersonSurname | АРБУЗОВА | CD | фамилия | |
| 10 | PersonInfo_PersonName | АНАСТАСИЯ | CD | имя | |
| 11 | PersonInfo_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | |
| 12 | PersonInfo_Sex | 1 | CD | пол | |
| 13 | PersonInfo_Birthday | 1987-07-25 | CD | дата рождения | |
| 14 | PersonInfo_Birthplace | город Саратов | CD | место рождения | |
| 15 | ResidencePlace_PostalCode | 410052 | CD | индекс | |
| 16 | ResidencePlace_CountryCode | RU | CD | страна alpha-2 | |
| 17 | ResidencePlace_CounryName | РОССИЯ | CD | страна, текст | |
| 18 | ResidencePlace_Region | Саратовская область | CD | регион | |
| 19 | ResidencePlace_City | Саратов | CD | город | |
| 20 | ResidencePlace_StreetHouse | Ул. Одесская д 11 кв 160 | CD | адрес | |

#### Итого, по документу:
- `doc_fields`: 20 из 20
- `doc_formalization_status`: confirmed

### `document`: Letter of Attorney
  - `uqi_prefix`: formalized.letter_of_attorney_1
  - `xml_target_root`: AltaLetterOfAttorney
  - `path`: alta\stable_source\LetterOfAttorney_1.xml
  - `file_name`: LetterOfAttorney_1.xml

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 11004 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | ДОВЕРЕННОСТЬ | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 2026-02-01 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | 1 | CD | номер документа | |
| 05 | Subject | link:alta\stable_source\LetterOfAttorney_1.xml | CD | текст доверенности | |
| 06 | EndDate | 2026-12-31 | CD | действительна до | |
| 07 | DocumentReference_PrDocumentName | ДОВЕРЕННОСТЬ | CD | наименование | |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер | |
| 09 | DocumentReference_PrDocumentDate | 2026-02-01 | CD | дата | |
| 10 | Organization_OrganizationName | ООО «СКИФ» | CD | выдавшая организация | |
| 11 | Organization_ShortName | ООО «СКИФ» | CD | краткое наименование | |
| 12 | Organization_OGRN | 1201600020390 | CD | ОГРН | |
| 13 | Organization_INN | 1650389298 | CD | ИНН | |
| 14 | Organization_KPP | 165001001 | CD | КПП | |
| 15 | Organization_Address_PostalCode | 423800 | CD | индекс | |
| 16 | Organization_Address_CountryCode | RU | CD | страна | |
| 17 | Organization_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 18 | Organization_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 19 | Organization_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 20 | Organization_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | CD | адрес | |
| 21 | Organization_OrganizationPerson_PersonSurname | Саранов | CD | подписант | |
| 22 | Organization_OrganizationPerson_PersonName | Дмитрий | CD | имя | |
| 23 | Organization_OrganizationPerson_PersonMiddleName | Олегович | CD | отчество | |
| 24 | Organization_OrganizationPerson_PersonPost | Директор | CD | должность | |
| 25 | EmpoweredPerson_PersonSurname | АРБУЗОВА | CD | уполномоченное лицо | |
| 26 | EmpoweredPerson_PersonName | АНАСТАСИЯ | CD | имя | |
| 27 | EmpoweredPerson_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | |
| 28 | EmpoweredPerson_PersonPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль | |
| 29 | EmpoweredPerson_Passport_IdentityCardCode | RU01001 | CD | код паспорта | |
| 30 | EmpoweredPerson_Passport_IdentityCardName | ПАСРФ | CD | наименование паспорта | |
| 31 | EmpoweredPerson_Passport_IdentityCardSeries | 63 09 | CD | серия | |
| 32 | EmpoweredPerson_Passport_IdentityCardNumber | 449948 | CD | номер | |
| 33 | EmpoweredPerson_Passport_IdentityCardDate | 2010-03-11 | CD | дата выдачи | |
| 34 | EmpoweredPerson_Passport_OrganizationName | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | |

#### Итого, по документу:
- `doc_fields`: 34 из 34
- `doc_formalization_status`: confirmed

## non_formalized (продолжение):

### `document`: Storage Report (ДО-1)
  - `uqi_prefix`: non_formalized.svh_1
  - `path`: md\ДО 14431420260204161621.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 1040414210/10092/5 | CD | номер лицензии/свидетельства СВХ | |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии/свидетельства СВХ | |
| 03 | actual_gross_weight | 3500.00 | CD | фактический вес по весам | copied_from:formalized.packing_list_1.GrossWeightQuantity |
| 04 | actual_places | 127 | CD | фактическое количество мест | copied_from:formalized.invoice_1.PlacesQuantity |
| 05 | transport_reg_number | O157AO774/BT374974 | CD | номер ТС | copied_from:formalized.cmr_1.CMRTransport_PrimeMoverStateSignID+Trailer |
| 06 | goods_1.tnved | | pending | код товара | в ДО-1 нет таблицы разбивки |

#### Итого, по документу:
- `doc_fields`: 6 из 6

### `document`: Storage Report Additional Sheet
  - `uqi_prefix`: non_formalized.svh_additional_sheet_1
  - `path`: md\ДО доп 14431520260204161645.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 0000080 | CD | № доп.листа/приложения | copied_from:md ДО-1 |
| 02 | date | 03.02.2026 | CD | дата доп.листа | copied_from:md ДО-1 |
| 03 | actual_gross_weight | 3500.00 | CD | фактический вес | copied_from:formalized.packing_list_1.GrossWeightQuantity |
| 04 | actual_places | 127 | CD | фактическое количество мест | copied_from:formalized.invoice_1.PlacesQuantity |
| 05 | transport_reg_number | O157AO774/BT374974 | CD | номер ТС | copied_from:formalized.cmr_1 |
| 06 | svh_address_region | Республика Татарстан | CD | регион СВХ | copied_from:formalized.cmr_1 |
| 07 | svh_address_city | Набережные Челны | CD | город СВХ | copied_from:formalized.cmr_1 |
| 08 | svh_address_street_house | Производственный пр-д, д.45 | CD | улица/дом СВХ | copied_from:formalized.cmr_1 |
| 09 | svh_customs_code | 10404083 | CD | код таможенного органа СВХ | from CMR Sender's instructions |

#### Итого, по документу:
- `doc_fields`: 9 из 9

### Итогo, по файлу:

`total_doc_fields` - 352
`total_fields` - 467
`formalization_status` - confirmed

## Нерешенные вопросы (Issues)

**Для полей:**
- `non_formalized.svh_1.goods_1.tnved`
  - `question`: В ДО-1 нет таблицы с разбивкой по товарам; код ТН ВЭД для строки СВХ не извлечён.
