# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ
- `источники данных`: md + operator_provided_data + master_data + stable_source

## 2. formalized

### `document`: Contract
  - `num`: 1
  - `uqi_prefix`: formalized.contract_1
  - `xml_target_root`: AltaE2CONT
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md
  - `file_name`: SALES CONTRACT NoLM-2553.md
  - `note`: Основной контракт купли-продажи

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 03011 | CD | код вида документа | константа |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта | copied_from:SALES CONTRACT NoLM-2553.md |
| 03 | ContractRegistration_PrDocumentDate | 02.07.2025 | CD | дата контракта | copied_from:SALES CONTRACT NoLM-2553.md |
| 04 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта | operator:formalized.contract_1.ContractTerms_Amount (Допсоглашение №1) |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты | operator:formalized.contract_1.currency_code_numeric (CNY) |
| 06 | ContractTerms_LastDate | 31.12.2026 | CD | срок действия/исполнения | copied_from:SALES CONTRACT NoLM-2553.md |
| 07 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки / Incoterms | operator:formalized.contract_1.delivery_terms |
| 08 | ContractTerms_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md | CD | текст контракта | ссылка на md-файл |
| 09 | ContractTerms_DealSign | 1 | CO | системный признак Альты | operator:formalized.contract_1.deal_sign |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта | copied_from:SALES CONTRACT NoLM-2553.md |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 | operator:formalized.contract_1.foreign_person_country_code_alpha2 |
| 12 | ForeignPerson_Address_CounryName | Китай | CD | страна продавца, текст | copied_from:SALES CONTRACT NoLM-2553.md (русский перевод) |
| 13 | ForeignPerson_Address_Region | Hebei | CD | регион/область продавца | copied_from:SALES CONTRACT NoLM-2553.md |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца | copied_from:SALES CONTRACT NoLM-2553.md |
| 15 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом продавца одной строкой | operator:formalized.contract_1.foreign_person_address_line |
| 16 | RussianPerson_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | покупатель/сторона контракта | master_data:master_data.md (declarant.organization_name) |
| 17 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН покупателя | master_data:master_data.md (declarant.ogrn); allow_cross_doc_master_data_to_contract_invoice |
| 18 | RussianPerson_INN | 1650389298 | CO | ИНН покупателя | master_data:master_data.md (declarant.inn); allow_cross_doc_master_data_to_contract_invoice |
| 19 | RussianPerson_KPP | 165001001 | CO | КПП покупателя | master_data:master_data.md (declarant.kpp); allow_cross_doc_master_data_to_contract_invoice |
| 20 | RussianPerson_Address_PostalCode | 423800 | CD | индекс покупателя | master_data:master_data.md (declarant.postal_code) |
| 21 | RussianPerson_Address_CountryCode | RU | CD | страна покупателя alpha-2 | master_data:master_data.md (declarant.country_code) |
| 22 | RussianPerson_Address_CounryName | РОССИЯ | CD | страна покупателя, текст | master_data:master_data.md (declarant.country_name) |
| 23 | RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион покупателя | master_data:master_data.md (declarant.region) |
| 24 | RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город покупателя | master_data:master_data.md (declarant.city) |
| 25 | RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator:formalized.contract_1.russian_person_address_line |
| 26 | doc_code | 03011 | CD | код вида документа | константа |
| 27 | doc_name | КОНТРАКТ | CD | наименование документа | константа |
| 28 | doc_number | LM-2553 | CD | номер документа | копируемое поле |
| 29 | doc_date | 02.07.2025 | CD | дата документа | копируемое поле |

- _audit: 29
- `doc_status`: confirmed

### `document`: Supplementary Contract
  - `num`: 1
  - `uqi_prefix`: formalized.supplementary_contract_1
  - `xml_target_root`: AltaSupplementaryContract
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md
  - `file_name`: 1 Supplementary agreement to the contract.md
  - `note`: Дополнительное соглашение №1 к контракту

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения | copied_from:1 Supplementary agreement to the contract.md |
| 02 | IssueDate | 25.11.2025 | CD | дата доп. соглашения | copied_from:1 Supplementary agreement to the contract.md |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта | copied_from:1 Supplementary agreement to the contract.md |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты | operator:formalized.supplementary_contract_1.currency_code_numeric (CNY) |
| 05 | ContractDescription_LastDate | 31.12.2026 | CO | новый срок действия/исполнения | operator:formalized.supplementary_contract_1.expiry_date |
| 06 | ContractDescription_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md | CD | текст доп. соглашения | ссылка на md-файл |
| 07 | ContractDescription_DealSign | 1 | CO | системный признак Альты | operator:formalized.supplementary_contract_1.deal_sign |
| 08 | ContractDescription_StockCategorySign | 0 | CO | системный признак Альты | operator:formalized.supplementary_contract_1.stock_category_sign |
| 09 | ContractDescription_BuyerLimitationSign | 0 | CO | системный признак Альты | operator:formalized.supplementary_contract_1.buyer_limitation_sign |
| 10 | ContractDescription_InsuranceSign | 0 | CO | системный признак Альты | operator:formalized.supplementary_contract_1.insurance_sign |
| 11 | RussianPerson_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | российская сторона | master_data:master_data.md (declarant.organization_name) |
| 12 | RussianPerson_ShortName | ООО "СКИФ" | CO | краткое наименование | operator:formalized.supplementary_contract_1.russian_person_short_name |
| 13 | RussianPerson_OGRN | 1201600020390 | CD | ОГРН | master_data:master_data.md (declarant.ogrn) |
| 14 | RussianPerson_INN | 1650389298 | CD | ИНН | master_data:master_data.md (declarant.inn) |
| 15 | RussianPerson_KPP | 165001001 | CD | КПП | master_data:master_data.md (declarant.kpp) |
| 16 | RussianPerson_Address_PostalCode | 423800 | CD | индекс | master_data:master_data.md (declarant.postal_code) |
| 17 | RussianPerson_Address_CountryCode | RU | CD | страна alpha-2 | master_data:master_data.md (declarant.country_code) |
| 18 | RussianPerson_Address_CounryName | РОССИЯ | CD | страна, текст | master_data:master_data.md (declarant.country_name) |
| 19 | RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data:master_data.md (declarant.region) |
| 20 | RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data:master_data.md (declarant.city) |
| 21 | RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CD | улица/дом одной строкой | master_data:master_data.md (declarant.street_house) |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | иностранная сторона | copied_from:1 Supplementary agreement to the contract.md |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator:formalized.supplementary_contract_1.foreign_person_short_name_equals_full |
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 | operator:formalized.supplementary_contract_1.foreign_person_country_code_alpha2 |
| 25 | ForeignPerson_Address_CounryName | Китай | CD | страна, текст | copied_from:1 Supplementary agreement to the contract.md (русский перевод) |
| 26 | ForeignPerson_Address_Region | Hebei | CO | регион | copied_from:formalized.contract_1.ForeignPerson_Address_Region; formalized.supplementary_contract_1.foreign_person_address_from_contract |
| 27 | ForeignPerson_Address_City | Shijiazhuang | CO | город/район | copied_from:formalized.contract_1.ForeignPerson_Address_City; formalized.supplementary_contract_1.foreign_person_address_from_contract |
| 28 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом одной строкой | copied_from:formalized.contract_1.ForeignPerson_Address_StreetHouse; formalized.supplementary_contract_1.foreign_person_address_from_contract |
| 29 | PersonSurname | Li | CO | фамилия подписанта | operator:formalized.supplementary_contract_1.signed_person_surname |
| 30 | PersonName | Jing | CO | имя подписанта | operator:formalized.supplementary_contract_1.signed_person_name |
| 31 | PersonMiddleName | | CO | отчество подписанта | operator:formalized.supplementary_contract_1.signed_person_middle_name |
| 32 | doc_code | 03012 | CD | код вида документа | константа |
| 33 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наименование документа | константа |
| 34 | doc_number | 1 | CD | номер документа | копируемое поле |
| 35 | doc_date | 25.11.2025 | CD | дата документа | копируемое поле |

- _audit: 35
- `doc_status`: confirmed

### `document`: Invoice
  - `num`: 1
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\CL на сетку .md
  - `file_name`: CL на сетку .md
  - `note`: Коммерческий инвойс LM-2591

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты | operator:formalized.invoice_1.exchange_rate |
| 02 | CurrencyCode | CNY | CO | валюта инвойса ISO 4217 alpha-3 | operator:formalized.invoice_1.currency_code |
| 03 | DocumentCode | 04021 | CD | код вида документа | константа |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест | copied_from:CL на сетку .md |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator:formalized.invoice_1.places_description |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто по инвойсу | operator:formalized.invoice_1.total_gross_weight (из PL totals) |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто по инвойсу | operator:formalized.invoice_1.total_net_weight (из PL totals) |
| 08 | GCost | 97260.00 | CO | системное поле Альты | operator:formalized.invoice_1.gcost |
| 09 | TotalCost | 97260.00 | CO | итого по инвойсу | operator:formalized.invoice_1.total_cost |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | copied_from:CL на сетку .md |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator:formalized.invoice_1.delivery_terms_numeric (EXW) |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator:formalized.invoice_1.delivery_terms_string |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | operator:formalized.invoice_1.dispatch_country_code |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | operator:formalized.invoice_1.trading_country_code |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | operator:formalized.invoice_1.destination_country_code |
| 16 | Registration_PrDocumentName | Commercial invoice / Комерческий инвойс | CD | наименование документа | copied_from:CL на сетку .md |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | copied_from:CL на сетку .md |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | copied_from:CL на сетку .md |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | copied_from:CL на сетку .md |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | copied_from:CL на сетку .md |
| 21 | Buyer_CompanyID | 1650389298 | CO | ИНН покупателя | master_data:master_data.md (declarant.inn); allow_cross_doc_master_data_to_contract_invoice |
| 22 | Buyer_KPPCode | 165001001 | CO | КПП покупателя | master_data:master_data.md (declarant.kpp); allow_cross_doc_master_data_to_contract_invoice |
| 23 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CO | наименование покупателя | master_data:master_data.md (declarant.organization_name) |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CO | индекс покупателя | master_data:master_data.md (declarant.postal_code) |
| 25 | Buyer_PostalAddress_CountryCode | RU | CO | страна покупателя alpha-2 | master_data:master_data.md (declarant.country_code) |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CO | страна покупателя, текст | master_data:master_data.md (declarant.country_name) |
| 27 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион | master_data:master_data.md (declarant.region) |
| 28 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город | master_data:master_data.md (declarant.city) |
| 29 | Buyer_PostalAddress_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис покупателя | operator:formalized.contract_1.russian_person_address_line |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | продавец | copied_from:CL на сетку .md |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | operator:formalized.invoice_1.seller_country_code_alpha2 |
| 32 | Seler_PostalAddress_CounryName | Китай | CD | страна продавца, текст | copied_from:CL на сетку .md (русский перевод) |
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | copied_from:CL на сетку .md |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город/район продавца | copied_from:CL на сетку .md |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом продавца | copied_from:CL на сетку .md |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | copied_from:formalized.invoice_1.Seler_Name; normalisation: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | copied_from:formalized.invoice_1.Seler_PostalAddress_CountryCode; normalisation: consignor=seller |
| 38 | Consignor_Address_CounryName | Китай | CD | страна грузоотправителя, текст | copied_from:formalized.invoice_1.Seler_PostalAddress_CounryName; normalisation: consignor=seller |
| 39 | Consignor_Address_Region | Hebei | CD | регион | copied_from:formalized.invoice_1.Seler_PostalAddress_Region; normalisation: consignor=seller |
| 40 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from:formalized.invoice_1.Seler_PostalAddress_City; normalisation: consignor=seller |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse; normalisation: consignor=seller |
| 42 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data:master_data.md (declarant.organization_name) |
| 43 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data:master_data.md (declarant.ogrn) |
| 44 | Consignee_INN | 1650389298 | CD | ИНН | master_data:master_data.md (declarant.inn) |
| 45 | Consignee_KPP | 165001001 | CD | КПП | master_data:master_data.md (declarant.kpp) |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | master_data:master_data.md (declarant.postal_code) |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | master_data:master_data.md (declarant.country_code) |
| 48 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | master_data:master_data.md (declarant.country_name) |
| 49 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data:master_data.md (declarant.region) |
| 50 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data:master_data.md (declarant.city) |
| 51 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CD | улица/дом/офис одной строкой | master_data:master_data.md (declarant.street_house) |
| 52 | doc_code | 04021 | CD | код вида документа | константа |
| 53 | doc_name | ИНВОЙС | CD | наименование документа | константа |
| 54 | doc_number | LM-2591 | CD | номер инвойса | копируемое поле |
| 55 | doc_date | 30.10.2025 | CD | дата инвойса | копируемое поле |

- _audit: 55
- `doc_status`: confirmed

#### Массив: InvoiceGoods[7]

#### InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:CL на сетку .md |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара | copied_from:CL на сетку .md |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке | copied_from:CL на сетку .md |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | copied_from:CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:CL на сетку .md (код 055) |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_1.gross_weight |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator:formalized.invoice_1.goods_1.net_weight |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from:CL на сетку .md |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | copied_from:CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric (Китай) |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:CL на сетку .md |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара | copied_from:CL на сетку .md |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке | copied_from:CL на сетку .md |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | copied_from:CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:CL на сетку .md (код 055) |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_2.gross_weight |
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator:formalized.invoice_1.goods_2.net_weight |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from:CL на сетку .md |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | copied_from:CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric (Китай) |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:CL на сетку .md |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | CD | описание товара | copied_from:CL на сетку .md |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке | copied_from:CL на сетку .md |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | copied_from:CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:CL на сетку .md (код 055) |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_3.gross_weight |
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator:formalized.invoice_1.goods_3.net_weight |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from:CL на сетку .md |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | copied_from:CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric (Китай) |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:CL на сетку .md |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CD | описание товара | copied_from:CL на сетку .md |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке | copied_from:CL на сетку .md |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | copied_from:CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:CL на сетку .md (код 055) |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_4.gross_weight |
| 08 | NetWeightQuantity | 144.00 | CO | нетто по строке | operator:formalized.invoice_1.goods_4.net_weight |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from:CL на сетку .md |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | copied_from:CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric (Китай) |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from:CL на сетку .md |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара | copied_from:CL на сетку .md |
| 03 | GoodsQuantity | 90 | CD | кол-во по строке | copied_from:CL на сетку .md |
| 04 | goods_supplementary_quantity | 3780 | CD | количество в доп.ед.изм | copied_from:CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:CL на сетку .md (код 055) |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_5.gross_weight |
| 08 | NetWeightQuantity | 491.40 | CO | нетто по строке | operator:formalized.invoice_1.goods_5.net_weight |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from:CL на сетку .md |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | copied_from:CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric (Китай) |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from:CL на сетку .md |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара | copied_from:CL на сетку .md |
| 03 | GoodsQuantity | 180 | CD | кол-во по строке | copied_from:CL на сетку .md |
| 04 | goods_supplementary_quantity | 8640 | CD | количество в доп.ед.изм | copied_from:CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:CL на сетку .md (код 055) |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_6.gross_weight |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто по строке | operator:formalized.invoice_1.goods_6.net_weight |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from:CL на сетку .md |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | copied_from:CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric (Китай) |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from:CL на сетку .md |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | copied_from:CL на сетку .md |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке | copied_from:CL на сетку .md |
| 04 | goods_supplementary_quantity | 240 | CD | количество в доп.ед.изм | copied_from:CL на сетку .md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from:CL на сетку .md |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | copied_from:CL на сетку .md (код 055) |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто по строке | operator:formalized.invoice_1.goods_7.gross_weight |
| 08 | NetWeightQuantity | 24.00 | CO | нетто по строке | operator:formalized.invoice_1.goods_7.net_weight |
| 09 | Price | 28 | CD | цена за единицу | copied_from:CL на сетку .md |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | copied_from:CL на сетку .md |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения | operator:formalized.invoice_1.goods_all.origin_country_code_numeric (Китай) |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator:formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator:formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator:formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator:formalized.invoice_1.goods_all.model |

- _item_audit: 15
- _array_audit: 7
- `array_status`: confirmed

### `document`: Packing List
  - `num`: 1
  - `uqi_prefix`: formalized.packing_list_1
  - `xml_target_root`: AltaE2PACK
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\PL на сетку .md
  - `file_name`: PL на сетку .md
  - `note`: Упаковочный лист LM-2591

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто по упаковочному | copied_from:PL на сетку .md |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто по упаковочному | copied_from:PL на сетку .md |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | грузоотправитель | copied_from:formalized.contract_1.ForeignPerson_OrganizationName; normalisation: consignor=seller |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator:formalized.packing_list_1.consignor_shortname_equals_full |
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | operator:formalized.packing_list_1.consignor_country_code_alpha2 |
| 06 | Consignor_Address_CounryName | Китай | CD | страна грузоотправителя, text | copied_from:SALES CONTRACT NoLM-2553.md (русский перевод) |
| 07 | Consignor_Address_Region | Hebei | CO | регион | copied_from:formalized.contract_1.ForeignPerson_Address_Region |
| 08 | Consignor_Address_City | Shijiazhuang | CO | город/район | copied_from:formalized.contract_1.ForeignPerson_Address_City |
| 09 | Consignor_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом одной строкой | copied_from:formalized.contract_1.ForeignPerson_Address_StreetHouse |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data:master_data.md (declarant.organization_name) |
| 11 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator:formalized.packing_list_1.consignee_shortname_equals_full |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data:master_data.md (declarant.ogrn) |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | master_data:master_data.md (declarant.inn) |
| 14 | Consignee_KPP | 165001001 | CD | КПП | master_data:master_data.md (declarant.kpp) |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | master_data:master_data.md (declarant.postal_code) |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | master_data:master_data.md (declarant.country_code) |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | master_data:master_data.md (declarant.country_name) |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data:master_data.md (declarant.region) |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data:master_data.md (declarant.city) |
| 20 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CD | улица/дом/офис одной строкой | master_data:master_data.md (declarant.street_house) |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | copied_from:PL на сетку .md |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | внутренний числовой код условий | operator:formalized.invoice_1.delivery_terms_numeric (EXW) |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from:PL на сетку .md |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | наименование контракта | operator:formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentName |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | copied_from:PL на сетку .md |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | copied_from:PL на сетку .md |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | константа |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | copied_from:PL на сетку .md |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | copied_from:PL на сетку .md |
| 30 | DeliveryTerms_Registration_PrDocumentName | Упаковочный лист | CO | наименование упаковочного | operator:formalized.packing_list_1.registration_doc_name |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | operator:formalized.packing_list_1.registration_doc_number |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | operator:formalized.packing_list_1.registration_doc_date |
| 33 | doc_code | 04131 | CD | код вида документа | константа |
| 34 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | константа |
| 35 | doc_number | LM-2591 | CD | номер документа | копируемое поле |
| 36 | doc_date | 30.10.2025 | CD | дата документа | копируемое поле |

- _audit: 36
- `doc_status`: confirmed

#### Массив: Goods[7]

#### Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки | copied_from:PL на сетку .md |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | copied_from:PL на сетку .md |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | copied_from:PL на сетку .md |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | copied_from:PL на сетку .md |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок | operator:formalized.packing_list_1.goods_1.paking_quantity |

- _item_audit: 5

#### Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки | copied_from:PL на сетку .md |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц | copied_from:PL на сетку .md |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто по строке | copied_from:PL на сетку .md |
| 04 | NetWeightQuantity | 460.80 | CD | нетто по строке | copied_from:PL на сетку .md |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок | operator:formalized.packing_list_1.goods_2.paking_quantity |

- _item_audit: 5

#### Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера . Размер рулона 1,42*0,64*0,22 | CD | описание строки | copied_from:PL на сетку .md |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | copied_from:PL на сетку .md |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто по строке | copied_from:PL на сетку .md |
| 04 | NetWeightQuantity | 252.00 | CD | нетто по строке | copied_from:PL на сетку .md |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок | operator:formalized.packing_list_1.goods_3.paking_quantity |

- _item_audit: 5

#### Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание строки | copied_from:PL на сетку .md |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц | copied_from:PL на сетку .md |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто по строке | copied_from:PL на сетку .md |
| 04 | NetWeightQuantity | 144.00 | CD | нетто по строке | copied_from:PL на сетку .md |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок | operator:formalized.packing_list_1.goods_4.paking_quantity |

- _item_audit: 5

#### Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА " Антимошка " из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание строки | copied_from:PL на сетку .md |
| 02 | GoodsQuantity | 90 | CD | количество мест/грузовых единиц | copied_from:PL на сетку .md |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто по строке | copied_from:PL на сетку .md |
| 04 | NetWeightQuantity | 491.40 | CD | нетто по строке | copied_from:PL на сетку .md |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок | operator:formalized.packing_list_1.goods_5.paking_quantity |

- _item_audit: 5

#### Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание строки | copied_from:PL на сетку .md |
| 02 | GoodsQuantity | 180 | CD | количество мест/грузовых единиц | copied_from:PL на сетку .md |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | copied_from:PL на сетку .md |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто по строке | copied_from:PL на сетку .md |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок | operator:formalized.packing_list_1.goods_6.paking_quantity |

- _item_audit: 5

#### Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки "Антипыльца " из полиэстера Размер рулона 1,72*0,35*0,31*1 | CD | описание строки | copied_from:PL на сетку .md |
| 02 | GoodsQuantity | 5 | CD | количество мест/грузовых единиц | copied_from:PL на сетку .md |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто по строке | copied_from:PL на сетку .md |
| 04 | NetWeightQuantity | 24.00 | CD | нетто по строке | copied_from:PL на сетку .md |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок | operator:formalized.packing_list_1.goods_7.paking_quantity |

- _item_audit: 5
- _array_audit: 7
- `array_status`: confirmed

#### Массив: TransportMeans[2]

#### TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | O157AO774 | CO | регистрационный номер | operator:formalized.packing_list_1.transport_1.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator:formalized.packing_list_1.transport_1.mode_code |
| 03 | NationalityCode | 000 | CO | код “национальности” ТС | operator:formalized.packing_list_1.transport_1.nationality_code |
| 04 | MoverIndicator | true | CO | признак тягача | operator:formalized.packing_list_1.transport_1.mover_indicator |

- _item_audit: 4

#### TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | BT374974 | CO | регистрационный номер | operator:formalized.packing_list_1.transport_2.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator:formalized.packing_list_1.transport_2.mode_code |
| 03 | NationalityCode | 000 | CO | код “национальности” ТС | operator:formalized.packing_list_1.transport_2.nationality_code |
| 04 | MoverIndicator | false | CO | признак тягача | operator:formalized.packing_list_1.transport_2.mover_indicator |

- _item_audit: 4
- _array_audit: 2
- `array_status`: confirmed

### `document`: CMR
  - `num`: 1
  - `uqi_prefix`: formalized.cmr_1
  - `xml_target_root`: AltaE3CMR
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\СМР от СВХ.md
  - `file_name`: СМР от СВХ.md
  - `note`: Международная товарно-транспортная накладная № 00378

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CO | язык документа | operator:formalized.cmr_1.language_code |
| 02 | CMR_Choice | 1 | CO | системный выбор/вариант Альты | operator:formalized.cmr_1.cmr_choice |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | copied_from:СМР от СВХ.md |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | copied_from:СМР от СВХ.md (дата принятия груза п.4) |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления | operator:formalized.cmr_1.registration_place |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата CMR | copied_from:СМР от СВХ.md (TakingCargoDate п.4) |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2 | operator:formalized.cmr_1.taking_cargo_country_code_alpha2 |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | Китай | CD | страна принятия груза, текст | copied_from:СМР от СВХ.md |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | operator:formalized.cmr_1.delivery_country_code_alpha2 |
| 10 | DeliveryPlace_CounryName | Россия | CD | страна доставки, текст | copied_from:СМР от СВХ.md |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки по Incoterms | operator:formalized.cmr_1.delivery_terms_place |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator:formalized.cmr_1.delivery_terms_string |
| 13 | GoodsQuantity | 127 | CD | общее количество грузовых мест | copied_from:СМР от СВХ.md |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто по CMR | copied_from:СМР от СВХ.md |
| 15 | CMRTransport_PrimeMoverStateSignID | O157AO774 | CD | гос. номер тягача | copied_from:СМР от СВХ.md |
| 16 | CMRTransport_TrailerStateSignID | BT374974 | CD | гос. номер прицепа | copied_from:СМР от СВХ.md |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | отправитель: наименование | copied_from:СМР от СВХ.md |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator:formalized.cmr_1.consignor_shortname_equals_full |
| 19 | Consignor_PostalAddress_CountryCode | CN | CO | страна alpha-2 | operator:formalized.packing_list_1.consignor_country_code_alpha2 |
| 20 | Consignor_Address_CounryName | Китай | CD | страна, текст | copied_from:СМР от СВХ.md |
| 21 | Consignor_Address_Region | Hebei | CD | регион | copied_from:СМР от СВХ.md |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from:СМР от СВХ.md |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from:СМР от СВХ.md |
| 24 | Consignor_Guarantee_OrganizationName | | CD | наименование гаранта | operator:formalized.cmr_1.consignor_guarantee_all (ОТСУТСТВУЕТ) |
| 25 | Consignor_Guarantee_ShortName | | CD | краткое наименование | operator:formalized.cmr_1.consignor_guarantee_all (ОТСУТСТВУЕТ) |
| 26 | Consignor_Guarantee_Address_CountryCode | | CD | страна alpha-2 | operator:formalized.cmr_1.consignor_guarantee_all (ОТСУТСТВУЕТ) |
| 27 | Consignor_Guarantee_Address_CounryName | | CD | страна, текст | operator:formalized.cmr_1.consignor_guarantee_all (ОТСУТСТВУЕТ) |
| 28 | Consignor_Guarantee_Address_Region | | CD | регион | operator:formalized.cmr_1.consignor_guarantee_all (ОТСУТСТВУЕТ) |
| 29 | Consignor_Guarantee_Address_City | | CD | город/район | operator:formalized.cmr_1.consignor_guarantee_all (ОТСУТСТВУЕТ) |
| 30 | Consignor_Guarantee_Address_StreetHouse | | CD | улица/дом одной строкой | operator:formalized.cmr_1.consignor_guarantee_all (ОТСУТСТВУЕТ) |
| 31 | Consignee_NameInf | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование получателя | master_data:master_data.md (declarant.organization_name) |
| 32 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator:formalized.cmr_1.consignee_shortname_equals_full |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | master_data:master_data.md (declarant.ogrn); operator:formalized.cmr_1.consignee_ogrn_from_master_data |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | copied_from:СМР от СВХ.md |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | copied_from:СМР от СВХ.md |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | copied_from:СМР от СВХ.md |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | master_data:master_data.md (declarant.country_code) |
| 38 | Consignee_Address_CounryName | Россия | CD | страна, текст | copied_from:СМР от СВХ.md |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | copied_from:СМР от СВХ.md |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | copied_from:СМР от СВХ.md |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом/офис одной строкой | copied_from:СМР от СВХ.md |
| 42 | doc_code | 02015 | CD | код вида документа | константа |
| 43 | doc_name | CMR | CD | наименование документа | константа |
| 44 | doc_number | 00378 | CD | номер документа | копируемое поле |
| 45 | doc_date | 20.01.2026 | CD | дата документа | копируемое поле (TakingCargoDate п.4) |

- _audit: 45
- `doc_status`: confirmed

#### Массив: CMRGoods[1]

#### CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | константа; авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CD | описание груза/товара | copied_from:non_formalized.svh_1.actual_gross_weight (ДО-1 содержал указание на инвойс); нормализация CMR без детализации |
| 03 | PakingQuantity | 127 | CO | кол-во упаковок | operator:formalized.cmr_1.goods_1.packing_quantity |

- _item_audit: 3
- _array_audit: 1
- `array_status`: confirmed

### `document`: Payment Order
  - `num`: 1
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_1_13.01.2026.md
  - `file_name`: currency_transfer_1_13.01.2026.md
  - `note`: Заявление на перевод валюты №1 (оплата по инвойсу)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator:formalized.payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator:formalized.payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | copied_from:currency_transfer_1_13.01.2026.md |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator:formalized.payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator:formalized.payment_order_all.priority |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from:currency_transfer_1_13.01.2026.md |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | copied_from:currency_transfer_1_13.01.2026.md |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | copied_from:currency_transfer_1_13.01.2026.md |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | copied_from:currency_transfer_1_13.01.2026.md |
| 10 | Payer_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | плательщик | master_data:master_data.md (declarant.organization_name) |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from:currency_transfer_1_13.01.2026.md |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator:formalized.payment_order_1.payer_kpp |
| 13 | Payer_Bank_BankName | Филиал «Центральный» Банка ВТБ (ПАО) в г. Москве | CO | реквизиты банка плательщика | master_data:master_data.md (declarant.bank_name); operator:formalized.payment_order_all.payer_bank_requisites_required (без полных реквизитов) |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | copied_from:currency_transfer_1_13.01.2026.md |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | реквизиты банка получателя | copied_from:currency_transfer_1_13.01.2026.md |
| 16 | PersonSurname | Саранов | CO | фамилия | operator:formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя | operator:formalized.payment_order_all.payer_sign.name |
| 18 | doc_code | 04023 | CD | код вида документа | константа |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | константа |
| 20 | doc_number | 1 | CD | номер документа | копируемое поле |
| 21 | doc_date | 13.01.2026 | CD | дата документа | копируемое поле |

- _audit: 21
- `doc_status`: confirmed

### `document`: Payment Order
  - `num`: 2
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_7_28.11.2025.md
  - `file_name`: currency_transfer_7_28.11.2025.md
  - `note`: Заявление на перевод валюты №7 (авансовый платеж)

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator:formalized.payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator:formalized.payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | copied_from:currency_transfer_7_28.11.2025.md |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator:formalized.payment_order_all.transaction_kind |
| 05 | Priority | 5 | CO | очередность | operator:formalized.payment_order_all.priority |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from:currency_transfer_7_28.11.2025.md |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | copied_from:currency_transfer_7_28.11.2025.md |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | copied_from:currency_transfer_7_28.11.2025.md |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | copied_from:currency_transfer_7_28.11.2025.md |
| 10 | Payer_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | плательщик | master_data:master_data.md (declarant.organization_name) |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from:currency_transfer_7_28.11.2025.md |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator:formalized.payment_order_2.payer_kpp |
| 13 | Payer_Bank_BankName | Филиал «Центральный» Банка ВТБ (ПАО) в г. Москве | CO | реквизиты банка плательщика | master_data:master_data.md (declarant.bank_name); operator:formalized.payment_order_all.payer_bank_requisites_required (без полных реквизитов) |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | copied_from:currency_transfer_7_28.11.2025.md |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT VTBRCNSHXXX; /СN767290000018 | CD | реквизиты банка получателя | copied_from:currency_transfer_7_28.11.2025.md |
| 16 | PersonSurname | Саранов | CO | фамилия | operator:formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя | operator:formalized.payment_order_all.payer_sign.name |
| 18 | doc_code | 04023 | CD | код вида документа | константа |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | константа |
| 20 | doc_number | 7 | CD | номер документа | копируемое поле |
| 21 | doc_date | 28.11.2025 | CD | дата документа | копируемое поле |

- _audit: 21
- `doc_status`: confirmed

### `document`: Service Invoice
  - `num`: 1
  - `uqi_prefix`: formalized.service_invoice_1
  - `xml_target_root`: AltaServiceInvoice
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_от_27-01-2026.md
  - `file_name`: Счет_№26-00378-tl_от_27-01-2026.md
  - `note`: Счет за транспортные услуги

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак документа Альты | operator:formalized.service_invoice_1.document_sign |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 03 | Currency | USD | CD | валюта итого ISO 4217 alpha-3 | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг/перевозчик | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 05 | BankName | АО "Райффайзенбанк"; БИК 044525700; Сч. № 30101810200000000700; Сч. № 40702810400000233463 | CD | банк исполнителя | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги/перевозку | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги/перевозку | copied_from:Счет_№26-00378-tl_от_27-01-2026.md (в тексте 13-05-2025) |
| 08 | PrDocumentNumber | ОТСУТСТВУЕТ | CO | номер связанного заказа | operator:formalized.service_invoice_1.payment_document_number |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CO | дата связанного заказа | operator:formalized.service_invoice_1.payment_document_date |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | грузоотправитель | copied_from:formalized.contract_1.ForeignPerson_OrganizationName; operator:formalized.service_invoice_1.consignor_decision |
| 14 | PostalCode | | CO | индекс | operator:formalized.service_invoice_1.consignor_postalcode_empty_ok |
| 15 | CountryCode | CN | CO | страна alpha-2 | operator:formalized.packing_list_1.consignor_country_code_alpha2; service_invoice_1.consignor_address_from_seller |
| 16 | CounryName | Китай | CO | страна, текст | copied_from:SALES CONTRACT NoLM-2553.md (русский перевод); service_invoice_1.consignor_address_from_seller |
| 17 | Region | Hebei | CO | регион | copied_from:formalized.contract_1.ForeignPerson_Address_Region; service_invoice_1.consignor_address_from_seller |
| 18 | Town | Shijiazhuang | CO | город/район | copied_from:formalized.contract_1.ForeignPerson_Address_City; service_invoice_1.consignor_address_from_seller |
| 19 | StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом одной строкой | copied_from:formalized.contract_1.ForeignPerson_Address_StreetHouse; service_invoice_1.consignor_address_from_seller |
| 20 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data:master_data.md (declarant.organization_name) |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | master_data:master_data.md (declarant.ogrn); operator:formalized.service_invoice_1.consignee_ogrn_from_master_data |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 24 | PostalCode | 423800 | CD | индекс | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 25 | CountryCode | RU | CD | страна alpha-2 | master_data:master_data.md (declarant.country_code) |
| 26 | CounryName | РОССИЯ | CD | страна, текст | master_data:master_data.md (declarant.country_name) |
| 27 | Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data:master_data.md (declarant.region) |
| 28 | Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data:master_data.md (declarant.city) |
| 29 | StreetHouse | проезд Хлебный | CD | улица | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 30 | House | 30 | CO | дом | operator:formalized.service_invoice_1.consignee_house |
| 31 | Room | 211 | CO | офис/кв | operator:formalized.service_invoice_1.consignee_room |
| 32 | Signature_Choice | 2 | CD | вариант подписи | copied_from:Счет_№26-00378-tl_от_27-01-2026.md (руководитель + бухгалтер) |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | пустой блок при Signature_Choice=2 |
| 34 | IndividualEntrepreneur_PersonName | | CD | имя ИП | пустой блок при Signature_Choice=2 |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | пустой блок при Signature_Choice=2 |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климовин | CD | фамилия руководителя | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л | CD | имя руководителя | copied_from:Счет_№26-00378-tl_от_27-01-2026.md (Л.А.) |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А | CD | отчество руководителя | copied_from:Счет_№26-00378-tl_от_27-01-2026.md (Л.А.) |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О | CD | имя бухгалтера | copied_from:Счет_№26-00378-tl_от_27-01-2026.md (О.А.) |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А | CD | отчество бухгалтера | copied_from:Счет_№26-00378-tl_от_27-01-2026.md (О.А.) |
| 42 | doc_code | 04031 | CD | код вида документа | константа |
| 43 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | константа |
| 44 | doc_number | 26-00378-tl | CD | номер счета | копируемое поле |
| 45 | doc_date | 27.01.2026 | CD | дата счета | копируемое поле |
| 46 | transport_to_border | 1404.00 | CO | стоимость до границы | operator:formalized.service_invoice_1.transport_to_border |
| 47 | transport_currency | USD | CD | валюта стоимости | copied_from:Счет_№26-00378-tl_от_27-01-2026.md (валюта первой строки) |

- _audit: 47
- `doc_status`: confirmed

#### Массив: ServiceDescription[2]

#### ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) - перевозка автотранспортом | CD | описание услуги | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 03 | ServiceName | | CO | наименование/маршрут | operator:formalized.service_invoice_1.service_1.service_name (ОТСУТСТВУЕТ) |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |

- _item_audit: 7

#### ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 03 | ServiceName | | CO | наименование/маршрут | operator:formalized.service_invoice_1.service_2.service_name (ОТСУТСТВУЕТ) |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | copied_from:Счет_№26-00378-tl_от_27-01-2026.md |

- _item_audit: 7
- _array_audit: 2
- `array_status`: confirmed

### `document`: Insurance Services Invoice
  - `num`: 1
  - `uqi_prefix`: formalized.insurance_document_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md
  - `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md
  - `note`: Счет на оплату страхования

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа | copied_from:Счет_№26-00378-tl_1_от_14-01-2026.md |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | copied_from:Счет_№26-00378-tl_1_от_14-01-2026.md |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | copied_from:Счет_№26-00378-tl_1_от_14-01-2026.md |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | основной текст/условия | operator:formalized.insurance_document_1.textpara_storage |
| 06 | doc_code | 04111 | CD | код вида документа | константа |
| 07 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | константа |
| 08 | doc_number | 26-00378-tl/1 | CD | номер документа | копируемое поле |
| 09 | doc_date | 14.01.2026 | CD | дата документа | копируемое поле |
| 10 | insurance_to_border | 910.34 | CO | стоимость страхования | operator:formalized.insurance_document_1.insurance_to_border |
| 11 | insurance_currency | RUB | CD | валюта страхования | copied_from:Счет_№26-00378-tl_1_от_14-01-2026.md (рубль) |

- _audit: 11
- `doc_status`: confirmed

### `document`: TechDescription
  - `num`: 1
  - `uqi_prefix`: formalized.tech_description_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md
  - `file_name`: техничка Антикот, антипыльца антимошка .md
  - `note`: Технические характеристики сеток

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование техописания | copied_from:техничка Антикот, антипыльца антимошка .md |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата техописания | operator:formalized.tech_description_1.date (по дате отгрузки/инвойса) |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания | operator:formalized.tech_description_1.number |
| 05 | TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md | CD | технический текст | ссылка на md-файл |
| 06 | doc_code | 05999 | CD | код вида документа | константа |
| 07 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | константа |
| 08 | doc_number | Б/Н | CO | номер документа | копируемое поле |
| 09 | doc_date | 30.10.2025 | CO | дата документа | копируемое поле |

- _audit: 9
- `doc_status`: confirmed

### `document`: Personal Passport
  - `num`: 1
  - `uqi_prefix`: formalized.passport_1
  - `xml_target_root`: AltaPassport
  - `path`: alta\stable_source\Passport_63_09_449948.xml
  - `file_name`: Passport_63_09_449948.xml
  - `note`: Паспорт представителя Арбузовой А.К.

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 11001 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ПАСПОРТ | CD | наименование документа | константа |
| 03 | DocumentHead_DocumentDate | 11.03.2010 | CD | дата документа | master_data:master_data.md (representative.passport_date) |
| 04 | DocumentHead_DocumentNumber | 63 09 449948 | CD | номер документа | master_data:master_data.md (Representative.passport_series + passport_number) |
| 05 | CardSeries | 63 09 | CD | серия | master_data:master_data.md (representative.passport_series) |
| 06 | CardNumber | 449948 | CD | номер | master_data:master_data.md (representative.passport_number) |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data:master_data.md (representative.passport_org) |
| 08 | CardDate | 11.03.2010 | CD | дата выдачи | master_data:master_data.md (representative.passport_date) |
| 09 | PersonInfo_PersonSurname | АРБУЗОВА | CD | фамилия | master_data:master_data.md (representative.surname) |
| 10 | PersonInfo_PersonName | АНАСТАСИЯ | CD | имя | master_data:master_data.md (representative.name) |
| 11 | PersonInfo_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | master_data:master_data.md (representative.middle_name) |
| 12 | PersonInfo_Sex | 1 | CD | пол | master_data:master_data.md (representative.sex) |
| 13 | PersonInfo_Birthday | 1987-07-25 | CD | дата рождения | master_data:master_data.md (representative.birthday) |
| 14 | PersonInfo_Birthplace | город Саратов | CD | место рождения | master_data:master_data.md (representative.birthplace) |
| 15 | ResidencePlace_PostalCode | 410052 | CD | индекс | master_data:master_data.md (representative.postal_code) |
| 16 | ResidencePlace_CountryCode | RU | CD | страна alpha-2 | master_data:master_data.md (representative.country_code) |
| 17 | ResidencePlace_CounryName | РОССИЯ | CD | страна, текст | master_data:master_data.md (representative.country_name) |
| 18 | ResidencePlace_Region | Саратовская область | CD | регион | master_data:master_data.md (representative.region) |
| 19 | ResidencePlace_City | Саратов | CD | город | master_data:master_data.md (representative.city) |
| 20 | ResidencePlace_StreetHouse | Ул. Одесская д 11 кв 160 | CD | адрес одной строкой | master_data:master_data.md (representative.street_house) |
| 21 | doc_code | 11001 | CD | код вида документа | константа |
| 22 | doc_name | ПАСПОРТ | CD | наименование документа | константа |
| 23 | doc_number | 63 09 449948 | CD | номер документа | копируемое поле |
| 24 | doc_date | 11.03.2010 | CD | дата документа | копируемое поле |

- _audit: 24
- `doc_status`: confirmed

### `document`: Letter of Attorney
  - `num`: 1
  - `uqi_prefix`: formalized.letter_of_attorney_1
  - `xml_target_root`: AltaLetterOfAttorney
  - `path`: alta\stable_source\LetterOfAttorney_1.xml
  - `file_name`: LetterOfAttorney_1.xml
  - `note`: Доверенность представителя Арбузовой А.К.

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 11004 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ДОВЕРЕННОСТЬ | CD | наименование документа | константа |
| 03 | DocumentHead_DocumentDate | 01.02.2026 | CD | дата документа | master_data:master_data.md (letter_of_attorney.date) |
| 04 | DocumentHead_DocumentNumber | 1 | CD | номер документа | master_data:master_data.md (letter_of_attorney.number) |
| 05 | Subject | link:alta\stable_source\LetterOfAttorney_1.xml | CD | текст доверенности | ссылка на эталонный xml-источник в stable_source |
| 06 | EndDate | 31.12.2026 | CD | действительна до | master_data:master_data.md (letter_of_attorney.end_date) |
| 07 | DocumentReference_PrDocumentName | ДОВЕРЕННОСТЬ | CD | наименование доверенности | master_data:master_data.md (letter_of_attorney.empowered_post) |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер доверенности | master_data:master_data.md (letter_of_attorney.number) |
| 09 | DocumentReference_PrDocumentDate | 01.02.2026 | CD | дата доверенности | master_data:master_data.md (letter_of_attorney.date) |
| 10 | Organization_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | выдавшая организация | master_data:master_data.md (declarant.organization_name) |
| 11 | Organization_ShortName | ООО "СКИФ" | CD | краткое наименование | master_data:master_data.md (declarant.short_name) |
| 12 | Organization_OGRN | 1201600020390 | CD | ОГРН | master_data:master_data.md (declarant.ogrn) |
| 13 | Organization_INN | 1650389298 | CD | ИНН | master_data:master_data.md (declarant.inn) |
| 14 | Organization_KPP | 165001001 | CD | КПП | master_data:master_data.md (declarant.kpp) |
| 15 | Organization_Address_PostalCode | 423800 | CD | индекс | master_data:master_data.md (declarant.postal_code) |
| 16 | Organization_Address_CountryCode | RU | CD | страна alpha-2 | master_data:master_data.md (declarant.country_code) |
| 17 | Organization_Address_CounryName | РОССИЯ | CD | страна, текст | master_data:master_data.md (declarant.country_name) |
| 18 | Organization_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data:master_data.md (declarant.region) |
| 19 | Organization_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data:master_data.md (declarant.city) |
| 20 | Organization_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CD | улица/дом одной строкой | master_data:master_data.md (declarant.street_house) |
| 21 | Organization_OrganizationPerson_PersonSurname | САРАНОВ | CD | подписант: фамилия | master_data:master_data.md (declarant.director_surname) |
| 22 | Organization_OrganizationPerson_PersonName | ДМИТРИЙ | CD | подписант: имя | master_data:master_data.md (declarant.director_name) |
| 23 | Organization_OrganizationPerson_PersonMiddleName | ОЛЕГОВИЧ | CD | подписант: отчество | master_data:master_data.md (declarant.director_middle_name) |
| 24 | Organization_OrganizationPerson_PersonPost | ГЕНЕРАЛЬНЫЙ ДИРЕКТОР | CD | должность подписанта | константа (роль руководителя) |
| 25 | EmpoweredPerson_PersonSurname | АРБУЗОВА | CD | уполномоченное лицо: фамилия | master_data:master_data.md (representative.surname) |
| 26 | EmpoweredPerson_PersonName | АНАСТАСИЯ | CD | уполномоченное лицо: имя | master_data:master_data.md (representative.name) |
| 27 | EmpoweredPerson_PersonMiddleName | КОНСТАНТИНОВНА | CD | уполномоченное лицо: отчество | master_data:master_data.md (representative.middle_name) |
| 28 | EmpoweredPerson_PersonPost | ПРЕДСТАВИТЕЛЬ | CD | роль/должность | master_data:master_data.md (letter_of_attorney.empowered_post) |
| 29 | EmpoweredPerson_Passport_IdentityCardCode | RU01001 | CD | код документа | master_data:master_data.md (letter_of_attorney.empowered_passport_code) |
| 30 | EmpoweredPerson_Passport_IdentityCardName | ПАСПОРТ РФ | CD | наименование документа | master_data:master_data.md (letter_of_attorney.empowered_passport_name) |
| 31 | EmpoweredPerson_Passport_IdentityCardSeries | 63 09 | CD | серия | master_data:master_data.md (representative.passport_series) |
| 32 | EmpoweredPerson_Passport_IdentityCardNumber | 449948 | CD | номер | master_data:master_data.md (representative.passport_number) |
| 33 | EmpoweredPerson_Passport_IdentityCardDate | 11.03.2010 | CD | дата выдачи | master_data:master_data.md (representative.passport_date) |
| 34 | EmpoweredPerson_Passport_OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data:master_data.md (representative.passport_org) |
| 35 | doc_code | 11004 | CD | код вида документа | константа |
| 36 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | константа |
| 37 | doc_number | 1 | CD | номер документа | копируемое поле |
| 38 | doc_date | 01.02.2026 | CD | дата документа | копируемое поле |

- _audit: 38
- `doc_status`: confirmed

### `document`: Transport Contract
  - `num`: 1
  - `uqi_prefix`: formalized.transport_contract_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\stable_source\FreeDoc_КООО_26651_М.xml
  - `file_name`: FreeDoc_КООО_26651_М.xml
  - `note`: Договор транспортной экспедиции КООО/26651/М

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04033 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ДОГОВОР ТРАНСПОРТНОЙ ЭКСПЕДИЦИИ | CD | наименование договора | константа |
| 03 | DocumentHead_DocumentDate | 13.05.2025 | CD | дата договора | master_data:master_data.md (transport_contract.date) |
| 04 | DocumentHead_DocumentNumber | КООО/26651/М | CD | номер договора | master_data:master_data.md (transport_contract.number) |
| 05 | TextPara | link:alta\stable_source\FreeDoc_КООО_26651_М.xml | CD | текст договора | ссылка на эталонный xml в stable_source |
| 06 | doc_code | 04033 | CD | код вида документа | константа |
| 07 | doc_name | ДОГОВОР ТРАНСПОРТНОЙ ЭКСПЕДИЦИИ | CD | наименование документа | константа |
| 08 | doc_number | КООО/26651/М | CD | номер документа | копируемое поле |
| 09 | doc_date | 13.05.2025 | CD | дата документа | копируемое поле |

- _audit: 9
- `doc_status`: confirmed

### `document`: EGRUL
  - `num`: 1
  - `uqi_prefix`: formalized.egrul_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml
  - `file_name`: FreeDoc_ЮЭ9965-25-106893283.xml
  - `note`: Сведения из ЕГРЮЛ ООО "СКИФ"

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04011 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование выписки | константа |
| 03 | DocumentHead_DocumentDate | 12.03.2020 | CD | дата выписки | master_data:master_data.md (declarant.registration_date) |
| 04 | DocumentHead_DocumentNumber | ЮЭ9965-25-106893283 | CD | номер выписки | по имени файла-эталона в stable_source |
| 05 | TextPara | link:alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml | CD | текст выписки | ссылка на эталонный xml в stable_source |
| 06 | doc_code | 04011 | CD | код вида документа | константа |
| 07 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | константа |
| 08 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | копируемое поле |
| 09 | doc_date | 12.03.2020 | CD | дата документа | копируемое поле |

- _audit: 9
- `doc_status`: confirmed

## 3. non_formalized

### `document`: Storage Report
  - `num`: 1
  - `uqi_prefix`: non_formalized.svh_1
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО 14431420260204161621.md
  - `file_name`: ДО 14431420260204161621.md
  - `note`: Отчет СВХ ДО-1 №0000080

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии СВХ | copied_from:ДО 14431420260204161621.md |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии СВХ | copied_from:ДО 14431420260204161621.md |
| 03 | actual_gross_weight | 3500.00 | CD | фактический вес брутто | copied_from:ДО доп 14431520260204161645.md (Totals); non_formalized.svh_1.actual_totals_from_svh_additional_sheet |
| 04 | actual_places | 127 | CD | фактическое количество мест | copied_from:ДО доп 14431520260204161645.md (Totals); non_formalized.svh_1.actual_totals_from_svh_additional_sheet |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС при въезде | copied_from:ДО 14431420260204161621.md |

- _audit: 5

#### Массив: goods[2]

#### goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 7019900095 | CD | код товара (ТН ВЭД) | copied_from:ДО 14431420260204161621.md |
| 02 | places | 27 | CD | кол-во грузовых мест | copied_from:ДО 14431420260204161621.md |
| 03 | gross_weight_kg | 1710 | CD | вес брутто по строке | copied_from:ДО 14431420260204161621.md |
| 04 | cost | 42228 | CD | стоимость по строке | copied_from:ДО 14431420260204161621.md |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from:ДО 14431420260204161621.md |

- _item_audit: 5

#### goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара (ТН ВЭД) | copied_from:ДО 14431420260204161621.md |
| 02 | places | 100 | CD | кол-во грузовых мест | copied_from:ДО 14431420260204161621.md |
| 03 | gross_weight_kg | 1790 | CD | вес брутто по строке | copied_from:ДО 14431420260204161621.md |
| 04 | cost | 55032 | CD | стоимость по строке | copied_from:ДО 14431420260204161621.md |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from:ДО 14431420260204161621.md |

- _item_audit: 5
- _array_audit: 2
- `array_status`: confirmed

### `document`: Storage Report Additional Sheet
  - `num`: 1
  - `uqi_prefix`: non_formalized.svh_additional_sheet_1
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО доп 14431520260204161645.md
  - `file_name`: ДО доп 14431520260204161645.md
  - `note`: Добавочный лист СВХ ДО-1 №1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 1 | CD | № доп.листа/приложения | copied_from:ДО доп 14431520260204161645.md |
| 02 | date | 03.02.2026 | CO | дата доп.листа | operator:non_formalized.svh_1.date (по дате основного отчета) |
| 03 | actual_gross_weight | 3500.00 | CD | фактический вес брутто | copied_from:ДО доп 14431520260204161645.md (Totals) |
| 04 | actual_places | 127 | CD | фактическое количество мест | copied_from:ДО доп 14431520260204161645.md (Totals) |
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CO | номер ТС при въезде | copied_from:non_formalized.svh_1.transport_reg_number (из основного отчета ДО-1) |
| 06 | svh_address_region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион СВХ | copied_from:formalized.cmr_1.Consignee_Address_Region; non_formalized.svh_additional_sheet_1.address_from_cmr |
| 07 | svh_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город/нас.пункт СВХ | copied_from:formalized.cmr_1.Consignee_Address_City; non_formalized.svh_additional_sheet_1.address_from_cmr |
| 08 | svh_address_street_house | ПРОИЗВОДСТВЕННЫЙ ПР-Д, Д. 45 | CO | улица/дом СВХ одной строкой | copied_from:formalized.cmr_1.Consignee_Address_StreetHouse; non_formalized.svh_additional_sheet_1.address_from_cmr (из п.3 CMR место доставки СВХ ООО «ЛОГИКАМ») |
| 09 | svh_customs_code | 10404083 | CO | код таможенного органа в зоне СВХ | copied_from:formalized.cmr_1.Consignee_Address_StreetHouse; non_formalized.svh_additional_sheet_1.address_from_cmr (из п.13 CMR Набережночелнинский т/п) |

- _audit: 9
- `doc_status`: confirmed

### `document`: Transit Declaration
  - `num`: 1
  - `uqi_prefix`: non_formalized.transit_declaration_1
  - `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ТД 10719110_240126_5011363_reg00378тд.md
  - `file_name`: ТД 10719110_240126_5011363_reg00378тд.md
  - `note`: Транзитная декларация № 10719110/240126/5011363

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 10719110/240126/5011363 | CO | номер ТД | operator:non_formalized.transit_declaration_1.number |
| 02 | date | 24.01.2026 | CO | дата ТД | operator:non_formalized.transit_declaration_1.date |
| 03 | customs_post_code | 10719110 | CD | код таможенного органа | copied_from:ТД 10719110_240126_5011363_reg00378тд.md |
| 04 | customs_post_name | ОТО И ТК №3 Таможенный пост Набережночелнинский | CD | наименование таможенного органа | copied_from:ТД 10719110_240126_5011363_reg00378тд.md |
| 05 | transport_reg_number | O157AO774/BT374974 | CD | ТС по ТД | copied_from:ТД 10719110_240126_5011363_reg00378тд.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Master Data
  - `num`: 1
  - `uqi_prefix`: non_formalized.stable_data_1
  - `path`: alta\master_data\master_data.md
  - `file_name`: master_data.md
  - `note`: Реквизиты и контакты ООО "СКИФ" и представителя

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | master_data.declarant.organization_name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование декларанта | master_data:master_data.md (declarant.organization_name) |
| 02 | master_data.declarant.phone | +7 (843) 207 18 90 | CD | телефон декларанта | master_data:master_data.md (declarant.phone) |
| 03 | master_data.declarant.email | PROM_TAT@MAIL.RU | CD | email декларанта | master_data:master_data.md (declarant.email) |
| 04 | master_data.representative.name | АРБУЗОВА АНАСТАСИЯ КОНСТАНТИНОВНА | CD | ФИО представителя | master_data:master_data.md (representative.surname + name + middle_name) |
| 05 | master_data.representative.phone | +7 927-222-0500 | CD | телефон представителя | master_data:master_data.md (representative.phone) |
| 06 | master_data.representative.email | A.K.ARBUZOVA@YANDEX.RU | CD | email представителя | master_data:master_data.md (representative.email) |

