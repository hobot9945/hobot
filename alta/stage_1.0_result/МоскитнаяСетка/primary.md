# primary.md — Этап 1 (МоскитнаяСетка)

## Метаданные
- название кейса: МоскитнаяСетка
- путь к папке поставки: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02
- примечание: структура товаров целевая для stage 2–3 — 7 строк как в Invoice/PL (см. operator_provided_data.md)

---

# I. formalized

## document: Contract
- uqi_prefix: formalized.contract_1
- xml_target_root: AltaE2CONT
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\SALES CONTRACT NoLM-2553.md
- file_name: SALES CONTRACT NoLM-2553.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 03011 | код вида документа | confirmed_document | |
| ContractRegistration_PrDocumentNumber | LM-2553 | № контракта | confirmed_document | |
| ContractRegistration_PrDocumentDate | 2025-07-02 | дата контракта | confirmed_document | |
| ContractTerms_Amount | 41904.30 | общая сумма контракта | confirmed_document | |
| ContractTerms_CurrencyCode | 156 | код валюты ISO numeric | confirmed_operator | operator_provided_data: contract.currency_code_numeric=CNY(156) |
| ContractTerms_LastDate | 2026-12-31 | срок действия/исполнения | confirmed_document | Shipment period END OF PERIOD: 31/12/2026 |
| ContractTerms_OtherTerms | EXW HEBEI | условия поставки / Incoterms | confirmed_operator | operator_provided_data: contract.delivery_terms |
| ContractTerms_ContractText | link:alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\контракт\\SALES CONTRACT NoLM-2553.pdf | текст контракта (link) | confirmed_document | в primary.md храним link |
| ContractTerms_DealSign | 1 | системный признак Альты | confirmed_operator | operator_provided_data: contract.deal_sign |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | продавец | confirmed_document | |
| ForeignPerson_Address_CountryCode | CN | страна продавца | confirmed_operator | подтверждено оператором |
| ForeignPerson_Address_CounryName | Китай | страна продавца (текст) | confirmed_operator | подтверждено оператором |
| ForeignPerson_Address_Region | Hebei | регион | confirmed_document | |
| ForeignPerson_Address_City | Shijiazhuang | город | confirmed_document | |
| ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District | улица/дом (строкой) | confirmed_document | нормализовано из адреса в контракте |
| RussianPerson_OrganizationName | ООО «СКИФ» | покупатель | confirmed_document | |
| RussianPerson_OGRN | 1201600020390 | ОГРН | confirmed_document | stable_source EGRUL |
| RussianPerson_INN | 1650389298 | ИНН | confirmed_document | stable_source EGRUL |
| RussianPerson_KPP | 165001001 | КПП | confirmed_document | stable_source EGRUL |
| RussianPerson_Address_PostalCode | 423800 | индекс | confirmed_document | EGRUL / Invoice / PL |
| RussianPerson_Address_CountryCode | RU | страна | confirmed_document | |
| RussianPerson_Address_CounryName | Россия | страна (текст) | confirmed_document | |
| RussianPerson_Address_Region | Республика Татарстан | регион | confirmed_document | |
| RussianPerson_Address_City | Набережные Челны | город | confirmed_document | |
| RussianPerson_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | адрес (строкой) | confirmed_document | |

---

## document: Supplementary Contract
- uqi_prefix: formalized.supplementary_contract_1
- xml_target_root: AltaSupplementaryContract
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\1 Supplementary agreement to the contract.md
- file_name: 1 Supplementary agreement to the contract.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentNumber | 1 | № доп. соглашения | confirmed_document | |
| IssueDate | 2025-11-25 | дата доп. соглашения | confirmed_document | |
| ContractDescription_Amount | 270000.00 | сумма по допнику | confirmed_document | |
| ContractDescription_CurrencyCode | 156 | код валюты numeric | confirmed_operator | operator_provided_data |
| ContractDescription_LastDate | 2026-12-31 | новый срок | confirmed_operator | operator_provided_data: expiry_date |
| ContractDescription_ContractText | link:alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\контракт\\1 Supplementary agreement to the contract.pdf | текст (link) | confirmed_document | |
| ContractDescription_DealSign | 1 | системный признак | confirmed_operator | |
| ContractDescription_StockCategorySign | 0 | системный признак | confirmed_operator | |
| ContractDescription_BuyerLimitationSign | 0 | системный признак | confirmed_operator | |
| ContractDescription_InsuranceSign | 0 | системный признак | confirmed_operator | |
| RussianPerson_OrganizationName | ООО «СКИФ» | российская сторона | confirmed_document | |
| RussianPerson_ShortName | ООО «СКИФ» | краткое наименование | confirmed_operator | решение: short=full |
| RussianPerson_OGRN | 1201600020390 | ОГРН | confirmed_document | stable_source |
| RussianPerson_INN | 1650389298 | ИНН | confirmed_document | stable_source |
| RussianPerson_KPP | 165001001 | КПП | confirmed_document | stable_source |
| RussianPerson_Address_PostalCode | 423800 | индекс | confirmed_document | |
| RussianPerson_Address_CountryCode | RU | страна | confirmed_document | |
| RussianPerson_Address_CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| RussianPerson_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | улица/дом | confirmed_document | |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | иностранная сторона | confirmed_document | |
| ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | краткое | confirmed_operator | foreign_person_short_name_equals_full=true |
| ForeignPerson_Address_CountryCode | CN | страна | confirmed_operator | подтверждено оператором |
| ForeignPerson_Address_CounryName | Китай | страна (текст) | confirmed_operator | подтверждено оператором |
| ForeignPerson_Address_Region | Hebei | регион | confirmed_operator | подтверждено оператором |
| ForeignPerson_Address_City | Shijiazhuang | город | confirmed_operator | подтверждено оператором |
| ForeignPerson_Address_StreetHouse | No. 5 Gaodong street | улица/дом | confirmed_operator | подтверждено оператором |

