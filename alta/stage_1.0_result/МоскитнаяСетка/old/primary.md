## meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 поставка / 7 товарных строк инвойса
- `источники данных:` md + operator_provided_data + stable_source (xml)

## formalized:

### document: Contract
- `uqi_prefix`: formalized.contract_1
- `xml_target_root`: AltaE2CONT
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md
- `file_name`: SALES CONTRACT NoLM-2553.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 03011 | CD | 03011 — код вида документа для графы 44: G44/G441 | |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта; графа 44: G44/G442 | |
| 03 | ContractRegistration_PrDocumentDate | 2025-07-02 | CD | дата контракта; графа 44: G44/G443 | |
| 04 | ContractTerms_Amount | 41904.30 | CD | общая сумма контракта; для контроля/сверки | |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric; для контроля/сверки | operator_provided_data.md: formalized.contract_1.currency_code_numeric
| 06 | ContractTerms_LastDate | 2026-12-31 | CD | срок действия/исполнения; для контроля/сверки | |
| 07 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки / Incoterms | operator_provided_data.md: formalized.contract_1.delivery_terms
| 08 | ContractTerms_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md | CD | текст контракта; в primary.md хранить link | |
| 09 | ContractTerms_DealSign | 1 | CO | системный признак Альты; для импорта | operator_provided_data.md: formalized.contract_1.deal_sign
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта | |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 из cb:country | operator_provided_data.md: formalized.contract_1.foreign_person_country_code_alpha2
| 12 | ForeignPerson_Address_CounryName | Китай | CD | страна продавца, текст (опечатка тега CounryName) | нормализация по cb:country
| 13 | ForeignPerson_Address_Region | Hebei | CD | регион/область продавца | |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца | |
| 15 | ForeignPerson_Address_StreetHouse | No. 5 Gaodong street, Xinhua District | CD | улица/дом продавца одной строкой | |
| 16 | RussianPerson_OrganizationName | ООО «СКИФ» | CD | покупатель/сторона контракта | |
| 17 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН покупателя | разрешено решением оператора: allow_cross_doc_master_data_to_contract_invoice
| 18 | RussianPerson_INN | 1650389298 | CO | ИНН покупателя | разрешено решением оператора: allow_cross_doc_master_data_to_contract_invoice
| 19 | RussianPerson_KPP | 165001001 | CO | КПП покупателя | разрешено решением оператора: allow_cross_doc_master_data_to_contract_invoice
| 20 | RussianPerson_Address_PostalCode | 423800 | CD | индекс покупателя | |
| 21 | RussianPerson_Address_CountryCode | RU | CD | страна покупателя alpha-2 | |
| 22 | RussianPerson_Address_CounryName | Россия | CD | страна покупателя, текст (опечатка тега CounryName) | |
| 23 | RussianPerson_Address_Region | Республика Татарстан | CD | регион покупателя | |
| 24 | RussianPerson_Address_City | Набережные Челны | CD | город покупателя | |
| 25 | RussianPerson_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом/офис одной строкой | |

#### Итого, по документу:
- `doc_fields`: 25 из 25
- `doc_formalization_status`: confirmed

