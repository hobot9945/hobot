## meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 2 товара
- `источники данных`: md + operator_provided_data + stable_source (xml)

## formalized

### `document`: Contract
  - `uqi_prefix`: formalized.contract_1
  - `xml_target_root`: AltaE2CONT
  - `path`: md\SALES CONTRACT NoLM-2553.md
  - `file_name`: SALES CONTRACT NoLM-2553.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | DocumentCode | 03011 | CO | код вида документа | решение оператора |
| 2 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта | |
| 3 | ContractRegistration_PrDocumentDate | 02.07.2025 | CD | дата контракта | |
| 4 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта | доп.соглашение №1; решение оператора |
| 5 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты | CNY; решение оператора |
| 6 | ContractTerms_LastDate | 31.12.2026 | CO | срок действия | решение оператора |
| 7 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки | решение оператора |
| 8 | ContractTerms_ContractText | link:md\SALES CONTRACT NoLM-2553.md | CD | текст контракта | |
| 9 | ContractTerms_DealSign | 1 | CO | системный признак | решение оператора |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец | |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 | подтверждено оператором |
| 12 | ForeignPerson_Address_CounryName | КИТАЙ | CD | страна продавца текст | из контракта |
| 13 | ForeignPerson_Address_Region | HEBEI | CD | регион | контракт: No.5 Gaodong Street, Xinhua District, Shijiazhuang |
| 14 | ForeignPerson_Address_City | SHIJIAZHUANG | CD | город | |
| 15 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District | CD | улица/дом | |
| 16 | RussianPerson_OrganizationName | ООО «СКИФ» | CO | покупатель | master_data |
| 17 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН | из master_data, решение оператора |
| 18 | RussianPerson_INN | 1650389298 | CO | ИНН | из master_data, решение оператора |
| 19 | RussianPerson_KPP | 165001001 | CO | КПП | из master_data, решение оператора |
| 20 | RussianPerson_Address_PostalCode | 423800 | CD | индекс | |
| 21 | RussianPerson_Address_CountryCode | RU | CD | страна alpha-2 | нормализация по cb:country |
| 22 | RussianPerson_Address_CounryName | РОССИЯ | CD | страна текст | |
| 23 | RussianPerson_Address_Region | Республика Татарстан (Татарстан) | CD | регион | |
| 24 | RussianPerson_Address_City | Набережные Челны | CD | город | |
| 25 | RussianPerson_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом | |

#### Итого, по документу:
- `doc_fields`: 25 из 25
- `doc_formalization_status`: confirmed

### `document`: Supplementary Contract
  - `uqi_prefix`: formalized.supplementary_contract_1
  - `xml_target_root`: AltaSupplementaryContract
  - `path`: md\1 Supplementary agreement to the contract.md
  - `file_name`: 1 Supplementary agreement to the contract.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | DocumentNumber | 1 | CD | № доп. соглашения | |
| 2 | IssueDate | 25.11.2025 | CD | дата доп. соглашения | |
| 3 | ContractDescription_Amount | 270000.00 | CD | новая сумма контракта | |
| 4 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты | CNY; решение оператора |
| 5 | ContractDescription_LastDate | 31.12.2026 | CO | срок действия | решение оператора |
| 6 | ContractDescription_ContractText | link:md\1 Supplementary agreement to the contract.md | CD | текст доп. соглашения | |
| 7 | ContractDescription_DealSign | 1 | CO | системный признак | решение оператора |
| 8 | ContractDescription_StockCategorySign | 0 | CO | системный признак | решение оператора |
| 9 | ContractDescription_BuyerLimitationSign | 0 | CO | системный признак | решение оператора |
| 10 | ContractDescription_InsuranceSign | 0 | CO | системный признак | решение оператора |
| 11 | RussianPerson_OrganizationName | ООО «СКИФ» | CO | покупатель | решение оператора: из contract_1 |
| 12 | RussianPerson_ShortName | ООО «СКИФ» | CO | краткое наименование | |
| 13 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН | из master_data |
| 14 | RussianPerson_INN | 1650389298 | CO | ИНН | из master_data |
| 15 | RussianPerson_KPP | 165001001 | CO | КПП | из master_data |
| 16 | RussianPerson_Address_PostalCode | 423800 | CO | индекс | из contract_1 |
| 17 | RussianPerson_Address_CountryCode | RU | CO | страна alpha-2 | |
| 18 | RussianPerson_Address_CounryName | РОССИЯ | CO | страна текст | |
| 19 | RussianPerson_Address_Region | Республика Татарстан (Татарстан) | CO | регион | |
| 20 | RussianPerson_Address_City | Набережные Челны | CO | город | |
| 21 | RussianPerson_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CO | улица/дом | |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец | |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | решение оператора: =полному |
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 | подтверждено оператором |
| 25 | ForeignPerson_Address_CounryName | КИТАЙ | CO | страна текст | из contract_1 |
| 26 | ForeignPerson_Address_Region | HEBEI | CO | регион | из contract_1 |
| 27 | ForeignPerson_Address_City | SHIJIAZHUANG | CO | город | из contract_1 |
| 28 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District | CO | улица/дом | из contract_1 |
| 29 | ContractSignedPerson_PersonSurname | Li | CO | фамилия подписанта | решение оператора |
| 30 | ContractSignedPerson_PersonName | Jing | CO | имя подписанта | решение оператора |
| 31 | ContractSignedPerson_PersonMiddleName | | CO | отчество подписанта | решение оператора: пусто |

