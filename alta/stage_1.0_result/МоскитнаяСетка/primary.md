# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 2 товара
- `источники данных`: md + operator_provided_data + master_data + stable_source

## 2. formalized

### `document`: Contract
- `uqi_prefix`: formalized.contract_1
- `xml_target_root`: AltaE2CONT
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md
- `file_name`: SALES CONTRACT NoLM-2553.md
- `note`: Контракт LM-2553 от 02.07.2025, продавец HEBEI LANGMAI, покупатель ООО "СКИФ"

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentCode | 03011 | CD | код вида документа | константа; derived |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md) |
| 03 | ContractRegistration_PrDocumentDate | 02.07.2025 | CD | дата контракта | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md) |
| 04 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта | operator: formalized.contract_1.ContractTerms_Amount (вариант B) |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator: formalized.contract_1.currency_code_numeric |
| 06 | ContractTerms_LastDate | 31.12.2026 | CD | срок действия/исполнения | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md) |
| 07 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки / Incoterms | operator: formalized.contract_1.delivery_terms |
| 08 | ContractTerms_ContractText | link:md/SALES CONTRACT NoLM-2553.md | CD | текст контракта | link на md-версию |
| 09 | ContractTerms_DealSign | 1 | CO | системный признак Альты | operator: formalized.contract_1.deal_sign; derived |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md) |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 | operator: formalized.contract_1.foreign_person_country_code_alpha2 |
| 12 | ForeignPerson_Address_CounryName | КИТАЙ | CD | страна продавца, текст | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md, рус. версия) |
| 13 | ForeignPerson_Address_Region | Hebei | CD | регион/область продавца | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md) |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md) |
| 15 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом продавца одной строкой | operator: formalized.contract_1.foreign_person_address_line |
| 16 | RussianPerson_OrganizationName | ООО "СКИФ" | CD | покупатель/сторона контракта | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md, рус. версия) |
| 17 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН покупателя | master_data: master_data.md (declarant_1.ogrn); разрешено оператором |
| 18 | RussianPerson_INN | 1650389298 | CO | ИНН покупателя | master_data: master_data.md (declarant_1.inn); разрешено оператором |
| 19 | RussianPerson_KPP | 165001001 | CO | КПП покупателя | master_data: master_data.md (declarant_1.kpp); разрешено оператором |
| 20 | RussianPerson_Address_PostalCode | 423800 | CD | индекс покупателя | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md) |
| 21 | RussianPerson_Address_CountryCode | RU | CD | страна покупателя alpha-2 | copied_from: formalized.contract_1 (SALES CONTRACT NoLM-2553.md) |
| 22 | RussianPerson_Address_CounryName | РОССИЯ | CD | страна покупателя, текст | master_data: master_data.md (declarant_1.country_name) |
| 23 | RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион покупателя | master_data: master_data.md (declarant_1.region) |
| 24 | RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город покупателя | master_data: master_data.md (declarant_1.city) |
| 25 | RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | operator: formalized.contract_1.russian_person_address_line |
| 26 | doc_code | 03011 | CD | код документа | константа; derived |
| 27 | doc_name | КОНТРАКТ | CD | наименование документа | константа; derived |
| 28 | doc_number | LM-2553 | CD | номер документа | = ContractRegistration_PrDocumentNumber |
| 29 | doc_date | 02.07.2025 | CD | дата документа | = ContractRegistration_PrDocumentDate |

- _audit: 29
- `doc_status`: confirmed

### `document`: Supplementary Contract
- `uqi_prefix`: formalized.supplementary_contract_1
- `xml_target_root`: AltaSupplementaryContract
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md
- `file_name`: 1 Supplementary agreement to the contract.md
- `note`: Доп. соглашение №1 от 25.11.2025 к контракту LM-2553

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения | copied_from: formalized.supplementary_contract_1 (1 Supplementary agreement to the contract.md) |
| 02 | IssueDate | 25.11.2025 | CD | дата доп. соглашения | copied_from: formalized.supplementary_contract_1 (1 Supplementary agreement to the contract.md) |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта | copied_from: formalized.supplementary_contract_1 (1 Supplementary agreement to the contract.md) |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator: formalized.supplementary_contract_1.currency_code_numeric |
| 05 | ContractDescription_LastDate | 31.12.2026 | CO | новый срок действия/исполнения | operator: formalized.supplementary_contract_1.expiry_date |
| 06 | ContractDescription_ContractText | link:md/1 Supplementary agreement to the contract.md | CD | текст доп. соглашения | link на md-версию |
| 07 | ContractDescription_DealSign | 1 | CO | системный признак Альты | operator: formalized.supplementary_contract_1.deal_sign; derived |
| 08 | ContractDescription_StockCategorySign | 0 | CO | системный признак Альты | operator: formalized.supplementary_contract_1.stock_category_sign; derived |
| 09 | ContractDescription_BuyerLimitationSign | 0 | CO | системный признак Альты | operator: formalized.supplementary_contract_1.buyer_limitation_sign; derived |
| 10 | ContractDescription_InsuranceSign | 0 | CO | системный признак Альты | operator: formalized.supplementary_contract_1.insurance_sign; derived |
| 11 | RussianPerson_OrganizationName | ООО "СКИФ" | CD | российская сторона; покупатель | copied_from: formalized.supplementary_contract_1 (рус. версия) |
| 12 | RussianPerson_ShortName | ООО "СКИФ" | CO | краткое наименование | operator: formalized.supplementary_contract_1.russian_person_short_name |
| 13 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН | master_data: master_data.md (declarant_1.ogrn); разрешено оператором |
| 14 | RussianPerson_INN | 1650389298 | CO | ИНН | master_data: master_data.md (declarant_1.inn); разрешено оператором |
| 15 | RussianPerson_KPP | 165001001 | CO | КПП | master_data: master_data.md (declarant_1.kpp); разрешено оператором |
| 16 | RussianPerson_Address_PostalCode | 423800 | CO | индекс | copied_from: formalized.contract_1.RussianPerson_Address_PostalCode (cross-doc) |
| 17 | RussianPerson_Address_CountryCode | RU | CO | страна alpha-2 | copied_from: formalized.contract_1.RussianPerson_Address_CountryCode (cross-doc) |
| 18 | RussianPerson_Address_CounryName | РОССИЯ | CO | страна, текст | copied_from: formalized.contract_1.RussianPerson_Address_CounryName (cross-doc) |
| 19 | RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион | copied_from: formalized.contract_1.RussianPerson_Address_Region (cross-doc) |
| 20 | RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город | copied_from: formalized.contract_1.RussianPerson_Address_City (cross-doc) |
| 21 | RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом одной строкой | copied_from: formalized.contract_1.RussianPerson_Address_StreetHouse (cross-doc) |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | иностранная сторона; продавец | copied_from: formalized.supplementary_contract_1 (1 Supplementary agreement to the contract.md) |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator: formalized.supplementary_contract_1.foreign_person_short_name_equals_full |
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 | operator: formalized.supplementary_contract_1.foreign_person_country_code_alpha2 |
| 25 | ForeignPerson_Address_CounryName | КИТАЙ | CO | страна, текст | copied_from: formalized.contract_1.ForeignPerson_Address_CounryName (cross-doc) |
| 26 | ForeignPerson_Address_Region | Hebei | CO | регион | copied_from: formalized.contract_1.ForeignPerson_Address_Region (cross-doc) |
| 27 | ForeignPerson_Address_City | Shijiazhuang | CO | город/район | copied_from: formalized.contract_1.ForeignPerson_Address_City (cross-doc) |
| 28 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, Hebei, China | CO | улица/дом одной строкой | copied_from: formalized.contract_1.ForeignPerson_Address_StreetHouse (cross-doc) |
| 29 | PersonSurname | Li | CO | фамилия подписанта | operator: formalized.supplementary_contract_1.signed_person_surname |
| 30 | PersonName | Jing | CO | имя подписанта | operator: formalized.supplementary_contract_1.signed_person_name |
| 31 | PersonMiddleName | | CO | отчество подписанта | operator: formalized.supplementary_contract_1.signed_person_middle_name |
| 32 | doc_code | 03012 | CD | код документа | константа; derived |
| 33 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наименование документа | константа; derived |
| 34 | doc_number | 1 | CD | номер документа | = DocumentNumber |
| 35 | doc_date | 25.11.2025 | CD | дата документа | = IssueDate |