### document: Supplementary Contract
- `uqi_prefix`: formalized.supplementary_contract_1
- `xml_target_root`: AltaSupplementaryContract
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md
- `file_name`: 1 Supplementary agreement to the contract.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения; графа 44: G44/G442 | |
| 02 | IssueDate | 2025-11-25 | CD | дата доп. соглашения; графа 44: G44/G443 | |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта | |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator_provided_data.md: formalized.supplementary_contract_1.currency_code_numeric
| 05 | ContractDescription_LastDate | 2026-12-31 | CO | новый срок действия/исполнения | operator_provided_data.md: formalized.supplementary_contract_1.expiry_date
| 06 | ContractDescription_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md | CD | текст доп. соглашения; хранить link | |
| 07 | ContractDescription_DealSign | 1 | CO | системный признак Альты; для импорта | operator_provided_data.md: formalized.supplementary_contract_1.deal_sign
| 08 | ContractDescription_StockCategorySign | 0 | CO | системный признак Альты | operator_provided_data.md: formalized.supplementary_contract_1.stock_category_sign
| 09 | ContractDescription_BuyerLimitationSign | 0 | CO | системный признак Альты | operator_provided_data.md: formalized.supplementary_contract_1.buyer_limitation_sign
| 10 | ContractDescription_InsuranceSign | 0 | CO | системный признак Альты | operator_provided_data.md: formalized.supplementary_contract_1.insurance_sign
| 11 | RussianPerson_OrganizationName | ООО «СКИФ» | CD | российская сторона; покупатель | |
| 12 | RussianPerson_ShortName | ООО «СКИФ» | CD | краткое наименование | |
| 13 | RussianPerson_OGRN | 1201600020390 | CD | ОГРН | |
| 14 | RussianPerson_INN | 1650389298 | CD | ИНН | |
| 15 | RussianPerson_KPP | 165001001 | CD | КПП | |
| 16 | RussianPerson_Address_PostalCode | 423800 | CD | индекс | |
| 17 | RussianPerson_Address_CountryCode | RU | CD | страна alpha-2 | |
| 18 | RussianPerson_Address_CounryName | Россия | CD | страна, текст | |
| 19 | RussianPerson_Address_Region | Республика Татарстан | CD | регион | |
| 20 | RussianPerson_Address_City | Набережные Челны | CD | город | |
| 21 | RussianPerson_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом одной строкой | |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | иностранная сторона; продавец | |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator_provided_data.md: foreign_person_short_name_equals_full=true
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 из cb:country | operator_provided_data.md: formalized.supplementary_contract_1.foreign_person_country_code_alpha2
| 25 | ForeignPerson_Address_CounryName | Китай | CD | страна, текст | нормализация по cb:country
| 26 | ForeignPerson_Address_Region | Hebei | CO | регион | подставлено из contract_1 по решению оператора |
| 27 | ForeignPerson_Address_City | Shijiazhuang | CO | город/район | подставлено из contract_1 по решению оператора |
| 28 | ForeignPerson_Address_StreetHouse | No. 5 Gaodong street, Xinhua District | CO | улица/дом одной строкой | подставлено из contract_1 по решению оператора |
| 29 | PersonSurname | Li | CO | фамилия подписанта | operator_provided_data.md: formalized.supplementary_contract_1.signed_person_surname
| 30 | PersonName | Jing | CO | имя подписанта | operator_provided_data.md: formalized.supplementary_contract_1.signed_person_name
| 31 | PersonMiddleName |  | CO | отчество подписанта | operator_provided_data.md: formalized.supplementary_contract_1.signed_person_middle_name

#### Итого, по документу:
- `doc_fields`: 31 из 31
- `doc_formalization_status`: pending

### document: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\CL на сетку.md
- `file_name`: CL на сетку.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты; источник для графы 23 | operator_provided_data.md: formalized.invoice_1.exchange_rate
| 02 | CurrencyCode | CNY | CO | валюта инвойса ISO 4217 alpha-3 | operator_provided_data.md: formalized.invoice_1.currency_code
| 03 | DocumentCode | 04021 | CD | 04021 — код вида документа для графы 44 | |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест по инвойсу | |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator_provided_data.md: formalized.invoice_1.places_description
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто по инвойсу | operator_provided_data.md: formalized.invoice_1.total_gross_weight (из PL)
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто по инвойсу | operator_provided_data.md: formalized.invoice_1.total_net_weight (из PL)
| 08 | GCost | 97260.00 | CO | системное поле Альты; дубль TotalCost | operator_provided_data.md: formalized.invoice_1.gcost
| 09 | TotalCost | 97260.00 | CD | итого по инвойсу | |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator_provided_data.md: formalized.invoice_1.delivery_terms_numeric
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator_provided_data.md: formalized.invoice_1.delivery_terms_string
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | operator_provided_data.md: formalized.invoice_1.dispatch_country_code
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | operator_provided_data.md: formalized.invoice_1.trading_country_code
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | operator_provided_data.md: formalized.invoice_1.destination_country_code
| 16 | Registration_PrDocumentName | Commercial invoice | CD | наименование документа | |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | |
| 18 | Registration_PrDocumentDate | 2025-10-30 | CD | дата инвойса | |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | |
| 20 | Contract_PrDocumentDate | 2025-07-02 | CD | дата контракта-ссылки | |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | |
| 23 | Buyer_Name | LLC «SKIF» | CD | наименование покупателя | |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | |
| 26 | Buyer_PostalAddress_CounryName | Россия | CD | страна покупателя, текст | |
| 27 | Buyer_PostalAddress_Region | Republic of Tatarstan | CD | регион | |
| 28 | Buyer_PostalAddress_City | Naberezhnye Chelny | CD | город | |
| 29 | Buyer_PostalAddress_StreetHouse | Khlebny Passage, hause 30, office 211 | CD | улица/дом | |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | продавец (опечатка Seler) | |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | operator_provided_data.md: formalized.invoice_1.seller_country_code_alpha2
| 32 | Seler_PostalAddress_CounryName | Китай | CD | страна продавца, текст | нормализация по cb:country
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город продавца | |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом продавца | |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | нормализация: consignor=seller
| 37 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | нормализация: consignor=seller
| 38 | Consignor_Address_CounryName | Китай | CO | страна грузоотправителя, текст | нормализация: consignor=seller
| 39 | Consignor_Address_Region | Hebei | CO | регион | нормализация: consignor=seller
| 40 | Consignor_Address_City | Shijiazhuang | CO | город/район | нормализация: consignor=seller
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CO | улица/дом | нормализация: consignor=seller
| 42 | Consignee_OrganizationName | LLC «SKIF» | CO | грузополучатель | operator_provided_data.md: consignee_equals_buyer=true
| 43 | Consignee_OGRN | 1201600020390 | CO | ОГРН | разрешено решением оператора: allow_cross_doc_master_data_to_contract_invoice
| 44 | Consignee_INN | 1650389298 | CO | ИНН | разрешено решением оператора: allow_cross_doc_master_data_to_contract_invoice
| 45 | Consignee_KPP | 165001001 | CO | КПП | разрешено решением оператора: allow_cross_doc_master_data_to_contract_invoice
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | |
| 48 | Consignee_Address_CounryName | Россия | CD | страна, текст | |
| 49 | Consignee_Address_Region | Республика Татарстан | CD | регион | |
| 50 | Consignee_Address_City | Набережные Челны | CD | город | |
| 51 | Consignee_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом/офис | |