#### Итого, по документу:
- `doc_fields`: 31 из 31
- `doc_formalization_status`: confirmed

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: md\CL на сетку.md
  - `file_name`: CL на сетку.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | CurrencyRate | 10.9430 | CO | курс валюты | решение оператора |
| 2 | CurrencyCode | CNY | CO | валюта инвойса alpha-3 | решение оператора |
| 3 | DocumentCode | 04021 | CO | код вида документа | решение оператора |
| 4 | PlacesQuantity | 127 | CD | кол-во грузовых мест | из инвойса: Qty/BG 127 pcs |
| 5 | PlacesDescription | Поддон | CO | описание мест | решение оператора |
| 6 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто | из PL totals; решение оператора |
| 7 | NetWeightQuantity | 3302.00 | CO | общий вес нетто | из PL totals; решение оператора |
| 8 | GCost | 97260.00 | CO | системное поле | решение оператора: =TotalCost |
| 9 | TotalCost | 97260.00 | CD | итого по инвойсу | |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий | EXW; решение оператора |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | решение оператора |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | решение оператора |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | решение оператора |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | решение оператора |
| 16 | Registration_PrDocumentName | Commercial invoice / Комерчесский инвойс | CD | наименование документа | |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | |
| 21 | Buyer_CompanyID | 1650389298 | CO | ИНН покупателя | master_data |
| 22 | Buyer_KPPCode | 165001001 | CO | КПП покупателя | master_data |
| 23 | Buyer_Name | ООО «СКИФ» | CD | наименование покупателя | |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | нормализация |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя текст | |
| 27 | Buyer_PostalAddress_Region | Республика Татарстан | CD | регион | |
| 28 | Buyer_PostalAddress_City | Набережные Челны | CD | город | |
| 29 | Buyer_PostalAddress_StreetHouse | Хлебный проезд, дом 30, офис 211 | CD | улица/дом | |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | продавец | |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | подтверждено оператором |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца текст | |
| 33 | Seler_PostalAddress_Region | HEBEI | CD | регион продавца | |
| 34 | Seler_PostalAddress_City | SHIJIAZHUANG | CD | город продавца | |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом | |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | решение оператора: =seller |
| 37 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | =seller |
| 38 | Consignor_Address_CounryName | КИТАЙ | CO | страна грузоотправителя текст | =seller |
| 39 | Consignor_Address_Region | HEBEI | CO | регион | =seller |
| 40 | Consignor_Address_City | SHIJIAZHUANG | CO | город | =seller |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CO | улица/дом | =seller |
| 42 | Consignee_OrganizationName | ООО «СКИФ» | CO | грузополучатель | решение оператора: =buyer |
| 43 | Consignee_OGRN | 1201600020390 | CO | ОГРН | master_data |
| 44 | Consignee_INN | 1650389298 | CO | ИНН | master_data |
| 45 | Consignee_KPP | 165001001 | CO | КПП | master_data |
| 46 | Consignee_Address_PostalCode | 423800 | CO | индекс | =buyer |
| 47 | Consignee_Address_CountryCode | RU | CO | страна alpha-2 | =buyer |
| 48 | Consignee_Address_CounryName | РОССИЯ | CO | страна текст | =buyer |
| 49 | Consignee_Address_Region | Республика Татарстан | CO | регион | =buyer |
| 50 | Consignee_Address_City | Набережные Челны | CO | город | =buyer |
| 51 | Consignee_Address_StreetHouse | Хлебный проезд, дом 30, офис 211 | CO | улица/дом | =buyer |

