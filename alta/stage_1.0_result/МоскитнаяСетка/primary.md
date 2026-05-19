# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 поставка / 2 товара (по ДО/ТД)
- `источники данных:` md + operator_provided_data + stable_source

## 2. formalized:

### document: Contract
- `uqi_prefix`: formalized.contract_1
- `xml_target_root`: AltaE2CONT
- `path`: md\SALES CONTRACT NoLM-2553.md
- `file_name`: SALES CONTRACT NoLM-2553.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 03011 | CD | код вида документа | derived: константа |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта | from: md/SALES CONTRACT NoLM-2553.md (semantic block 01) |
| 03 | ContractRegistration_PrDocumentDate | 02.07.2025 | CD | дата контракта | from: md/SALES CONTRACT NoLM-2553.md (semantic block 02) |
| 04 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта | operator: formalized.contract_1.ContractTerms_Amount (решение оператора; см. доп. соглашение №1) |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator: formalized.contract_1.currency_code_numeric |
| 06 | ContractTerms_LastDate | 31.12.2026 | CD | срок действия/исполнения | from: md/SALES CONTRACT NoLM-2553.md (END OF PERIOD) |
| 07 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки / Incoterms | operator: formalized.contract_1.delivery_terms |
| 08 | ContractTerms_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md | CD | текст контракта | link per schema |
| 09 | ContractTerms_DealSign | 1 | CO | системный признак Альты | operator: formalized.contract_1.deal_sign |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта | from: md/SALES CONTRACT NoLM-2553.md (semantic block 07) |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 | operator: formalized.contract_1.foreign_person_country_code_alpha2 |
| 12 | ForeignPerson_Address_CounryName | China | CD | страна продавца, текст | from: md/SALES CONTRACT NoLM-2553.md (semantic block 09) |
| 13 | ForeignPerson_Address_Region | Hebei | CD | регион/область продавца | from: md/SALES CONTRACT NoLM-2553.md (semantic block 10) |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца | from: md/SALES CONTRACT NoLM-2553.md (semantic block 11) |
| 15 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом продавца одной строкой | operator: chat |
| 16 | RussianPerson_OrganizationName | ООО «СКИФ» | CD | покупатель/сторона контракта | from: md/SALES CONTRACT NoLM-2553.md (semantic block 13) |
| 17 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН покупателя | master_data: stable_source/FreeDoc_ЮЭ9965-25-106893283.xml |
| 18 | RussianPerson_INN | 1650389298 | CD | ИНН покупателя | from: md/SALES CONTRACT NoLM-2553.md (semantic block 15) |
| 19 | RussianPerson_KPP | 165001001 | CD | КПП покупателя | from: md/SALES CONTRACT NoLM-2553.md (semantic block 16) |
| 20 | RussianPerson_Address_PostalCode | 423800 | CD | индекс покупателя | from: md/SALES CONTRACT NoLM-2553.md (semantic block 17) |
| 21 | RussianPerson_Address_CountryCode | RU | CD | страна покупателя alpha-2 | from: md/SALES CONTRACT NoLM-2553.md (semantic block 18) |
| 22 | RussianPerson_Address_CounryName | Russia | CD | страна покупателя, текст | from: md/SALES CONTRACT NoLM-2553.md (semantic block 19) |
| 23 | RussianPerson_Address_Region | Республика Татарстан | CD | регион покупателя | from: md/SALES CONTRACT NoLM-2553.md (semantic block 20) |
| 24 | RussianPerson_Address_City | Набережные Челны | CD | город покупателя | from: md/SALES CONTRACT NoLM-2553.md (semantic block 21) |
| 25 | RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator: chat (нормализация по ЕГРЮЛ/CMR) |
| 26 | doc_code | 03011 | CD | код документа | derived |
| 27 | doc_name | КОНТРАКТ | CD | наименование документа | derived |
| 28 | doc_number | LM-2553 | CD | номер документа | derived: =ContractRegistration_PrDocumentNumber |
| 29 | doc_date | 02.07.2025 | CD | дата документа | derived: =ContractRegistration_PrDocumentDate |

#### Итого, по документу:
- `doc_fields`: 29 из 29
- `doc_formalization_status`: confirmed

