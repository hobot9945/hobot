# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ
- `источники данных`: md + operator_provided_data + master_data + stable_source

### `document`: Contract
  - `uqi_prefix`: formalized.contract_1
  - `xml_target_root`: AltaE2CONT
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md
  - `file_name`: SALES CONTRACT NoLM-2553.md
  - `note`: Контракт на поставку москитных сеток

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 03011 | CD | код вида документа | константа |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта | copied_from:Contract |
| 03 | ContractRegistration_PrDocumentDate | 02.07.2025 | CD | дата контракта | copied_from:Contract |
| 04 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта | operator:decisions_from_chat (2026-05-08) |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты | operator:contract.currency_code_numeric |
| 06 | ContractTerms_LastDate | 31.12.2026 | CD | срок действия/исполнения | copied_from:Contract |
| 07 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки | operator:contract.delivery_terms |
| 08 | ContractTerms_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md | CD | текст контракта | copied_from:Contract |
| 09 | ContractTerms_DealSign | 1 | CO | системный признак Альты | operator:contract.deal_sign |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта | copied_from:Contract |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 | operator:contract.foreign_person_country_code_alpha2 |
| 12 | ForeignPerson_Address_CounryName | КИТАЙ | CD | страна продавца, текст | copied_from:Contract (China) |
| 13 | ForeignPerson_Address_Region | Hebei | CD | регион/область продавца | copied_from:Contract |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца | copied_from:Contract |
| 15 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом продавца одной строкой | operator:decisions_from_chat (2026-05-15) |
| 16 | RussianPerson_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO | покупатель/сторона контракта | operator:allow_cross_doc_master_data_to_contract_invoice |
| 17 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН покупателя | operator:allow_cross_doc_master_data_to_contract_invoice |
| 18 | RussianPerson_INN | 1650389298 | CO | ИНН покупателя | operator:allow_cross_doc_master_data_to_contract_invoice |
| 19 | RussianPerson_KPP | 165001001 | CO | КПП покупателя | operator:allow_cross_doc_master_data_to_contract_invoice |
| 20 | RussianPerson_Address_PostalCode | 423800 | CD | индекс покупателя | copied_from:Contract |
| 21 | RussianPerson_Address_CountryCode | RU | CD | страна покупателя alpha-2 | copied_from:Contract |
| 22 | RussianPerson_Address_CounryName | РОССИЯ | CD | страна покупателя, текст | copied_from:Contract |
| 23 | RussianPerson_Address_Region | Республика Татарстан | CD | регион покупателя | copied_from:Contract |
| 24 | RussianPerson_Address_City | Набережные Челны | CD | город покупателя | copied_from:Contract |
| 25 | RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator:decisions_from_chat (2026-05-15) |
| 26 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 27 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 28 | doc_code | 03011 | CD | код документа | константа |
| 29 | doc_name | КОНТРАКТ | CD | наименование документа | константа |
| 30 | doc_number | LM-2553 | CD | номер документа | copied_from:formalized.contract_1.ContractRegistration_PrDocumentNumber |
| 31 | doc_date | 02.07.2025 | CD | дата документа | copied_from:formalized.contract_1.ContractRegistration_PrDocumentDate |

- _audit: 31
- `doc_status`: confirmed

### `document`: Supplementary Contract
  - `uqi_prefix`: formalized.supplementary_contract_1
  - `xml_target_root`: AltaSupplementaryContract
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md
  - `file_name`: 1 Supplementary agreement to the contract.md
  - `note`: Дополнительное соглашение №1 к контракту

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения | copied_from:Supplementary Contract |
| 02 | IssueDate | 25.11.2025 | CD | дата доп. соглашения | copied_from:Supplementary Contract |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта | copied_from:Supplementary Contract |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты | operator:supplementary_contract.currency_code_numeric |
| 05 | ContractDescription_LastDate | 31.12.2026 | CO | новый срок действия/исполнения | operator:supplementary_contract.expiry_date |
| 06 | ContractDescription_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md | CD | текст доп. соглашения | copied_from:Supplementary Contract |
| 07 | ContractDescription_DealSign | 1 | CO | системный признак Альты | operator:supplementary_contract.deal_sign |
| 08 | ContractDescription_StockCategorySign | 0 | CO | системный признак Альты | operator:supplementary_contract.stock_category_sign |
| 09 | ContractDescription_BuyerLimitationSign | 0 | CO | системный признак Альты | operator:supplementary_contract.buyer_limitation_sign |
| 10 | ContractDescription_InsuranceSign | 0 | CO | системный признак Альты | operator:supplementary_contract.insurance_sign |
| 11 | RussianPerson_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO | российская сторона; покупатель | operator:allow_cross_doc_master_data_to_contract_invoice |
| 12 | RussianPerson_ShortName | ООО "СКИФ" | CO | краткое наименование | operator:supplementary_contract.russian_person_short_name |
| 13 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН | operator:allow_cross_doc_master_data_to_contract_invoice |
| 14 | RussianPerson_INN | 1650389298 | CO | ИНН | operator:allow_cross_doc_master_data_to_contract_invoice |
| 15 | RussianPerson_KPP | 165001001 | CO | КПП | operator:allow_cross_doc_master_data_to_contract_invoice |
| 16 | RussianPerson_Address_PostalCode | 423800 | CO | индекс | operator:allow_cross_doc_master_data_to_contract_invoice |
| 17 | RussianPerson_Address_CountryCode | RU | CO | страна alpha-2 | operator:allow_cross_doc_master_data_to_contract_invoice |
| 18 | RussianPerson_Address_CounryName | РОССИЯ | CO | страна, текст | operator:allow_cross_doc_master_data_to_contract_invoice |
| 19 | RussianPerson_Address_Region | Республика Татарстан | CO | регион | operator:allow_cross_doc_master_data_to_contract_invoice |
| 20 | RussianPerson_Address_City | Набережные Челны | CD | город | copied_from:Supplementary Contract |
| 21 | RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом одной строкой | operator:decisions_from_chat (2026-05-15) |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | иностранная сторона; продавец | copied_from:Supplementary Contract |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator:supplementary_contract.foreign_person_short_name_equals_full |
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 | operator:supplementary_contract.foreign_person_country_code_alpha2 |
| 25 | ForeignPerson_Address_CounryName | КИТАЙ | CO | страна, текст | operator:supplementary_contract.foreign_person_address_from_contract |
| 26 | ForeignPerson_Address_Region | Hebei | CO | регион | operator:supplementary_contract.foreign_person_address_from_contract |
| 27 | ForeignPerson_Address_City | Shijiazhuang | CO | город/район | operator:supplementary_contract.foreign_person_address_from_contract |
| 28 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом одной строкой | operator:supplementary_contract.foreign_person_address_from_contract |
| 29 | PersonSurname | Li | CO | фамилия подписанта | operator:supplementary_contract.signed_person_surname |
| 30 | PersonName | Jing | CO | имя подписанта | operator:supplementary_contract.signed_person_name |
| 31 | PersonMiddleName | | CO | отчество подписанта | operator:supplementary_contract.signed_person_middle_name |
| 32 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 33 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 34 | doc_code | 03012 | CD | код документа | константа |
| 35 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наименование документа | константа |
| 36 | doc_number | 1 | CD | номер документа | copied_from:formalized.supplementary_contract_1.DocumentNumber |
| 37 | doc_date | 25.11.2025 | CD | дата документа | copied_from:formalized.supplementary_contract_1.IssueDate |