#### InvoiceGoods_1
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 2 | GoodsDescription | Anti-cat mesh. Roll size 1.4*30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара | |
| 3 | GoodsQuantity | 60 | CD | кол-во в основной единице | Sets |
| 4 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед. | Quantity in M2 |
| 5 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед. | |
| 6 | MeasureUnitQualifierName | м² (квадратный метр) | CD | ед.изм доп.количества | |
| 7 | GrossWeightQuantity | 855.00 | CO | брутто по строке | из PL; решение оператора |
| 8 | NetWeightQuantity | 806.60 | CO | нетто по строке | из PL; решение оператора |
| 9 | Price | 5.85 | CD | цена за M2 | |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | страна происхождения цифровой код | CN; решение оператора |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | решение оператора |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | решение оператора |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | решение оператора |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_2
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 2 | GoodsDescription | Anti-cat mesh Roll size 1.6*30 / Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара | |
| 3 | GoodsQuantity | 30 | CD | кол-во | Sets |
| 4 | goods_supplementary_quantity | 1440 | CD | количество в M2 | |
| 5 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед. | |
| 6 | MeasureUnitQualifierName | м² (квадратный метр) | CD | ед.изм доп.количества | |
| 7 | GrossWeightQuantity | 490.00 | CO | брутто | из PL; решение оператора |
| 8 | NetWeightQuantity | 460.80 | CO | нетто | из PL; решение оператора |
| 9 | Price | 5.85 | CD | цена за M2 | |
| 10 | TotalCost | 8424.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | решение оператора |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | решение оператора |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | решение оператора |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | решение оператора |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_3
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 2 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 / Сетка против пыльцы «Антипыльца» из полиэстера. Размер рулона 1,4*30 M2 | CD | описание товара | |
| 3 | GoodsQuantity | 60 | CD | кол-во | Sets |
| 4 | goods_supplementary_quantity | 2520 | CD | количество в M2 | |
| 5 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед. | |
| 6 | MeasureUnitQualifierName | м² (квадратный метр) | CD | ед.изм доп.количества | |
| 7 | GrossWeightQuantity | 265.00 | CO | брутто | из PL; решение оператора |
| 8 | NetWeightQuantity | 252.00 | CO | нетто | из PL; решение оператора |
| 9 | Price | 6.35 | CD | цена за M2 | |
| 10 | TotalCost | 16002.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | решение оператора |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | решение оператора |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | решение оператора |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | решение оператора |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_4
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 2 | GoodsDescription | ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2 / Сетка против пыльцы «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | |
| 3 | GoodsQuantity | 30 | CD | кол-во | Sets |
| 4 | goods_supplementary_quantity | 1440 | CD | количество в M2 | |
| 5 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед. | |
| 6 | MeasureUnitQualifierName | м² (квадратный метр) | CD | ед.изм доп.количества | |
| 7 | GrossWeightQuantity | 155.00 | CO | брутто | из PL; решение оператора |
| 8 | NetWeightQuantity | 144.00 | CO | нетто | из PL; решение оператора |
| 9 | Price | 6.35 | CD | цена за M2 | |
| 10 | TotalCost | 9144.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | решение оператора |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | решение оператора |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | решение оператора |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | решение оператора |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_5
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 2 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 / СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара | |
| 3 | GoodsQuantity | 90 | CD | кол-во | Sets |
| 4 | goods_supplementary_quantity | 3780 | CD | количество в M2 | |
| 5 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед. | |
| 6 | MeasureUnitQualifierName | м² (квадратный метр) | CD | ед.изм доп.количества | |
| 7 | GrossWeightQuantity | 520.00 | CO | брутто | из PL; решение оператора |
| 8 | NetWeightQuantity | 491.40 | CO | нетто | из PL; решение оператора |
| 9 | Price | 3.40 | CD | цена за M2 | |
| 10 | TotalCost | 12852.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | решение оператора |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | решение оператора |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | решение оператора |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | решение оператора |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_6
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 2 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 / СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара | |
| 3 | GoodsQuantity | 180 | CD | кол-во | Sets |
| 4 | goods_supplementary_quantity | 8640 | CD | количество в M2 | |
| 5 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед. | |
| 6 | MeasureUnitQualifierName | м² (квадратный метр) | CD | ед.изм доп.количества | |
| 7 | GrossWeightQuantity | 1190.00 | CO | брутто | из PL; решение оператора |
| 8 | NetWeightQuantity | 1123.20 | CO | нетто | из PL; решение оператора |
| 9 | Price | 3.40 | CD | цена за M2 | |
| 10 | TotalCost | 29376.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | решение оператора |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | решение оператора |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | решение оператора |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | решение оператора |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_7
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 2 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 / Трехслойные сетки «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | |
| 3 | GoodsQuantity | 5 | CD | кол-во | Sets |
| 4 | goods_supplementary_quantity | 240 | CD | количество в M2 | |
| 5 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед. | |
| 6 | MeasureUnitQualifierName | м² (квадратный метр) | CD | ед.изм доп.количества | |
| 7 | GrossWeightQuantity | 25.00 | CO | брутто | из PL; решение оператора |
| 8 | NetWeightQuantity | 24.00 | CO | нетто | из PL; решение оператора |
| 9 | Price | 28.00 | CD | цена за M2 | |
| 10 | TotalCost | 6720.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | решение оператора |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | решение оператора |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | решение оператора |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак | решение оператора |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 105 из 105
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 52 из 52
- `doc_formalization_status`: confirmed

### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list_1
  - `xml_target_root`: AltaE2PACK
  - `path`: md\PL на сетку.md
  - `file_name`: PL на сетку.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто | |
| 2 | NetWeightQuantity | 3302.00 | CD | общий вес нетто | |
| 3 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | |
| 4 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | краткое наименование | решение оператора: =полному |
| 5 | Consignor_Address_CountryCode | CN | CO | страна alpha-2 | подтверждено оператором |
| 6 | Consignor_Address_CounryName | КИТАЙ | CD | страна текст | нормализация |
| 7 | Consignor_Address_Region | HEBEI | CD | регион | |
| 8 | Consignor_Address_City | SHIJIAZHUANG | CD | город | |
| 9 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | |
| 10 | Consignee_OrganizationName | ООО «СКИФ» | CD | грузополучатель | |
| 11 | Consignee_ShortName | ООО «СКИФ» | CO | краткое наименование | решение оператора: =полному |
| 12 | Consignee_OGRN | 1201600020390 | CO | ОГРН | master_data |
| 13 | Consignee_INN | 1650389298 | CO | ИНН | master_data |
| 14 | Consignee_KPP | 165001001 | CO | КПП | master_data |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | нормализация |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | |
| 18 | Consignee_Address_Region | Республика Татарстан | CD | регион | |
| 19 | Consignee_Address_City | Набережные Челны | CD | город | |
| 20 | Consignee_Address_StreetHouse | Хлебный проезд, дом 30, офис 211 | CD | улица/дом | |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий | EXW |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CD | наименование контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | Commercial invoice / Комерчесский инвойс | CD | наименование инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | Packing list / Упаковочный лист | CD | наименование упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | решение оператора |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | решение оператора |
| 33 | registration_doc_name | Упаковочный лист | CO | наименование для графы 44 | решение оператора |
| 34 | registration_doc_number | LM-2591 | CO | номер для графы 44 | решение оператора |
| 35 | registration_doc_date | 30.10.2025 | CO | дата для графы 44 | решение оператора |

#### Goods_1
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание | |
| 2 | GoodsQuantity | 60 | CD | кол-во мест | |
| 3 | GrossWeightQuantity | 855.00 | CD | брутто | |
| 4 | NetWeightQuantity | 806.60 | CD | нетто | |
| 5 | PackingInfo_PakingQuantity | 60 | CO | кол-во упаковок | решение оператора: =GoodsQuantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_2
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание | |
| 2 | GoodsQuantity | 30 | CD | кол-во мест | |
| 3 | GrossWeightQuantity | 490.00 | CD | брутто | |
| 4 | NetWeightQuantity | 460.80 | CD | нетто | |
| 5 | PackingInfo_PakingQuantity | 30 | CO | кол-во упаковок | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_3
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsDescription | ANTI-POLLEN MESH / Сетка против пыльцы «Антипыльца» из полиэстера. Размер рулона 1,42*0,64*0,22 | CD | описание | |
| 2 | GoodsQuantity | 6 | CD | кол-во мест | BG |
| 3 | GrossWeightQuantity | 265.00 | CD | брутто | |
| 4 | NetWeightQuantity | 252.00 | CD | нетто | |
| 5 | PackingInfo_PakingQuantity | 6 | CO | кол-во упаковок | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_4
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsDescription | ANTI-POLLEN MESH / Сетка против пыльцы «Антипыльца» из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание | |
| 2 | GoodsQuantity | 3 | CD | кол-во мест | BG |
| 3 | GrossWeightQuantity | 155.00 | CD | брутто | |
| 4 | NetWeightQuantity | 144.00 | CD | нетто | |
| 5 | PackingInfo_PakingQuantity | 3 | CO | кол-во упаковок | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_5
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsDescription | MIDGE MEHS / СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание | |
| 2 | GoodsQuantity | 9 | CD | кол-во мест | BG |
| 3 | GrossWeightQuantity | 520.00 | CD | брутто | |
| 4 | NetWeightQuantity | 491.40 | CD | нетто | |
| 5 | PackingInfo_PakingQuantity | 9 | CO | кол-во упаковок | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_6
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsDescription | MIDGE MEHS / СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание | |
| 2 | GoodsQuantity | 18 | CD | кол-во мест | BG |
| 3 | GrossWeightQuantity | 1190.00 | CD | брутто | |
| 4 | NetWeightQuantity | 1123.20 | CD | нетто | |
| 5 | PackingInfo_PakingQuantity | 18 | CO | кол-во упаковок | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_7
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsDescription | GRID WITH 3 LAYER / Трехслойные сетки «Антипыльца» из полиэстера Размер рулона 1,72*0,35*0,31*1 | CD | описание | |
| 2 | GoodsQuantity | 1 | CD | кол-во мест | BG |
| 3 | GrossWeightQuantity | 25.00 | CD | брутто | |
| 4 | NetWeightQuantity | 24.00 | CD | нетто | |
| 5 | PackingInfo_PakingQuantity | 1 | CO | кол-во упаковок | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву Goods:
- `array_elements`: 7
- `item_fields`: всего полей 35 из 35
- `array_status`: confirmed