### document: Supplementary Contract
- `uqi_prefix`: formalized.supplementary_contract_1
- `xml_target_root`: AltaSupplementaryContract
- `path`: md\1 Supplementary agreement to the contract.md
- `file_name`: 1 Supplementary agreement to the contract.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения | from: md/1 Supplementary agreement... (semantic block 01) |
| 02 | IssueDate | 25.11.2025 | CD | дата доп. соглашения | from: md/1 Supplementary agreement... (semantic block 02) |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта | from: md/1 Supplementary agreement... (semantic block 03) |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator: formalized.supplementary_contract_1.currency_code_numeric |
| 05 | ContractDescription_LastDate | 31.12.2026 | CO | новый срок действия/исполнения | operator: formalized.supplementary_contract_1.expiry_date |
| 06 | ContractDescription_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md | CD | текст доп. соглашения | link per schema |
| 07 | ContractDescription_DealSign | 1 | CO | системный признак Альты | operator: formalized.supplementary_contract_1.deal_sign |
| 08 | ContractDescription_StockCategorySign | 0 | CO | системный признак Альты | operator: formalized.supplementary_contract_1.stock_category_sign |
| 09 | ContractDescription_BuyerLimitationSign | 0 | CO | системный признак Альты | operator: formalized.supplementary_contract_1.buyer_limitation_sign |
| 10 | ContractDescription_InsuranceSign | 0 | CO | системный признак Альты | operator: formalized.supplementary_contract_1.insurance_sign |
| 11 | RussianPerson_OrganizationName | ООО «СКИФ» | CD | российская сторона; покупатель | copied_from: formalized.contract_1.RussianPerson_OrganizationName (same supply) |
| 12 | RussianPerson_ShortName | ООО "СКИФ" | CO | краткое наименование | operator: chat |
| 13 | RussianPerson_OGRN | 1201600020390 | CD | ОГРН | master_data: stable_source/FreeDoc_ЮЭ9965-25-106893283.xml |
| 14 | RussianPerson_INN | 1650389298 | CD | ИНН | copied_from: formalized.contract_1.RussianPerson_INN |
| 15 | RussianPerson_KPP | 165001001 | CD | КПП | copied_from: formalized.contract_1.RussianPerson_KPP |
| 16 | RussianPerson_Address_PostalCode | 423800 | CD | индекс | copied_from: formalized.contract_1.RussianPerson_Address_PostalCode |
| 17 | RussianPerson_Address_CountryCode | RU | CD | страна alpha-2 | copied_from: formalized.contract_1.RussianPerson_Address_CountryCode |
| 18 | RussianPerson_Address_CounryName | Russia | CD | страна, текст | copied_from: formalized.contract_1.RussianPerson_Address_CounryName |
| 19 | RussianPerson_Address_Region | Республика Татарстан | CD | регион | copied_from: formalized.contract_1.RussianPerson_Address_Region |
| 20 | RussianPerson_Address_City | Набережные Челны | CD | город | copied_from: formalized.contract_1.RussianPerson_Address_City |
| 21 | RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом одной строкой | operator: chat (нормализация по ЕГРЮЛ/CMR) |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | иностранная сторона; продавец | from: md/1 Supplementary agreement... (text) |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator: formalized.supplementary_contract_1.foreign_person_short_name_equals_full=true |
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 | operator: formalized.supplementary_contract_1.foreign_person_country_code_alpha2 |
| 25 | ForeignPerson_Address_CounryName | China | CD | страна, текст | copied_from: formalized.contract_1.ForeignPerson_Address_CounryName |
| 26 | ForeignPerson_Address_Region | Hebei | CO | регион | operator: formalized.supplementary_contract_1.foreign_person_address_from_contract=true |
| 27 | ForeignPerson_Address_City | Shijiazhuang | CO | город | operator: formalized.supplementary_contract_1.foreign_person_address_from_contract=true |
| 28 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом | operator: chat |
| 29 | PersonSurname | Li | CO | фамилия подписанта | operator: formalized.supplementary_contract_1.signed_person_surname |
| 30 | PersonName | Jing | CO | имя подписанта | operator: formalized.supplementary_contract_1.signed_person_name |
| 31 | PersonMiddleName |  | CO | отчество подписанта | operator: formalized.supplementary_contract_1.signed_person_middle_name |
| 32 | doc_code | 03012 | CD | код документа | derived |
| 33 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наименование документа | derived |
| 34 | doc_number | 1 | CD | номер документа | derived: =DocumentNumber |
| 35 | doc_date | 25.11.2025 | CD | дата документа | derived: =IssueDate |

#### Итого, по документу:
- `doc_fields`: 35 из 35
- `doc_formalization_status`: confirmed

### document: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: md/CL на сетку .md
- `file_name`: CL на сетку .md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты | operator: formalized.invoice_1.exchange_rate |
| 02 | CurrencyCode | CNY | CO | валюта инвойса | operator: formalized.invoice_1.currency_code |
| 03 | DocumentCode | 04021 | CD | код вида документа | derived: константа |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест | from: md/CL на сетку .md (Qty/BG) |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator: formalized.invoice_1.places_description |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто | operator: formalized.invoice_1.total_gross_weight (из PL) |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто | operator: formalized.invoice_1.total_net_weight (из PL) |
| 08 | GCost | 97260.00 | CO | системное поле Альты | operator: formalized.invoice_1.gcost (=TotalCost) |
| 09 | TotalCost | 97260.00 | CO | итого по инвойсу | operator: formalized.invoice_1.total_cost (из инвойса) |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | from: md/CL на сетку .md |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator: formalized.invoice_1.delivery_terms_numeric |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | from: md/CL на сетку .md |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | operator: formalized.invoice_1.dispatch_country_code |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | operator: formalized.invoice_1.trading_country_code |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | operator: formalized.invoice_1.destination_country_code |
| 16 | Registration_PrDocumentName | Коммерческий инвойс | CD | наименование документа | from: md/CL на сетку .md |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | from: md/CL на сетку .md |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | from: md/CL на сетку .md |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | from: md/CL на сетку .md |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | from: md/CL на сетку .md |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | copied_from: formalized.contract_1.RussianPerson_INN |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | copied_from: formalized.contract_1.RussianPerson_KPP |
| 23 | Buyer_Name | ООО «СКИФ» | CD | наименование покупателя | copied_from: formalized.contract_1.RussianPerson_OrganizationName |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | copied_from: formalized.contract_1.RussianPerson_Address_PostalCode |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | copied_from: formalized.contract_1.RussianPerson_Address_CountryCode |
| 26 | Buyer_PostalAddress_CounryName | Russia | CD | страна покупателя, текст | copied_from: formalized.contract_1.RussianPerson_Address_CounryName |
| 27 | Buyer_PostalAddress_Region | Республика Татарстан | CD | регион | copied_from: formalized.contract_1.RussianPerson_Address_Region |
| 28 | Buyer_PostalAddress_City | Набережные Челны | CD | город | copied_from: formalized.contract_1.RussianPerson_Address_City |
| 29 | Buyer_PostalAddress_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator: chat |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец | copied_from: formalized.contract_1.ForeignPerson_OrganizationName |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | operator: formalized.invoice_1.seller_country_code_alpha2 |
| 32 | Seler_PostalAddress_CounryName | China | CD | страна продавца, текст | copied_from: formalized.contract_1.ForeignPerson_Address_CounryName |
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | copied_from: formalized.contract_1.ForeignPerson_Address_Region |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город/район продавца | copied_from: formalized.contract_1.ForeignPerson_Address_City |
| 35 | Seler_PostalAddress_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом одной строкой | operator: chat |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | грузоотправитель | copied_from: formalized.invoice_1.Seler_Name (normalisation: consignor=seller) |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | copied_from: formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 38 | Consignor_Address_CounryName | China | CD | страна грузоотправителя, текст | copied_from: formalized.invoice_1.Seler_PostalAddress_CounryName |
| 39 | Consignor_Address_Region | Hebei | CD | регион | copied_from: formalized.invoice_1.Seler_PostalAddress_Region |
| 40 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from: formalized.invoice_1.Seler_PostalAddress_City |
| 41 | Consignor_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CD | улица/дом одной строкой | copied_from: formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 42 | Consignee_OrganizationName | ООО «СКИФ» | CD | грузополучатель | copied_from: formalized.invoice_1.Buyer_Name (normalisation: consignee=buyer) |
| 43 | Consignee_OGRN | 1201600020390 | CD | ОГРН | copied_from: formalized.contract_1.RussianPerson_OGRN |
| 44 | Consignee_INN | 1650389298 | CD | ИНН | copied_from: formalized.invoice_1.Buyer_CompanyID |
| 45 | Consignee_KPP | 165001001 | CD | КПП | copied_from: formalized.invoice_1.Buyer_KPPCode |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | copied_from: formalized.invoice_1.Buyer_PostalAddress_PostalCode |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | copied_from: formalized.invoice_1.Buyer_PostalAddress_CountryCode |
| 48 | Consignee_Address_CounryName | Russia | CD | страна, текст | copied_from: formalized.invoice_1.Buyer_PostalAddress_CounryName |
| 49 | Consignee_Address_Region | Республика Татарстан | CD | регион | copied_from: formalized.invoice_1.Buyer_PostalAddress_Region |
| 50 | Consignee_Address_City | Набережные Челны | CD | город | copied_from: formalized.invoice_1.Buyer_PostalAddress_City |
| 51 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CD | улица/дом/офис одной строкой | copied_from: formalized.invoice_1.Buyer_PostalAddress_StreetHouse |
| 52 | InvoiceGoods[] | | | массив товарных позиций | 7 элементов |

