---
название кейса: МоскитнаяСетка
путь к папке поставки: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
тип поставки: 1 ДТ / 7 товаров (в ТД/ДО-1 агрегировано по 2 кодам ТН ВЭД)
источник данных: md-файлы (этап 0), stable_source, operator_provided_data.md
---

# Раздел I: formalized

## document: Contract
- `uqi_prefix`: formalized.contract_1
- `xml_target_root`: AltaE2CONT
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md
- `file_name`: SALES CONTRACT NoLM-2553.md
- `status`: confirmed
- `note`: Рамочный контракт, сумма и условия уточнены в доп. соглашении №1 и инвойсе LM-2591

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 03011 | код вида документа для графы 44 | confirmed_document | константа |
| ContractRegistration_PrDocumentNumber | LM-2553 | № контракта | confirmed_document | |
| ContractRegistration_PrDocumentDate | 2025-07-02 | дата контракта | confirmed_document | |
| ContractTerms_Amount | 41904.30 | общая сумма контракта | confirmed_document | уточнена в доп. соглашении |
| ContractTerms_CurrencyCode | 156 | цифровой код валюты (CNY) | confirmed_operator | из operator_provided_data |
| ContractTerms_LastDate | 2026-12-31 | срок действия/исполнения | confirmed_document | |
| ContractTerms_OtherTerms | EXW HEBEI | условия поставки | confirmed_operator | решение оператора |
| ContractTerms_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\SALES CONTRACT NoLM-2553.md | текст контракта | confirmed_document | |
| ContractTerms_DealSign | 1 | системный признак Альты | confirmed_operator | |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | продавец | confirmed_document | |
| ForeignPerson_Address_CountryCode | CN | страна продавца | confirmed_document | |
| ForeignPerson_Address_CounryName | Китай | страна продавца (текст) | confirmed_document | |
| ForeignPerson_Address_Region | Hebei | регион продавца | confirmed_document | |
| ForeignPerson_Address_City | Shijiazhuang | город продавца | confirmed_document | |
| ForeignPerson_Address_StreetHouse | No. 5 Gaodong street, Xinhua District | адрес одной строкой | confirmed_document | |
| RussianPerson_OrganizationName | ООО «СКИФ» | покупатель | confirmed_document | |
| RussianPerson_OGRN | 1201600020390 | ОГРН покупателя | confirmed_document | из stable_source |
| RussianPerson_INN | 1650389298 | ИНН покупателя | confirmed_document | |
| RussianPerson_KPP | 165001001 | КПП покупателя | confirmed_document | |
| RussianPerson_Address_PostalCode | 423800 | индекс покупателя | confirmed_document | |
| RussianPerson_Address_CountryCode | RU | страна покупателя | confirmed_document | |
| RussianPerson_Address_CounryName | РОССИЯ | страна покупателя (текст) | confirmed_document | |
| RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | регион покупателя | confirmed_document | |
| RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город покупателя | confirmed_document | |
| RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | адрес одной строкой | confirmed_document | |

## document: Supplementary Contract
- `uqi_prefix`: formalized.supplementary_contract_1
- `xml_target_root`: AltaSupplementaryContract
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md
- `file_name`: 1 Supplementary agreement to the contract.md
- `status`: confirmed
- `note`: Доп. соглашение №1 от 25.11.2025, уточняет сумму и номенклатуру

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentNumber | 1 | № доп. соглашения | confirmed_document | |
| IssueDate | 2025-11-25 | дата доп. соглашения | confirmed_document | |
| ContractDescription_Amount | 270000.00 | новая сумма контракта | confirmed_document | |
| ContractDescription_CurrencyCode | 156 | код валюты | confirmed_operator | |
| ContractDescription_LastDate | 2026-12-31 | новый срок действия | confirmed_document | |
| ContractDescription_ContractText | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\1 Supplementary agreement to the contract.md | текст доп. соглашения | confirmed_document | |
| ContractDescription_DealSign | 1 | системный признак | confirmed_operator | |
| ContractDescription_StockCategorySign | 0 | системный признак | confirmed_operator | |
| ContractDescription_BuyerLimitationSign | 0 | системный признак | confirmed_operator | |
| ContractDescription_InsuranceSign | 0 | системный признак | confirmed_operator | |
| RussianPerson_OrganizationName | ООО «СКИФ» | российская сторона | confirmed_document | |
| RussianPerson_ShortName | ООО «СКИФ» | краткое наименование | confirmed_operator | |
| RussianPerson_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| RussianPerson_INN | 1650389298 | ИНН | confirmed_document | |
| RussianPerson_KPP | 165001001 | КПП | confirmed_document | |
| RussianPerson_Address_PostalCode | 423800 | индекс | confirmed_document | |
| RussianPerson_Address_CountryCode | RU | страна | confirmed_document | |
| RussianPerson_Address_CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| RussianPerson_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | адрес | confirmed_document | |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | иностранная сторона | confirmed_document | |
| ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | краткое наименование | confirmed_operator | foreign_person_shortname_equals_full=true |
| ForeignPerson_Address_CountryCode | CN | страна | confirmed_document | |
| ForeignPerson_Address_CounryName | Китай | страна (текст) | confirmed_document | |
| ForeignPerson_Address_Region | Hebei | регион | confirmed_document | |
| ForeignPerson_Address_City | Shijiazhuang | город | confirmed_document | |
| ForeignPerson_Address_StreetHouse | No. 5 Gaodong street, Xinhua District | адрес | confirmed_document | |

