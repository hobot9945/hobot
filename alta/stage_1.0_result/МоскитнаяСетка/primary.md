## meta:
- `название кейса`: МоскитнаяСетка / HEBEI LANGMAI IMPORT AND EXPORT / 02
- `путь к папке поставки`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 2 кода ТН ВЭД / 7 товарных строк инвойса
- `источники данных:` md + operator_provided_data + stable_source (xml)

## formalized

### document: Contract / Контракт (03011)
- `uqi_prefix`: formalized.contract_1
- `xml_target_root`: AltaE2CONT
- `path`: md\SALES CONTRACT NoLM-2553.md
- `file_name`: SALES CONTRACT NoLM-2553.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 03011 | CD | 03011 — код вида документа для графы 44: G44/G441; константа; derived | derived: constant |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта; графа 44: G44/G442 | md: SALES CONTRACT NoLM-2553.md (Page 1) |
| 03 | ContractRegistration_PrDocumentDate | 2025-07-02 | CD | дата контракта; графа 44: G44/G443 | md: SALES CONTRACT NoLM-2553.md (Page 1) |
| 04 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта; для контроля/сверки; в dt.xml обычно напрямую не печатается | operator: formalized.contract_1.ContractTerms_Amount |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric; для контроля/сверки; напр. CNY=156 | operator: formalized.contract_1.currency_code_numeric |
| 06 | ContractTerms_LastDate | 2026-12-31 | CD | срок действия/исполнения; для контроля/сверки | md: SALES CONTRACT NoLM-2553.md (Shipment period end) |
| 07 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки / Incoterms, напр. `EXW ...`; источник для графы 20: G_20_1 | operator: formalized.contract_1.delivery_terms |
| 08 | ContractTerms_ContractText | link:md\SALES CONTRACT NoLM-2553.md | CD | текст контракта; в primary.md хранить `link` на файл-источник | link to md |
| 09 | ContractTerms_DealSign | 1 | CO | `1` - системный признак Альты; для импорта; derived | operator: formalized.contract_1.deal_sign |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта; обычно совпадает с отправителем; может использоваться для сверок | md: SALES CONTRACT NoLM-2553.md |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 из `cb:country`; derived | operator: formalized.contract_1.foreign_person_country_code_alpha2 |
| 12 | ForeignPerson_Address_CounryName | Китай | CD | страна продавца, текст; **опечатка тега CounryName**; | derived from CN (cb:country) |
| 13 | ForeignPerson_Address_Region | Hebei | CD | регион/область продавца; для сверок | copied_from: formalized.invoice_1.Seler_PostalAddress_Region (md\\CL на сетку .md) |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца; для сверок | md: SALES CONTRACT NoLM-2553.md (Page 4, OCR mid) |
| 15 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, China | CD | улица/дом продавца одной строкой; для сверок | md: SALES CONTRACT NoLM-2553.md (Page 4, OCR mid) |
| 16 | RussianPerson_OrganizationName | ООО "СКИФ" | CD | покупатель/сторона контракта; обычно совпадает с декларантом/получателем; для сверок | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 17 | RussianPerson_OGRN | 1201600020390 | CD | ОГРН покупателя; для сверок/мастер-данных | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 18 | RussianPerson_INN | 1650389298 | CD | ИНН покупателя; для сверок/мастер-данных | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 19 | RussianPerson_KPP | 165001001 | CD | КПП покупателя; для сверок/мастер-данных | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 20 | RussianPerson_Address_PostalCode | 423800 | CD | индекс покупателя; для сверок/мастер-данных | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 21 | RussianPerson_Address_CountryCode | RU | CD | страна покупателя alpha-2; для сверок/мастер-данных | derived (cb:country) |
| 22 | RussianPerson_Address_CounryName | Россия | CD | страна покупателя, текст; **опечатка тега CounryName**; для сверок/мастер-данных | derived (cb:country) |
| 23 | RussianPerson_Address_Region | Республика Татарстан | CD | регион покупателя; для сверок/мастер-данных | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 24 | RussianPerson_Address_City | Набережные Челны | CD | город покупателя; для сверок/мастер-данных | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 25 | RussianPerson_Address_StreetHouse | ПР-Д ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой; для сверок/мастер-данных | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |

#### Итого, по документу:
- `doc_fields`: 25 из 25
- `doc_formalization_status`: confirmed