#### TransportMeans_1
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | Number | О157АО774 | CO | номер тягача | решение оператора |
| 2 | ModeCode | 31 | CO | код вида транспорта | решение оператора |
| 3 | NationalityCode | 000 | CO | код национальности | решение оператора |
| 4 | MoverIndicator | true | CO | признак тягача | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### TransportMeans_2
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | Number | ВТ374974 | CO | номер прицепа | решение оператора |
| 2 | ModeCode | 31 | CO | код вида транспорта | решение оператора |
| 3 | NationalityCode | 000 | CO | код национальности | решение оператора |
| 4 | MoverIndicator | false | CO | признак прицепа | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

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
  - `file_name`: СМР от СВХ.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | LanguageCode | RU | CO | язык документа | решение оператора |
| 2 | CMR_Choice | 1 | CO | системный выбор | решение оператора |
| 3 | RegistrationDocument_RegID | 00378 | CD | номер CMR | |
| 4 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | |
| 5 | RegistrationDocument_Place | Маньчжурия | CO | место составления | решение оператора |
| 6 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата принятия груза | |
| 7 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия alpha-2 | решение оператора |
| 8 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия текст | нормализация |
| 9 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | решение оператора |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки текст | |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки | решение оператора |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | решение оператора |
| 13 | GoodsQuantity | 127 | CD | общее кол-во мест | |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто | |
| 15 | CMRTransport_PrimeMoverStateSignID | O157AO774 | CD | гос.номер тягача | |
| 16 | CMRTransport_TrailerStateSignID | BT374974 | CD | гос.номер прицепа | |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование отправителя | |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | решение оператора |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна alpha-2 | нормализация |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна текст | |
| 21 | Consignor_Address_Region | HEBEI | CD | регион | |
| 22 | Consignor_Address_City | SHIJIAZHUANG | CD | город | |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | гарант | решение оператора |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое гаранта | решение оператора |
| 26 | Consignor_Guarantee_Address_CountryCode | | CO | страна гаранта | решение оператора: отсутствует |
| 27 | Consignor_Guarantee_Address_CounryName | | CO | страна текст | |
| 28 | Consignor_Guarantee_Address_Region | | CO | регион | |
| 29 | Consignor_Guarantee_Address_City | | CO | город | |
| 30 | Consignor_Guarantee_Address_StreetHouse | | CO | улица/дом | |
| 31 | Consignee_NameInf | ООО «Скиф» | CD | наименование получателя | |
| 32 | Consignee_ShortName | ООО «Скиф» | CO | краткое наименование | решение оператора |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | решение оператора: из master_data |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | из CMR |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | из CMR |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | нормализация |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом | |

#### CMRGoods_1
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsNumeric | 1 | CD | порядковый номер | авто-нумерация единственной строки |
| 2 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CO | описание груза | исключение CMRGoodsDescription: из non_formalized.svh_1 |
| 3 | GoodsPackingInfo_PakingQuantity | 127 | CO | кол-во упаковок/мест | решение оператора |

#### Итого, по элементу массива:
- `item_fields`: 3 из 3

#### Итого, по массиву CMRGoods:
- `array_elements`: 1
- `item_fields`: всего полей 3 из 3
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 42 из 42
- `doc_formalization_status`: confirmed

### `document`: Payment Order 1
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: md\currency_transfer_1_13.01.2026.md
  - `file_name`: currency_transfer_1_13.01.2026.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | DocumentCode | 04023 | CO | код вида документа | решение оператора |