#### InvoiceGoods_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм для ДТ | |
| 05 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | 055 | CD | код ед.изм доп.количества (cb:unit) | |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator_provided_data.md: formalized.invoice_1.goods_1.gross_weight (из PL)
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator_provided_data.md: formalized.invoice_1.goods_1.net_weight (из PL)
| 09 | Price | 5.85 | CD | цена за единицу | Price per M2
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data.md: origin_country_code_numeric
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data.md: manufacturer
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator_provided_data.md: trade_mark
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data.md: goods_mark
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data.md: model

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 * 30 / Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм для ДТ | |
| 05 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | 055 | CD | код ед.изм доп.количества (cb:unit) | |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator_provided_data.md: formalized.invoice_1.goods_2.gross_weight
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator_provided_data.md: formalized.invoice_1.goods_2.net_weight
| 09 | Price | 5.85 | CD | цена за единицу | Price per M2
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data.md
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data.md
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator_provided_data.md
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data.md
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data.md

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_3
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 / Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2 / Материал: полиэстер | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм для ДТ | |
| 05 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | 055 | CD | код ед.изм доп.количества | |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator_provided_data.md: formalized.invoice_1.goods_3.gross_weight
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator_provided_data.md: formalized.invoice_1.goods_3.net_weight
| 09 | Price | 6.35 | CD | цена за единицу | Price per M2
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data.md
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data.md
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator_provided_data.md
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data.md
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data.md

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_4
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2 / Сетка против пыльцы Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CD | описание товара | |
| 03 | GoodsQuantity | 30 | CD | кол-во | |
| 04 | goods_supplementary_quantity | 1440 | CD | доп.кол-во | |
| 05 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование | |
| 06 | MeasureUnitQualifierName | 055 | CD | код | |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто | operator_provided_data.md: formalized.invoice_1.goods_4.gross_weight
| 08 | NetWeightQuantity | 144.00 | CO | нетто | operator_provided_data.md: formalized.invoice_1.goods_4.net_weight
| 09 | Price | 6.35 | CD | цена | Price per M2
| 10 | TotalCost | 9144.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | происхождение (numeric) | operator_provided_data.md
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data.md
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator_provided_data.md
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator_provided_data.md
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator_provided_data.md

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_5
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара | |
| 03 | GoodsQuantity | 90 | CD | кол-во | |
| 04 | goods_supplementary_quantity | 3780 | CD | доп.кол-во | |
| 05 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование | |
| 06 | MeasureUnitQualifierName | 055 | CD | код | |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто | operator_provided_data.md: formalized.invoice_1.goods_5.gross_weight
| 08 | NetWeightQuantity | 491.40 | CO | нетто | operator_provided_data.md: formalized.invoice_1.goods_5.net_weight
| 09 | Price | 3.40 | CD | цена | Price per M2
| 10 | TotalCost | 12852.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | происхождение (numeric) | operator_provided_data.md
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data.md
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator_provided_data.md
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator_provided_data.md
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator_provided_data.md

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_6
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 : Fiberglass / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание | |
| 03 | GoodsQuantity | 180 | CD | кол-во | |
| 04 | goods_supplementary_quantity | 8640 | CD | доп.кол-во | |
| 05 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование | |
| 06 | MeasureUnitQualifierName | 055 | CD | код | |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто | operator_provided_data.md: formalized.invoice_1.goods_6.gross_weight
| 08 | NetWeightQuantity | 1123.20 | CO | нетто | operator_provided_data.md: formalized.invoice_1.goods_6.net_weight
| 09 | Price | 3.40 | CD | цена | Price per M2
| 10 | TotalCost | 29376.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | происхождение (numeric) | operator_provided_data.md
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data.md
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator_provided_data.md
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator_provided_data.md
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator_provided_data.md

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_7
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 / Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CD | описание | |
| 03 | GoodsQuantity | 5 | CD | кол-во | |
| 04 | goods_supplementary_quantity | 240 | CD | доп.кол-во | |
| 05 | goods_supplementary_uom_name | м² (квадратный метр) | CD | наименование | |
| 06 | MeasureUnitQualifierName | 055 | CD | код | |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто | operator_provided_data.md: formalized.invoice_1.goods_7.gross_weight
| 08 | NetWeightQuantity | 24.00 | CO | нетто | operator_provided_data.md: formalized.invoice_1.goods_7.net_weight
| 09 | Price | 28.00 | CD | цена | Price per M2
| 10 | TotalCost | 6720.00 | CD | стоимость | |
| 11 | OriginCountryCode | 156 | CO | происхождение (numeric) | operator_provided_data.md
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data.md
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator_provided_data.md
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator_provided_data.md
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator_provided_data.md

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 105 из 15 * 7
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 52 из 52
- `doc_formalization_status`: confirmed