### document: Supplementary Contract / Дополнительное соглашение (03012)
- `uqi_prefix`: formalized.supplementary_contract_1
- `xml_target_root`: AltaSupplementaryContract
- `path`: md\1 Supplementary agreement to the contract.md
- `file_name`: 1 Supplementary agreement to the contract.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения; графа 44: G44/G442 | md: 1 Supplementary agreement... |
| 02 | IssueDate | 2025-11-25 | CD | дата доп. соглашения; графа 44: G44/G443 | md: 1 Supplementary agreement... |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта; для контроля/сверки | md: 1 Supplementary agreement... |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric; для контроля/сверки | operator: formalized.supplementary_contract_1.currency_code_numeric |
| 05 | ContractDescription_LastDate | 2026-12-31 | CO | новый срок действия/исполнения; для контроля/сверки | operator: formalized.supplementary_contract_1.expiry_date |
| 06 | ContractDescription_ContractText | link:md\1 Supplementary agreement to the contract.md | CD | текст доп. соглашения; в primary.md хранить `link` на файл-источник; в dt.xml напрямую не переносится | link to md |
| 07 | ContractDescription_DealSign | 1 | CO | `1` - системный признак Альты; для импорта; константа; derived | operator: formalized.supplementary_contract_1.deal_sign |
| 08 | ContractDescription_StockCategorySign | 0 | CO | `0` - системный признак Альты; для импорта; константа; derived | operator: formalized.supplementary_contract_1.stock_category_sign |
| 09 | ContractDescription_BuyerLimitationSign | 0 | CO | `0` - системный признак Альты; для импорта; константа; derived | operator: formalized.supplementary_contract_1.buyer_limitation_sign |
| 10 | ContractDescription_InsuranceSign | 0 | CO | `0` - системный признак Альты; для импорта; константа; derived | operator: formalized.supplementary_contract_1.insurance_sign |
| 11 | RussianPerson_OrganizationName | ООО "СКИФ" | CD | российская сторона; покупатель; для сверок/мастер-данных | master_data |
| 12 | RussianPerson_ShortName | ООО "СКИФ" | CD | краткое наименование; для сверок/мастер-данных | master_data |
| 13 | RussianPerson_OGRN | 1201600020390 | CD | ОГРН; для сверок/мастер-данных | master_data |
| 14 | RussianPerson_INN | 1650389298 | CD | ИНН; для сверок/мастер-данных | master_data |
| 15 | RussianPerson_KPP | 165001001 | CD | КПП; для сверок/мастер-данных | master_data |
| 16 | RussianPerson_Address_PostalCode | 423800 | CD | индекс; для сверок/мастер-данных | master_data |
| 17 | RussianPerson_Address_CountryCode | RU | CD | страна alpha-2; для сверок/мастер-данных | derived (cb:country) |
| 18 | RussianPerson_Address_CounryName | Россия | CD | страна, текст; **опечатка тега CounryName**; для сверок/мастер-данных | derived (cb:country) |
| 19 | RussianPerson_Address_Region | Республика Татарстан | CD | регион; для сверок/мастер-данных | master_data |
| 20 | RussianPerson_Address_City | Набережные Челны | CD | город; для сверок/мастер-данных | master_data |
| 21 | RussianPerson_Address_StreetHouse | ПР-Д ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом одной строкой; для сверок/мастер-данных | master_data |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | иностранная сторона; продавец; для сверок | md: 1 Supplementary agreement... |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | краткое наименование; для сверок | md: 1 Supplementary agreement... |
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 из `cb:country`; для сверок | operator: formalized.supplementary_contract_1.foreign_person_country_code_alpha2 |
| 25 | ForeignPerson_Address_CounryName | Китай | CD | страна, текст; **опечатка тега CounryName**; для сверок | derived (cb:country) |
| 26 | ForeignPerson_Address_Region | Hebei | CD | регион; для сверок | copied_from: formalized.invoice_1.Seler_PostalAddress_Region (md\\CL на сетку .md) |
| 27 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район; для сверок | copied_from: formalized.contract_1.ForeignPerson_Address_City (md\\SALES CONTRACT NoLM-2553.md) |
| 28 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District, Shijiazhuang, China | CD | улица/дом одной строкой; для сверок | copied_from: formalized.contract_1.ForeignPerson_Address_StreetHouse (md\\SALES CONTRACT NoLM-2553.md) |
| 29 | PersonSurname | Li | CO | фамилия подписанта | operator: formalized.supplementary_contract_1.signed_person_surname |
| 30 | PersonName | Jing | CO | имя подписанта | operator: formalized.supplementary_contract_1.signed_person_name |
| 31 | PersonMiddleName |  | CO | отчество подписанта | operator: formalized.supplementary_contract_1.signed_person_middle_name |

#### Итого, по документу:
- `doc_fields`: 31 из 31
- `doc_formalization_status`: confirmed

