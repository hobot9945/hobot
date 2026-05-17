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
|---:|---|---|---|---|---|
| 01 | DocumentCode | 03011 | CD | код вида документа | derived: константа |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта | from: md/SALES CONTRACT NoLM-2553.md (semantic block 01) |
| 03 | ContractRegistration_PrDocumentDate | 02.07.2025 | CD | дата контракта | from: md/SALES CONTRACT NoLM-2553.md (semantic block 02) |
| 04 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта | operator: formalized.contract_1.ContractTerms_Amount (решение оператора; см. доп. соглашение №1) |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator: formalized.contract_1.currency_code_numeric |
| 06 | ContractTerms_LastDate | 31.12.2026 | CD | срок действия/исполнения | from: md/SALES CONTRACT NoLM-2553.md (END OF PERIOD) |
| 07 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки / Incoterms | operator: formalized.contract_1.delivery_terms |
| 08 | ContractTerms_ContractText | link:md\\SALES CONTRACT NoLM-2553.md | CD | текст контракта | link per schema |
| 09 | ContractTerms_DealSign | 1 | CO | системный признак Альты | operator: formalized.contract_1.deal_sign |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта | from: md/SALES CONTRACT NoLM-2553.md (semantic block 07) |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 | operator: formalized.contract_1.foreign_person_country_code_alpha2 |
| 12 | ForeignPerson_Address_CounryName | China | CD | страна продавца, текст | from: md/SALES CONTRACT NoLM-2553.md (semantic block 09) |
| 13 | ForeignPerson_Address_Region | Hebei | CD | регион/область продавца | from: md/SALES CONTRACT NoLM-2553.md (semantic block 10) |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца | from: md/SALES CONTRACT NoLM-2553.md (semantic block 11) |
| 15 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом продавца одной строкой | operator: chat |
| 16 | RussianPerson_OrganizationName | ООО «СКИФ» | CD | покупатель/сторона контракта | from: md/SALES CONTRACT NoLM-2553.md (semantic block 13) |
| 17 | RussianPerson_OGRN | 1201600020390 | CD | ОГРН покупателя | master_data: stable_source/FreeDoc_ЮЭ9965-25-106893283.xml |
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
- `doc_formalization_status`: pending

### document: Supplementary Contract
- `uqi_prefix`: formalized.supplementary_contract_1
- `xml_target_root`: AltaSupplementaryContract
- `path`: md\1 Supplementary agreement to the contract.md
- `file_name`: 1 Supplementary agreement to the contract.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения | from: md/1 Supplementary agreement... (semantic block 01) |
| 02 | IssueDate | 25.11.2025 | CD | дата доп. соглашения | from: md/1 Supplementary agreement... (semantic block 02) |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта | from: md/1 Supplementary agreement... (semantic block 03) |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator: formalized.supplementary_contract_1.currency_code_numeric |
| 05 | ContractDescription_LastDate | 31.12.2026 | CO | новый срок действия/исполнения | operator: formalized.supplementary_contract_1.expiry_date |
| 06 | ContractDescription_ContractText | link:md\\1 Supplementary agreement to the contract.md | CD | текст доп. соглашения | link per schema |
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
- `doc_formalization_status`: pending

### document: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: md\CL на сетку .md
- `file_name`: CL на сетку .md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты | operator: formalized.invoice_1.exchange_rate |
| 02 | CurrencyCode | CNY | CO | валюта инвойса ISO 4217 alpha-3 | operator: formalized.invoice_1.currency_code (в md указано RMB) |
| 03 | DocumentCode | 04021 | CD | код вида документа | derived: константа |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест | from: md/CL... (Qty/BG 127) |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator: formalized.invoice_1.places_description |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто | operator: formalized.invoice_1.total_gross_weight (из PL totals) |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто | operator: formalized.invoice_1.total_net_weight (из PL totals) |
| 08 | GCost | 97260.00 | CO | системное поле Альты | operator: formalized.invoice_1.gcost |
| 09 | TotalCost | 97260.00 | CO | итого по инвойсу | operator: formalized.invoice_1.total_cost |
| 10 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CD | место поставки | from: md/CL... (Delivery place) |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator: formalized.invoice_1.delivery_terms_numeric |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator: formalized.invoice_1.delivery_terms_string |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | operator: formalized.invoice_1.dispatch_country_code |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | operator: formalized.invoice_1.trading_country_code |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | operator: formalized.invoice_1.destination_country_code |
| 16 | Registration_PrDocumentName | Commercial invoice | CD | наименование документа | normalized from md title |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | from: md/CL... (INVOICE NUMBER) |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | from: md/CL... (DATE) |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | from: md/CL... |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | from: md/CL... |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | from: stable_source/FreeDoc_ЮЭ... + совпадает с md/CMR |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | copied_from: stable_source/FreeDoc_ЮЭ... |
| 23 | Buyer_Name | ООО "СКИФ" | CD | наименование покупателя | master_data: stable_source/FreeDoc_ЮЭ... |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | master_data: stable_source/FreeDoc_ЮЭ... |
| 25 | Buyer_PostalAddress_CountryCode | RU | CO | страна покупателя alpha-2 | operator implied by decisions/questions (см. DOC_CONVERSION_REVIEW) |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя, текст | master_data: stable_source/FreeDoc_ЮЭ... |
| 27 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data: stable_source/FreeDoc_ЮЭ... |
| 28 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data: stable_source/FreeDoc_ЮЭ... |
| 29 | Buyer_PostalAddress_StreetHouse | ПР-Д ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом | master_data: stable_source/FreeDoc_ЮЭ... |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | продавец | from: md/CL... |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | operator: formalized.invoice_1.seller_country_code_alpha2 |
| 32 | Seler_PostalAddress_CounryName | China | CD | страна продавца, текст | from: md/CL... |
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | from: md/CL... |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город продавца | from: md/CL... |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом продавца | from: md/CL... |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | нормализация: consignor=seller (schema rule) |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | нормализация: consignor=seller |
| 38 | Consignor_Address_CounryName | China | CD | страна грузоотправителя, текст | нормализация: consignor=seller |
| 39 | Consignor_Address_Region | Hebei | CD | регион | нормализация: consignor=seller |
| 40 | Consignor_Address_City | Shijiazhuang | CD | город | нормализация: consignor=seller |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | нормализация: consignor=seller |
| 42 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | operator: formalized.invoice_1.consignee_equals_buyer=true |
| 43 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data: stable_source/FreeDoc_ЮЭ... |
| 44 | Consignee_INN | 1650389298 | CD | ИНН | master_data |
| 45 | Consignee_KPP | 165001001 | CD | КПП | master_data |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | master_data |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | master_data |
| 48 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | master_data |
| 49 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data |
| 50 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data |
| 51 | Consignee_Address_StreetHouse | ПР-Д ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_data |
| 52 | InvoiceGoods_[n] |  | CD | товарные позиции | 7 элементов, см. ниже |