### document: Packing List
- `uqi_prefix`: formalized.packing_list_1
- `xml_target_root`: AltaE2PACK
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\PL на сетку.md
- `file_name`: PL на сетку.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто по упаковочному | |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто по упаковочному | |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | CD | грузоотправитель | |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | CO | краткое наименование | operator_provided_data.md: consignor_shortname_equals_full=true
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | operator_provided_data.md: consignor_country_code_alpha2
| 06 | Consignor_Address_CounryName | Китай | CD | страна грузоотправителя, текст | нормализация по cb:country
| 07 | Consignor_Address_Region | Hebei | CD | регион | |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город | |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | |
| 10 | Consignee_OrganizationName | LLC «SKIF» | CD | грузополучатель | |
| 11 | Consignee_ShortName | LLC «SKIF» | CO | краткое наименование | operator_provided_data.md: consignee_shortname_equals_full=true
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | |
| 14 | Consignee_KPP | 165001001 | CD | КПП | |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | |
| 17 | Consignee_Address_CounryName | Россия | CD | страна, текст | |
| 18 | Consignee_Address_Region | Republic of Tatarstan | CD | регион | |
| 19 | Consignee_Address_City | Naberezhnye Chelny | CD | город | |
| 20 | Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | CD | улица/дом/офис | |
| 21 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CD | место поставки | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | внутренний числовой код условий | operator_provided_data.md: formalized.invoice_1.delivery_terms_numeric
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | SALES CONTRACT | CD | наименование контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 2025-07-02 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | Commercial invoice | CD | наименование инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 2025-10-30 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | Packing list | CD | наименование упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CD | № упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 2025-10-30 | CD | дата упаковочного | |
| 33 | registration_doc_name | Упаковочный лист | CO | наименование документа для графы 44 | operator_provided_data.md
| 34 | registration_doc_number | LM-2591 | CO | номер документа | operator_provided_data.md
| 35 | registration_doc_date | 2025-10-30 | CO | дата документа | operator_provided_data.md

#### Goods_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки | |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок/мест в упаковке | operator decision: =GoodsQuantity

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество | |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто | |
| 04 | NetWeightQuantity | 460.80 | CD | нетто | |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок | operator decision: =GoodsQuantity

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_3
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH / Сетка против пыльцы Антипыльца " из полиэстера. Размер рулона 1,42*0,64*0,22 | CD | описание | |
| 02 | GoodsQuantity | 6 | CD | количество | Qty BG
| 03 | GrossWeightQuantity | 265.00 | CD | брутто | |
| 04 | NetWeightQuantity | 252.00 | CD | нетто | |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок | operator decision: =GoodsQuantity

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_4
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH / Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание | |
| 02 | GoodsQuantity | 3 | CD | количество | Qty BG
| 03 | GrossWeightQuantity | 155.00 | CD | брутто | |
| 04 | NetWeightQuantity | 144.00 | CD | нетто | |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок | operator decision: =GoodsQuantity

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_5
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание | |
| 02 | GoodsQuantity | 9 | CD | количество | Qty BG
| 03 | GrossWeightQuantity | 520.00 | CD | брутто | |
| 04 | NetWeightQuantity | 491.40 | CD | нетто | |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок | operator decision: =GoodsQuantity

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_6
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание | |
| 02 | GoodsQuantity | 18 | CD | количество | Qty BG
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто | |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто | |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок | operator decision: =GoodsQuantity

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_7
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | GRID WITH 3 LAYER / Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,72*0,35* 0,31*1 | CD | описание | |
| 02 | GoodsQuantity | 1 | CD | количество | Qty BG
| 03 | GrossWeightQuantity | 25.00 | CD | брутто | |
| 04 | NetWeightQuantity | 24.00 | CD | нетто | |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок | operator decision: =GoodsQuantity

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 35 из 5 * 7
- `array_status`: confirmed

#### TransportMeans_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | Number | О157АО774 | CO | регистрационный номер | operator_provided_data.md
| 02 | ModeCode | 31 | CO | код вида транспорта | operator_provided_data.md
| 03 | NationalityCode | 000 | CO | код “национальности” ТС | operator_provided_data.md
| 04 | MoverIndicator | true | CO | true для тягача | operator_provided_data.md

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### TransportMeans_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | Number | ВТ374974 | CO | регистрационный номер | operator_provided_data.md
| 02 | ModeCode | 31 | CO | код вида транспорта | operator_provided_data.md
| 03 | NationalityCode | 000 | CO | код “национальности” ТС | operator_provided_data.md
| 04 | MoverIndicator | false | CO | false для прицепа | operator_provided_data.md

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 8 из 4 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 37 из 37
- `doc_formalization_status`: confirmed

### document: CMR
- `uqi_prefix`: formalized.cmr_1
- `xml_target_root`: AltaE3CMR
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\СМР от СВХ.md
- `file_name`: СМР от СВХ.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | LanguageCode | RU | CO | язык документа | operator_provided_data.md
| 02 | CMR_Choice | 1 | CO | системный выбор/вариант Альты | operator_provided_data.md
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | |
| 04 | RegistrationDocument_DateInf | 2026-01-20 | CD | дата CMR | |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления | operator_provided_data.md
| 06 | TrakingCargo_TakingCargoDate | 2026-01-20 | CD | дата принятия груза | |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2 | operator_provided_data.md
| 08 | TrakingCargo_TakingCargoPlace_CounryName | Китай | CD | страна принятия груза, текст | нормализация по cb:country
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | operator_provided_data.md
| 10 | DeliveryPlace_CounryName | Россия | CD | страна доставки, текст | |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки (если указано) | operator_provided_data.md
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator_provided_data.md
| 13 | GoodsQuantity | 127 | CD | общее количество грузовых мест | |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто по CMR | |
| 15 | CMRTransport_PrimeMoverStateSignID | О157АО774 | CD | гос. номер тягача | из поля 25 Truck number plate
| 16 | CMRTransport_TrailerStateSignID | ВТ374974 | CD | гос. номер прицепа | из поля 25 Truck number plate
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование отправителя | |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator_provided_data.md
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна отправителя alpha-2 | |
| 20 | Consignor_Address_CounryName | Китай | CD | страна, текст | нормализация по cb:country
| 21 | Consignor_Address_Region | Hebei | CD | регион | |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город | |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | гарант отправителя | operator_provided_data.md: consignor_guarantee_all
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое наименование | operator_provided_data.md
| 26 | Consignor_Guarantee_Address_CountryCode |  | CO | страна alpha-2 | ОТСУТСТВУЕТ
| 27 | Consignor_Guarantee_Address_CounryName |  | CO | страна, текст | ОТСУТСТВУЕТ
| 28 | Consignor_Guarantee_Address_Region |  | CO | регион | ОТСУТСТВУЕТ
| 29 | Consignor_Guarantee_Address_City |  | CO | город | ОТСУТСТВУЕТ
| 30 | Consignor_Guarantee_Address_StreetHouse |  | CO | улица/дом | ОТСУТСТВУЕТ
| 31 | Consignee_NameInf | ООО «Скиф» | CD | наименование получателя | |
| 32 | Consignee_ShortName | ООО «Скиф» | CO | краткое наименование | operator_provided_data.md
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | подставлено из non_formalized.master_data_1 по решению оператора |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | |
| 38 | Consignee_Address_CounryName | Россия | CD | страна, текст | |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом | |

#### CMRGoods_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | авто-нумерация единственной строки
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CO | описание груза/товара как в CMR | исключение CMRGoodsDescription — источник non_formalized.svh_1
| 03 | PakingQuantity | 127 | CO | кол-во упаковок/мест | operator_provided_data.md: goods_1.packing_quantity

#### Итого, по элементу массива:
- `item_fields`: 3 из 3

#### Итого, по массиву:
- `array_elements`: 1
- `item_fields`: всего полей 3 из 3 * 1
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 42 из 42
- `doc_formalization_status`: pending

### document: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_7_28.11.2025.md
- `file_name`: currency_transfer_7_28.11.2025.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | 04023 — код вида документа | operator_provided_data.md
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator_provided_data.md
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator_provided_data.md
| 05 | Priority | 5 | CO | очередность | operator_provided_data.md
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 2025-11-28 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator_provided_data.md
| 13 | Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) | CD | банк плательщика | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CO | фамилия | operator_provided_data.md
| 17 | PersonName | Дмитрий | CO | имя | operator_provided_data.md

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### document: Payment Order
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_1_13.01.2026.md
- `file_name`: currency_transfer_1_13.01.2026.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | 04023 — код вида документа | operator_provided_data.md
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator_provided_data.md
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator_provided_data.md
| 05 | Priority | 5 | CO | очередность | operator_provided_data.md
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 2026-01-13 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН | |
| 12 | Payer_KPP | 165001001 | CO | КПП | operator_provided_data.md
| 13 | Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) | CD | банк плательщика | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CO | фамилия | operator_provided_data.md
| 17 | PersonName | Дмитрий | CO | имя | operator_provided_data.md

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### document: Service Invoice
- `uqi_prefix`: formalized.service_invoice_1
- `xml_target_root`: AltaServiceInvoice
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_от_27-01-2026.md
- `file_name`: Счет_№26-00378-tl_от_27-01-2026.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак документа Альты | operator_provided_data.md
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого | |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг/перевозчик | |
| 05 | BankName | АО "Райффайзенбанк", Россия, 119002, г. Москва, пл Смоленская-Сенная, д. 28 | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги/перевозку | |
| 07 | ContractDetails_PrDocumentDate | 2025-05-13 | CD | дата договора | |
| 08 | PrDocumentNumber | ОТСУТСТВУЕТ | CO | номер связанного документа/заказа | operator_provided_data.md
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CO | дата связанного документа/заказа | operator_provided_data.md
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 2026-01-27 | CD | дата счета | |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | consignor=seller
| 14 | PostalCode |  | CO | индекс | PostalCode оставлять пустым если отсутствует
| 15 | CountryCode | CN | CO | страна | consignor=seller
| 16 | CounryName | Китай | CO | страна, текст | consignor=seller
| 17 | Region | Hebei | CO | регион | consignor=seller
| 18 | Town | Shijiazhuang | CO | город | consignor=seller
| 19 | StreetHouse | No. 5 Gaodong street | CO | улица/дом | consignor=seller
| 20 | Consignee_OrganizationName | ООО "СКиФ" | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | подставлено из non_formalized.master_data_1 по решению оператора |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | |
| 24 | PostalCode | 423800 | CD | индекс | |
| 25 | CountryCode | RU | CD | страна alpha-2 | |
| 26 | CounryName | Россия | CD | страна, текст | |
| 27 | Region | Республика Татарстан | CD | регион | |
| 28 | Town | Набережные Челны | CD | город | |
| 29 | StreetHouse | проезд Хлебный | CD | улица | |
| 30 | House | 30 | CO | дом | operator_provided_data.md
| 31 | Room | 211 | CO | офис/кв | operator_provided_data.md
| 32 | Signature_Choice | 1 | CO | вариант подписи | operator_provided_data.md
| 33 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CO | фамилия руководителя | подтверждено оператором по md:Счет_№26-00378-tl_от_27-01-2026.md |
| 34 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CO | инициалы/имя руководителя | подтверждено оператором по md:Счет_№26-00378-tl_от_27-01-2026.md |
| 35 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CO | фамилия бухгалтера | подтверждено оператором по md:Счет_№26-00378-tl_от_27-01-2026.md |
| 36 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CO | инициалы/имя бухгалтера | подтверждено оператором по md:Счет_№26-00378-tl_от_27-01-2026.md |

#### ServiceDescription_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator_provided_data.md
| 04 | TaxRate | 0% | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### ServiceDescription_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator_provided_data.md
| 04 | TaxRate | 0% | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 14 из 7 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 37 из 37
- `doc_formalization_status`: pending

### document: Insurance Document
- `uqi_prefix`: formalized.insurance_document_1
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md
- `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | 04111 — код вида документа | |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 2026-01-14 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | |
| 05 | TextPara_[1] | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | основной текст/условия (link) | operator_provided_data.md: textpara_storage=link

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

### document: TechDescription
- `uqi_prefix`: formalized.tech_description_1
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка.md
- `file_name`: техничка Антикот, антипыльца антимошка.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | 05999 — код вида документа | |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование техописания | |
| 03 | DocumentHead_DocumentDate | 2025-10-30 | CO | дата техописания | operator_provided_data.md: tech_description defaults
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания | operator_provided_data.md: tech_description defaults
| 05 | TextPara_[1] | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка.md | CD | технический текст без пересказа (link) | |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

## non_formalized:

### document: Storage Report (SVH)
- `uqi_prefix`: non_formalized.svh_1
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО 14431420260204161621.md
- `file_name`: ДО 14431420260204161621.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | number | 0000080 | CD | № ДО-1/ДО-2 | |
| 02 | date | 2026-02-03 | CD | дата ДО-1/ДО-2 | |
| 03 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии/свидетельства СВХ | |
| 04 | warehouse_license_date | 2025-09-18 | CD | дата лицензии/свидетельства СВХ | |
| 05 | actual_gross_weight | 3500 | CD | фактический вес по весам | источник: md\\ДО доп 14431520260204161645.md (строка "Итого") |
| 06 | actual_places | 127 | CD | фактическое количество мест | источник: md\\ДО доп 14431520260204161645.md (строка "Итого") |
| 07 | transport_reg_number | О157АО774/ВТ374974 | CD | номер ТС при въезде/по отчету СВХ | |

#### goods_1
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | tnved | 7019900095 | CD | код товара | |
| 02 | places | 27 | CD | кол-во грузовых мест по строке | |
| 03 | gross_weight_kg | 1710 | CD | вес брутто по строке | |
| 04 | cost | 42228 | CD | стоимость по строке | |
| 05 | currency_code | CNY | CD | буквенный код валюты | |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### goods_2
| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара | |
| 02 | places | 100 | CD | кол-во мест | |
| 03 | gross_weight_kg | 1790 | CD | вес брутто | |
| 04 | cost | 55032 | CD | стоимость | |
| 05 | currency_code | CNY | CD | валюта | |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 10 из 5 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 8 из 8

### document: Storage Report Additional Sheet
- `uqi_prefix`: non_formalized.svh_additional_sheet_1
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО доп 14431520260204161645.md
- `file_name`: ДО доп 14431520260204161645.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | number | 1 | CD | № доп.листа/приложения | |
| 02 | date | 2026-02-03 | CD | дата доп.листа | |
| 03 | actual_gross_weight | 3500 | CD | фактический вес | строка "Итого"
| 04 | actual_places | 127 | CD | фактическое количество мест | строка "Итого"
| 05 | transport_reg_number | O157AO774 (Прицеп: BT374974) | CD | номер ТС при въезде | |
| 06 | svh_address_region | Республика Татарстан | CD | регион СВХ | источник: md\\СМР от СВХ.md, п.3 |
| 07 | svh_address_city | Набережные Челны | CD | город/нас.пункт СВХ | источник: md\\СМР от СВХ.md, п.3 |
| 08 | svh_address_street_house | Производственный пр-д, д.45 | CD | улица/дом СВХ | источник: md\\СМР от СВХ.md, п.3 |
| 09 | svh_customs_code | 10404083 | CD | код таможенного органа | из CMR п.13

#### Итого, по документу:
- `doc_fields`: 9 из 9

### document: Transit Declaration
- `uqi_prefix`: non_formalized.td_1
- `path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ТД 10719110_240126_5011363_reg00378тд.md
- `file_name`: ТД 10719110_240126_5011363_reg00378тд.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | number | 10719110/240126/5011363 | CD | номер ТД | |
| 02 | date | 2026-01-24 | CD | дата ТД | Выпуск разрешен 24.01.2026
| 03 | customs_post_code | 10404083 | CD | код таможенного органа | |
| 04 | customs_post_name | ОТО и ТК №3 т/п Набережночелнинский | CD | наименование таможенного органа | |
| 05 | transport_reg_number | О157АО774/ВТ374974 | CD | ТС по ТД | |

#### Итого, по документу:
- `doc_fields`: 5 из 5