- _audit: 37
- `doc_status`: confirmed

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\CL на сетку .md
  - `file_name`: CL на сетку .md
  - `note`: Коммерческий инвойс

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты | operator:invoice.exchange_rate |
| 02 | CurrencyCode | CNY | CO | валюта инвойса | operator:invoice.currency_code |
| 03 | DocumentCode | 04021 | CD | код вида документа | константа |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест | copied_from:Invoice |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator:invoice.places_description |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто | operator:invoice.total_gross_weight |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто | operator:invoice.total_net_weight |
| 08 | GCost | 97260.00 | CO | системное поле Альты | operator:invoice.gcost |
| 09 | TotalCost | 97260.00 | CO | итого по инвойсу | operator:invoice.total_cost |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | copied_from:Invoice |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator:invoice.delivery_terms_numeric |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator:invoice.delivery_terms_string |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | operator:invoice.dispatch_country_code |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | operator:invoice.trading_country_code |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | operator:invoice.destination_country_code |
| 16 | Registration_PrDocumentName | Commercial invoice / Комерческий инвойс | CD | наименование документа | copied_from:Invoice |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | copied_from:Invoice |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | copied_from:Invoice |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | copied_from:Invoice |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | copied_from:Invoice |
| 21 | Buyer_CompanyID | 1650389298 | CO | ИНН покупателя | operator:allow_cross_doc_master_data_to_contract_invoice |
| 22 | Buyer_KPPCode | 165001001 | CO | КПП покупателя | operator:allow_cross_doc_master_data_to_contract_invoice |
| 23 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO | наименование покупателя | operator:allow_cross_doc_master_data_to_contract_invoice |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | copied_from:Invoice |
| 25 | Buyer_PostalAddress_CountryCode | RU | CO | страна покупателя alpha-2 | operator:allow_cross_doc_master_data_to_contract_invoice |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CO | страна покупателя, текст | operator:allow_cross_doc_master_data_to_contract_invoice |
| 27 | Buyer_PostalAddress_Region | Республика Татарстан | CO | регион | operator:allow_cross_doc_master_data_to_contract_invoice |
| 28 | Buyer_PostalAddress_City | Набережные Челны | CD | город | copied_from:Invoice |
| 29 | Buyer_PostalAddress_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator:decisions_from_chat (2026-05-15) |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | продавец | copied_from:Invoice |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | operator:invoice.seller_country_code_alpha2 |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | copied_from:Invoice (China) |
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | copied_from:Invoice |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город/район продавца | copied_from:Invoice |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from:Invoice |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | operator:invoice.consignor_equals_seller |
| 37 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | operator:invoice.consignor_equals_seller |
| 38 | Consignor_Address_CounryName | КИТАЙ | CO | страна грузоотправителя, текст | operator:invoice.consignor_equals_seller |
| 39 | Consignor_Address_Region | Hebei | CO | регион | operator:invoice.consignor_equals_seller |
| 40 | Consignor_Address_City | Shijiazhuang | CO | город/район | operator:invoice.consignor_equals_seller |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CO | улица/дом одной строкой | operator:invoice.consignor_equals_seller |
| 42 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO | грузополучатель | operator:invoice.consignee_equals_buyer |
| 43 | Consignee_OGRN | 1201600020390 | CO | ОГРН | operator:allow_cross_doc_master_data_to_contract_invoice |
| 44 | Consignee_INN | 1650389298 | CO | ИНН | operator:allow_cross_doc_master_data_to_contract_invoice |
| 45 | Consignee_KPP | 165001001 | CO | КПП | operator:allow_cross_doc_master_data_to_contract_invoice |
| 46 | Consignee_Address_PostalCode | 423800 | CO | индекс | operator:allow_cross_doc_master_data_to_contract_invoice |
| 47 | Consignee_Address_CountryCode | RU | CO | страна alpha-2 | operator:allow_cross_doc_master_data_to_contract_invoice |
| 48 | Consignee_Address_CounryName | РОССИЯ | CO | страна, текст | operator:allow_cross_doc_master_data_to_contract_invoice |
| 49 | Consignee_Address_Region | Республика Татарстан | CO | регион | operator:allow_cross_doc_master_data_to_contract_invoice |
| 50 | Consignee_Address_City | Набережные Челны | CO | город | operator:allow_cross_doc_master_data_to_contract_invoice |
| 51 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator:decisions_from_chat (2026-05-15) |
| 52 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 53 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 54 | doc_code | 04021 | CD | код документа | константа |
| 55 | doc_name | ИНВОЙС | CD | наименование документа | константа |
| 56 | doc_number | LM-2591 | CD | номер документа | copied_from:formalized.invoice_1.Registration_PrDocumentNumber |
| 57 | doc_date | 30.10.2025 | CD | дата документа | copied_from:formalized.invoice_1.Registration_PrDocumentDate |

- _audit: 57
- `doc_status`: confirmed

#### formalized.invoice_1 Массив: InvoiceGoods[7]
- _array_audit: 7

#### formalized.invoice_1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:Invoice |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара | copied_from:Invoice |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | copied_from:Invoice |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | copied_from:Invoice |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:Invoice |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:Invoice |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator:invoice_goods_weights |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator:invoice_goods_weights |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from:Invoice |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | copied_from:Invoice |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:invoice_goods.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_goods.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:invoice_goods.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:invoice_goods.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:invoice_goods.model |

- _item_audit: 15

#### formalized.invoice_1 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:Invoice |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара | copied_from:Invoice |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | copied_from:Invoice |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | copied_from:Invoice |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:Invoice |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:Invoice |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator:invoice_goods_weights |
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator:invoice_goods_weights |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from:Invoice |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | copied_from:Invoice |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:invoice_goods.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_goods.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:invoice_goods.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:invoice_goods.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:invoice_goods.model |

- _item_audit: 15

#### formalized.invoice_1 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:Invoice |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы \"Антипыльца\" из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | CD | описание товара | copied_from:Invoice |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | copied_from:Invoice |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | copied_from:Invoice |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:Invoice |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:Invoice |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator:invoice_goods_weights |
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator:invoice_goods_weights |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from:Invoice |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | copied_from:Invoice |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:invoice_goods.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_goods.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:invoice_goods.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:invoice_goods.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:invoice_goods.model |

- _item_audit: 15

#### formalized.invoice_1 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:Invoice |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы \"Антипыльца\" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CD | описание товара | copied_from:Invoice |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | copied_from:Invoice |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | copied_from:Invoice |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:Invoice |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:Invoice |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто по строке | operator:invoice_goods_weights |
| 08 | NetWeightQuantity | 144.00 | CO | нетто по строке | operator:invoice_goods_weights |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from:Invoice |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | copied_from:Invoice |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:invoice_goods.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_goods.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:invoice_goods.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:invoice_goods.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:invoice_goods.model |

- _item_audit: 15

#### formalized.invoice_1 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from:Invoice |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА \"Антимошка\" из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара | copied_from:Invoice |
| 03 | GoodsQuantity | 90 | CD | кол-во по строке инвойса | copied_from:Invoice |
| 04 | goods_supplementary_quantity | 3780 | CD | количество в доп.ед.изм | copied_from:Invoice |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:Invoice |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:Invoice |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто по строке | operator:invoice_goods_weights |
| 08 | NetWeightQuantity | 491.40 | CO | нетто по строке | operator:invoice_goods_weights |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from:Invoice |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | copied_from:Invoice |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:invoice_goods.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_goods.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:invoice_goods.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:invoice_goods.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:invoice_goods.model |

- _item_audit: 15

#### formalized.invoice_1 Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from:Invoice |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА \"Антимошка\" из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара | copied_from:Invoice |
| 03 | GoodsQuantity | 180 | CD | кол-во по строке инвойса | copied_from:Invoice |
| 04 | goods_supplementary_quantity | 8640 | CD | количество в доп.ед.изм | copied_from:Invoice |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:Invoice |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:Invoice |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто по строке | operator:invoice_goods_weights |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто по строке | operator:invoice_goods_weights |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from:Invoice |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | copied_from:Invoice |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:invoice_goods.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_goods.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:invoice_goods.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:invoice_goods.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:invoice_goods.model |

- _item_audit: 15

#### formalized.invoice_1 Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:Invoice |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки \"Антипыльца\" из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | copied_from:Invoice |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке инвойса | copied_from:Invoice |
| 04 | goods_supplementary_quantity | 240 | CD | количество в доп.ед.изм | copied_from:Invoice |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:Invoice |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:Invoice |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто по строке | operator:invoice_goods_weights |
| 08 | NetWeightQuantity | 24.00 | CO | нетто по строке | operator:invoice_goods_weights |
| 09 | Price | 28 | CD | цена за единицу | copied_from:Invoice |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | copied_from:Invoice |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator:invoice_goods.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:invoice_goods.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:invoice_goods.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:invoice_goods.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:invoice_goods.model |

- _item_audit: 15

### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list_1
  - `xml_target_root`: AltaE2PACK
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\PL на сетку .md
  - `file_name`: PL на сетку .md
  - `note`: Упаковочный лист

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто | operator:invoice.total_gross_weight |
| 02 | NetWeightQuantity | 3302.00 | CO | общий вес нетто | operator:invoice.total_net_weight |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | operator:invoice.consignor_equals_seller |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | краткое наименование | operator:packing_list.consignor_shortname_equals_full |
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | operator:packing_list.consignor_country_code_alpha2 |
| 06 | Consignor_Address_CounryName | КИТАЙ | CO | страна грузоотправителя, текст | operator:invoice.consignor_equals_seller |
| 07 | Consignor_Address_Region | Hebei | CO | регион | operator:invoice.consignor_equals_seller |
| 08 | Consignor_Address_City | Shijiazhuang | CO | город/район | operator:invoice.consignor_equals_seller |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CO | улица/дом одной строкой | operator:invoice.consignor_equals_seller |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO | грузополучатель | operator:invoice.consignee_equals_buyer |
| 11 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator:packing_list.consignee_shortname_equals_full |
| 12 | Consignee_OGRN | 1201600020390 | CO | ОГРН | operator:allow_cross_doc_master_data_to_contract_invoice |
| 13 | Consignee_INN | 1650389298 | CO | ИНН | operator:allow_cross_doc_master_data_to_contract_invoice |
| 14 | Consignee_KPP | 165001001 | CO | КПП | operator:allow_cross_doc_master_data_to_contract_invoice |
| 15 | Consignee_Address_PostalCode | 423800 | CO | индекс | operator:allow_cross_doc_master_data_to_contract_invoice |
| 16 | Consignee_Address_CountryCode | RU | CO | страна alpha-2 | operator:allow_cross_doc_master_data_to_contract_invoice |
| 17 | Consignee_Address_CounryName | РОССИЯ | CO | страна, текст | operator:allow_cross_doc_master_data_to_contract_invoice |
| 18 | Consignee_Address_Region | Республика Татарстан | CO | регион | operator:allow_cross_doc_master_data_to_contract_invoice |
| 19 | Consignee_Address_City | Набережные Челны | CO | город | operator:allow_cross_doc_master_data_to_contract_invoice |
| 20 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator:decisions_from_chat (2026-05-15) |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | copied_from:Packing List |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | внутренний числовой код условий | operator:invoice.delivery_terms_numeric |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator:invoice.delivery_terms_string |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | наименование контракта | operator:decisions_from_chat (2026-05-08) |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | copied_from:Packing List |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | copied_from:Packing List |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | константа |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | copied_from:Packing List |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | copied_from:Packing List |
| 30 | DeliveryTerms_Registration_PrDocumentName | Упаковочный лист | CO | наименование упаковочного | operator:packing_list.registration_doc_name |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | operator:packing_list.registration_doc_number |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | operator:packing_list.registration_doc_date |
| 33 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 34 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 35 | doc_code | 04131 | CD | код документа | константа |
| 36 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | константа |
| 37 | doc_number | LM-2591 | CD | номер документа | copied_from:formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentNumber |
| 38 | doc_date | 30.10.2025 | CD | дата документа | copied_from:formalized.packing_list_1.DeliveryTerms_Registration_PrDocumentDate |

- _audit: 38
- `doc_status`: confirmed

#### formalized.packing_list_1 Массив: Goods[7]
- _array_audit: 7

#### formalized.packing_list_1 Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки | copied_from:Packing List |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | copied_from:Packing List |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | copied_from:Packing List |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | copied_from:Packing List |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок/мест в упаковке | operator:packing_list.goods_1.paking_quantity |

- _item_audit: 5

#### formalized.packing_list_1 Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки | copied_from:Packing List |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц | copied_from:Packing List |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто по строке | copied_from:Packing List |
| 04 | NetWeightQuantity | 460.80 | CD | нетто по строке | copied_from:Packing List |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок/мест в упаковке | operator:packing_list.goods_2.paking_quantity |

- _item_audit: 5

#### formalized.packing_list_1 Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца \" из полиэстера . Размер рулона 1,42*0,64*0,22 | CD | описание строки | copied_from:Packing List |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | copied_from:Packing List |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто по строке | copied_from:Packing List |
| 04 | NetWeightQuantity | 252.00 | CD | нетто по строке | copied_from:Packing List |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок/мест в упаковке | operator:packing_list.goods_3.paking_quantity |

- _item_audit: 5

#### formalized.packing_list_1 Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца \" из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание строки | copied_from:Packing List |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц | copied_from:Packing List |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто по строке | copied_from:Packing List |
| 04 | NetWeightQuantity | 144.00 | CD | нетто по строке | copied_from:Packing List |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок/мест в упаковке | operator:packing_list.goods_4.paking_quantity |

- _item_audit: 5

#### formalized.packing_list_1 Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА \" Антимошка \" из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание строки | copied_from:Packing List |
| 02 | GoodsQuantity | 90 | CD | количество мест/грузовых единиц | copied_from:Packing List |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто по строке | copied_from:Packing List |
| 04 | NetWeightQuantity | 491.40 | CD | нетто по строке | copied_from:Packing List |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок/мест в упаковке | operator:packing_list.goods_5.paking_quantity |

- _item_audit: 5

#### formalized.packing_list_1 Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА \"Антимошка \" из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание строки | copied_from:Packing List |
| 02 | GoodsQuantity | 180 | CD | количество мест/грузовых единиц | copied_from:Packing List |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | copied_from:Packing List |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто по строке | copied_from:Packing List |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок/мест в упаковке | operator:packing_list.goods_6.paking_quantity |

- _item_audit: 5

#### formalized.packing_list_1 Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки \"Антипыльца \" из полиэстера Размер рулона 1,72*0,35*0,31*1 | CD | описание строки | copied_from:Packing List |
| 02 | GoodsQuantity | 5 | CD | количество мест/грузовых единиц | copied_from:Packing List |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто по строке | copied_from:Packing List |
| 04 | NetWeightQuantity | 24.00 | CD | нетто по строке | copied_from:Packing List |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок/мест в упаковке | operator:packing_list.goods_7.paking_quantity |

- _item_audit: 5

#### formalized.packing_list_1 Массив: TransportMeans[2]
- _array_audit: 2

#### formalized.packing_list_1 Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | О157АО774 | CO | регистрационный номер | operator:packing_list.transport_1.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator:packing_list.transport_1.mode_code |
| 03 | NationalityCode | 000 | CO | код “национальности” ТС | operator:packing_list.transport_1.nationality_code |
| 04 | MoverIndicator | true | CO | признак тягача | operator:packing_list.transport_1.mover_indicator |

- _item_audit: 4

#### formalized.packing_list_1 Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | ВТ374974 | CO | регистрационный номер | operator:packing_list.transport_2.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator:packing_list.transport_2.mode_code |
| 03 | NationalityCode | 000 | CO | код “национальности” ТС | operator:packing_list.transport_2.nationality_code |
| 04 | MoverIndicator | false | CO | признак тягача | operator:packing_list.transport_2.mover_indicator |

- _item_audit: 4

