# primary.md — ПриточнаяВентиляция (этап 1)

- название кейса: ПриточнаяВентиляция
- путь к папке поставки: alta\\source\\ПриточнаяВентиляция\\1\\
- тип поставки: 1 партия / 1 товар / автоперевозка (CMR)
- примечание: первичка распознана в md (этап 0). Часть md помечена как best-effort со вставками [...].

---

# I. formalized

## document: Contract (03011)
- uqi_prefix: formalized.contract_1
- xml_target_root: AltaE2CONT
- path: alta\\source\\ПриточнаяВентиляция\\1 Supplementary agreement to the _25AZC003.pdf
- file_name: 1 Supplementary agreement to the _25AZC003.pdf
- status: pending
- note: По содержимому это Sales Contract No:25AZC003 (см. review_0.md). Полный текст контракта хранить как link.

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 03011 | код вида документа для графы 44 | confirmed_document | |
| ContractRegistration_PrDocumentNumber | 25AZC003 | № контракта | confirmed_document | SALES CONTRACT No:25AZC003 |
| ContractRegistration_PrDocumentDate | 2025-04-10 | дата контракта | confirmed_document | CONTRACT DATE: APR 10 2025 |
| ContractTerms_Amount | 136000 | общая сумма контракта | confirmed_document | RMB |
| ContractTerms_CurrencyCode | 156 | ISO 4217 numeric | pending | в тексте валюта RMB; numeric код для CNY=156 по cb:country/cb:unit? (нужно подтверждение, что подразумевается CNY) |
| ContractTerms_LastDate | 2026-12-31 | срок исполнения | confirmed_document | The date for fulfilment... December 31, 2026 |
| ContractTerms_OtherTerms | EXW Ningbo | Incoterms | confirmed_document | EXW указан в контракте и инвойсе |
| ContractTerms_ContractText | link:alta\\source\\ПриточнаяВентиляция\\1 Supplementary agreement to the _25AZC003.pdf | текст контракта (link) | confirmed_document | |
| ContractTerms_DealSign | 1 | системный признак | confirmed_document | default |
| ForeignPerson_OrganizationName | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | продавец | confirmed_document | |
| ForeignPerson_Address_CountryCode | CN | страна продавца alpha-2 | confirmed_document | |
| ForeignPerson_Address_CounryName | China | страна продавца | confirmed_document | |
| ForeignPerson_Address_Region | Haishu District | регион | confirmed_document | из адреса Add:D4-109...Haishu District |
| ForeignPerson_Address_City | Ningbo | город | confirmed_document | |
| ForeignPerson_Address_StreetHouse | D4-109, Liangzhu Culture Park | улица/дом | confirmed_document | остальная часть адреса: 315175 Ningbo, CHINA |
| RussianPerson_OrganizationName | LLC «SKIF» | покупатель | confirmed_document | |
| RussianPerson_OGRN | 1201600020390 | ОГРН покупателя | confirmed_document | stable_source: EGRUL |
| RussianPerson_INN | 1650389298 | ИНН | confirmed_document | stable_source + платежка + транзитка |
| RussianPerson_KPP | 165001001 | КПП | confirmed_document | stable_source + платежка + транзитка |
| RussianPerson_Address_PostalCode | 423800 | индекс | confirmed_document | |
| RussianPerson_Address_CountryCode | RU | страна alpha-2 | confirmed_document | |
| RussianPerson_Address_CounryName | Russia | страна | confirmed_document | |
| RussianPerson_Address_Region | Republic of Tatarstan | регион | confirmed_document | |
| RussianPerson_Address_City | Naberezhnye Chelny | город | confirmed_document | |
| RussianPerson_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | улица/дом/офис | confirmed_document | |

## document: Invoice (04021)
- uqi_prefix: formalized.invoice_1
- xml_target_root: AltaE2I
- path: alta\\source\\ПриточнаяВентиляция\\1\\Инвойс 25AZC003B.pdf
- file_name: Инвойс 25AZC003B.pdf
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| CurrencyRate |  | курс валюты | pending | нужен курс на дату оформления ДТ (обычно берется из таможни/ЦБ) |
| CurrencyCode | CNY | валюта инвойса ISO alpha-3 | confirmed_document | |
| DocumentCode | 04021 | код вида документа | confirmed_document | |
| PlacesQuantity | 2 | кол-во мест | confirmed_document | CMR/ДО-1: 2 cl/2 места |
| PlacesDescription |  | описание мест | pending | в документах: "2 cl" без расшифровки |
| GrossWeightQuantity | 383 | общий брутто | confirmed_document | |
| NetWeightQuantity | 312.50 | общий нетто | confirmed_document | из PL |
| GCost | 13600 | дубль TotalCost | confirmed_document | |
| TotalCost | 13600 | итого | confirmed_document | RMB/CNY |
| DeliveryTerms_DeliveryPlace | Ningbo | место поставки | confirmed_document | EXW Ningbo |
| DeliveryTerms_DeliveryTermsNumericCode |  | код условий | pending | нужен внутренний код Альты для EXW |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия | confirmed_document | |
| DeliveryTerms_DispatchCountryCode | CN | страна отправления | confirmed_document | |
| DeliveryTerms_TradingCountryCode | CN | торгующая страна | confirmed_document | |
| DeliveryTerms_DestinationCountryCode | RU | страна назначения | confirmed_document | |
| Registration_PrDocumentName | COMMERCIAL INVOICE | наименование | confirmed_document | |
| Registration_PrDocumentNumber | 25AZC003B | номер инвойса | confirmed_document | |
| Registration_PrDocumentDate | 2025-04-10 | дата | confirmed_document | |
| Contract_PrDocumentNumber | 25AZC003 | № контракта-ссылки | confirmed_document | |
| Contract_PrDocumentDate | 2025-04-10 | дата контракта | confirmed_document | |
| Buyer_CompanyID | 1650389298 | ИНН покупателя | confirmed_document | |
| Buyer_KPPCode | 165001001 | КПП | confirmed_document | |
| Buyer_Name | LLC "SKIF" | покупатель | confirmed_document | |
| Buyer_PostalAddress_PostalCode | 423800 | индекс | confirmed_document | |
| Buyer_PostalAddress_CountryCode | RU | страна alpha-2 | confirmed_document | |
| Buyer_PostalAddress_CounryName | RUSSIAN FEDERATION | страна | confirmed_document | |
| Buyer_PostalAddress_Region | Republic of Tatarstan | регион | confirmed_document | |
| Buyer_PostalAddress_City | Naberezhnye Chelny | город | confirmed_document | |
| Buyer_PostalAddress_StreetHouse | Khlebny Passage, hause 30, office 211 | улица/дом | confirmed_document | |
| Seler_Name | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | продавец | confirmed_document | |
| Seler_PostalAddress_CountryCode | CN | страна | confirmed_document | |
| Seler_PostalAddress_CounryName | CHINA | страна | confirmed_document | |
| Seler_PostalAddress_Region | Haishu District | регион | confirmed_document | |
| Seler_PostalAddress_City | Ningbo | город | confirmed_document | |
| Seler_PostalAddress_StreetHouse | D4-109, Liangzhu Culture Park | улица/дом | confirmed_document | 315175 Ningbo |
| Consignor_OrganizationName | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | грузоотправитель | confirmed_document | |
| Consignor_Address_CountryCode | CN | страна | confirmed_document | |
| Consignor_Address_CounryName | CHINA | страна | confirmed_document | |
| Consignor_Address_Region | Haishu District | регион | confirmed_document | |
| Consignor_Address_City | Ningbo | город | confirmed_document | |
| Consignor_Address_StreetHouse | D4-109, Liangzhu Culture Park | улица/дом | confirmed_document | |
| Consignee_OrganizationName | LLC "SKIF" | грузополучатель | confirmed_document | |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | stable_source |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | RUSSIAN FEDERATION | страна | confirmed_document | |
| Consignee_Address_Region | Republic of Tatarstan | регион | confirmed_document | |
| Consignee_Address_City | Naberezhnye Chelny | город | confirmed_document | |
| Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | улица/дом | confirmed_document | |

### InvoiceGoods_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsCode | 8481309908 | ТН ВЭД | confirmed_document | |
| GoodsDescription | Plastic air vent / air infiltration valve | описание | confirmed_document | |
| GoodsQuantity | 1000 | количество | confirmed_document | sets/pcs |
| goods_supplementary_quantity |  | доп.кол-во | pending | доп.единица для ДТ (если нужна) |
| goods_supplementary_uom_name |  | наименование доп.ед. | pending | |
| MeasureUnitQualifierName | шт | единица | pending | в первичке "Sets"; для ДТ обычно "шт" (код 796) — требуется подтверждение |
| GrossWeightQuantity | 383 | брутто | confirmed_document | по поставке; в инвойсе строка как 383kg по CMR/ДО-1 |
| NetWeightQuantity | 312.50 | нетто | confirmed_document | из PL |
| Price | 13.60 | цена | confirmed_document | RMB |
| TotalCost | 13600.00 | стоимость | confirmed_document | |
| OriginCountryCode | 156 | цифровой код страны происхождения | confirmed_document | CHINA → 156 |
| AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | производитель | confirmed_document | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | ТМ | confirmed_operator | по умолчанию, если нет в первичке |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | маркировка | confirmed_operator | по умолчанию |
| AdditionalGoodsDescription_GoodsModel | KIV-125 | модель/обозначение | confirmed_document | из PL/технички |

## document: Packing List (04131)
- uqi_prefix: formalized.packing_list_1
- xml_target_root: AltaE2PACK
- path: alta\\source\\ПриточнаяВентиляция\\1\\PL 25AZC003B.pdf
- file_name: PL 25AZC003B.pdf
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| GrossWeightQuantity | 383.00 | общий брутто | confirmed_document | |
| NetWeightQuantity | 312.50 | общий нетто | confirmed_document | |
| Consignor_OrganizationName | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | отправитель | confirmed_document | |
| Consignor_ShortName |  | краткое | pending | |
| Consignor_Address_CountryCode | CN | страна | confirmed_document | |
| Consignor_Address_CounryName | CHINA | страна | confirmed_document | |
| Consignor_Address_Region | Haishu District | регион | confirmed_document | |
| Consignor_Address_City | Ningbo | город | confirmed_document | |
| Consignor_Address_StreetHouse | D4-109, Liangzhu Culture Park | улица/дом | confirmed_document | |
| Consignee_OrganizationName | LLC "SKIF" | получатель | confirmed_document | |
| Consignee_ShortName |  | краткое | pending | |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | stable_source |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | RUSSIAN FEDERATION | страна | confirmed_document | |
| Consignee_Address_Region | Republic of Tatarstan | регион | confirmed_document | |
| Consignee_Address_City | Naberezhnye Chelny | город | confirmed_document | |
| Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | адрес | confirmed_document | |
| DeliveryTerms_DeliveryPlace | Ningbo | место | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode |  | код условий | pending | внутренний код EXW |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentName | SALES CONTRACT | наименование | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentNumber | 25AZC003 | № | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentDate | 2025-04-10 | дата | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentName | COMMERCIAL INVOICE | наименование | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentNumber | 25AZC003B | № | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentDate | 2025-04-10 | дата | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentName | PACKING LIST | наименование | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentNumber | 25AZC003B | № | confirmed_document | по PL "INVOICE NUMBER" |
| DeliveryTerms_Registration_PrDocumentDate | 2025-04-10 | дата | confirmed_document | |
| registration_doc_name | УПАКОВОЧНЫЙ ЛИСТ | печать графа 44 | confirmed_operator | |
| registration_doc_number | 25AZC003B | номер | confirmed_document | |
| registration_doc_date | 2025-04-10 | дата | confirmed_document | |

### Goods_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Plastic air vent KIV-125 / Пластиковый воздухозаборник КИВ-125 | описание грузовой строки | confirmed_document | |
| GoodsQuantity | 2 | кол-во мест/груз.единиц | confirmed_document | "2" cartons? в таблице Ctn Qty=2 |
| GrossWeightQuantity | 383.00 | брутто | confirmed_document | |
| NetWeightQuantity | 312.50 | нетто | confirmed_document | |
| PackingInfo.PakingQuantity | 24 pcs | кол-во в месте | confirmed_document | Qty/Ctn 24 pcs |

### TransportMeans_1
| field | value | description | status | note |
|---|---|---|---|---|
| Number |  | номер ТС | pending | PL не содержит номера ТС |
| ModeCode | 31 | код транспорта | confirmed_document | из ДО-1: код вида транспорта 31 |
| NationalityCode | 000 | национальность | pending | нет в первичке |
| MoverIndicator | true | тягач | pending | нет данных |

## document: CMR (02015)
- uqi_prefix: formalized.cmr_1
- xml_target_root: AltaE3CMR
- path: alta\\source\\ПриточнаяВентиляция\\1\\СМР.pdf
- file_name: СМР.pdf
- status: pending
- note: md получен best-effort из скриншотов, см. CMR_12327 (parsed from screenshots).md

| field | value | description | status | note |
|---|---|---|---|---|
| LanguageCode | RU | язык | confirmed_operator | |
| CMR_Choice | 1 | системный признак | confirmed_operator | default |
| RegistrationDocument_RegID | 12327 | номер CMR | confirmed_document | |
| RegistrationDocument_DateInf | 2025-07-01 | дата | confirmed_document | |
| RegistrationDocument_Place |  | место составления | pending | |
| TrakingCargo_TakingCargoDate | 2025-07-01 | дата принятия | confirmed_document | |
| TrakingCargo_TakingCargoPlace_CountryCode | CN | страна принятия | confirmed_document | |
| TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | страна принятия | confirmed_document | |
| DeliveryPlace_CountryCode | RU | страна доставки | confirmed_document | |
| DeliveryPlace_CounryName | РОССИЯ | страна доставки | confirmed_document | |
| DeliveryTerms_DeliveryPlace |  | место по Incoterms | pending | |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия | confirmed_document | из заявки; в CMR не указано явно |
| GoodsQuantity | 2 | кол-во мест | confirmed_document | 2 cl |
| CMRGoodsWeight_GrossWeightQuantity | 383 | общий брутто | confirmed_document | |
| CMRTransport_PrimeMoverStateSignID | А488ОУ67 | тягач | confirmed_document | |
| CMRTransport_TrailerStateSignID | А67261-5 | прицеп | confirmed_document | |
| Consignor_NameInf | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | отправитель | confirmed_document | |
| Consignor_ShortName |  | краткое | pending | |
| Consignor_PostalAddress_CountryCode | CN | страна | confirmed_document | |
| Consignor_Address_CounryName | CHINA | страна | confirmed_document | |
| Consignor_Address_Region | Haishu District | регион | pending | в md не выделено явно |
| Consignor_Address_City | Ningbo | город | confirmed_document | |
| Consignor_Address_StreetHouse | D4-109, Liangzhu Culture Park | адрес | confirmed_document | |
| Consignor_Guarantee_OrganizationName |  | гарант | pending | в CMR неизвестно |
| Consignor_Guarantee_ShortName |  | гарант | pending | |
| Consignor_Guarantee_Address_CountryCode |  | страна | pending | |
| Consignor_Guarantee_Address_CounryName |  | страна | pending | |
| Consignor_Guarantee_Address_Region |  | регион | pending | |
| Consignor_Guarantee_Address_City |  | город | pending | |
| Consignor_Guarantee_Address_StreetHouse |  | адрес | pending | |
| Consignee_NameInf | LLC "SKIF" | получатель | confirmed_document | |
| Consignee_ShortName |  | краткое | pending | |
| Consignee_OGRNID | 1201600020390 | ОГРН | confirmed_document | stable_source |
| Consignee_INNID | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPPCode | 165001001 | КПП | confirmed_document | |
| Consignee_PostalAddress_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_PostalAddress_CountryCode | RU | страна | confirmed_document | |
| Consignee_Address_CounryName | RUSSIAN FEDERATION | страна | confirmed_document | |
| Consignee_Address_Region | Republic of Tatarstan | регион | confirmed_document | |
| Consignee_Address_City | Naberezhnye Chelny | город | confirmed_document | |
| Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | адрес | confirmed_document | |

### CMRGoods_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsNumeric | 1 | номер строки | confirmed_operator | |
| GoodsDescription | Приточный клапан | описание | confirmed_document | |
| GoodsPackingInfo.PakingQuantity | 2 | кол-во мест | confirmed_document | 2 cl |

## document: Payment Order / Заявление на перевод (04023)
- uqi_prefix: formalized.payment_order_1
- xml_target_root: AltaPaymentOrder
- path: alta\\source\\ПриточнаяВентиляция\\1\\Платежка.pdf
- file_name: Платежка.pdf
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04023 | код | confirmed_document | |
| PaymentModeCode |  | системный код | pending | |
| PaymentAmount | 13600.00 | сумма | confirmed_document | CNY |
| TransactionKind |  | вид операции | pending | |
| Priority |  | очередность | pending | |
| Purpose | PURCHASE OF AN PLASTIC AIR VENT. CONTRACT NO.: 25AZC003, DATE: APR 10, 2025 | назначение | confirmed_document | |
| ValueSpelledOut | Тринадцать тысяч шестьсот юаней 00/100 | сумма прописью | confirmed_document | |
| DocumentReference_PrDocumentNumber | 1 | № заявления | confirmed_document | |
| DocumentReference_PrDocumentDate | 2025-05-21 | дата | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | плательщик | confirmed_document | |
| Payer_INN | 1650389298 | ИНН | confirmed_document | |
| Payer_KPP | 165001001 | КПП | confirmed_document | |
| Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) БИК 044525411 | банк плательщика | confirmed_document | |
| Payee_OrganizationName | NINGBO ZENTEC AIR CONDITIONING AND REFRIGERATION CO., LTD | получатель | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX; CNAPS 767290000018; A/C 40807156800610017774 | банк получателя | confirmed_document | |

### PayerSign
| field | value | description | status | note |
|---|---|---|---|---|
| PersonSurname | Саранов | фамилия | confirmed_document | подпись клиента |
| PersonName | Дмитрий Олегович | имя/отчество | confirmed_document | |

## document: Service invoice / Счет за перевозку (04031) — USD
- uqi_prefix: formalized.service_invoice_1
- xml_target_root: AltaServiceInvoice
- path: alta\\source\\ПриточнаяВентиляция\\1\\документы от Любы\\Счет_№25-12327-k_от_22-05-2025 (2).pdf
- file_name: Счет_№25-12327-k_от_22-05-2025 (2).pdf
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentSign | 1 | системный признак | confirmed_operator | default |
| TotalServiceCost | 1200.00 | итого | confirmed_document | USD |
| Currency | USD | валюта | confirmed_document | |
| ServiceProvider_Name | ООО «Трансимпериал» | исполнитель | confirmed_document | |
| ServiceProvider_PaymentRequisitions.BankName | АО "Райффайзенбанк" | банк исполнителя | confirmed_document | |
| ContractDetails_PrDocumentNumber | КООО/26651/М | № договора | confirmed_document | |
| ContractDetails_PrDocumentDate | 2025-05-13 | дата договора | confirmed_document | |
| PaymentDocument.PrDocumentNumber | 1 | заявка № | confirmed_document | |
| PaymentDocument.PrDocumentDate | 2025-05-21 | дата заявки | confirmed_document | |
| Registration_PrDocumentName | Счет на оплату | наименование | confirmed_document | |
| Registration_PrDocumentNumber | 25-12327-k | номер счета | confirmed_document | |
| Registration_PrDocumentDate | 2025-05-22 | дата счета | confirmed_document | |
| Consignor_OrganizationName | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | грузоотправитель | pending | в счете не указан напрямую |
| Consignor_SubjectAddressDetails.PostalCode |  | индекс | pending | |
| Consignor_SubjectAddressDetails.CountryCode |  | страна | pending | |
| Consignor_SubjectAddressDetails.CounryName |  | страна | pending | |
| Consignor_SubjectAddressDetails.Region |  | регион | pending | |
| Consignor_SubjectAddressDetails.Town |  | город | pending | |
| Consignor_SubjectAddressDetails.StreetHouse |  | улица | pending | |
| Consignee_OrganizationName | ООО "СКиФ" | грузополучатель | confirmed_document | |
| Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | ОГРН | confirmed_document | stable_source |
| Consignee_RFOrganizationFeatures_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_RFOrganizationFeatures_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_SubjectAddressDetails.PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_SubjectAddressDetails.CountryCode | RU | страна | confirmed_document | |
| Consignee_SubjectAddressDetails.CounryName | РОССИЯ | страна | confirmed_document | |
| Consignee_SubjectAddressDetails.Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_SubjectAddressDetails.Town | Набережные Челны | город | confirmed_document | |
| Consignee_SubjectAddressDetails.StreetHouse | проезд Хлебный | улица | confirmed_document | |
| Consignee_SubjectAddressDetails.House | 30 | дом | confirmed_document | |
| Consignee_SubjectAddressDetails.Room | 211 | офис | confirmed_document | |
| Signature_Choice | 1 | вариант подписи | pending | |
| SignatureDirectorChiefAccountant_Director_PersonSurname |  | директор | pending | |
| SignatureDirectorChiefAccountant_Director_PersonName |  | директор | pending | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname |  | бухгалтер | pending | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonName |  | бухгалтер | pending | |

### ServiceDescription_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по заявке №1 от 21.05.2025 по маршруту: China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | описание | confirmed_document | |
| CurrencyCode | USD | валюта | confirmed_document | |
| ServiceName | China, Ningbo - Маньчжурия/Забайкальск | маршрут | confirmed_document | |
| TaxRate | 0% | ставка | confirmed_document | |
| TaxSum | 0.00 | налог | confirmed_document | |
| ServiceCost_Amount | 624.00 | стоимость | confirmed_document | |
| ServiceCost_Currency | USD | валюта | confirmed_document | |

### ServiceDescription_2
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, г. Набережные Челны, Производственный пр-д | описание | confirmed_document | |
| CurrencyCode | USD | валюта | confirmed_document | |
| ServiceName | Маньчжурия/Забайкальск - Набережные Челны | маршрут | confirmed_document | |
| TaxRate | 0% | ставка | confirmed_document | |
| TaxSum | 0.00 | налог | confirmed_document | |
| ServiceCost_Amount | 576.00 | стоимость | confirmed_document | |
| ServiceCost_Currency | USD | валюта | confirmed_document | |

## document: Service invoice / Счет за перевозку (04031) — RUB
- uqi_prefix: formalized.service_invoice_2
- xml_target_root: AltaServiceInvoice
- path: alta\\source\\ПриточнаяВентиляция\\1\\документы от Любы\\Счет_№25-12327-k_1_от_22-05-2025 (3).pdf
- file_name: Счет_№25-12327-k_1_от_22-05-2025 (3).pdf
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentSign | 1 | системный признак | confirmed_operator | |
| TotalServiceCost | 30000.00 | итого | confirmed_document | RUB |
| Currency | RUB | валюта | confirmed_document | |
| ServiceProvider_Name | ООО «Трансимпериал» | исполнитель | confirmed_document | |
| ServiceProvider_PaymentRequisitions.BankName | АО "Райффайзенбанк" | банк | confirmed_document | |
| ContractDetails_PrDocumentNumber | КООО/26651/М | № договора | confirmed_document | |
| ContractDetails_PrDocumentDate | 2025-05-13 | дата | confirmed_document | |
| PaymentDocument.PrDocumentNumber | 1 | заявка № | confirmed_document | |
| PaymentDocument.PrDocumentDate | 2025-05-21 | дата | confirmed_document | |
| Registration_PrDocumentName | Счет на оплату | наименование | confirmed_document | |
| Registration_PrDocumentNumber | 25-12327-k/1 | номер | confirmed_document | |
| Registration_PrDocumentDate | 2025-05-22 | дата | confirmed_document | |
| Consignor_OrganizationName |  | грузоотправитель | pending | |
| Consignor_SubjectAddressDetails.PostalCode |  | индекс | pending | |
| Consignor_SubjectAddressDetails.CountryCode |  | страна | pending | |
| Consignor_SubjectAddressDetails.CounryName |  | страна | pending | |
| Consignor_SubjectAddressDetails.Region |  | регион | pending | |
| Consignor_SubjectAddressDetails.Town |  | город | pending | |
| Consignor_SubjectAddressDetails.StreetHouse |  | улица | pending | |
| Consignee_OrganizationName | ООО "СКиФ" | грузополучатель | confirmed_document | |
| Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_RFOrganizationFeatures_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_RFOrganizationFeatures_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_SubjectAddressDetails.PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_SubjectAddressDetails.CountryCode | RU | страна | confirmed_document | |
| Consignee_SubjectAddressDetails.CounryName | РОССИЯ | страна | confirmed_document | |
| Consignee_SubjectAddressDetails.Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_SubjectAddressDetails.Town | Набережные Челны | город | confirmed_document | |
| Consignee_SubjectAddressDetails.StreetHouse | проезд Хлебный | улица | confirmed_document | |
| Consignee_SubjectAddressDetails.House | 30 | дом | confirmed_document | |
| Consignee_SubjectAddressDetails.Room | 211 | офис | confirmed_document | |
| Signature_Choice | 1 | вариант подписи | pending | |
| SignatureDirectorChiefAccountant_Director_PersonSurname |  | директор | pending | |
| SignatureDirectorChiefAccountant_Director_PersonName |  | директор | pending | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname |  | бухгалтер | pending | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonName |  | бухгалтер | pending | |

### ServiceDescription_1
| field | value | description | status | note |
|---|---|---|---|---|
| GoodsDescription | Транспортно-экспедиционные услуги по договору №КООО/26651/М от 13-05-2025 по заявке №1 от 21.05.2025 по маршруту: Россия, Республика Татарстан, г. Набережные Челны, Производственный пр-д - Россия, Республика Татарстан, г. Набережные Челны, Транспортный пр-д; перевозка автотранспортом | описание | confirmed_document | |
| CurrencyCode | RUB | валюта | confirmed_document | |
| ServiceName | Набережные Челны (Производственный пр-д) - Набережные Челны (Транспортный пр-д) | маршрут | confirmed_document | |
| TaxRate | 20% | ставка | confirmed_document | |
| TaxSum | 5000.00 | НДС | confirmed_document | |
| ServiceCost_Amount | 25000.00 | стоимость | confirmed_document | |
| ServiceCost_Currency | RUB | валюта | confirmed_document | |

## document: TechDescription (05999)
- uqi_prefix: formalized.tech_description_1
- xml_target_root: AltaFreeDoc
- path: alta\\source\\ПриточнаяВентиляция\\1\\тех описание\\техничка КИВ 125.pdf
- file_name: техничка КИВ 125.pdf
- status: pending

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 05999 | код вида документа | confirmed_document | |
| DocumentHead_DocumentName | Технические характеристики КИВ-125 | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2025-04-10 | дата | confirmed_document | Исх номер 1СК1004 от 10.04.2025 |
| DocumentHead_DocumentNumber | 1СК1004 | номер | confirmed_document | |

### DocumentBody_TextSection.TextPara_1
| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\\source\\ПриточнаяВентиляция\\1\\тех описание\\техничка КИВ 125.pdf | текст | confirmed_document | распознана только стр.1, стр.2 — изображения |

## document: Transport Contract (04033)
- uqi_prefix: formalized.transport_contract_1
- xml_target_root: AltaFreeDoc
- path: alta\\stable_source\\FreeDoc_КООО_26651_М.xml
- file_name: FreeDoc_КООО_26651_М.xml
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04033 | код | confirmed_document | |
| DocumentHead_DocumentName | ДОГОВОР ПО ПЕРЕВОЗКЕ | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2025-05-13 | дата | confirmed_document | |
| DocumentHead_DocumentNumber | КООО/26651/М | номер | confirmed_document | |

### DocumentBody_TextSection.TextPara_1
| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\\stable_source\\FreeDoc_КООО_26651_М.xml | текст | confirmed_document | использовать содержимое TextPara из xml |

## document: EGRUL (04011)
- uqi_prefix: formalized.egrul_1
- xml_target_root: AltaFreeDoc
- path: alta\\stable_source\\FreeDoc_ЮЭ9965-25-106893283.xml
- file_name: FreeDoc_ЮЭ9965-25-106893283.xml
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| DocumentCode | 04011 | код | confirmed_document | |
| DocumentHead_DocumentName | ВЫПИСКА ИЗ ЕГРЮЛ | наименование | confirmed_document | |
| DocumentHead_DocumentDate | 2025-07-14 | дата | confirmed_document | |
| DocumentHead_DocumentNumber | ЮЭ9965-25-106893283 | номер | confirmed_document | |

### DocumentBody_TextSection.TextPara_1
| field | value | description | status | note |
|---|---|---|---|---|
| TextPara | link:alta\\stable_source\\FreeDoc_ЮЭ9965-25-106893283.xml | текст | confirmed_document | использовать TextPara из xml |

## document: Passport (11001)
- uqi_prefix: formalized.passport_1
- xml_target_root: AltaPassport
- path: alta\\stable_source\\Passport_63_09_449948.xml
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
| ResidencePlace_CounryName | РОССИЯ | страна | confirmed_document | |
| ResidencePlace_Region | Саратовская область | регион | confirmed_document | |
| ResidencePlace_City | Саратов | город | confirmed_document | |
| ResidencePlace_StreetHouse | Ул. Одесская д 11 кв 160 | адрес | confirmed_document | |

## document: Letter of Attorney (11004)
- uqi_prefix: formalized.letter_of_attorney_1
- xml_target_root: AltaLetterOfAttorney
- path: alta\\stable_source\\LetterOfAttorney_1.xml
- file_name: LetterOfAttorney_1.xml
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| Subject | link:alta\\stable_source\\LetterOfAttorney_1.xml | текст доверенности | confirmed_document | использовать Subject из xml |
| EndDate | 2026-12-31 | действительна до | confirmed_document | |
| DocumentReference_PrDocumentName | ДОВЕРЕННОСТЬ | наименование | confirmed_document | |
| DocumentReference_PrDocumentNumber | 1 | номер | confirmed_document | |
| DocumentReference_PrDocumentDate | 2026-02-01 | дата | confirmed_document | |
| Organization_OrganizationName | ООО «СКИФ» | организация | confirmed_document | |
| Organization_ShortName | ООО «СКИФ» | краткое | confirmed_document | |
| Organization_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Organization_INN | 1650389298 | ИНН | confirmed_document | |
| Organization_KPP | 165001001 | КПП | confirmed_document | |
| Organization_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Organization_Address_CountryCode | RU | страна | confirmed_document | |
| Organization_Address_CounryName | РОССИЯ | страна | confirmed_document | |
| Organization_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | регион | confirmed_document | |
| Organization_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | город | confirmed_document | |
| Organization_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | адрес | confirmed_document | |
| Organization_OrganizationPerson_PersonSurname | Саранов | подписант | confirmed_document | |
| Organization_OrganizationPerson_PersonName | Дмитрий | подписант | confirmed_document | |
| Organization_OrganizationPerson_PersonMiddleName | Олегович | подписант | confirmed_document | |
| Organization_OrganizationPerson_PersonPost | Директор | должность | confirmed_document | |
| EmpoweredPerson_PersonSurname | АРБУЗОВА | уполномоченный | confirmed_document | |
| EmpoweredPerson_PersonName | АНАСТАСИЯ | уполномоченный | confirmed_document | |
| EmpoweredPerson_PersonMiddleName | КОНСТАНТИНОВНА | уполномоченный | confirmed_document | |
| EmpoweredPerson_PersonPost | УПОЛНОМОЧЕННОЕ ЛИЦО | должность | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardCode | RU01001 | код документа | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardName | ПАСРФ | наименование | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardSeries | 63 09 | серия | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardNumber | 449948 | номер | confirmed_document | |
| EmpoweredPerson_Passport_IdentityCardDate | 2010-03-11 | дата выдачи | confirmed_document | |
| EmpoweredPerson_Passport_OrganizationName | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | кем выдан | confirmed_document | |

---

# II. non_formalized

## document: svh (DO-1)
- uqi_prefix: non_formalized.svh_1
- path: alta\\source\\ПриточнаяВентиляция\\1\\СВХ\\до.pdf
- file_name: до.pdf
- status: confirmed
- note: md best-effort.

| field | value | description | status | note |
|---|---|---|---|---|
| number | 0000478 | № ДО-1 | confirmed_document | |
| date | 2025-07-14 | дата | confirmed_document | |
| warehouse_license_number | 10404/141210/10092/04 | лицензия СВХ | confirmed_document | |
| warehouse_license_date | 2019-08-21 | дата | confirmed_document | |
| actual_gross_weight | 383 | факт. брутто | confirmed_document | |
| actual_places | 2 | факт. места | confirmed_document | |
| transport_reg_number | А488ОУ67 / А67261-5 | ТС | confirmed_document | |

### goods_1
| field | value | description | status | note |
|---|---|---|---|---|
| tnved | 8481309908 | код | confirmed_document | |
| places | 2 | места | confirmed_document | |
| gross_weight_kg | 383 | брутто | confirmed_document | |
| cost | 13600 | стоимость | confirmed_document | |
| currency_code | CNY | валюта | confirmed_document | |

## document: svh_additional_sheet / уведомление
- uqi_prefix: non_formalized.svh_additional_sheet_1
- path: alta\\source\\ПриточнаяВентиляция\\1\\СВХ\\ВТамПостНабережныхЧелнов.pdf
- file_name: ВТамПостНабережныхЧелнов.pdf
- status: pending
- note: md best-effort.

| field | value | description | status | note |
|---|---|---|---|---|
| number |  | № доп.листа | pending | в документе нет явного номера |
| date | 2025-07-14 | дата | confirmed_document | |
| actual_gross_weight |  | вес | pending | |
| actual_places |  | места | pending | |
| transport_reg_number |  | ТС | pending | в таблице поле №т/с пустое |
| svh_address_region | Республика Татарстан | регион СВХ | pending | документ адресован таможне, не СВХ |
| svh_address_city | Набережные Челны | город | pending | |
| svh_address_street_house | Производственный пр-д, д. 45 | улица/дом | confirmed_document | из CMR/ДО-1 |
| svh_customs_code | 10404083 | код таможни | confirmed_document | из CMR/ТД |

## document: master data
- uqi_prefix: non_formalized.master_data_1
- path: alta\\stable_source\\
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| declarant_name | ООО "СКИФ" | декларант | confirmed_document | EGRUL |
| declarant_ogrn | 1201600020390 | ОГРН | confirmed_document | EGRUL |
| declarant_inn | 1650389298 | ИНН | confirmed_document | |
| declarant_kpp | 165001001 | КПП | confirmed_document | |
| declarant_address_postal_code | 423800 | индекс | confirmed_document | EGRUL |
| declarant_address_country_code | RU | страна | confirmed_document | |
| declarant_address_country_name | РОССИЯ | страна | confirmed_document | |
| declarant_address_region | Республика Татарстан | регион | confirmed_document | EGRUL |
| declarant_address_city | Набережные Челны | город | confirmed_document | EGRUL |
| declarant_address_street | Хлебный пр-д | улица | pending | в EGRUL "ПР-Д ХЛЕБНЫЙ" без разбиения |
| declarant_address_building | 30 | дом | confirmed_document | EGRUL |
| declarant_address_room | офис 211 | офис | confirmed_document | EGRUL |
| declarant_phone | +7 937 779-26-56 | телефон | confirmed_document | контракт/инвойс/EGRUL |
| declarant_email |  | email | pending | |
| representative_last_name | АРБУЗОВА | фамилия | confirmed_document | паспорт/доверенность |
| representative_first_name | АНАСТАСИЯ | имя | confirmed_document | |
| representative_middle_name | КОНСТАНТИНОВНА | отчество | confirmed_document | |
| representative_position | УПОЛНОМОЧЕННОЕ ЛИЦО | должность | confirmed_document | доверенность |
| representative_phone | +7-927-030-70-07 | телефон | confirmed_document | доверенность |
| representative_email |  | email | pending | |
| representative_passport_code | RU01001 | код док-та | confirmed_document | доверенность |
| representative_passport_name | ПАСРФ | наименование | confirmed_document | |
| representative_passport_series | 63 09 | серия | confirmed_document | |
| representative_passport_number | 449948 | номер | confirmed_document | |
| representative_passport_date | 2010-03-11 | дата выдачи | confirmed_document | |
| representative_passport_issuer | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | кем выдан | confirmed_document | |
| representative_authority_doc_name | ДОВЕРЕННОСТЬ | док полномочий | confirmed_document | |
| representative_authority_doc_number | 1 | № | confirmed_document | |
| representative_authority_doc_date_from | 2026-02-01 | дата начала | confirmed_document | |
| representative_authority_doc_date_to | 2026-12-31 | дата окончания | confirmed_document | |
| note | Sources: stable_source (EGRUL, Passport, LetterOfAttorney, Transport contract). | примечание | confirmed_document | |

## document: transit declaration
- uqi_prefix: non_formalized.td_1
- path: alta\\source\\ПриточнаяВентиляция\\1\\документы от Любы\\ТД_12327.pdf
- file_name: ТД_12327.pdf
- status: confirmed

| field | value | description | status | note |
|---|---|---|---|---|
| number | 10719110/060725/5070039 | № ТД | confirmed_document | |
| date | 2025-07-06 | дата | confirmed_document | |
| customs_post_code | 10404083 | код поста | confirmed_document | |
| customs_post_name | Набережночелнинский | наименование | pending | в md обрезано |
| transport_reg_number | А488ОУ67 / A6726I5 | ТС | pending | в md прицеп читается как A6726I5 vs A67261-5 |

---

# III. Нерешенные вопросы

- formalized.contract_1.ContractTerms_CurrencyCode
  - question: В контракте валюта указана как RMB. Подтверди, что для целей Альты использовать ISO 4217 numeric = 156 (CNY) или другой код?

- formalized.invoice_1.CurrencyRate
  - question: Нужен курс валюты для графы 23 (если Альта требует его в импортируемом инвойсе). Какой курс/источник использовать?

- formalized.invoice_1.DeliveryTerms_DeliveryTermsNumericCode
  - question: Нужен внутренний числовой код Альты для условия EXW. Есть ли у вас справочник/пример, какой код ставить?

- formalized.invoice_1.InvoiceGoods_1.MeasureUnitQualifierName
  - question: В инвойсе количество в "Sets". Для ДТ/Альты в MeasureUnitQualifierName ставим "шт" или другое? Подтверди единицу.

- formalized.invoice_1.PlacesDescription
  - question: В документах указано "2 cl". Это cartons? Как правильно описать грузовые места (PlacesDescription)?

- formalized.payment_order_1.PaymentModeCode
  - question: Какой PaymentModeCode использовать для банковского перевода 100% TT in advance? Если есть эталон — пришли.

- formalized.payment_order_1.TransactionKind
  - question: Какой TransactionKind ставить для такого платежного документа (заявление на перевод)?

- formalized.payment_order_1.Priority
  - question: Priority (очередность) в заявлении отсутствует. Как заполнять в Альте (оставить пусто/"."/"5")?

- formalized.cmr_1.RegistrationDocument_Place
  - question: Место составления CMR 12327 не читается в md. Нужно ли оно для импорта? Если нужно — какое значение?

- formalized.cmr_1.Consignor_Guarantee_*
  - question: В CMR блок "гарант отправителя" не читается/не выделен. Он реально есть? Если да — пришли/подтверди реквизиты, иначе можно оставить пустым.

- formalized.service_invoice_1.Signature_*
  - question: В счетах Трансимпериал нет подписей/ФИО директора/бухгалтера в md. Нужно ли заполнять Signature-блок для импорта? Если да — откуда взять значения?

- non_formalized.td_1.transport_reg_number
  - question: В транзитке прицеп распознан как A6726I5, а в CMR/ДО-1 как А67261-5. Подтверди правильное написание номера прицепа.