- _audit: 6
- `doc_status`: confirmed

### Итого, по файлу:

`total_unreliable_fields`: 0
`formalization_status`: confirmed

## 5. Нерешенные вопросы (Issues)

**Для полей:**
- `formalized.contract_1.ContractTerms_OtherTerms`
  - `question`: Каковы условия поставки / Incoterms по контракту? Изначально в контракте они отсутствовали, но по решению оператора (в operator_provided_data.md) зафиксировано значение "EXW HEBEI". Конфликт разрешен.
- `formalized.invoice_1.PlacesQuantity`
  - `question`: В инвойсе указано "127 BG (pcs)" (пакеты/места). В ДО-1/ДО-2 и ТД зафиксировано "127 мест". Для ДТ используется подтвержденное оператором значение "127 мест" (упаковка "Поддон", 127 упаковок/пакетов). Конфликт разрешен.
- `formalized.service_invoice_1.consignor_postalcode_empty_ok`
  - `question`: Индекс грузоотправителя в счете за перевозку отсутствует. Оператор разрешил оставить поле пустым. Конфликт разрешен.

**Для общих вопросов:**
- `[Общий]`
  - `question`: Все возникшие в ходе конвертации и анализа документов вопросы были успешно разрешены на основе данных из `operator_provided_data.md` и перенесены со статусом CO (confirmed_operator). База фактов полностью подтверждена.

## 6. `unreliable_fields`:

