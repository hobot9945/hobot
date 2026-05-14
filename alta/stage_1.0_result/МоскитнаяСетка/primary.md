## meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 2 товара (2 кода ТН ВЭД)
- `источники данных:` md + operator_provided_data + stable_source (xml)

## formalized

### `document`: Contract
  - `uqi_prefix`: formalized.contract_1
  - `xml_target_root`: AltaE2CONT
  - `path`: md\SALES CONTRACT NoLM-2553.md
  - `file_name`: SALES CONTRACT NoLM-2553.md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 03011 | CD | код вида документа | derived: константа по схеме |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта | source: md\SALES CONTRACT NoLM-2553.md (стр. 9–10) |
| 03 | ContractRegistration_PrDocumentDate | 02.07.2025 | CD | дата контракта | source: md\SALES CONTRACT NoLM-2553.md (стр. 10; русск. блок стр. 68) |
| 04 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта | operator: decisions_from_chat 2026-05-08; в исходном контракте 41904.30 RMB, но допсогл. меняет сумму |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator: formalized.contract_1.currency_code_numeric |
| 06 | ContractTerms_LastDate | 31.12.2026 | CD | срок действия/исполнения | source: md\SALES CONTRACT NoLM-2553.md (стр. 49–53) |
| 07 | ContractTerms_OtherTerms | EXW HEBEI | CO | условия поставки / Incoterms | operator: formalized.contract_1.delivery_terms |
| 08 | ContractTerms_ContractText | link:md\SALES CONTRACT NoLM-2553.md | CD | текст контракта | link вместо полного текста |
| 09 | ContractTerms_DealSign | 1 | CO | системный признак Альты | operator: formalized.contract_1.deal_sign |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта | source: md\SALES CONTRACT NoLM-2553.md (стр. 20–22) |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 | operator: formalized.contract_1.foreign_person_country_code_alpha2 |
| 12 | ForeignPerson_Address_CounryName | Китай | CD | страна продавца, текст | derived: по alpha-2 CN (cb:country) |
| 13 | ForeignPerson_Address_Region | Hebei | CD | регион/область продавца | source: md\SALES CONTRACT NoLM-2553.md (стр. 403–405) |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца | source: md\SALES CONTRACT NoLM-2553.md (стр. 403–405) |
| 15 | ForeignPerson_Address_StreetHouse | No. 5 Gaodong Street, Xinhua District | CD | улица/дом продавца одной строкой | source: md\SALES CONTRACT NoLM-2553.md (стр. 403–405) |
| 16 | RussianPerson_OrganizationName | ООО "СКИФ" | CD | покупатель/сторона контракта | source: md\SALES CONTRACT NoLM-2553.md (стр. 81–82) |
| 17 | RussianPerson_OGRN | 1201600020390 | CD | ОГРН покупателя | master_data: FreeDoc_ЮЭ9965-25-106893283.xml (ЕГРЮЛ) |
| 18 | RussianPerson_INN | 1650389298 | CD | ИНН покупателя | source: CMR (md\СМР от СВХ.md, стр. 17–21) |
| 19 | RussianPerson_KPP | 165001001 | CD | КПП покупателя | source: CMR (md\СМР от СВХ.md, стр. 20) |
| 20 | RussianPerson_Address_PostalCode | 423800 | CD | индекс покупателя | source: invoice/pl/contract (423800) |
| 21 | RussianPerson_Address_CountryCode | RU | CD | страна покупателя alpha-2 | derived: Россия |
| 22 | RussianPerson_Address_CounryName | Россия | CD | страна покупателя, текст | derived |
| 23 | RussianPerson_Address_Region | Республика Татарстан | CD | регион покупателя | source: invoice/pl/cmr |
| 24 | RussianPerson_Address_City | Набережные Челны | CD | город покупателя | source: invoice/pl/cmr |
| 25 | RussianPerson_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом/офис одной строкой | source: invoice/pl/cmr |
| 26 | doc_code | 03011 | CD | код документа | derived |
| 27 | doc_name | КОНТРАКТ | CD | наименование документа | derived |
| 28 | doc_number | LM-2553 | CD | номер документа | derived: =ContractRegistration_PrDocumentNumber |
| 29 | doc_date | 02.07.2025 | CD | дата документа | derived: =ContractRegistration_PrDocumentDate |

#### Итого, по документу:
- `doc_fields`: 29 из 29
- `doc_formalization_status`: confirmed


### `document`: Supplementary Contract
  - `uqi_prefix`: formalized.supplementary_contract_1
  - `xml_target_root`: AltaSupplementaryContract
  - `path`: md\1 Supplementary agreement to the contract.md
  - `file_name`: 1 Supplementary agreement to the contract.md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения | source: md\1 Supplementary agreement to the contract.md (стр. 9–10) |
| 02 | IssueDate | 25.11.2025 | CD | дата доп. соглашения | source: md\1 Supplementary agreement to the contract.md (стр. 13) |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта | source: md\1 Supplementary agreement to the contract.md (стр. 31–33) |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator: formalized.supplementary_contract_1.currency_code_numeric |
| 05 | ContractDescription_LastDate | 31.12.2026 | CO | новый срок действия/исполнения | operator: formalized.supplementary_contract_1.expiry_date |
| 06 | ContractDescription_ContractText | link:md\1 Supplementary agreement to the contract.md | CD | текст доп. соглашения | link вместо полного текста |
| 07 | ContractDescription_DealSign | 1 | CO | системный признак Альты | operator |
| 08 | ContractDescription_StockCategorySign | 0 | CO | системный признак Альты | operator |
| 09 | ContractDescription_BuyerLimitationSign | 0 | CO | системный признак Альты | operator |
| 10 | ContractDescription_InsuranceSign | 0 | CO | системный признак Альты | operator |
| 11 | RussianPerson_OrganizationName | ООО "СКИФ" | CD | российская сторона | copied_from: formalized.contract_1.RussianPerson_OrganizationName |
| 12 | RussianPerson_ShortName | ООО "СКИФ" | CD | краткое наименование | derived: short=full |
| 13 | RussianPerson_OGRN | 1201600020390 | CD | ОГРН | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 14 | RussianPerson_INN | 1650389298 | CD | ИНН | source: CMR |
| 15 | RussianPerson_KPP | 165001001 | CD | КПП | source: CMR |
| 16 | RussianPerson_Address_PostalCode | 423800 | CD | индекс | copied_from: formalized.contract_1.RussianPerson_Address_PostalCode |
| 17 | RussianPerson_Address_CountryCode | RU | CD | страна alpha-2 | derived |
| 18 | RussianPerson_Address_CounryName | Россия | CD | страна, текст | derived |
| 19 | RussianPerson_Address_Region | Республика Татарстан | CD | регион | copied_from: formalized.contract_1.RussianPerson_Address_Region |
| 20 | RussianPerson_Address_City | Набережные Челны | CD | город | copied_from: formalized.contract_1.RussianPerson_Address_City |
| 21 | RussianPerson_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом одной строкой | copied_from: formalized.contract_1.RussianPerson_Address_StreetHouse |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | иностранная сторона | source: md\1 Supplementary agreement... (стр. 16–19) |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator: foreign_person_short_name_equals_full |
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 | operator |
| 25 | ForeignPerson_Address_CounryName | Китай | CD | страна, текст | derived |
| 26 | ForeignPerson_Address_Region | Hebei | CO | регион | operator: разрешено взять из контракта |
| 27 | ForeignPerson_Address_City | Shijiazhuang | CO | город/район | operator: разрешено взять из контракта |
| 28 | ForeignPerson_Address_StreetHouse | No. 5 Gaodong Street, Xinhua District | CO | улица/дом | operator: разрешено взять из контракта |
| 29 | PersonSurname | Li | CO | фамилия подписанта | operator |
| 30 | PersonName | Jing | CO | имя подписанта | operator |
| 31 | PersonMiddleName |  | CO | отчество подписанта | operator: пусто |
| 32 | doc_code | 03012 | CD | код документа | derived |
| 33 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наименование документа | derived |
| 34 | doc_number | 1 | CD | номер документа | derived: =DocumentNumber |
| 35 | doc_date | 25.11.2025 | CD | дата документа | derived: =IssueDate |

#### Итого, по документу:
- `doc_fields`: 35 из 35
- `doc_formalization_status`: confirmed


### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: md\CL на сетку .md
  - `file_name`: CL на сетку .md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты | operator: formalized.invoice_1.exchange_rate |
| 02 | CurrencyCode | CNY | CO | валюта инвойса ISO alpha-3 | operator |
| 03 | DocumentCode | 04021 | CD | код вида документа | derived |
| 04 | PlacesQuantity | 127 | CD | кол-во грузовых мест по инвойсу | source: md\CL на сетку .md (стр. 53) |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто по инвойсу | operator: взято из PL totals |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто по инвойсу | operator: взято из PL totals |
| 08 | GCost | 97260.00 | CO | системное поле Альты | operator: gcost=TotalCost |
| 09 | TotalCost | 97260.00 | CO | итого по инвойсу | operator (и source: md\CL..., стр.51) |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | source: md\CL..., стр.54 |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | operator |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | operator |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | operator |
| 16 | Registration_PrDocumentName | Commercial invoice | CD | наименование документа для печати | source: md\CL..., заголовок |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса | source: md\CL..., стр.14 |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса | source: md\CL..., стр.15 |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки | source: md\CL..., стр.17 |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки | source: md\CL..., стр.18 |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | source: CMR |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | source: CMR |
| 23 | Buyer_Name | ООО "СКИФ" | CD | наименование покупателя | source: md\CL..., стр.21 |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | source: md\CL..., стр.22 |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | derived |
| 26 | Buyer_PostalAddress_CounryName | Россия | CD | страна покупателя, текст | derived |
| 27 | Buyer_PostalAddress_Region | Республика Татарстан | CD | регион | source: md\CL..., стр.22 |
| 28 | Buyer_PostalAddress_City | Naberezhnye Chelny | CD | город | source: md\CL..., стр.22 |
| 29 | Buyer_PostalAddress_StreetHouse | Khlebny Passage, hause 30, office 211 | CD | улица/дом/офис | source: md\CL..., стр.22–23 |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | продавец | source: md\CL..., стр.9 |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | operator |
| 32 | Seler_PostalAddress_CounryName | Китай | CD | страна продавца, текст | derived |
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | source: md\CL..., стр.10 |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город продавца | source: md\CL..., стр.10 |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | source: md\CL..., стр.10 |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | derived: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | derived: consignor=seller |
| 38 | Consignor_Address_CounryName | Китай | CD | страна грузоотправителя, текст | derived |
| 39 | Consignor_Address_Region | Hebei | CD | регион | derived |
| 40 | Consignor_Address_City | Shijiazhuang | CD | город/район | derived |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | derived |
| 42 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | operator: consignee_equals_buyer |
| 43 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data: FreeDoc_ЮЭ9965-25-106893283.xml |
| 44 | Consignee_INN | 1650389298 | CD | ИНН | source: CMR |
| 45 | Consignee_KPP | 165001001 | CD | КПП | source: CMR |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | source: md\CL... |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | derived |
| 48 | Consignee_Address_CounryName | Россия | CD | страна, текст | derived |
| 49 | Consignee_Address_Region | Республика Татарстан | CD | регион | source: md\CL... |
| 50 | Consignee_Address_City | Naberezhnye Chelny | CD | город | source: md\CL... |
| 51 | Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | CD | улица/дом/офис | source: md\CL... |
| 52 | InvoiceGoods_[n] |  | CD | товарные позиции | 7 строк по инвойсу |

#### InvoiceGoods_1
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | source: md\CL..., строка 1 |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30; Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара | source: md\CL..., стр.30–31 |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке | source: md\CL..., стр.31 |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм | source: md\CL..., стр.31 |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | derived: Quantity in M2 |
| 06 | MeasureUnitQualifierName | м² (квадратный метр) | CD | единица измерения доп.количества | derived: cb:unit 055 |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator: invoice_1.goods_1.gross_weight |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator |
| 09 | Price | 5.85 | CD | цена за единицу | source: md\CL..., стр.31 |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | source: md\CL..., стр.31 |
| 11 | OriginCountryCode | 156 | CO | код страны происхождения (numeric) | operator: goods_all.origin_country_code_numeric |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator: goods_all.manufacturer |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_2
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | source: md\CL..., строка 2 |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 *30 / Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара | source: md\CL..., стр.32–33 |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке | source |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в M2 | source |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм | derived |
| 06 | MeasureUnitQualifierName | м² (квадратный метр) | CD | единица измерения | derived |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто | operator |
| 08 | NetWeightQuantity | 460.80 | CO | нетто | operator |
| 09 | Price | 5.85 | CD | цена | source |
| 10 | TotalCost | 8424.00 | CD | стоимость | source |
| 11 | OriginCountryCode | 156 | CO | страна происхождения numeric | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак/маркировка | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_3
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | source |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 / Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 | CD | описание товара | source: md\CL..., стр.39–40 |
| 03 | GoodsQuantity | 60 | CD | кол-во | source |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в M2 | source |
| 05 | goods_supplementary_uom_name | м² | CD | наименование | derived |
| 06 | MeasureUnitQualifierName | м² (квадратный метр) | CD | единица | derived |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто | operator |
| 08 | NetWeightQuantity | 252.00 | CO | нетто | operator |
| 09 | Price | 6.35 | CD | цена | source |
| 10 | TotalCost | 16002.00 | CD | стоимость | source |
| 11 | OriginCountryCode | 156 | CO | происхождение numeric | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_4
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код | source |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2 / Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 | CD | описание | source: md\CL..., стр.41–42 |
| 03 | GoodsQuantity | 30 | CD | кол-во | source |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в M2 | source |
| 05 | goods_supplementary_uom_name | м² | CD | наименование | derived |
| 06 | MeasureUnitQualifierName | м² (квадратный метр) | CD | единица | derived |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто | operator |
| 08 | NetWeightQuantity | 144.00 | CO | нетто | operator |
| 09 | Price | 6.35 | CD | цена | source |
| 10 | TotalCost | 9144.00 | CD | стоимость | source |
| 11 | OriginCountryCode | 156 | CO | происхождение | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_5
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7019900095 | CD | код | source: md\CL..., стр.44 |
| 02 | GoodsDescription | MIDGE MENS Material: Fiberglass. Roll size: 1,4*30 M2 / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 | CD | описание | source |
| 03 | GoodsQuantity | 90 | CD | кол-во | source |
| 04 | goods_supplementary_quantity | 3780 | CD | количество в M2 | source |
| 05 | goods_supplementary_uom_name | м² | CD | наименование | derived |
| 06 | MeasureUnitQualifierName | м² (квадратный метр) | CD | единица | derived |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто | operator |
| 08 | NetWeightQuantity | 491.40 | CO | нетто | operator |
| 09 | Price | 3.4 | CD | цена | source |
| 10 | TotalCost | 12852.00 | CD | стоимость | source |
| 11 | OriginCountryCode | 156 | CO | происхождение | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_6
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7019900095 | CD | код | source |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 | CD | описание | source |
| 03 | GoodsQuantity | 180 | CD | кол-во | source |
| 04 | goods_supplementary_quantity | 8640 | CD | количество в M2 | source |
| 05 | goods_supplementary_uom_name | м² | CD | наименование | derived |
| 06 | MeasureUnitQualifierName | м² (квадратный метр) | CD | единица | derived |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто | operator |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто | operator |
| 09 | Price | 3.4 | CD | цена | source |
| 10 | TotalCost | 29376.00 | CD | стоимость | source |
| 11 | OriginCountryCode | 156 | CO | происхождение | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_7
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код | source: md\CL..., стр.48–49 |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 / Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CD | описание | source |
| 03 | GoodsQuantity | 5 | CD | кол-во | source |
| 04 | goods_supplementary_quantity | 240 | CD | количество в M2 | source |
| 05 | goods_supplementary_uom_name | м² | CD | наименование | derived |
| 06 | MeasureUnitQualifierName | м² (квадратный метр) | CD | единица | derived |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто | operator |
| 08 | NetWeightQuantity | 24.00 | CO | нетто | operator |
| 09 | Price | 28 | CD | цена | source |
| 10 | TotalCost | 6720.00 | CD | стоимость | source |
| 11 | OriginCountryCode | 156 | CO | происхождение | operator |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | ТМ | operator |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | знак | operator |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель | operator |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 105 из 15 * 7
- `array_status`: confirmed

| 53 | doc_code | 04021 | CD | код документа | derived |
| 54 | doc_name | ИНВОЙС | CD | наименование документа | derived |
| 55 | doc_number | LM-2591 | CD | номер документа | derived |
| 56 | doc_date | 30.10.2025 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 56 из 56
- `doc_formalization_status`: confirmed


### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list_1
  - `xml_target_root`: AltaE2PACK
  - `path`: md\PL на сетку .md
  - `file_name`: PL на сетку .md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто | source: md\PL..., totals |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто | source: md\PL..., totals |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CD | грузоотправитель | source: md\PL..., стр.9 |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | краткое наименование | operator: consignor_shortname_equals_full |
| 05 | Consignor_Address_CountryCode | CN | CO | страна | operator |
| 06 | Consignor_Address_CounryName | Китай | CD | страна, текст | derived |
| 07 | Consignor_Address_Region | Hebei | CD | регион | source: md\PL..., стр.10 |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город | source |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | source |
| 10 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | source: md\PL..., стр.21 |
| 11 | Consignee_ShortName | ООО "СКИФ" | CO | краткое наименование | operator |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data: ЕГРЮЛ |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | source: CMR |
| 14 | Consignee_KPP | 165001001 | CD | КПП | source: CMR |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | source: md\PL..., стр.22 |
| 16 | Consignee_Address_CountryCode | RU | CD | страна | derived |
| 17 | Consignee_Address_CounryName | Россия | CD | страна, текст | derived |
| 18 | Consignee_Address_Region | Республика Татарстан | CD | регион | source |
| 19 | Consignee_Address_City | Naberezhnye Chelny | CD | город | source |
| 20 | Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | CD | улица/дом/офис | source |
| 21 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CD | место поставки | source: md\PL..., стр.26 |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | внутренний числовой код условий | operator |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | source: md\PL..., стр.49 |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CO | наименование контракта | operator: decisions_from_chat 2026-05-08 |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | source |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | source |
| 27 | DeliveryTerms_Invoice_PrDocumentName | Packing list / Упаковочный лист | CD | наименование инвойса для печати | source: заголовок PL |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | source: md\PL..., стр.14 |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | source |
| 30 | DeliveryTerms_Registration_PrDocumentName | Упаковочный лист | CO | наименование упаковочного | operator |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | operator |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | operator |
| 33 | Goods_[n] |  | CD | товарные/грузовые строки | 7 строк |

#### Goods_1
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки | source: md\PL..., стр.30 |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | source |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто | source |
| 04 | NetWeightQuantity | 806.60 | CD | нетто | source |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок/мест | operator: =GoodsQuantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_2
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки | source: md\PL..., стр.31 |
| 02 | GoodsQuantity | 30 | CD | количество | source |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто | source |
| 04 | NetWeightQuantity | 460.80 | CD | нетто | source |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок/мест | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_3
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | ANTI-POLLEN MESH / Сетка против пыльцы "Антипыльца" ... Размер рулона 1,42*0,64*0,22 | CD | описание | source: md\PL..., стр.32 |
| 02 | GoodsQuantity | 6 | CD | количество | source: md\PL..., стр.32 (Qty BG) |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто | source |
| 04 | NetWeightQuantity | 252.00 | CD | нетто | source |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок/мест | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_4
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | ANTI-POLLEN MESH / Сетка против пыльцы "Антипыльца" ... Размер рулона 1,62*0,64*0,23 | CD | описание | source: md\PL..., стр.38 |
| 02 | GoodsQuantity | 3 | CD | количество | source |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто | source |
| 04 | NetWeightQuantity | 144.00 | CD | нетто | source |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок/мест | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_5
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | MIDGE MEHS / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" ... Размер рулона 1,42*0,55*0,18 | CD | описание | source: md\PL..., стр.39 |
| 02 | GoodsQuantity | 9 | CD | количество | source |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто | source |
| 04 | NetWeightQuantity | 491.40 | CD | нетто | source |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок/мест | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_6
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | MIDGE MEHS / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" ... Размер рулона 1,62*0,55*18 | CD | описание | source: md\PL..., стр.40 |
| 02 | GoodsQuantity | 18 | CD | количество | source |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто | source |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто | source |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок/мест | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_7
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | GRID WITH 3 LAYER / Трехслойные сетки "Антипыльца" ... | CD | описание | source: md\PL..., стр.41 |
| 02 | GoodsQuantity | 1 | CD | количество | source |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто | source |
| 04 | NetWeightQuantity | 24.00 | CD | нетто | source |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок/мест | operator |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 35 из 5 * 7
- `array_status`: confirmed

| 34 | TransportMeans_[n] |  | CD | транспорт | 2 элемента |

#### TransportMeans_1
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | Number | О157АО774 | CO | регистрационный номер | operator |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator |
| 03 | NationalityCode | 000 | CO | код национальности ТС | operator |
| 04 | MoverIndicator | true | CO | тягач/прицеп | operator |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### TransportMeans_2
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | Number | ВТ374974 | CO | регистрационный номер | operator |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator |
| 03 | NationalityCode | 000 | CO | код национальности ТС | operator |
| 04 | MoverIndicator | false | CO | тягач/прицеп | operator |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 8 из 4 * 2
- `array_status`: confirmed

| 35 | doc_code | 04131 | CD | код документа | derived |
| 36 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | derived |
| 37 | doc_number | LM-2591 | CD | номер документа | derived |
| 38 | doc_date | 30.10.2025 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 38 из 38
- `doc_formalization_status`: confirmed


### `document`: CMR
  - `uqi_prefix`: formalized.cmr_1
  - `xml_target_root`: AltaE3CMR
  - `path`: md\СМР от СВХ.md
  - `file_name`: СМР от СВХ.md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | LanguageCode | RU | CO | язык документа | operator |
| 02 | CMR_Choice | 1 | CO | системный выбор/вариант Альты | operator |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR | source: md\СМР..., стр.10 |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR | source: md\СМР..., стр.30 |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления | operator |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CD | дата принятия груза | source |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2 | operator |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | Китай | CD | страна принятия груза, текст | derived |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | operator |
| 10 | DeliveryPlace_CounryName | Россия | CD | страна доставки, текст | derived |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки по Incoterms | operator |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator |
| 13 | GoodsQuantity | 127 | CD | общее количество мест | source: md\СМР..., стр.39–41 |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто | source: md\СМР..., стр.42–44 |
| 15 | CMRTransport_PrimeMoverStateSignID | 0157AO774 | CD | гос. номер тягача | source: md\СМР..., стр.63–64 |
| 16 | CMRTransport_TrailerStateSignID | BT374974 | CD | гос. номер прицепа | source |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование отправителя | source: md\СМР..., стр.13 |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна | derived |
| 20 | Consignor_Address_CounryName | Китай | CD | страна, текст | derived |
| 21 | Consignor_Address_Region | Hebei | CD | регион | copied_from: Invoice seller |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город | copied_from |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом | copied_from |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | наименование гаранта | operator |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое наименование | operator |
| 26 | Consignor_Guarantee_Address_CountryCode | ОТСУТСТВУЕТ | CO | страна | operator |
| 27 | Consignor_Guarantee_Address_CounryName | ОТСУТСТВУЕТ | CO | страна, текст | operator |
| 28 | Consignor_Guarantee_Address_Region | ОТСУТСТВУЕТ | CO | регион | operator |
| 29 | Consignor_Guarantee_Address_City | ОТСУТСТВУЕТ | CO | город | operator |
| 30 | Consignor_Guarantee_Address_StreetHouse | ОТСУТСТВУЕТ | CO | улица/дом | operator |
| 31 | Consignee_NameInf | ООО «Скиф» | CD | наименование получателя | source: md\СМР..., стр.17 |
| 32 | Consignee_ShortName | ООО «Скиф» | CO | краткое наименование | operator |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | operator: разрешено подставить из master_data |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | source: md\СМР..., стр.20 |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | source: md\СМР..., стр.20 |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | source |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна | derived |
| 38 | Consignee_Address_CounryName | Россия | CD | страна, текст | derived |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | source |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | source |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом/офис | source |
| 42 | CMRGoods_[n] |  | CD | грузовые строки | 1 строка (в документе нет детализации) |

#### CMRGoods_1
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | note: авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CD | описание груза/товара | source: md\СМР..., стр.36–38 |
| 03 | PakingQuantity | 127 | CO | кол-во упаковок/мест | operator: goods_1.packing_quantity |

#### Итого, по элементу массива:
- `item_fields`: 3 из 3

#### Итого, по массиву:
- `array_elements`: 1
- `item_fields`: всего полей 3 из 3 * 1
- `array_status`: confirmed

| 43 | doc_code | 02015 | CD | код документа | derived |
| 44 | doc_name | CMR | CD | наименование документа | derived |
| 45 | doc_number | 00378 | CD | номер документа | derived |
| 46 | doc_date | 20.01.2026 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 46 из 46
- `doc_formalization_status`: confirmed


### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: md\currency_transfer_7_28.11.2025.md
  - `file_name`: currency_transfer_7_28.11.2025.md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator: payment_order_all.document_code |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | source: md\currency_transfer_7_28.11.2025.md (стр. 15–17) |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator |
| 05 | Priority | 5 | CO | очередность | operator |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553, DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | source: md\currency_transfer_7_28.11.2025.md (стр. 41–44) |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | source: md\currency_transfer_7_28.11.2025.md (стр. 16–17) |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения | source: md\currency_transfer_7_28.11.2025.md (стр. 9–11) |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения | source |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | source |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | source |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator |
| 13 | Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) 044525411 | CD | банк плательщика | source: md\currency_transfer..., стр.50–53 |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | source |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT: VTBRCNSHXXX | CD | банк получателя | source |
| 16 | PersonSurname | Саранов | CO | фамилия | operator |
| 17 | PersonName | Дмитрий | CO | имя | operator |
| 18 | doc_code | 04023 | CD | код документа | derived |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | derived |
| 20 | doc_number | 7 | CD | номер документа | derived |
| 21 | doc_date | 28.11.2025 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 21 из 21
- `doc_formalization_status`: confirmed


### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: md\currency_transfer_1_13.01.2026.md
  - `file_name`: currency_transfer_1_13.01.2026.md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CO | код вида документа | operator |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | source: md\currency_transfer_1_13.01.2026.md (стр. 15–17) |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator |
| 05 | Priority | 5 | CO | очередность | operator |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553, DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | source |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | source |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | source |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения | source |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | source |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | source |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator |
| 13 | Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) 044525411 | CD | банк плательщика | source |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель | source |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH; SWIFT: VTBRCNSHXXX | CD | банк получателя | source |
| 16 | PersonSurname | Саранов | CO | фамилия | operator |
| 17 | PersonName | Дмитрий | CO | имя | operator |
| 18 | doc_code | 04023 | CD | код документа | derived |
| 19 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | derived |
| 20 | doc_number | 1 | CD | номер документа | derived |
| 21 | doc_date | 13.01.2026 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 21 из 21
- `doc_formalization_status`: confirmed


### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice_1
  - `xml_target_root`: AltaServiceInvoice
  - `path`: md\Счет_№26-00378-tl_от_27-01-2026.md
  - `file_name`: Счет_№26-00378-tl_от_27-01-2026.md
  - `note`: транспортно-экспедиционные услуги (Transimperial)

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentSign | 1 | CO | системный признак документа Альты | operator |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | source: md\Счет_№26-00378-tl_от_27-01-2026.md (стр.47–49) |
| 03 | Currency | USD | CD | валюта итого | source: md\Счет..., стр.40 |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг | source: md\Счет..., стр.22–25 |
| 05 | BankName | АО "Райффайзенбанк" | CD | банк исполнителя | source: md\Счет..., стр.14–20 |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги/перевозку | source: md\Счет..., стр.44–46 |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги/перевозку | source: md\Счет..., стр.44–46 |
| 08 | PrDocumentNumber | ОТСУТСТВУЕТ | CO | номер связанного документа/заказа | operator |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CO | дата связанного документа/заказа | operator |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | derived: из заголовка |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета | source: md\Счет..., стр.27 |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета | source |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | CO | грузоотправитель | operator: consignor=seller |
| 14 | PostalCode |  | CO | индекс | operator: оставлять пустым если отсутствует |
| 15 | CountryCode | CN | CD | страна | copied_from: Invoice seller |
| 16 | CounryName | Китай | CD | страна, текст | derived |
| 17 | Region | Hebei | CD | регион | copied_from |
| 18 | Town | Shijiazhuang | CD | город/район | copied_from |
| 19 | StreetHouse | No. 5 Gaodong street | CD | улица/дом | copied_from |
| 20 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | source: md\Счет..., покупатель |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator: разрешено из master_data |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | source: md\Счет..., стр.35–38 |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | source |
| 24 | PostalCode | 423800 | CD | индекс | source |
| 25 | CountryCode | RU | CD | страна | derived |
| 26 | CounryName | Россия | CD | страна, текст | derived |
| 27 | Region | Республика Татарстан | CD | регион | source |
| 28 | Town | Набережные Челны | CD | город | source |
| 29 | StreetHouse | проезд Хлебный | CD | улица | source |
| 30 | House | 30 | CO | дом | operator |
| 31 | Room | 211 | CO | офис/кв | operator |
| 32 | Signature_Choice | 2 | CD | вариант подписи | derived |
| 33 | IndividualEntrepreneur_PersonSurname |  | CD | фамилия ИП | derived: для варианта 2 поле пустое |
| 34 | IndividualEntrepreneur_PersonName |  | CD | инициал/имя ИП | derived: для варианта 2 поле пустое |
| 35 | IndividualEntrepreneur_PersonMiddleName |  | CD | инициал/отчество индивидуального предпринимателя | derived: для варианта 2 поле пустое |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климовин | CO | фамилия руководителя | operator |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CO | инициал/имя руководителя | operator |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А. | CD | инициал/отчество руководителя | derived: из инициалов "Л.А." |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CO | фамилия бухгалтера | operator |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CO | инициал/имя бухгалтера | operator |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А. | CD | инициал/отчество бухгалтера | derived: из инициалов "О.А." |
| 42 | ServiceDescription_[n] |  | CD | услуги | 2 строки |

#### ServiceDescription_1
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении ... China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | source: md\Счет..., стр.44 |
| 02 | CurrencyCode | USD | CD | валюта строки | copied_from: Currency |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator |
| 04 | TaxRate | 0% | CD | ставка налога | source: md\Счет..., стр.48 |
| 05 | TaxSum | 0.00 | CD | сумма налога | source |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | source |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | derived |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### ServiceDescription_2
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ ... - Россия, Республика Татарстан, Набережные Челны | CD | описание услуги | source: md\Счет..., стр.45 |
| 02 | CurrencyCode | USD | CD | валюта строки | derived |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator |
| 04 | TaxRate | 0% | CD | ставка налога | source |
| 05 | TaxSum | 0.00 | CD | сумма налога | source |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | source |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | derived |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 14 из 7 * 2
- `array_status`: pending

| 43 | doc_code | 04023 | pending | код документа | конфликт в схеме: в шаблоне service_invoice указано 04023/ПЛАТЕЖНОЕ ПОРУЧЕНИЕ; требуется сверка (вероятно должно быть 04031) |
| 44 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | pending | наименование документа | см. примечание по конфликту кода/имени в шаблоне |
| 45 | doc_number | 26-00378-tl | CD | номер документа | derived |
| 46 | doc_date | 27.01.2026 | CD | дата документа | derived |
| 47 | transport_to_border | 1404.00 | CD | стоимость перевозки до границы | derived: строка 1 |

#### Итого, по документу:
- `doc_fields`: 47 из 47
- `doc_formalization_status`: pending


### `document`: Insurance Services Invoice
  - `uqi_prefix`: formalized.insurance_document_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\Счет_№26-00378-tl_1_от_14-01-2026.md
  - `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md
  - `note`: счет за страхование груза

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04111 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | Счет на оплату (страхование груза) | CD | наименование документа | derived: из заголовка/назначения |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа | source: md\Счет...tl/1, стр.11 |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа | source: md\Счет...tl/1, стр.11 |
| 05 | TextPara | link:md\Счет_№26-00378-tl_1_от_14-01-2026.md | CO | основной текст/условия | operator: textpara_storage=link |
| 06 | doc_code | 04111 | CD | код документа | derived |
| 07 | doc_name | СТРАХОВОЙ ДОКУМЕНТ | CD | наименование документа | derived |
| 08 | doc_number | 26-00378-tl/1 | CD | номер документа | derived |
| 09 | doc_date | 14.01.2026 | CD | дата документа | derived |
| 10 | insurance_to_border | 910.34 RUB | CD | стоимость страхования продавцом | source: md\Счет...tl/1, стр.42–48 |

#### Итого, по документу:
- `doc_fields`: 10 из 10
- `doc_formalization_status`: confirmed


### `document`: TechDescription
  - `uqi_prefix`: formalized.tech_description_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\техничка Антикот, антипыльца антимошка .md
  - `file_name`: техничка Антикот, антипыльца антимошка .md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 05999 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование техописания | source: md\техничка..., стр.11 |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата техописания | operator: tech_description defaults |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания | operator |
| 05 | TextPara | link:md\техничка Антикот, антипыльца антимошка .md | CD | технический текст | link вместо полного текста |
| 06 | doc_code | 05999 | CD | код документа | derived |
| 07 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | derived |
| 08 | doc_number | Б/Н | CD | номер документа | derived |
| 09 | doc_date | 30.10.2025 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 9 из 9
- `doc_formalization_status`: confirmed


### `document`: Transport Contract
  - `uqi_prefix`: formalized.transport_contract_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: stable_source\FreeDoc_КООО_26651_М.xml
  - `file_name`: FreeDoc_КООО_26651_М.xml

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04033 | CD | код вида документа | source: stable_source\FreeDoc_КООО_26651_М.xml |
| 02 | DocumentHead_DocumentName | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование договора | source |
| 03 | DocumentHead_DocumentDate | 2025-05-13 | CD | дата договора | source |
| 04 | DocumentHead_DocumentNumber | КООО/26651/М | CD | номер договора | source |
| 05 | TextPara | link:stable_source\FreeDoc_КООО_26651_М.xml | CD | текст договора | link на xml с TextPara |
| 06 | doc_code | 04033 | CD | код документа | derived |
| 07 | doc_name | ДОГОВОР ТРАНСПОРТНОЙ ЭКСПЕДИЦИИ | CD | наименование документа | derived |
| 08 | doc_number | КООО/26651/М | CD | номер документа | derived |
| 09 | doc_date | 2025-05-13 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 9 из 9
- `doc_formalization_status`: confirmed


### `document`: EGRUL
  - `uqi_prefix`: formalized.egrul_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: stable_source\FreeDoc_ЮЭ9965-25-106893283.xml
  - `file_name`: FreeDoc_ЮЭ9965-25-106893283.xml

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04011 | CD | код вида документа | source: stable_source\FreeDoc_ЮЭ9965-25-106893283.xml |
| 02 | DocumentHead_DocumentName | ВЫПИСКА ИЗ  ЕГРЮЛ | CD | наименование выписки | source |
| 03 | DocumentHead_DocumentDate | 2025-07-14 | CD | дата выписки | source |
| 04 | DocumentHead_DocumentNumber | ЮЭ9965-25-106893283 | CD | номер выписки | source |
| 05 | TextPara | link:stable_source\FreeDoc_ЮЭ9965-25-106893283.xml | CD | текст выписки | link |
| 06 | doc_code | 04011 | CD | код документа | derived |
| 07 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | derived |
| 08 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | derived |
| 09 | doc_date | 2025-07-14 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 9 из 9
- `doc_formalization_status`: confirmed


### `document`: Passport
  - `uqi_prefix`: formalized.passport_1
  - `xml_target_root`: AltaPassport
  - `path`: stable_source\Passport_63_09_449948.xml
  - `file_name`: Passport_63_09_449948.xml

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 11001 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | ПАСПОРТ | CD | наименование документа | derived |
| 03 | DocumentHead_DocumentDate | 2010-03-11 | CD | дата документа | derived: =CardDate |
| 04 | DocumentHead_DocumentNumber | 63 09 449948 | CD | номер документа | derived |
| 05 | CardSeries | 63 09 | CD | серия | source: stable_source\Passport_63_09_449948.xml |
| 06 | CardNumber | 449948 | CD | номер | source |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | source |
| 08 | CardDate | 2010-03-11 | CD | дата выдачи | source |
| 09 | PersonInfo_PersonSurname | АРБУЗОВА | CD | фамилия | source |
| 10 | PersonInfo_PersonName | АНАСТАСИЯ | CD | имя | source |
| 11 | PersonInfo_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | source |
| 12 | PersonInfo_Sex | 1 | CD | пол | source |
| 13 | PersonInfo_Birthday | 1987-07-25 | CD | дата рождения | source |
| 14 | PersonInfo_Birthplace | город Саратов | CD | место рождения | source |
| 15 | ResidencePlace_PostalCode | 410052 | CD | индекс | source |
| 16 | ResidencePlace_CountryCode | RU | CD | страна alpha-2 | source |
| 17 | ResidencePlace_CounryName | РОССИЯ | CD | страна, текст | source |
| 18 | ResidencePlace_Region | Саратовская область | CD | регион | source |
| 19 | ResidencePlace_City | Саратов | CD | город | source |
| 20 | ResidencePlace_StreetHouse | Ул. Одесская д 11 кв 160 | CD | адрес одной строкой | source |
| 21 | doc_code | 11001 | CD | код документа | derived |
| 22 | doc_name | ПАСПОРТ | CD | наименование документа | derived |
| 23 | doc_number | 63 09 449948 | CD | номер документа | derived |
| 24 | doc_date | 2010-03-11 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 24 из 24
- `doc_formalization_status`: confirmed


### `document`: Letter of Attorney
  - `uqi_prefix`: formalized.letter_of_attorney_1
  - `xml_target_root`: AltaLetterOfAttorney
  - `path`: stable_source\LetterOfAttorney_1.xml
  - `file_name`: LetterOfAttorney_1.xml

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 11004 | CD | код вида документа | derived |
| 02 | DocumentHead_DocumentName | ДОВЕРЕННОСТЬ | CD | наименование доверенности | derived |
| 03 | DocumentHead_DocumentDate | 2026-02-01 | CD | дата доверенности | derived |
| 04 | DocumentHead_DocumentNumber | 1 | CD | номер доверенности | derived |
| 05 | Subject | link:stable_source\LetterOfAttorney_1.xml | CD | текст доверенности | link |
| 06 | EndDate | 2026-12-31 | CD | действительна до | source |
| 07 | DocumentReference_PrDocumentName | ДОВЕРЕННОСТЬ | CD | наименование доверенности | source |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер доверенности | source |
| 09 | DocumentReference_PrDocumentDate | 2026-02-01 | CD | дата доверенности | source |
| 10 | Organization_OrganizationName | ООО «СКИФ» | CD | выдавшая организация | source |
| 11 | Organization_ShortName | ООО «СКИФ» | CD | краткое наименование | source |
| 12 | Organization_OGRN | 1201600020390 | CD | ОГРН | source |
| 13 | Organization_INN | 1650389298 | CD | ИНН | source |
| 14 | Organization_KPP | 165001001 | CD | КПП | source |
| 15 | Organization_Address_PostalCode | 423800 | CD | индекс | source |
| 16 | Organization_Address_CountryCode | RU | CD | страна | source |
| 17 | Organization_Address_CounryName | РОССИЯ | CD | страна, текст | source |
| 18 | Organization_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | source |
| 19 | Organization_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | source |
| 20 | Organization_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | CD | улица/дом | source |
| 21 | Organization_OrganizationPerson_PersonSurname | Саранов | CD | подписант от организации | source |
| 22 | Organization_OrganizationPerson_PersonName | Дмитрий | CD | имя | source |
| 23 | Organization_OrganizationPerson_PersonMiddleName | Олегович | CD | отчество | source |
| 24 | Organization_OrganizationPerson_PersonPost | Директор | CD | должность | source |
| 25 | EmpoweredPerson_PersonSurname | АРБУЗОВА | CD | уполномоченное лицо | source |
| 26 | EmpoweredPerson_PersonName | АНАСТАСИЯ | CD | имя | source |
| 27 | EmpoweredPerson_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | source |
| 28 | EmpoweredPerson_PersonPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | source |
| 29 | EmpoweredPerson_Passport_IdentityCardCode | RU01001 | CD | код документа | source |
| 30 | EmpoweredPerson_Passport_IdentityCardName | ПАСРФ | CD | наименование документа | source |
| 31 | EmpoweredPerson_Passport_IdentityCardSeries | 63 09 | CD | серия | source |
| 32 | EmpoweredPerson_Passport_IdentityCardNumber | 449948 | CD | номер | source |
| 33 | EmpoweredPerson_Passport_IdentityCardDate | 2010-03-11 | CD | дата выдачи | source |
| 34 | EmpoweredPerson_Passport_OrganizationName | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | source |
| 35 | doc_code | 11004 | CD | код документа | derived |
| 36 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | derived |
| 37 | doc_number | 1 | CD | номер документа | derived |
| 38 | doc_date | 2026-02-01 | CD | дата документа | derived |

#### Итого, по документу:
- `doc_fields`: 38 из 38
- `doc_formalization_status`: confirmed


## non_formalized

### `document`: Storage Report (ДО-1)
  - `uqi_prefix`: non_formalized.svh_1
  - `path`: md\ДО 14431420260204161621.md
  - `file_name`: ДО 14431420260204161621.md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии/свидетельства СВХ | source: CMR (md\СМР..., стр.50) |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии/свидетельства СВХ | source: CMR (стр.50) |
| 03 | actual_gross_weight | 3500 | CD | фактический вес по весам | source: md\ДО..., итого стр.59–63 |
| 04 | actual_places | 127 | CD | фактическое количество мест | source: md\ДО..., итого |
| 05 | transport_reg_number | 0157A0774 | pending | номер ТС при въезде/по отчету СВХ | в ДО читается фрагмент (стр.15–16) как unreliable_parts; требуется уточнить |
| 06 | goods_[n] |  | CD | товары в разрезе строк ДО | 2 строки (2 кода) |

#### goods_1
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | tnved | 7019900095 | CD | код товара | source: md\ДО..., стр.29–31 |
| 02 | places | 27 | CD | кол-во грузовых мест | source: md\ДО..., стр.41 |
| 03 | gross_weight_kg | 1710 | CD | вес брутто по строке | source: md\ДО..., стр.42 |
| 04 | cost | 42228 | CD | стоимость по строке | source: md\ДО..., стр.43 |
| 05 | currency_code | CNY | CD | буквенный код валюты | derived: по ДО CNY |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### goods_2
| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | tnved | 5804101000 | CD | код товара | source: md\ДО..., стр.45–47 |
| 02 | places | 100 | CD | кол-во грузовых мест | source: md\ДО..., стр.55 |
| 03 | gross_weight_kg | 1790 | CD | вес брутто | source |
| 04 | cost | 55032 | CD | стоимость | source |
| 05 | currency_code | CNY | CD | валюта | derived |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 10 из 5 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 6 из 6


### `document`: Storage Report Additional Sheet
  - `uqi_prefix`: non_formalized.svh_additional_sheet_1
  - `path`: md\ДО доп 14431520260204161645.md
  - `file_name`: ДО доп 14431520260204161645.md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | number | 2 | CD | № доп.листа/приложения | source: md\ДО доп..., стр.12–14 |
| 02 | date | 03.02.2026 | CD | дата доп.листа | source: md\ДО доп..., стр.34 |
| 03 | actual_gross_weight | 3500 | pending | фактический вес по весам | в доплисте итоги не читаются; использовать ДО-1 как источник |
| 04 | actual_places | 127 | pending | фактическое количество мест | см. выше |
| 05 | transport_reg_number | 0157A0774 | CD | номер ТС | source: md\ДО доп..., стр.16 |
| 06 | svh_address_region | Республика Татарстан | CO | регион СВХ | operator: разрешено из CMR |
| 07 | svh_address_city | Набережные Челны | CO | город СВХ | operator |
| 08 | svh_address_street_house | Производственный пр-д, 45, СВХ ООО «ЛОГТИКАМ» | CO | улица/дом СВХ | operator |
| 09 | svh_customs_code | 10404083 | CD | код таможенного органа в зоне СВХ | source: CMR (стр.46) |

#### Итого, по документу:
- `doc_fields`: 9 из 9


### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.td_1
  - `path`: md\ТД 10719110_240126_5011363_reg00378тд.md
  - `file_name`: ТД 10719110_240126_5011363_reg00378тд.md

| num | field | value | status | description | note |
|---:|-------|-------|--------|-------------|------|
| 01 | number | 10719110/240126/5011363 | CO | номер ТД | operator: confirmed |
| 02 | date | 24.01.2026 | CO | дата ТД | operator: confirmed |
| 03 | customs_post_code | 10719110 | CD | код таможенного органа | source: md\ТД..., стр.77–80 |
| 04 | customs_post_name | п/п МАПП Забайкальск | CD | наименование таможенного органа | source: md\ТД..., стр.77–80 |
| 05 | transport_reg_number | 0157A0774/BT374974 | CD | ТС по ТД | source: md\ТД..., стр.39–42 |

#### Итого, по документу:
- `doc_fields`: 5 из 5


## Нерешенные вопросы (Issues)

**Для полей:**
- `formalized.service_invoice_1.IndividualEntrepreneur_*`
  - `question`: В схеме Service Invoice блок подписей содержит поля ИП (33–35). В счете исполнитель = ООО «Трансимпериал», ИП нет. Подтверди: оставить эти поля пустыми как pending или заполнить «ОТСУТСТВУЕТ»?

- `formalized.service_invoice_1.SignatureDirectorChiefAccountant_*_PersonMiddleName`
  - `question`: Отчества директора/бухгалтера не указаны. Подтверди: оставляем пусто (pending) или можно считать, что отчества отсутствуют?

- `formalized.service_invoice_1.doc_code/doc_name`
  - `question`: В primary_schema.md для Service Invoice в конце документа указаны неформализуемые поля с кодом/именем платежного поручения (04023). Похоже на ошибку схемы (для Service Invoice ожидается 04031). Подтверди, какой doc_code/doc_name использовать.

- `non_formalized.svh_1.transport_reg_number`
  - `question`: В ДО-1 номер ТС читается фрагментарно и помечен как unreliable_parts. Можно ли взять номер ТС из CMR/ТД (0157AO774/BT374974) для поля transport_reg_number? Если да — какой формат предпочитаешь?

- `non_formalized.svh_additional_sheet_1.actual_gross_weight/actual_places`
  - `question`: В доп.листе ДО итоги не читаются. Подтверди: брать итоги из ДО-1 (127 мест, 3500 кг) и ставить статус CD.

**Для общих вопросов:**
- `[Общий]`
  - `question`: Подтверди, что валюта RMB в документах трактуется как CNY (numeric 156) для всех полей.


### Итогo, по файлу:
- `total_unreliable_fields`: 1
- `total_doc_fields`:  (см. total_fields)
- `total_fields`:  (не подсчитано автоматически)
- `formalization_status`: Partial