#### 52: InvoiceGoods[]

#### InvoiceGoods_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL на сетку .md |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара | from: md/CL на сетку .md |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | from: md/CL на сетку .md |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | from: md/CL на сетку .md |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit code 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit code 055 |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_1.gross_weight (из PL) |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator: formalized.invoice_1.goods_1.net_weight (из PL) |
| 09 | Price | 5.85 | CD | цена за единицу | from: md/CL на сетку .md (Price per M2) |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | from: md/CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.4 * 30 | CD | модель/модификация | from: md/CL на сетку .md (derived) |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_2
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL на сетку .md |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара | from: md/CL на сетку .md |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | from: md/CL на сетку .md |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | from: md/CL на сетку .md |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit code 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit code 055 |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_2.gross_weight (из PL) |
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator: formalized.invoice_1.goods_2.net_weight (из PL) |
| 09 | Price | 5.85 | CD | цена за единицу | from: md/CL на сетку .md (Price per M2) |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | from: md/CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.6 * 30 | CD | модель/модификация | from: md/CL на сетку .md (derived) |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_3
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL на сетку .md |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы «Антипыльца» из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | CD | описание товара | from: md/CL на сетку .md |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | from: md/CL на сетку .md |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | from: md/CL на сетку .md |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit code 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit code 055 |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_3.gross_weight (из PL) |
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator: formalized.invoice_1.goods_3.net_weight (из PL) |
| 09 | Price | 6.35 | CD | цена за единицу | from: md/CL на сетку .md (Price per M2) |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | from: md/CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.4 * 30 | CD | модель/модификация | from: md/CL на сетку .md (derived) |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_4
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL на сетку .md |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CD | описание товара | from: md/CL на сетку .md |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | from: md/CL на сетку .md |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | from: md/CL на сетку .md |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit code 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit code 055 |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_4.gross_weight (из PL) |
| 08 | NetWeightQuantity | 144.00 | CO | нетто по строке | operator: formalized.invoice_1.goods_4.net_weight (из PL) |
| 09 | Price | 6.35 | CD | цена за единицу | from: md/CL на сетку .md (Price per M2) |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | from: md/CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.6 * 30 | CD | модель/модификация | from: md/CL на сетку .md (derived) |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_5
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | from: md/CL на сетку .md |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара | from: md/CL на сетку .md |
| 03 | GoodsQuantity | 90 | CD | кол-во по строке инвойса | from: md/CL на сетку .md |
| 04 | goods_supplementary_quantity | 3780 | CD | количество в доп.ед.изм | from: md/CL на сетку .md |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit code 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit code 055 |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_5.gross_weight (из PL) |
| 08 | NetWeightQuantity | 491.40 | CO | нетто по строке | operator: formalized.invoice_1.goods_5.net_weight (из PL) |
| 09 | Price | 3.4 | CD | цена за единицу | from: md/CL на сетку .md (Price per M2) |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | from: md/CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.4 * 30 | CD | модель/модификация | from: md/CL на сетку .md (derived) |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_6
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | from: md/CL на сетку .md |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА «Антимошка» из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара | from: md/CL на сетку .md |
| 03 | GoodsQuantity | 180 | CD | кол-во по строке инвойса | from: md/CL на сетку .md |
| 04 | goods_supplementary_quantity | 8640 | CD | количество в доп.ед.изм | from: md/CL на сетку .md |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit code 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit code 055 |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_6.gross_weight (из PL) |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто по строке | operator: formalized.invoice_1.goods_6.net_weight (из PL) |
| 09 | Price | 3.4 | CD | цена за единицу | from: md/CL на сетку .md (Price per M2) |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | from: md/CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.6 * 30 | CD | модель/модификация | from: md/CL на сетку .md (derived) |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_7
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL на сетку .md |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки «Антипыльца» из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | from: md/CL на сетку .md |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке инвойса | from: md/CL на сетку .md |
| 04 | goods_supplementary_quantity | 240 | CD | количество в доп.ед.изм | from: md/CL на сетку .md |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | cb:unit code 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | cb:unit code 055 |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_7.gross_weight (из PL) |
| 08 | NetWeightQuantity | 24.00 | CO | нетто по строке | operator: formalized.invoice_1.goods_7.net_weight (из PL) |
| 09 | Price | 28 | CD | цена за единицу | from: md/CL на сетку .md (Price per M2) |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | from: md/CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | Roll size 1.6 * 30 | CD | модель/модификация | from: md/CL на сетку .md (derived) |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 105 из 15 * 7
- `array_status`: confirmed

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 53 | doc_code | 04021 | CD | код документа | derived |
| 54 | doc_name | ИНВОЙС | CD | наименование документа | derived |
| 55 | doc_number | LM-2591 | CD | номер документа | derived: =Registration_PrDocumentNumber |
| 56 | doc_date | 30.10.2025 | CD | дата документа | derived: =Registration_PrDocumentDate |

#### Итого, по документу:
- `doc_fields`: 56 из 56
- `doc_formalization_status`: confirmed

### document: Packing List
- `uqi_prefix`: formalized.packing_list_1
- `xml_target_root`: AltaE2PACK
- `path`: md/PL на сетку .md
- `file_name`: PL на сетку .md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто | from: md/PL на сетку .md (totals) |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто | from: md/PL на сетку .md (totals) |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | грузоотправитель | copied_from: formalized.invoice_1.Seler_Name |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator: formalized.packing_list_1.consignor_shortname_equals_full=true |
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | operator: formalized.packing_list_1.consignor_country_code_alpha2 |
| 06 | Consignor_Address_CounryName | China | CD | страна грузоотправителя, текст | copied_from: formalized.invoice_1.Seler_PostalAddress_CounryName |
| 07 | Consignor_Address_Region | Hebei | CD | регион | copied_from: formalized.invoice_1.Seler_PostalAddress_Region |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from: formalized.invoice_1.Seler_PostalAddress_City |
| 09 | Consignor_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CD | улица/дом одной строкой | copied_from: formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 10 | Consignee_OrganizationName | ООО «СКИФ» | CD | грузополучатель | copied_from: formalized.invoice_1.Buyer_Name |
| 11 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator: formalized.packing_list_1.consignee_shortname_equals_full=true |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | copied_from: formalized.invoice_1.Consignee_OGRN |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | copied_from: formalized.invoice_1.Buyer_CompanyID |
| 14 | Consignee_KPP | 165001001 | CD | КПП | copied_from: formalized.invoice_1.Buyer_KPPCode |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | copied_from: formalized.invoice_1.Buyer_PostalAddress_PostalCode |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | copied_from: formalized.invoice_1.Buyer_PostalAddress_CountryCode |
| 17 | Consignee_Address_CounryName | Russia | CD | страна, текст | copied_from: formalized.invoice_1.Buyer_PostalAddress_CounryName |
| 18 | Consignee_Address_Region | Республика Татарстан | CD | регион | copied_from: formalized.invoice_1.Buyer_PostalAddress_Region |
| 19 | Consignee_Address_City | Набережные Челны | CD | город | copied_from: formalized.invoice_1.Buyer_PostalAddress_City |
| 20 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CD | улица/дом/офис одной строкой | copied_from: formalized.invoice_1.Buyer_PostalAddress_StreetHouse |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | from: md/PL на сетку .md |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий поставки | copied_from: formalized.invoice_1.DeliveryTerms_DeliveryTermsNumericCode |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | from: md/PL на сетку .md |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | наименование контракта | operator: formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentName |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | from: md/PL на сетку .md |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | from: md/PL на сетку .md |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | derived |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | from: md/PL на сетку .md |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | from: md/PL на сетку .md |
| 30 | DeliveryTerms_Registration_PrDocumentName | Упаковочный лист | CO | наименование упаковочного | operator: formalized.packing_list_1.registration_doc_name |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | operator: formalized.packing_list_1.registration_doc_number |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | operator: formalized.packing_list_1.registration_doc_date |
| 33 | Goods[] | | | грузовые/упаковочные строки | 7 элементов |

#### 33: Goods[]

#### Goods_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки | from: md/PL на сетку .md |
| 02 | GoodsQuantity | 60 | CD | кол-во мест в строке | from: md/PL на сетку .md |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | from: md/PL на сетку .md |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | from: md/PL на сетку .md |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок в упаковке | operator: formalized.packing_list_1.goods_1.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_2
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки | from: md/PL на сетку .md |
| 02 | GoodsQuantity | 30 | CD | кол-во мест в строке | from: md/PL на сетку .md |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто по строке | from: md/PL на сетку .md |
| 04 | NetWeightQuantity | 460.80 | CD | нетто по строке | from: md/PL на сетку .md |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок в упаковке | operator: formalized.packing_list_1.goods_2.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_3
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера . Размер рулона 1,42*0,64*0,22 | CD | описание строки | from: md/PL на сетку .md |
| 02 | GoodsQuantity | 60 | CD | кол-во мест в строке | from: md/PL на сетку .md |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто по строке | from: md/PL на сетку .md |
| 04 | NetWeightQuantity | 252.00 | CD | нетто по строке | from: md/PL на сетку .md |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок в упаковке | operator: formalized.packing_list_1.goods_3.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_4
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание строки | from: md/PL на сетку .md |
| 02 | GoodsQuantity | 30 | CD | кол-во мест в строке | from: md/PL на сетку .md |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто по строке | from: md/PL на сетку .md |
| 04 | NetWeightQuantity | 144.00 | CD | нетто по строке | from: md/PL на сетку .md |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок в упаковке | operator: formalized.packing_list_1.goods_4.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_5
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА " Антимошка " из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание строки | from: md/PL на сетку .md |
| 02 | GoodsQuantity | 90 | CD | кол-во мест в строке | from: md/PL на сетку .md |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто по строке | from: md/PL на сетку .md |
| 04 | NetWeightQuantity | 491.40 | CD | нетто по строке | from: md/PL на сетку .md |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок в упаковке | operator: formalized.packing_list_1.goods_5.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_6
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание строки | from: md/PL на сетку .md |
| 02 | GoodsQuantity | 180 | CD | кол-во мест в строке | from: md/PL на сетку .md |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | from: md/PL на сетку .md |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто по строке | from: md/PL на сетку .md |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок в упаковке | operator: formalized.packing_list_1.goods_6.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_7
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки "Антипыльца " из полиэстера Размер рулона 1,72*0,35*0,31*1 | CD | описание строки | from: md/PL на сетку .md |
| 02 | GoodsQuantity | 5 | CD | кол-во мест в строке | from: md/PL на сетку .md |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто по строке | from: md/PL на сетку .md |
| 04 | NetWeightQuantity | 24.00 | CD | нетто по строке | from: md/PL на сетку .md |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок в упаковке | operator: formalized.packing_list_1.goods_7.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 35 из 5 * 7
- `array_status`: confirmed

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 34 | TransportMeans[] | | | транспортные средства | 2 элемента |

#### 34: TransportMeans[]

#### TransportMeans_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | О157АО774 | CO | регистрационный номер | operator: formalized.packing_list_1.transport_1.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator: formalized.packing_list_1.transport_1.mode_code |
| 03 | NationalityCode | 000 | CO | код национальности ТС | operator: formalized.packing_list_1.transport_1.nationality_code |
| 04 | MoverIndicator | true | CO | признак тягача | operator: formalized.packing_list_1.transport_1.mover_indicator |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### TransportMeans_2
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | ВТ374974 | CO | регистрационный номер | operator: formalized.packing_list_1.transport_2.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator: formalized.packing_list_1.transport_2.mode_code |
| 03 | NationalityCode | 000 | CO | код национальности ТС | operator: formalized.packing_list_1.transport_2.nationality_code |
| 04 | MoverIndicator | false | CO | признак тягача | operator: formalized.packing_list_1.transport_2.mover_indicator |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 8 из 4 * 2
- `array_status`: confirmed

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 35 | doc_code | 04131 | CD | код документа | derived |
| 36 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | derived |
| 37 | doc_number | LM-2591 | CO | номер документа | operator: =DeliveryTerms_Registration_PrDocumentNumber |
| 38 | doc_date | 30.10.2025 | CO | дата документа | operator: =DeliveryTerms_Registration_PrDocumentDate |

#### Итого, по документу:
- `doc_fields`: 38 из 38
- `doc_formalization_status`: confirmed

### document: CMR
- `uqi_prefix`: formalized.cmr_1
- `xml_target_root`: AltaE3CMR
- `path`: md/СМР от СВХ.md
- `file_name`: СМР от СВХ.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CO | язык документа | operator: formalized.cmr_1.language_code |
| 02 | CMR_Choice | 1 | CO | системный выбор Альты | operator: formalized.cmr_1.cmr_choice |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | from: md/СМР от СВХ.md |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | from: md/СМР от СВХ.md |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления | operator: formalized.cmr_1.registration_place |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата принятия груза | from: md/СМР от СВХ.md (taking over) |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2 | operator: formalized.cmr_1.taking_cargo_country_code_alpha2 |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | Китай | CD | страна принятия груза, текст | from: md/СМР от СВХ.md |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | operator: formalized.cmr_1.delivery_country_code_alpha2 |
| 10 | DeliveryPlace_CounryName | Россия | CD | страна доставки, текст | from: md/СМР от СВХ.md |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки по Incoterms | operator: formalized.cmr_1.delivery_terms_place |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator: formalized.cmr_1.delivery_terms_string |
| 13 | GoodsQuantity | 127 | CD | общее количество мест по CMR | from: md/СМР от СВХ.md |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто по CMR | from: md/СМР от СВХ.md |
| 15 | CMRTransport_PrimeMoverStateSignID | O157AO774 | CD | гос. номер тягача | from: md/СМР от СВХ.md |
| 16 | CMRTransport_TrailerStateSignID | BT374974 | CD | гос. номер прицепа | from: md/СМР от СВХ.md |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование отправителя | from: md/СМР от СВХ.md |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator: formalized.cmr_1.consignor_shortname_equals_full=true |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна alpha-2 | copied_from: formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 20 | Consignor_Address_CounryName | China | CD | страна, текст | copied_from: formalized.invoice_1.Seler_PostalAddress_CounryName |
| 21 | Consignor_Address_Region | Hebei | CD | регион | copied_from: formalized.invoice_1.Seler_PostalAddress_Region |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from: formalized.invoice_1.Seler_PostalAddress_City |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street, Shijiazhuang, Hebei, China | CD | улица/дом одной строкой | copied_from: formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | наименование гаранта | operator: formalized.cmr_1.consignor_guarantee_all |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое наименование | operator: formalized.cmr_1.consignor_guarantee_all |
| 26 | Consignor_Guarantee_Address_CountryCode | | CO | страна alpha-2 | operator: formalized.cmr_1.consignor_guarantee_all |
| 27 | Consignor_Guarantee_Address_CounryName | | CO | страна, текст | operator: formalized.cmr_1.consignor_guarantee_all |
| 28 | Consignor_Guarantee_Address_Region | | CO | регион | operator: formalized.cmr_1.consignor_guarantee_all |
| 29 | Consignor_Guarantee_Address_City | | CO | город/район | operator: formalized.cmr_1.consignor_guarantee_all |
| 30 | Consignor_Guarantee_Address_StreetHouse | | CO | улица/дом одной строкой | operator: formalized.cmr_1.consignor_guarantee_all |
| 31 | Consignee_NameInf | ООО «Скиф» | CD | наименование получателя | from: md/СМР от СВХ.md |
| 32 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator: formalized.cmr_1.consignee_shortname_equals_full=true |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | operator: formalized.cmr_1.consignee_ogrn_from_master_data |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | from: md/СМР от СВХ.md |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | from: md/СМР от СВХ.md |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | from: md/СМР от СВХ.md |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | copied_from: formalized.invoice_1.Consignee_Address_CountryCode |
| 38 | Consignee_Address_CounryName | Россия | CD | страна, текст | from: md/СМР от СВХ.md |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | from: md/СМР от СВХ.md |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | from: md/СМР от СВХ.md |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом/офис одной строкой | from: md/СМР от СВХ.md |
| 42 | CMRGoods[] | | | товарные/грузовые строки | 1 элемент |

#### 42: CMRGoods[]

#### CMRGoods_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CD | описание груза/товара | copied_from: non_formalized.svh_1 (исключение схемы) |
| 03 | PakingQuantity | 127 | CO | кол-во упаковок/мест | operator: formalized.cmr_1.goods_1.packing_quantity |

#### Итого, по элементу массива:
- `item_fields`: 3 из 3

#### Итого, по массиву:
- `array_elements`: 1
- `item_fields`: всего полей 3 из 3 * 1
- `array_status`: confirmed

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 43 | doc_code | 02015 | CD | код документа | derived |
| 44 | doc_name | CMR | CD | наименование документа | derived |
| 45 | doc_number | 00378 | CD | номер документа | derived: =RegistrationDocument_RegID |
| 46 | doc_date | 20.01.2026 | CD | дата документа | derived: =RegistrationDocument_DateInf |

#### Итого, по документу:
- `doc_fields`: 46 из 46
- `doc_formalization_status`: confirmed


### document: Payment Order (1)
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: md/currency_transfer_7_28.11.2025.md
- `file_name`: currency_transfer_7_28.11.2025.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator: formalized.payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator: formalized.payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 400000.00 | CD | сумма платежа | from: md/currency_transfer_7_28.11.2025.md (рублевый эквивалент обеспечения) |
| 04 | TransactionKind | 01 | CO | вид операции | operator: formalized.payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator: formalized.payment_order_all.priority |
| 06 | Purpose | Заявление на перевод валюты №7 от 28.11.2025. Перевод собственных средств в юанях CNY 36154.20 по контракту LM-2553 от 02.07.2025 | CD | назначение платежа | from: md/currency_transfer_7_28.11.2025.md (смысловой перевод) |
| 07 | ValueSpelledOut | Четыреста тысяч рублей 00 копеек | CD | сумма прописью | from: md/currency_transfer_7_28.11.2025.md |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | from: md/currency_transfer_7_28.11.2025.md |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | from: md/currency_transfer_7_28.11.2025.md |
| 10 | Payer_OrganizationName | ООО «СКИФ» | CD | плательщик | copied_from: formalized.contract_1.RussianPerson_OrganizationName |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from: formalized.contract_1.RussianPerson_INN |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator: formalized.payment_order_1.payer_kpp |
| 13 | Payer_Bank_BankName | Филиал «Центральный» Банка ВТБ (ПАО) в г. Москве, БИК 044525411 | CD | банк плательщика | from: md/currency_transfer_7_28.11.2025.md (сверка с реквизитами контракта) |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | copied_from: formalized.contract_1.ForeignPerson_OrganizationName |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SWIFT: VTBRCNSH, A/C: 40807156900610036383 | CD | банк получателя | from: md/currency_transfer_7_28.11.2025.md |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator: formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator: formalized.payment_order_all.payer_sign.name |

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 18 | doc_code | 04023 | CD | код документа | derived |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | derived |
| 20 | doc_number | 7 | CD | номер документа | derived: =DocumentReference_PrDocumentNumber |
| 21 | doc_date | 28.11.2025 | CD | дата документа | derived: =DocumentReference_PrDocumentDate |

#### Итого, по документу:
- `doc_fields`: 21 из 21
- `doc_formalization_status`: confirmed


### document: Payment Order (2)
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `path`: md/currency_transfer_1_13.01.2026.md
- `file_name`: currency_transfer_1_13.01.2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator: formalized.payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator: formalized.payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 67260.00 | CD | сумма платежа | from: md/currency_transfer_1_13.01.2026.md (оплата остатка по инвойсу) |
| 04 | TransactionKind | 01 | CO | вид операции | operator: formalized.payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator: formalized.payment_order_all.priority |
| 06 | Purpose | Заявление на перевод валюты №1 от 13.01.2026. Перевод собственных средств в юанях CNY 61105.80 (оплата по инвойсу LM-2591) по контракту LM-2553 от 02.07.2025 | CD | назначение платежа | from: md/currency_transfer_1_13.01.2026.md (смысловой перевод) |
| 07 | ValueSpelledOut | Шестьдесят семь тысяч двести шестьдесят юаней 00 фэней | CD | сумма прописью | from: md/currency_transfer_1_13.01.2026.md |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | from: md/currency_transfer_1_13.01.2026.md |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | from: md/currency_transfer_1_13.01.2026.md |
| 10 | Payer_OrganizationName | ООО «СКИФ» | CD | плательщик | copied_from: formalized.contract_1.RussianPerson_OrganizationName |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from: formalized.contract_1.RussianPerson_INN |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator: formalized.payment_order_2.payer_kpp |
| 13 | Payer_Bank_BankName | Филиал «Центральный» Банка ВТБ (ПАО) в г. Москве, БИК 044525411 | CD | банк плательщика | from: md/currency_transfer_1_13.01.2026.md |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | copied_from: formalized.contract_1.ForeignPerson_OrganizationName |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SWIFT: VTBRCNSH, A/C: 40807156900610036383 | CD | банк получателя | from: md/currency_transfer_1_13.01.2026.md |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator: formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator: formalized.payment_order_all.payer_sign.name |

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 18 | doc_code | 04023 | CD | код документа | derived |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | derived |
| 20 | doc_number | 1 | CD | номер документа | derived: =DocumentReference_PrDocumentNumber |
| 21 | doc_date | 13.01.2026 | CD | дата документа | derived: =DocumentReference_PrDocumentDate |

#### Итого, по документу:
- `doc_fields`: 21 из 21
- `doc_formalization_status`: confirmed

### document: Service Invoice
- `uqi_prefix`: formalized.service_invoice_1
- `xml_target_root`: AltaServiceInvoice
- `path`: md/Счет_№26-00378-tl_от_27-01-2026.md
- `file_name`: Счет_№26-00378-tl_от_27-01-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак документа | operator: formalized.service_invoice_1.document_sign |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 03 | Currency | USD | CD | валюта итого | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг/перевозчик | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 05 | BankName | АО "Райффайзенбанк", БИК 044525700, Сч. № 30101810200000000700 | CD | банк исполнителя | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 08 | PrDocumentNumber | ОТСУТСТВУЕТ | CO | номер связанного заказа | operator: formalized.service_invoice_1.payment_document_number |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CO | дата связанного заказа | operator: formalized.service_invoice_1.payment_document_date |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | грузоотправитель | operator: formalized.service_invoice_1.consignor_decision=seller |
| 14 | PostalCode | | CO | индекс грузоотправителя | operator: service_invoice_1.consignor_postalcode_empty_ok |
| 15 | CountryCode | CN | CO | страна alpha-2 | operator: service_invoice_1.consignor_address_from_seller |
| 16 | CounryName | China | CO | страна, текст | operator: service_invoice_1.consignor_address_from_seller |
| 17 | Region | Hebei | CO | регион | operator: service_invoice_1.consignor_address_from_seller |
| 18 | Town | Shijiazhuang | CO | город/район | operator: service_invoice_1.consignor_address_from_seller |
| 19 | StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом одной строкой | operator: service_invoice_1.consignor_address_from_seller |
| 20 | Consignee_OrganizationName | ООО "Скиф" (ООО "СКИФ") | CD | грузополучатель | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator: formalized.service_invoice_1.consignee_ogrn_from_master_data |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 24 | PostalCode | 423800 | CD | индекс грузополучателя | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 25 | CountryCode | RU | CD | страна alpha-2 | copied_from: formalized.invoice_1.Consignee_Address_CountryCode |
| 26 | CounryName | Россия | CD | страна, текст | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 27 | Region | Республика Татарстан | CD | регион | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 28 | Town | Набережные Челны | CD | город | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 29 | StreetHouse | проезд Хлебный | CD | улица | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 30 | House | 30 | CO | дом | operator: formalized.service_invoice_1.consignee_house |
| 31 | Room | 211 | CO | офис/кв | operator: formalized.service_invoice_1.consignee_room |
| 32 | Signature_Choice | 2 | CD | вариант подписи | from: md/Счет_№26-00378-tl_от_27-01-2026.md (руководитель/бухгалтер) |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | отстутствует по Signature_Choice=2 |
| 34 | IndividualEntrepreneur_PersonName | | CD | инициалы ИП | отстутствует по Signature_Choice=2 |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | отстутствует по Signature_Choice=2 |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климовин | CD | фамилия руководителя | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л | CD | имя руководителя | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А | CD | отчество руководителя | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О | CD | имя бухгалтера | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А | CD | отчество бухгалтера | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 42 | ServiceDescription[] | | | услуги | 2 элемента |

#### 42: ServiceDescription[]

#### ServiceDescription_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) - перевозка автотранспортом | CD | описание услуги | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 02 | CurrencyCode | USD | CD | валюта строки | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator: formalized.service_invoice_1.service_1.service_name |
| 04 | TaxRate | 0% | CD | ставка налога | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 05 | TaxSum | 0.00 | CD | сумма налога | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | from: md/Счет_№26-00378-tl_от_27-01-2026.md |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### ServiceDescription_2
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 02 | CurrencyCode | USD | CD | валюта строки | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator: formalized.service_invoice_1.service_2.service_name |
| 04 | TaxRate | 0% | CD | ставка налога | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 05 | TaxSum | 0.00 | CD | сумма налога | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | from: md/Счет_№26-00378-tl_от_27-01-2026.md |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | from: md/Счет_№26-00378-tl_от_27-01-2026.md |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 14 из 7 * 2
- `array_status`: confirmed

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 43 | doc_code | 04031 | CD | код документа | derived |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | derived |
| 45 | doc_number | 26-00378-tl | CD | номер документа | derived: =Registration_PrDocumentNumber |
| 46 | doc_date | 27.01.2026 | CD | дата документа | derived: =Registration_PrDocumentDate |
| 47 | transport_to_border | 1404.00 | CO | стоимость до границы | operator: formalized.service_invoice_1.transport_to_border |
| 48 | transport_currency | USD | CD | валюта | derived: =ServiceDescription_1.ServiceCost_Currency |

#### Итого, по документу:
- `doc_fields`: 48 из 48
- `doc_formalization_status`: confirmed


### document: Insurance Services Invoice
- `uqi_prefix`: formalized.insurance_document_1
- `xml_target_root`: AltaFreeDoc
- `path`: md/Счет_№26-00378-tl_1_от_14-01-2026.md
- `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | derived: константа |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа | from: md/Счет_№26-00378-tl_1_от_14-01-2026.md |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | from: md/Счет_№26-00378-tl_1_от_14-01-2026.md |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | from: md/Счет_№26-00378-tl_1_от_14-01-2026.md |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | основной текст/условия | operator: formalized.insurance_document_1.textpara_storage |
| 06 | doc_code | 04111 | CD | код документа | derived |
| 07 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | derived |
| 08 | doc_number | 26-00378-tl/1 | CD | номер документа | derived: =DocumentHead_DocumentNumber |
| 09 | doc_date | 14.01.2026 | CD | дата документа | derived: =DocumentHead_DocumentDate |
| 10 | insurance_to_border | 910.34 | CO | стоимость страхования | operator: formalized.insurance_document_1.insurance_to_border |
| 11 | insurance_currency | RUB | CD | валюта страхования | from: md/Счет_№26-00378-tl_1_от_14-01-2026.md |

#### Итого, по документу:
- `doc_fields`: 11 из 11
- `doc_formalization_status`: confirmed


### document: TechDescription
- `uqi_prefix`: formalized.tech_description_1
- `xml_target_root`: AltaFreeDoc
- `path`: md/техничка Антикот, антипыльца антимошка .md
- `file_name`: техничка Антикот, антипыльца антимошка .md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | derived: константа |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование техописания | from: md/техничка Антикот, антипыльца антимошка .md |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата техописания | operator: formalized.tech_description_1.date |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания | operator: formalized.tech_description_1.number |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md | CD | технический текст | link per schema |
| 06 | doc_code | 05999 | CD | код документа | derived |
| 07 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | derived |
| 08 | doc_number | Б/Н | CO | номер документа | derived: =DocumentHead_DocumentNumber |
| 09 | doc_date | 30.10.2025 | CO | дата документа | derived: =DocumentHead_DocumentDate |

#### Итого, по документу:
- `doc_fields`: 9 из 9
- `doc_formalization_status`: confirmed

## 3. non_formalized:

### document: Storage Report
- `uqi_prefix`: non_formalized.svh_1
- `path`: md/ДО 14431420260204161621.md
- `file_name`: ДО 14431420260204161621.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии СВХ | from: md/ДО 14431420260204161621.md |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии СВХ | from: md/ДО 14431420260204161621.md |
| 03 | actual_gross_weight | 3500.00 | CO | фактический вес по весам | operator: non_formalized.svh_1.actual_totals_from_svh_additional_sheet |
| 04 | actual_places | 127 | CO | фактическое количество мест | operator: non_formalized.svh_1.actual_totals_from_svh_additional_sheet |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС при въезде | from: md/ДО 14431420260204161621.md |
| 06 | goods[] | | | товары по строкам ДО | 2 элемента |

#### 06: goods[]

#### goods_1
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 7019900095 | CD | код товара | from: md/ДО 14431420260204161621.md |
| 02 | places | 27 | CD | кол-во грузовых мест | from: md/ДО 14431420260204161621.md |
| 03 | gross_weight_kg | 1710 | CD | вес брутто | from: md/ДО 14431420260204161621.md |
| 04 | cost | 42228 | CD | стоимость | from: md/ДО 14431420260204161621.md |
| 05 | currency_code | CNY | CD | буквенный код валюты | from: md/ДО 14431420260204161621.md |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### goods_2
| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара | from: md/ДО 14431420260204161621.md |
| 02 | places | 100 | CD | кол-во грузовых мест | from: md/ДО 14431420260204161621.md |
| 03 | gross_weight_kg | 1790 | CD | вес брутто | from: md/ДО 14431420260204161621.md |
| 04 | cost | 55032 | CD | стоимость | from: md/ДО 14431420260204161621.md |
| 05 | currency_code | CNY | CD | буквенный код валюты | from: md/ДО 14431420260204161621.md |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 10 из 5 * 2
- `array_status`: confirmed

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 07 | doc_code | 10061 | CD | код документа | derived: ДО-1 |
| 08 | doc_name | ОТЧЕТ ДО-1 | CD | наименование документа | derived |
| 09 | doc_number | 0000080 | CO | номер документа | operator: non_formalized.svh_1.number |
| 10 | doc_date | 03.02.2026 | CO | дата документа | operator: non_formalized.svh_1.date |

#### Итого, по документу:
- `doc_fields`: 10 из 10
- `doc_formalization_status`: confirmed


### document: Storage Report Additional Sheet
- `uqi_prefix`: non_formalized.svh_additional_sheet_1
- `path`: md/ДО доп 14431520260204161645.md
- `file_name`: ДО доп 14431520260204161645.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 1 | CD | № доп.листа | from: md/ДО доп 14431520260204161645.md |
| 02 | date | 03.02.2026 | CD | дата доп.листа | from: md/ДО доп 14431520260204161645.md (derived) |
| 03 | actual_gross_weight | 3500 | CD | фактический вес брутто | from: md/ДО доп 14431520260204161645.md (totals) |
| 04 | actual_places | 127 | CD | фактическое количество мест | from: md/ДО доп 14431520260204161645.md (totals) |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС при въезде | copied_from: non_formalized.svh_1.transport_reg_number |
| 06 | svh_address_region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион СВХ | operator: non_formalized.svh_additional_sheet_1.address_from_cmr |
| 07 | svh_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город СВХ | operator: non_formalized.svh_additional_sheet_1.address_from_cmr |
| 08 | svh_address_street_house | ПРОИЗВОДСТВЕННЫЙ ПРОЕЗД, Д. 45 | CO | улица/дом СВХ | operator: non_formalized.svh_additional_sheet_1.address_from_cmr |
| 09 | svh_customs_code | 10404083 | CD | код таможенного поста СВХ | from: md/СМР от СВХ.md (instructions) |

#### Итого, по документу:
- `doc_fields`: 9 из 9
- `doc_formalization_status`: confirmed


### document: Transit Declaration
- `uqi_prefix`: non_formalized.td_1
- `path`: md/ТД 10719110_240126_5011363_reg00378тд.md
- `file_name`: ТД 10719110_240126_5011363_reg00378тд.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 10719110/240126/5011363 | CO | номер ТД | operator: non_formalized.transit_declaration_1.number |
| 02 | date | 24.01.2026 | CO | дата ТД | operator: non_formalized.transit_declaration_1.date |
| 03 | customs_post_code | 10719110 | CD | код таможни отправления | from: md/ТД 10719110_240126_5011363_reg00378тд.md |
| 04 | customs_post_name | ОТСУТСТВУЕТ | CD | наименование таможни | отсутствует в ТД (только код) |
| 05 | transport_reg_number | O157AO774/BT374974 | CD | ТС по ТД | from: md/ТД 10719110_240126_5011363_reg00378тд.md |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed


### document: Stable Data
- `uqi_prefix`: non_formalized.stable_data_1
- `path`: stable_source/stable_data.md
- `file_name`: stable_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | declarant_organization_name | ООО "СКИФ" | CD | наименование декларанта | from: stable_source/stable_data.md |
| 02 | declarant_phone | +7 (843) 207 18 90 | CD | телефон декларанта | from: stable_source/stable_data.md |
| 03 | declarant_email | PROM_TAT@MAIL.RU | CD | email декларанта | from: stable_source/stable_data.md |
| 04 | representative_name | Анастасия Константиновна Арбузова | CD | ФИО представителя | from: stable_source/stable_data.md |
| 05 | representative_phone | +7 927-222-0500 | CD | телефон представителя | from: stable_source/stable_data.md |
| 06 | representative_email | A.K.ARBUZOVA@YANDEX.RU | CD | email представителя | from: stable_source/stable_data.md |

#### Итого, по документу:
- `doc_fields`: 6 из 6
- `doc_formalization_status`: confirmed


## 4. Итого, по файлу:

- `total_unreliable_fields`: 0
- `total_doc_fields`: 314
- `total_fields`: 441
- `formalization_status`: confirmed


## 5. Нерешенные вопросы (Issues)

- Нет. Все вопросы разрешены решениями оператора.


## 6. unreliable_fields:

- Нет.