### ContractSignedPerson
| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname | Li | фамилия подписанта | confirmed_operator | |
| PersonName | Jing | имя подписанта | confirmed_operator | |
| PersonMiddleName |  | отчество | confirmed_operator | решение: пусто |

---

## document: Invoice
- uqi_prefix: formalized.invoice_1
- xml_target_root: AltaE2I
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\CL на сетку.md
- file_name: CL на сетку.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| CurrencyRate | 10.9430 | курс валюты | confirmed_operator | operator_provided_data |
| CurrencyCode | CNY | валюта инвойса | confirmed_operator | |
| DocumentCode | 04021 | код вида документа | confirmed_document | |
| PlacesQuantity | 127 | количество мест | confirmed_document | PL total Qty BG 127 |
| PlacesDescription | Поддон | описание мест | confirmed_operator | |
| GrossWeightQuantity | 3500.00 | общий брутто | confirmed_document | PL total Gross 3500,00 |
| NetWeightQuantity | 3302.00 | общий нетто | confirmed_document | PL total Net 3302,00 |
| GCost | 97260.00 | системное поле | confirmed_operator | решение оператора: =TotalCost |
| TotalCost | 97260.00 | итого | confirmed_document | CL Total 97260,00 |
| DeliveryTerms_DeliveryPlace | HEBEI | место поставки | confirmed_document | EXW HEBEI |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | числовой код условий | confirmed_operator | |
| DeliveryTerms_DeliveryTermsStringCode | EXW | строковый код условий | confirmed_operator | |
| DeliveryTerms_DispatchCountryCode | CN | страна отправления | confirmed_operator | |
| DeliveryTerms_TradingCountryCode | CN | торгующая страна | confirmed_operator | |
| DeliveryTerms_DestinationCountryCode | RU | страна назначения | confirmed_operator | |
| Registration_PrDocumentName | Commercial invoice | наименование | confirmed_document | |
| Registration_PrDocumentNumber | LM-2591 | номер | confirmed_document | |
| Registration_PrDocumentDate | 2025-10-30 | дата | confirmed_document | |
| Contract_PrDocumentNumber | LM-2553 | № контракта-ссылки | confirmed_document | |
| Contract_PrDocumentDate | 2025-07-02 | дата контракта-ссылки | confirmed_document | |
| Buyer_CompanyID | 1650389298 | ИНН покупателя | confirmed_document | EGRUL |
| Buyer_KPPCode | 165001001 | КПП покупателя | confirmed_document | EGRUL |
| Buyer_Name | ООО «СКИФ» | покупатель | confirmed_document | |
| Buyer_PostalAddress_PostalCode | 423800 | индекс | confirmed_document | |
| Buyer_PostalAddress_CountryCode | RU | страна | confirmed_document | |
| Buyer_PostalAddress_CounryName | Россия | страна (текст) | confirmed_document | |
| Buyer_PostalAddress_Region | Republic of Tatarstan | регион | confirmed_document | из инвойса/PL (англ.) |
| Buyer_PostalAddress_City | Naberezhnye Chelny | город | confirmed_document | |
| Buyer_PostalAddress_StreetHouse | Khlebny Passage, hause 30, office 211 | улица/дом | confirmed_document | как в инвойсе |
| Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | продавец | confirmed_document | |
| Seler_PostalAddress_CountryCode | CN | страна | confirmed_operator | подтверждено оператором |
| Seler_PostalAddress_CounryName | China | страна (текст) | confirmed_document | |
| Seler_PostalAddress_Region | Hebei | регион | confirmed_document | |
| Seler_PostalAddress_City | Shijiazhuang | город | confirmed_document | |
| Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | улица/дом | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | грузоотправитель | confirmed_operator | решение: consignor_equals_seller=true |
| Consignor_Address_CountryCode | CN | страна | confirmed_operator | подтверждено оператором |
| Consignor_Address_CounryName | China | страна (текст) | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street | улица/дом | confirmed_document | |
| Consignee_OrganizationName | LLC «SKIF» | грузополучатель | confirmed_operator | решение: consignee_equals_buyer=true |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | stable_source |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | stable_source |
| Consignee_KPP | 165001001 | КПП | confirmed_document | stable_source |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | Russia | страна (текст) | confirmed_document | |
| Consignee_Address_Region | Republic of Tatarstan | регион | confirmed_document | |
| Consignee_Address_City | Naberezhnye Chelny | город | confirmed_document | |
| Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | улица/дом | confirmed_document | |

