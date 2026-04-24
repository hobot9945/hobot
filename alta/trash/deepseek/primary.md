# Метаданные

- название кейса: МоскитнаяСетка
- путь к папке поставки: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- тип поставки: 1 инвойс / 1 упаковочный / 1 CMR / 1 ДО-1 / 1 доплист ДО-1 / 1 ТД / 2 платежных документа / 2 счета / 2 техописания
- примечание: структура товаров целевая для этапов 2–3 = 7 строк как в invoice/packing list (решение оператора в operator_provided_data.md)

---

# I. formalized

## document: Contract (03011)
- uqi_prefix: formalized.contract_1
- xml_target_root: AltaE2CONT
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md
- file_name: SALES CONTRACT NoLM-2553.md
- status: confirmed
- note: В контракте сумма 41904.30 RMB, в доп. соглашении №1 сумма изменена на 270000.00 RMB (см. supplementary_contract_1). Реквизиты РФ-стороны (ОГРН/ИНН/КПП/адрес) дополнены из ЕГРЮЛ (stable_source) как мастер-данные.

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 03011 | код вида документа | confirmed_document | константа |
| ContractRegistration_PrDocumentNumber | LM-2553 | № контракта | confirmed_document | |
| ContractRegistration_PrDocumentDate | 2025-07-02 | дата контракта | confirmed_document | |
| ContractTerms_Amount | 41904.30 | общая сумма контракта | confirmed_document | из контракта |
| ContractTerms_CurrencyCode | 156 | цифровой код валюты | confirmed_operator | operator_provided_data.md |
| ContractTerms_LastDate | 2026-12-31 | срок действия/исполнения | confirmed_document | shipment period end |
| ContractTerms_OtherTerms | EXW HEBEI | условия поставки / Incoterms | confirmed_operator | решение оператора |
| ContractTerms_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md | текст контракта (link) | confirmed_document | |
| ContractTerms_DealSign | 1 | системный признак Альты | confirmed_operator | решение оператора |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | продавец | confirmed_document | |
| ForeignPerson_Address_CountryCode | CN | страна продавца (alpha-2) | confirmed_document | нормализация из “China” по cb:country |
| ForeignPerson_Address_CounryName | Китай | страна продавца (текст) | confirmed_document | |
| ForeignPerson_Address_Region | Hebei | регион | confirmed_document | |
| ForeignPerson_Address_City | Shijiazhuang | город | confirmed_document | |
| ForeignPerson_Address_StreetHouse | No. 5 Gaodong street. Shijiazhuang. Hebei China | улица/дом одной строкой | confirmed_document | |
| RussianPerson_OrganizationName | ООО «СКИФ» | покупатель | confirmed_document | |
| RussianPerson_OGRN | 1201600020390 | ОГРН | confirmed_document | из ЕГРЮЛ (stable_source) |
| RussianPerson_INN | 1650389298 | ИНН | confirmed_document | из ЕГРЮЛ (stable_source) |
| RussianPerson_KPP | 165001001 | КПП | confirmed_document | из ЕГРЮЛ (stable_source) |
| RussianPerson_Address_PostalCode | 423800 | индекс | confirmed_document | из ЕГРЮЛ (stable_source) |
| RussianPerson_Address_CountryCode | RU | страна (alpha-2) | confirmed_document | |
| RussianPerson_Address_CounryName | Россия | страна (текст) | confirmed_document | |
| RussianPerson_Address_Region | Республика Татарстан | регион | confirmed_document | из ЕГРЮЛ (stable_source) |
| RussianPerson_Address_City | Набережные Челны | город | confirmed_document | из ЕГРЮЛ (stable_source) |
| RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | улица/дом/офис одной строкой | confirmed_document | из ЕГРЮЛ (stable_source) |

---

## document: Supplementary Contract (03012)
- uqi_prefix: formalized.supplementary_contract_1
- xml_target_root: AltaSupplementaryContract
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md
- file_name: 1 Supplementary agreement to the contract.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentNumber | 1 | № доп. соглашения | confirmed_document | |
| IssueDate | 2025-11-25 | дата доп. соглашения | confirmed_document | |
| ContractDescription_Amount | 270000.00 | новая сумма контракта | confirmed_document | |
| ContractDescription_CurrencyCode | 156 | цифровой код валюты | confirmed_operator | operator_provided_data.md |
| ContractDescription_LastDate | 2026-12-31 | новый срок действия/исполнения | confirmed_operator | operator_provided_data.md |
| ContractDescription_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md | текст (link) | confirmed_document | |
| ContractDescription_DealSign | 1 | системный признак | confirmed_operator | |
| ContractDescription_StockCategorySign | 0 | системный признак | confirmed_operator | |
| ContractDescription_BuyerLimitationSign | 0 | системный признак | confirmed_operator | |
| ContractDescription_InsuranceSign | 0 | системный признак | confirmed_operator | |
| RussianPerson_OrganizationName | ООО «СКИФ» | российская сторона | confirmed_document | |
| RussianPerson_ShortName | ООО «СКИФ» | краткое наименование | confirmed_document | из ЕГРЮЛ (stable_source) |
| RussianPerson_OGRN | 1201600020390 | ОГРН | confirmed_document | из ЕГРЮЛ |
| RussianPerson_INN | 1650389298 | ИНН | confirmed_document | |
| RussianPerson_KPP | 165001001 | КПП | confirmed_document | |
| RussianPerson_Address_PostalCode | 423800 | индекс | confirmed_document | |
| RussianPerson_Address_CountryCode | RU | страна | confirmed_document | |
| RussianPerson_Address_CounryName | Россия | страна (текст) | confirmed_document | |
| RussianPerson_Address_Region | Республика Татарстан | регион | confirmed_document | |
| RussianPerson_Address_City | Набережные Челны | город | confirmed_document | |
| RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | улица/дом/офис | confirmed_document | |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | иностранная сторона | confirmed_document | |
| ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | краткое наименование | confirmed_operator | operator_provided_data.md |
| ForeignPerson_Address_CountryCode | CN | страна | confirmed_document | |
| ForeignPerson_Address_CounryName | Китай | страна (текст) | confirmed_document | |
| ForeignPerson_Address_Region | Hebei | регион | confirmed_document | |
| ForeignPerson_Address_City | Shijiazhuang | город | confirmed_document | |
| ForeignPerson_Address_StreetHouse | No. 5 Gaodong street. Shijiazhuang. Hebei China | улица/дом | confirmed_document | |