### `document`: CMR
  - `uqi_prefix`: formalized.cmr_1
  - `xml_target_root`: AltaE3CMR
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\СМР от СВХ.md
  - `file_name`: СМР от СВХ.md
  - `note`: Международная товарно-транспортная накладная

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CO | язык документа | operator:cmr.language_code |
| 02 | CMR_Choice | 1 | CO | системный выбор/вариант Альты | operator:cmr.cmr_choice |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | copied_from:CMR |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | copied_from:CMR |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления | operator:cmr.registration_place |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата принятия груза | copied_from:CMR |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2 | operator:cmr.taking_cargo_country_code_alpha2 |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза, текст | copied_from:CMR (Китай) |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | operator:cmr.delivery_country_code_alpha2 |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки, текст | copied_from:CMR (Россия) |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки по Incoterms | operator:cmr.delivery_terms_place |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator:cmr.delivery_terms_string |
| 13 | GoodsQuantity | 127 | CD | общее количество грузовых мест | copied_from:CMR |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто | copied_from:CMR |
| 15 | CMRTransport_PrimeMoverStateSignID | О157АО774 | CD | гос. номер тягача | copied_from:CMR |
| 16 | CMRTransport_TrailerStateSignID | ВТ374974 | CD | гос. номер прицепа | copied_from:CMR |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование отправителя | copied_from:CMR |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator:cmr.consignor_shortname_equals_full |
| 19 | Consignor_PostalAddress_CountryCode | CN | CO | страна alpha-2 | operator:cmr.taking_cargo_country_code_alpha2 |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна, текст | copied_from:CMR (China) |
| 21 | Consignor_Address_Region | Hebei | CD | регион | copied_from:CMR |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from:CMR |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from:CMR |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | наименование гаранта | operator:cmr.consignor_guarantee_all |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое наименование | operator:cmr.consignor_guarantee_all |
| 26 | Consignor_Guarantee_Address_CountryCode | CN | CO | страна alpha-2 | operator:cmr.consignor_guarantee_all |
| 27 | Consignor_Guarantee_Address_CounryName | КИТАЙ | CO | страна, текст | operator:cmr.consignor_guarantee_all |
| 28 | Consignor_Guarantee_Address_Region | ОТСУТСТВУЕТ | CO | регион | operator:cmr.consignor_guarantee_all |
| 29 | Consignor_Guarantee_Address_City | ОТСУТСТВУЕТ | CO | город/район | operator:cmr.consignor_guarantee_all |
| 30 | Consignor_Guarantee_Address_StreetHouse | ОТСУТСТВУЕТ | CO | улица/дом одной строкой | operator:cmr.consignor_guarantee_all |
| 31 | Consignee_NameInf | ООО «Скиф» | CD | наименование получателя | copied_from:CMR |
| 32 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator:cmr.consignee_shortname_equals_full |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | operator:decisions_from_chat (2026-05-01) |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | copied_from:CMR |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | copied_from:CMR |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | copied_from:CMR |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | copied_from:CMR |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | copied_from:CMR (Россия) |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | copied_from:CMR |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | copied_from:CMR |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом/офис одной строкой | copied_from:CMR |
| 42 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 43 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 44 | doc_code | 02015 | CD | код документа | константа |
| 45 | doc_name | CMR | CD | наименование документа | константа |
| 46 | doc_number | 00378 | CD | номер документа | copied_from:formalized.cmr_1.RegistrationDocument_RegID |
| 47 | doc_date | 20.01.2026 | CD | дата документа | copied_from:formalized.cmr_1.RegistrationDocument_DateInf |

- _audit: 47
- `doc_status`: confirmed

#### formalized.cmr_1 Массив: CMRGoods[1]
- _array_audit: 1

#### formalized.cmr_1 Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CD | описание груза/товара | исключение CMRGoodsDescription — источник non_formalized.svh_1 |
| 03 | PakingQuantity | 127 | CO | кол-во упаковок/мест | operator:cmr.goods_1.packing_quantity |

- _item_audit: 3

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_1_13.01.2026.md
  - `file_name`: currency_transfer_1_13.01.2026.md
  - `note`: Заявление на перевод №1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator:payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator:payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | copied_from:Payment Order 1 |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator:payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator:payment_order_all.priority |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from:Payment Order 1 |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | copied_from:Payment Order 1 |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | copied_from:Payment Order 1 |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | copied_from:Payment Order 1 |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | copied_from:Payment Order 1 |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from:Payment Order 1 |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator:payment_order_1.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ \"ЦЕНТРАЛЬНЫЙ\" БАНКА ВТБ (ПАО)) | CD | банк плательщика | copied_from:Payment Order 1 |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD; account 40807156900610036383 | CD | получатель платежа | copied_from:Payment Order 1 |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | реквизиты банка получателя | copied_from:Payment Order 1 |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator:payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator:payment_order_all.payer_sign.name |
| 18 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 19 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 20 | doc_code | 04023 | CD | код документа | константа |
| 21 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | константа |
| 22 | doc_number | 1 | CD | номер документа | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentNumber |
| 23 | doc_date | 13.01.2026 | CD | дата документа | copied_from:formalized.payment_order_1.DocumentReference_PrDocumentDate |

- _audit: 23
- `doc_status`: confirmed

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_7_28.11.2025.md
  - `file_name`: currency_transfer_7_28.11.2025.md
  - `note`: Заявление на перевод №7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator:payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator:payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | copied_from:Payment Order 7 |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator:payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator:payment_order_all.priority |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from:Payment Order 7 |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | copied_from:Payment Order 7 |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | copied_from:Payment Order 7 |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | copied_from:Payment Order 7 |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | copied_from:Payment Order 7 |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from:Payment Order 7 |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator:payment_order_2.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (ФИЛИАЛ \"ЦЕНТРАЛЬНЫЙ\" БАНКА ВТБ (ПАО)) | CD | банк плательщика | copied_from:Payment Order 7 |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD; account 40807156900610036383 | CD | получатель платежа | copied_from:Payment Order 7 |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | реквизиты банка получателя | copied_from:Payment Order 7 |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator:payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator:payment_order_all.payer_sign.name |
| 18 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 19 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 20 | doc_code | 04023 | CD | код документа | константа |
| 21 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | константа |
| 22 | doc_number | 7 | CD | номер документа | copied_from:formalized.payment_order_2.DocumentReference_PrDocumentNumber |
| 23 | doc_date | 28.11.2025 | CD | дата документа | copied_from:formalized.payment_order_2.DocumentReference_PrDocumentDate |

- _audit: 23
- `doc_status`: confirmed

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice_1
  - `xml_target_root`: AltaServiceInvoice
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_от_27-01-2026.md
  - `file_name`: Счет_№26-00378-tl_от_27-01-2026.md
  - `note`: Счет за перевозку

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак документа Альты | operator:service_invoice.document_sign |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | copied_from:Service Invoice |
| 03 | Currency | USD | CD | валюта итого | copied_from:Service Invoice |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг/перевозчик | copied_from:Service Invoice |
| 05 | BankName | АО "Райффайзенбанк"; БИК 044525700; Сч. № 30101810200000000700; Сч. № 40702810400000233463 | CD | банк исполнителя | copied_from:Service Invoice |
| 06 | ContractDetails_PrDocumentNumber | №КООО/26651/М | CD | № договора на услуги/перевозку | copied_from:Service Invoice |
| 07 | ContractDetails_PrDocumentDate | 13-05-2025 | CD | дата договора на услуги/перевозку | copied_from:Service Invoice |
| 08 | PaymentDocument_PrDocumentNumber | ОТСУТСТВУЕТ | CO | номер связанного документа | operator:service_invoice.payment_document_number |
| 09 | PaymentDocument_PrDocumentDate | ОТСУТСТВУЕТ | CO | дата связанного документа | operator:service_invoice.payment_document_date |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | copied_from:Service Invoice |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | copied_from:Service Invoice |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | copied_from:Service Invoice |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | operator:service_invoice.consignor_decision |
| 14 | Consignor_SubjectAddressDetails_PostalCode | | CO | индекс | operator:service_invoice_1.consignor_postalcode_empty_ok |
| 15 | Consignor_SubjectAddressDetails_CountryCode | CN | CO | страна alpha-2 | operator:service_invoice_1.consignor_address_from_seller |
| 16 | Consignor_SubjectAddressDetails_CounryName | КИТАЙ | CO | страна, текст | operator:service_invoice_1.consignor_address_from_seller |
| 17 | Consignor_SubjectAddressDetails_Region | Hebei | CO | регион | operator:service_invoice_1.consignor_address_from_seller |
| 18 | Consignor_SubjectAddressDetails_Town | Shijiazhuang | CO | город/район | operator:service_invoice_1.consignor_address_from_seller |
| 19 | Consignor_SubjectAddressDetails_StreetHouse | No. 5 Gaodong street | CO | улица/дом одной строкой | operator:service_invoice_1.consignor_address_from_seller |
| 20 | Consignee_OrganizationName | ООО "Скиф" (ООО "СКИФ") | CD | грузополучатель | copied_from:Service Invoice |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator:service_invoice_1.consignee_ogrn_from_master_data |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | copied_from:Service Invoice |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | copied_from:Service Invoice |
| 24 | Consignee_SubjectAddressDetails_PostalCode | 423800 | CD | индекс | copied_from:Service Invoice |
| 25 | Consignee_SubjectAddressDetails_CountryCode | RU | CO | страна alpha-2 | operator:allow_cross_doc_master_data_to_contract_invoice |
| 26 | Consignee_SubjectAddressDetails_CounryName | РОССИЯ | CO | страна, текст | operator:allow_cross_doc_master_data_to_contract_invoice |
| 27 | Consignee_SubjectAddressDetails_Region | Республика Татарстан | CD | регион | copied_from:Service Invoice |
| 28 | Consignee_SubjectAddressDetails_Town | Набережные Челны | CD | город | copied_from:Service Invoice |
| 29 | Consignee_SubjectAddressDetails_StreetHouse | проезд Хлебный | CD | улица | copied_from:Service Invoice |
| 30 | Consignee_SubjectAddressDetails_House | 30 | CO | дом | operator:service_invoice.consignee_house |
| 31 | Consignee_SubjectAddressDetails_Room | 211 | CO | офис/кв | operator:service_invoice.consignee_room |
| 32 | Signature_Choice | 2 | CD | вариант подписи | copied_from:Service Invoice |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | copied_from:Service Invoice |
| 34 | IndividualEntrepreneur_PersonName | | CD | первый инициал/имя ИП | copied_from:Service Invoice |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | второй инициал/отчество ИП | copied_from:Service Invoice |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климовин | CD | фамилия руководителя | copied_from:Service Invoice |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CD | первый инициал/имя руководителя | copied_from:Service Invoice |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | | CD | второй инициал/отчество руководителя | copied_from:Service Invoice |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | copied_from:Service Invoice |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CD | первый инициал/имя бухгалтера | copied_from:Service Invoice |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | | CD | второй инициал/отчество бухгалтера | copied_from:Service Invoice |
| 42 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 43 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 44 | doc_code | 04031 | CD | код документа | константа |
| 45 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | константа |
| 46 | doc_number | 26-00378-tl | CD | номер документа | copied_from:formalized.service_invoice_1.Registration_PrDocumentNumber |
| 47 | doc_date | 27.01.2026 | CD | дата документа | copied_from:formalized.service_invoice_1.Registration_PrDocumentDate |
| 48 | transport_to_border | 1404.00 | CO | стоимость маршрута до границы | operator:service_invoice_1.transport_to_border |
| 49 | transport_currency | USD | CO | валюта стоимости | operator:service_invoice_1.transport_currency |

- _audit: 49
- `doc_status`: confirmed

#### formalized.service_invoice_1 Массив: ServiceDescription[2]
- _array_audit: 2

#### formalized.service_invoice_1 Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) - перевозка автотранспортом | CD | описание услуги | copied_from:Service Invoice |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:Service Invoice |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator:service_invoice_1.service_1.service_name |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from:Service Invoice |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:Service Invoice |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | copied_from:Service Invoice |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | copied_from:Service Invoice |

- _item_audit: 7

#### formalized.service_invoice_1 Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | copied_from:Service Invoice |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:Service Invoice |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator:service_invoice_1.service_2.service_name |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from:Service Invoice |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:Service Invoice |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | copied_from:Service Invoice |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | copied_from:Service Invoice |

- _item_audit: 7

### `document`: Insurance Document
  - `uqi_prefix`: formalized.insurance_document_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md
  - `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md
  - `note`: Счет за страховку

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | Счет на оплату №26-00378-tl/1 от 14.01.2026 г. | CD | наименование документа | copied_from:Insurance Document |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | copied_from:Insurance Document |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | copied_from:Insurance Document |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | основной текст/условия | operator:insurance_document_1.textpara_storage |
| 06 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 07 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 08 | doc_code | 04111 | CD | код документа | константа |
| 09 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | константа |
| 10 | doc_number | 26-00378-tl/1 | CD | номер документа | copied_from:formalized.insurance_document_1.DocumentHead_DocumentNumber |
| 11 | doc_date | 14.01.2026 | CD | дата документа | copied_from:formalized.insurance_document_1.DocumentHead_DocumentDate |
| 12 | insurance_to_border | 910.34 | CO | стоимость страхования продавцом | operator:insurance_document_1.insurance_to_border |
| 13 | insurance_currency | RUB | CD | валюта страхования | copied_from:Insurance Document |

- _audit: 13
- `doc_status`: confirmed

### `document`: TechDescription
  - `uqi_prefix`: formalized.tech_description_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md
  - `file_name`: техничка Антикот, антипыльца антимошка .md
  - `note`: Техническое описание

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | Технические характеристики — Сетки из полиэстера 5804101000 / Сетки из стекловолокна 7019900095 | CD | наименование техописания | copied_from:TechDescription |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата техописания | operator:tech_description_1.date |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания | operator:tech_description_1.number |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md | CD | технический текст без пересказа | copied_from:TechDescription |
| 06 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 07 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 08 | doc_code | 05999 | CD | код документа | константа |
| 09 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | константа |
| 10 | doc_number | Б/Н | CD | номер документа | copied_from:formalized.tech_description_1.DocumentHead_DocumentNumber |
| 11 | doc_date | 30.10.2025 | CD | дата документа | copied_from:formalized.tech_description_1.DocumentHead_DocumentDate |

- _audit: 11
- `doc_status`: confirmed

### `document`: EGRUL
  - `uqi_prefix`: master_data.egrul_1
  - `xml_target_root`: 
  - `path`: alta\master_data\master_data.md
  - `file_name`: master_data.md
  - `note`: Выписка из ЕГРЮЛ

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ \"СКИФ\" | CD | наименование организации | master_data:master_data.md |
| 02 | ShortName | ООО \"СКИФ\" | CD | краткое наименование | master_data:master_data.md |
| 03 | OGRN | 1201600020390 | CD | ОГРН | master_data:master_data.md |
| 04 | INN | 1650389298 | CD | ИНН | master_data:master_data.md |
| 05 | KPP | 165001001 | CD | КПП | master_data:master_data.md |
| 06 | Address_PostalCode | 423800 | CD | индекс | master_data:master_data.md |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | master_data:master_data.md |
| 08 | Address_CounryName | РОССИЯ | CD | страна, текст | master_data:master_data.md |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data:master_data.md |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data:master_data.md |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | master_data:master_data.md |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | master_data:master_data.md |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | master_data:master_data.md |
| 14 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 15 | kind_code | 2 | CD | подавался ли ранее документ в таможню | константа |
| 16 | dt_number | 10418010/150725/5103886 | CD | номер ДТ, в которой подавался документ | master_data:master_data.md |
| 17 | doc_code | 04011 | CD | код документа | константа |
| 18 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | константа |
| 19 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | master_data:master_data.md |
| 20 | doc_date | 14.07.2025 | CD | дата документа | master_data:master_data.md |

- _audit: 20
- `doc_status`: confirmed

### `document`: Personal Passport
  - `uqi_prefix`: master_data.passport_1
  - `xml_target_root`: 
  - `path`: alta\master_data\master_data.md
  - `file_name`: master_data.md
  - `note`: Паспорт представителя

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | master_data:master_data.md |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | master_data:master_data.md |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | master_data:master_data.md |
| 04 | CardSeries | 63 09 | CD | серия | master_data:master_data.md |
| 05 | CardNumber | 449948 | CD | номер | master_data:master_data.md |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | master_data:master_data.md |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data:master_data.md |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_data:master_data.md |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | master_data:master_data.md |
| 10 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 11 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 12 | doc_code | 11001 | CD | код документа | константа |
| 13 | doc_name | ПАСПОРТ | CD | наименование документа | константа |
| 14 | doc_number | 63 09 449948 | CD | номер документа | master_data:master_data.md |
| 15 | doc_date | 11.03.2010 | CD | дата документа | master_data:master_data.md |

- _audit: 15
- `doc_status`: confirmed

### `document`: Letter of Attorney
  - `uqi_prefix`: master_data.letter_of_attorney_1
  - `xml_target_root`: 
  - `path`: alta\master_data\master_data.md
  - `file_name`: master_data.md
  - `note`: Доверенность представителя

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data:master_data.md |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_data:master_data.md |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_data:master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_data:master_data.md |
| 05 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 06 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 07 | doc_code | 11004 | CD | код документа | константа |
| 08 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | константа |
| 09 | doc_number | 1 | CD | номер документа | master_data:master_data.md |
| 10 | doc_date | 01.02.2026 | CD | дата документа | master_data:master_data.md |

- _audit: 10
- `doc_status`: confirmed

### `document`: Transport Contract
  - `uqi_prefix`: master_data.transport_contract_1
  - `xml_target_root`: 
  - `path`: alta\master_data\master_data.md
  - `file_name`: master_data.md
  - `note`: Договор транспортной экспедиции

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 02 | kind_code | 2 | CD | подавался ли ранее документ в таможню | константа |
| 03 | dt_number | 10418010/150725/5103886 | CD | номер ДТ, в которой подавался документ | master_data:master_data.md |
| 04 | doc_code | 04033 | CD | код документа | константа |
| 05 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование документа | константа |
| 06 | doc_number | КООО/26651/М | CD | номер документа | master_data:master_data.md |
| 07 | doc_date | 13.05.2025 | CD | дата документа | master_data:master_data.md |

- _audit: 7
- `doc_status`: confirmed

### `document`: Exemption Letter
  - `uqi_prefix`: master_data.exemption_letter_1
  - `xml_target_root`: 
  - `path`: alta\master_data\master_data.md
  - `file_name`: master_data.md
  - `note`: Отказное письмо

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 02 | kind_code | 2 | CD | подавался ли ранее документ в таможню | константа |
| 03 | dt_number | 10418010/220825/5128789 | CD | номер ДТ, в которой подавался документ | master_data:master_data.md |
| 04 | doc_code | 09023 | CD | код документа | константа |
| 05 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | константа |
| 06 | doc_number | 24968/МЛ10 | CD | номер документа | master_data:master_data.md |
| 07 | doc_date | 20.08.2025 | CD | дата документа | master_data:master_data.md |

- _audit: 7
- `doc_status`: confirmed

### `document`: Exemption Letter (source)
  - `uqi_prefix`: master_data.exemption_letter_source_1
  - `xml_target_root`: 
  - `path`: alta\master_data\master_data.md
  - `file_name`: master_data.md
  - `note`: Отказное письмо (источник)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 02 | kind_code | 2 | CD | подавался ли ранее документ в таможню | константа |
| 03 | dt_number | 10418010/220825/5128789 | CD | номер ДТ, в которой подавался документ | master_data:master_data.md |
| 04 | doc_code | 09999 | CD | код документа | константа |
| 05 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | константа |
| 06 | doc_number | 24968/МЛ10 | CD | номер документа | master_data:master_data.md |
| 07 | doc_date | 20.08.2025 | CD | дата документа | master_data:master_data.md |

- _audit: 7
- `doc_status`: confirmed

### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.td_1
  - `xml_target_root`: 
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ТД 10719110_240126_5011363_reg00378тд.md
  - `file_name`: ТД 10719110_240126_5011363_reg00378тд.md
  - `note`: Транзитная декларация

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs_post_code | 10719110 | CD | код таможенного органа | copied_from:Transit Declaration |
| 02 | customs_post_name | Таможенный орган отправления | CD | наименование таможенного органа | copied_from:Transit Declaration |
| 03 | transport_reg_number | O157AO774/BT374974 | CD | ТС по ТД | copied_from:Transit Declaration |
| 04 | doc_gr44 | true | CD | служебный признак включения в графу 44 | константа |
| 05 | kind_code | 0 | CD | подавался ли ранее документ в таможню | константа |
| 06 | doc_code | 09013 | CD | код документа | константа |
| 07 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | наименование документа | константа |
| 08 | doc_number | 10719110/240126/5011363 | CO | номер документа | operator:transit_declaration_1.number |
| 09 | doc_date | 24.01.2026 | CO | дата документа | operator:transit_declaration_1.date |

- _audit: 9
- `doc_status`: confirmed

### `document`: Storage Report
  - `uqi_prefix`: non_formalized.svh_1
  - `xml_target_root`: 
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО 14431420260204161621.md
  - `file_name`: ДО 14431420260204161621.md
  - `note`: Отчет СВХ (ДО-1)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии/свидетельства СВХ | copied_from:Storage Report |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии/свидетельства СВХ | copied_from:Storage Report |
| 03 | actual_gross_weight | 3500 | CO | фактический вес по весам | operator:svh_1.actual_totals_from_svh_additional_sheet |
| 04 | actual_places | 127 | CO | фактическое количество мест | operator:svh_1.actual_totals_from_svh_additional_sheet |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС при въезде | copied_from:Storage Report |
| 06 | doc_gr44 | false | CD | служебный признак включения в графу 44 | константа |

- _audit: 6
- `doc_status`: confirmed

#### non_formalized.svh_1 Массив: goods[2]
- _array_audit: 2

#### non_formalized.svh_1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 7019900095 | CD | код товара | copied_from:Storage Report |
| 02 | places | 27 | CD | кол-во грузовых мест по строке | copied_from:Storage Report |
| 03 | gross_weight_kg | 1710 | CD | вес брутто по строке | copied_from:Storage Report |
| 04 | cost | 42228 | CD | стоимость по строке | copied_from:Storage Report |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from:Storage Report |

- _item_audit: 5

#### non_formalized.svh_1 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара | copied_from:Storage Report |
| 02 | places | 100 | CD | кол-во грузовых мест по строке | copied_from:Storage Report |
| 03 | gross_weight_kg | 1790 | CD | вес брутто по строке | copied_from:Storage Report |
| 04 | cost | 55032 | CD | стоимость по строке | copied_from:Storage Report |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from:Storage Report |

- _item_audit: 5

### `document`: Storage Report Additional Sheet
  - `uqi_prefix`: non_formalized.svh_additional_sheet_1
  - `xml_target_root`: 
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО доп 14431520260204161645.md
  - `file_name`: ДО доп 14431520260204161645.md
  - `note`: Добавочный лист к отчету СВХ

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 0000080 | CO | № доп.листа/приложения | operator:svh_1.number |
| 02 | date | 03.02.2026 | CO | дата доп.листа | operator:svh_1.date |
| 03 | actual_gross_weight | 3500 | CD | фактический вес по весам | copied_from:Storage Report Additional Sheet |
| 04 | actual_places | 127 | CD | фактическое количество мест | copied_from:Storage Report Additional Sheet |
| 05 | transport_reg_number | O157AO774/BT374974 | CO | номер ТС при въезде | operator:svh_additional_sheet_1.address_from_cmr |
| 06 | svh_address_region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион СВХ | operator:svh_additional_sheet_1.svh_address_region |
| 07 | svh_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город/нас.пункт СВХ | operator:svh_additional_sheet_1.svh_address_city |
| 08 | svh_address_street_house | Производственный пр-д, д. 45 | CO | улица/дом СВХ одной строкой | operator:svh_additional_sheet_1.svh_address_street_house |
| 09 | svh_customs_code | 10404083 | CO | код таможенного органа в зоне СВХ | operator:svh_additional_sheet_1.svh_customs_code |
| 10 | doc_gr44 | false | CD | служебный признак включения в графу 44 | константа |

- _audit: 10
- `doc_status`: confirmed

### Итогo, по файлу:

`total_unreliable_fields`: 0
`formalization_status`: confirmed

## 5. Нерешенные вопросы (Issues)

**Для полей:**
- Нет.

**Для общих вопросов:**
- `[Общий]`
  - `question`: Нет.

## 6. `unreliable_fields`:
- Нет.