### InvoiceGoods_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | ТН ВЭД | confirmed_document | |
| GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | описание | confirmed_document | |
| GoodsQuantity | 60 | кол-во | confirmed_document | Q-ty Sets |
| goods_supplementary_quantity | 2520 | доп.кол-во | confirmed_document | Quantity in M2 |
| goods_supplementary_uom_name | м² | доп.ед. | confirmed_operator | cb:unit 055 |
| MeasureUnitQualifierName | м² | ед.изм | confirmed_operator | |
| GrossWeightQuantity | 855.00 | брутто | confirmed_operator | из PL weights |
| NetWeightQuantity | 806.60 | нетто | confirmed_operator | из PL weights |
| Price | 5.85 | цена за м2 | confirmed_document | |
| TotalCost | 14742.00 | стоимость строки | confirmed_document | |
| OriginCountryCode | 156 | код страны происхождения numeric | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак/маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель/модификация | confirmed_operator | |

### InvoiceGoods_2
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | ТН ВЭД | confirmed_document | |
| GoodsDescription | Anti-cat mesh Roll size 1.6 * 30 / Москитная сетка «Антикот» Размер рулона 1,6*30 | описание | confirmed_document | |
| GoodsQuantity | 30 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 1440 | доп.кол-во | confirmed_document | |
| goods_supplementary_uom_name | м² | доп.ед. | confirmed_operator | |
| MeasureUnitQualifierName | м² | ед.изм | confirmed_operator | |
| GrossWeightQuantity | 490.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 460.80 | нетто | confirmed_operator | |
| Price | 5.85 | цена | confirmed_document | |
| TotalCost | 8424.00 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | происхождение | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_3
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | ТН ВЭД | confirmed_document | |
| GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 / Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 60 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 2520 | доп.кол-во | confirmed_document | |
| goods_supplementary_uom_name | м² | доп.ед. | confirmed_operator | |
| MeasureUnitQualifierName | м² | ед.изм | confirmed_operator | |
| GrossWeightQuantity | 265.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 252.00 | нетто | confirmed_operator | |
| Price | 6.35 | цена | confirmed_document | |
| TotalCost | 16002.00 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | происхождение | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_4
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | ТН ВЭД | confirmed_document | |
| GoodsDescription | ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2 / Сетка против пыльцы Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 30 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 1440 | доп.кол-во | confirmed_document | |
| goods_supplementary_uom_name | м² | доп.ед. | confirmed_operator | |
| MeasureUnitQualifierName | м² | ед.изм | confirmed_operator | |
| GrossWeightQuantity | 155.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 144.00 | нетто | confirmed_operator | |
| Price | 6.35 | цена | confirmed_document | |
| TotalCost | 9144.00 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | происхождение | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_5
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 7019900095 | ТН ВЭД | confirmed_document | |
| GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 90 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 3780 | доп.кол-во | confirmed_document | |
| goods_supplementary_uom_name | м² | доп.ед. | confirmed_operator | |
| MeasureUnitQualifierName | м² | ед.изм | confirmed_operator | |
| GrossWeightQuantity | 520.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 491.40 | нетто | confirmed_operator | |
| Price | 3.4 | цена | confirmed_document | |
| TotalCost | 12852.00 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | происхождение | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_6
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 7019900095 | ТН ВЭД | confirmed_document | |
| GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 : Fiberglass / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 180 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 8640 | доп.кол-во | confirmed_document | |
| goods_supplementary_uom_name | м² | доп.ед. | confirmed_operator | |
| MeasureUnitQualifierName | м² | ед.изм | confirmed_operator | |
| GrossWeightQuantity | 1190.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 1123.20 | нетто | confirmed_operator | |
| Price | 3.4 | цена | confirmed_document | |
| TotalCost | 29376.00 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | происхождение | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_7
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | ТН ВЭД | confirmed_document | |
| GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 / Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 5 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 240 | доп.кол-во | confirmed_document | |
| goods_supplementary_uom_name | м² | доп.ед. | confirmed_operator | |
| MeasureUnitQualifierName | м² | ед.изм | confirmed_operator | |
| GrossWeightQuantity | 25.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 24.00 | нетто | confirmed_operator | |
| Price | 28 | цена | confirmed_document | |
| TotalCost | 6720.00 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | происхождение | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

---

## document: Packing List
- uqi_prefix: formalized.packing_list_1
- xml_target_root: AltaE2PACK
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\PL на сетку.md
- file_name: PL на сетку.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| GrossWeightQuantity | 3500.00 | общий брутто | confirmed_document | |
| NetWeightQuantity | 3302.00 | общий нетто | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | отправитель | confirmed_document | |
| Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | краткое | confirmed_operator | short=full |
| Consignor_Address_CountryCode | CN | страна | confirmed_operator | подтверждено оператором |
| Consignor_Address_CounryName | China | страна (текст) | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street | улица/дом | confirmed_document | |
| Consignee_OrganizationName | LLC «SKIF» | получатель | confirmed_document | |
| Consignee_ShortName | LLC «SKIF» | краткое | confirmed_operator | short=full |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | Russia | страна (текст) | confirmed_document | |
| Consignee_Address_Region | Republic of Tatarstan | регион | confirmed_document | |
| Consignee_Address_City | Naberezhnye Chelny | город | confirmed_document | |
| Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | улица/дом | confirmed_document | |
| DeliveryTerms_DeliveryPlace | HEBEI | место поставки | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | числовой код | confirmed_operator | |
| DeliveryTerms_DeliveryTermsStringCode | EXW | строковый | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentName | SALES CONTRACT | наименование | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | № | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentDate | 2025-07-02 | дата | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentName | INVOICE | наименование | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | № | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentDate | 2025-10-30 | дата | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentName | Packing list | наименование | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | № | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentDate | 2025-10-30 | дата | confirmed_document | |

#### Goods_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Anti-cat mesh / Антикот 1,4*0,16*0,16 | описание | confirmed_document | модель из строки 1 |
| GoodsQuantity | 60 | места/упаковки | confirmed_document | Qty BG |
| GrossWeightQuantity | 855.00 | брутто | confirmed_document | |
| NetWeightQuantity | 806.60 | нетто | confirmed_document | |
| PackingInfo.PakingQuantity | 60 | кол-во упаковок | confirmed_operator | решение: =GoodsQuantity |

#### Goods_2
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Anti-cat mesh / Антикот 1,6*0,16*0,16 | описание | confirmed_document | |
| GoodsQuantity | 30 | места/упаковки | confirmed_document | |
| GrossWeightQuantity | 490.00 | брутто | confirmed_document | |
| NetWeightQuantity | 460.80 | нетто | confirmed_document | |
| PackingInfo.PakingQuantity | 30 | кол-во упаковок | confirmed_operator | |

#### Goods_3
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | ANTI-POLLEN MESH 1,42*0,64*0,22 | описание | confirmed_document | |
| GoodsQuantity | 6 | места/упаковки | confirmed_document | Qty BG |
| GrossWeightQuantity | 265.00 | брутто | confirmed_document | |
| NetWeightQuantity | 252.00 | нетто | confirmed_document | |
| PackingInfo.PakingQuantity | 6 | кол-во упаковок | confirmed_operator | |

#### Goods_4
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | ANTI-POLLEN MESH 1,62*0,64*0,23 | описание | confirmed_document | |
| GoodsQuantity | 3 | места/упаковки | confirmed_document | |
| GrossWeightQuantity | 155.00 | брутто | confirmed_document | |
| NetWeightQuantity | 144.00 | нетто | confirmed_document | |
| PackingInfo.PakingQuantity | 3 | кол-во упаковок | confirmed_operator | |

#### Goods_5
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | MIDGE MEHS 1,42*0,55*0,18 | описание | confirmed_document | |
| GoodsQuantity | 9 | места/упаковки | confirmed_document | |
| GrossWeightQuantity | 520.00 | брутто | confirmed_document | |
| NetWeightQuantity | 491.40 | нетто | confirmed_document | |
| PackingInfo.PakingQuantity | 9 | кол-во упаковок | confirmed_operator | |

#### Goods_6
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | MIDGE MEHS 1,62*0,55*18 | описание | confirmed_document | |
| GoodsQuantity | 18 | места/упаковки | confirmed_document | |
| GrossWeightQuantity | 1190.00 | брутто | confirmed_document | |
| NetWeightQuantity | 1123.20 | нетто | confirmed_document | |
| PackingInfo.PakingQuantity | 18 | кол-во упаковок | confirmed_operator | |

#### Goods_7
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | GRID WITH 3 LAYER | описание | confirmed_document | |
| GoodsQuantity | 1 | места/упаковки | confirmed_document | |
| GrossWeightQuantity | 25.00 | брутто | confirmed_document | |
| NetWeightQuantity | 24.00 | нетто | confirmed_document | |
| PackingInfo.PakingQuantity | 1 | кол-во упаковок | confirmed_operator | |

#### TransportMeans_1
| field | value | description | status | note |
|---|---|---|---|---|
| Number | О157АО774 | номер | confirmed_operator | |
| ModeCode | 31 | вид транспорта | confirmed_operator | |
| NationalityCode | 000 | национальность | confirmed_operator | |
| MoverIndicator | true | тягач | confirmed_operator | |

#### TransportMeans_2
| field | value | description | status | note |
|---|---|---|---|---|
| Number | ВТ374974 | номер | confirmed_operator | |
| ModeCode | 31 | вид транспорта | confirmed_operator | |
| NationalityCode | 000 | национальность | confirmed_operator | |
| MoverIndicator | false | прицеп | confirmed_operator | |

---

## document: CMR
- uqi_prefix: formalized.cmr_1
- xml_target_root: AltaE3CMR
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\СМР от СВХ.md
- file_name: СМР от СВХ.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| LanguageCode | RU | язык | confirmed_operator | |
| CMR_Choice | 1 | системный признак | confirmed_operator | |
| RegistrationDocument_RegID | 00378 | номер CMR | confirmed_document | |
| RegistrationDocument_DateInf | 2026-01-20 | дата CMR | confirmed_document | |
| RegistrationDocument_Place | Маньчжурия | место составления | confirmed_document | |
| TrakingCargo_TakingCargoDate | 2026-01-20 | дата принятия | confirmed_document | |
| TrakingCargo_TakingCargoPlace_CountryCode | CN | страна принятия | confirmed_operator | |
| TrakingCargo_TakingCargoPlace_CounryName | Китай | страна (текст) | confirmed_operator | |
| DeliveryPlace_CountryCode | RU | страна доставки | confirmed_operator | |
| DeliveryPlace_CounryName | Россия | страна доставки (текст) | confirmed_operator | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | место поставки | confirmed_operator | |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия | confirmed_operator | |
| GoodsQuantity | 127 | кол-во мест | confirmed_document | |
| CMRGoodsWeight_GrossWeightQuantity | 3500.00 | брутто | confirmed_document | |
| CMRTransport_PrimeMoverStateSignID | О157АО774 | тягач | confirmed_document | поле 25 |
| CMRTransport_TrailerStateSignID | ВТ374974 | прицеп | confirmed_document | поле 25 |
| Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | отправитель | confirmed_document | |
| Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | краткое | confirmed_operator | short=full |
| Consignor_PostalAddress_CountryCode | CN | страна | confirmed_operator | |
| Consignor_Address_CounryName | China | страна (текст) | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street | улица/дом | confirmed_document | |
| Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | гарант | confirmed_operator | |
| Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | гарант | confirmed_operator | |
| Consignor_Guarantee_Address_CountryCode |  | страна | confirmed_operator | |
| Consignor_Guarantee_Address_CounryName |  | страна (текст) | confirmed_operator | |
| Consignor_Guarantee_Address_Region |  | регион | confirmed_operator | |
| Consignor_Guarantee_Address_City |  | город | confirmed_operator | |
| Consignor_Guarantee_Address_StreetHouse |  | улица/дом | confirmed_operator | |
| Consignee_NameInf | ООО «Скиф» | получатель | confirmed_document | |
| Consignee_ShortName | ООО «Скиф» | краткое | confirmed_operator | short=full |
| Consignee_OGRNID | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_INNID | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPPCode | 165001001 | КПП | confirmed_document | |
| Consignee_PostalAddress_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_PostalAddress_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| Consignee_Address_Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_Address_City | Набережные Челны | город | confirmed_document | |
| Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | улица/дом | confirmed_document | |

### CMRGoods_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsNumeric | 1 | номер строки | confirmed_operator | в выгрузке одна строка |
| GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | описание | confirmed_document | поле 6 |

#### GoodsPackingInfo
| field | value | description | status | note |
|---|---|---|---|---|
| PackingCode | PX | код упаковки | confirmed_operator | |
| PakingQuantity | 127 | кол-во | confirmed_operator | |
| PackingDescription | ПОДДОН | описание | confirmed_operator | |

---

## document: Payment Order
- uqi_prefix: formalized.payment_order_1
- xml_target_root: AltaPaymentOrder
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\currency_transfer_1_13.01.2026.md
- file_name: currency_transfer_1_13.01.2026.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04023 | код документа | confirmed_operator | решение оператора |
| PaymentModeCode | 0 | способ платежа | confirmed_operator | |
| PaymentAmount | 63219.00 | сумма | confirmed_document | |
| TransactionKind | 01 | вид операции | confirmed_operator | |
| Priority | 5 | очередность | confirmed_operator | |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | назначение | confirmed_document | поле 70 |
| ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | сумма прописью | confirmed_document | |
| DocumentReference_PrDocumentNumber | 1 | № заявления | confirmed_document | |
| DocumentReference_PrDocumentDate | 2026-01-13 | дата | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | плательщик | confirmed_document | |
| Payer_INN | 1650389298 | ИНН | confirmed_document | |
| Payer_KPP | 165001001 | КПП | confirmed_operator | оператор: payer_kpp |
| Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) 044525411 | банк плательщика | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | получатель | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX | банк получателя | confirmed_document | |

### PayerSign
| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname | Саранов | фамилия | confirmed_operator | решение оператора |
| PersonName | Дмитрий | имя | confirmed_operator | решение оператора |

---

## document: Payment Order
- uqi_prefix: formalized.payment_order_2
- xml_target_root: AltaPaymentOrder
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\currency_transfer_7_28.11.2025.md
- file_name: currency_transfer_7_28.11.2025.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04023 | код документа | confirmed_operator | |
| PaymentModeCode | 0 | способ | confirmed_operator | |
| PaymentAmount | 34041.00 | сумма | confirmed_document | |
| TransactionKind | 01 | вид операции | confirmed_operator | |
| Priority | 5 | очередность | confirmed_operator | |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | назначение | confirmed_document | |
| ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | сумма прописью | confirmed_document | |
| DocumentReference_PrDocumentNumber | 7 | № заявления | confirmed_document | |
| DocumentReference_PrDocumentDate | 2025-11-28 | дата | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | плательщик | confirmed_document | |
| Payer_INN | 1650389298 | ИНН | confirmed_document | |
| Payer_KPP | 165001001 | КПП | confirmed_operator | |
| Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) 044525411 | банк плательщика | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | получатель | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX | банк получателя | confirmed_document | |

### PayerSign
| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname | Саранов | фамилия | confirmed_operator | решение оператора |
| PersonName | Дмитрий | имя | confirmed_operator | решение оператора |

---

## document: Service Invoice
- uqi_prefix: formalized.service_invoice_1
- xml_target_root: AltaServiceInvoice
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\Счет_№26-00378-tl_от_27-01-2026.md
- file_name: Счет_№26-00378-tl_от_27-01-2026.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentSign | 1 | системный признак | confirmed_operator | |
| TotalServiceCost | 2700.00 | итого | confirmed_document | |
| Currency | USD | валюта | confirmed_document | |
| ServiceProvider_Name | ООО «Трансимпериал» | исполнитель | confirmed_document | |

### ServiceProvider_PaymentRequisitions
| field | value | description | status | note |
|---|---|---|---|---|
| BankName | АО "Райффайзенбанк" | банк | confirmed_document | |

| field | value | description | status | note |
|---|---|---|---|---|
| ContractDetails_PrDocumentNumber | КООО/26651/М | договор перевозки | confirmed_document | stable_source FreeDoc_КООО |
| ContractDetails_PrDocumentDate | 2025-05-13 | дата | confirmed_document | stable_source |

### PaymentDocument
| field | value | description | status | note |
|---|---|---|---|---|
| PrDocumentNumber | 26-00378-tl | номер | confirmed_operator | эталон выгрузки |
| PrDocumentDate | 2026-01-27 | дата | confirmed_document | |

| field | value | description | status | note |
|---|---|---|---|---|
| Registration_PrDocumentName | Инвойс (счет-фактура) за перевозку/погрузку | наименование | confirmed_operator | эталон выгрузки |
| Registration_PrDocumentNumber | 26-00378-tl | номер | confirmed_document | |
| Registration_PrDocumentDate | 2026-01-27 | дата | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | грузоотправитель | confirmed_operator | решение: =seller |
| Consignee_OrganizationName | ООО «СКИФ» | грузополучатель | confirmed_document | |
| Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_RFOrganizationFeatures_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_RFOrganizationFeatures_KPP | 165001001 | КПП | confirmed_document | |

### Consignee_SubjectAddressDetails
| field | value | description | status | note |
|---|---|---|---|---|
| PostalCode | 423800 | индекс | confirmed_document | |
| CountryCode | RU | страна | confirmed_document | |
| CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| StreetHouse | ПРОЕЗД ХЛЕБНЫЙ | улица | confirmed_document | |
| House | 30 | дом | confirmed_operator | |
| Room | 211 | офис | confirmed_operator | |

### ServiceDescription_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | описание услуги | confirmed_document | |
| CurrencyCode | USD | валюта строки | confirmed_document | |
| ServiceName | ОТСУТСТВУЕТ | наименование | confirmed_operator | |
| TaxRate | 0.00 | ставка | confirmed_operator | эталон 0.00 |
| TaxSum | 0.00 | сумма налога | confirmed_operator | |
| ServiceCost_Amount | 1404.00 | стоимость | confirmed_document | |
| ServiceCost_Currency | USD | валюта | confirmed_document | |

### ServiceDescription_2
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | описание | confirmed_document | |
| CurrencyCode | USD | валюта | confirmed_document | |
| ServiceName | ОТСУТСТВУЕТ | наименование | confirmed_operator | |
| TaxRate | 0.00 | ставка | confirmed_operator | |
| TaxSum | 0.00 | налог | confirmed_operator | |
| ServiceCost_Amount | 1296.00 | стоимость | confirmed_document | |
| ServiceCost_Currency | USD | валюта | confirmed_document | |

| field | value | description | status | note |
|---|---|---|---|---|
| Signature_Choice | 1 | вариант подписи | confirmed_operator | решение оператора |
| SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | директор | confirmed_document | |
| SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | инициалы | confirmed_document | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно О.А. | бухгалтер | confirmed_document | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | инициалы | confirmed_document | |

---

## document: Insurance Document
- uqi_prefix: formalized.insurance_document_1
- xml_target_root: AltaFreeDoc
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\Счет_№26-00378-tl_1_от_14-01-2026.md
- file_name: Счет_№26-00378-tl_1_от_14-01-2026.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04111 | код | confirmed_document | эталон выгрузки |
| DocumentHead_DocumentName | СЧЕТ ЗА СТРАХОВКУ | наименование | confirmed_operator | эталон выгрузки |
| DocumentHead_DocumentDate | 2026-01-14 | дата | confirmed_document | |
| DocumentHead_DocumentNumber | 26-00378-TL/1 | номер | confirmed_document | |

### DocumentBody_TextSection
#### TextPara_1
| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\Счет_№26-00378-tl_1_от_14-01-2026.pdf | текст | confirmed_operator | решение оператора: хранить как link |

---

## document: TechDescription
- uqi_prefix: formalized.tech_description_1
- xml_target_root: AltaFreeDoc
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\техничка Антикот, антипыльца антимошка.md
- file_name: техничка Антикот, антипыльца антимошка.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 05999 | код | confirmed_document | |
| DocumentHead_DocumentName | Технические характеристики | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2025-10-30 | дата | confirmed_operator | defaults |
| DocumentHead_DocumentNumber | Б/Н | номер | confirmed_operator | defaults |

### DocumentBody_TextSection
#### TextPara_1
| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\техничка Антикот, антипыльца антимошка .pdf | текст | confirmed_document | хранить как link |

---

# II. non_formalized

## document: Storage report (ДО-1)
- uqi_prefix: non_formalized.svh_1
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\ДО 14431420260204161621.md
- file_name: ДО 14431420260204161621.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| number | 0000080 | № отчета | confirmed_document | |
| date | 2026-02-03 | дата | confirmed_document | |
| warehouse_license_number | 10404/141210/10092/5 | лицензия СВХ | confirmed_document | |
| warehouse_license_date | 2025-09-18 | дата лицензии | confirmed_document | в CMR/ДО указано действует с 18.09.2025 |
| actual_gross_weight | 3500 | фактический вес брутто | confirmed_document | итого по ДО: 3500 |
| actual_places | 127 | фактические места | confirmed_document | |
| transport_reg_number | О157АО774/ВТ374974 | ТС | confirmed_document | |

#### goods_1
| field | value | description | status | note |
|---|---|---|---|---|
| tnved | 7019900095 | код | confirmed_document | |
| places | 27 | места | confirmed_document | |
| gross_weight_kg | 1710 | брутто | confirmed_document | |
| cost | 42228 | стоимость | confirmed_document | |
| currency_code | CNY | валюта | confirmed_document | |

#### goods_2
| field | value | description | status | note |
|---|---|---|---|---|
| tnved | 5804101000 | код | confirmed_document | |
| places | 100 | места | confirmed_document | |
| gross_weight_kg | 1790 | брутто | confirmed_document | |
| cost | 55032 | стоимость | confirmed_document | |
| currency_code | CNY | валюта | confirmed_document | |

---

## document: Storage report additional sheet
- uqi_prefix: non_formalized.svh_additional_sheet_1
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\ДО доп 14431520260204161645.md
- file_name: ДО доп 14431520260204161645.md
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| number | 1 | № доп листа | confirmed_document | |
| date | 2026-02-03 | дата | confirmed_document | |
| actual_gross_weight | 3500 | вес | confirmed_document | итого |
| actual_places | 127 | места | confirmed_document | итого |
| transport_reg_number | О157АО774/ВТ374974 | ТС | pending | в тексте доплиста не указано |
| svh_address_region | Республика Татарстан | регион СВХ | confirmed_document | из CMR п.13 |
| svh_address_city | Набережные Челны | город СВХ | confirmed_document | |
| svh_address_street_house | Производственный пр-д, д. 45 | улица/дом | confirmed_document | CMR п.13 |
| svh_customs_code | 10404083 | код таможни | confirmed_document | CMR п.13 |

---

## document: Transit Declaration
- uqi_prefix: non_formalized.td_1
- path: alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\md\\ТД 10719110_240126_5011363_reg00378тд.md
- file_name: ТД 10719110_240126_5011363_reg00378тд.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| number | 10719110/240126/5011363 | № ТД | confirmed_document | |
| date | 2026-01-24 | дата | confirmed_document | выпуск 24.01.2026 |
| customs_post_code | 10719110 | код поста | confirmed_document | т/п МАПП Забайкальск |
| customs_post_name | т/п МАПП Забайкальск | наименование | confirmed_document | |
| transport_reg_number | О157АО774/ВТ374974 | ТС | confirmed_document | |

---

## document: Master data
- uqi_prefix: non_formalized.master_data_1
- path: alta\\stable_source\\FreeDoc_ЮЭ9965-25-106893283.xml; alta\\stable_source\\Passport_63_09_449948.xml; alta\\stable_source\\LetterOfAttorney_1.xml
- file_name: stable_source bundle
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| declarant_name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | декларант | confirmed_document | EGRUL |
| declarant_ogrn | 1201600020390 | ОГРН | confirmed_document | |
| declarant_inn | 1650389298 | ИНН | confirmed_document | |
| declarant_kpp | 165001001 | КПП | confirmed_document | |
| declarant_address_postal_code | 423800 | индекс | confirmed_document | |
| declarant_address_country_code | RU | страна | confirmed_document | |
| declarant_address_country_name | РОССИЯ | страна | confirmed_document | |
| declarant_address_region | РЕСПУБЛИКА ТАТАРСТАН (ТАТАРСТАН) | регион | confirmed_document | EGRUL |
| declarant_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| declarant_address_street | ПР-Д ХЛЕБНЫЙ | улица | confirmed_document | EGRUL |
| declarant_address_building | 30 | дом | confirmed_document | EGRUL |
| declarant_address_room | 211 | офис | confirmed_document | |
| declarant_phone | +7 937 779-26-56 | телефон | confirmed_document | contract/stable_source транспортный договор |
| declarant_email |  | email | confirmed_operator | решение оператора: пусто |
| representative_last_name | АРБУЗОВА | фамилия | confirmed_document | passport/loa |
| representative_first_name | АНAСТАСИЯ | имя | confirmed_document | passport |
| representative_middle_name | КОНСТАНТИНОВНА | отчество | confirmed_document | passport |
| representative_position | УПОЛНОМОЧЕННОЕ ЛИЦО | должность | confirmed_document | loa |
| representative_phone | +7-927-030-70-07 | телефон | confirmed_document | loa subject |
| representative_email |  | email | confirmed_operator | решение оператора: пусто |
| representative_passport_code | RU01001 | код | confirmed_document | loa |
| representative_passport_name | ПАСРФ | наименование | confirmed_document | |
| representative_passport_series | 63 09 | серия | confirmed_document | |
| representative_passport_number | 449948 | номер | confirmed_document | |
| representative_passport_date | 2010-03-11 | дата выдачи | confirmed_document | |
| representative_passport_issuer | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | кем выдан | confirmed_document | |
| representative_authority_doc_name | ДОВЕРЕННОСТЬ | документ | confirmed_document | |
| representative_authority_doc_number | 1 | № | confirmed_document | |
| representative_authority_doc_date_from | 2026-02-01 | дата начала | confirmed_document | |
| representative_authority_doc_date_to | 2026-12-31 | дата окончания | confirmed_document | |
| note | источники: stable_source (EGRUL, Passport, LetterOfAttorney) | примечание | confirmed_document | |

---

# III. Нерешенные вопросы
- нет