### document: Invoice / Инвойс (04021)
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: md\CL на сетку .md
- `file_name`: CL на сетку .md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты; источник для графы 23: G_23_1, G_23_2 | operator: formalized.invoice_1.exchange_rate |
| 02 | CurrencyCode | CNY | CO | валюта инвойса ISO 4217 alpha-3, напр. `CNY`, `USD`; источник для графы 22: G_22_3 | operator: formalized.invoice_1.currency_code |
| 03 | DocumentCode | 04021 | CD | 04021 — код вида документа для графы 44: G44/G441; константа; derived | derived: constant |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест по инвойсу; приоритет #3 для графы 6: G_6_1 | md: CL на сетку .md |
| 05 | PlacesDescription | Поддон | CO | описание мест, напр. "Поддон"; для сверок/контекста, обычно не в dt.xml напрямую | operator: formalized.invoice_1.places_description |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто по инвойсу; для сверок с CMR/PL/СВХ | operator: formalized.invoice_1.total_gross_weight |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто по инвойсу; для сверок | operator: formalized.invoice_1.total_net_weight |
| 08 | GCost | 97260.00 | CO | системное поле Альты; дубль `TotalCost`; для импорта/совместимости; derived | operator: formalized.invoice_1.gcost |
| 09 | TotalCost | 97260.00 | CO | итого по инвойсу; источник для графы 22: G_22_2 | operator: formalized.invoice_1.total_cost |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms; источник для графы 20: G_20_2 | md: CL на сетку .md (HEBEI/Хэбей) |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки; источник для графы 20: G_20_1_1 / внутренний код Альты | operator: formalized.invoice_1.delivery_terms_numeric |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий, напр. `EXW`; источник для графы 20: G_20_1 | operator: formalized.invoice_1.delivery_terms_string |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2; источник для графы 15A: G_15A_1 | operator: formalized.invoice_1.dispatch_country_code |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2; источник для графы 11: G_11_1 | operator: formalized.invoice_1.trading_country_code |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2; источник для графы 17A: G_17A_1 | operator: formalized.invoice_1.destination_country_code |
| 16 | Registration_PrDocumentName | Commercial invoice | CD | наименование документа для печати/сверок; может использоваться в графе 44: G44/G444 | md: CL на сетку .md |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса; графа 44: G44/G442 | md: CL на сетку .md |
| 18 | Registration_PrDocumentDate | 2025-10-30 | CD | дата инвойса; графа 44: G44/G443 | md: CL на сетку .md |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки; для связи документов; графа 44: G44/G442 (для контракта) | md: CL на сетку .md |
| 20 | Contract_PrDocumentDate | 2025-07-02 | CD | дата контракта-ссылки; для связи документов; графа 44: G44/G443 (для контракта) | md: CL на сетку .md |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя; источник для мастер-данных; графы 8/9/14: G_8_6, G_9_4, G_14_4 | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя; источник для мастер-данных; графы 8/9/14: G_8_6, G_9_4, G_14_4 | master_data |
| 23 | Buyer_Name | ООО "СКИФ" | CD | наименование покупателя; графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM | master_data |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS | master_data |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC | derived (cb:country) |
| 26 | Buyer_PostalAddress_CounryName | Россия | CD | страна покупателя, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, G_14_CN | derived |
| 27 | Buyer_PostalAddress_Region | Республика Татарстан | CD | регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB | master_data |
| 28 | Buyer_PostalAddress_City | Набережные Челны | CD | город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT | master_data |
| 29 | Buyer_PostalAddress_StreetHouse | ПР-Д ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR | master_data |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец; **опечатка Seler**; источник для графы 2: G_2_NAM | md: CL на сетку .md |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 из `cb:country`; графа 2: G_2_7 | operator: formalized.invoice_1.seller_country_code_alpha2 |
| 32 | Seler_PostalAddress_CounryName | Китай | CD | страна продавца, текст; **опечатка CounryName**; графа 2: G_2_50 | derived |
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца; графа 2: G_2_SUB | md: CL на сетку .md |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город/район продавца; графа 2: G_2_CIT | md: CL на сетку .md |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street. | CD | улица/дом одной строкой; графа 2: G_2_STR | md: CL на сетку .md |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | грузоотправитель; если отличается от продавца — для сверок/графы 2 | нормализация: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 из `cb:country`; для сверок | нормализация: consignor=seller |
| 38 | Consignor_Address_CounryName | Китай | CD | страна грузоотправителя, текст; **опечатка CounryName**; для сверок | нормализация: consignor=seller |
| 39 | Consignor_Address_Region | Hebei | CD | регион; для сверок | нормализация: consignor=seller |
| 40 | Consignor_Address_City | Shijiazhuang | CD | город/район; для сверок | нормализация: consignor=seller |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street. | CD | улица/дом одной строкой; для сверок | нормализация: consignor=seller |
| 42 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель; обычно получатель/декларант; графы 8/9/14: G_8_NAM, G_9_NAM, G_14_NAM | master_data |
| 43 | Consignee_OGRN | 1201600020390 | CD | ОГРН; графы 8/9/14: G_8_1, G_9_1, G_14_1 | master_data |
| 44 | Consignee_INN | 1650389298 | CD | ИНН; графы 8/9/14: G_8_6, G_9_4, G_14_4 | master_data |
| 45 | Consignee_KPP | 165001001 | CD | КПП; графы 8/9/14: G_8_6, G_9_4, G_14_4 | master_data |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс; графы 8/9/14: G_8_POS, G_9_POS, G_14_POS | master_data |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2; графы 8/9/14: G_8_7, G_9_CC, G_14_CC | derived |
| 48 | Consignee_Address_CounryName | Россия | CD | страна, текст; **опечатка CounryName**; графы 8/9/14: G_8_50, G_9_CN, G_14_CN | derived |
| 49 | Consignee_Address_Region | Республика Татарстан | CD | регион; графы 8/9/14: G_8_SUB, G_9_SUB, G_14_SUB | master_data |
| 50 | Consignee_Address_City | Набережные Челны | CD | город; графы 8/9/14: G_8_CIT, G_9_CIT, G_14_CIT | master_data |
| 51 | Consignee_Address_StreetHouse | ПР-Д ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой; графы 8/9/14: G_8_STR, G_9_STR, G_14_STR | master_data |

#### InvoiceGoods_1

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД; источник для графы 33: G_33_1 | md: CL на сетку .md row 1 |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара как в инвойсе; источник для графы 31 | md |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке инвойса в “основной” единице строки | md |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм для ДТ; например, `Quantity in M2` | md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм из `cb:unit` | md |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества для ДТ, наименование из `cb:unit` | derived: cb:unit 055 |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_1.gross_weight (from PL) |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator: formalized.invoice_1.goods_1.net_weight (from PL) |
| 09 | Price | 5.85 | CD | цена за единицу | md |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator: formalized.invoice_1.goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: formalized.invoice_1.goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator: formalized.invoice_1.goods_all.trade_mark |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator: formalized.invoice_1.goods_all.goods_mark |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator: formalized.invoice_1.goods_all.model |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_2

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД; источник для графы 33: G_33_1 | md row 2 |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 * 30 / Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара как в инвойсе | md |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке инвойса | md |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | md |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп.количества | derived cb:unit 055 |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_2.gross_weight |
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator: formalized.invoice_1.goods_2.net_weight |
| 09 | Price | 5.85 | CD | цена за единицу | md |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator goods_all |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator goods_all |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator goods_all |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator goods_all |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator goods_all |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_3

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | md row 3 |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 / Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2 / Материал: полиэстер | CD | описание товара | md |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке | md |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | md |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп.количества | derived cb:unit 055 |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_3.gross_weight |
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator: formalized.invoice_1.goods_3.net_weight |
| 09 | Price | 6.35 | CD | цена за единицу | md |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator goods_all |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator goods_all |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator goods_all |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator goods_all |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator goods_all |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_4

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | md row 4 |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2 / Сетка против пыльцы Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CD | описание товара | md |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке | md |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм | md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | md |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп.количества | derived cb:unit 055 |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_4.gross_weight |
| 08 | NetWeightQuantity | 144.00 | CO | нетто по строке | operator: formalized.invoice_1.goods_4.net_weight |
| 09 | Price | 6.35 | CD | цена за единицу | md |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator goods_all |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator goods_all |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator goods_all |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator goods_all |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator goods_all |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_5

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | md row 5 |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара | md |
| 03 | GoodsQuantity | 90 | CD | кол-во по строке | md |
| 04 | goods_supplementary_quantity | 3780 | CD | количество в доп.ед.изм | md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | md |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп.количества | derived cb:unit 055 |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_5.gross_weight |
| 08 | NetWeightQuantity | 491.40 | CO | нетто по строке | operator: formalized.invoice_1.goods_5.net_weight |
| 09 | Price | 3.40 | CD | цена за единицу | md |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator goods_all |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator goods_all |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator goods_all |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator goods_all |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator goods_all |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_6

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД | md row 6 |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 : Fiberglass / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара | md |
| 03 | GoodsQuantity | 180 | CD | кол-во по строке | md |
| 04 | goods_supplementary_quantity | 8640 | CD | количество в доп.ед.изм | md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | md |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп.количества | derived cb:unit 055 |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_6.gross_weight |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто по строке | operator: formalized.invoice_1.goods_6.net_weight |
| 09 | Price | 3.40 | CD | цена за единицу | md |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator goods_all |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator goods_all |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator goods_all |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator goods_all |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator goods_all |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_7

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | md row 7 |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 / Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара | md |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке | md |
| 04 | goods_supplementary_quantity | 240 | CD | количество в доп.ед.изм | md |
| 05 | goods_supplementary_uom_name | M2 | CD | наименование доп.ед.изм | md |
| 06 | MeasureUnitQualifierName | м² | CD | ед. изм. доп.количества | derived cb:unit 055 |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто по строке | operator: formalized.invoice_1.goods_7.gross_weight |
| 08 | NetWeightQuantity | 24.00 | CO | нетто по строке | operator: formalized.invoice_1.goods_7.net_weight |
| 09 | Price | 28.00 | CD | цена за единицу | md |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | md |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator goods_all |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator goods_all |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator goods_all |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator goods_all |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator goods_all |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 105 из 15 * 7
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 52 из 52
- `doc_formalization_status`: confirmed

### document: Packing List / Упаковочный лист (04131)
- `uqi_prefix`: formalized.packing_list_1
- `xml_target_root`: AltaE2PACK
- `path`: md\PL на сетку .md
- `file_name`: PL на сетку .md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто по упаковочному; для сверок; может участвовать по графам 35/38 при необходимости; derived | md totals |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто по упаковочному; для сверок; derived | md totals |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | грузоотправитель; для сверок с инвойсом/CMR | copied_from: formalized.invoice_1.Seler_Name (md\\CL на сетку .md) |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | краткое наименование; для сверок | operator: formalized.packing_list_1.consignor_shortname_equals_full=true |
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2, используй `cb:country`; для сверок | operator: formalized.packing_list_1.consignor_country_code_alpha2 |
| 06 | Consignor_Address_CounryName | Китай | CD | страна грузоотправителя, текст; **опечатка CounryName**; для сверок | derived |
| 07 | Consignor_Address_Region | Hebei | CD | регион; для сверок | copied_from: formalized.invoice_1.Seler_PostalAddress_Region |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город/район; для сверок | copied_from: formalized.invoice_1.Seler_PostalAddress_City |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street. | CD | улица/дом одной строкой; для сверок | copied_from: formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 10 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель; для сверок/мастер-данных | master_data |
| 11 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование; для сверок/мастер-данных | operator: formalized.packing_list_1.consignee_shortname_equals_full=true |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН; для сверок/мастер-данных | master_data |
| 13 | Consignee_INN | 1650389298 | CD | ИНН; для сверок/мастер-данных | master_data |
| 14 | Consignee_KPP | 165001001 | CD | КПП; для сверок/мастер-данных | master_data |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс; для сверок/мастер-данных | master_data |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2; для сверок/мастер-данных | derived |
| 17 | Consignee_Address_CounryName | Россия | CD | страна, текст; **опечатка CounryName**; для сверок/мастер-данных | derived |
| 18 | Consignee_Address_Region | Республика Татарстан | CD | регион; для сверок/мастер-данных | master_data |
| 19 | Consignee_Address_City | Набережные Челны | CD | город; для сверок/мастер-данных | master_data |
| 20 | Consignee_Address_StreetHouse | ПР-Д ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой; для сверок/мастер-данных | master_data |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms; источник для графы 20: G_20_2 | md: PL на сетку .md |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | внутренний числовой код условий; источник для графы 20 | operator: formalized.invoice_1.delivery_terms_numeric (принято для связанного набора документов) |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий, напр. `EXW`; источник для графы 20 | md + operator |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | наименование контракта для печати/графы 44 | operator: formalized.packing_list_1.DeliveryTerms_Contract_PrDocumentName |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта; графа 44 | md |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 2025-07-02 | CD | дата контракта; графа 44 | md |
| 27 | DeliveryTerms_Invoice_PrDocumentName | Commercial invoice | CD | наименование инвойса для печати/графы 44 | derived from invoice |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса; графа 44 | md |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 2025-10-30 | CD | дата инвойса; графа 44 | md |
| 30 | DeliveryTerms_Registration_PrDocumentName | Упаковочный лист | CO | наименование упаковочного; графа 44 | operator: formalized.packing_list_1.registration_doc_name |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного; графа 44 | operator: formalized.packing_list_1.registration_doc_number |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 2025-10-30 | CO | дата упаковочного; графа 44 | operator: formalized.packing_list_1.registration_doc_date |
| 33 | registration_doc_name | Упаковочный лист | CO | наименование документа для графы 44: G44/G444; напр. `УПАКОВОЧНЫЙ ЛИСТ` | operator |
| 34 | registration_doc_number | LM-2591 | CO | номер документа для графы 44: G44/G442 | operator |
| 35 | registration_doc_date | 30.10.2025 | CO | дата документа для графы 44: G44/G443 | operator |

#### Goods_1

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки как в документе | md: PL |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц в строке | md |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | md |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | md |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок/мест в упаковке | operator: formalized.packing_list_1.goods_1.paking_quantity (решение: =GoodsQuantity) |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_2

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки как в документе | md |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц в строке | md |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто по строке | md |
| 04 | NetWeightQuantity | 460.80 | CD | нетто по строке | md |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок/мест в упаковке | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_3

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера . Размер рулона 1,42*0,64*0,22 | CD | описание строки как в документе | md |
| 02 | GoodsQuantity | 6 | CD | количество мест/грузовых единиц в строке | md |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто по строке | md |
| 04 | NetWeightQuantity | 252.00 | CD | нетто по строке | md |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок/мест в упаковке | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_4

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание строки как в документе | md |
| 02 | GoodsQuantity | 3 | CD | количество мест/грузовых единиц в строке | md |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто по строке | md |
| 04 | NetWeightQuantity | 144.00 | CD | нетто по строке | md |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок/мест в упаковке | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_5

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание строки как в документе | md |
| 02 | GoodsQuantity | 9 | CD | количество мест/грузовых единиц в строке | md |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто по строке | md |
| 04 | NetWeightQuantity | 491.40 | CD | нетто по строке | md |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок/мест в упаковке | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_6

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание строки как в документе | md |
| 02 | GoodsQuantity | 18 | CD | количество мест/грузовых единиц в строке | md |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | md |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто по строке | md |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок/мест в упаковке | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_7

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки "Антипыльца " из полиэстера Размер рулона 1,72*0,35*0,31*1 | CD | описание строки как в документе | md |
| 02 | GoodsQuantity | 1 | CD | количество мест/грузовых единиц в строке | md |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто по строке | md |
| 04 | NetWeightQuantity | 24.00 | CD | нетто по строке | md |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок/мест в упаковке | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 35 из 5 * 7
- `array_status`: confirmed

#### TransportMeans_1

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | Number | О157АО774 | CO | регистрационный номер; источник для графы 18: G_18 | operator: formalized.packing_list_1.transport_1.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator: formalized.packing_list_1.transport_1.mode_code |
| 03 | NationalityCode | 000 | CO | код “национальности” ТС | operator: formalized.packing_list_1.transport_1.nationality_code |
| 04 | MoverIndicator | true | CO | признак тягач/прицеп | operator: formalized.packing_list_1.transport_1.mover_indicator |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### TransportMeans_2

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | Number | ВТ374974 | CO | регистрационный номер; источник для графы 18: G_18 | operator: formalized.packing_list_1.transport_2.number |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator |
| 03 | NationalityCode | 000 | CO | код “национальности” ТС | operator |
| 04 | MoverIndicator | false | CO | признак тягач/прицеп | operator |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 8 из 4 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 37 из 37
- `doc_formalization_status`: confirmed

### document: CMR / Международная товарно-транспортная накладная (02015)
- `uqi_prefix`: formalized.cmr_1
- `xml_target_root`: AltaE3CMR
- `path`: md\СМР от СВХ.md
- `file_name`: СМР от СВХ.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | LanguageCode | RU | CO | язык документа; для импорта/совместимости; обычно `RU`; константа; derived | operator: formalized.cmr_1.language_code |
| 02 | CMR_Choice | 1 | CO | системный выбор/вариант Альты; для импорта, обычно `1`; константа; derived | operator: formalized.cmr_1.cmr_choice |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR; графа 44: G44/G442 | md: СМР от СВХ.md |
| 04 | RegistrationDocument_DateInf | 2026-01-20 | CD | дата CMR; графа 44: G44/G443 | md |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления; для сверок/контекста | operator: formalized.cmr_1.registration_place |
| 06 | TrakingCargo_TakingCargoDate | 2026-01-20 | CD | дата CMR; **опечатка TrakingCargo**; для сверок/контекста | md |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2; для сверок/контекста | operator: formalized.cmr_1.taking_cargo_country_code_alpha2 |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | Китай | CD | страна принятия груза, текст; **опечатка CounryName** | derived |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2; для сверок/контекста | operator: formalized.cmr_1.delivery_country_code_alpha2 |
| 10 | DeliveryPlace_CounryName | Россия | CD | страна доставки, текст; **опечатка CounryName** | derived |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки по Incoterms, если указано в CMR | operator: formalized.cmr_1.delivery_terms_place |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки, напр. `EXW` | operator: formalized.cmr_1.delivery_terms_string |
| 13 | GoodsQuantity | 127 | CD | общее количество грузовых мест/упаковок по CMR | md |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто по CMR | md |
| 15 | CMRTransport_PrimeMoverStateSignID | О157АО774 | CD | гос. номер тягача | md (0157AO774) нормализация: латиница->кириллица |
| 16 | CMRTransport_TrailerStateSignID | ВТ374974 | CD | гос. номер прицепа | md (BT374974) нормализация: латиница->кириллица |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование; для сверок с инвойсом/контрактом | md |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | краткое наименование; для сверок | operator: formalized.cmr_1.consignor_shortname_equals_full=true |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна alpha-2; для сверок | md |
| 20 | Consignor_Address_CounryName | Китай | CD | страна, текст; **опечатка CounryName**; для сверок | derived |
| 21 | Consignor_Address_Region | Hebei | CD | регион; для сверок | md |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город/район; для сверок | md |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street. | CD | улица/дом одной строкой; для сверок | md |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | наименование гаранта | operator: formalized.cmr_1.consignor_guarantee_all |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое наименование | operator |
| 26 | Consignor_Guarantee_Address_CountryCode |  | CO | страна alpha-2 | operator |
| 27 | Consignor_Guarantee_Address_CounryName |  | CO | страна, текст; **опечатка CounryName** | operator |
| 28 | Consignor_Guarantee_Address_Region |  | CO | регион | operator |
| 29 | Consignor_Guarantee_Address_City |  | CO | город/район | operator |
| 30 | Consignor_Guarantee_Address_StreetHouse |  | CO | улица/дом одной строкой | operator |
| 31 | Consignee_NameInf | ООО "СКИФ" | CD | наименование получателя | master_data |
| 32 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | operator: formalized.cmr_1.consignee_shortname_equals_full=true |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН; **суффикс ID — часть тега Альты** | operator: formalized.cmr_1.consignee_ogrn_from_master_data=true |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН; **суффикс ID — часть тега Альты** | md |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | md |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | md |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | derived |
| 38 | Consignee_Address_CounryName | Россия | CD | страна, текст; **опечатка CounryName** | derived |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | md |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | md |
| 41 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | CD | улица/дом/офис одной строкой | md |

#### CMRGoods_1

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | derived: single line |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CD | описание груза/товара как в CMR | md |
| 03 | PakingQuantity | 127 | CO | кол-во упаковок/мест; **опечатка PakingQuantity** | operator: formalized.cmr_1.goods_1.packing_quantity |

#### Итого, по элементу массива:
- `item_fields`: 3 из 3

#### Итого, по массиву:
- `array_elements`: 1
- `item_fields`: всего полей 3 из 3 * 1
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 42 из 42
- `doc_formalization_status`: confirmed

### document: Payment Order / Заявление на перевод (04023)
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: md\currency_transfer_1_13.01.2026.md
- `file_name`: currency_transfer_1_13.01.2026.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | 04023 — код вида документа для графы 44: G44/G441; константа; derived | operator: formalized.payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа в структуре Альты; derived | operator: formalized.payment_order_all.payment_mode_code |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | md |
| 04 | TransactionKind | 01 | CO | вид операции/код; derived | operator |
| 05 | Priority | 5 | CO | очередность | operator |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025. INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | md |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | md |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения; графа 44 | md |
| 09 | DocumentReference_PrDocumentDate | 2026-01-13 | CD | дата платежного поручения; графа 44 | md |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик: наименование | md |
| 11 | Payer_INN | 1650389298 | CD | плательщик: ИНН | md |
| 12 | Payer_KPP | 165001001 | CO | плательщик: КПП | operator: formalized.payment_order_1.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (форма заявления на перевод) | CD | плательщик: банк / реквизиты | md |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель: наименование / реквизиты | md |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; /CN767290000018 | CD | реквизиты банка получателя | md |
| 16 | PersonSurname | Саранов | CO | фамилия | operator: formalized.payment_order_all.payer_sign.surname |
| 17 | PersonName | Дмитрий | CO | имя | operator: formalized.payment_order_all.payer_sign.name |

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### document: Payment Order / Заявление на перевод (04023)
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `path`: md\currency_transfer_7_28.11.2025.md
- `file_name`: currency_transfer_7_28.11.2025.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CO | 04023 — код вида документа для графы 44: G44/G441; константа; derived | operator |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа; derived | operator |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | md |
| 04 | TransactionKind | 01 | CO | вид операции/код; derived | operator |
| 05 | Priority | 5 | CO | очередность | operator |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025. INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | md |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | md |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | md |
| 09 | DocumentReference_PrDocumentDate | 2025-11-28 | CD | дата платежного поручения | md |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | md |
| 11 | Payer_INN | 1650389298 | CD | плательщик: ИНН | md |
| 12 | Payer_KPP | 165001001 | CO | плательщик: КПП | operator: formalized.payment_order_2.payer_kpp |
| 13 | Payer_Bank_BankName | ВТБ (форма заявления на перевод) | CD | плательщик: банк / реквизиты | md |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель | md |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; /CN767290000018 | CD | реквизиты банка получателя | md |
| 16 | PersonSurname | Саранов | CO | фамилия | operator |
| 17 | PersonName | Дмитрий | CO | имя | operator |

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### document: Service Invoice / Счет за перевозку (04031)
- `uqi_prefix`: formalized.service_invoice_1
- `xml_target_root`: AltaServiceInvoice
- `path`: md\Счет_№26-00378-tl_от_27-01-2026.md
- `file_name`: Счет_№26-00378-tl_от_27-01-2026.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentSign | 1 | CO | системный признак документа Альты; derived | operator: formalized.service_invoice_1.document_sign |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | md |
| 03 | Currency | USD | CD | валюта итого ISO 4217 alpha-3 | md |
| 04 | ServiceProvider_Name | ООО "Трансимпериал" | CD | исполнитель услуг/перевозчик: наименование | md |
| 05 | BankName | АО "Райффайзенбанк", БИК 044525700, Сч. № 30101810200000000700, Сч. № 40702810400000233463 | CD | банк исполнителя | md |
| 06 | ContractDetails_PrDocumentNumber | KOOO/26651/M | CD | № договора на услуги/перевозку | md |
| 07 | ContractDetails_PrDocumentDate | 2025-05-13 | CD | дата договора на услуги/перевозку | md |
| 08 | PrDocumentNumber | ОТСУТСТВУЕТ | CO | связанный документ/заказ: номер | operator: formalized.service_invoice_1.payment_document_number |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CO | связанный документ/заказ: дата | operator: formalized.service_invoice_1.payment_document_date |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | md |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | md |
| 12 | Registration_PrDocumentDate | 2026-01-27 | CD | дата счета | md |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | грузоотправитель | operator: formalized.service_invoice_1.consignor_decision=seller |
| 14 | PostalCode |  | CO | индекс | operator: service_invoice_1.consignor_address_from_seller=true (PostalCode оставлять пустым если отсутствует) |
| 15 | CountryCode | CN | CD | страна alpha-2 | copied_from: formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 16 | CounryName | Китай | CD | страна, текст; **опечатка CounryName** | derived |
| 17 | Region | Hebei | CD | регион | copied_from: formalized.invoice_1.Seler_PostalAddress_Region |
| 18 | Town | Shijiazhuang | CD | город/район | copied_from: formalized.invoice_1.Seler_PostalAddress_City |
| 19 | StreetHouse | No. 5 Gaodong street. | CD | улица/дом одной строкой | copied_from: formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 20 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | md |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator: formalized.service_invoice_1.consignee_ogrn_from_master_data=true |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | md |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | md |
| 24 | PostalCode | 423800 | CD | индекс | md |
| 25 | CountryCode | RU | CD | страна alpha-2 | md |
| 26 | CounryName | Россия | CD | страна, текст; **опечатка CounryName** | md |
| 27 | Region | Республика Татарстан | CD | регион | md |
| 28 | Town | Набережные Челны | CD | город | md |
| 29 | StreetHouse | проезд Хлебный | CD | улица | md |
| 30 | House | 30 | CO | дом | operator: formalized.service_invoice_1.consignee_house |
| 31 | Room | 211 | CO | офис/кв | operator: formalized.service_invoice_1.consignee_room |
| 32 | Signature_Choice | 1 | CO | вариант подписи | operator: formalized.service_invoice_1.signature_choice |
| 33 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CO | фамилия руководителя | operator: formalized.service_invoice_1.signatures_confirmed=true |
| 34 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CO | инициалы/имя руководителя | operator |
| 35 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CO | фамилия бухгалтера | operator |
| 36 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CO | инициалы/имя бухгалтера | operator |

#### ServiceDescription_1

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №KOOO/26651/M от 13-05-2025 по транспортному заказу №26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск). Перевозка автотранспортом | CD | многострочное описание услуги | md |
| 02 | CurrencyCode | USD | CD | валюта строки ISO alpha-3 | md |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator: formalized.service_invoice_1.service_1.service_name |
| 04 | TaxRate | 0% | CD | ставка налога | md |
| 05 | TaxSum | 0.00 | CD | сумма налога | md |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | md |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | md |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### ServiceDescription_2

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | md |
| 02 | CurrencyCode | USD | CD | валюта строки ISO alpha-3 | md |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator: formalized.service_invoice_1.service_2.service_name |
| 04 | TaxRate | 0% | CD | ставка налога | md |
| 05 | TaxSum | 0.00 | CD | сумма налога | md |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | md |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | md |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 14 из 7 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 37 из 37
- `doc_formalization_status`: confirmed

### document: Insurance Document / Счет за страховку (04111)
- `uqi_prefix`: formalized.insurance_document_1
- `xml_target_root`: AltaFreeDoc
- `path`: md\Счет_№26-00378-tl_1_от_14-01-2026.md
- `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | 04111 — код вида документа для графы 44: G44/G441; константа; derived | derived: constant |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа; графа 44: G44/G444 | md |
| 03 | DocumentHead_DocumentDate | 2026-01-14 | CD | дата документа; графа 44: G44/G443 | md |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа; графа 44: G44/G442 | md |
| 05 | TextPara | link:md\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | основной текст/условия | operator: formalized.insurance_document_1.textpara_storage=link |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

### document: TechDescription / Техническое описание (05999)
- `uqi_prefix`: formalized.tech_description_1
- `xml_target_root`: AltaFreeDoc
- `path`: md\техничка Антикот, антипыльца антимошка .md
- `file_name`: техничка Антикот, антипыльца антимошка .md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | 05999 — код вида документа для графы 44: G44/G44; константа; derived | derived: constant |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование техописания; графа 44: G44/G444 | md |
| 03 | DocumentHead_DocumentDate | 2025-10-30 | CO | дата техописания; графа 44: G44/G443 | operator: formalized.tech_description_1.date |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания; графа 44: G44/G442 | operator: formalized.tech_description_1.number |
| 05 | TextPara | link:md\техничка Антикот, антипыльца антимошка .md | CD | технический текст без пересказа | link to md |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

## non_formalized

### document: Storage Report / ДО-1
- `uqi_prefix`: non_formalized.svh_1
- `path`: md\ДО 14431420260204161621.md
- `file_name`: ДО 14431420260204161621.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | warehouse_license_number | 1040414210/10092/Б | CD | номер лицензии/свидетельства СВХ; цель: графа 30: G_30_1 | md |
| 02 | warehouse_license_date | 2025-09-18 | CD | дата лицензии/свидетельства СВХ; цель: графа 30 | md |
| 03 | actual_gross_weight | 3500.00 | CO | фактический вес по весам; цель: сверка графы 35 | operator: non_formalized.svh_1.actual_totals_from_svh_additional_sheet=true (взято из ДО доп) |
| 04 | actual_places | 127 | CO | фактическое количество мест; цель: графа 6 | operator: non_formalized.svh_1.actual_totals_from_svh_additional_sheet=true |
| 05 | transport_reg_number | О157АО774 | CD | номер ТС при въезде/по отчету СВХ | md (0157AO774) нормализация |

#### goods_1

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | tnved | 7019900095 | CD | код товара; цель: сопоставление с товарными блоками ДТ | md |
| 02 | places | 27 | CD | кол-во мест по строке | md |
| 03 | gross_weight_kg | 1710.00 | CD | вес брутто по строке (кг) | md |
| 04 | cost | 42228.00 | CD | стоимость по строке | md |
| 05 | currency_code | CNY | CD | валюта по строке | md |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### goods_2

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара | md |
| 02 | places | 100 | CD | кол-во мест по строке | md |
| 03 | gross_weight_kg | 1790.00 | CD | вес брутто по строке (кг) | md |
| 04 | cost | 55032.00 | CD | стоимость по строке | md |
| 05 | currency_code | CNY | CD | валюта по строке | md |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 10 из 5 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 6 из 6

### document: Storage Report Additional Sheet / Добавочный лист к ДО
- `uqi_prefix`: non_formalized.svh_additional_sheet_1
- `path`: md\ДО доп 14431520260204161645.md
- `file_name`: ДО доп 14431520260204161645.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | number | 2 | CD | № доп.листа/приложения; цель: G_30P_1; | md: ДО доп 14431520260204161645.md |
| 02 | date | 2026-02-03 | CD | дата доп.листа; цель: G_30P_1 | md: ДО доп 14431520260204161645.md |
| 03 | actual_gross_weight | 3500.00 | CD | фактический вес брутто по весам | md |
| 04 | actual_places | 127 | CD | фактическое количество мест | md |
| 05 | transport_reg_number |  | pending | номер ТС при въезде | отсутствует в md |
| 06 | svh_address_region | Республика Татарстан | CO | регион СВХ | operator: non_formalized.svh_additional_sheet_1.address_from_cmr=true |
| 07 | svh_address_city | Набережные Челны | CO | город/нас.пункт СВХ | operator |
| 08 | svh_address_street_house | Производственный пр-д, д.45, СВХ ООО «ЛОГИКАМ» | CO | улица/дом СВХ одной строкой | operator: non_formalized.svh_additional_sheet_1.address_from_cmr=true (CMR п.3) |
| 09 | svh_customs_code | 10404083 | CO | код таможенного органа в зоне СВХ | operator: non_formalized.svh_additional_sheet_1.address_from_cmr=true (CMR п.13) |

#### Итого, по документу:
- `doc_fields`: 9 из 9

### document: Transit Declaration / Транзитная декларация (09013)
- `uqi_prefix`: non_formalized.td_1
- `path`: md\ТД 10719110_240126_5011363_reg00378тд.md
- `file_name`: ТД 10719110_240126_5011363_reg00378тд.md

| num | field | value | status | description | note |
|---:|---|---|---|---|---|
| 01 | number | 10719110/240126/5011363 | CO | номер ТД; цель: G44/G442 | operator: non_formalized.transit_declaration_1.number |
| 02 | date | 2026-01-24 | CO | дата ТД; цель: G44/G443 | operator: non_formalized.transit_declaration_1.date |
| 03 | customs_post_code | 10719110 | CD | код таможенного органа; цель: графа 29 | md |
| 04 | customs_post_name | п/п МАПП Забайкальск | CD | наименование таможенного органа; цель: графа 29 | md |
| 05 | transport_reg_number | О157АО774/ВТ374974 | CD | ТС по ТД; цель: сверка графы 18 | md normalization |

#### Итого, по документу:
- `doc_fields`: 5 из 5

## Нерешенные вопросы (Issues)

**Для полей:**
- `formalized.contract_1.ForeignPerson_Address_StreetHouse` / `ForeignPerson_Address_City` (OCR mid)
  - `question`: Уточнить при необходимости (этап 2.0/поля адреса): подтверждаешь, что адрес продавца = "No.5 Gaodong Street, Xinhua District, Shijiazhuang, China" и город Shijiazhuang? (источник: md\SALES CONTRACT NoLM-2553.md, Page 4, confidence mid)
- `non_formalized.svh_additional_sheet_1.transport_reg_number`
  - `question`: В добавочном листе ДО №2 нет номера ТС при въезде. Подтвердить, что использовать тягач "О157АО774" как номер ТС для этого поля? (источники: md\ДО доп 144315...; md\ДО 144314...; md\СМР от СВХ.md)

### Итогo, по файлу:

`total_doc_fields` - 25+31+52+37+42+17+17+37+5+5+6+9+5 = 288
`total_fields` - включает массивы: invoice_goods 105, packing_goods 35, packing_transport 8, cmr_goods 3, svh_goods 10, service_desc 14.
`formalization_status` - confirmed (все формализуемые документы заполнены без pending)
