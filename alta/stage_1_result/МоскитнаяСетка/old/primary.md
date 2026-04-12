# primary.md (Этап 1) — МоскитнаяСетка

case_name: МоскитнаяСетка
source_folder: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02
dt_scope: 1 поставка / подготовка для ДТ (целевая структура товаров для дальнейших этапов: 7 строк как в invoice/packing list)
status: pending
unresolved_conflicts_count: 0
unresolved_pending_count: 25
note: Регенерация по первичке (pdf/png) + stable_source + operator_provided_data.md + ответам оператора в чате. Pending оставлены только там, где значение отсутствует в первичке и не было подтверждено оператором.

---

## I. formalized

### document: Contract
- uqi_prefix: formalized.contract_1
- xml_target_root: AltaE2CONT
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/контракт/SALES CONTRACT NoLM-2553.pdf (прочитан по PNG SALES_CONTRACT_2553_1..5.png)
- file_name: SALES CONTRACT NoLM-2553.pdf
- status: confirmed
- note: Полный текст хранить ссылкой.

| field | value | status | note |
|---|---|---|---|
| DocumentCode | 03011 | confirmed_document | |
| ContractRegistration_PrDocumentNumber | LM-2553 | confirmed_document | |
| ContractRegistration_PrDocumentDate | 2025-07-02 | confirmed_document | |
| ContractTerms_Amount | 41904.30 | confirmed_document | сумма в RMB как в контракте |
| ContractTerms_CurrencyCode | 156 | confirmed_operator | operator_provided_data.md: CNY numeric=156 |
| ContractTerms_LastDate | 2026-12-31 | confirmed_document | shipment period end |
| ContractTerms_OtherTerms | EXW HEBEI | confirmed_operator | решение оператора |
| ContractTerms_ContractText | link: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/контракт/SALES CONTRACT NoLM-2553.pdf | confirmed_document | хранить ссылкой |
| ContractTerms_DealSign | 1 | confirmed_operator | решение оператора |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_document | seller |
| ForeignPerson_Address_CountryCode | CN | confirmed_document | |
| ForeignPerson_Address_CounryName | CHINA | confirmed_document | |
| ForeignPerson_Address_Region | HEBEI | confirmed_document | |
| ForeignPerson_Address_City | SHIJIAZHUANG | confirmed_document | |
| ForeignPerson_Address_StreetHouse | No. 5 Gaodong Street, Xinhua District | confirmed_document | |
| RussianPerson_OrganizationName | LLC «SKIF» | confirmed_document | buyer |
| RussianPerson_OGRN | 1201600020390 | confirmed_document | stable_source: EGRUL |
| RussianPerson_INN | 1650389298 | confirmed_document | stable_source: EGRUL |
| RussianPerson_KPP | 165001001 | confirmed_document | stable_source: EGRUL |
| RussianPerson_Address_PostalCode | 423800 | confirmed_document | |
| RussianPerson_Address_CountryCode | RU | confirmed_document | |
| RussianPerson_Address_CounryName | RUSSIA | confirmed_document | |
| RussianPerson_Address_Region | REPUBLIC OF TATARSTAN | confirmed_document | |
| RussianPerson_Address_City | NABEREZHNYE CHELNY | confirmed_document | |
| RussianPerson_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | confirmed_document | |

### document: Supplementary Contract
- uqi_prefix: formalized.supplementary_contract_1
- xml_target_root: AltaSupplementaryContract
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/контракт/1 Supplementary agreement to the contract.pdf
- file_name: 1 Supplementary agreement to the contract.pdf
- status: pending
- note: В документе JING LI без отчества.

| field | value | status | note |
|---|---|---|---|
| DocumentNumber | 1 | confirmed_document | |
| IssueDate | 2025-11-25 | confirmed_document | |
| ContractDescription_Amount | 270000.00 | confirmed_document | |
| ContractDescription_CurrencyCode | 156 | confirmed_operator | operator_provided_data.md |
| ContractDescription_LastDate | 2026-12-31 | confirmed_operator | operator_provided_data.md: expiry_date |
| ContractDescription_ContractText | link: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/контракт/1 Supplementary agreement to the contract.pdf | confirmed_document | хранить ссылкой |
| ContractDescription_DealSign | 1 | confirmed_operator | operator_provided_data.md |
| ContractDescription_StockCategorySign | 0 | confirmed_operator | operator_provided_data.md |
| ContractDescription_BuyerLimitationSign | 0 | confirmed_operator | operator_provided_data.md |
| ContractDescription_InsuranceSign | 0 | confirmed_operator | operator_provided_data.md |
| RussianPerson_OrganizationName | ООО «СКИФ» | confirmed_document | |
| RussianPerson_ShortName | ООО «СКИФ» | confirmed_document | |
| RussianPerson_OGRN | 1201600020390 | confirmed_document | stable_source: EGRUL |
| RussianPerson_INN | 1650389298 | confirmed_document | stable_source: EGRUL |
| RussianPerson_KPP | 165001001 | confirmed_document | stable_source: EGRUL |
| RussianPerson_Address_PostalCode | 423800 | confirmed_document | |
| RussianPerson_Address_CountryCode | RU | confirmed_document | |
| RussianPerson_Address_CounryName | РОССИЯ | confirmed_document | |
| RussianPerson_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | confirmed_document | |
| RussianPerson_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | confirmed_document | |
| RussianPerson_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | confirmed_document | |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_document | |
| ForeignPerson_ShortName | pending | pending | |
| ForeignPerson_Address_CountryCode | CN | confirmed_document | |
| ForeignPerson_Address_CounryName | CHINA | confirmed_document | |
| ForeignPerson_Address_Region | HEBEI | confirmed_document | |
| ForeignPerson_Address_City | SHIJIAZHUANG | confirmed_document | |
| ForeignPerson_Address_StreetHouse | No. 5 Gaodong street | confirmed_document | |

#### ContractSignedPerson
| field | value | status | note |
|---|---|---|---|
| PersonSurname | Li | confirmed_operator | operator_provided_data.md |
| PersonName | Jing | confirmed_operator | operator_provided_data.md |
| PersonMiddleName | pending | pending | в первичке отсутствует |

### document: Invoice
- uqi_prefix: formalized.invoice_1
- xml_target_root: AltaE2I
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/CL на сетку .pdf
- file_name: CL на сетку .pdf
- status: pending
- note: По инвойсу не указаны отдельными полями Consignor/Consignee (кроме buyer/seller). Веса по строкам и итоги gross/net взяты из PL по решению оператора (operator_provided_data.md).

| field | value | status | note |
|---|---|---|---|
| CurrencyRate | 10.9430 | confirmed_operator | operator_provided_data.md |
| CurrencyCode | CNY | confirmed_operator | решение оператора |
| DocumentCode | 04021 | confirmed_document | |
| PlacesQuantity | 127 | confirmed_document | Qty/BG 127 pcs |
| PlacesDescription | Поддон | confirmed_operator | operator_provided_data.md |
| GrossWeightQuantity | 3500.00 | confirmed_operator | operator_provided_data.md (from PL totals) |
| NetWeightQuantity | 3302.00 | confirmed_operator | operator_provided_data.md (from PL totals) |
| GCost | 97260.00 | confirmed_operator | решение оператора: =TotalCost |
| TotalCost | 97260.00 | confirmed_document | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | confirmed_operator | решение оператора |
| DeliveryTerms_DeliveryTermsStringCode | EXW | confirmed_document | |
| DeliveryTerms_DispatchCountryCode | CN | confirmed_operator | operator_provided_data.md |
| DeliveryTerms_TradingCountryCode | pending | pending | не указано в первичке |
| DeliveryTerms_DestinationCountryCode | RU | confirmed_operator | operator_provided_data.md |
| Registration_PrDocumentName | Commercial invoice | confirmed_document | |
| Registration_PrDocumentNumber | LM-2591 | confirmed_document | |
| Registration_PrDocumentDate | 2025-10-30 | confirmed_document | |
| Contract_PrDocumentNumber | LM-2553 | confirmed_document | |
| Contract_PrDocumentDate | 2025-07-02 | confirmed_document | |
| Buyer_CompanyID | 1650389298 | confirmed_document | stable_source: EGRUL |
| Buyer_KPPCode | 165001001 | confirmed_document | stable_source: EGRUL |
| Buyer_Name | LLC «SKIF» | confirmed_document | |
| Buyer_PostalAddress_PostalCode | 423800 | confirmed_document | |
| Buyer_PostalAddress_CountryCode | RU | confirmed_document | |
| Buyer_PostalAddress_CounryName | Russia | confirmed_document | |
| Buyer_PostalAddress_Region | Republic of Tatarstan | confirmed_document | |
| Buyer_PostalAddress_City | Naberezhnye Chelny | confirmed_document | |
| Buyer_PostalAddress_StreetHouse | Khlebny Passage, hause 30, office 211 | confirmed_document | |
| Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | confirmed_document | |
| Seler_PostalAddress_CountryCode | CN | confirmed_document | |
| Seler_PostalAddress_CounryName | China | confirmed_document | |
| Seler_PostalAddress_Region | Hebei | confirmed_document | |
| Seler_PostalAddress_City | Shijiazhuang | confirmed_document | |
| Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | confirmed_document | |
| Consignor_OrganizationName | pending | pending | отсутствует в инвойсе как отдельное поле |
| Consignor_Address_CountryCode | pending | pending | |
| Consignor_Address_CounryName | pending | pending | |
| Consignor_Address_Region | pending | pending | |
| Consignor_Address_City | pending | pending | |
| Consignor_Address_StreetHouse | pending | pending | |
| Consignee_OrganizationName | pending | pending | отсутствует в инвойсе как отдельное поле |
| Consignee_OGRN | pending | pending | |
| Consignee_INN | pending | pending | |
| Consignee_KPP | pending | pending | |
| Consignee_Address_PostalCode | pending | pending | |
| Consignee_Address_CountryCode | pending | pending | |
| Consignee_Address_CounryName | pending | pending | |
| Consignee_Address_Region | pending | pending | |
| Consignee_Address_City | pending | pending | |
| Consignee_Address_StreetHouse | pending | pending | |

#### InvoiceGoods_1
| field | value | status | note |
|---|---|---|---|
| GoodsCode | 5804101000 | confirmed_document | |
| GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester | confirmed_document | |
| GoodsQuantity | 2520 | confirmed_document | Quantity in M2 |
| MeasureUnitQualifierName | M2 | confirmed_document | |
| GrossWeightQuantity | 855.00 | confirmed_operator | operator_provided_data.md (from PL goods_1) |
| NetWeightQuantity | 806.60 | confirmed_operator | operator_provided_data.md (from PL goods_1) |
| Price | 5.85 | confirmed_document | price per M2 |
| TotalCost | 14742.00 | confirmed_document | |
| OriginCountryCode | 156 | confirmed_operator | operator_provided_data.md |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_operator | operator_provided_data.md |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | confirmed_operator | решение оператора |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | confirmed_operator | решение оператора |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | confirmed_operator | operator_provided_data.md |

#### InvoiceGoods_2
| field | value | status | note |
|---|---|---|---|
| GoodsCode | 5804101000 | confirmed_document | |
| GoodsDescription | Anti-cat mesh Roll size 1.6 *30 | confirmed_document | |
| GoodsQuantity | 1440 | confirmed_document | |
| MeasureUnitQualifierName | M2 | confirmed_document | |
| GrossWeightQuantity | 490.00 | confirmed_operator | from PL goods_2 |
| NetWeightQuantity | 460.80 | confirmed_operator | from PL goods_2 |
| Price | 5.85 | confirmed_document | |
| TotalCost | 8424.00 | confirmed_document | |
| OriginCountryCode | 156 | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | confirmed_operator | |

#### InvoiceGoods_3
| field | value | status | note |
|---|---|---|---|
| GoodsCode | 5804101000 | confirmed_document | |
| GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 | confirmed_document | |
| GoodsQuantity | 2520 | confirmed_document | |
| MeasureUnitQualifierName | M2 | confirmed_document | |
| GrossWeightQuantity | 265.00 | confirmed_operator | from PL goods_3 |
| NetWeightQuantity | 252.00 | confirmed_operator | from PL goods_3 |
| Price | 6.35 | confirmed_document | |
| TotalCost | 16002.00 | confirmed_document | |
| OriginCountryCode | 156 | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | confirmed_operator | |

#### InvoiceGoods_4
| field | value | status | note |
|---|---|---|---|
| GoodsCode | 5804101000 | confirmed_document | |
| GoodsDescription | ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2 | confirmed_document | |
| GoodsQuantity | 1440 | confirmed_document | |
| MeasureUnitQualifierName | M2 | confirmed_document | |
| GrossWeightQuantity | 155.00 | confirmed_operator | from PL goods_4 |
| NetWeightQuantity | 144.00 | confirmed_operator | from PL goods_4 |
| Price | 6.35 | confirmed_document | |
| TotalCost | 9144.00 | confirmed_document | |
| OriginCountryCode | 156 | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | confirmed_operator | |

#### InvoiceGoods_5
| field | value | status | note |
|---|---|---|---|
| GoodsCode | 7019900095 | confirmed_document | |
| GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 | confirmed_document | |
| GoodsQuantity | 3780 | confirmed_document | |
| MeasureUnitQualifierName | M2 | confirmed_document | |
| GrossWeightQuantity | 520.00 | confirmed_operator | from PL goods_5 |
| NetWeightQuantity | 491.40 | confirmed_operator | from PL goods_5 |
| Price | 3.4 | confirmed_document | |
| TotalCost | 12852.00 | confirmed_document | |
| OriginCountryCode | 156 | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | confirmed_operator | |

#### InvoiceGoods_6
| field | value | status | note |
|---|---|---|---|
| GoodsCode | 7019900095 | confirmed_document | |
| GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 | confirmed_document | |
| GoodsQuantity | 8640 | confirmed_document | |
| MeasureUnitQualifierName | M2 | confirmed_document | |
| GrossWeightQuantity | 1190.00 | confirmed_operator | from PL goods_6 |
| NetWeightQuantity | 1123.20 | confirmed_operator | from PL goods_6 |
| Price | 3.4 | confirmed_document | |
| TotalCost | 29376.00 | confirmed_document | |
| OriginCountryCode | 156 | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | confirmed_operator | |

#### InvoiceGoods_7
| field | value | status | note |
|---|---|---|---|
| GoodsCode | 5804101000 | confirmed_document | |
| GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 | confirmed_document | |
| GoodsQuantity | 240 | confirmed_document | |
| MeasureUnitQualifierName | M2 | confirmed_document | |
| GrossWeightQuantity | 25.00 | confirmed_operator | from PL goods_7 |
| NetWeightQuantity | 24.00 | confirmed_operator | from PL goods_7 |
| Price | 28 | confirmed_document | |
| TotalCost | 6720.00 | confirmed_document | |
| OriginCountryCode | 156 | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | confirmed_operator | |

### document: Packing List
- uqi_prefix: formalized.packing_list_1
- xml_target_root: AltaE2PACK
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/PL на сетку .pdf
- file_name: PL на сетку .pdf
- status: pending

| field | value | status | note |
|---|---|---|---|
| GrossWeightQuantity | 3500.00 | confirmed_document | |
| NetWeightQuantity | 3302.00 | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | confirmed_document | |
| Consignor_ShortName | pending | pending | |
| Consignor_Address_CountryCode | CN | confirmed_document | |
| Consignor_Address_CounryName | China | confirmed_document | |
| Consignor_Address_Region | Hebei | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street | confirmed_document | |
| Consignee_OrganizationName | LLC «SKIF» | confirmed_document | |
| Consignee_ShortName | pending | pending | |
| Consignee_OGRN | 1201600020390 | confirmed_document | stable_source: EGRUL |
| Consignee_INN | 1650389298 | confirmed_document | stable_source: EGRUL |
| Consignee_KPP | 165001001 | confirmed_document | stable_source: EGRUL |
| Consignee_Address_PostalCode | 423800 | confirmed_document | |
| Consignee_Address_CountryCode | RU | confirmed_document | |
| Consignee_Address_CounryName | Russia | confirmed_document | |
| Consignee_Address_Region | Republic of Tatarstan | confirmed_document | |
| Consignee_Address_City | Naberezhnye Chelny | confirmed_document | |
| Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | confirmed_document | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | confirmed_operator | решение оператора |
| DeliveryTerms_DeliveryTermsStringCode | EXW | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentName | SALES CONTRACT | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentDate | 2025-07-02 | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentName | INVOICE | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentDate | 2025-10-30 | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentName | Упаковочный лист | confirmed_operator | operator_provided_data.md |
| DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | confirmed_operator | operator_provided_data.md |
| DeliveryTerms_Registration_PrDocumentDate | 2025-10-30 | confirmed_operator | operator_provided_data.md |

#### Goods_1
| field | value | status | note |
|---|---|---|---|
| GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | confirmed_document | |
| GoodsQuantity | 60 | confirmed_document | Qty BG |
| GrossWeightQuantity | 855.00 | confirmed_document | |
| NetWeightQuantity | 806.60 | confirmed_document | |
| PackingInfo.PakingQuantity | 60 | confirmed_operator | решение оператора: =GoodsQuantity |

#### Goods_2
| field | value | status | note |
|---|---|---|---|
| GoodsDescription | Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | confirmed_document | |
| GoodsQuantity | 30 | confirmed_document | |
| GrossWeightQuantity | 490.00 | confirmed_document | |
| NetWeightQuantity | 460.80 | confirmed_document | |
| PackingInfo.PakingQuantity | 30 | confirmed_operator | |

#### Goods_3
| field | value | status | note |
|---|---|---|---|
| GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы ... Размер рулона 1,42*0,64*0,22 | confirmed_document | |
| GoodsQuantity | 6 | confirmed_document | |
| GrossWeightQuantity | 265.00 | confirmed_document | |
| NetWeightQuantity | 252.00 | confirmed_document | |
| PackingInfo.PakingQuantity | 6 | confirmed_operator | |

#### Goods_4
| field | value | status | note |
|---|---|---|---|
| GoodsDescription | ANTI-POLLEN MESH/Сетка против пыльцы ... Размер рулона 1,62*0,64*0,23 | confirmed_document | |
| GoodsQuantity | 3 | confirmed_document | |
| GrossWeightQuantity | 155.00 | confirmed_document | |
| NetWeightQuantity | 144.00 | confirmed_document | |
| PackingInfo.PakingQuantity | 3 | confirmed_operator | |

#### Goods_5
| field | value | status | note |
|---|---|---|---|
| GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" ... 1,42*0,55*0,18 | confirmed_document | |
| GoodsQuantity | 9 | confirmed_document | |
| GrossWeightQuantity | 520.00 | confirmed_document | |
| NetWeightQuantity | 491.40 | confirmed_document | |
| PackingInfo.PakingQuantity | 9 | confirmed_operator | |

#### Goods_6
| field | value | status | note |
|---|---|---|---|
| GoodsDescription | MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" ... 1,62*0,55*18 | confirmed_document | |
| GoodsQuantity | 18 | confirmed_document | |
| GrossWeightQuantity | 1190.00 | confirmed_document | |
| NetWeightQuantity | 1123.20 | confirmed_document | |
| PackingInfo.PakingQuantity | 18 | confirmed_operator | |

#### Goods_7
| field | value | status | note |
|---|---|---|---|
| GoodsDescription | GRID WITH 3 LAYER/Трехслойные сетки "Антипыльца" ... 1,72*0,35*0,31*1 | confirmed_document | |
| GoodsQuantity | 1 | confirmed_document | |
| GrossWeightQuantity | 25.00 | confirmed_document | |
| NetWeightQuantity | 24.00 | confirmed_document | |
| PackingInfo.PakingQuantity | 1 | confirmed_operator | |

#### TransportMeans_1
| field | value | status | note |
|---|---|---|---|
| Number | О157АО774 | confirmed_operator | operator_provided_data.md |
| ModeCode | 31 | confirmed_operator | |
| NationalityCode | 000 | confirmed_operator | |
| MoverIndicator | true | confirmed_operator | |

#### TransportMeans_2
| field | value | status | note |
|---|---|---|---|
| Number | ВТ374974 | confirmed_operator | operator_provided_data.md |
| ModeCode | 31 | confirmed_operator | |
| NationalityCode | 000 | confirmed_operator | |
| MoverIndicator | false | confirmed_operator | |

### document: CMR
- uqi_prefix: formalized.cmr_1
- xml_target_root: AltaE3CMR
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/СМР от СВХ.PNG
- file_name: СМР от СВХ.PNG
- status: pending

| field | value | status | note |
|---|---|---|---|
| LanguageCode | RU | confirmed_operator | operator_provided_data.md |
| CMR_Choice | 1 | confirmed_operator | operator_provided_data.md |
| RegistrationDocument_RegID | 00378 | confirmed_document | |
| RegistrationDocument_DateInf | 2026-01-20 | confirmed_document | |
| RegistrationDocument_Place | Маньчжурия | confirmed_document | |
| TrakingCargo_TakingCargoDate | 2026-01-20 | confirmed_document | |
| TrakingCargo_TakingCargoPlace_CountryCode | CN | confirmed_operator | решение оператора |
| TrakingCargo_TakingCargoPlace_CounryName | Китай | confirmed_document | |
| DeliveryPlace_CountryCode | RU | confirmed_operator | решение оператора |
| DeliveryPlace_CounryName | Россия | confirmed_operator | |
| DeliveryTerms_DeliveryPlace | pending | pending | не указано в CMR |
| DeliveryTerms_DeliveryTermsStringCode | pending | pending | |
| GoodsQuantity | 127 | confirmed_document | TOTAL 127 |
| CMRGoodsWeight_GrossWeightQuantity | 3500.00 | confirmed_document | |
| CMRTransport_PrimeMoverStateSignID | О157АО774 | confirmed_document | |
| CMRTransport_TrailerStateSignID | ВТ374974 | confirmed_document | |
| Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD | confirmed_document | |
| Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD | confirmed_operator | решение оператора: =полное |
| Consignor_PostalAddress_CountryCode | CN | confirmed_document | |
| Consignor_Address_CounryName | China | confirmed_document | |
| Consignor_Address_Region | Hebei | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street | confirmed_document | |
| Consignor_Guarantee_OrganizationName | pending | pending | отсутствует в CMR |
| Consignor_Guarantee_ShortName | pending | pending | |
| Consignor_Guarantee_Address_CountryCode | pending | pending | |
| Consignor_Guarantee_Address_CounryName | pending | pending | |
| Consignor_Guarantee_Address_Region | pending | pending | |
| Consignor_Guarantee_Address_City | pending | pending | |
| Consignor_Guarantee_Address_StreetHouse | pending | pending | |
| Consignee_NameInf | ООО «Скиф» | confirmed_document | |
| Consignee_ShortName | ООО «Скиф» | confirmed_operator | решение оператора: =полное |
| Consignee_OGRNID | 1201600020390 | confirmed_document | stable_source: EGRUL |
| Consignee_INNID | 1650389298 | confirmed_document | stable_source: EGRUL |
| Consignee_KPPCode | 165001001 | confirmed_document | stable_source: EGRUL |
| Consignee_PostalAddress_PostalCode | 423800 | confirmed_document | |
| Consignee_PostalAddress_CountryCode | RU | confirmed_document | |
| Consignee_Address_CounryName | Россия | confirmed_document | |
| Consignee_Address_Region | Республика Татарстан | confirmed_document | |
| Consignee_Address_City | Набережные Челны | confirmed_document | |
| Consignee_Address_StreetHouse | Хлебный пр-д, 30, офис 211 | confirmed_document | |

#### CMRGoods_1
| field | value | status | note |
|---|---|---|---|
| GoodsNumeric | 1 | confirmed_document | единственная строка |
| GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | confirmed_document | |
| GoodsNomenclatureCode | 5804101000 | confirmed_operator | operator_provided_data.md |
| GoodsQuantity | 127 | confirmed_document | TOTAL |
| GrossWeightQuantity | 3500.00 | confirmed_document | |
| GoodsPackingInfo.PackingCode | PX | confirmed_operator | operator_provided_data.md |
| GoodsPackingInfo.PakingQuantity | 127 | confirmed_operator | operator_provided_data.md |
| GoodsPackingInfo.PackingDescription | ПОДДОН | confirmed_operator | operator_provided_data.md |

### document: Payment Order
- uqi_prefix: formalized.payment_order_1
- xml_target_root: AltaPaymentOrder
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/платежки/currency_transfer_7_28.11.2025.pdf
- file_name: currency_transfer_7_28.11.2025.pdf
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| DocumentCode | 04023 | confirmed_operator | решение оператора |
| PaymentModeCode | 0 | confirmed_operator | operator_provided_data.md |
| PaymentAmount | 34041.00 | confirmed_document | |
| TransactionKind | 01 | confirmed_operator | operator_provided_data.md |
| Priority | 5 | confirmed_operator | решение оператора |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | confirmed_document | |
| ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | confirmed_document | |
| DocumentReference_PrDocumentNumber | 7 | confirmed_document | |
| DocumentReference_PrDocumentDate | 2025-11-28 | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | confirmed_document | |
| Payer_INN | 1650389298 | confirmed_document | |
| Payer_KPP | 165001001 | confirmed_operator | operator_provided_data.md |
| Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX | confirmed_document | |

#### PayerSign
| field | value | status | note |
|---|---|---|---|
| PersonSurname | Саранов | confirmed_document | подпись клиента |
| PersonName | Дмитрий | confirmed_document | |

### document: Payment Order
- uqi_prefix: formalized.payment_order_2
- xml_target_root: AltaPaymentOrder
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/платежки/currency_transfer_1_13.01.2026.pdf
- file_name: currency_transfer_1_13.01.2026.pdf
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| DocumentCode | 04023 | confirmed_operator | |
| PaymentModeCode | 0 | confirmed_operator | |
| PaymentAmount | 63219.00 | confirmed_document | |
| TransactionKind | 01 | confirmed_operator | |
| Priority | 5 | confirmed_operator | решение оператора |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | confirmed_document | |
| ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | confirmed_document | |
| DocumentReference_PrDocumentNumber | 1 | confirmed_document | |
| DocumentReference_PrDocumentDate | 2026-01-13 | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | confirmed_document | |
| Payer_INN | 1650389298 | confirmed_document | |
| Payer_KPP | 165001001 | confirmed_operator | |
| Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX | confirmed_document | |

#### PayerSign
| field | value | status | note |
|---|---|---|---|
| PersonSurname | Саранов | confirmed_document | |
| PersonName | Дмитрий | confirmed_document | |

### document: Service Invoice
- uqi_prefix: formalized.service_invoice_1
- xml_target_root: AltaServiceInvoice
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/Счет_№26-00378-tl_от_27-01-2026.pdf
- file_name: Счет_№26-00378-tl_от_27-01-2026.pdf
- status: pending

| field | value | status | note |
|---|---|---|---|
| DocumentSign | pending | pending | отсутствует в первичке |
| TotalServiceCost | 2700.00 | confirmed_document | |
| Currency | USD | confirmed_document | |
| ServiceProvider_Name | ООО «Трансимпериал» | confirmed_document | |
| ContractDetails_PrDocumentNumber | КООО/26651/М | confirmed_document | |
| ContractDetails_PrDocumentDate | 2025-05-13 | confirmed_document | |
| Registration_PrDocumentName | Счет на оплату | confirmed_document | |
| Registration_PrDocumentNumber | 26-00378-tl | confirmed_document | |
| Registration_PrDocumentDate | 2026-01-27 | confirmed_document | |
| Consignor_OrganizationName | pending | pending | |
| Consignee_OrganizationName | ООО "СКиФ" | confirmed_document | |
| Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | confirmed_document | stable_source: EGRUL |
| Consignee_RFOrganizationFeatures_INN | 1650389298 | confirmed_document | stable_source: EGRUL |
| Consignee_RFOrganizationFeatures_KPP | 165001001 | confirmed_document | stable_source: EGRUL |
| Signature_Choice | pending | pending | |
| SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | confirmed_document | |
| SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | confirmed_document | как в счете |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | confirmed_document | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | confirmed_document | |

#### ServiceProvider_PaymentRequisitions
| field | value | status | note |
|---|---|---|---|
| BankName | АО "Райффайзенбанк" | confirmed_document | |

#### PaymentDocument
| field | value | status | note |
|---|---|---|---|
| PrDocumentNumber | pending | pending | отсутствует |
| PrDocumentDate | pending | pending | |

#### Consignor_SubjectAddressDetails
| field | value | status | note |
|---|---|---|---|
| PostalCode | pending | pending | |
| CountryCode | pending | pending | |
| CounryName | pending | pending | |
| Region | pending | pending | |
| Town | pending | pending | |
| StreetHouse | pending | pending | |

#### Consignee_SubjectAddressDetails
| field | value | status | note |
|---|---|---|---|
| PostalCode | 423800 | confirmed_document | |
| CountryCode | RU | confirmed_document | |
| CounryName | Россия | confirmed_document | |
| Region | Республика Татарстан | confirmed_document | |
| Town | Набережные Челны | confirmed_document | |
| StreetHouse | проезд Хлебный, д. 30, кв/оф. 211 | confirmed_document | |
| House | pending | pending | |
| Room | pending | pending | |

#### ServiceDescription_1
| field | value | status | note |
|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 ... маршрут China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск) | confirmed_document | |
| CurrencyCode | USD | confirmed_document | |
| ServiceName | pending | pending | |
| TaxRate | 0.00 | confirmed_document | НДС 0% |
| TaxSum | 0.00 | confirmed_document | |
| ServiceCost_Amount | 1404.00 | confirmed_document | |
| ServiceCost_Currency | USD | confirmed_document | |

#### ServiceDescription_2
| field | value | status | note |
|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | confirmed_document | |
| CurrencyCode | USD | confirmed_document | |
| ServiceName | pending | pending | |
| TaxRate | 0.00 | confirmed_document | |
| TaxSum | 0.00 | confirmed_document | |
| ServiceCost_Amount | 1296.00 | confirmed_document | |
| ServiceCost_Currency | USD | confirmed_document | |

### document: Insurance Document
- uqi_prefix: formalized.insurance_document_1
- xml_target_root: AltaFreeDoc
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/Счет_№26-00378-tl_1_от_14-01-2026.pdf
- file_name: Счет_№26-00378-tl_1_от_14-01-2026.pdf
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| DocumentCode | 04111 | confirmed_operator | решение оператора: импортировать как страховой документ |
| DocumentHead_DocumentName | Счет на оплату | confirmed_document | |
| DocumentHead_DocumentDate | 2026-01-14 | confirmed_document | |
| DocumentHead_DocumentNumber | 26-00378-tl/1 | confirmed_document | |

#### DocumentBody_TextSection
| field | value | status | note |
|---|---|---|---|
| TextPara_1 | link: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/Счет_№26-00378-tl_1_от_14-01-2026.pdf | confirmed_document | хранить ссылкой |

### document: TechDescription
- uqi_prefix: formalized.tech_description_1
- xml_target_root: AltaFreeDoc
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/техничка .pdf
- file_name: техничка .pdf
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| DocumentCode | 05999 | confirmed_document | |
| DocumentHead_DocumentName | Техническое описание | confirmed_operator | классификация документа |
| DocumentHead_DocumentDate | 2025-10-30 | confirmed_operator | operator_provided_data.md |
| DocumentHead_DocumentNumber | Б/Н | confirmed_operator | operator_provided_data.md |

#### DocumentBody_TextSection
| field | value | status | note |
|---|---|---|---|
| TextPara_1 | link: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/техничка .pdf | confirmed_document | хранить ссылкой |

### document: TechDescription
- uqi_prefix: formalized.tech_description_2
- xml_target_root: AltaFreeDoc
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/техничка Антикот, антипыльца антимошка .pdf
- file_name: техничка Антикот, антипыльца антимошка .pdf
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| DocumentCode | 05999 | confirmed_document | |
| DocumentHead_DocumentName | Техническое описание | confirmed_operator | |
| DocumentHead_DocumentDate | 2025-10-30 | confirmed_operator | |
| DocumentHead_DocumentNumber | Б/Н | confirmed_operator | |

#### DocumentBody_TextSection
| field | value | status | note |
|---|---|---|---|
| TextPara_1 | link: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/техничка Антикот, антипыльца антимошка .pdf | confirmed_document | |

### document: Personal Passport
- uqi_prefix: formalized.passport_1
- xml_target_root: AltaPassport
- full_path: alta/stable_source/Passport_63_09_449948.xml
- file_name: Passport_63_09_449948.xml
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| CardSeries | 63 09 | confirmed_document | |
| CardNumber | 449948 | confirmed_document | |
| OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | confirmed_document | |
| CardDate | 2010-03-11 | confirmed_document | |
| PersonInfo_PersonSurname | АРБУЗОВА | confirmed_document | |
| PersonInfo_PersonName | АНАСТАСИЯ | confirmed_document | |
| PersonInfo_PersonMiddleName | КОНСТАНТИНОВНА | confirmed_document | |
| PersonInfo_Sex | 1 | confirmed_document | |
| PersonInfo_Birthday | 1987-07-25 | confirmed_document | |
| PersonInfo_Birthplace | город Саратов | confirmed_document | |
| ResidencePlace_PostalCode | 410052 | confirmed_document | |
| ResidencePlace_CountryCode | RU | confirmed_document | |
| ResidencePlace_CounryName | РОССИЯ | confirmed_document | |
| ResidencePlace_Region | Саратовская область | confirmed_document | |
| ResidencePlace_City | Саратов | confirmed_document | |
| ResidencePlace_StreetHouse | Ул. Одесская д 11 кв 160 | confirmed_document | |

### document: Letter of Attorney
- uqi_prefix: formalized.letter_of_attorney_1
- xml_target_root: AltaLetterOfAttorney
- full_path: alta/stable_source/LetterOfAttorney_1.xml
- file_name: LetterOfAttorney_1.xml
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| Subject | link: alta/stable_source/LetterOfAttorney_1.xml | confirmed_document | хранить ссылкой |
| EndDate | 2026-12-31 | confirmed_document | |
| DocumentReference_PrDocumentName | ДОВЕРЕННОСТЬ | confirmed_document | |
| DocumentReference_PrDocumentNumber | 1 | confirmed_document | |
| DocumentReference_PrDocumentDate | 2026-02-01 | confirmed_document | |
| Organization_OrganizationName | ООО «СКИФ» | confirmed_document | |
| Organization_ShortName | ООО «СКИФ» | confirmed_document | |
| Organization_OGRN | 1201600020390 | confirmed_document | |
| Organization_INN | 1650389298 | confirmed_document | |
| Organization_KPP | 165001001 | confirmed_document | |
| Organization_Address_PostalCode | 423800 | confirmed_document | |
| Organization_Address_CountryCode | RU | confirmed_document | |
| Organization_Address_CounryName | РОССИЯ | confirmed_document | |
| Organization_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | confirmed_document | |
| Organization_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | confirmed_document | |
| Organization_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | confirmed_document | |
| Organization_OrganizationPerson_PersonSurname | Саранов | confirmed_document | |
| Organization_OrganizationPerson_PersonName | Дмитрий | confirmed_document | |
| Organization_OrganizationPerson_PersonMiddleName | Олегович | confirmed_document | |
| Organization_OrganizationPerson_PersonPost | Директор | confirmed_document | |
| EmpoweredPerson_PersonSurname | АРБУЗОВА | confirmed_document | |
| EmpoweredPerson_PersonName | АНАСТАСИЯ | confirmed_document | |
| EmpoweredPerson_PersonMiddleName | КОНСТАНТИНОВНА | confirmed_document | |
| EmpoweredPerson_PersonPost | УПОЛНОМОЧЕННОЕ ЛИЦО | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardCode | RU01001 | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardName | ПАСРФ | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardSeries | 63 09 | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardNumber | 449948 | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardDate | 2010-03-11 | confirmed_document | |
| EmpoweredPerson_Passport_OrganizationName | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | confirmed_document | |

### document: Transport Contract
- uqi_prefix: formalized.transport_contract_1
- xml_target_root: AltaFreeDoc
- full_path: alta/stable_source/FreeDoc_КООО_26651_М.xml
- file_name: FreeDoc_КООО_26651_М.xml
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| DocumentCode | 04033 | confirmed_document | |
| DocumentHead_DocumentName | ДОГОВОР ПО ПЕРЕВОЗКЕ | confirmed_document | |
| DocumentHead_DocumentDate | 2025-05-13 | confirmed_document | |
| DocumentHead_DocumentNumber | КООО/26651/М | confirmed_document | |

#### DocumentBody_TextSection
| field | value | status | note |
|---|---|---|---|
| TextPara_1 | link: alta/stable_source/FreeDoc_КООО_26651_М.xml | confirmed_document | хранить ссылкой |

### document: EGRUL
- uqi_prefix: formalized.egrul_1
- xml_target_root: AltaFreeDoc
- full_path: alta/stable_source/FreeDoc_ЮЭ9965-25-106893283.xml
- file_name: FreeDoc_ЮЭ9965-25-106893283.xml
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| DocumentCode | 04011 | confirmed_document | |
| DocumentHead_DocumentName | ВЫПИСКА ИЗ ЕГРЮЛ | confirmed_document | |
| DocumentHead_DocumentDate | 2025-07-14 | confirmed_document | |
| DocumentHead_DocumentNumber | ЮЭ9965-25-106893283 | confirmed_document | |

#### DocumentBody_TextSection
| field | value | status | note |
|---|---|---|---|
| TextPara_1 | link: alta/stable_source/FreeDoc_ЮЭ9965-25-106893283.xml | confirmed_document | хранить ссылкой |

### document: Transit Declaration
- uqi_prefix: formalized.td_1
- xml_target_root: (n/a)
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/ТД 10719110_240126_5011363_reg 00378тд (1).pdf
- file_name: ТД 10719110_240126_5011363_reg 00378тд (1).pdf
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| number | 10719110/240126/5011363 | confirmed_document | |
| date | 2026-01-24 | confirmed_document | |

---

## II. non_formalized

### document: Storage Report (ДО-1)
- uqi_prefix: non_formalized.svh_1
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/ДО 14431420260204161621.PNG
- file_name: ДО 14431420260204161621.PNG
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| number | 0000080 | confirmed_document | отчет №0000080 |
| date | 2026-02-03 | confirmed_document | отчет от 03.02.2026 |
| warehouse_license | 10404/141210/10092/5 от 18.09.2025 | confirmed_document | СВХ ООО «ЛОГИКАМ» |
| actual_gross_weight | 3500 | confirmed_document | |
| actual_places | 127 | confirmed_document | |
| transport_reg_number | О157АО774/ВТ374974 | confirmed_document | |
| non_xml_fields | link: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/ДО 14431420260204161621.PNG | confirmed_document | строки: 7019900095 (27 мест, 1710 кг, 42228 CNY) и 5804101000 (100 мест, 1790 кг, 55032 CNY) |

### document: Storage Report Additional Sheet
- uqi_prefix: non_formalized.svh_additional_sheet_1
- full_path: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/ДО доп 14431520260204161645.PNG
- file_name: ДО доп 14431520260204161645.PNG
- status: confirmed

| field | value | status | note |
|---|---|---|---|
| number | 0000080 / добавочный лист №1 | confirmed_document | |
| date | 2026-02-03 | confirmed_document | |
| actual_gross_weight | 3500 | confirmed_document | |
| actual_places | 127 | confirmed_document | |
| transport_reg_number | О157АО774/ВТ374974 | confirmed_document | |
| non_xml_fields | link: alta/source/МоскитнаяСетка/HEBEI LANGMAI IMPORT AND EXPORT/02/ДО доп 14431520260204161645.PNG | confirmed_document | итог 127 мест, 3500 кг, 97260 CNY |

---

## III. Нерешенные вопросы (pending)

- formalized.supplementary_contract_1.ContractSignedPerson.PersonMiddleName
  - question: В доп.соглашении подписант продавца указан как "JING LI" без отчества. Подтверди: оставляем пустым (пустая строка) как confirmed_operator?

- formalized.invoice_1.DeliveryTerms_TradingCountryCode
  - question: В инвойсе нет отдельного поля TradingCountryCode. Подтверди: ставим "CN" (как DispatchCountryCode) и статус confirmed_operator?

- formalized.invoice_1.Consignor_OrganizationName
  - question: В инвойсе нет отдельного поля Consignor. Подтверди: заполняем consignor = seller (HEBEI LANGMAI...) и адрес как у seller, статус confirmed_operator?
- formalized.invoice_1.Consignor_Address_CountryCode
  - question: См. предыдущий вопрос (consignor= seller). Подтверди значение.
- formalized.invoice_1.Consignor_Address_CounryName
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignor_Address_Region
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignor_Address_City
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignor_Address_StreetHouse
  - question: См. предыдущий вопрос.

- formalized.invoice_1.Consignee_OrganizationName
  - question: В инвойсе нет отдельного поля Consignee. Подтверди: consignee = buyer (LLC SKIF / ООО "СКИФ") и реквизиты по stable_source, статус confirmed_operator?
- formalized.invoice_1.Consignee_OGRN
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignee_INN
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignee_KPP
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignee_Address_PostalCode
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignee_Address_CountryCode
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignee_Address_CounryName
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignee_Address_Region
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignee_Address_City
  - question: См. предыдущий вопрос.
- formalized.invoice_1.Consignee_Address_StreetHouse
  - question: См. предыдущий вопрос.

- formalized.packing_list_1.Consignor_ShortName
  - question: В PL ShortName не выделен. Подтверди: заполняем ShortName = OrganizationName, статус confirmed_operator?

- formalized.packing_list_1.Consignee_ShortName
  - question: В PL ShortName не выделен. Подтверди: заполняем ShortName = OrganizationName, статус confirmed_operator?

- formalized.cmr_1.DeliveryTerms_DeliveryPlace
  - question: В CMR не указано место поставки по Incoterms. Подтверди: оставляем пустым (confirmed_operator) или ставим "HEBEI"?

- formalized.cmr_1.DeliveryTerms_DeliveryTermsStringCode
  - question: В CMR не указаны условия Incoterms. Подтверди: ставим "EXW" как в инвойсе/PL (confirmed_operator)?

- formalized.cmr_1.Consignor_Guarantee_OrganizationName
  - question: В CMR нет блока гаранта отправителя. Подтверди: считаем "ОТСУТСТВУЕТ" (confirmed_operator) для всех полей блока Guarantee?

- formalized.service_invoice_1.DocumentSign
  - question: Для ServiceInvoice DocumentSign обычно "1", но в счете не указано. Подтверди: ставим "1" (confirmed_operator)?

- formalized.service_invoice_1.Signature_Choice
  - question: Для ServiceInvoice Signature_Choice: подставляем "1" (confirmed_operator)?

- formalized.service_invoice_1.PaymentDocument.PrDocumentNumber
  - question: В счете нет ссылки на платежный документ. Оставляем пустым (confirmed_operator)?

- formalized.service_invoice_1.PaymentDocument.PrDocumentDate
  - question: См. предыдущий.

- formalized.service_invoice_1.Consignor_OrganizationName
  - question: В счете consignor не указан. Подтверди: consignor = seller из инвойса (confirmed_operator)?

- formalized.service_invoice_1.Consignor_SubjectAddressDetails.PostalCode
  - question: См. предыдущий (consignor). Заполняем адрес как у seller?

- formalized.service_invoice_1.Consignor_SubjectAddressDetails.CountryCode
  - question: См. предыдущий.

- formalized.service_invoice_1.Consignor_SubjectAddressDetails.CounryName
  - question: См. предыдущий.

- formalized.service_invoice_1.Consignor_SubjectAddressDetails.Region
  - question: См. предыдущий.

- formalized.service_invoice_1.Consignor_SubjectAddressDetails.Town
  - question: См. предыдущий.

- formalized.service_invoice_1.Consignor_SubjectAddressDetails.StreetHouse
  - question: См. предыдущий.

- formalized.service_invoice_1.Consignee_SubjectAddressDetails.House
  - question: В счете адрес одной строкой. Подтверди: House=30 (confirmed_operator) или оставить пустым?

- formalized.service_invoice_1.Consignee_SubjectAddressDetails.Room
  - question: Подтверди: Room=211 (confirmed_operator) или оставить пустым?

- formalized.service_invoice_1.ServiceDescription_1.ServiceName
  - question: ServiceName в счете не выделен. Подтверди: можно оставить пустым (confirmed_operator)?

- formalized.service_invoice_1.ServiceDescription_2.ServiceName
  - question: Аналогично.