| 2 | PaymentModeCode | 0 | CO | системный код способа платежа | решение оператора |
| 3 | PaymentAmount | 63219.00 | CD | сумма платежа | |
| 4 | TransactionKind | 01 | CO | вид операции | решение оператора |
| 5 | Priority | 5 | CO | очередность | решение оператора |
| 6 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 7 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | |
| 8 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | |
| 9 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | решение оператора |
| 13 | Payer_Bank_BankName | ФИЛИАЛ «ЦЕНТРАЛЬНЫЙ» БАНКА ВТБ (ПАО) | CD | банк плательщика | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | банк получателя | |
| 16 | PayerSign_PersonSurname | Саранов | CO | фамилия подписанта | решение оператора |
| 17 | PayerSign_PersonName | Дмитрий | CO | имя подписанта | решение оператора |

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### `document`: Payment Order 2
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: md\currency_transfer_7_28.11.2025.md
  - `file_name`: currency_transfer_7_28.11.2025.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | DocumentCode | 04023 | CO | код вида документа | решение оператора |
| 2 | PaymentModeCode | 0 | CO | системный код | решение оператора |
| 3 | PaymentAmount | 34041.00 | CD | сумма платежа | |
| 4 | TransactionKind | 01 | CO | вид операции | решение оператора |
| 5 | Priority | 5 | CO | очередность | решение оператора |
| 6 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 7 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | |
| 8 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | |
| 9 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | решение оператора |
| 13 | Payer_Bank_BankName | ФИЛИАЛ «ЦЕНТРАЛЬНЫЙ» БАНКА ВТБ (ПАО) | CD | банк плательщика | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | банк получателя | |
| 16 | PayerSign_PersonSurname | Саранов | CO | фамилия | решение оператора |
| 17 | PayerSign_PersonName | Дмитрий | CO | имя | решение оператора |

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice_1
  - `xml_target_root`: AltaServiceInvoice
  - `path`: md\Счет_№26-00378-tl_от_27-01-2026.md
  - `file_name`: Счет_№26-00378-tl_от_27-01-2026.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | DocumentSign | 1 | CO | системный признак | решение оператора |
| 2 | TotalServiceCost | 2700.00 | CD | итого по услугам | |
| 3 | Currency | USD | CD | валюта итого alpha-3 | |
| 4 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг | |
| 5 | ServiceProvider_PaymentRequisitions_BankName | АО «Райффайзенбанк» | CD | банк исполнителя | |
| 6 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги | |
| 7 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора | |
| 8 | PaymentDocument_PrDocumentNumber | ОТСУТСТВУЕТ | CO | номер связанного документа | решение оператора |
| 9 | PaymentDocument_PrDocumentDate | ОТСУТСТВУЕТ | CO | дата связанного документа | решение оператора |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | решение оператора: =seller |
| 14 | Consignor_SubjectAddressDetails_PostalCode | | CO | индекс | решение оператора: пусто |
| 15 | Consignor_SubjectAddressDetails_CountryCode | CN | CO | страна alpha-2 | =seller |
| 16 | Consignor_SubjectAddressDetails_CounryName | КИТАЙ | CO | страна текст | =seller |
| 17 | Consignor_SubjectAddressDetails_Region | HEBEI | CO | регион | =seller |
| 18 | Consignor_SubjectAddressDetails_Town | SHIJIAZHUANG | CO | город | =seller |
| 19 | Consignor_SubjectAddressDetails_StreetHouse | No. 5 Gaodong street | CO | улица/дом | =seller |
| 20 | Consignee_OrganizationName | ООО «СКиФ» | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | решение оператора: из master_data |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | |
| 24 | Consignee_SubjectAddressDetails_PostalCode | 423800 | CD | индекс | |
| 25 | Consignee_SubjectAddressDetails_CountryCode | RU | CD | страна alpha-2 | |
| 26 | Consignee_SubjectAddressDetails_CounryName | РОССИЯ | CD | страна текст | |
| 27 | Consignee_SubjectAddressDetails_Region | Республика Татарстан | CD | регион | |
| 28 | Consignee_SubjectAddressDetails_Town | Набережные Челны | CD | город | |
| 29 | Consignee_SubjectAddressDetails_StreetHouse | проезд Хлебный | CD | улица | |
| 30 | Consignee_SubjectAddressDetails_House | 30 | CO | дом | решение оператора |
| 31 | Consignee_SubjectAddressDetails_Room | 211 | CO | офис | решение оператора |
| 32 | Signature_Choice | 1 | CO | вариант подписи | решение оператора |
| 33 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CO | фамилия руководителя | решение оператора |
| 34 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CO | инициалы руководителя | решение оператора |
| 35 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CO | фамилия бухгалтера | решение оператора |
| 36 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CO | инициалы бухгалтера | решение оператора |

