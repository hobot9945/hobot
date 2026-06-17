# Первичные данные

## 1. meta:
- `название кейса`: ПриточнаяВентиляция
- `путь к папке поставки`: alta\source\ПриточнаяВентиляция\01
- `direction`: ИМ
- `тип поставки`: 1 ДТ/1 товар
- `источники данных:`: md + master_data.md

## 2. formalized/master_data/non_formalized:

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: md\Инвойс 25AZC003B.md
  - `file_name`: Инвойс 25AZC003B.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | CurrencyCode | CNY | CD | валюта инвойса ISO 4217 alpha-3 | |
| 02 | DocumentCode | 04021 | CD | код вида документа для графы 44 | |
| 03 | PlacesQuantity | 2 | CD | кол-во грузовых мест по инвойсу | |
| 04 | PlacesDescription | 2 картонные коробки | CD | описание мест | |
| 05 | GrossWeightQuantity | 383 | CD | общий вес брутто по инвойсу | |
| 06 | NetWeightQuantity | 312.5 | CD | общий вес нетто по инвойсу | |
| 07 | GCost | 13600.00 | CD | системное поле Альты; дубль TotalCost | |
| 08 | TotalCost | 13600.00 | CD | итого по инвойсу | |
| 09 | DeliveryTerms_DeliveryPlace | Ningbo | CD | место поставки по Incoterms | |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий поставки | |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления alpha-2 | |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна alpha-2 | |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения alpha-2 | |
| 15 | Registration_PrDocumentName | ИНВОЙС 25AZC003B | CD | наименование документа для печати/сверок | |
| 16 | Registration_PrDocumentNumber | 25AZC003B | CD | номер инвойса | |
| 17 | Registration_PrDocumentDate | 10.04.2025 | CD | дата инвойса | |
| 18 | Contract_PrDocumentNumber | 25AZC003B | CD | № контракта-ссылки | |
| 19 | Contract_PrDocumentDate | 10.04.2025 | CD | дата контракта-ссылки | |
| 20 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | |
| 21 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | |
| 22 | Buyer_Name | LLC «SKIF» | CD | наименование покупателя | |
| 23 | Buyer_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | |
| 25 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя, текст | |
| 26 | Buyer_PostalAddress_Region | Республика Татарстан | CD | регион | |
| 27 | Buyer_PostalAddress_City | Набережные Челны | CD | город | |
| 28 | Buyer_PostalAddress_StreetHouse | Khlebny Passage, hause 30, office 211 | CD | улица/дом/офис одной строкой | |
| 29 | Seler_Name | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | продавец | |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца alpha-2 | |
| 31 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | |
| 32 | Seler_PostalAddress_Region | Zhejiang | CD | регион продавца | |
| 33 | Seler_PostalAddress_City | Ningbo | CD | город/район продавца | |
| 34 | Seler_PostalAddress_StreetHouse | D4-109, Liangzhu Culture Park, Haishu District, 315175 Ningbo | CD | улица/дом одной строкой | |
| 35 | Consignor_OrganizationName | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | грузоотправитель | |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | |
| 37 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | |
| 38 | Consignor_Address_Region | Zhejiang | CD | регион | |
| 39 | Consignor_Address_City | Ningbo | CD | город/район | |
| 40 | Consignor_Address_StreetHouse | D4-109, Liangzhu Culture Park, Haishu District, 315175 Ningbo | CD | улица/дом одной строкой | |
| 41 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | |
| 42 | Consignee_OGRN | 1201600020390 | CD | ОГРН | |
| 43 | Consignee_INN | 1650389298 | CD | ИНН | |
| 44 | Consignee_KPP | 165001001 | CD | КПП | |
| 45 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | |
| 46 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | |
| 47 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 48 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 49 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 50 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 51 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 52 | doc_code | 04021 | CD | код документа | |
| 53 | doc_name | ИНВОЙС | CD | наименование документа | |
| 54 | doc_number | 25AZC003B | CD | номер документа | |
| 55 | doc_date | 10.04.2025 | CD | дата документа | |

- _audit: 55

- `doc_status`: confirmed

#### InvoiceGoods[1] Массив: InvoiceGoods[1]
- _array_audit: 1

#### Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 8481309908 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Пластиковый воздухозаборник для вентиляции | CD | описание товара | |
| 03 | GoodsQuantity | 1000 | CD | кол-во по строке инвойса | |
| 04 | goods_supplementary_quantity | | | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | | | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | | | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 383 | CD | брутто по строке | |
| 08 | NetWeightQuantity | 312.5 | CD | нетто по строке | |
| 09 | Price | 13.6 | CD | цена за единицу | |
| 10 | TotalCost | 13600.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка/ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак/маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | КИВ-125 | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ i | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара j | |

- _item_audit: 17

### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list
  - `xml_target_root`: AltaE2PACK
  - `path`: md\PL 25AZC003B.md
  - `file_name`: PL 25AZC003B.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GrossWeightQuantity | 383 | CD | общий вес брутто по упаковочному | |
| 02 | NetWeightQuantity | 312.5 | CD | общий вес нетто по упаковочному | |
| 03 | Consignor_OrganizationName | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | грузоотправитель | |
| 04 | Consignor_ShortName | ОТСУТСТВУЕТ | CD | краткое наименование | |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | |
| 07 | Consignor_Address_Region | Zhejiang | CD | регион | |
| 08 | Consignor_Address_City | Ningbo | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | D4-109, Liangzhu Culture Park, Haishu District, 315175 Ningbo | CD | улица/дом одной строкой | |
| 10 | Consignee_OrganizationName | ООО "СКИФ" | CD | грузополучатель | |
| 11 | Consignee_ShortName | ОТСУТСТВУЕТ | CD | краткое наименование | |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | |
| 14 | Consignee_KPP | 165001001 | CD | КПП | |
| 15 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 18 | Consignee_Address_Region | Республика Татарстан | CD | регион | |
| 19 | Consignee_Address_City | Набережные Челны | CD | город | |
| 20 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 21 | DeliveryTerms_DeliveryPlace | Ningbo | CD | место поставки по Incoterms | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наименование контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | 25AZC003B | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 10.04.2025 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 25AZC003B | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 10.04.2025 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | 25AZC003B | CD | № упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 10.04.2025 | CD | дата упаковочного | |
| 33 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 34 | doc_code | 04131 | CD | код документа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | |
| 36 | doc_number | 25AZC003B | CD | номер документа | |
| 37 | doc_date | 10.04.2025 | CD | дата документа | |

- _audit: 37

#### Goods[1] Массив: Goods[1]
- _array_audit: 1

#### Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Пластиковый воздухозаборник КИВ-125 | CD | описание строки как в документе | |
| 02 | GoodsQuantity | 2 | CD | количество мест/грузовых единиц в строке | |
| 03 | GrossWeightQuantity | 383 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 312.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 24 | CD | кол-во упаковок/мест в упаковке | |

- _item_audit: 5

#### TransportMeans[2] Массив: TransportMeans[2]
- _array_audit: 2

#### Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | Number | А4880У67 | CD | регистрационный номер | |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 000 | CD | код национальности ТС | |
| 04 | MoverIndicator | true | CD | признак тягача | |

- _item_audit: 4

#### Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | Number | А672615 | CD | регистрационный номер | |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 000 | CD | код национальности ТС | |
| 04 | MoverIndicator | false | CD | признак прицепа | |

- _item_audit: 4

- `doc_status`: confirmed

### `document`: CMR
  - `uqi_prefix`: formalized.cmr
  - `xml_target_root`: AltaE3CMR
  - `path`: md\СМР.md
  - `file_name`: СМР.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | LanguageCode | RU | CD | язык документа | |
| 02 | CMR_Choice | 1 | CD | системный выбор/вариант Альты | |
| 03 | RegistrationDocument_RegID | Б/Н | CD | номер CMR | |
| 04 | RegistrationDocument_DateInf | 01.07.2025 | CD | дата CMR | |
| 05 | RegistrationDocument_Place | г. Набережные Челны, Производственный пр-д, д. 45 | CD | место составления | |
| 06 | TrakingCargo_TakingCargoDate | 01.07.2025 | CD | дата принятия груза | |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CD | страна принятия груза alpha-2 | |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза, текст | |
| 09 | DeliveryPlace_CountryCode | RU | CD | страна доставки alpha-2 | |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки, текст | |
| 11 | DeliveryTerms_DeliveryPlace | Ningbo | CD | место поставки по Incoterms | |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | условия поставки | |
| 13 | GoodsQuantity | 2 | CD | общее число грузовых мест/упаковок по CMR | |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 383 | CD | общий вес брутто по CMR | |
| 15 | CMRTransport_PrimeMoverStateSignID | А4880У67 | CD | гос. номер тягача | |
| 16 | CMRTransport_TrailerStateSignID | А672615 | CD | гос. номер прицепа | |
| 17 | Consignor_NameInf | Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd | CD | наименование отправителя | |
| 18 | Consignor_ShortName | ОТСУТСТВУЕТ | CD | краткое наименование отправителя | |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна отправителя alpha-2 | |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна отправителя, текст | |
| 21 | Consignor_Address_Region | Zhejiang | CD | регион | |
| 22 | Consignor_Address_City | Ningbo | CD | город/район | |
| 23 | Consignor_Address_StreetHouse | D4-109, Liangzhu Culture Park, Haishu District, 315175 Ningbo | CD | улица/дом одной строкой | |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CD | наименование гаранта | |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CD | краткое наименование гаранта | |
| 26 | Consignor_Guarantee_Address_CountryCode | ОТСУТСТВУЕТ | CD | страна гаранта alpha-2 | |
| 27 | Consignor_Guarantee_Address_CounryName | ОТСУТСТВУЕТ | CD | страна гаранта, текст | |
| 28 | Consignor_Guarantee_Address_Region | ОТСУТСТВУЕТ | CD | регион гаранта | |
| 29 | Consignor_Guarantee_Address_City | ОТСУТСТВУЕТ | CD | город/район гаранта | |
| 30 | Consignor_Guarantee_Address_StreetHouse | ОТСУТСТВУЕТ | CD | улица/дом гаранта | |
| 31 | Consignee_NameInf | ООО "СКИФ" | CD | наименование получателя | |
| 32 | Consignee_ShortName | ОТСУТСТВУЕТ | CD | краткое наименование получателя | |
| 33 | Consignee_OGRNID | 1201600020390 | CD | ОГРН | |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 39 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 40 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 41 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 42 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 43 | doc_code | 02015 | CD | код документа | |
| 44 | doc_name | CMR | CD | наименование документа | |
| 45 | doc_number | Б/Н | CD | номер документа | |
| 46 | doc_date | 01.07.2025 | CD | дата документа | |

- _audit: 46

#### CMRGoods[1] Массив: CMRGoods[1]
- _array_audit: 1

#### Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | |
| 02 | GoodsDescription | Приточный клапан | CD | описание груза/товара как в CMR | |
| 03 | PakingQuantity | 2 | CD | кол-во упаковок/мест | |

- _item_audit: 3

- `doc_status`: confirmed

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: md\Платежка.md
  - `file_name`: Платежка.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CD | код вида документа для графы 44 | |
| 02 | PaymentModeCode | 1 | CD | системный код способа платежа | |
| 03 | PaymentAmount | 13600.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции/код | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN PLASTIC AIR VENT. CONTRACT NO.: 25AZC003, DATE: APR 10, 2025 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Тринадцать тысяч шестьсот юаней 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 21.05.2025 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | ООО "СКИФ" | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | |
| 13 | Payer_Bank_BankName | 4070215621615000051 | CD | банк плательщика / реквизиты | |
| 14 | Payee_OrganizationName | NINGBO ZENTEC AIR CONDITIONING AND REFRIGERATION CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX | CD | банк получателя / реквизиты | |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | |
| 17 | PersonName | Дмитрий Олегович | CD | имя подписанта | |
| 18 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 19 | doc_code | 04023 | CD | код документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 1 | CD | номер документа | |
| 22 | doc_date | 21.05.2025 | CD | дата документа | |

- _audit: 22

- `doc_status`: confirmed

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice
  - `xml_target_root`: AltaServiceInvoice
  - `path`: md\Счет_№25-12327-k_от_22-05-2025.md
  - `file_name`: Счет_№25-12327-k_от_22-05-2025.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentSign | 1 | CD | системный признак документа Альты | |
| 02 | TotalServiceCost | 1200.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого ISO 4217 alpha-3 | |
| 04 | ServiceProvider_Name | Общество с ограниченной ответственностью «Трансмипериал» | CD | исполнитель услуг/перевозчик | |
| 05 | BankName | АО "Райффайзенбанк" БИК 044525700 Сч. № 40702810400000233463 | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги/перевозку | |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги/перевозку | |
| 08 | PrDocumentNumber | 1 | CD | связанный документ/заказ: номер | |
| 09 | PrDocumentDate | 21.05.2025 | CD | связанный документ/заказ: дата | |
| 10 | Registration_PrDocumentName | Счет на оплату №25-12327-к от 22.05.2025 г. | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | 25-12327-к | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 22.05.2025 | CD | дата счета | |
| 13 | Consignor_OrganizationName | ООО СКиФ | CD | грузоотправитель | |
| 14 | PostalCode | | CD | почтовый индекс | |
| 15 | CountryCode | RU | CD | страна alpha-2 | |
| 16 | CounryName | Россия | CD | страна, текст | |
| 17 | Region | Республика Татарстан | CD | регион | |
| 18 | Town | Набережные Челны | CD | город/район | |
| 19 | StreetHouse | проезд Хлебный, д.30, кв/оф. 211 | CD | улица/дом одной строкой | |
| 20 | Consignee_OrganizationName | ООО СКиФ | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CD | ОГРН | |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | |
| 24 | PostalCode | 423800 | CD | почтовый индекс | |
| 25 | CountryCode | RU | CD | страна alpha-2 | |
| 26 | CounryName | Россия | CD | страна, текст | |
| 27 | Region | Республика Татарстан | CD | регион | |
| 28 | Town | Набережные Челны | CD | город | |
| 29 | StreetHouse | проезд Хлебный | CD | улица | |
| 30 | House | д.30 | CD | дом | |
| 31 | Room | оф. 211 | CD | офис/кв | |
| 32 | Signature_Choice | 2 | CD | вариант подписи | |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | |
| 34 | IndividualEntrepreneur_PersonName | | CD | первый инициал/имя ИП | |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | второй инициал/отчество ИП | |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | | CD | фамилия руководителя | |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | | CD | первый инициал/имя руководителя | |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | | CD | второй инициал/отчество руководителя | |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | | CD | фамилия бухгалтера | |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | | CD | первый инициал/имя бухгалтера | |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | | CD | второй инициал/отчество бухгалтера | |
| 42 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 43 | doc_code | 04031 | CD | код документа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | |
| 45 | doc_number | 25-12327-к | CD | номер документа | |
| 46 | doc_date | 22.05.2025 | CD | дата документа | |
| 47 | transport_to_border | 624.00 | CD | стоимость маршрута до границы | |
| 48 | transport_currency | USD | CD | валюта стоимости | |

- _audit: 48

#### ServiceDescription[2] Массив: ServiceDescription[2]
- _array_audit: 2

#### Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по заявке № 1 от 21.05.2025 по маршруту: China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | многострочное описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки ISO alpha-3 | |
| 03 | ServiceName | China, Ningbo - граница РФ | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 624.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |

- _item_audit: 7

#### Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, г. Набережные Челны, Производственный пр-д | CD | многострочное описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки ISO alpha-3 | |
| 03 | ServiceName | граница РФ - Набережные Челны | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 576.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |

- _item_audit: 7

- `doc_status`: confirmed

### `document`: Tech Description
  - `uqi_prefix`: formalized.tech_description
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\техничка КИВ 125.md
  - `file_name`: техничка КИВ 125.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 05999 | CD | код вида документа для графы 44 | |
| 02 | DocumentHead_DocumentName | Технические характеристики КИВ-125 | CD | наименование техописания | |
| 03 | DocumentHead_DocumentDate | 10.04.2025 | CD | дата техописания | |
| 04 | DocumentHead_DocumentNumber | 1СК1004 | CD | номер техописания | |
| 05 | TextPara | link:md\техничка КИВ 125.md | CD | ссылка на файл-источник | |
| 06 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 07 | doc_code | 05999 | CD | код документа | |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | |
| 09 | doc_number | 1СК1004 | CD | номер документа | |
| 10 | doc_date | 10.04.2025 | CD | дата документа | |

- _audit: 10

- `doc_status`: confirmed

## 3. master_data

### `document`: Contract
  - `uqi_prefix`: master_data.contract
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 03011 | CD | код документа | |
| 03 | doc_name | КОНТРАКТ | CD | наименование документа | |
| 04 | doc_number | 25AZC003B | CD | номер документа | |
| 05 | doc_date | 10.04.2025 | CD | дата документа | |

- _audit: 5

- `doc_status`: confirmed

### `document`: Supplementary Contract
  - `uqi_prefix`: master_data.supplementary_contract_1
  - `path`: master_data.md
  - `file_name`: master_data.md
  - `note`: документ отсутствует в master_data.md и первичке

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 03012 | CD | код документа | |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | наименование документа | |
| 04 | doc_number | ОТСУТСТВУЕТ | CD | номер документа | |
| 05 | doc_date | ОТСУТСТВУЕТ | CD | дата документа | |

- _audit: 5

- `doc_status`: confirmed

### `document`: UNK
  - `uqi_prefix`: master_data.unk
  - `path`: master_data.md
  - `file_name`: master_data.md
  - `note`: документ отсутствует в master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 03031 | CD | код документа | |
| 03 | doc_name | УНК | CD | наименование документа | |
| 04 | doc_number | ОТСУТСТВУЕТ | CD | номер документа | |
| 05 | doc_date | ОТСУТСТВУЕТ | CD | дата документа | |

- _audit: 5

- `doc_status`: confirmed

### `document`: EGRUL
  - `uqi_prefix`: master_data.egrul
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование организации | |
| 02 | ShortName | ООО "СКИФ" | CD | краткое наименование | |
| 03 | OGRN | 1201600020390 | CD | ОГРН | |
| 04 | INN | 1650389298 | CD | ИНН | |
| 05 | KPP | 165001001 | CD | КПП | |
| 06 | Address_PostalCode | 423800 | CD | индекс | |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | |
| 08 | Address_CounryName | РОССИЯ | CD | страна, текст | |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | |
| 14 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 15 | doc_code | 04011 | CD | код документа | |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | |
| 18 | doc_date | 14.07.2025 | CD | дата документа | |

- _audit: 18

- `doc_status`: confirmed

### `document`: Personal Passport
  - `uqi_prefix`: master_data.passport
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | |
| 04 | CardSeries | 63 09 | CD | серия | |
| 05 | CardNumber | 449948 | CD | номер | |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | |
| 08 | Phone | +7 927-222-0500 | CD | телефон | |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | |
| 10 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 11 | doc_code | 11001 | CD | код документа | |
| 12 | doc_name | ПАСПОРТ | CD | наименование документа | |
| 13 | doc_number | 63 09 449948 | CD | номер документа | |
| 14 | doc_date | 11.03.2010 | CD | дата документа | |

- _audit: 14

- `doc_status`: confirmed

### `document`: Letter of Attorney
  - `uqi_prefix`: master_data.letter_of_attorney
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentNumber | 1 | CD | номер доверенности | |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | |
| 03 | EndDate | 31.12.2026 | CD | действительна до | |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | |
| 05 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 06 | doc_code | 11004 | CD | код документа | |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | |
| 08 | doc_number | 1 | CD | номер документа | |
| 09 | doc_date | 01.02.2026 | CD | дата документа | |

- _audit: 9

- `doc_status`: confirmed

### `document`: Transport Contract
  - `uqi_prefix`: master_data.transport_contract
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 04033 | CD | код документа | |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование документа | |
| 04 | doc_number | КООО/26651/М | CD | номер документа | |
| 05 | doc_date | 13.05.2025 | CD | дата документа | |

- _audit: 5

- `doc_status`: confirmed

### `document`: Exemption Letter
  - `uqi_prefix`: master_data.exemption_letter
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 09023 | CD | код документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | |
| 05 | doc_date | 20.08.2025 | CD | дата документа | |

- _audit: 5

- `doc_status`: confirmed

### `document`: Exemption Letter (source)
  - `uqi_prefix`: master_data.exemption_letter_source
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 02 | doc_code | 09999 | CD | код документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | |
| 05 | doc_date | 20.08.2025 | CD | дата документа | |

- _audit: 5

- `doc_status`: confirmed

## 4. non_formalized

### `document`: Goods Description
  - `uqi_prefix`: non_formalized.goods_description_1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | false | CD | служебный признак включения в графу 44 | |

- _audit: 1

#### goods_[1] Массив: goods_[1]
- _array_audit: 1

#### Элемент массива: goods_[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tn_ved | 8481309908 | CD | ТН ВЭД | |
| 02 | description | Клапан обратный (невозвратный) прочий: приточный клапан канального типа КИВ-125, пластиковый воздухозаборник для вентиляции, предназначен для подачи свежего воздуха в помещение с регулируемой подачей воздуха, фильтрацией и тепло-шумоизоляцией. Материалы: пластиковый оголовок ABS, ППУ, ретикулированный поролон. Диаметр воздуховода 125 мм. | CD | описание товара | |

- _item_audit: 2

- `doc_status`: confirmed

### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.td
  - `path`: md\ТД_12327.md
  - `file_name`: ТД_12327.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | customs_post_code | 10719110 | CD | код таможенного органа | |
| 02 | customs_post_name | ОТСУТСТВУЕТ | CD | наименование таможенного органа | |
| 03 | transport_reg_number | А4880У67, А672615 | CD | ТС по ТД | |
| 04 | doc_gr44 | true | CD | служебный признак включения в графу 44 | |
| 05 | doc_code | 09013 | CD | код документа | |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | наименование документа | |
| 07 | doc_number | 10719110/060725/5070039 | CD | номер документа | |
| 08 | doc_date | 06.07.2025 | CD | дата документа | |

- _audit: 8

- `doc_status`: confirmed

### `document`: Storage Report
  - `uqi_prefix`: non_formalized.svh
  - `path`: md\до.md
  - `file_name`: до.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | warehouse_license_number | 10404/141210/10092/04 | CD | номер лицензии/свидетельства СВХ | |
| 02 | warehouse_license_date | 21.08.2019 | CD | дата лицензии/свидетельства СВХ | |
| 03 | actual_gross_weight | 383 | CD | фактический вес по весам | |
| 04 | actual_places | 2 | CD | фактическое количество мест | |
| 05 | transport_reg_number | А488ОУ67 (прицеп: А672615) | CD | номер ТС при въезде/по отчету СВХ | |
| 06 | doc_gr44 | false | CD | служебный признак включения в графу 44 | |

- _audit: 6

#### goods_[1] Массив: goods_[1]
- _array_audit: 1

#### Элемент массива: goods_[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tnved | 8481309908 | CD | код товара | |
| 02 | places | 2 | CD | кол-во грузовых мест по строке | |
| 03 | gross_weight_kg | 383 | CD | вес брутто по строке | |
| 04 | cost | 13600 | CD | стоимость по строке | |
| 05 | currency_code | CNY | CD | буквенный код валюты | |

- _item_audit: 5

- `doc_status`: confirmed

### `document`: Certificate of Origin
  - `uqi_prefix`: non_formalized.certificate_of_origin
  - `note`: документ отсутствует в первичке

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | number | ОТСУТСТВУЕТ | CD | номер сертификата | |
| 02 | date | ОТСУТСТВУЕТ | CD | дата сертификата | |
| 03 | doc_gr44 | false | CD | служебный признак включения в графу 44 | |

- _audit: 3

- `doc_status`: confirmed

### `document`: Conformity Document
  - `uqi_prefix`: non_formalized.conformity_document
  - `note`: документ отсутствует в первичке

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | number | ОТСУТСТВУЕТ | CD | номер декларации/сертификата | |
| 02 | date_start | ОТСУТСТВУЕТ | CD | дата начала действия | |
| 03 | date_end | ОТСУТСТВУЕТ | CD | дата окончания действия | |
| 04 | doc_gr44 | false | CD | служебный признак включения в графу 44 | |

- _audit: 4

- `doc_status`: confirmed

## 4. Итоги по файлу

`total_unreliable_fields`: 0
`primary_status`: confirmed

## 5. Нерешенные вопросы (Issues)
**Для полей:**
- `formalized.invoice_1.PlacesQuantity`
## 5. Нерешенные вопросы (Issues)

(нет)

## 6. `unreliable_fields`:

(нет)
(нет)