#### ContractSignedPerson_1
| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname | Li | фамилия | confirmed_operator | |
| PersonName | Jing | имя | confirmed_operator | |
| PersonMiddleName | | отчество | confirmed_operator | пусто по решению оператора |

## document: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\CL на сетку .md
- `file_name`: CL на сетку .md (приоритет XLSX)
- `status`: confirmed
- `note`: Инвойс LM-2591, 7 товарных строк, целевая структура для этапов 2-3

| field | value | description | status | note |
|---|---|---|---|---|
| CurrencyRate | 10.9430 | курс валюты | confirmed_operator | |
| CurrencyCode | CNY | валюта инвойса | confirmed_operator | |
| DocumentCode | 04021 | код вида документа | confirmed_document | |
| PlacesQuantity | 127 | кол-во мест | confirmed_document | |
| PlacesDescription | Поддон | описание мест | confirmed_operator | |
| GrossWeightQuantity | 3500.00 | общий вес брутто | confirmed_document | из PL totals |
| NetWeightQuantity | 3302.00 | общий вес нетто | confirmed_document | из PL totals |
| GCost | 97260.00 | системное поле Альты | confirmed_operator | =TotalCost |
| TotalCost | 97260.00 | итого по инвойсу | confirmed_document | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | место поставки | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | код условий поставки | confirmed_operator | |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия поставки | confirmed_document | |
| DeliveryTerms_DispatchCountryCode | CN | страна отправления | confirmed_operator | |
| DeliveryTerms_TradingCountryCode | CN | торгующая страна | confirmed_operator | |
| DeliveryTerms_DestinationCountryCode | RU | страна назначения | confirmed_document | |
| Registration_PrDocumentName | INVOICE | наименование документа | confirmed_document | |
| Registration_PrDocumentNumber | LM-2591 | номер инвойса | confirmed_document | |
| Registration_PrDocumentDate | 2025-10-30 | дата инвойса | confirmed_document | |
| Contract_PrDocumentNumber | LM-2553 | № контракта-ссылки | confirmed_document | |
| Contract_PrDocumentDate | 2025-07-02 | дата контракта-ссылки | confirmed_document | |
| Buyer_CompanyID | 1650389298 | ИНН покупателя | confirmed_document | |
| Buyer_KPPCode | 165001001 | КПП покупателя | confirmed_document | |
| Buyer_Name | ООО «СКИФ» | наименование покупателя | confirmed_document | |
| Buyer_PostalAddress_PostalCode | 423800 | индекс | confirmed_document | |
| Buyer_PostalAddress_CountryCode | RU | страна | confirmed_document | |
| Buyer_PostalAddress_CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| Buyer_PostalAddress_StreetHouse | проезд Хлебный, д. 30, офис 211 | адрес | confirmed_document | |
| Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | продавец | confirmed_document | |
| Seler_PostalAddress_CountryCode | CN | страна | confirmed_document | |
| Seler_PostalAddress_CounryName | Китай | страна (текст) | confirmed_document | |
| Seler_PostalAddress_Region | Hebei | регион | confirmed_document | |
| Seler_PostalAddress_City | Shijiazhuang | город | confirmed_document | |
| Seler_PostalAddress_StreetHouse | No. 5 Gaodong street, Xinhua District | адрес | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | грузоотправитель | confirmed_operator | consignor_equals_seller=true |
| Consignor_Address_CountryCode | CN | страна | confirmed_document | |
| Consignor_Address_CounryName | Китай | страна (текст) | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street, Xinhua District | адрес | confirmed_document | |
| Consignee_OrganizationName | ООО «СКИФ» | грузополучатель | confirmed_operator | consignee_equals_buyer=true |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | адрес | confirmed_document | |

#### InvoiceGoods_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | описание товара | confirmed_document | |
| GoodsQuantity | 60 | кол-во | confirmed_document | |
| MeasureUnitQualifierName | шт | ед. измерения | confirmed_document | |
| GrossWeightQuantity | 855.00 | брутто | confirmed_operator | из PL |
| NetWeightQuantity | 806.60 | нетто | confirmed_operator | из PL |
| Price | 14742 | цена за единицу | confirmed_document | |
| TotalCost | 14742 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | торговая марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_2
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30 | описание товара | confirmed_document | |
| GoodsQuantity | 30 | кол-во | confirmed_document | |
| MeasureUnitQualifierName | шт | ед. измерения | confirmed_document | |
| GrossWeightQuantity | 490.00 | брутто | confirmed_operator | из PL |
| NetWeightQuantity | 460.80 | нетто | confirmed_operator | из PL |
| Price | 8424 | цена за единицу | confirmed_document | |
| TotalCost | 8424 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | торговая марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_3
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы "Антипыльца" из полиэстера | описание товара | confirmed_document | |
| GoodsQuantity | 60 | кол-во | confirmed_document | |
| MeasureUnitQualifierName | шт | ед. измерения | confirmed_document | |
| GrossWeightQuantity | 265.00 | брутто | confirmed_operator | из PL |
| NetWeightQuantity | 252.00 | нетто | confirmed_operator | из PL |
| Price | 16002 | цена за единицу | confirmed_document | |
| TotalCost | 16002 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | торговая марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_4
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2/Сетка против пыльцы Антипыльца" из полиэстера | описание товара | confirmed_document | |
| GoodsQuantity | 30 | кол-во | confirmed_document | |
| MeasureUnitQualifierName | шт | ед. измерения | confirmed_document | |
| GrossWeightQuantity | 155.00 | брутто | confirmed_operator | из PL |
| NetWeightQuantity | 144.00 | нетто | confirmed_operator | из PL |
| Price | 9144 | цена за единицу | confirmed_document | |
| TotalCost | 9144 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | торговая марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_5
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 7019900095 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА " Антимошка " из стекловолокна | описание товара | confirmed_document | |
| GoodsQuantity | 90 | кол-во | confirmed_document | |
| MeasureUnitQualifierName | шт | ед. измерения | confirmed_document | |
| GrossWeightQuantity | 520.00 | брутто | confirmed_operator | из PL |
| NetWeightQuantity | 491.40 | нетто | confirmed_operator | из PL |
| Price | 12852 | цена за единицу | confirmed_document | |
| TotalCost | 12852 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | торговая марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_6
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 7019900095 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 : Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна | описание товара | confirmed_document | |
| GoodsQuantity | 180 | кол-во | confirmed_document | |
| MeasureUnitQualifierName | шт | ед. измерения | confirmed_document | |
| GrossWeightQuantity | 1190.00 | брутто | confirmed_operator | из PL |
| NetWeightQuantity | 1123.20 | нетто | confirmed_operator | из PL |
| Price | 29376 | цена за единицу | confirmed_document | |
| TotalCost | 29376 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | торговая марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

#### InvoiceGoods_7
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки "Антипыльца " из полиэстера | описание товара | confirmed_document | |
| GoodsQuantity | 5 | кол-во | confirmed_document | |
| MeasureUnitQualifierName | шт | ед. измерения | confirmed_document | |
| GrossWeightQuantity | 25.00 | брутто | confirmed_operator | из PL |
| NetWeightQuantity | 24.00 | нетто | confirmed_operator | из PL |
| Price | 6720 | цена за единицу | confirmed_document | |
| TotalCost | 6720 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | торговая марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

## document: Packing List
- `uqi_prefix`: formalized.packing_list_1
- `xml_target_root`: AltaE2PACK
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\PL на сетку .md
- `file_name`: PL на сетку .md
- `status`: confirmed
- `note`: Упаковочный лист LM-2591, 7 грузовых строк, 2 ТС

| field | value | description | status | note |
|---|---|---|---|---|
| GrossWeightQuantity | 3500.00 | общий вес брутто | confirmed_document | |
| NetWeightQuantity | 3302.00 | общий вес нетто | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | грузоотправитель | confirmed_document | |
| Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | краткое наименование | confirmed_operator | |
| Consignor_Address_CountryCode | CN | страна | confirmed_document | |
| Consignor_Address_CounryName | Китай | страна (текст) | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street. Shijiazhuang. Hebei China | адрес | confirmed_document | |
| Consignee_OrganizationName | ООО «СКИФ» | грузополучатель | confirmed_document | |
| Consignee_ShortName | ООО «СКИФ» | краткое наименование | confirmed_operator | |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | адрес | confirmed_document | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny/Набережные Челны | место поставки | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | код условий | confirmed_operator | |
| DeliveryTerms_DeliveryTermsStringCode | EXW HEBEI/ EXW Хэбей | условия | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentName | SALES CONTRACT | наименование | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | № | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentDate | 2025-07-02 | дата | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentName | INVOICE | наименование | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | № | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentDate | 2025-10-30 | дата | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentName | PACKING LIST | наименование | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | № | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentDate | 2025-10-30 | дата | confirmed_document | |
| registration_doc_name | Упаковочный лист | наименование для графы 44 | confirmed_operator | |
| registration_doc_number | LM-2591 | номер для графы 44 | confirmed_operator | |
| registration_doc_date | 2025-10-30 | дата для графы 44 | confirmed_operator | |

#### Goods_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Anti-cat mesh. Roll size 1.4 * 0.16 * 0.16 Material: polyester/ Москитная сетка «Антикот» | описание | confirmed_document | |
| GoodsQuantity | 60 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 855 | брутто | confirmed_document | |
| NetWeightQuantity | 806.6 | нетто | confirmed_document | |
| PackingInfo/PakingQuantity | 60 | кол-во упаковок | confirmed_operator | =GoodsQuantity |

#### Goods_2
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Anti-cat mesh Roll size 1.6 * 0.16 * 0.16 | описание | confirmed_document | |
| GoodsQuantity | 30 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 490 | брутто | confirmed_document | |
| NetWeightQuantity | 460.8 | нетто | confirmed_document | |
| PackingInfo/PakingQuantity | 30 | кол-во упаковок | confirmed_operator | |

#### Goods_3
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | ANTI-POLLEN MESH. Material: polyester/Сетка против пыльцы Антипыльца" | описание | confirmed_document | |
| GoodsQuantity | 6 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 265 | брутто | confirmed_document | |
| NetWeightQuantity | 252 | нетто | confirmed_document | |
| PackingInfo/PakingQuantity | 6 | кол-во упаковок | confirmed_operator | |

#### Goods_4
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | ANTI-POLLEN MESH. Material: polyeste/Сетка против пыльцы Антипыльца" | описание | confirmed_document | |
| GoodsQuantity | 3 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 155 | брутто | confirmed_document | |
| NetWeightQuantity | 144 | нетто | confirmed_document | |
| PackingInfo/PakingQuantity | 3 | кол-во упаковок | confirmed_operator | |

#### Goods_5
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1.42*0.55*0.18 /СЕТКА СРЕДНЕГО РАЗМЕРА " Антимошка " | описание | confirmed_document | |
| GoodsQuantity | 9 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 520 | брутто | confirmed_document | |
| NetWeightQuantity | 491.4 | нетто | confirmed_document | |
| PackingInfo/PakingQuantity | 9 | кол-во упаковок | confirmed_operator | |

#### Goods_6
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1.62*0.55*18 | описание | confirmed_document | |
| GoodsQuantity | 18 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 1190 | брутто | confirmed_document | |
| NetWeightQuantity | 1123.2 | нетто | confirmed_document | |
| PackingInfo/PakingQuantity | 18 | кол-во упаковок | confirmed_operator | |

#### Goods_7
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1.72*0.35*0.31*1/Трехслойные сетки "Антипыльца " | описание | confirmed_document | |
| GoodsQuantity | 1 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 25 | брутто | confirmed_document | |
| NetWeightQuantity | 24 | нетто | confirmed_document | |
| PackingInfo/PakingQuantity | 1 | кол-во упаковок | confirmed_operator | |

#### TransportMeans_1
| field | value | description | status | note |
|---|---|---|---|---|
| Number | О157АО774 | номер ТС | confirmed_operator | тягач |
| ModeCode | 31 | код вида транспорта | confirmed_operator | авто |
| NationalityCode | 000 | код национальности | confirmed_operator | |
| MoverIndicator | true | тягач/прицеп | confirmed_operator | true=тягач |

#### TransportMeans_2
| field | value | description | status | note |
|---|---|---|---|---|
| Number | ВТ374974 | номер ТС | confirmed_operator | прицеп |
| ModeCode | 31 | код вида транспорта | confirmed_operator | |
| NationalityCode | 000 | код национальности | confirmed_operator | |
| MoverIndicator | false | тягач/прицеп | confirmed_operator | false=прицеп |

## document: CMR
- `uqi_prefix`: formalized.cmr_1
- `xml_target_root`: AltaE3CMR
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\СМР от СВХ.md
- `file_name`: СМР от СВХ.md
- `status`: confirmed
- `note`: CMR №00378 от 20.01.2026, товар указан ссылкой на спецификацию инвойса

| field | value | description | status | note |
|---|---|---|---|---|
| LanguageCode | RU | язык документа | confirmed_operator | |
| CMR_Choice | 1 | системный выбор | confirmed_operator | |
| RegistrationDocument_RegID | 00378 | номер CMR | confirmed_document | |
| RegistrationDocument_DateInf | 2026-01-20 | дата CMR | confirmed_document | |
| RegistrationDocument_Place | Маньчжурия | место составления | confirmed_operator | |
| TrakingCargo_TakingCargoDate | 2026-01-20 | дата принятия груза | confirmed_document | из даты CMR |
| TrakingCargo_TakingCargoPlace_CountryCode | CN | страна принятия | confirmed_operator | |
| TrakingCargo_TakingCargoPlace_CounryName | Китай | страна (текст) | confirmed_operator | |
| DeliveryPlace_CountryCode | RU | страна доставки | confirmed_operator | |
| DeliveryPlace_CounryName | РОССИЯ | страна (текст) | confirmed_operator | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | место поставки | confirmed_operator | |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия поставки | confirmed_operator | |
| GoodsQuantity | 127 | общее кол-во мест | confirmed_document | |
| CMRGoodsWeight_GrossWeightQuantity | 3500.00 | общий вес брутто | confirmed_document | |
| CMRTransport_PrimeMoverStateSignID | О157АО774 | гос. номер тягача | confirmed_operator | |
| CMRTransport_TrailerStateSignID | ВТ374974 | гос. номер прицепа | confirmed_operator | |
| Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | наименование | confirmed_document | |
| Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | краткое | confirmed_operator | |
| Consignor_PostalAddress_CountryCode | CN | страна | confirmed_document | |
| Consignor_Address_CounryName | Китай | страна (текст) | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street. Shijiazhuang. Hebei China | адрес | confirmed_document | |
| Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | гарант | confirmed_operator | решение оператора |
| Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | | confirmed_operator | |
| Consignor_Guarantee_Address_CountryCode | | | pending | не требуется |
| Consignor_Guarantee_Address_CounryName | | | pending | не требуется |
| Consignor_Guarantee_Address_Region | | | pending | не требуется |
| Consignor_Guarantee_Address_City | | | pending | не требуется |
| Consignor_Guarantee_Address_StreetHouse | | | pending | не требуется |
| Consignee_NameInf | ООО «Скиф» | получатель | confirmed_document | |
| Consignee_ShortName | ООО «Скиф» | краткое | confirmed_operator | |
| Consignee_OGRNID | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_INNID | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPPCode | 165001001 | КПП | confirmed_document | |
| Consignee_PostalAddress_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_PostalAddress_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| Consignee_Address_StreetHouse | проезд Хлебный, д. 30, офис 211 | адрес | confirmed_document | |

#### CMRGoods_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsNumeric | 1 | номер строки | confirmed_document | агрегировано |
| GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | описание | confirmed_document | |
| GoodsNomenclatureCode | 5804101000 / 7019900095 | код ТН ВЭД | confirmed_document | агрегировано |
| GoodsQuantity | 127 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 3500.00 | брутто | confirmed_document | |

## document: Payment Order (1)
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_7_28.11.2025.md
- `file_name`: currency_transfer_7_28.11.2025.md
- `status`: confirmed
- `note`: Заявка №7 от 28.11.2025, аванс 34041 CNY

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04023 | код вида | confirmed_operator | |
| PaymentModeCode | 0 | код способа | confirmed_operator | |
| PaymentAmount | 34041.00 | сумма платежа | confirmed_document | |
| TransactionKind | 01 | вид операции | confirmed_operator | |
| Priority | 5 | очередность | confirmed_operator | |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553, DATE:JULY 02.2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | назначение | confirmed_document | |
| ValueSpelledOut | pending | сумма прописью | pending | в документе только цифры |
| DocumentReference_PrDocumentNumber | 7 | номер платежки | confirmed_document | № заявки |
| DocumentReference_PrDocumentDate | 2025-11-28 | дата | confirmed_document | исполнено 01.12.2025, заявка 28.11 |
| Payer_OrganizationName | LLC SKIF | плательщик | confirmed_document | |
| Payer_INN | 1650389298 | ИНН | confirmed_document | |
| Payer_KPP | 165001001 | КПП | confirmed_document | |
| Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) | банк | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | получатель | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | банк получателя | confirmed_document | |

#### PayerSign_1
| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname | pending | фамилия | pending | в заявке не указан |
| PersonName | pending | имя | pending | |

## document: Payment Order (2)
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\currency_transfer_1_13.01.2026.md
- `file_name`: currency_transfer_1_13.01.2026.md
- `status`: confirmed
- `note`: Заявка №1 от 13.01.2026, остаток 63219 CNY

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04023 | код вида | confirmed_operator | |
| PaymentModeCode | 0 | код способа | confirmed_operator | |
| PaymentAmount | 63219.00 | сумма платежа | confirmed_document | |
| TransactionKind | 01 | вид операции | confirmed_operator | |
| Priority | 5 | очередность | confirmed_operator | |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553, DATE:JULY 02.2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | назначение | confirmed_document | |
| ValueSpelledOut | pending | сумма прописью | pending | |
| DocumentReference_PrDocumentNumber | 1 | номер платежки | confirmed_document | |
| DocumentReference_PrDocumentDate | 2026-01-13 | дата | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | плательщик | confirmed_document | |
| Payer_INN | 1650389298 | ИНН | confirmed_document | |
| Payer_KPP | 165001001 | КПП | confirmed_document | |
| Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) | банк | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | получатель | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | банк получателя | confirmed_document | |

#### PayerSign_1
| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname | pending | фамилия | pending | |
| PersonName | pending | имя | pending | |

## document: Service Invoice
- `uqi_prefix`: formalized.service_invoice_1
- `xml_target_root`: AltaServiceInvoice
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_от_27-01-2026.md
- `file_name`: Счет_№26-00378-tl_от_27-01-2026.md
- `status`: confirmed
- `note`: Счет за транспортно-экспедиционные услуги №26-00378-tl от 27.01.2026

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentSign | 1 | системный признак | confirmed_operator | |
| TotalServiceCost | 2700.00 | итого по услугам | confirmed_document | USD |
| Currency | USD | валюта | confirmed_document | |
| ServiceProvider_Name | ООО «Трансмипериал» | исполнитель | confirmed_document | |
| ServiceProvider_PaymentRequisitions/BankName | АО "Райффайзенбанк", г. Москва | банк | confirmed_document | |
| ContractDetails_PrDocumentNumber | КООО/26651/М | № договора | confirmed_document | |
| ContractDetails_PrDocumentDate | 2025-05-13 | дата договора | confirmed_document | |
| Registration_PrDocumentName | Счет на оплату | наименование | confirmed_document | |
| Registration_PrDocumentNumber | 26-00378-tl | номер счета | confirmed_document | |
| Registration_PrDocumentDate | 2026-01-27 | дата счета | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD. | грузоотправитель | confirmed_operator | consignor_equals_seller=true |
| Consignor_SubjectAddressDetails/PostalCode | pending | индекс | pending | |
| Consignor_SubjectAddressDetails/CountryCode | CN | страна | confirmed_document | |
| Consignor_SubjectAddressDetails/CounryName | Китай | страна (текст) | confirmed_document | |
| Consignor_SubjectAddressDetails/Region | Hebei | регион | confirmed_document | |
| Consignor_SubjectAddressDetails/Town | Shijiazhuang | город | confirmed_document | |
| Consignor_SubjectAddressDetails/StreetHouse | No. 5 Gaodong street | адрес | confirmed_document | |
| Consignee_OrganizationName | ООО «СКИФ» | получатель | confirmed_document | |
| Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_RFOrganizationFeatures_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_RFOrganizationFeatures_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_SubjectAddressDetails/PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_SubjectAddressDetails/CountryCode | RU | страна | confirmed_document | |
| Consignee_SubjectAddressDetails/CounryName | РОССИЯ | страна (текст) | confirmed_document | |
| Consignee_SubjectAddressDetails/Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| Consignee_SubjectAddressDetails/Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| Consignee_SubjectAddressDetails/StreetHouse | проезд Хлебный, д. 30 | улица | confirmed_document | |
| Consignee_SubjectAddressDetails/House | 30 | дом | confirmed_operator | |
| Consignee_SubjectAddressDetails/Room | 211 | офис | confirmed_operator | |
| PaymentDocument/PrDocumentNumber | ОТСУТСТВУЕТ | номер заявки | confirmed_operator | |
| PaymentDocument/PrDocumentDate | ОТСУТСТВУЕТ | дата заявки | confirmed_operator | |
| Signature_Choice | 1 | вариант подписи | confirmed_operator | |
| SignatureDirectorChiefAccountant_Director_PersonSurname | pending | фамилия директора | pending | |
| SignatureDirectorChiefAccountant_Director_PersonName | pending | имя директора | pending | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | pending | фамилия бухгалтера | pending | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | pending | имя бухгалтера | pending | |

#### ServiceDescription_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении... China, Hengshui - граница РФ... | описание услуги | confirmed_document | |
| CurrencyCode | USD | валюта | confirmed_document | |
| ServiceName | ОТСУТСТВУЕТ | наименование маршрута | confirmed_operator | |
| TaxRate | 0 | ставка НДС | confirmed_document | НДС 0% |
| TaxSum | 0.00 | сумма налога | confirmed_document | |
| ServiceCost_Amount | 1404.00 | стоимость | confirmed_document | |
| ServiceCost_Currency | USD | валюта | confirmed_document | |

#### ServiceDescription_2
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги... граница РФ... - Россия, Республика Татарстан, Набережные Челны | описание услуги | confirmed_document | |
| CurrencyCode | USD | валюта | confirmed_document | |
| ServiceName | ОТСУТСТВУЕТ | наименование маршрута | confirmed_operator | |
| TaxRate | 0 | ставка НДС | confirmed_document | |
| TaxSum | 0.00 | сумма налога | confirmed_document | |
| ServiceCost_Amount | 1296.00 | стоимость | confirmed_document | |
| ServiceCost_Currency | USD | валюта | confirmed_document | |

## document: Insurance Document
- `uqi_prefix`: formalized.insurance_document_1
- `xml_target_root`: AltaFreeDoc
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md
- `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md
- `status`: confirmed
- `note`: Счет на страхование груза №26-00378-tl/1 от 14.01.2026

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04111 | код вида | confirmed_document | |
| DocumentHead_DocumentName | Счет на оплату страховки | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2026-01-14 | дата | confirmed_document | |
| DocumentHead_DocumentNumber | 26-00378-tl/1 | номер | confirmed_document | |

#### TextPara_1
| field | value | description | status | note |
|---|---|---|---|---|
| TextPara_1 | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\Счет_№26-00378-tl_1_от_14-01-2026.md | текст счета | confirmed_document | |

## document: TechDescription
- `uqi_prefix`: formalized.tech_description_1
- `xml_target_root`: AltaFreeDoc
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md
- `file_name`: техничка Антикот, антипыльца антимошка .md
- `status`: confirmed
- `note`: Техническое описание товаров (приоритет DOCX версия)

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 05999 | код вида | confirmed_document | |
| DocumentHead_DocumentName | Техническое описание товаров | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2025-10-30 | дата | confirmed_operator | из даты инвойса/PL |
| DocumentHead_DocumentNumber | Б/Н | номер | confirmed_operator | |

#### TextPara_1
| field | value | description | status | note |
|---|---|---|---|---|
| TextPara_1 | link:alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\техничка Антикот, антипыльца антимошка .md | текст ТД | confirmed_document | |

# Раздел II: non_formalized

## document: Storage Report (svh_1)
- `uqi_prefix`: non_formalized.svh_1
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО 14431420260204161621.md
- `file_name`: ДО 14431420260204161621.md
- `status`: confirmed
- `note`: Отчет СВХ №0000080 от 03.02.2026

| field | value | description | status | note |
|---|---|---|---|---|
| number | 0000080 | № ДО-1 | confirmed_document | |
| date | 2026-02-03 | дата ДО-1 | confirmed_document | |
| warehouse_license_number | 10404/141210/10092/5 | № лицензии СВХ | confirmed_document | |
| warehouse_license_date | 2025-09-18 | дата лицензии | confirmed_document | |
| actual_gross_weight | 3500.00 | фактический вес | confirmed_document | |
| actual_places | 127 | фактическое кол-во мест | confirmed_document | |
| transport_reg_number | O157A0774 (Прицеп: BT374974) | номер ТС | confirmed_document | |

#### goods_1
| field | value | description | status | note |
|---|---|---|---|---|
| tnved | 5804101000 | код ТН ВЭД | confirmed_document | |
| places | 100 | кол-во мест | confirmed_document | |
| gross_weight_kg | 1790 | вес брутто | confirmed_document | |
| cost | 55032 | стоимость | confirmed_document | |
| currency_code | CNY | валюта | confirmed_document | |

#### goods_2
| field | value | description | status | note |
|---|---|---|---|---|
| tnved | 7019900095 | код ТН ВЭД | confirmed_document | |
| places | 27 | кол-во мест | confirmed_document | |
| gross_weight_kg | 1710 | вес брутто | confirmed_document | |
| cost | 42228 | стоимость | confirmed_document | |
| currency_code | CNY | валюта | confirmed_document | |

## document: Storage Report Additional Sheet (svh_additional_sheet_1)
- `uqi_prefix`: non_formalized.svh_additional_sheet_1
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ДО доп 14431520260204161645.md
- `file_name`: ДО доп 14431520260204161645.md
- `status`: confirmed
- `note`: Добавочный лист №1 к отчету №0000080

| field | value | description | status | note |
|---|---|---|---|---|
| number | 1 | № доп.листа | confirmed_document | |
| date | 2026-02-03 | дата | confirmed_document | |
| actual_gross_weight | 3500.00 | фактический вес | confirmed_document | итого |
| actual_places | 127 | фактическое кол-во мест | confirmed_document | итого |
| transport_reg_number | O157A0774 | номер ТС | confirmed_document | |
| svh_address_region | Республика Татарстан | регион СВХ | confirmed_document | |
| svh_address_city | Набережные Челны | город СВХ | confirmed_document | |
| svh_address_street_house | Производственный пр-д, д.45 | улица СВХ | confirmed_document | |
| svh_customs_code | 10404083 | код таможни | confirmed_document | ОТО и ТК N3 |

## document: Transit Declaration (td_1)
- `uqi_prefix`: non_formalized.td_1
- `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\md\ТД 10719110_240126_5011363_reg00378тд.md
- `file_name`: ТД 10719110_240126_5011363_reg00378тд.md
- `status`: confirmed
- `note`: ТД №10719110/240126/5011363 от 24.01.2026

| field | value | description | status | note |
|---|---|---|---|---|
| number | 10719110/240126/5011363 | номер ТД | confirmed_document | |
| date | 2026-01-24 | дата ТД | confirmed_document | |
| customs_post_code | 10404083 | код таможни | confirmed_document | |
| customs_post_name | ОТО и ТК №3 т/п Набережночелнинский | наименование | confirmed_document | |
| transport_reg_number | 0157A0774 / B1734974 RU | номер ТС | confirmed_document | |

## document: Master Data (master_data_1)
- `uqi_prefix`: non_formalized.master_data_1
- `full_path`: alta\stable_source\LetterOfAttorney_1.xml, Passport_63_09_449948.xml, FreeDoc_ЮЭ9965-25-106893283.xml
- `file_name`: stable_source XMLs
- `status`: confirmed
- `note`: Мастер-данные декларанта и представителя

| field | value | description | status | note |
|---|---|---|---|---|
| declarant_name | ООО «СКИФ» | наименование | confirmed_document | |
| declarant_ogrn | 1201600020390 | ОГРН | confirmed_document | |
| declarant_inn | 1650389298 | ИНН | confirmed_document | |
| declarant_kpp | 165001001 | КПП | confirmed_document | |
| declarant_address_postal_code | 423800 | индекс | confirmed_document | |
| declarant_address_country_code | RU | страна | confirmed_document | |
| declarant_address_country_name | РОССИЯ | страна (текст) | confirmed_document | |
| declarant_address_region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| declarant_address_city | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| declarant_address_street | проезд Хлебный | улица | confirmed_document | |
| declarant_address_building | 30 | дом | confirmed_document | |
| declarant_address_room | 211 | офис | confirmed_document | |
| declarant_phone | +7 937 779-26-56 | телефон | confirmed_document | |
| declarant_email | pending | email | pending | |
| representative_last_name | АРБУЗОВА | фамилия | confirmed_document | |
| representative_first_name | АНАСТАСИЯ | имя | confirmed_document | |
| representative_middle_name | КОНСТАНТИНОВНА | отчество | confirmed_document | |
| representative_position | УПОЛНОМОЧЕННОЕ ЛИЦО | должность | confirmed_document | |
| representative_phone | +7-927-030-70-07 | телефон | confirmed_document | из текста доверенности |
| representative_email | pending | email | pending | |
| representative_passport_code | RU01001 | код документа | confirmed_document | |
| representative_passport_name | ПАСРФ | наименование | confirmed_document | |
| representative_passport_series | 63 09 | серия | confirmed_document | |
| representative_passport_number | 449948 | номер | confirmed_document | |
| representative_passport_date | 2010-03-11 | дата выдачи | confirmed_document | |
| representative_passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | кем выдан | confirmed_document | |
| representative_authority_doc_name | ДОВЕРЕННОСТЬ | наименование | confirmed_document | |
| representative_authority_doc_number | 1 | номер | confirmed_document | |
| representative_authority_doc_date_from | 2026-02-01 | дата начала | confirmed_document | |
| representative_authority_doc_date_to | 2026-12-31 | дата окончания | confirmed_document | |
| note | Данные взяты из XML stable_source (Passport, LOA, EGRUL). Адрес разбит по полям согласно выписке ЕГРЮЛ. | | confirmed_document | |

# Раздел III: Нерешенные вопросы

- `[Общий]`
    - `question`: Все ключевые поля заполнены на основе первички и подтвержденных решений оператора. 
    В поле `ValueSpelledOut` (сумма прописью) для платежек установлен `pending`, так как в исходных заявках указан 
    только цифровой формат. Это не является блокером для этапов 2-3. Если требуется точная формулировка для печати, 
    укажите.