#### ContractSignedPerson

| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname | Li | фамилия подписанта | confirmed_operator | operator_provided_data.md |
| PersonName | Jing | имя подписанта | confirmed_operator | operator_provided_data.md |
| PersonMiddleName |  | отчество подписанта | confirmed_operator | оператор: пусто |

---

## document: Invoice (04021)
- uqi_prefix: formalized.invoice_1
- xml_target_root: AltaE2I
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\CL на сетку .md
- file_name: CL на сетку .md
- status: confirmed
- note: В инвойсе у Buyer указано два адреса (423800 Татарстан и 185001 Карелия). В формализуемых полях использован адрес 423800 (совпадает с ЕГРЮЛ/прочими документами).

| field | value | description | status | note |
|---|---|---|---|---|
| CurrencyRate | 10.9430 | курс валюты | confirmed_operator | operator_provided_data.md |
| CurrencyCode | CNY | валюта инвойса | confirmed_operator | решение оператора |
| DocumentCode | 04021 | код вида документа | confirmed_document | константа |
| PlacesQuantity | 127 | кол-во грузовых мест | confirmed_document | |
| PlacesDescription | Поддон | описание мест | confirmed_operator | operator_provided_data.md |
| GrossWeightQuantity | 3500.00 | общий вес брутто | confirmed_operator | из PL totals (operator_provided_data.md) |
| NetWeightQuantity | 3302.00 | общий вес нетто | confirmed_operator | из PL totals (operator_provided_data.md) |
| GCost | 97260.00 | системное поле | confirmed_operator | решение оператора: =TotalCost |
| TotalCost | 97260.00 | итого по инвойсу | confirmed_document | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | место поставки | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | числовой код условий | confirmed_operator | решение оператора |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия поставки | confirmed_operator | решение оператора |
| DeliveryTerms_DispatchCountryCode | CN | страна отправления | confirmed_operator | operator_provided_data.md |
| DeliveryTerms_TradingCountryCode | CN | торгующая страна | confirmed_operator | решение оператора |
| DeliveryTerms_DestinationCountryCode | RU | страна назначения | confirmed_operator | operator_provided_data.md |
| Registration_PrDocumentName | Commercial invoice | наименование документа | confirmed_document | нормализация |
| Registration_PrDocumentNumber | LM-2591 | номер инвойса | confirmed_document | |
| Registration_PrDocumentDate | 2025-10-30 | дата инвойса | confirmed_document | |
| Contract_PrDocumentNumber | LM-2553 | № контракта-ссылки | confirmed_document | |
| Contract_PrDocumentDate | 2025-07-02 | дата контракта-ссылки | confirmed_document | |
| Buyer_CompanyID | 1650389298 | ИНН покупателя | confirmed_document | из ЕГРЮЛ (stable_source) |
| Buyer_KPPCode | 165001001 | КПП покупателя | confirmed_document | из ЕГРЮЛ (stable_source) |
| Buyer_Name | LLC «SKIF» | наименование покупателя | confirmed_document | |
| Buyer_PostalAddress_PostalCode | 423800 | индекс | confirmed_document | из ЕГРЮЛ (stable_source) |
| Buyer_PostalAddress_CountryCode | RU | страна | confirmed_document | |
| Buyer_PostalAddress_CounryName | Россия | страна (текст) | confirmed_document | |
| Buyer_PostalAddress_Region | Republic of Tatarstan | регион | confirmed_document | как в инвойсе |
| Buyer_PostalAddress_City | Naberezhnye Chelny | город | confirmed_document | |
| Buyer_PostalAddress_StreetHouse | Khlebny Passage, hause 30, office 211 | улица/дом/офис | confirmed_document | |
| Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | продавец | confirmed_document | |
| Seler_PostalAddress_CountryCode | CN | страна | confirmed_document | |
| Seler_PostalAddress_CounryName | Китай | страна (текст) | confirmed_document | |
| Seler_PostalAddress_Region | Hebei | регион | confirmed_document | |
| Seler_PostalAddress_City | Shijiazhuang | город | confirmed_document | |
| Seler_PostalAddress_StreetHouse | No. 5 Gaodong street. Shijiazhuang. Hebei China | улица/дом | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | грузоотправитель | confirmed_operator | operator: consignor_equals_seller=true |
| Consignor_Address_CountryCode | CN | страна | confirmed_operator | |
| Consignor_Address_CounryName | Китай | страна (текст) | confirmed_operator | |
| Consignor_Address_Region | Hebei | регион | confirmed_operator | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_operator | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street. Shijiazhuang. Hebei China | улица/дом | confirmed_operator | |
| Consignee_OrganizationName | LLC «SKIF» | грузополучатель | confirmed_operator | operator: consignee_equals_buyer=true |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | из ЕГРЮЛ (stable_source) |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | Россия | страна (текст) | confirmed_document | |
| Consignee_Address_Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_Address_City | Набережные Челны | город | confirmed_document | |
| Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | улица/дом/офис | confirmed_document | |

#### InvoiceGoods_1

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | описание товара | confirmed_document | |
| GoodsQuantity | 60 | кол-во (sets/rolls) | confirmed_document | |
| goods_supplementary_quantity | 2520 | количество в м² | confirmed_document | |
| goods_supplementary_uom_name | м² | наименование доп. ед. изм. | confirmed_document | cb:unit 055 |
| MeasureUnitQualifierName | м² | доп.ед.изм для ДТ | confirmed_document | |
| GrossWeightQuantity | 855.00 | брутто по строке | confirmed_operator | из PL (operator_provided_data.md) |
| NetWeightQuantity | 806.60 | нетто по строке | confirmed_operator | |
| Price | 5.85 | цена за м² | confirmed_document | |
| TotalCost | 14742 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | цифровой код страны происхождения | confirmed_operator | operator_provided_data.md |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | operator_provided_data.md |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | товарная марка/ТМ | confirmed_operator | решение оператора |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | решение оператора |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель/модификация | confirmed_operator | решение оператора |

#### InvoiceGoods_2

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Anti-cat mesh Roll size 1.6 *30 / Москитная сетка «Антикот» Размер рулона 1,6*30 | описание | confirmed_document | |
| GoodsQuantity | 30 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 1440 | м² | confirmed_document | |
| goods_supplementary_uom_name | м² | ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м² | доп.ед.изм | confirmed_document | |
| GrossWeightQuantity | 490.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 460.80 | нетто | confirmed_operator | |
| Price | 5.85 | цена | confirmed_document | |
| TotalCost | 8424 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | страна происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_3

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 / Сетка против пыльцы «Антипыльца» из полиэстера. Размер рулона 1,4*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 60 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 2520 | м² | confirmed_document | |
| goods_supplementary_uom_name | м² | ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м² | доп.ед.изм | confirmed_document | |
| GrossWeightQuantity | 265.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 252.00 | нетто | confirmed_operator | |
| Price | 6.35 | цена | confirmed_document | |
| TotalCost | 16002 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | страна происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_4

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,6*30 M2 / Сетка против пыльцы «Антипыльца» из полиэстера. Размер рулона 1,6*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 30 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 1440 | м² | confirmed_document | |
| goods_supplementary_uom_name | м² | ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м² | доп.ед.изм | confirmed_document | |
| GrossWeightQuantity | 155.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 144.00 | нетто | confirmed_operator | |
| Price | 6.35 | цена | confirmed_document | |
| TotalCost | 9144 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | страна происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_5

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 7019900095 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 / Сетка среднего размера «Антимошка» из стекловолокна. Размер рулона 1,4*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 90 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 3780 | м² | confirmed_document | |
| goods_supplementary_uom_name | м² | ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м² | доп.ед.изм | confirmed_document | |
| GrossWeightQuantity | 520.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 491.40 | нетто | confirmed_operator | |
| Price | 3.4 | цена | confirmed_document | |
| TotalCost | 12852 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | страна происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_6

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 7019900095 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 / Сетка среднего размера «Антимошка» из стекловолокна. Размер рулона 1,6*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 180 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 8640 | м² | confirmed_document | |
| goods_supplementary_uom_name | м² | ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м² | доп.ед.изм | confirmed_document | |
| GrossWeightQuantity | 1190.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 1123.20 | нетто | confirmed_operator | |
| Price | 3.4 | цена | confirmed_document | |
| TotalCost | 29376 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | страна происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_7

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 / Трехслойные сетки «Антипыльца» из полиэстера. Размер рулона 1,6*30 M2 | описание | confirmed_document | |
| GoodsQuantity | 5 | кол-во | confirmed_document | |
| goods_supplementary_quantity | 240 | м² | confirmed_document | |
| goods_supplementary_uom_name | м² | ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м² | доп.ед.изм | confirmed_document | |
| GrossWeightQuantity | 25.00 | брутто | confirmed_operator | |
| NetWeightQuantity | 24.00 | нетто | confirmed_operator | |
| Price | 28 | цена | confirmed_document | |
| TotalCost | 6720 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | страна происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

---

## document: Packing List (04131)
- uqi_prefix: formalized.packing_list_1
- xml_target_root: AltaE2PACK
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\PL на сетку .md
- file_name: PL на сетку .md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| GrossWeightQuantity | 3500 | общий брутто | confirmed_document | |
| NetWeightQuantity | 3302 | общий нетто | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | грузоотправитель | confirmed_document | |
| Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | краткое | confirmed_operator | operator_provided_data.md |
| Consignor_Address_CountryCode | CN | страна | confirmed_document | |
| Consignor_Address_CounryName | Китай | страна (текст) | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street. Shijiazhuang. Hebei China | улица/дом | confirmed_document | |
| Consignee_OrganizationName | LLC «SKIF» | грузополучатель | confirmed_document | |
| Consignee_ShortName | LLC «SKIF» | краткое | confirmed_operator | operator_provided_data.md |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | из ЕГРЮЛ (stable_source) |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | Россия | страна (текст) | confirmed_document | |
| Consignee_Address_Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_Address_City | Набережные Челны | город | confirmed_document | |
| Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | улица/дом/офис | confirmed_document | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | место поставки | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | числовой код условий | confirmed_operator | решение оператора |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия поставки | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentName | Sales contract | наименование контракта | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | № контракта | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentDate | 2025-07-02 | дата контракта | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentName | Invoice | наименование инвойса | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | № инвойса | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentDate | 2025-10-30 | дата инвойса | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentName | Packing list | наименование упаковочного | confirmed_operator | operator_provided_data.md |
| DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | № упаковочного | confirmed_operator | operator_provided_data.md |
| DeliveryTerms_Registration_PrDocumentDate | 2025-10-30 | дата упаковочного | confirmed_operator | operator_provided_data.md |
| registration_doc_name | Упаковочный лист | для графы 44 | confirmed_operator | |
| registration_doc_number | LM-2591 | № документа | confirmed_operator | |
| registration_doc_date | 2025-10-30 | дата документа | confirmed_operator | |

#### Goods_1

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Anti-cat mesh. Roll size 1.4 * 0.16 * 0.16 Material: polyester / Москитная сетка «Антикот» | описание строки | confirmed_document | |
| GoodsQuantity | 60 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 855 | брутто | confirmed_document | |
| NetWeightQuantity | 806.6 | нетто | confirmed_document | |

##### PackingInfo

| field | value | description | status | note |
|---|---|---|---|---|
| PakingQuantity | 60 | кол-во упаковок/мест | confirmed_operator | решение оператора: =GoodsQuantity |

#### Goods_2

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Anti-cat mesh Roll size 1.6 * 0.16 * 0.16 / Москитная сетка «Антикот» | описание | confirmed_document | |
| GoodsQuantity | 30 | места | confirmed_document | |
| GrossWeightQuantity | 490 | брутто | confirmed_document | |
| NetWeightQuantity | 460.8 | нетто | confirmed_document | |

##### PackingInfo

| field | value | description | status | note |
|---|---|---|---|---|
| PakingQuantity | 30 | кол-во упаковок | confirmed_operator | |

#### Goods_3

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | ANTI-POLLEN MESH. Material: polyester / Сетка против пыльцы «Антипыльца» | описание | confirmed_document | |
| GoodsQuantity | 6 | места | confirmed_document | |
| GrossWeightQuantity | 265 | брутто | confirmed_document | |
| NetWeightQuantity | 252 | нетто | confirmed_document | |

##### PackingInfo

| field | value | description | status | note |
|---|---|---|---|---|
| PakingQuantity | 6 | кол-во упаковок | confirmed_operator | |

#### Goods_4

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | ANTI-POLLEN MESH. Material: polyeste / Сетка против пыльцы «Антипыльца» | описание | confirmed_document | |
| GoodsQuantity | 3 | места | confirmed_document | |
| GrossWeightQuantity | 155 | брутто | confirmed_document | |
| NetWeightQuantity | 144 | нетто | confirmed_document | |

##### PackingInfo

| field | value | description | status | note |
|---|---|---|---|---|
| PakingQuantity | 3 | кол-во упаковок | confirmed_operator | |

#### Goods_5

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | MIDGE MEHS Material: Fiberglass / Сетка «Антимошка» | описание | confirmed_document | |
| GoodsQuantity | 9 | места | confirmed_document | |
| GrossWeightQuantity | 520 | брутто | confirmed_document | |
| NetWeightQuantity | 491.4 | нетто | confirmed_document | |

##### PackingInfo

| field | value | description | status | note |
|---|---|---|---|---|
| PakingQuantity | 9 | кол-во упаковок | confirmed_operator | |

#### Goods_6

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | MIDGE MESH Material: Fiberglass / Сетка «Антимошка» | описание | confirmed_document | |
| GoodsQuantity | 18 | места | confirmed_document | |
| GrossWeightQuantity | 1190 | брутто | confirmed_document | |
| NetWeightQuantity | 1123.2 | нетто | confirmed_document | |

##### PackingInfo

| field | value | description | status | note |
|---|---|---|---|---|
| PakingQuantity | 18 | кол-во упаковок | confirmed_operator | |

#### Goods_7

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | GRID WITH 3 LAYER made of polyester / Трехслойные сетки «Антипыльца» | описание | confirmed_document | |
| GoodsQuantity | 1 | места | confirmed_document | |
| GrossWeightQuantity | 25 | брутто | confirmed_document | |
| NetWeightQuantity | 24 | нетто | confirmed_document | |

##### PackingInfo

| field | value | description | status | note |
|---|---|---|---|---|
| PakingQuantity | 1 | кол-во упаковок | confirmed_operator | |

#### TransportMeans_1

| field | value | description | status | note |
|---|---|---|---|---|
| Number | О157АО774 | регистрационный номер | confirmed_operator | operator_provided_data.md (кириллица) |
| ModeCode | 31 | код вида транспорта | confirmed_operator | |
| NationalityCode | 000 | национальность | confirmed_operator | |
| MoverIndicator | true | тягач | confirmed_operator | |

#### TransportMeans_2

| field | value | description | status | note |
|---|---|---|---|---|
| Number | ВТ374974 | регистрационный номер | confirmed_operator | operator_provided_data.md (кириллица) |
| ModeCode | 31 | код вида транспорта | confirmed_operator | |
| NationalityCode | 000 | национальность | confirmed_operator | |
| MoverIndicator | false | прицеп | confirmed_operator | |

---

## document: CMR (02015)
- uqi_prefix: formalized.cmr_1
- xml_target_root: AltaE3CMR
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\СМР от СВХ.md
- file_name: СМР от СВХ.md
- status: pending
- note: md-версия CMR содержит ограниченный набор полей; часть реквизитов заполнена по решениям оператора из operator_provided_data.md. Поля, отсутствующие в CMR/не подтвержденные оператором, оставлены pending без подстановок из других документов.

| field | value | description | status | note |
|---|---|---|---|---|
| LanguageCode | RU | язык документа | confirmed_operator | operator_provided_data.md |
| CMR_Choice | 1 | системный выбор | confirmed_operator | operator_provided_data.md |
| RegistrationDocument_RegID | 00378 | номер CMR | confirmed_document | |
| RegistrationDocument_DateInf | 2026-01-20 | дата CMR | confirmed_document | |
| RegistrationDocument_Place | Маньчжурия | место составления | confirmed_operator | operator_provided_data.md |
| TrakingCargo_TakingCargoDate |  | дата принятия груза | pending | в CMR md не указано явно |
| TrakingCargo_TakingCargoPlace_CountryCode | CN | страна принятия груза | confirmed_operator | решение оператора (в CMR md место приема указано как СВХ, это конфликт) |
| TrakingCargo_TakingCargoPlace_CounryName | Китай | страна принятия груза (текст) | confirmed_operator | |
| DeliveryPlace_CountryCode | RU | страна доставки | confirmed_operator | решение оператора |
| DeliveryPlace_CounryName | Россия | страна доставки (текст) | confirmed_operator | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | место поставки | confirmed_operator | operator_provided_data.md |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия поставки | confirmed_operator | operator_provided_data.md |
| GoodsQuantity | 127 | кол-во мест | confirmed_document | |
| CMRGoodsWeight_GrossWeightQuantity | 3500.00 | общий брутто | confirmed_document | |
| CMRTransport_PrimeMoverStateSignID | O157A0774 | номер тягача | confirmed_document | из ДО-1/ТД (латиница) |
| CMRTransport_TrailerStateSignID | BT374974 | номер прицепа | confirmed_document | из ДО-1 (латиница) |
| Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | отправитель | confirmed_document | |
| Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | краткое | confirmed_operator | operator_provided_data.md |
| Consignor_PostalAddress_CountryCode |  | страна | pending | не указано в CMR md |
| Consignor_Address_CounryName |  | страна (текст) | pending | |
| Consignor_Address_Region |  | регион | pending | |
| Consignor_Address_City |  | город | pending | |
| Consignor_Address_StreetHouse |  | улица/дом | pending | |
| Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | гарант | confirmed_operator | решение оператора |
| Consignor_Guarantee_ShortName | ОТСУТТВУЕТ | гарант | confirmed_operator | решение оператора (см. operator_provided_data.md) |
| Consignor_Guarantee_Address_CountryCode |  | страна | confirmed_operator | гарант отсутствует |
| Consignor_Guarantee_Address_CounryName |  | страна | confirmed_operator | |
| Consignor_Guarantee_Address_Region |  | регион | confirmed_operator | |
| Consignor_Guarantee_Address_City |  | город | confirmed_operator | |
| Consignor_Guarantee_Address_StreetHouse |  | улица/дом | confirmed_operator | |
| Consignee_NameInf | ООО «Скиф» | получатель | confirmed_document | |
| Consignee_ShortName | ООО «Скиф» | краткое | confirmed_operator | operator_provided_data.md |
| Consignee_OGRNID | 1201600020390 | ОГРН | confirmed_document | из ЕГРЮЛ (stable_source) |
| Consignee_INNID | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPPCode | 165001001 | КПП | confirmed_document | |
| Consignee_PostalAddress_PostalCode | 423800 | индекс | confirmed_document | из CMR md |
| Consignee_PostalAddress_CountryCode | RU | страна | confirmed_document | нормализация |
| Consignee_Address_CounryName | Россия | страна (текст) | confirmed_document | |
| Consignee_Address_Region | Республика Татарстан | регион | confirmed_document | из CMR md |
| Consignee_Address_City | Набережные Челны | город | confirmed_document | из CMR md |
| Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | улица/дом/офис | confirmed_document | из CMR md |

#### CMRGoods_1

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsNumeric | 1 | номер строки | confirmed_operator | operator_provided_data.md |
| GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | описание | confirmed_document | |
| GoodsNomenclatureCode | 5804101000 | код (по смыслу ТН ВЭД) | confirmed_operator | operator_provided_data.md |
| GoodsQuantity | 127 | места | confirmed_document | |
| GrossWeightQuantity | 3500.00 | брутто | confirmed_document | |

##### GoodsPackingInfo

| field | value | description | status | note |
|---|---|---|---|---|
| PackingCode | PX | код упаковки | confirmed_operator | operator_provided_data.md |
| PakingQuantity | 127 | кол-во упаковок | confirmed_operator | operator_provided_data.md |
| PackingDescription | ПОДДОН | описание упаковки | confirmed_operator | operator_provided_data.md |

---

## document: Payment Order (04023)
- uqi_prefix: formalized.payment_order_1
- xml_target_root: AltaPaymentOrder
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_7_28.11.2025.md
- file_name: currency_transfer_7_28.11.2025.md
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04023 | код вида документа | confirmed_operator | константа (решение оператора) |
| PaymentModeCode | 0 | системный код | confirmed_operator | operator_provided_data.md |
| PaymentAmount | 34041.00 | сумма платежа | confirmed_document | |
| TransactionKind | 01 | вид операции | confirmed_operator | operator_provided_data.md |
| Priority | 5 | очередность | confirmed_operator | решение оператора |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553, DATE:JULY 02.2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | назначение | confirmed_document | |
| ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | сумма прописью | confirmed_document | |
| DocumentReference_PrDocumentNumber | 7 | номер заявления на перевод | confirmed_document | |
| DocumentReference_PrDocumentDate | 2025-11-28 | дата заявления | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | плательщик | confirmed_document | |
| Payer_INN | 1650389298 | ИНН | confirmed_document | |
| Payer_KPP | 165001001 | КПП | confirmed_operator | operator_provided_data.md |
| Payer_Bank_BankName | Филиал «Центральный» Банка ВТБ (ПАО) 044525411 | банк плательщика | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | получатель | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH SWIFT: VTBRCNSHXXX | банк получателя | confirmed_document | |

#### PayerSign

| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname |  | фамилия | pending | в md не указано |
| PersonName |  | имя/инициалы | pending | в md не указано |

---

## document: Payment Order (04023)
- uqi_prefix: formalized.payment_order_2
- xml_target_root: AltaPaymentOrder
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_1_13.01.2026.md
- file_name: currency_transfer_1_13.01.2026.md
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04023 | код вида документа | confirmed_operator | |
| PaymentModeCode | 0 | системный код | confirmed_operator | |
| PaymentAmount | 63219.00 | сумма | confirmed_document | |
| TransactionKind | 01 | вид операции | confirmed_operator | |
| Priority | 5 | очередность | confirmed_operator | |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553, DATE:JULY 02.2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | назначение | confirmed_document | |
| ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | сумма прописью | confirmed_document | |
| DocumentReference_PrDocumentNumber | 1 | номер заявления | confirmed_document | |
| DocumentReference_PrDocumentDate | 2026-01-13 | дата | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | плательщик | confirmed_document | |
| Payer_INN | 1650389298 | ИНН | confirmed_document | |
| Payer_KPP | 165001001 | КПП | confirmed_operator | |
| Payer_Bank_BankName | Филиал «Центральный» Банка ВТБ (ПАО) 044525411 | банк плательщика | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | получатель | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH SWIFT: VTBRCNSHXXX | банк получателя | confirmed_document | |

#### PayerSign

| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname |  | фамилия | pending | |
| PersonName |  | имя/инициалы | pending | |

---

## document: Service Invoice (04031)
- uqi_prefix: formalized.service_invoice_1
- xml_target_root: AltaServiceInvoice
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_от_27-01-2026.md
- file_name: Счет_№26-00378-tl_от_27-01-2026.md
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentSign | 1 | системный признак | confirmed_operator | operator_provided_data.md |
| TotalServiceCost | 2700.00 | итого | confirmed_document | |
| Currency | USD | валюта | confirmed_document | |
| ServiceProvider_Name | ООО «Трансмипериал» | исполнитель услуг | confirmed_document | как в счете (в договоре: ООО «Трансимпериал») |
| ContractDetails_PrDocumentNumber | КООО/26651/М | № договора | confirmed_document | |
| ContractDetails_PrDocumentDate | 2025-05-13 | дата договора | confirmed_document | stable_source FreeDoc_КООО_26651_М.xml |
| Registration_PrDocumentName | Счет на оплату | наименование счета | confirmed_document | |
| Registration_PrDocumentNumber | 26-00378-tl | номер счета | confirmed_document | |
| Registration_PrDocumentDate | 2026-01-27 | дата счета | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | грузоотправитель | confirmed_operator | operator: consignor_equals_seller=true |
| Consignee_OrganizationName | ООО «СКИФ» | грузополучатель | confirmed_document | |
| Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | ОГРН | confirmed_document | из ЕГРЮЛ |
| Consignee_RFOrganizationFeatures_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_RFOrganizationFeatures_KPP | 165001001 | КПП | confirmed_document | |
| Signature_Choice | 1 | вариант подписи | confirmed_operator | operator_provided_data.md |
| SignatureDirectorChiefAccountant_Director_PersonSurname |  | фамилия директора | pending | в счете не указано |
| SignatureDirectorChiefAccountant_Director_PersonName |  | имя/инициалы директора | pending | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname |  | фамилия бухгалтера | pending | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonName |  | имя/инициалы бухгалтера | pending | |

#### ServiceProvider_PaymentRequisitions

| field | value | description | status | note |
|---|---|---|---|---|
| BankName | АО «Райффайзенбанк», г. Москва; БИК 044525700; сч. 40702810400000233463 | банк исполнителя | confirmed_document | |

#### PaymentDocument

| field | value | description | status | note |
|---|---|---|---|---|
| PrDocumentNumber | ОТСУТСТВУЕТ | номер связанного документа | confirmed_operator | решение оператора |
| PrDocumentDate | ОТСУТСТВУЕТ | дата связанного документа | confirmed_operator | решение оператора |

#### Consignor_SubjectAddressDetails

| field | value | description | status | note |
|---|---|---|---|---|
| PostalCode |  | индекс | pending | в первичке не найден |
| CountryCode | CN | страна | confirmed_operator | по продавцу |
| CounryName | Китай | страна (текст) | confirmed_operator | |
| Region | Hebei | регион | confirmed_operator | |
| Town | Shijiazhuang | город | confirmed_operator | |
| StreetHouse | No. 5 Gaodong street | улица/дом | confirmed_operator | |

#### Consignee_SubjectAddressDetails

| field | value | description | status | note |
|---|---|---|---|---|
| PostalCode | 423800 | индекс | confirmed_document | |
| CountryCode | RU | страна | confirmed_document | |
| CounryName | Россия | страна (текст) | confirmed_document | |
| Region | Республика Татарстан | регион | confirmed_document | |
| Town | Набережные Челны | город | confirmed_document | |
| StreetHouse | проезд Хлебный | улица | confirmed_document | |
| House | 30 | дом | confirmed_operator | решение оператора |
| Room | 211 | офис | confirmed_operator | решение оператора |

#### ServiceDescription_1

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | описание услуги | confirmed_document | как в счете |
| CurrencyCode | USD | валюта строки | confirmed_document | |
| ServiceName | ОТСУТСТВУЕТ | наименование/маршрут | confirmed_operator | решение оператора |
| TaxRate | 0% | ставка | confirmed_document | |
| TaxSum | 0,00 | сумма налога | confirmed_document | |
| ServiceCost_Amount | 1404,00 | стоимость строки | confirmed_document | |
| ServiceCost_Currency | USD | валюта стоимости | confirmed_document | |

#### ServiceDescription_2

| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | описание услуги | confirmed_document | как в счете |
| CurrencyCode | USD | валюта | confirmed_document | |
| ServiceName | ОТСУТСТВУЕТ | наименование/маршрут | confirmed_operator | решение оператора |
| TaxRate | 0% | ставка | confirmed_document | |
| TaxSum | 0,00 | сумма | confirmed_document | |
| ServiceCost_Amount | 1296,00 | стоимость | confirmed_document | |
| ServiceCost_Currency | USD | валюта | confirmed_document | |

---

## document: Insurance Document (04111)
- uqi_prefix: formalized.insurance_document_1
- xml_target_root: AltaFreeDoc
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md
- file_name: Счет_№26-00378-tl_1_от_14-01-2026.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04111 | код вида документа | confirmed_document | константа |
| DocumentHead_DocumentName | Счет на оплату | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2026-01-14 | дата | confirmed_document | |
| DocumentHead_DocumentNumber | 26-00378-tl/1 | номер | confirmed_document | |

#### DocumentBody_TextSection

##### TextPara_1

| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md | текст (link) | confirmed_document | |

---

## document: TechDescription (05999)
- uqi_prefix: formalized.tech_description_1
- xml_target_root: AltaFreeDoc
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md
- file_name: техничка Антикот, антипыльца антимошка .md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 05999 | код вида документа | confirmed_document | константа |
| DocumentHead_DocumentName | Технические характеристики | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2025-10-30 | дата | confirmed_operator | operator_provided_data.md defaults |
| DocumentHead_DocumentNumber | Б/Н | номер | confirmed_operator | operator_provided_data.md defaults |

#### DocumentBody_TextSection

##### TextPara_1

| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md | текст (link) | confirmed_document | |

---

## document: TechDescription (05999)
- uqi_prefix: formalized.tech_description_2
- xml_target_root: AltaFreeDoc
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка .md
- file_name: техничка .md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 05999 | код вида документа | confirmed_document | константа |
| DocumentHead_DocumentName | Техническое описание товаров | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2025-10-30 | дата | confirmed_operator | operator_provided_data.md defaults |
| DocumentHead_DocumentNumber | Б/Н | номер | confirmed_operator | operator_provided_data.md defaults |

#### DocumentBody_TextSection

##### TextPara_1

| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка .md | текст (link) | confirmed_document | |

---

## document: EGRUL (04011)
- uqi_prefix: formalized.egrul_1
- xml_target_root: AltaFreeDoc
- full_path: alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml
- file_name: FreeDoc_ЮЭ9965-25-106893283.xml
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04011 | код вида документа | confirmed_document | константа |
| DocumentHead_DocumentName | ВЫПИСКА ИЗ ЕГРЮЛ | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2025-07-14 | дата | confirmed_document | |
| DocumentHead_DocumentNumber | ЮЭ9965-25-106893283 | номер | confirmed_document | |

#### DocumentBody_TextSection

##### TextPara_1

| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml | текст (link) | confirmed_document | cp1251 |

---

## document: Transport Contract (04033)
- uqi_prefix: formalized.transport_contract_1
- xml_target_root: AltaFreeDoc
- full_path: alta\stable_source\FreeDoc_КООО_26651_М.xml
- file_name: FreeDoc_КООО_26651_М.xml
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04033 | код вида документа | confirmed_document | константа |
| DocumentHead_DocumentName | ДОГОВОР ПО ПЕРЕВОЗКЕ | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2025-05-13 | дата | confirmed_document | |
| DocumentHead_DocumentNumber | КООО/26651/М | номер | confirmed_document | |

#### DocumentBody_TextSection

##### TextPara_1

| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\stable_source\FreeDoc_КООО_26651_М.xml | текст (link) | confirmed_document | cp1251 |

---

## document: Passport (11001)
- uqi_prefix: formalized.passport_1
- xml_target_root: AltaPassport
- full_path: alta\stable_source\Passport_63_09_449948.xml
- file_name: Passport_63_09_449948.xml
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| CardSeries | 63 09 | серия | confirmed_document | |
| CardNumber | 449948 | номер | confirmed_document | |
| OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | кем выдан | confirmed_document | |
| CardDate | 2010-03-11 | дата выдачи | confirmed_document | |
| PersonInfo_PersonSurname | АРБУЗОВА | фамилия | confirmed_document | |
| PersonInfo_PersonName | АНАСТАСИЯ | имя | confirmed_document | |
| PersonInfo_PersonMiddleName | КОНСТАНТИНОВНА | отчество | confirmed_document | |
| PersonInfo_Sex | 1 | пол | confirmed_document | |
| PersonInfo_Birthday | 1987-07-25 | дата рождения | confirmed_document | |
| PersonInfo_Birthplace | город Саратов | место рождения | confirmed_document | |
| ResidencePlace_PostalCode | 410052 | индекс | confirmed_document | |
| ResidencePlace_CountryCode | RU | страна | confirmed_document | |
| ResidencePlace_CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| ResidencePlace_Region | Саратовская область | регион | confirmed_document | |
| ResidencePlace_City | Саратов | город | confirmed_document | |
| ResidencePlace_StreetHouse | Ул. Одесская д 11 кв 160 | адрес | confirmed_document | |

---

## document: Letter of Attorney (11004)
- uqi_prefix: formalized.letter_of_attorney_1
- xml_target_root: AltaLetterOfAttorney
- full_path: alta\stable_source\LetterOfAttorney_1.xml
- file_name: LetterOfAttorney_1.xml
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| Subject | link:alta\stable_source\LetterOfAttorney_1.xml | текст доверенности (link) | confirmed_document | cp1251 |
| EndDate | 2026-12-31 | действительна до | confirmed_document | |
| DocumentReference_PrDocumentName | ДОВЕРЕННОСТЬ | наименование | confirmed_document | |
| DocumentReference_PrDocumentNumber | 1 | номер | confirmed_document | |
| DocumentReference_PrDocumentDate | 2026-02-01 | дата | confirmed_document | |
| Organization_OrganizationName | ООО «СКИФ» | выдавшая организация | confirmed_document | |
| Organization_ShortName | ООО «СКИФ» | краткое | confirmed_document | |
| Organization_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Organization_INN | 1650389298 | ИНН | confirmed_document | |
| Organization_KPP | 165001001 | КПП | confirmed_document | |
| Organization_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Organization_Address_CountryCode | RU | страна | confirmed_document | |
| Organization_Address_CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| Organization_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| Organization_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| Organization_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | улица/дом/офис | confirmed_document | |
| Organization_OrganizationPerson_PersonSurname | Саранов | подписант | confirmed_document | |
| Organization_OrganizationPerson_PersonName | Дмитрий | подписант | confirmed_document | |
| Organization_OrganizationPerson_PersonMiddleName | Олегович | подписант | confirmed_document | |
| Organization_OrganizationPerson_PersonPost | Директор | должность | confirmed_document | |
| EmpoweredPerson_PersonSurname | АРБУЗОВА | уполномоченное лицо | confirmed_document | |
| EmpoweredPerson_PersonName | АНАСТАСИЯ | имя | confirmed_document | |
| EmpoweredPerson_PersonMiddleName | КОНСТАНТИНОВНА | отчество | confirmed_document | |
| EmpoweredPerson_PersonPost | УПОЛНОМОЧЕННОЕ ЛИЦО | роль/должность | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardCode | RU01001 | код документа | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardName | ПАСРФ | наименование | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardSeries | 63 09 | серия | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardNumber | 449948 | номер | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardDate | 2010-03-11 | дата выдачи | confirmed_document | |
| EmpoweredPerson_Passport_OrganizationName | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | кем выдан | confirmed_document | |

---

# II. non_formalized

## document: Storage report (ДО-1)
- uqi_prefix: non_formalized.svh_1
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО 14431420260204161621.md
- file_name: ДО 14431420260204161621.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| number | 0000080 | № ДО-1 | confirmed_document | |
| date | 2026-02-03 | дата ДО-1 | confirmed_document | |
| warehouse_license_number | 10404/141210/10092/5 | лицензия СВХ | confirmed_document | |
| warehouse_license_date | 2025-09-18 | дата лицензии | confirmed_document | |
| actual_gross_weight | 3500 | фактический брутто | confirmed_document | |
| actual_places | 127 | фактические места | confirmed_document | |
| transport_reg_number | O157A0774 (прицеп BT374974) | ТС | confirmed_document | как в ДО-1 |

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
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО доп 14431520260204161645.md
- file_name: ДО доп 14431520260204161645.md
- status: pending
- note: Доплист содержит только заголовок + одну строку текста + итоги. Адрес/код таможни СВХ в нем не указаны.

| field | value | description | status | note |
|---|---|---|---|---|
| number | 1 | № доплиста | confirmed_document | |
| date |  | дата доплиста | pending | в md не указано явно |
| actual_gross_weight | 3500 | брутто | confirmed_document | из итога |
| actual_places | 127 | места | confirmed_document | из итога |
| transport_reg_number |  | ТС | pending | в md нет |
| svh_address_region |  | регион СВХ | pending | в md нет |
| svh_address_city |  | город СВХ | pending | |
| svh_address_street_house |  | улица/дом СВХ | pending | |
| svh_customs_code |  | код таможни СВХ | pending | |

---

## document: Transit declaration
- uqi_prefix: non_formalized.td_1
- full_path: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ТД 10719110_240126_5011363_reg00378тд.md
- file_name: ТД 10719110_240126_5011363_reg00378тд.md
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| number | 10719110/240126/5011363 | № ТД | confirmed_document | |
| date | 2026-01-24 | дата ТД | confirmed_document | |
| customs_post_code | 10404083 | код таможенного органа назначения | confirmed_document | |
| customs_post_name | ОТО и ТК №3 т/п Набережночелнинский | наименование | confirmed_document | |
| transport_reg_number | 0157A0774 / B1734974 RU | ТС по ТД | confirmed_document | OCR (есть конфликт с ДО-1/PL по прицепу) |

---

## document: Master data
- uqi_prefix: non_formalized.master_data_1
- full_path: alta\stable_source (EGRUL/LOA/Passport)
- file_name: -
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| declarant_name | ООО «СКИФ» | декларант | confirmed_document | ЕГРЮЛ |
| declarant_ogrn | 1201600020390 | ОГРН | confirmed_document | |
| declarant_inn | 1650389298 | ИНН | confirmed_document | |
| declarant_kpp | 165001001 | КПП | confirmed_document | |
| declarant_address_postal_code | 423800 | индекс | confirmed_document | |
| declarant_address_country_code | RU | страна | confirmed_document | |
| declarant_address_country_name | Россия | страна (текст) | confirmed_document | |
| declarant_address_region | Республика Татарстан | регион | confirmed_document | |
| declarant_address_city | Набережные Челны | город | confirmed_document | |
| declarant_address_street | проезд Хлебный | улица | confirmed_document | |
| declarant_address_building | 30 | дом | confirmed_document | |
| declarant_address_room | 211 | офис | confirmed_document | |
| declarant_phone | +7 937 779-26-56 | телефон | confirmed_document | инвойс/контракт |
| declarant_email |  | e-mail | pending | в первичке не найден |
| representative_last_name | АРБУЗОВА | фамилия | confirmed_document | LOA/паспорт |
| representative_first_name | АНАСТАСИЯ | имя | confirmed_document | |
| representative_middle_name | КОНСТАНТИНОВНА | отчество | confirmed_document | |
| representative_position | УПОЛНОМОЧЕННОЕ ЛИЦО | должность/статус | confirmed_document | LOA |
| representative_phone | +7-927-030-70-07 | телефон | confirmed_document | LOA Subject |
| representative_email |  | e-mail | pending | в первичке не найден |
| representative_passport_code | RU01001 | код документа | confirmed_document | LOA |
| representative_passport_name | ПАСРФ | наименование | confirmed_document | |
| representative_passport_series | 63 09 | серия | confirmed_document | |
| representative_passport_number | 449948 | номер | confirmed_document | |
| representative_passport_date | 2010-03-11 | дата выдачи | confirmed_document | |
| representative_passport_issuer | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | кем выдан | confirmed_document | |
| representative_authority_doc_name | ДОВЕРЕННОСТЬ | документ полномочий | confirmed_document | |
| representative_authority_doc_number | 1 | № доверенности | confirmed_document | |
| representative_authority_doc_date_from | 2026-02-01 | дата начала | confirmed_document | |
| representative_authority_doc_date_to | 2026-12-31 | дата окончания | confirmed_document | |
| note | источники: FreeDoc_ЮЭ9965-25-106893283.xml; LetterOfAttorney_1.xml; Passport_63_09_449948.xml; FreeDoc_КООО_26651_М.xml | примечание | confirmed_document | |

---

# III. Нерешенные вопросы

- formalized.cmr_1.TrakingCargo_TakingCargoDate
  - question: В CMR md-файле не указана дата принятия груза к перевозке. Подтверди дату.

- formalized.cmr_1.Consignor_PostalAddress_CountryCode
  - question: В CMR md-файле нет явной страны/адреса отправителя в формате полей Альты (есть только строка отправителя). Что ставить в Consignor_* адресные поля?

- formalized.payment_order_1.PayerSign.PersonSurname
  - question: В currency_transfer_7_28.11.2025.md нет ФИО подписанта. Что ставить в PayerSign (фамилия)?

- formalized.payment_order_1.PayerSign.PersonName
  - question: Что ставить в PayerSign (имя/инициалы)?

- formalized.payment_order_2.PayerSign.PersonSurname
  - question: В currency_transfer_1_13.01.2026.md нет ФИО подписанта. Что ставить в PayerSign (фамилия)?

- formalized.payment_order_2.PayerSign.PersonName
  - question: Что ставить в PayerSign (имя/инициалы)?

- formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonSurname
  - question: В счете 26-00378-tl от 27.01.2026 подписи/ФИО руководителя и бухгалтера не указаны. Что ставим в подписи?

- formalized.service_invoice_1.SignatureDirectorChiefAccountant_Director_PersonName
  - question: См. выше.

- formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname
  - question: См. выше.

- formalized.service_invoice_1.SignatureDirectorChiefAccountant_ChiefAccountant_PersonName
  - question: См. выше.

- formalized.service_invoice_1.Consignor_SubjectAddressDetails.PostalCode
  - question: Для ServiceInvoice нужен PostalCode отправителя (продавца). В первичке не найден. Оставляем пустым?

- non_formalized.svh_additional_sheet_1.date
  - question: Подтверди дату доплиста №1 к отчету ДО-1 (если = 03.02.2026).

- non_formalized.svh_additional_sheet_1.transport_reg_number
  - question: Подтверди ТС для доплиста (если = O157A0774 / BT374974).

- non_formalized.svh_additional_sheet_1.svh_address_region
  - question: Подтверди адрес СВХ ООО «ЛОГИКАМ» (регион/город/улица/дом) и код таможни в зоне СВХ.

- non_formalized.master_data_1.declarant_email
  - question: Укажи e-mail ООО «СКИФ» (если нужен).

- non_formalized.master_data_1.representative_email
  - question: Укажи e-mail Арбузовой А.К. (если нужен).