#### ServiceDescription_1
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | |
| 2 | CurrencyCode | USD | CD | валюта строки | |
| 3 | ServiceName | ОТСУТСТВУЕТ | CO | наименование услуги | решение оператора |
| 4 | TaxRate | 0 | CD | ставка налога | НДС 0% |
| 5 | TaxSum | 0.00 | CD | сумма налога | |
| 6 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | |
| 7 | ServiceCost_Currency | USD | CD | валюта стоимости | |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### ServiceDescription_2
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | |
| 2 | CurrencyCode | USD | CD | валюта строки | |
| 3 | ServiceName | ОТСУТСТВУЕТ | CO | наименование услуги | решение оператора |
| 4 | TaxRate | 0 | CD | ставка налога | |
| 5 | TaxSum | 0.00 | CD | сумма налога | |
| 6 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | |
| 7 | ServiceCost_Currency | USD | CD | валюта стоимости | |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### Итого, по массиву ServiceDescription:
- `array_elements`: 2
- `item_fields`: всего полей 14 из 14
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 37 из 37
- `doc_formalization_status`: confirmed

### `document`: Insurance Document
  - `uqi_prefix`: formalized.insurance_document_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\Счет_№26-00378-tl_1_от_14-01-2026.md
  - `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | DocumentCode | 04111 | CO | код вида документа | решение оператора |
| 2 | DocumentHead_DocumentName | Счет на оплату №26-00378-tl/1 от 14.01.2026 г. | CD | наименование документа | |
| 3 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | |
| 4 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | |
| 5 | DocumentBody_TextSection_TextPara | link:md\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | текст | решение оператора: link |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

### `document`: TechDescription
  - `uqi_prefix`: formalized.tech_description_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\техничка Антикот, антипыльца антимошка.md
  - `file_name`: техничка Антикот, антипыльца антимошка.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | DocumentCode | 05999 | CO | код вида документа | решение оператора |
| 2 | DocumentHead_DocumentName | Технические характеристики | CD | наименование | |
| 3 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата | решение оператора |
| 4 | DocumentHead_DocumentNumber | Б/Н | CO | номер | решение оператора |
| 5 | DocumentBody_TextSection_TextPara | link:md\техничка Антикот, антипыльца антимошка.md | CD | текст | |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

## non_formalized

### `document`: Storage Report (ДО-1)
  - `uqi_prefix`: non_formalized.svh_1
  - `path`: md\ДО 14431420260204161621.md
  - `file_name`: ДО 14431420260204161621.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | number | 0000080 | CD | № ДО-1 | |
| 2 | date | 03.02.2026 | CD | дата ДО-1 | |
| 3 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии СВХ | |
| 4 | warehouse_license_date | 18.09.2025 | CD | дата лицензии СВХ | |
| 5 | actual_gross_weight | 3500 | CD | фактический вес брутто | из доп.листа; решение оператора |
| 6 | actual_places | 127 | CD | фактическое кол-во мест | из доп.листа; решение оператора |
| 7 | transport_reg_number | O157AO774 / BT374974 | CD | номер ТС | |

#### goods_1
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | tnved | 7019900095 | CD | код ТН ВЭД | |
| 2 | places | 27 | CD | кол-во мест | |
| 3 | gross_weight_kg | 1710 | CD | вес брутто | |
| 4 | cost | 42228 | CD | стоимость | |
| 5 | currency_code | CNY | CD | валюта | |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### goods_2
| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | tnved | 5804101000 | CD | код ТН ВЭД | |
| 2 | places | 100 | CD | кол-во мест | |
| 3 | gross_weight_kg | 1790 | CD | вес брутто | |
| 4 | cost | 55032 | CD | стоимость | |
| 5 | currency_code | CNY | CD | валюта | |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву goods:
- `array_elements`: 2
- `item_fields`: всего полей 10 из 10
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 8 из 8

### `document`: Storage Report Additional Sheet
  - `uqi_prefix`: non_formalized.svh_additional_sheet_1
  - `path`: md\ДО доп 14431520260204161645.md
  - `file_name`: ДО доп 14431520260204161645.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | number | 1 | CD | № доп.листа | |
| 2 | date | 03.02.2026 | CD | дата доп.листа | |
| 3 | actual_gross_weight | 3500 | CD | фактический вес | |
| 4 | actual_places | 127 | CD | фактическое кол-во мест | |
| 5 | transport_reg_number | O157AO774 / BT374974 | CO | номер ТС | решение оператора: из основного ДО-1 |
| 6 | svh_address_region | Республика Татарстан | CD | регион СВХ | из CMR п.3; решение оператора |
| 7 | svh_address_city | Набережные Челны | CD | город СВХ | из CMR п.3 |
| 8 | svh_address_street_house | Производственный пр-д, д.45 | CD | улица/дом СВХ | из CMR п.3 |
| 9 | svh_customs_code | 10404083 | CO | код таможенного органа | решение оператора: из ТД |

#### Итого, по документу:
- `doc_fields`: 9 из 9

### `document`: Master Data
  - `uqi_prefix`: non_formalized.master_data_1
  - `path`: stable_source\

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | declarant_name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ «СКИФ» | CD | наименование декларанта | ЕГРЮЛ |
| 2 | declarant_ogrn | 1201600020390 | CD | ОГРН | ЕГРЮЛ |
| 3 | declarant_inn | 1650389298 | CD | ИНН | ЕГРЮЛ |
| 4 | declarant_kpp | 165001001 | CD | КПП | ЕГРЮЛ |
| 5 | declarant_address_postal_code | 423800 | CD | индекс | ЕГРЮЛ |
| 6 | declarant_address_country_code | RU | CD | страна alpha-2 | ЕГРЮЛ |
| 7 | declarant_address_country_name | РОССИЯ | CD | страна текст | ЕГРЮЛ |
| 8 | declarant_address_region | РЕСПУБЛИКА ТАТАРСТАН (ТАТАРСТАН) | CD | регион | ЕГРЮЛ |
| 9 | declarant_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | ЕГРЮЛ |
| 10 | declarant_address_street | ПР-Д ХЛЕБНЫЙ | CD | улица | ЕГРЮЛ |
| 11 | declarant_address_building | 30 | CD | дом | ЕГРЮЛ |
| 12 | declarant_address_room | 211 | CD | офис | ЕГРЮЛ |
| 13 | declarant_phone | +7 937 779-26-56 | CD | телефон | контракт |
| 14 | declarant_email | | CO | e-mail | решение оператора: пусто |
| 15 | representative_last_name | АРБУЗОВА | CD | фамилия представителя | Паспорт/Доверенность |
| 16 | representative_first_name | АНАСТАСИЯ | CD | имя | Паспорт/Доверенность |
| 17 | representative_middle_name | КОНСТАНТИНОВНА | CD | отчество | Паспорт/Доверенность |
| 18 | representative_position | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | должность | Доверенность |
| 19 | representative_phone | +7-927-030-70-07 | CD | телефон | Доверенность |
| 20 | representative_email | | CO | e-mail | решение оператора: пусто |
| 21 | representative_passport_code | RU01001 | CD | код документа | Доверенность |
| 22 | representative_passport_name | ПАСРФ | CD | наименование документа | Доверенность |
| 23 | representative_passport_series | 63 09 | CD | серия | Паспорт |
| 24 | representative_passport_number | 449948 | CD | номер | Паспорт |
| 25 | representative_passport_date | 2010-03-11 | CD | дата выдачи | Паспорт |
| 26 | representative_passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | Паспорт |
| 27 | representative_authority_doc_name | ДОВЕРЕННОСТЬ | CD | наименование док-та полномочий | Доверенность |
| 28 | representative_authority_doc_number | 1 | CD | № доверенности | Доверенность |
| 29 | representative_authority_doc_date_from | 2026-02-01 | CD | дата начала | Доверенность |
| 30 | representative_authority_doc_date_to | 2026-12-31 | CD | дата окончания | Доверенность |
| 31 | note | Источники: Passport_63_09_449948.xml, LetterOfAttorney_1.xml, FreeDoc_ЮЭ9965-25-106893283.xml (ЕГРЮЛ), FreeDoc_КООО_26651_М.xml (договор перевозки) | CD | пояснения | |

#### Итого, по документу:
- `doc_fields`: 31 из 31

### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.td_1
  - `path`: md\ТД 10719110_240126_5011363_reg00378тд.md
  - `file_name`: ТД 10719110_240126_5011363_reg00378тд.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 1 | number | 10719110/240126/5011363 | CD | номер ТД | |
| 2 | date | 24.01.2026 | CD | дата ТД | |
| 3 | customs_post_code | 10404083 | CD | код таможенного органа | |
| 4 | customs_post_name | ОТО И ТК №3 Т/П НАБЕРЕЖНОЧЕЛНИНСКИЙ | CD | наименование таможенного органа | |
| 5 | transport_reg_number | О157АО774/ВТ374974 | CD | ТС по ТД | |

#### Итого, по документу:
- `doc_fields`: 5 из 5

### Итого, по файлу:
- `total_fields`: формализуемые: contract(25)+supplementary(31)+invoice(52)+packing(37)+cmr(42)+payment_order_1(17)+payment_order_2(17)+service_invoice(37)+insurance(5)+tech_description(5)=268; неформализуемые: svh(8)+svh_additional(9)+master_data(31)+td(5)=53; всего: 321
- `formalization_status`: confirmed

## Нерешенные вопросы (Issues)

Все вопросы решены оператором. Pending отсутствуют.