#### 52: InvoiceGoods[]

#### InvoiceGoods_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL... item 1 |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester | CD | описание товара | from: md/CL... item 1 |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке | from: md/CL... Q-ty Sets |
| 04 | goods_supplementary_quantity | 2520 | CD | кол-во в доп.ед.изм | from: md/CL... Quantity in M2 |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | from: md/CL... |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества (наименование для ДТ) | derived: cb:unit=055 (м²) |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_1.gross_weight (from packing list) |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator: formalized.invoice_1.goods_1.net_weight |
| 09 | Price | 5.85 | CD | цена за единицу | from: md/CL... |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | from: md/CL... |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator: formalized.invoice_1.goods_all.model |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL... item 2 |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 *30 | CD | описание товара | from: md/CL... |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке | from: md/CL... |
| 04 | goods_supplementary_quantity | 1440 | CD | доп.кол-во | from: md/CL... |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм | from: md/CL... |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. для ДТ | derived: cb:unit=055 (м²) |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто | operator: formalized.invoice_1.goods_2.gross_weight |
| 08 | NetWeightQuantity | 460.80 | CO | нетто | operator: formalized.invoice_1.goods_2.net_weight |
| 09 | Price | 5.85 | CD | цена | from: md/CL... |
| 10 | TotalCost | 8424.00 | CD | стоимость | from: md/CL... |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_3
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL... item 3 |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 | CD | описание товара | from: md/CL... |
| 03 | GoodsQuantity | 60 | CD | кол-во | from: md/CL... |
| 04 | goods_supplementary_quantity | 2520 | CD | доп.кол-во | from: md/CL... |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм | from: md/CL... |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм | derived: cb:unit=055 (м²) |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто | operator: formalized.invoice_1.goods_3.gross_weight |
| 08 | NetWeightQuantity | 252.00 | CO | нетто | operator: formalized.invoice_1.goods_3.net_weight |
| 09 | Price | 6.35 | CD | цена | from: md/CL... |
| 10 | TotalCost | 16002.00 | CD | стоимость | from: md/CL... |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_4
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL... item 4 |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2 | CD | описание | from: md/CL... |
| 03 | GoodsQuantity | 30 | CD | кол-во | from: md/CL... |
| 04 | goods_supplementary_quantity | 1440 | CD | доп.кол-во | from: md/CL... |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм | from: md/CL... |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм | derived: cb:unit=055 (м²) |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто | operator: formalized.invoice_1.goods_4.gross_weight |
| 08 | NetWeightQuantity | 144.00 | CO | нетто | operator: formalized.invoice_1.goods_4.net_weight |
| 09 | Price | 6.35 | CD | цена | from: md/CL... |
| 10 | TotalCost | 9144.00 | CD | стоимость | from: md/CL... |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_5
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | from: md/CL... item 5 |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 | CD | описание | from: md/CL... |
| 03 | GoodsQuantity | 90 | CD | кол-во | from: md/CL... |
| 04 | goods_supplementary_quantity | 3780 | CD | доп.кол-во | from: md/CL... |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм | from: md/CL... |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм | derived: cb:unit=055 (м²) |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто | operator: formalized.invoice_1.goods_5.gross_weight |
| 08 | NetWeightQuantity | 491.40 | CO | нетто | operator: formalized.invoice_1.goods_5.net_weight |
| 09 | Price | 3.40 | CD | цена | from: md/CL... |
| 10 | TotalCost | 12852.00 | CD | стоимость | from: md/CL... |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_6
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | from: md/CL... item 6 |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 | CD | описание | from: md/CL... |
| 03 | GoodsQuantity | 180 | CD | кол-во | from: md/CL... |
| 04 | goods_supplementary_quantity | 8640 | CD | доп.кол-во | from: md/CL... |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм | from: md/CL... |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм | derived: cb:unit=055 (м²) |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто | operator: formalized.invoice_1.goods_6.gross_weight |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто | operator: formalized.invoice_1.goods_6.net_weight |
| 09 | Price | 3.40 | CD | цена | from: md/CL... |
| 10 | TotalCost | 29376.00 | CD | стоимость | from: md/CL... |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_7
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | from: md/CL... item 7 |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 | CD | описание | from: md/CL... |
| 03 | GoodsQuantity | 5 | CD | кол-во | from: md/CL... |
| 04 | goods_supplementary_quantity | 240 | CD | доп.кол-во | from: md/CL... |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм | from: md/CL... |
| 06 | MeasureUnitQualifierName | м² | CD | ед.изм | derived: cb:unit=055 (м²) |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто | operator: formalized.invoice_1.goods_7.gross_weight |
| 08 | NetWeightQuantity | 24.00 | CO | нетто | operator: formalized.invoice_1.goods_7.net_weight |
| 09 | Price | 28.00 | CD | цена | from: md/CL... |
| 10 | TotalCost | 6720.00 | CD | стоимость | from: md/CL... |
| 11 | OriginCountryCode | 156 | CO | страна происхождения | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 105 из 15 * 7
- `array_status`: pending

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 53 | doc_code | 04021 | CD | код документа | derived |
| 54 | doc_name | ИНВОЙС | CD | наименование документа | derived |
| 55 | doc_number | LM-2591 | CD | номер документа | derived: =Registration_PrDocumentNumber |
| 56 | doc_date | 30.10.2025 | CD | дата документа | derived: =Registration_PrDocumentDate |