- _audit: 35
- `doc_status`: confirmed

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\CL на сетку .md
- `file_name`: CL на сетку .md
- `note`: Инвойс LM-2591 от 30.10.2025

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты | operator: formalized.invoice_1.exchange_rate |
| 02 | CurrencyCode | CNY | CO | валюта инвойса ISO 4217 alpha-3 | operator: formalized.invoice_1.currency_code |
| 03 | DocumentCode | 04021 | CD | код вида документа | константа; derived |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест по инвойсу | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator: formalized.invoice_1.places_description |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто по инвойсу | operator: formalized.invoice_1.total_gross_weight (из PL totals) |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто по инвойсу | operator: formalized.invoice_1.total_net_weight (из PL totals) |
| 08 | GCost | 97260.00 | CO | системное поле Альты | operator: formalized.invoice_1.gcost (=TotalCost) |
| 09 | TotalCost | 97260.00 | CD | итого по инвойсу | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator: formalized.invoice_1.delivery_terms_numeric |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator: formalized.invoice_1.delivery_terms_string |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | operator: formalized.invoice_1.dispatch_country_code |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | operator: formalized.invoice_1.trading_country_code |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | operator: formalized.invoice_1.destination_country_code |
| 16 | Registration_PrDocumentName | Commercial invoice / Комерческий инвойс | CD | наименование документа | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 21 | Buyer_CompanyID | 1650389298 | CO | ИНН покупателя | master_data: master_data.md (declarant_1.inn); разрешено оператором |
| 22 | Buyer_KPPCode | 165001001 | CO | КПП покупателя | master_data: master_data.md (declarant_1.kpp); разрешено оператором |
| 23 | Buyer_Name | ООО "СКИФ" | CD | наименование покупателя | copied_from: formalized.invoice_1 (CL на сетку .md); нормализация рус. |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 25 | Buyer_PostalAddress_CountryCode | RU | CO | страна покупателя alpha-2 | master_data: master_data.md (declarant_1.country_code) |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CO | страна покупателя, текст | master_data: master_data.md (declarant_1.country_name) |
| 27 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион | master_data: master_data.md (declarant_1.region) |
| 28 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город | master_data: master_data.md (declarant_1.city) |
| 29 | Buyer_PostalAddress_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | copied_from: formalized.contract_1.RussianPerson_Address_StreetHouse (cross-doc) |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | продавец | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | operator: formalized.invoice_1.seller_country_code_alpha2 |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | copied_from: formalized.contract_1.ForeignPerson_Address_CounryName (cross-doc, рус.) |
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город/район продавца | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from: formalized.invoice_1 (CL на сетку .md) |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | нормализация: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | нормализация: consignor=seller |
| 38 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | нормализация: consignor=seller |
| 39 | Consignor_Address_Region | Hebei | CD | регион | нормализация: consignor=seller |
| 40 | Consignor_Address_City | Shijiazhuang | CD | город/район | нормализация: consignor=seller |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | нормализация: consignor=seller |
| 42 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | нормализация: consignee=buyer |
| 43 | Consignee_OGRN | 1201600020390 | CO | ОГРН | master_data: master_data.md (declarant_1.ogrn); разрешено оператором |
| 44 | Consignee_INN | 1650389298 | CO | ИНН | master_data: master_data.md (declarant_1.inn); разрешено оператором |
| 45 | Consignee_KPP | 165001001 | CO | КПП | master_data: master_data.md (declarant_1.kpp); разрешено оператором |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | нормализация: consignee=buyer |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | нормализация: consignee=buyer |
| 48 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | нормализация: consignee=buyer |
| 49 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | нормализация: consignee=buyer |
| 50 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | нормализация: consignee=buyer |
| 51 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CD | улица/дом/офис одной строкой | нормализация: consignee=buyer |
| 52 | doc_code | 04021 | CD | код документа | константа; derived |
| 53 | doc_name | ИНВОЙС | CD | наименование документа | константа; derived |
| 54 | doc_number | LM-2591 | CD | номер документа | = Registration_PrDocumentNumber |
| 55 | doc_date | 30.10.2025 | CD | дата документа | = Registration_PrDocumentDate |

- _audit: 55
- `doc_status`: confirmed

#### 3.1 InvoiceGoods[7] Массив: InvoiceGoods[7]
- _array_audit: 7

#### 3.1 InvoiceGoods[7] Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from: formalized.invoice_1.InvoiceGoods_1 (CL на сетку .md) |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара | copied_from: formalized.invoice_1.InvoiceGoods_1 (CL на сетку .md) |
| 03 | GoodsQuantity | 60 | CD | кол-во в основной единице | copied_from: formalized.invoice_1.InvoiceGoods_1 (CL на сетку .md) |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_1 (CL на сетку .md) |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_1 (CL на сетку .md) |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | derived from cb:unit 055 (M2) |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_1.gross_weight (from PL) |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator: formalized.invoice_1.goods_1.net_weight (from PL); derived |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from: formalized.invoice_1.InvoiceGoods_1 (CL на сетку .md) |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | copied_from: formalized.invoice_1.InvoiceGoods_1 (CL на сетку .md) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator: formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### 3.1 InvoiceGoods[7] Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from: formalized.invoice_1.InvoiceGoods_2 (CL на сетку .md) |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара | copied_from: formalized.invoice_1.InvoiceGoods_2 (CL на сетку .md) |
| 03 | GoodsQuantity | 30 | CD | кол-во в основной единице | copied_from: formalized.invoice_1.InvoiceGoods_2 (CL на сетку .md) |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_2 (CL на сетку .md) |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_2 (CL на сетку .md) |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | derived from cb:unit 055 (M2) |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_2.gross_weight (from PL) |
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator: formalized.invoice_1.goods_2.net_weight (from PL); derived |
| 09 | Price | 5.85 | CD | цена за единицу | copied_from: formalized.invoice_1.InvoiceGoods_2 (CL на сетку .md) |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | copied_from: formalized.invoice_1.InvoiceGoods_2 (CL на сетку .md) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator: formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### 3.1 InvoiceGoods[7] Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from: formalized.invoice_1.InvoiceGoods_3 (CL на сетку .md) |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2/Материал: полиэстер | CD | описание товара | copied_from: formalized.invoice_1.InvoiceGoods_3 (CL на сетку .md) |
| 03 | GoodsQuantity | 60 | CD | кол-во в основной единице | copied_from: formalized.invoice_1.InvoiceGoods_3 (CL на сетку .md) |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_3 (CL на сетку .md) |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_3 (CL на сетку .md) |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | derived from cb:unit 055 (M2) |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_3.gross_weight (from PL) |
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator: formalized.invoice_1.goods_3.net_weight (from PL); derived |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from: formalized.invoice_1.InvoiceGoods_3 (CL на сетку .md) |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | copied_from: formalized.invoice_1.InvoiceGoods_3 (CL на сетку .md) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator: formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### 3.1 InvoiceGoods[7] Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from: formalized.invoice_1.InvoiceGoods_4 (CL на сетку .md) |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2/Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CD | описание товара | copied_from: formalized.invoice_1.InvoiceGoods_4 (CL на сетку .md) |
| 03 | GoodsQuantity | 30 | CD | кол-во в основной единице | copied_from: formalized.invoice_1.InvoiceGoods_4 (CL на сетку .md) |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_4 (CL на сетку .md) |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_4 (CL на сетку .md) |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | derived from cb:unit 055 (M2) |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_4.gross_weight (from PL) |
| 08 | NetWeightQuantity | 144.00 | CO | нетто по строке | operator: formalized.invoice_1.goods_4.net_weight (from PL); derived |
| 09 | Price | 6.35 | CD | цена за единицу | copied_from: formalized.invoice_1.InvoiceGoods_4 (CL на сетку .md) |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | copied_from: formalized.invoice_1.InvoiceGoods_4 (CL на сетку .md) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator: formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### 3.1 InvoiceGoods[7] Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from: formalized.invoice_1.InvoiceGoods_5 (CL на сетку .md) |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара | copied_from: formalized.invoice_1.InvoiceGoods_5 (CL на сетку .md) |
| 03 | GoodsQuantity | 90 | CD | кол-во в основной единице | copied_from: formalized.invoice_1.InvoiceGoods_5 (CL на сетку .md) |
| 04 | goods_supplementary_quantity | 3780 | CD | количество в доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_5 (CL на сетку .md) |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_5 (CL на сетку .md) |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | derived from cb:unit 055 (M2) |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_5.gross_weight (from PL) |
| 08 | NetWeightQuantity | 491.40 | CO | нетто по строке | operator: formalized.invoice_1.goods_5.net_weight (from PL); derived |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from: formalized.invoice_1.InvoiceGoods_5 (CL на сетку .md) |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | copied_from: formalized.invoice_1.InvoiceGoods_5 (CL на сетку .md) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator: formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### 3.1 InvoiceGoods[7] Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | copied_from: formalized.invoice_1.InvoiceGoods_6 (CL на сетку .md) |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара | copied_from: formalized.invoice_1.InvoiceGoods_6 (CL на сетку .md) |
| 03 | GoodsQuantity | 180 | CD | кол-во в основной единице | copied_from: formalized.invoice_1.InvoiceGoods_6 (CL на сетку .md) |
| 04 | goods_supplementary_quantity | 8640 | CD | количество в доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_6 (CL на сетку .md) |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_6 (CL на сетку .md) |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | derived from cb:unit 055 (M2) |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_6.gross_weight (from PL) |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто по строке | operator: formalized.invoice_1.goods_6.net_weight (from PL); derived |
| 09 | Price | 3.4 | CD | цена за единицу | copied_from: formalized.invoice_1.InvoiceGoods_6 (CL на сетку .md) |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | copied_from: formalized.invoice_1.InvoiceGoods_6 (CL на сетку .md) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator: formalized.invoice_1.goods_all.model |

- _item_audit: 15

#### 3.1 InvoiceGoods[7] Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | copied_from: formalized.invoice_1.InvoiceGoods_7 (CL на сетку .md) |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | copied_from: formalized.invoice_1.InvoiceGoods_7 (CL на сетку .md) |
| 03 | GoodsQuantity | 5 | CD | кол-во в основной единице | copied_from: formalized.invoice_1.InvoiceGoods_7 (CL на сетку .md) |
| 04 | goods_supplementary_quantity | 240 | CD | количество в доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_7 (CL на сетку .md) |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | copied_from: formalized.invoice_1.InvoiceGoods_7 (CL на сетку .md) |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества | derived from cb:unit 055 (M2) |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_7.gross_weight (from PL) |
| 08 | NetWeightQuantity | 24.00 | CO | нетто по строке | operator: formalized.invoice_1.goods_7.net_weight (from PL); derived |
| 09 | Price | 28 | CD | цена за единицу | copied_from: formalized.invoice_1.InvoiceGoods_7 (CL на сетку .md) |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | copied_from: formalized.invoice_1.InvoiceGoods_7 (CL на сетку .md) |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator: formalized.invoice_1.goods_all.model |

- _item_audit: 15
- `doc_status`: confirmed

### `document`: Packing List
- `uqi_prefix`: formalized.packing_list_1
- `xml_target_root`: AltaE2PACK
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\PL на сетку .md
- `file_name`: PL на сетку .md
- `note`: Упаковочный лист LM-2591 от 30.10.2025

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто по упаковочному | copied_from: formalized.packing_list_1 (PL на сетку .md); derived |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто по упаковочному | copied_from: formalized.packing_list_1 (PL на сетку .md); derived |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | copied_from: formalized.invoice_1.Seler_Name (cross-doc; consignor=seller) |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | краткое наименование | operator: formalized.packing_list_1.consignor_shortname_equals_full |
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | operator: formalized.packing_list_1.consignor_country_code_alpha2 |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | copied_from: formalized.invoice_1.Seler_PostalAddress_CounryName (cross-doc; consignor=seller) |
| 07 | Consignor_Address_Region | Hebei | CD | регион | copied_from: formalized.invoice_1.Seler_PostalAddress_Region (cross-doc; consignor=seller) |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from: formalized.invoice_1.Seler_PostalAddress_City (cross-doc; consignor=seller) |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from: formalized.invoice_1.Seler_PostalAddress_StreetHouse (cross-doc; consignor=seller) |
| 10 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | copied_from: formalized.packing_list_1 (PL на сетку .md); нормализация рус. |
| 11 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator: formalized.packing_list_1.consignee_shortname_equals_full |
| 12 | Consignee_OGRN | 1201600020390 | CO | ОГРН | master_data: master_data.md (declarant_1.ogrn) |
| 13 | Consignee_INN | 1650389298 | CO | ИНН | master_data: master_data.md (declarant_1.inn) |
| 14 | Consignee_KPP | 165001001 | CO | КПП | master_data: master_data.md (declarant_1.kpp) |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | copied_from: formalized.packing_list_1 (PL на сетку .md) |
| 16 | Consignee_Address_CountryCode | RU | CO | страна alpha-2 | master_data: master_data.md (declarant_1.country_code) |
| 17 | Consignee_Address_CounryName | РОССИЯ | CO | страна, текст | master_data: master_data.md (declarant_1.country_name) |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CO | регион | master_data: master_data.md (declarant_1.region) |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CO | город | master_data: master_data.md (declarant_1.city) |
| 20 | Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | CO | улица/дом/офис одной строкой | copied_from: formalized.contract_1.RussianPerson_Address_StreetHouse (cross-doc) |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | copied_from: formalized.packing_list_1 (PL на сетку .md) |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | внутренний числовой код условий | operator: formalized.invoice_1.delivery_terms_numeric (cross-doc) |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | copied_from: formalized.packing_list_1 (PL на сетку .md) |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | наименование контракта | operator: formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentName |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | copied_from: formalized.packing_list_1 (PL на сетку .md) |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | copied_from: formalized.packing_list_1 (PL на сетку .md) |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | константа; derived |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | copied_from: formalized.packing_list_1 (PL на сетку .md) |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | copied_from: formalized.packing_list_1 (PL на сетку .md) |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CO | наименование упаковочного | operator: formalized.packing_list_1.registration_doc_name |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | operator: formalized.packing_list_1.registration_doc_number |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | operator: formalized.packing_list_1.registration_doc_date |
| 33 | doc_code | 04131 | CD | код документа | константа; derived |
| 34 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | константа; derived |
| 35 | doc_number | LM-2591 | CD | номер документа | = DeliveryTerms_Registration_PrDocumentNumber |
| 36 | doc_date | 30.10.2025 | CD | дата документа | = DeliveryTerms_Registration_PrDocumentDate |

- _audit: 36

#### Packing List Массив: Goods[7]
- _array_audit: 7

#### Packing List Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки | copied_from: formalized.packing_list_1.Goods_1 (PL на сетку .md) |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | copied_from: formalized.packing_list_1.Goods_1 (PL на сетку .md) |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | copied_from: formalized.packing_list_1.Goods_1 (PL на сетку .md) |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | copied_from: formalized.packing_list_1.Goods_1 (PL на сетку .md) |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок/мест | operator: formalized.packing_list_1.goods_1.paking_quantity |

- _item_audit: 5

#### Packing List Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки | copied_from: formalized.packing_list_1.Goods_2 (PL на сетку .md) |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц | copied_from: formalized.packing_list_1.Goods_2 (PL на сетку .md) |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто по строке | copied_from: formalized.packing_list_1.Goods_2 (PL на сетку .md) |
| 04 | NetWeightQuantity | 460.80 | CD | нетто по строке | copied_from: formalized.packing_list_1.Goods_2 (PL на сетку .md) |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок/мест | operator: formalized.packing_list_1.goods_2.paking_quantity |

- _item_audit: 5

#### Packing List Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера . Размер рулона 1,42*0,64*0,22 | CD | описание строки | copied_from: formalized.packing_list_1.Goods_3 (PL на сетку .md) |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | copied_from: formalized.packing_list_1.Goods_3 (PL на сетку .md) |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто по строке | copied_from: formalized.packing_list_1.Goods_3 (PL на сетку .md) |
| 04 | NetWeightQuantity | 252.00 | CD | нетто по строке | copied_from: formalized.packing_list_1.Goods_3 (PL на сетку .md) |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок/мест | operator: formalized.packing_list_1.goods_3.paking_quantity |

- _item_audit: 5

#### Packing List Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание строки | copied_from: formalized.packing_list_1.Goods_4 (PL на сетку .md) |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц | copied_from: formalized.packing_list_1.Goods_4 (PL на сетку .md) |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто по строке | copied_from: formalized.packing_list_1.Goods_4 (PL на сетку .md) |
| 04 | NetWeightQuantity | 144.00 | CD | нетто по строке | copied_from: formalized.packing_list_1.Goods_4 (PL на сетку .md) |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок/мест | operator: formalized.packing_list_1.goods_4.paking_quantity |

- _item_audit: 5

#### Packing List Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА " Антимошка " из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание строки | copied_from: formalized.packing_list_1.Goods_5 (PL на сетку .md) |
| 02 | GoodsQuantity | 90 | CD | количество мест/грузовых единиц | copied_from: formalized.packing_list_1.Goods_5 (PL на сетку .md) |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто по строке | copied_from: formalized.packing_list_1.Goods_5 (PL на сетку .md) |
| 04 | NetWeightQuantity | 491.40 | CD | нетто по строке | copied_from: formalized.packing_list_1.Goods_5 (PL на сетку .md) |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок/мест | operator: formalized.packing_list_1.goods_5.paking_quantity |

- _item_audit: 5

#### Packing List Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание строки | copied_from: formalized.packing_list_1.Goods_6 (PL на сетку .md) |
| 02 | GoodsQuantity | 180 | CD | количество мест/грузовых единиц | copied_from: formalized.packing_list_1.Goods_6 (PL на сетку .md) |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | copied_from: formalized.packing_list_1.Goods_6 (PL на сетку .md) |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто по строке | copied_from: formalized.packing_list_1.Goods_6 (PL на сетку .md) |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок/мест | operator: formalized.packing_list_1.goods_6.paking_quantity |

- _item_audit: 5

#### Packing List Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки "Антипыльца " из полиэстера Размер рулона 1,72*0,35*0,31*1 | CD | описание строки | copied_from: formalized.packing_list_1.Goods_7 (PL на сетку .md) |
| 02 | GoodsQuantity | 5 | CD | количество мест/грузовых единиц | copied_from: formalized.packing_list_1.Goods_7 (PL на сетку .md) |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто по строке | copied_from: formalized.packing_list_1.Goods_7 (PL на сетку .md) |
| 04 | NetWeightQuantity | 24.00 | CD | нетто по строке | copied_from: formalized.packing_list_1.Goods_7 (PL на сетку .md) |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок/мест | operator: formalized.packing_list_1.goods_7.paking_quantity |

- _item_audit: 5

#### Packing List Массив: TransportMeans[2]
- _array_audit: 2

#### Packing List Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | Number | О157АО774 | CO | регистрационный номер | operator: formalized.packing_list_1.transport_1.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator: formalized.packing_list_1.transport_1.mode_code |
| 03 | NationalityCode | 000 | CO | код национальности ТС | operator: formalized.packing_list_1.transport_1.nationality_code |
| 04 | MoverIndicator | true | CO | MoverIndicator | operator: formalized.packing_list_1.transport_1.mover_indicator |

- _item_audit: 4

#### Packing List Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | Number | ВТ374974 | CO | регистрационный номер | operator: formalized.packing_list_1.transport_2.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator: formalized.packing_list_1.transport_2.mode_code |
| 03 | NationalityCode | 000 | CO | код национальности ТС | operator: formalized.packing_list_1.transport_2.nationality_code |
| 04 | MoverIndicator | false | CO | MoverIndicator | operator: formalized.packing_list_1.transport_2.mover_indicator |

- _item_audit: 4
- `doc_status`: confirmed

### `document`: CMR
- `uqi_prefix`: formalized.cmr_1
- `xml_target_root`: AltaE3CMR
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\СМР от СВХ.md
- `file_name`: СМР от СВХ.md
- `note`: CMR №00378 от 20.01.2026

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | LanguageCode | RU | CO | язык документа | operator: formalized.cmr_1.language_code; derived |
| 02 | CMR_Choice | 1 | CO | системный выбор/вариант Альты | operator: formalized.cmr_1.cmr_choice; derived |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | copied_from: formalized.cmr_1 (СМР от СВХ.md, дата принятия груза = дата CMR) |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления | operator: formalized.cmr_1.registration_place |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата CMR | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2 | operator: formalized.cmr_1.taking_cargo_country_code_alpha2 |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза, текст | copied_from: formalized.cmr_1 (СМР от СВХ.md, рус.) |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | operator: formalized.cmr_1.delivery_country_code_alpha2 |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки, текст | copied_from: formalized.cmr_1 (СМР от СВХ.md, рус.) |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки по Incoterms | operator: formalized.cmr_1.delivery_terms_place |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator: formalized.cmr_1.delivery_terms_string |
| 13 | GoodsQuantity | 127 | CD | общее количество грузовых мест | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто по CMR | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 15 | CMRTransport_PrimeMoverStateSignID | О157АО774 | CD | гос. номер тягача | copied_from: formalized.cmr_1 (СМР от СВХ.md); нормализация кириллица |
| 16 | CMRTransport_TrailerStateSignID | ВТ374974 | CD | гос. номер прицепа | copied_from: formalized.cmr_1 (СМР от СВХ.md); нормализация кириллица |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование отправителя | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator: formalized.cmr_1.consignor_shortname_equals_full |
| 19 | Consignor_PostalAddress_CountryCode | CN | CO | страна alpha-2 | copied_from: formalized.invoice_1.Seler_PostalAddress_CountryCode (cross-doc) |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна, текст | copied_from: formalized.cmr_1 (СМР от СВХ.md, рус.) |
| 21 | Consignor_Address_Region | Hebei | CD | регион | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город/район | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | наименование гаранта | operator: formalized.cmr_1.consignor_guarantee_all |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое наименование | operator: formalized.cmr_1.consignor_guarantee_all |
| 26 | Consignor_Guarantee_Address_CountryCode | ОТСУТСТВУЕТ | CO | страна alpha-2 | operator: formalized.cmr_1.consignor_guarantee_all |
| 27 | Consignor_Guarantee_Address_CounryName | ОТСУТСТВУЕТ | CO | страна, текст | operator: formalized.cmr_1.consignor_guarantee_all |
| 28 | Consignor_Guarantee_Address_Region | ОТСУТСТВУЕТ | CO | регион | operator: formalized.cmr_1.consignor_guarantee_all |
| 29 | Consignor_Guarantee_Address_City | ОТСУТСТВУЕТ | CO | город/район | operator: formalized.cmr_1.consignor_guarantee_all |
| 30 | Consignor_Guarantee_Address_StreetHouse | ОТСУТСТВУЕТ | CO | улица/дом одной строкой | operator: formalized.cmr_1.consignor_guarantee_all |
| 31 | Consignee_NameInf | ООО «СКИФ» | CD | наименование получателя | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 32 | Consignee_ShortName | ООО «СКИФ» | CO | краткое наименование | operator: formalized.cmr_1.consignee_shortname_equals_full |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | operator: formalized.cmr_1.consignee_ogrn_from_master_data |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 37 | Consignee_PostalAddress_CountryCode | RU | CO | страна alpha-2 | master_data: master_data.md (declarant_1.country_code) |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 39 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 40 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом/офис одной строкой | copied_from: formalized.cmr_1 (СМР от СВХ.md) |
| 42 | doc_code | 02015 | CD | код документа | константа; derived |
| 43 | doc_name | CMR | CD | наименование документа | константа; derived |
| 44 | doc_number | 00378 | CD | номер документа | = RegistrationDocument_RegID |
| 45 | doc_date | 20.01.2026 | CD | дата документа | = RegistrationDocument_DateInf |

- _audit: 45

#### CMR Массив: CMRGoods[1]
- _array_audit: 1

#### CMR Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CD | описание груза/товара | исключение CMRGoodsDescription — источник non_formalized.svh_1 (ДО-1) |
| 03 | PakingQuantity | 127 | CD | кол-во упаковок/мест | copied_from: formalized.cmr_1.GoodsQuantity |

- _item_audit: 3
- `doc_status`: confirmed

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_7_28.11.2025.md
- `file_name`: currency_transfer_7_28.11.2025.md
- `note`: Заявление на перевод №7 от 28.11.2025

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator: formalized.payment_order_all.document_code; derived |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator: formalized.payment_order_all.payment_mode_code; derived |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md) |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator: formalized.payment_order_all.transaction_kind; derived |
| 05 | Priority | 5 | CO | очередность | operator: formalized.payment_order_all.priority; derived |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md) |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md) |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md) |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md) |
| 10 | Payer_OrganizationName | ООО "СКИФ" | CD | плательщик | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md); нормализация рус. |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md) |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator: formalized.payment_order_1.payer_kpp |
| 13 | Payer_Bank_BankName | Филиал «Центральный» Банка ВТБ (ПАО) в г. Москве | CD | реквизиты банка плательщика | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md) |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD\naccount 40807156900610036383 | CD | получатель платежа | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md) |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH\nSWIFT VTBRCNSHXXX\n/СN767290000018 | CD | реквизиты банка получателя | copied_from: formalized.payment_order_1 (currency_transfer_7_28.11.2025.md) |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator: formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator: formalized.payment_order_all.payer_sign.name |
| 18 | doc_code | 04023 | CD | код документа | константа; derived |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | константа; derived |
| 20 | doc_number | 7 | CD | номер документа | = DocumentReference_PrDocumentNumber |
| 21 | doc_date | 28.11.2025 | CD | дата документа | = DocumentReference_PrDocumentDate |

- _audit: 21
- `doc_status`: confirmed

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_1_13.01.2026.md
- `file_name`: currency_transfer_1_13.01.2026.md
- `note`: Заявление на перевод №1 от 13.01.2026

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator: formalized.payment_order_all.document_code; derived |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator: formalized.payment_order_all.payment_mode_code; derived |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md) |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator: formalized.payment_order_all.transaction_kind; derived |
| 05 | Priority | 5 | CO | очередность | operator: formalized.payment_order_all.priority; derived |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md) |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md) |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md) |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md) |
| 10 | Payer_OrganizationName | ООО "СКИФ" | CD | плательщик | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md); нормализация рус. |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md) |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator: formalized.payment_order_2.payer_kpp |
| 13 | Payer_Bank_BankName | Филиал «Центральный» Банка ВТБ (ПАО) в г. Москве | CD | реквизиты банка плательщика | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md) |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD\naccount 40807156900610036383 | CD | получатель платежа | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md) |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH\nSWIFT VTBRCNSHXXX\n/СN767290000018 | CD | реквизиты банка получателя | copied_from: formalized.payment_order_2 (currency_transfer_1_13.01.2026.md) |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator: formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator: formalized.payment_order_all.payer_sign.name |
| 18 | doc_code | 04023 | CD | код документа | константа; derived |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | константа; derived |
| 20 | doc_number | 1 | CD | номер документа | = DocumentReference_PrDocumentNumber |
| 21 | doc_date | 13.01.2026 | CD | дата документа | = DocumentReference_PrDocumentDate |

- _audit: 21
- `doc_status`: confirmed

### `document`: Service Invoice
- `uqi_prefix`: formalized.service_invoice_1
- `xml_target_root`: AltaServiceInvoice
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_от_27-01-2026.md
- `file_name`: Счет_№26-00378-tl_от_27-01-2026.md
- `note`: Счет за перевозку №26-00378-tl от 27.01.2026, ООО «Трансимпериал»

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentSign | 1 | CO | системный признак документа Альты | operator: formalized.service_invoice_1.document_sign; derived |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 03 | Currency | USD | CD | валюта итого ISO 4217 alpha-3 | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг/перевозчик | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 05 | BankName | АО "Райффайзенбанк"; БИК 044525700; Сч. № 30101810200000000700; Сч. № 40702810400000233463 | CD | банк исполнителя | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги/перевозку | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги/перевозку | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 08 | PrDocumentNumber | 26-00378-tl | CD | номер связанного документа/заказа | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 09 | PrDocumentDate | 12.01.2026 | CD | дата связанного документа/заказа | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | operator: service_invoice_1.consignor_decision = seller |
| 14 | PostalCode | | CO | индекс | operator: formalized.service_invoice_1.consignor_postalcode_empty_ok |
| 15 | CountryCode | CN | CO | страна alpha-2 | copied_from: formalized.invoice_1.Seler_PostalAddress_CountryCode (cross-doc; consignor=seller) |
| 16 | CounryName | КИТАЙ | CO | страна, текст | copied_from: formalized.invoice_1.Seler_PostalAddress_CounryName (cross-doc; consignor=seller) |
| 17 | Region | Hebei | CO | регион | copied_from: formalized.invoice_1.Seler_PostalAddress_Region (cross-doc; consignor=seller) |
| 18 | Town | Shijiazhuang | CO | город/район | copied_from: formalized.invoice_1.Seler_PostalAddress_City (cross-doc; consignor=seller) |
| 19 | StreetHouse | No. 5 Gaodong street | CO | улица/дом одной строкой | copied_from: formalized.invoice_1.Seler_PostalAddress_StreetHouse (cross-doc; consignor=seller) |
| 20 | Consignee_OrganizationName | ООО «СКИФ» | CD | грузополучатель | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator: formalized.service_invoice_1.consignee_ogrn_from_master_data |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 24 | PostalCode | 423800 | CD | индекс | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 25 | CountryCode | RU | CO | страна alpha-2 | master_data: master_data.md (declarant_1.country_code) |
| 26 | CounryName | РОССИЯ | CD | страна, текст | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 27 | Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 28 | Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 29 | StreetHouse | проезд Хлебный | CD | улица | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 30 | House | 30 | CO | дом | operator: formalized.service_invoice_1.consignee_house |
| 31 | Room | 211 | CO | офис/кв | operator: formalized.service_invoice_1.consignee_room |
| 32 | Signature_Choice | 2 | CD | вариант подписи | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md); руководитель + бухгалтер |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | вариант 2 — пусто |
| 34 | IndividualEntrepreneur_PersonName | | CD | первый инициал/имя ИП | вариант 2 — пусто |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | второй инициал/отчество ИП | вариант 2 — пусто |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климовин | CD | фамилия руководителя | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CD | первый инициал/имя руководителя | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | | CD | второй инициал/отчество руководителя | не указано |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CD | первый инициал/имя бухгалтера | copied_from: formalized.service_invoice_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | | CD | второй инициал/отчество бухгалтера | не указано |
| 42 | doc_code | 04031 | CD | код документа | константа; derived |
| 43 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | константа; derived |
| 44 | doc_number | 26-00378-tl | CD | номер документа | = Registration_PrDocumentNumber |
| 45 | doc_date | 27.01.2026 | CD | дата документа | = Registration_PrDocumentDate |
| 46 | transport_to_border | 1404.00 | CO | стоимость маршрута до границы | operator: formalized.service_invoice_1.transport_to_border |
| 47 | transport_currency | USD | CD | валюта страхования | = ServiceDescription[1].ServiceCost_Currency |

- _audit: 47

#### Service Invoice Массив: ServiceDescription[2]
- _array_audit: 2

#### Service Invoice Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsDescription | | CD | многострочное описание услуги | отсутствует |
| 02 | CurrencyCode | USD | CD | валюта строки ISO alpha-3 | copied_from: formalized.service_invoice_1.ServiceDescription_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 03 | ServiceName | China, Hengshui - граница РФ (п/п Манчжурия/Забайкальск) | CD | наименование/маршрут | copied_from: formalized.service_invoice_1.ServiceDescription_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from: formalized.service_invoice_1.ServiceDescription_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from: formalized.service_invoice_1.ServiceDescription_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | copied_from: formalized.service_invoice_1.ServiceDescription_1 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | copied_from: formalized.service_invoice_1.ServiceDescription_1 (Счет_№26-00378-tl_от_27-01-2026.md) |

- _item_audit: 7

#### Service Invoice Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | GoodsDescription | | CD | многострочное описание услуги | отсутствует |
| 02 | CurrencyCode | USD | CD | валюта строки ISO alpha-3 | copied_from: formalized.service_invoice_1.ServiceDescription_2 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 03 | ServiceName | граница РФ (п/п Манчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | наименование/маршрут | copied_from: formalized.service_invoice_1.ServiceDescription_2 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 04 | TaxRate | 0% | CD | ставка налога | copied_from: formalized.service_invoice_1.ServiceDescription_2 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 05 | TaxSum | 0.00 | CD | сумма налога | copied_from: formalized.service_invoice_1.ServiceDescription_2 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | copied_from: formalized.service_invoice_1.ServiceDescription_2 (Счет_№26-00378-tl_от_27-01-2026.md) |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | copied_from: formalized.service_invoice_1.ServiceDescription_2 (Счет_№26-00378-tl_от_27-01-2026.md) |

- _item_audit: 7
- `doc_status`: confirmed

### `document`: Insurance Document
- `uqi_prefix`: formalized.insurance_document_1
- `xml_target_root`: AltaFreeDoc
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md
- `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md
- `note`: Счет за страховку №26-00378-tl/1 от 14.01.2026

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentCode | 04111 | CD | код вида документа | константа; derived |
| 02 | DocumentHead_DocumentName | Счет на оплату №26-00378-tl/1 от 14.01.2026 г. | CD | наименование документа | copied_from: formalized.insurance_document_1 (Счет_№26-00378-tl_1_от_14-01-2026.md) |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | copied_from: formalized.insurance_document_1 (Счет_№26-00378-tl_1_от_14-01-2026.md) |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | copied_from: formalized.insurance_document_1 (Счет_№26-00378-tl_1_от_14-01-2026.md) |
| 05 | TextPara | link:md/Счет_№26-00378-tl_1_от_14-01-2026.md | CD | основной текст/условия | link на md-версию |
| 06 | doc_code | 04111 | CD | код документа | константа; derived |
| 07 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | константа; derived |
| 08 | doc_number | 26-00378-tl/1 | CD | номер документа | = DocumentHead_DocumentNumber |
| 09 | doc_date | 14.01.2026 | CD | дата документа | = DocumentHead_DocumentDate |
| 10 | insurance_to_border | 910.34 | CO | стоимость страхования продавцом | operator: formalized.insurance_document_1.insurance_to_border |
| 11 | insurance_currency | RUB | CD | валюта страхования | operator: formalized.insurance_document_1.insurance_to_border (RUB) |

- _audit: 11
- `doc_status`: confirmed

### `document`: TechDescription
- `uqi_prefix`: formalized.tech_description_1
- `xml_target_root`: AltaFreeDoc
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md
- `file_name`: техничка Антикот, антипыльца антимошка .md
- `note`: Техническое описание на оба кода ТН ВЭД (5804101000 и 7019900095)

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа; derived |
| 02 | DocumentHead_DocumentName | Технические характеристики — Сетки из полиэстера 5804101000 / Сетки из стекловолокна 7019900095 | CD | наименование техописания | copied_from: formalized.tech_description_1 (техничка Антикот, антипыльца антимошка .md) |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата техописания | operator: formalized.tech_description_1.date |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания | operator: formalized.tech_description_1.number |
| 05 | TextPara | link:md/техничка Антикот, антипыльца антимошка .md | CD | технический текст | link на md-версию |
| 06 | doc_code | 05999 | CD | код документа | константа; derived |
| 07 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | константа; derived |
| 08 | doc_number | Б/Н | CD | номер документа | = DocumentHead_DocumentNumber |
| 09 | doc_date | 30.10.2025 | CD | дата документа | = DocumentHead_DocumentDate |

- _audit: 9
- `doc_status`: confirmed

### `document`: Passport
- `uqi_prefix`: formalized.passport_1
- `xml_target_root`: AltaPassport
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\stable_source\Passport_63_09_449948.xml
- `file_name`: Passport_63_09_449948.xml
- `note`: Паспорт представителя Арбузовой А.К. (master_data / stable_source)

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentCode | 11001 | CD | код вида документа | константа; derived |
| 02 | DocumentHead_DocumentName | ПАСПОРТ | CD | наименование документа | константа; derived |
| 03 | DocumentHead_DocumentDate | 11.03.2010 | CD | дата документа | master_data: master_data.md (representative_1.passport_date) |
| 04 | DocumentHead_DocumentNumber | 63 09 449948 | CD | номер документа | master_data: master_data.md (representative_1.passport_series + passport_number) |
| 05 | CardSeries | 63 09 | CD | серия | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 06 | CardNumber | 449948 | CD | номер | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 08 | CardDate | 2010-03-11 | CD | дата выдачи | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 09 | PersonInfo_PersonSurname | АРБУЗОВА | CD | фамилия | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 10 | PersonInfo_PersonName | АНАСТАСИЯ | CD | имя | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 11 | PersonInfo_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 12 | PersonInfo_Sex | 1 | CD | пол | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 13 | PersonInfo_Birthday | 1987-07-25 | CD | дата рождения | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 14 | PersonInfo_Birthplace | город Саратов | CD | место рождения | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 15 | ResidencePlace_PostalCode | 410052 | CD | индекс | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 16 | ResidencePlace_CountryCode | RU | CD | страна alpha-2 | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 17 | ResidencePlace_CounryName | РОССИЯ | CD | страна, текст | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 18 | ResidencePlace_Region | Саратовская область | CD | регион | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 19 | ResidencePlace_City | Саратов | CD | город | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 20 | ResidencePlace_StreetHouse | Ул. Одесская д 11 кв 160 | CD | адрес одной строкой | copied_from: formalized.passport_1 (Passport_63_09_449948.xml) |
| 21 | doc_code | 11001 | CD | код документа | константа; derived |
| 22 | doc_name | ПАСПОРТ | CD | наименование документа | константа; derived |
| 23 | doc_number | 63 09 449948 | CD | номер документа | = DocumentHead_DocumentNumber |
| 24 | doc_date | 11.03.2010 | CD | дата документа | = DocumentHead_DocumentDate |

- _audit: 24
- `doc_status`: confirmed

### `document`: Letter of Attorney
- `uqi_prefix`: formalized.letter_of_attorney_1
- `xml_target_root`: AltaLetterOfAttorney
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\stable_source\LetterOfAttorney_1.xml
- `file_name`: LetterOfAttorney_1.xml
- `note`: Доверенность №1 от 01.02.2026 (master_data / stable_source)

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentCode | 11004 | CD | код вида документа | константа; derived |
| 02 | DocumentHead_DocumentName | ДОВЕРЕННОСТЬ | CD | наименование документа | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 03 | DocumentHead_DocumentDate | 01.02.2026 | CD | дата документа | master_data: master_data.md (letter_of_attorney_1.date) |
| 04 | DocumentHead_DocumentNumber | 1 | CD | номер документа | master_data: master_data.md (letter_of_attorney_1.number) |
| 05 | Subject | link:stable_source/LetterOfAttorney_1.xml | CD | текст доверенности | link на xml-файл |
| 06 | EndDate | 31.12.2026 | CD | действительна до | master_data: master_data.md (letter_of_attorney_1.end_date) |
| 07 | DocumentReference_PrDocumentName | ДОВЕРЕННОСТЬ | CD | наименование доверенности | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер доверенности | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 09 | DocumentReference_PrDocumentDate | 2026-02-01 | CD | дата доверенности | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 10 | Organization_OrganizationName | ООО «СКИФ» | CD | выдавшая организация | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 11 | Organization_ShortName | ООО «СКИФ» | CD | краткое наименование | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 12 | Organization_OGRN | 1201600020390 | CD | ОГРН | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 13 | Organization_INN | 1650389298 | CD | ИНН | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 14 | Organization_KPP | 165001001 | CD | КПП | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 15 | Organization_Address_PostalCode | 423800 | CD | индекс | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 16 | Organization_Address_CountryCode | RU | CD | страна alpha-2 | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 17 | Organization_Address_CounryName | РОССИЯ | CD | страна, текст | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 18 | Organization_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 19 | Organization_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 20 | Organization_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | CD | улица/дом одной строкой | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 21 | Organization_OrganizationPerson_PersonSurname | Саранов | CD | подписант от организации | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 22 | Organization_OrganizationPerson_PersonName | Дмитрий | CD | имя/инициалы | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 23 | Organization_OrganizationPerson_PersonMiddleName | Олегович | CD | отчество | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 24 | Organization_OrganizationPerson_PersonPost | Директор | CD | должность | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 25 | EmpoweredPerson_PersonSurname | АРБУЗОВА | CD | уполномоченное лицо | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 26 | EmpoweredPerson_PersonName | АНАСТАСИЯ | CD | имя | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 27 | EmpoweredPerson_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 28 | EmpoweredPerson_PersonPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 29 | EmpoweredPerson_Passport_IdentityCardCode | RU01001 | CD | код документа | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 30 | EmpoweredPerson_Passport_IdentityCardName | ПАСПОРТ РФ | CD | наименование документа | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml); нормализация |
| 31 | EmpoweredPerson_Passport_IdentityCardSeries | 63 09 | CD | серия | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 32 | EmpoweredPerson_Passport_IdentityCardNumber | 449948 | CD | номер | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 33 | EmpoweredPerson_Passport_IdentityCardDate | 2010-03-11 | CD | дата выдачи | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 34 | EmpoweredPerson_Passport_OrganizationName | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | copied_from: formalized.letter_of_attorney_1 (LetterOfAttorney_1.xml) |
| 35 | doc_code | 11004 | CD | код документа | константа; derived |
| 36 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | = DocumentHead_DocumentName |
| 37 | doc_number | 1 | CD | номер документа | = DocumentHead_DocumentNumber |
| 38 | doc_date | 01.02.2026 | CD | дата документа | = DocumentHead_DocumentDate |

- _audit: 38
- `doc_status`: confirmed

### `document`: Transport Contract
- `uqi_prefix`: non_formalized.transport_contract_1
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\stable_source\FreeDoc_КООО_26651_М.xml
- `file_name`: FreeDoc_КООО_26651_М.xml
- `note`: Договор транспортной экспедиции №КООО/26651/М от 13.05.2025 (stable_source)

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentCode | 04033 | CD | код вида документа | константа; derived |
| 02 | DocumentHead_DocumentName | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование договора | copied_from: formalized.transport_contract_1 (FreeDoc_КООО_26651_М.xml) |
| 03 | DocumentHead_DocumentDate | 13.05.2025 | CD | дата договора | copied_from: formalized.transport_contract_1 (FreeDoc_КООО_26651_М.xml) |
| 04 | DocumentHead_DocumentNumber | КООО/26651/М | CD | номер договора | copied_from: formalized.transport_contract_1 (FreeDoc_КООО_26651_М.xml) |
| 05 | TextPara | link:stable_source/FreeDoc_КООО_26651_М.xml | CD | текст договора | link на xml-файл |
| 06 | doc_code | 04033 | CD | код документа | константа; derived |
| 07 | doc_name | ДОГОВОР ТРАНСПОРТНОЙ ЭКСПЕДИЦИИ | CD | наименование документа | константа; derived |
| 08 | doc_number | КООО/26651/М | CD | номер документа | = DocumentHead_DocumentNumber |
| 09 | doc_date | 13.05.2025 | CD | дата документа | = DocumentHead_DocumentDate |

- _audit: 9
- `doc_status`: confirmed

### `document`: EGRUL
- `uqi_prefix`: non_formalized.egrul_1
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml
- `file_name`: FreeDoc_ЮЭ9965-25-106893283.xml
- `note`: Выписка из ЕГРЮЛ №ЮЭ9965-25-106893283 от 14.07.2025 (stable_source)

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | DocumentCode | 04011 | CD | код вида документа | константа; derived |
| 02 | DocumentHead_DocumentName | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование выписки | copied_from: formalized.egrul_1 (FreeDoc_ЮЭ9965-25-106893283.xml) |
| 03 | DocumentHead_DocumentDate | 14.07.2025 | CD | дата выписки | copied_from: formalized.egrul_1 (FreeDoc_ЮЭ9965-25-106893283.xml) |
| 04 | DocumentHead_DocumentNumber | ЮЭ9965-25-106893283 | CD | номер выписки | copied_from: formalized.egrul_1 (FreeDoc_ЮЭ9965-25-106893283.xml) |
| 05 | TextPara | link:stable_source/FreeDoc_ЮЭ9965-25-106893283.xml | CD | текст выписки | link на xml-файл |
| 06 | doc_code | 04011 | CD | код документа | константа; derived |
| 07 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | константа; derived |
| 08 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | = DocumentHead_DocumentNumber |
| 09 | doc_date | 14.07.2025 | CD | дата документа | = DocumentHead_DocumentDate |

- _audit: 9
- `doc_status`: confirmed

## 3. non_formalized

### `document`: Storage Report (ДО-1)
- `uqi_prefix`: non_formalized.svh_1
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО 14431420260204161621.md
- `file_name`: ДО 14431420260204161621.md
- `note`: Отчет СВХ ДО-1 №0000080 от 03.02.2026

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии/свидетельства СВХ | copied_from: non_formalized.svh_1 (ДО 14431420260204161621.md) |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии/свидетельства СВХ | copied_from: non_formalized.svh_1 (ДО 14431420260204161621.md) |
| 03 | actual_gross_weight | 3500.00 | CD | фактический вес по весам | copied_from: non_formalized.svh_additional_sheet_1 (ДО доп 14431520260204161645.md) |
| 04 | actual_places | 127 | CD | фактическое количество мест | copied_from: non_formalized.svh_additional_sheet_1 (ДО доп 14431520260204161645.md) |
| 05 | transport_reg_number | О157АО774 (Прицеп: ВТ374974) | CD | номер ТС при въезде | copied_from: non_formalized.svh_1 (ДО 14431420260204161621.md) |

- _audit: 5

#### Storage Report Массив: goods[2]
- _array_audit: 2

#### Storage Report Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | tnved | 7019900095 | CD | код товара | copied_from: non_formalized.svh_1.goods_1 (ДО 14431420260204161621.md) |
| 02 | places | 27 | CD | кол-во грузовых мест | copied_from: non_formalized.svh_1.goods_1 (ДО 14431420260204161621.md) |
| 03 | gross_weight_kg | 1710 | CD | вес брутто по строке | copied_from: non_formalized.svh_1.goods_1 (ДО 14431420260204161621.md) |
| 04 | cost | 42228 | CD | стоимость по строке | copied_from: non_formalized.svh_1.goods_1 (ДО 14431420260204161621.md) |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from: non_formalized.svh_1.goods_1 (ДО 14431420260204161621.md) |

- _item_audit: 5

#### Storage Report Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | tnved | 5804101000 | CD | код товара | copied_from: non_formalized.svh_1.goods_2 (ДО 14431420260204161621.md) |
| 02 | places | 100 | CD | кол-во грузовых мест | copied_from: non_formalized.svh_1.goods_2 (ДО 14431420260204161621.md) |
| 03 | gross_weight_kg | 1790 | CD | вес брутто по строке | copied_from: non_formalized.svh_1.goods_2 (ДО 14431420260204161621.md) |
| 04 | cost | 55032 | CD | стоимость по строке | copied_from: non_formalized.svh_1.goods_2 (ДО 14431420260204161621.md) |
| 05 | currency_code | CNY | CD | буквенный код валюты | copied_from: non_formalized.svh_1.goods_2 (ДО 14431420260204161621.md) |

- _item_audit: 5
- `doc_status`: confirmed

### `document`: Transit Declaration
- `uqi_prefix`: non_formalized.td_1
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ТД 10719110_240126_5011363_reg00378тд.md
- `file_name`: ТД 10719110_240126_5011363_reg00378тд.md
- `note`: Транзитная декларация №10719110/240126/5011363 от 24.01.2026

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | number | 10719110/240126/5011363 | CO | номер ТД | operator: non_formalized.transit_declaration_1.number |
| 02 | date | 24.01.2026 | CO | дата ТД | operator: non_formalized.transit_declaration_1.date |
| 03 | customs_post_code | 10404083 | CD | код таможенного органа | copied_from: non_formalized.td_1 (ТД 10719110_240126_5011363_reg00378тд.md) |
| 04 | customs_post_name | ОТО И ТК №3 Т/П НАБЕРЕЖНОЧЕЛНИНСКИЙ | CD | наименование таможенного органа | copied_from: non_formalized.td_1 (ТД 10719110_240126_5011363_reg00378тд.md) |
| 05 | transport_reg_number | О157АО774/ВТ374974 | CD | ТС по ТД | copied_from: non_formalized.td_1 (ТД 10719110_240126_5011363_reg00378тд.md) |

- _audit: 5
- `doc_status`: confirmed

### `document`: Storage Report Additional Sheet
- `uqi_prefix`: non_formalized.svh_additional_sheet_1
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО доп 14431520260204161645.md
- `file_name`: ДО доп 14431520260204161645.md
- `note`: Добавочный лист №1 к отчету ДО-1 №0000080 от 03.02.2026

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | number | 1 | CD | № доп.листа/приложения | copied_from: non_formalized.svh_additional_sheet_1 (ДО доп 14431520260204161645.md) |
| 02 | date | 03.02.2026 | CD | дата доп.листа | copied_from: non_formalized.svh_1 (ДО 14431420260204161621.md) |
| 03 | actual_gross_weight | 3500 | CD | фактический вес по весам | copied_from: non_formalized.svh_additional_sheet_1 (ДО доп 14431520260204161645.md) |
| 04 | actual_places | 127 | CD | фактическое количество мест | copied_from: non_formalized.svh_additional_sheet_1 (ДО доп 14431520260204161645.md) |
| 05 | transport_reg_number | О157АО774 (Прицеп: ВТ374974) | CD | номер ТС при въезде | copied_from: non_formalized.svh_1.transport_reg_number |
| 06 | svh_address_region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион СВХ | copied_from: non_formalized.td_1 (ТД; адрес СВХ из CMR п.3) |
| 07 | svh_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город/нас.пункт СВХ | copied_from: non_formalized.td_1 (ТД; адрес СВХ из CMR п.3) |
| 08 | svh_address_street_house | Производственный пр-д, д. 45 | CD | улица/дом СВХ | copied_from: non_formalized.td_1 (ТД; адрес СВХ из CMR п.3) |
| 09 | svh_customs_code | 10404083 | CD | код таможенного органа | copied_from: non_formalized.td_1 (ТД) |

- _audit: 9
- `doc_status`: confirmed

### `document`: Master Data
- `uqi_prefix`: non_formalized.master_data
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\master_data\master_data.md
- `file_name`: master_data.md
- `note`: Мастер-данные декларанта и представителя

| num | field | value | status | description | note |
|--------------|-------------|-------------------|-----------------|---------------------|------------------|
| 01 | master_data.declarant.organization_name | ООО "СКИФ" | CD | наименование декларанта | master_data: master_data.md (declarant_1.organization_name) |
| 02 | master_data.declarant.phone | +7 (843) 207 18 90 | CD | телефон декларанта | master_data: master_data.md (declarant_1.phone) |
| 03 | master_data.declarant.email | PROM_TAT@MAIL.RU | CD | email декларанта | master_data: master_data.md (declarant_1.email) |
| 04 | master_data.representative.name | АРБУЗОВА АНАСТАСИЯ КОНСТАНТИНОВНА | CD | ФИО представителя | master_data: master_data.md (representative_1) |
| 05 | master_data.representative.phone | +7 927-222-0500 | CD | телефон представителя | master_data: master_data.md (representative_1.phone) |
| 06 | master_data.representative.email | A.K.ARBUZOVA@YANDEX.RU | CD | email представителя | master_data: master_data.md (representative_1.email) |

- _audit: 6
- `doc_status`: confirmed

## 4. Итоги по файлу

### Итого, по файлу:

`total_unreliable_fields`: 0
`formalization_status`: confirmed

## 5. Нерешенные вопросы (Issues)

- Нет.

## 6. `unreliable_fields`:
- Нет.