### document: Master data
- `uqi_prefix`: non_formalized.master_data_1
- `path`: alta\stable_source\(multiple)
- `file_name`: FreeDoc_ЮЭ9965-25-106893283.xml; LetterOfAttorney_1.xml; Passport_63_09_449948.xml

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | declarant_name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование декларанта | EGRUL
| 02 | declarant_ogrn | 1201600020390 | CD | ОГРН | EGRUL
| 03 | declarant_inn | 1650389298 | CD | ИНН | EGRUL
| 04 | declarant_kpp | 165001001 | CD | КПП | EGRUL
| 05 | declarant_address_postal_code | 423800 | CD | индекс | EGRUL
| 06 | declarant_address_country_code | RU | CD | страна alpha-2 | |
| 07 | declarant_address_country_name | Россия | CD | страна, текст | |
| 08 | declarant_address_region | РЕСПУБЛИКА ТАТАРСТАН (ТАТАРСТАН) | CD | регион | EGRUL
| 09 | declarant_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | EGRUL
| 10 | declarant_address_street | ПР-Д ХЛЕБНЫЙ | CD | улица | EGRUL
| 11 | declarant_address_building | 30 | CD | дом | EGRUL
| 12 | declarant_address_room | ОФИС 211 | CD | офис | EGRUL
| 13 | declarant_phone | +7 937 779-26-56 | CD | телефон | контракт/договор перевозки
| 14 | declarant_email |  | CO | e-mail | operator_provided_data.md
| 15 | representative_last_name | АРБУЗОВА | CD | фамилия | LOA/Passport
| 16 | representative_first_name | АНАСТАСИЯ | CD | имя | LOA/Passport
| 17 | representative_middle_name | КОНСТАНТИНОВНА | CD | отчество | LOA/Passport
| 18 | representative_position | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | должность/статус | LOA
| 19 | representative_phone | +7-927-030-70-07 | CD | телефон | LOA Subject
| 20 | representative_email |  | CO | e-mail | operator_provided_data.md
| 21 | representative_passport_code | RU01001 | CD | код документа | LOA
| 22 | representative_passport_name | ПАСРФ | CD | наименование документа | LOA
| 23 | representative_passport_series | 63 09 | CD | серия | LOA/Passport
| 24 | representative_passport_number | 449948 | CD | номер | LOA/Passport
| 25 | representative_passport_date | 2010-03-11 | CD | дата выдачи | LOA/Passport
| 26 | representative_passport_issuer | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | LOA/Passport
| 27 | representative_authority_doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа полномочий | LOA
| 28 | representative_authority_doc_number | 1 | CD | № доверенности | LOA
| 29 | representative_authority_doc_date_from | 2026-02-01 | CD | дата начала | LOA
| 30 | representative_authority_doc_date_to | 2026-12-31 | CD | дата окончания | LOA
| 31 | note | Источники: FreeDoc_ЮЭ9965-25-106893283.xml (ЕГРЮЛ), LetterOfAttorney_1.xml, Passport_63_09_449948.xml; declarant/representative emails пустые по решению оператора | CD | пояснение | |

#### Итого, по документу:
- `doc_fields`: 31 из 31

## Нерешенные вопросы (Issues)
- formalized.supplementary_contract_1.ForeignPerson_Address_Region
  - `question`: В доп. соглашении №1 (md\1 Supplementary agreement to the contract.md) отсутствует адрес продавца. Подтверди регион/город/улицу продавца (как в контракте: Hebei / Shijiazhuang / No.5 Gaodong street ...), чтобы закрыть поля.
- formalized.supplementary_contract_1.ForeignPerson_Address_City
  - `question`: То же, требуется подтверждение города продавца для доп. соглашения.
- formalized.supplementary_contract_1.ForeignPerson_Address_StreetHouse
  - `question`: То же, требуется подтверждение улица/дом продавца для доп. соглашения.
- formalized.cmr_1.Consignee_OGRNID
  - `question`: В CMR (md\СМР от СВХ.md) ОГРН получателя не указан. Можно ли подставить ОГРН 1201600020390 из master data (статус CO) для поля Consignee_OGRNID?
- formalized.service_invoice_1.Consignee_RFOrganizationFeatures_OGRN
  - `question`: В счете ТЭО (md\Счет_№26-00378-tl_от_27-01-2026.md) ОГРН покупателя не указан. Можно ли подставить ОГРН 1201600020390 из master data (статус CO)?
- formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonSurname
  - `question`: В счете ТЭО (md\Счет_№26-00378-tl_от_27-01-2026.md) есть подпись «Ген. директор ___ Климович Л.А.». Подтверди: Director surname=Климович, name/initials=Л.А.?
- formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname
  - `question`: В счете ТЭО (md\Счет_№26-00378-tl_от_27-01-2026.md) есть подпись «Бухгалтер ___ Лехно О.А.». Подтверди: ChiefAccountant surname=Лехно, name/initials=О.А.?
- non_formalized.svh_additional_sheet_1.svh_address_region
  - `question`: В доп.листе ДО (md\ДО доп 14431520260204161645.md) нет адреса СВХ. Подтверди регион/город/улица СВХ ООО «ЛОГИКАМ» (из CMR п.3: Производственный пр-д, д.45, Набережные Челны).
- non_formalized.svh_additional_sheet_1.svh_address_city
  - `question`: То же: город/нас.пункт СВХ.
- non_formalized.svh_additional_sheet_1.svh_address_street_house
  - `question`: То же: улица/дом СВХ.

### Итогo, по файлу:
- `total_fields`: 263
- `formalization_status`: pending
- `total_doc_fields`: 256