#### Итого, по документу:
- `doc_fields`: 56 из 56
- `doc_formalization_status`: pending

### document: Packing List
- `uqi_prefix`: formalized.packing_list_1
- `xml_target_root`: AltaE2PACK
- `path`: md\PL на сетку .md
- `file_name`: PL на сетку .md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто | from: md/PL... totals |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто | from: md/PL... totals |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | copied_from: formalized.invoice_1.Seler_Name |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | краткое наименование | operator: formalized.packing_list_1.consignor_shortname_equals_full=true |
| 05 | Consignor_Address_CountryCode | CN | CO | страна alpha-2 | operator: formalized.packing_list_1.consignor_country_code_alpha2 |
| 06 | Consignor_Address_CounryName | China | CD | страна, текст | copied_from: formalized.invoice_1.Seler_PostalAddress_CounryName |
| 07 | Consignor_Address_Region | Hebei | CD | регион | copied_from: formalized.invoice_1.Seler_PostalAddress_Region |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город | copied_from: formalized.invoice_1.Seler_PostalAddress_City |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | copied_from: formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 10 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | from: md/PL... |
| 11 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator: formalized.packing_list_1.consignee_shortname_equals_full=true |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data: stable_source/FreeDoc_ЮЭ... |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | master_data |
| 14 | Consignee_KPP | 165001001 | CD | КПП | master_data |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | master_data |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | master_data |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | master_data |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data |
| 20 | Consignee_Address_StreetHouse | ПР-Д ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_data |
| 21 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CD | место поставки | from: md/PL... |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий | operator: formalized.invoice_1.delivery_terms_numeric (used for PL too) |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | from: md/PL... |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | наименование контракта | operator: formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentName |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | from: md/PL... |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | from: md/PL... |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | derived: нормализация по типу документа |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | from: md/PL... |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | from: md/PL... |
| 30 | DeliveryTerms_Registration_PrDocumentName | Упаковочный лист | CO | наименование упаковочного | operator: formalized.packing_list_1.registration_doc_name |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | operator: formalized.packing_list_1.registration_doc_number |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | operator: formalized.packing_list_1.registration_doc_date |
| 33 | Goods_[n] |  | CD | грузовые/упаковочные строки | 7 элементов, см. ниже |

#### 33: Goods[]

#### Goods_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки | from: md/PL... item 1 |
| 02 | GoodsQuantity | 60 | CD | количество мест | from: md/PL... Qty BG |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | from: md/PL... |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | from: md/PL... |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок/мест в упаковке | operator: formalized.packing_list_1.goods_1.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание | from: md/PL... |
| 02 | GoodsQuantity | 30 | CD | мест | from: md/PL... |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто | from: md/PL... |
| 04 | NetWeightQuantity | 460.80 | CD | нетто | from: md/PL... |
| 05 | PakingQuantity | 30 | CO | упаковок | operator: formalized.packing_list_1.goods_2.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_3
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца | CD | описание | from: md/PL... |
| 02 | GoodsQuantity | 6 | CD | мест | from: md/PL... |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто | from: md/PL... |
| 04 | NetWeightQuantity | 252.00 | CD | нетто | from: md/PL... |
| 05 | PakingQuantity | 6 | CO | упаковок | operator: formalized.packing_list_1.goods_3.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_4
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца | CD | описание | from: md/PL... |
| 02 | GoodsQuantity | 3 | CD | мест | from: md/PL... |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто | from: md/PL... |
| 04 | NetWeightQuantity | 144.00 | CD | нетто | from: md/PL... |
| 05 | PakingQuantity | 3 | CO | упаковок | operator: formalized.packing_list_1.goods_4.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_5
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" | CD | описание | from: md/PL... |
| 02 | GoodsQuantity | 9 | CD | мест | from: md/PL... |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто | from: md/PL... |
| 04 | NetWeightQuantity | 491.40 | CD | нетто | from: md/PL... |
| 05 | PakingQuantity | 9 | CO | упаковок | operator: formalized.packing_list_1.goods_5.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_6
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" | CD | описание | from: md/PL... |
| 02 | GoodsQuantity | 18 | CD | мест | from: md/PL... |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто | from: md/PL... |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто | from: md/PL... |
| 05 | PakingQuantity | 18 | CO | упаковок | operator: formalized.packing_list_1.goods_6.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_7
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки "Антипыльца" | CD | описание | from: md/PL... |
| 02 | GoodsQuantity | 1 | CD | мест | from: md/PL... |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто | from: md/PL... |
| 04 | NetWeightQuantity | 24.00 | CD | нетто | from: md/PL... |
| 05 | PakingQuantity | 1 | CO | упаковок | operator: formalized.packing_list_1.goods_7.paking_quantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 35 из 5 * 7
- `array_status`: confirmed

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 34 | TransportMeans_[n] |  | CD | транспорт | 2 элемента, см. ниже |

#### 34: TransportMeans[]

#### TransportMeans_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | Number | О157АО774 | CO | регистрационный номер | operator: formalized.packing_list_1.transport_1.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator: formalized.packing_list_1.transport_1.mode_code |
| 03 | NationalityCode | 000 | CO | код “национальности” | operator: formalized.packing_list_1.transport_1.nationality_code |
| 04 | MoverIndicator | true | CO | тягач/прицеп | operator: formalized.packing_list_1.transport_1.mover_indicator |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### TransportMeans_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | Number | ВТ374974 | CO | регистрационный номер | operator: formalized.packing_list_1.transport_2.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator: formalized.packing_list_1.transport_2.mode_code |
| 03 | NationalityCode | 000 | CO | код “национальности” | operator: formalized.packing_list_1.transport_2.nationality_code |
| 04 | MoverIndicator | false | CO | тягач/прицеп | operator: formalized.packing_list_1.transport_2.mover_indicator |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 8 из 4 * 2
- `array_status`: confirmed

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 35 | doc_code | 04131 | CD | код документа | derived |
| 36 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | derived |
| 37 | doc_number | LM-2591 | CO | номер документа | operator: formalized.packing_list_1.registration_doc_number |
| 38 | doc_date | 30.10.2025 | CO | дата документа | operator: formalized.packing_list_1.registration_doc_date |

#### Итого, по документу:
- `doc_fields`: 38 из 38
- `doc_formalization_status`: pending

### document: CMR
- `uqi_prefix`: formalized.cmr_1
- `xml_target_root`: AltaE3CMR
- `path`: md\СМР от СВХ.md
- `file_name`: СМР от СВХ.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | LanguageCode | RU | CO | язык документа | operator: formalized.cmr_1.language_code |
| 02 | CMR_Choice | 1 | CO | системный выбор/вариант Альты | operator: formalized.cmr_1.cmr_choice |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | from: md/СМР... (CMR №) |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CO | дата CMR | operator decision in chat: использовать дату принятия груза (см. md: taking over date) |
| 05 | RegistrationDocument_Place | Манчжурия | CO | место составления | operator: formalized.cmr_1.registration_place |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата принятия груза | from: md/СМР... |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2 | operator: formalized.cmr_1.taking_cargo_country_code_alpha2 |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | Китай | CD | страна принятия груза, текст | from: md/СМР... |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | operator: formalized.cmr_1.delivery_country_code_alpha2 |
| 10 | DeliveryPlace_CounryName | Россия | CD | страна доставки, текст | from: md/СМР... |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки по Incoterms | operator: formalized.cmr_1.delivery_terms_place |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator: formalized.cmr_1.delivery_terms_string |
| 13 | GoodsQuantity | 127 | CD | общее количество мест | from: md/СМР... |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто | from: md/СМР... |
| 15 | CMRTransport_PrimeMoverStateSignID | O157AO774 | CD | гос. номер тягача | from: md/СМР... |
| 16 | CMRTransport_TrailerStateSignID | BT374974 | CD | гос. номер прицепа | from: md/СМР... |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | отправитель: наименование | from: md/СМР... |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | отправитель: краткое наименование | operator: formalized.cmr_1.consignor_shortname_equals_full=true |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна отправителя alpha-2 | derived from text "China" |
| 20 | Consignor_Address_CounryName | China | CD | страна отправителя, текст | from: md/СМР... |
| 21 | Consignor_Address_Region | Hebei | CD | регион | from: md/СМР... |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город | from: md/СМР... |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | from: md/СМР... |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | гарант отправителя | operator: formalized.cmr_1.consignor_guarantee_all |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | гарант отправителя | operator: formalized.cmr_1.consignor_guarantee_all |
| 26 | Consignor_Guarantee_Address_CountryCode |  | CO | страна | operator: formalized.cmr_1.consignor_guarantee_all |
| 27 | Consignor_Guarantee_Address_CounryName |  | CO | страна, текст | operator: formalized.cmr_1.consignor_guarantee_all |
| 28 | Consignor_Guarantee_Address_Region |  | CO | регион | operator: formalized.cmr_1.consignor_guarantee_all |
| 29 | Consignor_Guarantee_Address_City |  | CO | город | operator: formalized.cmr_1.consignor_guarantee_all |
| 30 | Consignor_Guarantee_Address_StreetHouse |  | CO | улица/дом | operator: formalized.cmr_1.consignor_guarantee_all |
| 31 | Consignee_NameInf | ООО "СКИФ" | CO | получатель: наименование | operator decision: consignee=ООО СКИФ (Наб.Челны) |
| 32 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator: formalized.cmr_1.consignee_shortname_equals_full=true |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | operator: formalized.cmr_1.consignee_ogrn_from_master_data=true (master_data) |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | from: md/СМР... (Consignee p.2) |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | from: md/СМР... (Consignee p.2) |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | from: md/СМР... (Consignee p.2) |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | from: md/СМР... ("Россия") |
| 38 | Consignee_Address_CounryName | Россия | CD | страна, текст | from: md/СМР... |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | from: md/СМР... |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | from: md/СМР... |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом/офис | from: md/СМР... |
| 42 | CMRGoods_[n] |  | CD | грузовые строки | 1 элемент (по правилу отсутствия детализации) |

#### 42: CMRGoods[]

#### CMRGoods_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 | CO | описание груза | operator: chat |
| 03 | PakingQuantity | 127 | CO | кол-во упаковок/мест | operator: formalized.cmr_1.goods_1.packing_quantity |

#### Итого, по элементу массива:
- `item_fields`: 3 из 3

#### Итого, по массиву:
- `array_elements`: 1
- `item_fields`: всего полей 3 из 3 * 1
- `array_status`: pending

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 43 | doc_code | 02015 | CD | код документа | derived |
| 44 | doc_name | CMR | CD | наименование документа | derived |
| 45 | doc_number | 00378 | CD | номер документа | derived |
| 46 | doc_date | 20.01.2026 | CO | дата документа | operator decision |

#### Итого, по документу:
- `doc_fields`: 46 из 46
- `doc_formalization_status`: pending

### document: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: md\currency_transfer_7_28.11.2025.md
- `file_name`: currency_transfer_7_28.11.2025.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator: formalized.payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator: formalized.payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | from: md/currency_transfer_7... (34.041,00) |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator: formalized.payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator: formalized.payment_order_all.priority |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | from: md/currency_transfer_7... |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | from: md/currency_transfer_7... |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | from: md/currency_transfer_7... |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | from: md/currency_transfer_7... |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | from: md/currency_transfer_7... |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | from: md/currency_transfer_7... |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator: formalized.payment_order_1.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО)) | CD | банк плательщика | operator: реквизиты банка плательщика не требуются |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD; account 40807156900610036383 | CD | получатель | from: md/currency_transfer_7... |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | банк получателя | from: md/currency_transfer_7... |
| 16 | PersonSurname | Саранов | CO | подписант: фамилия | operator: formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | подписант: имя | operator: formalized.payment_order_all.payer_sign.name |
| 18 | doc_code | 04023 | CD | код документа | derived |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | derived |
| 20 | doc_number | 7 | CD | номер документа | derived |
| 21 | doc_date | 28.11.2025 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 21 из 21
- `doc_formalization_status`: pending

### document: Payment Order
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `path`: md\currency_transfer_1_13.01.2026.md
- `file_name`: currency_transfer_1_13.01.2026.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator: formalized.payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код | operator: formalized.payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | from: md/currency_transfer_1... (63.219,00) |
| 04 | TransactionKind | 01 | CO | вид операции | operator |
| 05 | Priority | 5 | CO | очередность | operator |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | from: md/currency_transfer_1... |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | from: md/currency_transfer_1... |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер | from: md/currency_transfer_1... |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата | from: md/currency_transfer_1... |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | from: md/currency_transfer_1... |
| 11 | Payer_INN | 1650389298 | CD | ИНН | from: md/currency_transfer_1... |
| 12 | Payer_KPP | 165001001 | CO | КПП | operator: formalized.payment_order_2.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО)); БИК 044525411 (частично) | CD | банк плательщика | operator: реквизиты банка плательщика не требуются |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD; account 40807156900610036383 | CD | получатель | from: md/currency_transfer_1... |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | банк получателя | from: md/currency_transfer_1... |
| 16 | PersonSurname | Саранов | CO | подписант: фамилия | operator |
| 17 | PersonName | Дмитрий | CO | подписант: имя | operator |
| 18 | doc_code | 04023 | CD | код | derived |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование | derived |
| 20 | doc_number | 1 | CD | номер документа | derived |
| 21 | doc_date | 13.01.2026 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 21 из 21
- `doc_formalization_status`: pending

### document: Service Invoice
- `uqi_prefix`: formalized.service_invoice_1
- `xml_target_root`: AltaServiceInvoice
- `path`: md\Счет_№26-00378-tl_от_27-01-2026.md
- `file_name`: Счет_№26-00378-tl_от_27-01-2026.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак документа | operator: formalized.service_invoice_1.document_sign |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | from: md/Счет_... totals |
| 03 | Currency | USD | CD | валюта итого | from: md/Счет_... |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг | from: md/Счет_... |
| 05 | BankName | АО "Райффайзенбанк"; БИК 044525700; Сч. № 30101810200000000700; Сч. № 40702810400000233463 | CD | банк исполнителя | from: md/Счет_... |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги | from: md/Счет_... |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги | from: md/Счет_... |
| 08 | PrDocumentNumber | 26-00378-tl | CD | связанный документ/заказ: номер | from: md/Счет_... |
| 09 | PrDocumentDate | 12.01.2026 | CD | связанный документ/заказ: дата | from: md/Счет_... |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | from: md/Счет_... |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | from: md/Счет_... |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | from: md/Счет_... |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | operator: formalized.service_invoice_1.consignor_decision=seller |
| 14 | PostalCode |  | CD | индекс | operator: оставлять пустым |
| 15 | CountryCode | CN | CD | страна alpha-2 | copied_from: formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 16 | CounryName | China | CD | страна, текст | copied_from |
| 17 | Region | Hebei | CD | регион | copied_from |
| 18 | Town | Shijiazhuang | CD | город | copied_from |
| 19 | StreetHouse | No. 5 Gaodong street | CD | улица/дом | copied_from |
| 20 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | from: md/Счет_... |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator: formalized.service_invoice_1.consignee_ogrn_from_master_data=true |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | from: md/Счет_... |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | from: md/Счет_... |
| 24 | PostalCode | 423800 | CD | индекс | from: md/Счет_... |
| 25 | CountryCode | RU | CD | страна alpha-2 | derived from text |
| 26 | CounryName | Россия | CD | страна, текст | from: md/Счет_... |
| 27 | Region | Республика Татарстан | CD | регион | from: md/Счет_... |
| 28 | Town | Набережные Челны | CD | город | from: md/Счет_... |
| 29 | StreetHouse | проезд Хлебный | CD | улица | from: md/Счет_... |
| 30 | House | 30 | CO | дом | operator: formalized.service_invoice_1.consignee_house |
| 31 | Room | 211 | CO | офис | operator: formalized.service_invoice_1.consignee_room |
| 32 | Signature_Choice | 2 | CD | вариант подписи | from: md/Счет_... |
| 33 | IndividualEntrepreneur_PersonSurname |  | CD | фамилия ИП | не заполняется при choice=2 |
| 34 | IndividualEntrepreneur_PersonName |  | CD | имя ИП | не заполняется при choice=2 |
| 35 | IndividualEntrepreneur_PersonMiddleName |  | CD | отчество ИП | не заполняется при choice=2 |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климовин | CD | фамилия руководителя | from: md/Счет_... |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CD | имя/инициал | from: md/Счет_... |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName |  | CD | отчество | не указано |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | from: md/Счет_... |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CD | имя/инициал | from: md/Счет_... |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName |  | CD | отчество | не указано |
| 42 | ServiceDescription_[n] |  | CD | услуги | 2 элемента |

#### 42: ServiceDescription[]

#### ServiceDescription_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении ... China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) | CD | описание услуги | from: md/Счет_... |
| 02 | CurrencyCode | USD | CD | валюта строки | from: md/Счет_... |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator: formalized.service_invoice_1.service_1.service_name |
| 04 | TaxRate | 0% | CD | ставка налога | from: md/Счет_... |
| 05 | TaxSum | 0.00 | CD | сумма налога | from: md/Счет_... |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | from: md/Счет_... |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | from: md/Счет_... |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### ServiceDescription_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ ... Набережные Челны | CD | описание услуги | from: md/Счет_... |
| 02 | CurrencyCode | USD | CD | валюта строки | from: md/Счет_... |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator: formalized.service_invoice_1.service_2.service_name |
| 04 | TaxRate | 0% | CD | ставка | from: md/Счет_... |
| 05 | TaxSum | 0.00 | CD | налог | from: md/Счет_... |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость | from: md/Счет_... |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | from: md/Счет_... |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 14 из 7 * 2
- `array_status`: confirmed

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 43 | doc_code | 04031 | CD | код документа | derived |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование | derived |
| 45 | doc_number | 26-00378-tl | CD | номер | derived |
| 46 | doc_date | 27.01.2026 | CD | дата | derived |
| 47 | transport_to_border | 1404.00 | CO | стоимость перевозки до границы | operator: до границы по услуге 1; после границы 1296.00 USD (для справки) |

#### Итого, по документу:
- `doc_fields`: 47 из 47
- `doc_formalization_status`: pending

### document: Insurance Services Invoice
- `uqi_prefix`: formalized.insurance_document_1
- `xml_target_root`: AltaFreeDoc
- `path`: md\Счет_№26-00378-tl_1_от_14-01-2026.md
- `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа | from: md/Счет_... |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | from: md/Счет_... |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | from: md/Счет_... |
| 05 | TextPara | link:md\\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | основной текст/условия | operator: formalized.insurance_document_1.textpara_storage=link |
| 06 | doc_code | 04111 | CD | код | derived |
| 07 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование | derived |
| 08 | doc_number | 26-00378-tl/1 | CD | номер | derived |
| 09 | doc_date | 14.01.2026 | CD | дата | derived |
| 10 | insurance_to_border | 910.34 | CO | стоимость страхования продавцом | operator: страхование до границы |

#### Итого, по документу:
- `doc_fields`: 10 из 10
- `doc_formalization_status`: pending

### document: TechDescription
- `uqi_prefix`: formalized.tech_description_1
- `xml_target_root`: AltaFreeDoc
- `path`: md\техничка Антикот, антипыльца антимошка .md
- `file_name`: техничка Антикот, антипыльца антимошка .md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | Технические характеристики — Сетки из полиэстера 5804101000 / Сетки из стекловолокна 7019900095 | CD | наименование | from: md/техничка... |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата техописания | operator: formalized.tech_description_1.date |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания | operator: formalized.tech_description_1.number |
| 05 | TextPara | link:md\\техничка Антикот, антипыльца антимошка .md | CD | технический текст | link per schema |
| 06 | doc_code | 05999 | CD | код | derived |
| 07 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование | derived |
| 08 | doc_number | Б/Н | CO | номер документа | operator |
| 09 | doc_date | 30.10.2025 | CO | дата документа | operator |

#### Итого, по документу:
- `doc_fields`: 9 из 9
- `doc_formalization_status`: confirmed

### document: Transport Contract
- `uqi_prefix`: formalized.transport_contract_1
- `xml_target_root`: AltaFreeDoc
- `path`: stable_source\FreeDoc_КООО_26651_М.xml
- `file_name`: FreeDoc_КООО_26651_М.xml

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04033 | CD | код вида документа | from xml |
| 02 | DocumentHead_DocumentName | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование договора | from xml |
| 03 | DocumentHead_DocumentDate | 13.05.2025 | CD | дата договора | from xml |
| 04 | DocumentHead_DocumentNumber | КООО/26651/М | CD | номер договора | from xml |
| 05 | TextPara | link:stable_source\\FreeDoc_КООО_26651_М.xml | CD | текст договора | link per schema |
| 06 | doc_code | 04033 | CD | код | derived |
| 07 | doc_name | ДОГОВОР ТРАНСПОРТНОЙ ЭКСПЕДИЦИИ | CD | наименование | derived |
| 08 | doc_number | КООО/26651/М | CD | номер | derived |
| 09 | doc_date | 13.05.2025 | CD | дата | derived |

#### Итого, по документу:
- `doc_fields`: 9 из 9
- `doc_formalization_status`: confirmed

### document: EGRUL
- `uqi_prefix`: formalized.egrul_1
- `xml_target_root`: AltaFreeDoc
- `path`: stable_source\FreeDoc_ЮЭ9965-25-106893283.xml
- `file_name`: FreeDoc_ЮЭ9965-25-106893283.xml

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04011 | CD | код вида документа | from xml |
| 02 | DocumentHead_DocumentName | ВЫПИСКА ИЗ  ЕГРЮЛ | CD | наименование выписки | from xml |
| 03 | DocumentHead_DocumentDate | 14.07.2025 | CD | дата выписки | from xml |
| 04 | DocumentHead_DocumentNumber | ЮЭ9965-25-106893283 | CD | номер выписки | from xml |
| 05 | TextPara | link:stable_source\\FreeDoc_ЮЭ9965-25-106893283.xml | CD | текст выписки | link per schema |
| 06 | doc_code | 04011 | CD | код | derived |
| 07 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование | derived |
| 08 | doc_number | ЮЭ9965-25-106893283 | CD | номер | derived |
| 09 | doc_date | 14.07.2025 | CD | дата | derived |

#### Итого, по документу:
- `doc_fields`: 9 из 9
- `doc_formalization_status`: confirmed

### document: Personal Passport
- `uqi_prefix`: formalized.passport_1
- `xml_target_root`: AltaPassport
- `path`: stable_source\Passport_63_09_449948.xml
- `file_name`: Passport_63_09_449948.xml

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 11001 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | ПАСПОРТ | CD | наименование документа | derived |
| 03 | DocumentHead_DocumentDate | 2010-03-11 | CD | дата документа | derived: =CardDate |
| 04 | DocumentHead_DocumentNumber | 63 09 449948 | CD | номер документа | derived |
| 05 | CardSeries | 63 09 | CD | серия | from xml |
| 06 | CardNumber | 449948 | CD | номер | from xml |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | from xml |
| 08 | CardDate | 2010-03-11 | CD | дата выдачи | from xml |
| 09 | PersonInfo_PersonSurname | АРБУЗОВА | CD | фамилия | from xml |
| 10 | PersonInfo_PersonName | АНАСТАСИЯ | CD | имя | from xml |
| 11 | PersonInfo_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | from xml |
| 12 | PersonInfo_Sex | 1 | CD | пол | from xml |
| 13 | PersonInfo_Birthday | 1987-07-25 | CD | дата рождения | from xml |
| 14 | PersonInfo_Birthplace | город Саратов | CD | место рождения | from xml |
| 15 | ResidencePlace_PostalCode | 410052 | CD | индекс | from xml |
| 16 | ResidencePlace_CountryCode | RU | CD | страна | from xml |
| 17 | ResidencePlace_CounryName | РОССИЯ | CD | страна, текст | from xml |
| 18 | ResidencePlace_Region | Саратовская область | CD | регион | from xml |
| 19 | ResidencePlace_City | Саратов | CD | город | from xml |
| 20 | ResidencePlace_StreetHouse | Ул. Одесская д 11 кв 160 | CD | адрес | from xml |
| 21 | doc_code | 11001 | CD | код | derived |
| 22 | doc_name | ПАСПОРТ | CD | наименование | derived |
| 23 | doc_number | 63 09 449948 | CD | номер | derived |
| 24 | doc_date | 2010-03-11 | CD | дата | derived |

#### Итого, по документу:
- `doc_fields`: 24 из 24
- `doc_formalization_status`: confirmed

### document: Letter of Attorney
- `uqi_prefix`: formalized.letter_of_attorney_1
- `xml_target_root`: AltaLetterOfAttorney
- `path`: stable_source\LetterOfAttorney_1.xml
- `file_name`: LetterOfAttorney_1.xml

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 11004 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | ДОВЕРЕННОСТЬ | CD | наименование документа | derived: =DocumentReference_PrDocumentName |
| 03 | DocumentHead_DocumentDate | 2026-02-01 | CD | дата документа | derived: =DocumentReference_PrDocumentDate |
| 04 | DocumentHead_DocumentNumber | 1 | CD | номер документа | derived: =DocumentReference_PrDocumentNumber |
| 05 | Subject | link:stable_source\\LetterOfAttorney_1.xml | CD | текст доверенности | link per schema |
| 06 | EndDate | 2026-12-31 | CD | действительна до | from xml |
| 07 | DocumentReference_PrDocumentName | ДОВЕРЕННОСТЬ | CD | наименование доверенности | from xml |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер доверенности | from xml |
| 09 | DocumentReference_PrDocumentDate | 2026-02-01 | CD | дата доверенности | from xml |
| 10 | Organization_OrganizationName | ООО «СКИФ» | CD | выдавшая организация | from xml |
| 11 | Organization_ShortName | ООО «СКИФ» | CD | краткое наименование | from xml |
| 12 | Organization_OGRN | 1201600020390 | CD | ОГРН | from xml |
| 13 | Organization_INN | 1650389298 | CD | ИНН | from xml |
| 14 | Organization_KPP | 165001001 | CD | КПП | from xml |
| 15 | Organization_Address_PostalCode | 423800 | CD | индекс | from xml |
| 16 | Organization_Address_CountryCode | RU | CD | страна | from xml |
| 17 | Organization_Address_CounryName | РОССИЯ | CD | страна, текст | from xml |
| 18 | Organization_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | from xml |
| 19 | Organization_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | from xml |
| 20 | Organization_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | CD | улица/дом | from xml |
| 21 | Organization_OrganizationPerson_PersonSurname | Саранов | CD | подписант | from xml |
| 22 | Organization_OrganizationPerson_PersonName | Дмитрий | CD | подписант | from xml |
| 23 | Organization_OrganizationPerson_PersonMiddleName | Олегович | CD | подписант | from xml |
| 24 | Organization_OrganizationPerson_PersonPost | Директор | CD | должность | from xml |
| 25 | EmpoweredPerson_PersonSurname | АРБУЗОВА | CD | уполномоченное лицо | from xml |
| 26 | EmpoweredPerson_PersonName | АНАСТАСИЯ | CD | имя | from xml |
| 27 | EmpoweredPerson_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | from xml |
| 28 | EmpoweredPerson_PersonPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | должность | from xml |
| 29 | EmpoweredPerson_Passport_IdentityCardCode | RU01001 | CD | код документа | from xml |
| 30 | EmpoweredPerson_Passport_IdentityCardName | ПАСРФ | CD | наименование документа | from xml |
| 31 | EmpoweredPerson_Passport_IdentityCardSeries | 63 09 | CD | серия | from xml |
| 32 | EmpoweredPerson_Passport_IdentityCardNumber | 449948 | CD | номер | from xml |
| 33 | EmpoweredPerson_Passport_IdentityCardDate | 2010-03-11 | CD | дата выдачи | from xml |
| 34 | EmpoweredPerson_Passport_OrganizationName | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | from xml |
| 35 | doc_code | 11004 | CD | код | derived |
| 36 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование | derived |
| 37 | doc_number | 1 | CD | номер | derived |
| 38 | doc_date | 2026-02-01 | CD | дата | derived |

#### Итого, по документу:
- `doc_fields`: 38 из 38
- `doc_formalization_status`: confirmed

## 3. non_formalized:

### document: Storage Report (ДО-1)
- `uqi_prefix`: non_formalized.svh_1
- `path`: md\ДО 14431420260204161621.md
- `file_name`: ДО 14431420260204161621.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии/свидетельства СВХ | from: md/ДО... |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии/свидетельства СВХ | from: md/ДО... |
| 03 | actual_gross_weight | 3500 | CD | фактический вес по весам | copied_from: non_formalized.svh_additional_sheet_1 (operator allowed) |
| 04 | actual_places | 127 | CD | фактическое количество мест | copied_from: non_formalized.svh_additional_sheet_1 (operator allowed) |
| 05 | transport_reg_number | O157AO774/BT374974 | CD | номер ТС | normalized from md/ДО... |
| 06 | goods_[n] |  | CD | товары в разрезе строк ДО | 2 элемента |

#### 06: goods[]

#### goods_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | tnved | 7019900095 | CD | код товара | from: md/ДО... |
| 02 | places | 27 | CD | мест по строке | from: md/ДО... |
| 03 | gross_weight_kg | 1710 | CD | брутто по строке | from: md/ДО... |
| 04 | cost | 42228.00 | CD | стоимость | from: md/ДО... |
| 05 | currency_code | CNY | CD | валютный код | from: md/ДО... |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### goods_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара | from: md/ДО... |
| 02 | places | 100 | CD | мест | from: md/ДО... |
| 03 | gross_weight_kg | 1790 | CD | брутто | from: md/ДО... |
| 04 | cost | 55032.00 | CD | стоимость | from: md/ДО... |
| 05 | currency_code | CNY | CD | валюта | from: md/ДО... |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 10 из 5 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 6 из 6

### document: Storage Report Additional Sheet (ДО-1)
- `uqi_prefix`: non_formalized.svh_additional_sheet_1
- `path`: md\ДО доп 14431520260204161645.md
- `file_name`: ДО доп 14431520260204161645.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | number | 1 | CD | № доп.листа/приложения | from: md/ДО доп... |
| 02 | date | 03.02.2026 | CD | дата доп.листа | derived: = дата ДО-1 (из operator data) |
| 03 | actual_gross_weight | 3500 | CD | фактический вес брутто | from: md/ДО доп... totals |
| 04 | actual_places | 127 | CD | фактическое количество мест | from: md/ДО доп... totals |
| 05 | transport_reg_number | O157AO774/BT374974 | CD | номер ТС | from: md/ДО-1 (same report) |
| 06 | svh_address_region | Республика Татарстан | CD | регион СВХ | from: md/СМР... (place of delivery) |
| 07 | svh_address_city | Набережные Челны | CD | город СВХ | from: md/СМР... |
| 08 | svh_address_street_house | Производственный пр-д, д.45 | CD | улица/дом СВХ | from: md/СМР... |
| 09 | svh_customs_code | 10404083 | CD | код таможенного органа | from: md/СМР... block 13 |

#### Итого, по документу:
- `doc_fields`: 9 из 9

### document: Transit Declaration
- `uqi_prefix`: non_formalized.td_1
- `path`: md\ТД 10719110_240126_5011363_reg00378тд.md
- `file_name`: ТД 10719110_240126_5011363_reg00378тд.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | number | 10719110/240126/5011363 | CO | номер ТД | operator: non_formalized.transit_declaration_1.number |
| 02 | date | 24.01.2026 | CO | дата ТД | operator: non_formalized.transit_declaration_1.date |
| 03 | customs_post_code | 10404083 | CD | код таможенного органа | from: md/ТД... field 53 |
| 04 | customs_post_name | ОТО И ТК №3 Т/П НАБЕРЕЖНОЧЕЛНИНСКИЙ | CD | наименование таможенного органа | from: md/ТД... field 53 |
| 05 | transport_reg_number | O157AO774/BT374974 | CD | ТС по ТД | from: md/ТД... field 18 |

#### Итого, по документу:
- `doc_fields`: 5 из 5

## 4. Итогo, по файлу:
- `total_unreliable_fields`: 0
- `total_doc_fields`: n/a (not computed)
- `total_fields`: 828
- `formalization_status`: pending

## 5. Нерешенные вопросы (Issues)

## 6. `unreliable_fields`:
