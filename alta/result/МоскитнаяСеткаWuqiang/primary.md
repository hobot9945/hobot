# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСеткаWuqiang
- `путь к папке поставки`: alta\source\МоскитнаяСеткаWuqiang\01
- `direction`: ИМ
- `тип поставки`: 1 ДТ/14 товаров
- `источники данных:` md + operator_provided_data.md + master_data.md

## 2. formalized/master_data/non_formalized:

### `document`: Contract
- `uqi_prefix`: master_data.contract
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 03011 | CD | константа | |
| 03 | doc_name | КОНТРАКТ | CD | константа | |
| 04 | doc_number | 26HL-1103 | CD | номер контракта | master_data.md |
| 05 | doc_date | 31.03.2026 | CD | дата контракта | master_data.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: UNK
- `uqi_prefix`: master_data.unk
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\operator\operator_provided_data.md
- `file_name`: operator_provided_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CO | константа | |
| 02 | doc_code | 03031 | CO | константа | |
| 03 | doc_name | УНК | CO | константа | |
| 04 | doc_number | ОТСУТСТВУЕТ | CO | номер УНК | operator_provided_data.md |
| 05 | doc_date | ОТСУТСТВУЕТ | CO | дата УНК | operator_provided_data.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: EGRUL
- `uqi_prefix`: master_data.egrul
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование декларанта | master_data.md |
| 02 | ShortName | ООО "СКИФ" | CD | краткое наименование | master_data.md |
| 03 | OGRN | 1201600020390 | CD | ОГРН декларанта | master_data.md |
| 04 | INN | 1650389298 | CD | ИНН декларанта | master_data.md |
| 05 | KPP | 165001001 | CD | КПП декларанта | master_data.md |
| 06 | Address_PostalCode | 423800 | CD | индекс декларанта | master_data.md |
| 07 | Address_CountryCode | RU | CD | код страны декларанта | master_data.md |
| 08 | Address_CounryName | РОССИЯ | CD | страна декларанта | master_data.md |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион декларанта | master_data.md |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город декларанта | master_data.md |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом декларанта | master_data.md |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | master_data.md |
| 13 | Email | PROM_TAT@MAIL.RU | CD | эл. почта | master_data.md |
| 14 | doc_gr44 | true | CD | константа | |
| 15 | doc_code | 04011 | CD | константа | |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | константа | |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер выписки | master_data.md |
| 18 | doc_date | 14.07.2025 | CD | дата выписки | master_data.md |

- _audit: 18
- `doc_status`: confirmed

### `document`: Personal Passport
- `uqi_prefix`: master_data.passport
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия представителя | master_data.md |
| 02 | PersonName | АНАСТАСИЯ | CD | имя представителя | master_data.md |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество представителя | master_data.md |
| 04 | CardSeries | 63 09 | CD | серия паспорта | master_data.md |
| 05 | CardNumber | 449948 | CD | номер паспорта | master_data.md |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи паспорта | master_data.md |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан паспорт | master_data.md |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_data.md |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | эл. почта | master_data.md |
| 10 | doc_gr44 | true | CD | константа | |
| 11 | doc_code | 11001 | CD | константа | |
| 12 | doc_name | ПАСПОРТ | CD | константа | |
| 13 | doc_number | 63 09 449948 | CD | номер паспорта для 44гр | master_data.md |
| 14 | doc_date | 11.03.2010 | CD | дата выдачи паспорта для 44гр | master_data.md |

- _audit: 14
- `doc_status`: confirmed

### `document`: Letter of Attorney
- `uqi_prefix`: master_data.letter_of_attorney
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data.md |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_data.md |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | должность доверенного лица | master_data.md |
| 05 | doc_gr44 | true | CD | константа | |
| 06 | doc_code | 11004 | CD | константа | |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | константа | |
| 08 | doc_number | 1 | CD | номер доверенности для 44гр | master_data.md |
| 09 | doc_date | 01.02.2026 | CD | дата доверенности для 44гр | master_data.md |

- _audit: 9
- `doc_status`: confirmed

### `document`: Transport Contract
- `uqi_prefix`: master_data.transport_contract
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 04033 | CD | константа | |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | константа | |
| 04 | doc_number | КООО/26651/М | CD | номер договора перевозки | master_data.md |
| 05 | doc_date | 13.05.2025 | CD | дата договора перевозки | master_data.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter
- `uqi_prefix`: master_data.exemption_letter
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 09023 | CD | константа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | константа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер отказного письма | master_data.md |
| 05 | doc_date | 20.08.2025 | CD | дата отказного письма | master_data.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter (source)
- `uqi_prefix`: master_data.exemption_letter_source
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 09999 | CD | константа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | константа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер отказного письма (ист.) | master_data.md |
| 05 | doc_date | 20.08.2025 | CD | дата отказного письма (ист.) | master_data.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Invoice 1
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\CL 26HL-1103-A.md
- `file_name`: CL 26HL-1103-A.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | CNY | CD | валюта инвойса | CL 26HL-1103-A.md |
| 02 | DocumentCode | 04021 | CD | код вида документа | константа |
| 03 | PlacesQuantity | 5 | CD | кол-во грузовых мест | CL 26HL-1103-A.md |
| 04 | PlacesDescription | BG | CD | описание мест | CL 26HL-1103-A.md |
| 05 | GrossWeightQuantity | 305 | CD | общий вес брутто | copied_from:formalized.packing_list.GrossWeightQuantity (PL.md) |
| 06 | NetWeightQuantity | 240 | CD | общий вес нетто | copied_from:formalized.packing_list.NetWeightQuantity (PL.md) |
| 07 | GCost | 15360.00 | CD | системное поле стоимости | CL 26HL-1103-A.md |
| 08 | TotalCost | 15360.00 | CD | итого по инвойсу | CL 26HL-1103-A.md |
| 09 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | CL 26HL-1103-A.md |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | код условий поставки | EXW условий поставки |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | CL 26HL-1103-A.md |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | CL 26HL-1103-A.md |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | CL 26HL-1103-A.md |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | CL 26HL-1103-A.md |
| 15 | Registration_PrDocumentName | Коммерческий инвойс | CD | наименование документа | CL 26HL-1103-A.md |
| 16 | Registration_PrDocumentNumber | 26HL-1103-A | CD | номер инвойса | CL 26HL-1103-A.md |
| 17 | Registration_PrDocumentDate | 31.03.2026 | CD | дата инвойса | CL 26HL-1103-A.md |
| 18 | Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта-ссылки | CL 26HL-1103-A.md |
| 19 | Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта-ссылки | CL 26HL-1103-A.md |
| 20 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | master_data.md |
| 21 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | master_data.md |
| 22 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование покупателя | master_data.md |
| 23 | Buyer_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | master_data.md |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | master_data.md |
| 25 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя текст | master_data.md |
| 26 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион покупателя | master_data.md |
| 27 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город покупателя | master_data.md |
| 28 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом покупателя | master_data.md |
| 29 | Seler_Name | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | продавец | master_data.md |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | CL 26HL-1103-A.md |
| 31 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца текст | CL 26HL-1103-A.md |
| 32 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | CL 26HL-1103-A.md |
| 33 | Seler_PostalAddress_City | Wuqiang, Hengshui | CD | город продавца | CL 26HL-1103-A.md |
| 34 | Seler_PostalAddress_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом продавца | CL 26HL-1103-A.md |
| 35 | Consignor_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | грузоотправитель | copied_from:formalized.invoice_1.Seler_Name |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | copied_from:formalized.invoice_1.Seler_PostalAddress_CountryCode |
| 37 | Consignor_Address_CounryName | КИТАЙ | CD | страна ГО текст | copied_from:formalized.invoice_1.Seler_PostalAddress_CounryName |
| 38 | Consignor_Address_Region | Hebei | CD | регион грузоотправителя | copied_from:formalized.invoice_1.Seler_PostalAddress_Region |
| 39 | Consignor_Address_City | Wuqiang, Hengshui | CD | город грузоотправителя | copied_from:formalized.invoice_1.Seler_PostalAddress_City |
| 40 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом грузоотправителя | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse |
| 41 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 42 | Consignee_OGRN | 1201600020390 | CD | ОГРН грузополучателя | master_data.md |
| 43 | Consignee_INN | 1650389298 | CD | ИНН грузополучателя | master_data.md |
| 44 | Consignee_KPP | 165001001 | CD | КПП грузополучателя | master_data.md |
| 45 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс получателя | master_data.md |
| 46 | Consignee_Address_CountryCode | RU | CD | страна получателя | master_data.md |
| 47 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя текст | master_data.md |
| 48 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data.md |
| 49 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data.md |
| 50 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом получателя | master_data.md |
| 51 | doc_gr44 | true | CD | константа | |
| 52 | doc_code | 04021 | CD | константа | |
| 53 | doc_name | ИНВОЙС | CD | константа | |
| 54 | doc_number | 26HL-1103-A | CD | номер для 44гр | CL 26HL-1103-A.md |
| 55 | doc_date | 31.03.2026 | CD | дата для 44гр | CL 26HL-1103-A.md |

- _audit: 55
- `doc_status`: confirmed

#### 3.1 Товарные позиции Массив: InvoiceGoods[1]
- _array_audit: 1

#### 3.1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103-A.md |
| 02 | GoodsDescription | Антипыльца 100 г черная 1,6 м*30 м | CD | описание товара на русском | CL 26HL-1103-A.md |
| 03 | GoodsQuantity | 50 | CD | кол-во в рулонах | CL 26HL-1103-A.md |
| 04 | goods_supplementary_quantity | 2400 | CD | кол-во в доп. ед. | CL 26HL-1103-A.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103-A.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103-A.md |
| 07 | GrossWeightQuantity | 305 | CD | брутто строки | copied_from:formalized.packing_list.Goods[14].GrossWeightQuantity |
| 08 | NetWeightQuantity | 240 | CD | нетто строки | copied_from:formalized.packing_list.Goods[14].NetWeightQuantity |
| 09 | Price | 307.2 | CD | цена за рулон | CL 26HL-1103-A.md |
| 10 | TotalCost | 15360.00 | CD | стоимость по строке | CL 26HL-1103-A.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103-A.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103-A.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103-A.md |
| 15 | AdditionalGoodsDescription_GoodsModel | АНТИПЫЛЬЦА 1.6*30 | CD | модель товара | CL 26HL-1103-A.md |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 1 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

### `document`: Invoice 2
- `uqi_prefix`: formalized.invoice_2
- `xml_target_root`: AltaE2I
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\CL 26HL-1103.md
- `file_name`: CL 26HL-1103.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | CNY | CD | валюта инвойса | CL 26HL-1103.md |
| 02 | DocumentCode | 04021 | CD | код вида документа | константа |
| 03 | PlacesQuantity | 201 | CD | кол-во грузовых мест | CL 26HL-1103.md |
| 04 | PlacesDescription | BG | CD | описание мест | CL 26HL-1103.md |
| 05 | GrossWeightQuantity | 3155 | CD | общий вес брутто | copied_from:formalized.packing_list.GrossWeightQuantity (PL.md) |
| 06 | NetWeightQuantity | 2960 | CD | общий вес нетто | copied_from:formalized.packing_list.NetWeightQuantity (PL.md) |
| 07 | GCost | 72607.44 | CD | системное поле стоимости | CL 26HL-1103.md |
| 08 | TotalCost | 72607.44 | CD | итого по инвойсу | CL 26HL-1103.md |
| 09 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | CL 26HL-1103.md |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | код условий поставки | EXW условий поставки |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | CL 26HL-1103.md |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | CL 26HL-1103.md |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | CL 26HL-1103.md |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | CL 26HL-1103.md |
| 15 | Registration_PrDocumentName | Коммерческий инвойс | CD | наименование документа | CL 26HL-1103.md |
| 16 | Registration_PrDocumentNumber | 26HL-1103 | CD | номер инвойса | CL 26HL-1103.md |
| 17 | Registration_PrDocumentDate | 31.03.2026 | CD | дата инвойса | CL 26HL-1103.md |
| 18 | Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта-ссылки | CL 26HL-1103.md |
| 19 | Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта-ссылки | CL 26HL-1103.md |
| 20 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | master_data.md |
| 21 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | master_data.md |
| 22 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование покупателя | master_data.md |
| 23 | Buyer_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | master_data.md |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | master_data.md |
| 25 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя текст | master_data.md |
| 26 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион покупателя | master_data.md |
| 27 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город покупателя | master_data.md |
| 28 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом покупателя | master_data.md |
| 29 | Seler_Name | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | продавец | master_data.md |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | CL 26HL-1103.md |
| 31 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца текст | CL 26HL-1103.md |
| 32 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | CL 26HL-1103.md |
| 33 | Seler_PostalAddress_City | Wuqiang, Hengshui | CD | город продавца | CL 26HL-1103.md |
| 34 | Seler_PostalAddress_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом продавца | CL 26HL-1103.md |
| 35 | Consignor_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | грузоотправитель | copied_from:formalized.invoice_2.Seler_Name |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | copied_from:formalized.invoice_2.Seler_PostalAddress_CountryCode |
| 37 | Consignor_Address_CounryName | КИТАЙ | CD | страна ГО текст | copied_from:formalized.invoice_2.Seler_PostalAddress_CounryName |
| 38 | Consignor_Address_Region | Hebei | CD | регион грузоотправителя | copied_from:formalized.invoice_2.Seler_PostalAddress_Region |
| 39 | Consignor_Address_City | Wuqiang, Hengshui | CD | город грузоотправителя | copied_from:formalized.invoice_2.Seler_PostalAddress_City |
| 40 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом грузоотправителя | copied_from:formalized.invoice_2.Seler_PostalAddress_StreetHouse |
| 41 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 42 | Consignee_OGRN | 1201600020390 | CD | ОГРН грузополучателя | master_data.md |
| 43 | Consignee_INN | 1650389298 | CD | ИНН грузополучателя | master_data.md |
| 44 | Consignee_KPP | 165001001 | CD | КПП грузополучателя | master_data.md |
| 45 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс получателя | master_data.md |
| 46 | Consignee_Address_CountryCode | RU | CD | страна получателя | master_data.md |
| 47 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя текст | master_data.md |
| 48 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data.md |
| 49 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data.md |
| 50 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом получателя | master_data.md |
| 51 | doc_gr44 | true | CD | константа | |
| 52 | doc_code | 04021 | CD | константа | |
| 53 | doc_name | ИНВОЙС | CD | константа | |
| 54 | doc_number | 26HL-1103 | CD | номер для 44гр | CL 26HL-1103.md |
| 55 | doc_date | 31.03.2026 | CD | дата для 44гр | CL 26HL-1103.md |

- _audit: 55
- `doc_status`: confirmed

#### 3.1 Товарные позиции Массив: InvoiceGoods[13]
- _array_audit: 13

#### 3.1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм. 1.4m x 30m. Черная | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 10 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 420 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 42 | CD | брутто строки | copied_from:formalized.packing_list.Goods[1].GrossWeightQuantity |
| 08 | NetWeightQuantity | 39 | CD | нетто строки | copied_from:formalized.packing_list.Goods[1].NetWeightQuantity |
| 09 | Price | 96.6 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 966.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | ПЛИССЕ 1.4*30 | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 1 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм 1.6m x 30m. Черная | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 10 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 480 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 36.4 | CD | брутто строки | copied_from:formalized.packing_list.Goods[2].GrossWeightQuantity |
| 08 | NetWeightQuantity | 34 | CD | нетто строки | copied_from:formalized.packing_list.Goods[2].NetWeightQuantity |
| 09 | Price | 110.4 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 1104.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | ПЛИССЕ 1.6*30 | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 2 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Европейская плиссированная сетка16 мм 1.4m x 30m . Черная | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 10 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 420 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 39 | CD | брутто строки | copied_from:formalized.packing_list.Goods[3].GrossWeightQuantity |
| 08 | NetWeightQuantity | 36 | CD | нетто строки | copied_from:formalized.packing_list.Goods[3].NetWeightQuantity |
| 09 | Price | 98.7 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 987.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | ЕВРОПЕЙСКАЯ ПЛИССЕ 1.4*30 | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 3 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Европейская плиссированная сетка16 мм 1.6m x 30m. Черная | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 10 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 480 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 34.4 | CD | брутто строки | copied_from:formalized.packing_list.Goods[4].GrossWeightQuantity |
| 08 | NetWeightQuantity | 31.5 | CD | нетто строки | copied_from:formalized.packing_list.Goods[4].NetWeightQuantity |
| 09 | Price | 112.8 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 1128.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | ЕВРОПЕЙСКАЯ ПЛИССЕ 1.6*30 | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 4 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Сетка от кошек 220 гр "Антикот" 1.4m x 30m. Серая | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 30 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 1260 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 303 | CD | брутто строки | copied_from:formalized.packing_list.Goods[5].GrossWeightQuantity |
| 08 | NetWeightQuantity | 277 | CD | нетто строки | copied_from:formalized.packing_list.Goods[5].NetWeightQuantity |
| 09 | Price | 197.4 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 5922.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | АНТИКОТ 220Г 1.4*30 | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 1 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Сетка от кошек 320 гр "Антикот" 1.6m x 30m. Серая | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 50 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 2400 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 800 | CD | брутто строки | copied_from:formalized.packing_list.Goods[6].GrossWeightQuantity |
| 08 | NetWeightQuantity | 768 | CD | нетто строки | copied_from:formalized.packing_list.Goods[6].NetWeightQuantity |
| 09 | Price | 278.4 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 13920.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | АНТИКОТ 320Г 1.6*30 СЕРАЯ | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 2 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Сетка от кошек 320 гр "Антикот" 1.4 m x 30m. Серая | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 50 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 2100 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 710 | CD | брутто строки | copied_from:formalized.packing_list.Goods[7].GrossWeightQuantity |
| 08 | NetWeightQuantity | 672 | CD | нетто строки | copied_from:formalized.packing_list.Goods[7].NetWeightQuantity |
| 09 | Price | 243.6 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 12180.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | АНТИКОТ 320Г 1.4*30 СЕРАЯ | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 3 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Сетка от кошек 320 гр "Антикот" 1.6m x 30m. Черная | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 30 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 1440 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 480 | CD | брутто строки | copied_from:formalized.packing_list.Goods[8].GrossWeightQuantity |
| 08 | NetWeightQuantity | 461 | CD | нетто строки | copied_from:formalized.packing_list.Goods[8].NetWeightQuantity |
| 09 | Price | 278.4 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 8352.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | АНТИКОТ 320Г 1.6*30 ЧЕРНАЯ | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 4 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Сетка от кошек 320 гр "Антикот" 1.4m x 30m. Черная | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 30 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 1260 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 426 | CD | брутто строки | copied_from:formalized.packing_list.Goods[9].GrossWeightQuantity |
| 08 | NetWeightQuantity | 403 | CD | нетто строки | copied_from:formalized.packing_list.Goods[9].NetWeightQuantity |
| 09 | Price | 243.6 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 7308.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | АНТИКОТ 320Г 1.4*30 ЧЕРНАЯ | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 5 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*50 М2 | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 10 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 800 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 45.46 | CD | брутто строки | copied_from:formalized.packing_list.Goods[10].GrossWeightQuantity |
| 08 | NetWeightQuantity | 33 | CD | нетто строки | copied_from:formalized.packing_list.Goods[10].NetWeightQuantity |
| 09 | Price | 753.6 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 7536.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | АНТИПЫЛЬ 30Г 1.6*50 | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 6 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*100 М2 | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 5 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 800 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 22.74 | CD | брутто строки | copied_from:formalized.packing_list.Goods[11].GrossWeightQuantity |
| 08 | NetWeightQuantity | 16.5 | CD | нетто строки | copied_from:formalized.packing_list.Goods[11].NetWeightQuantity |
| 09 | Price | 1507.2 | CD | цена за рулон | CL 26HL-1103.md |
| 10 | TotalCost | 7536.00 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | АНТИПЫЛЬ 30Г 1.6*100 | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 7 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7314490000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | сетка 18, размер ячейки 0,18 мм материал SS304. Размер рулона 1,6*30 М2 цвет оригинальный | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 3 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 144 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 51 | CD | брутто строки | copied_from:formalized.packing_list.Goods[12].GrossWeightQuantity |
| 08 | NetWeightQuantity | 43 | CD | нетто строки | copied_from:formalized.packing_list.Goods[12].NetWeightQuantity |
| 09 | Price | 357.95 | CD | расчетная цена за рулон (1073.86/3) | расчет |
| 10 | TotalCost | 1073.86 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | НЕРЖАВЕЙКА 18 ЯЧЕЙКА 0.18ММ 1.6*30 | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 1 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

#### 3.1 Элемент массива: InvoiceGoods[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7314490000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | GoodsDescription | сетка17 мм материал SS304. Размер рулона 1,4*30 М2 Черный | CD | описание товара на русском | CL 26HL-1103.md |
| 03 | GoodsQuantity | 10 | CD | кол-во в рулонах | CL 26HL-1103.md |
| 04 | goods_supplementary_quantity | 420 | CD | кол-во в доп. ед. | CL 26HL-1103.md |
| 05 | goods_supplementary_uom_name | м2 | CD | доп. единица | CL 26HL-1103.md |
| 06 | MeasureUnitQualifierName | м2 | CD | единица доп. кол-ва | CL 26HL-1103.md |
| 07 | GrossWeightQuantity | 165 | CD | брутто строки | copied_from:formalized.packing_list.Goods[13].GrossWeightQuantity |
| 08 | NetWeightQuantity | 146 | CD | нетто строки | copied_from:formalized.packing_list.Goods[13].NetWeightQuantity |
| 09 | Price | 459.46 | CD | расчетная цена за рулон (4594.58/10) | расчет |
| 10 | TotalCost | 4594.58 | CD | стоимость по строке | CL 26HL-1103.md |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происх. | cb:country |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO. LTD | CD | производитель | CL 26HL-1103.md |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | торговая марка | CL 26HL-1103.md |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | CL 26HL-1103.md |
| 15 | AdditionalGoodsDescription_GoodsModel | НЕРЖАВЕЙКА 20 ЯЧЕЙКА 0.17ММ 1.4*30 | CD | модель товара | CL 26HL-1103.md |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | расчет |
| 17 | dt_tovg_index | 2 | CD | индекс позиции ДТ | расчет |

- _item_audit: 17

### `document`: Packing List
- `uqi_prefix`: formalized.packing_list
- `xml_target_root`: AltaE2PACK
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\PL.md
- `file_name`: PL.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3460 | CD | общий вес брутто | PL.md |
| 02 | NetWeightQuantity | 3200 | CD | общий вес нетто | PL.md |
| 03 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | PL.md |
| 04 | Consignor_ShortName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | краткое наименование | PL.md |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | PL.md |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна ГО текст | PL.md |
| 07 | Consignor_Address_Region | Hebei | CD | регион грузоотправителя | PL.md |
| 08 | Consignor_Address_City | Wuqiang | CD | город грузоотправителя | PL.md |
| 09 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом грузоотправителя | PL.md |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 11 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | master_data.md |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН грузополучателя | master_data.md |
| 13 | Consignee_INN | 1650389298 | CD | ИНН грузополучателя | master_data.md |
| 14 | Consignee_KPP | 165001001 | CD | КПП грузополучателя | master_data.md |
| 15 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс получателя | master_data.md |
| 16 | Consignee_Address_CountryCode | RU | CD | страна получателя | master_data.md |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя текст | master_data.md |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | master_data.md |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | master_data.md |
| 20 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом получателя | master_data.md |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | PL.md |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | код условий поставки | EXW условий поставки |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | PL.md |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наименование контракта | PL.md |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | 26HL-1103 | CD | номер контракта | PL.md |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта | PL.md |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | PL.md |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 26HL-1103 | CD | номер инвойса | PL.md |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 31.03.2026 | CD | дата инвойса | PL.md |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование упаковочного | PL.md |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | Б/Н | CO | номер упаковочного | operator_provided_data.md |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 31.03.2026 | CD | дата упаковочного | PL.md |
| 33 | doc_gr44 | true | CD | константа | |
| 34 | doc_code | 04131 | CD | константа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | константа | |
| 36 | doc_number | Б/Н | CO | номер для 44гр | operator_provided_data.md |
| 37 | doc_date | 31.03.2026 | CD | дата для 44гр | PL.md |

- _audit: 37
- `doc_status`: confirmed

#### 3.2 Товарные/грузовые строки Массив: Goods[14]
- _array_audit: 14

#### 3.2 Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.4m x 30m Black / Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм. 1.4m x 30m. Черная | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 10 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 42 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 39 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 2 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.6m x 30m Black / Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм 1.6m x 30m. Черная | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 10 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 36.4 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 34 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 2 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | European Pleated Mesh 16mm 1.4m x 30m Black /Европейская плиссированная сетка16 мм 1.4m x 30m . Черная | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 10 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 39 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 36 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 2 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | European Pleated Mesh 16mm 1.6m x 30m. Black /Европейская плиссированная сетка16 мм 1.6m x 30m. Черная | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 10 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 34.4 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 31.5 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 2 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 220g 1.4m x 30m Grey /Сетка от кошек 220 гр "Антикот" 1.4m x 30m. Серая | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 30 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 303 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 277 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Grey /Сетка от кошек 320 гр "Антикот" 1.6m x 30m. Серая | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 50 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 800 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 768 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 50 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.4m x 30m Grey /Сетка от кошек 320 гр "Антикот" 1.4 m x 30m. Серая | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 50 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 710 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 672 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 50 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Black /Сетка от кошек 320 гр "Антикот" 1.6m x 30m. Черная | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 30 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 480 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 461 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.4m x 30m Black /Сетка от кошек 320 гр "Антикот" 1.4m x 30m. Черная | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 30 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 426 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 403 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*50 M2.Black /Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*50 М2 | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 10 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 45.46 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 33 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 2 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*30 M2.Black. Roll size: 1.6*100 m2 /Пылезащитная сетка 30 г "Антипыль " Черная из полиэстера Размер рулона 1,6*100 М2 | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 5 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 22.74 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 16.5 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 2 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Mesh 18 Mesh 0.18mm материал SS304 material SS304 Roll size: 1.6*30 m2 Original / сетка 18, размер ячейки 0,18 мм материал SS304. Размер рулона 1,6*30 М2 цвет оригинальный | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 3 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 51 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 43 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 1 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Mesh 20 0.17 mm материал SS304 material SS304 Roll size: 1.4*30 m black / сетка17 мм материал SS304. Размер рулона 1,4*30 М2 | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 10 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 165 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 146 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 1 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Элемент массива: Goods[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti polent Mesh 100g polyester Black / Антипыльца 100 г черная | CD | описание грузовой строки | PL.md |
| 02 | GoodsQuantity | 10 | CD | кол-во единиц строки | PL.md |
| 03 | GrossWeightQuantity | 305 | CD | брутто строки | PL.md |
| 04 | NetWeightQuantity | 240 | CD | нетто строки | PL.md |
| 05 | PakingQuantity | 5 | CD | кол-во упаковок | PL.md |

- _item_audit: 5

#### 3.2 Транспорт Массив: TransportMeans[2]
- _array_audit: 2

#### 3.2 Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | M869OM67 | CD | рег. номер тягача | copied_from:formalized.cmr.CMRTransport_PrimeMoverStateSignID |
| 02 | ModeCode | 31 | CD | код транспорта (авто) | copied_from:non_formalized.svh.transport_reg_number |
| 03 | NationalityCode | 643 | CD | код страны тягача | cb:country |
| 04 | MoverIndicator | true | CD | признак тягача | константа |

- _item_audit: 4

#### 3.2 Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | AM015667 | CD | рег. номер прицепа | copied_from:formalized.cmr.CMRTransport_TrailerStateSignID |
| 02 | ModeCode | 31 | CD | код транспорта (авто) | copied_from:non_formalized.svh.transport_reg_number |
| 03 | NationalityCode | 000 | CD | код страны прицепа | константа |
| 04 | MoverIndicator | false | CD | признак тягача (ложно) | константа |

- _item_audit: 4

### `document`: CMR
- `uqi_prefix`: formalized.cmr
- `xml_target_root`: AltaE3CMR
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\СМР.md
- `file_name`: СМР.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | LanguageCode | RU | CD | язык документа | константа |
| 02 | CMR_Choice | 1 | CD | системный выбор Альты | константа |
| 03 | RegistrationDocument_RegID | 09225 | CD | номер CMR | СМР.md |
| 04 | RegistrationDocument_DateInf | 27.05.2026 | CD | дата CMR | СМР.md |
| 05 | RegistrationDocument_Place | МАНЬЧЖУРИЯ | CD | место составления | СМР.md |
| 06 | TrakingCargo_TakingCargoDate | 27.05.2026 | CD | дата принятия груза | СМР.md |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CD | страна принятия груза | СМР.md |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза текст | СМР.md |
| 09 | DeliveryPlace_CountryCode | RU | CD | страна доставки | СМР.md |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки текст | СМР.md |
| 11 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryPlace (operator) |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | условия поставки | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode (operator) |
| 13 | GoodsQuantity | 206 | CD | общее число мест по CMR | СМР.md |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3460 | CD | общий вес брутто по CMR | СМР.md |
| 15 | CMRTransport_PrimeMoverStateSignID | M869OM67 | CD | гос. номер тягача | СМР.md |
| 16 | CMRTransport_TrailerStateSignID | AM015667 | CD | гос. номер прицепа | СМР.md |
| 17 | Consignor_NameInf | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD. | CD | отправитель | СМР.md |
| 18 | Consignor_ShortName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD. | CD | краткое наим. ГО | СМР.md |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна ГО | СМР.md |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна ГО текст | СМР.md |
| 21 | Consignor_Address_Region | HEBEI | CD | регион ГО | СМР.md |
| 22 | Consignor_Address_City | WUQIANG, HENGSHUI | CD | город ГО | СМР.md |
| 23 | Consignor_Address_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом ГО | СМР.md |
| 24 | Consignor_Guarantee_OrganizationName | | CD | наим. гаранта ГО | ОТСУТСТВУЕТ |
| 25 | Consignor_Guarantee_ShortName | | CD | краткое наим. гаранта | ОТСУТСТВУЕТ |
| 26 | Consignor_Guarantee_Address_CountryCode | | CD | страна гаранта | ОТСУТСТВУЕТ |
| 27 | Consignor_Guarantee_Address_CounryName | | CD | страна гаранта текст | ОТСУТСТВУЕТ |
| 28 | Consignor_Guarantee_Address_Region | | CD | регион гаранта | ОТСУТСТВУЕТ |
| 29 | Consignor_Guarantee_Address_City | | CD | город гаранта | ОТСУТСТВУЕТ |
| 30 | Consignor_Guarantee_Address_StreetHouse | | CD | улица/дом гаранта | ОТСУТСТВУЕТ |
| 31 | Consignee_NameInf | LLC «SKIF» | CD | получатель | СМР.md |
| 32 | Consignee_ShortName | LLC «SKIF» | CD | краткое наим. получателя | СМР.md |
| 33 | Consignee_OGRNID | 1201600020390 | CD | ОГРН получателя | master_data.md |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН получателя | master_data.md |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП получателя | master_data.md |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | почтовый индекс получателя | СМР.md |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна получателя | СМР.md |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна получателя текст | СМР.md |
| 39 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | СМР.md |
| 40 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | СМР.md |
| 41 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом получателя | СМР.md |
| 42 | doc_gr44 | true | CD | константа | |
| 43 | doc_code | 02015 | CD | константа | |
| 44 | doc_name | CMR | CD | константа | |
| 45 | doc_number | 09225 | CD | номер для 44гр | СМР.md |
| 46 | doc_date | 27.05.2026 | CD | дата для 44гр | СМР.md |

- _audit: 46
- `doc_status`: confirmed

#### 3.3 Товарные/грузовые строки Массив: CMRGoods[1]
- _array_audit: 1

#### 3.3 Элемент массива: CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | авто-нумерация единственной строки |
| 02 | GoodsDescription | ТОВАР ЗАГРУЖЕН, СОГЛАСНО СПЕЦИФИКАЦИИ К INVOICE № 26HL-1103-A ОТ 31.03.2026; INVOICE № 26HL-1103 ОТ 31.03.2026 | CD | описание груза | СМР.md |
| 03 | PakingQuantity | 206 | CD | кол-во упаковок | СМР.md |

- _item_audit: 3

### `document`: Payment Order 1
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\currency_transfer_5_03.04.2026.md
- `file_name`: currency_transfer_5_03.04.2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | константа |
| 02 | PaymentModeCode | 1 | CD | способ платежа | currency_transfer_5_03.04.2026.md |
| 03 | PaymentAmount | 72607.44 | CD | сумма платежа | currency_transfer_5_03.04.2026.md |
| 04 | TransactionKind | 01 | CD | вид операции | константа |
| 05 | Priority | . | CD | очередность платежа | константа |
| 06 | Purpose | PURCHASE OF AN MESH. CONTRACT NO.: 26HL-1103, DATE: MARCH 31ST, 2026, INVOICE NO: 26HL-1103, DATE: 31.03.2026 | CD | назначение платежа | currency_transfer_5_03.04.2026.md |
| 07 | ValueSpelledOut | Семьдесят две тысячи шестьсот семь юаней 44/100 | CD | сумма прописью | currency_transfer_5_03.04.2026.md |
| 08 | DocumentReference_PrDocumentNumber | 5 | CD | номер платежного поручения | currency_transfer_5_03.04.2026.md |
| 09 | DocumentReference_PrDocumentDate | 03.04.2026 | CD | дата платежного поручения | currency_transfer_5_03.04.2026.md |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | currency_transfer_5_03.04.2026.md |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | currency_transfer_5_03.04.2026.md |
| 12 | Payer_KPP | | CD | КПП плательщика | ОТСУТСТВУЕТ |
| 13 | Payer_Bank_BankName | Филиал "Центральный" Банка ВТБ (ПАО), счёт 40702156216150000051 | CD | банк плательщика | currency_transfer_5_03.04.2026.md |
| 14 | Payee_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD, счёт 40807156200610025308 | CD | получатель | currency_transfer_5_03.04.2026.md |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, VTBRCNSHXXX, SHANGHAI, CN | CD | банк получателя | currency_transfer_5_03.04.2026.md |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | currency_transfer_5_03.04.2026.md |
| 17 | PersonName | Дмитрий | CD | имя подписанта | currency_transfer_5_03.04.2026.md |
| 18 | doc_gr44 | true | CD | константа | |
| 19 | doc_code | 04023 | CD | константа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | константа | |
| 21 | doc_number | 5 | CD | номер для 44гр | currency_transfer_5_03.04.2026.md |
| 22 | doc_date | 03.04.2026 | CD | дата для 44гр | currency_transfer_5_03.04.2026.md |

- _audit: 22
- `doc_status`: confirmed

### `document`: Payment Order 2
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\currency_transfer_6_08.04.2026.md
- `file_name`: currency_transfer_6_08.04.2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | константа |
| 02 | PaymentModeCode | 1 | CD | способ платежа | currency_transfer_6_08.04.2026.md |
| 03 | PaymentAmount | 15360.00 | CD | сумма платежа | currency_transfer_6_08.04.2026.md |
| 04 | TransactionKind | 01 | CD | вид операции | константа |
| 05 | Priority | . | CD | очередность платежа | константа |
| 06 | Purpose | PURCHASE OF AN MESH. CONTRACT NO.: 26HL-1103, DATE: MARCH 31ST, 2026, INVOICE NO: 26HL-1103-A, DATE: 31.03.2026 | CD | назначение платежа | currency_transfer_6_08.04.2026.md |
| 07 | ValueSpelledOut | Пятнадцать тысяч триста шестьдесят юаней 00/100 | CD | сумма прописью | currency_transfer_6_08.04.2026.md |
| 08 | DocumentReference_PrDocumentNumber | 6 | CD | номер платежного поручения | currency_transfer_6_08.04.2026.md |
| 09 | DocumentReference_PrDocumentDate | 08.04.2026 | CD | дата платежного поручения | currency_transfer_6_08.04.2026.md |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | currency_transfer_6_08.04.2026.md |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | currency_transfer_6_08.04.2026.md |
| 12 | Payer_KPP | | CD | КПП плательщика | ОТСУТСТВУЕТ |
| 13 | Payer_Bank_BankName | Филиал "Центральный" Банка ВТБ (ПАО), счёт 40702156216150000051 | CD | банк плательщика | currency_transfer_6_08.04.2026.md |
| 14 | Payee_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD, счёт 40807156200610025308 | CD | получатель | currency_transfer_6_08.04.2026.md |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, VTBRCNSHXXX, SHANGHAI, CN | CD | банк получателя | currency_transfer_6_08.04.2026.md |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | currency_transfer_6_08.04.2026.md |
| 17 | PersonName | Дмитрий | CD | имя подписанта | currency_transfer_6_08.04.2026.md |
| 18 | doc_gr44 | true | CD | константа | |
| 19 | doc_code | 04023 | CD | константа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | константа | |
| 21 | doc_number | 6 | CD | номер для 44гр | currency_transfer_6_08.04.2026.md |
| 22 | doc_date | 08.04.2026 | CD | дата для 44гр | currency_transfer_6_08.04.2026.md |

- _audit: 22
- `doc_status`: confirmed

### `document`: Insurance Invoice
- `uqi_prefix`: formalized.insurance_invoice
- `xml_target_root`: AltaFreeDoc
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\Счет_№26-09225-tl_1_от_11-05-2026.md
- `file_name`: Счет_№26-09225-tl_1_от_11-05-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа | Счет_№26-09225-tl_1_от_11-05-2026.md |
| 03 | DocumentHead_DocumentDate | 11.05.2026 | CD | дата документа | Счет_№26-09225-tl_1_от_11-05-2026.md |
| 04 | DocumentHead_DocumentNumber | 26-09225-tl/1 | CD | номер документа | Счет_№26-09225-tl_1_от_11-05-2026.md |
| 05 | TextPara | link:C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\Счет_№26-09225-tl_1_от_11-05-2026.md | CD | ссылка на техописание | константа |
| 06 | doc_gr44 | true | CD | константа | |
| 07 | doc_code | 04111 | CD | константа | |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | константа | |
| 09 | doc_number | 26-09225-tl/1 | CD | номер для 44гр | Счет_№26-09225-tl_1_от_11-05-2026.md |
| 10 | doc_date | 11.05.2026 | CD | дата для 44гр | Счет_№26-09225-tl_1_от_11-05-2026.md |
| 11 | insurance_to_border | 798.04 | CD | стоимость страхования | Счет_№26-09225-tl_1_от_11-05-2026.md |
| 12 | insurance_currency | RUB | CD | валюта страхования | Счет_№26-09225-tl_1_от_11-05-2026.md |

- _audit: 12
- `doc_status`: confirmed

### `document`: Service Invoice
- `uqi_prefix`: formalized.service_invoice
- `xml_target_root`: AltaServiceInvoice
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\Счет_№26-09225-tl_от_12-05-2026.md
- `file_name`: Счет_№26-09225-tl_от_12-05-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CD | системный признак | константа |
| 02 | TotalServiceCost | 2610.00 | CD | общая стоимость услуг | Счет_№26-09225-tl_от_12-05-2026.md |
| 03 | Currency | USD | CD | валюта стоимости | Счет_№26-09225-tl_от_12-05-2026.md |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | перевозчик | Счет_№26-09225-tl_от_12-05-2026.md |
| 05 | ServiceProvider_PaymentRequisitions | АО "Альфа-Банк", БИК 044525593, Сч. № 40702810001600010931 | CD | реквизиты перевозчика | Счет_№26-09225-tl_от_12-05-2026.md |
| 06 | ContractDetails_PrDocumentNumber | KOOO/26651/M | CD | номер договора перевозки | Счет_№26-09225-tl_от_12-05-2026.md |
| 07 | ContractDetails_PrDocumentDate | 13-05-2025 | CD | дата договора перевозки | Счет_№26-09225-tl_от_12-05-2026.md |
| 08 | PaymentDocument_PrDocumentNumber | 26-09225-tl | CD | номер заказа | Счет_№26-09225-tl_от_12-05-2026.md |
| 09 | PaymentDocument_PrDocumentDate | 07.04.2026 | CD | дата заказа | Счет_№26-09225-tl_от_12-05-2026.md |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование документа | Счет_№26-09225-tl_от_12-05-2026.md |
| 11 | Registration_PrDocumentNumber | 26-09225-tl | CD | номер счета | Счет_№26-09225-tl_от_12-05-2026.md |
| 12 | Registration_PrDocumentDate | 12.05.2026 | CD | дата счета | Счет_№26-09225-tl_от_12-05-2026.md |
| 13 | Consignor_OrganizationName | | CD | отправитель услуг | ОТСУТСТВУЕТ |
| 14 | Consignor_SubjectAddressDetails_PostalCode | | CD | индекс отправителя | ОТСУТСТВУЕТ |
| 15 | Consignor_SubjectAddressDetails_CountryCode | | CD | страна отправителя | ОТСУТСТВУЕТ |
| 16 | Consignor_SubjectAddressDetails_CounryName | | CD | страна отправителя текст | ОТСУТСТВУЕТ |
| 17 | Consignor_SubjectAddressDetails_Region | | CD | регион отправителя | ОТСУТСТВУЕТ |
| 18 | Consignor_SubjectAddressDetails_Town | | CD | город отправителя | ОТСУТСТВУЕТ |
| 19 | Consignor_SubjectAddressDetails_StreetHouse | | CD | улица/дом отправителя | ОТСУТСТВУЕТ |
| 20 | Consignee_OrganizationName | Общество с ограниченной ответственностью "СКИФ" | CD | получатель услуг | Счет_№26-09225-tl_от_12-05-2026.md |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CD | ОГРН получателя | master_data.md |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН получателя | master_data.md |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП получателя | master_data.md |
| 24 | Consignee_SubjectAddressDetails_PostalCode | 423800 | CD | почтовый индекс получателя | Счет_№26-09225-tl_от_12-05-2026.md |
| 25 | Consignee_SubjectAddressDetails_CountryCode | RU | CD | страна получателя | Счет_№26-09225-tl_от_12-05-2026.md |
| 26 | Consignee_SubjectAddressDetails_CounryName | РОССИЯ | CD | страна получателя текст | Счет_№26-09225-tl_от_12-05-2026.md |
| 27 | Consignee_SubjectAddressDetails_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион получателя | Счет_№26-09225-tl_от_12-05-2026.md |
| 28 | Consignee_SubjectAddressDetails_Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город получателя | Счет_№26-09225-tl_от_12-05-2026.md |
| 29 | Consignee_SubjectAddressDetails_StreetHouse | проезд Хлебный | CD | улица получателя | Счет_№26-09225-tl_от_12-05-2026.md |
| 30 | Consignee_SubjectAddressDetails_House | д. 30 | CD | дом получателя | Счет_№26-09225-tl_от_12-05-2026.md |
| 31 | Consignee_SubjectAddressDetails_Room | кв/оф. 211 | CD | офис получателя | Счет_№26-09225-tl_от_12-05-2026.md |
| 32 | Signature_Choice | 2 | CD | признак подписи (руков./бух.) | константа |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | ОТСУТСТВУЕТ |
| 34 | IndividualEntrepreneur_PersonName | | CD | имя ИП | ОТСУТСТВУЕТ |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | ОТСУТСТВУЕТ |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CD | фамилия директора | Счет_№26-09225-tl_от_12-05-2026.md |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л. | CD | имя директора | Счет_№26-09225-tl_от_12-05-2026.md |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А. | CD | отчество директора | Счет_№26-09225-tl_от_12-05-2026.md |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | Счет_№26-09225-tl_от_12-05-2026.md |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О. | CD | имя бухгалтера | Счет_№26-09225-tl_от_12-05-2026.md |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А. | CD | отчество бухгалтера | Счет_№26-09225-tl_от_12-05-2026.md |
| 42 | doc_gr44 | true | CD | константа | |
| 43 | doc_code | 04031 | CD | константа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | константа | |
| 45 | doc_number | 26-09225-tl | CD | номер для 44гр | Счет_№26-09225-tl_от_12-05-2026.md |
| 46 | doc_date | 12.05.2026 | CD | дата для 44гр | Счет_№26-09225-tl_от_12-05-2026.md |
| 47 | transport_to_border | 1358.00 | CD | стоимость до границы | Счет_№26-09225-tl_от_12-05-2026.md |
| 48 | transport_currency | USD | CD | валюта стоимости до границы | Счет_№26-09225-tl_от_12-05-2026.md |

- _audit: 48
- `doc_status`: confirmed

#### 3.5 Услуги Массив: ServiceDescription[2]
- _array_audit: 2

#### 3.5 Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №KOOO/26651/M от 13-05-2025 по транспортному заказу №26-09225-tl от 07.04.2026 по маршруту: China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | Счет_№26-09225-tl_от_12-05-2026.md |
| 02 | CurrencyCode | USD | CD | валюта строки | Счет_№26-09225-tl_от_12-05-2026.md |
| 03 | ServiceName | China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) | CD | маршрут услуги | Счет_№26-09225-tl_от_12-05-2026.md |
| 04 | TaxRate | 0 | CD | ставка налога | Счет_№26-09225-tl_от_12-05-2026.md |
| 05 | TaxSum | 0.00 | CD | сумма налога | Счет_№26-09225-tl_от_12-05-2026.md |
| 06 | ServiceCost_Amount | 1358.00 | CD | стоимость услуги | Счет_№26-09225-tl_от_12-05-2026.md |
| 07 | ServiceCost_Currency | USD | CD | валюта услуги | Счет_№26-09225-tl_от_12-05-2026.md |

- _item_audit: 7

#### 3.5 Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, г. Набережные Челны | CD | описание услуги | Счет_№26-09225-tl_от_12-05-2026.md |
| 02 | CurrencyCode | USD | CD | валюта строки | Счет_№26-09225-tl_от_12-05-2026.md |
| 03 | ServiceName | граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, г. Набережные Челны | CD | маршрут услуги | Счет_№26-09225-tl_от_12-05-2026.md |
| 04 | TaxRate | 0 | CD | ставка налога | Счет_№26-09225-tl_от_12-05-2026.md |
| 05 | TaxSum | 0.00 | CD | сумма налога | Счет_№26-09225-tl_от_12-05-2026.md |
| 06 | ServiceCost_Amount | 1252.00 | CD | стоимость услуги | Счет_№26-09225-tl_от_12-05-2026.md |
| 07 | ServiceCost_Currency | USD | CD | валюта услуги | Счет_№26-09225-tl_от_12-05-2026.md |

- _item_audit: 7

### `document`: Tech Description
- `uqi_prefix`: formalized.tech_description
- `xml_target_root`: AltaFreeDoc
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\техничка Антикот, антипыльца нержавейка плесе  .md
- `file_name`: техничка Антикот, антипыльца нержавейка плесе  .md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование техописания | техничка Антикот, антипыльца нержавейка плесе  .md |
| 03 | DocumentHead_DocumentDate | 31.03.2026 | CD | дата техописания | техничка Антикот, антипыльца нержавейка плесе  .md |
| 04 | DocumentHead_DocumentNumber | 31032026 | CD | номер техописания | техничка Антикот, антипыльца нержавейка плесе  .md |
| 05 | TextPara | link:C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\техничка Антикот, антипыльца нержавейка плесе  .md | CD | ссылка на техописание | константа |
| 06 | doc_gr44 | true | CD | константа | |
| 07 | doc_code | 05999 | CD | константа | |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | константа | |
| 09 | doc_number | 31032026 | CD | номер для 44гр | техничка Антикот, антипыльца нержавейка плесе  .md |
| 10 | doc_date | 31.03.2026 | CD | дата для 44гр | техничка Антикот, антипыльца нержавейка плесе  .md |

- _audit: 10
- `doc_status`: confirmed

### `document`: Goods Description 1
- `uqi_prefix`: non_formalized.goods_description_1
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\CL 26HL-1103-A.md
- `file_name`: CL 26HL-1103-A.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | константа | |

- _audit: 1
- `doc_status`: confirmed

#### 3.5 Товарные группы Массив: goods[1]
- _array_audit: 1

#### 3.5 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103-A.md |
| 02 | description | СЕТКА МОСКИТНАЯ РУЛОННАЯ ИЗ ПОЛИЭСТЕРА (100% ПОЛИЭФИРНЫЕ ВОЛОКНА), ПЛЕТЕНАЯ, ОДНОТОННО ОКРАШЕННАЯ (ЧЕРНАЯ), ДЛЯ ЗАЩИТЫ ПОМЕЩЕНИЙ ОТ НАСЕКОМЫХ, ПЫЛИ И ПЫЛЬЦЫ. РАЗМЕР РУЛОНА 1.6*30 М, ПЛОТНОСТЬ 100 Г/М2. НЕ ВОЕННОГО НАЗНАЧЕНИЯ, НЕ ДЛЯ КОНТАКТА С ПИЩЕЙ, НЕ КРИПТОГРАФИЯ. ТМ: ОТСУТСТВУЕТ. | CD | описание для графы 31 | расчет |

- _item_audit: 2

### `document`: Goods Description 2
- `uqi_prefix`: non_formalized.goods_description_2
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\CL 26HL-1103.md
- `file_name`: CL 26HL-1103.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | константа | |

- _audit: 1
- `doc_status`: confirmed

#### 3.5 Товарные группы Массив: goods[3]
- _array_audit: 3

#### 3.5 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 6303921000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | description | СЕТЧАТАЯ ТКАНЬ (ШТОРЫ) ИЗ ПОЛИЭСТЕРА (100% СИНТЕТИЧЕСКИЕ ВОЛОКНА) С ПЛИССИРОВКОЙ, ГОФРИРОВАННАЯ (СЕТКА-ГАРМОШКА), ДЛЯ ИСПОЛЬЗОВАНИЯ В МОСКИТНЫХ СИСТЕМАХ ОКН/ДВЕРЕЙ ДЛЯ ЗАЩИТЫ ОТ НАСЕКОМЫХ. МЕТОД ИЗГОТОВЛЕНИЯ: ПЛЕТЕНИЕ. РАЗМЕРЫ: 1.4*30 М, 1.6*30 М. ТМ: ОТСУТСТВУЕТ. | CD | описание для графы 31 | расчет |

- _item_audit: 2

#### 3.5 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 5804101000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | description | СЕТКА МОСКИТНАЯ ИЗ ПОЛИЭСТЕРА С ПОКРЫТИЕМ ПВХ, ПЛЕТЕНАЯ, ДЛЯ ЗАЩИТЫ ОТ НАСЕКОМЫХ, С ПОВЫШЕННОЙ ПРОЧНОСТЬЮ ОТ КОГТЕЙ ЖИВОТНЫХ (АНТИКОТ) ИЛИ МЕЛКОЙ ЯЧЕЙКОЙ (АНТИПЫЛЬ), ДЛЯ ПЛАСТИКОВЫХ ОКОН. РАЗМЕРЫ РУЛОНОВ: 1.4*30 М, 1.6*30 М, 1.6*50 М, 1.6*100 М. ТМ: ОТСУТСТВУЕТ. | CD | описание для графы 31 | расчет |

- _item_audit: 2

#### 3.5 Элемент массива: goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 7314490000 | CD | код ТН ВЭД | CL 26HL-1103.md |
| 02 | description | СЕТКА МОСКИТНАЯ МЕТАЛЛИЧЕСКАЯ ИЗ ПРОВОЛОКИ ИЗ НЕРЖАВЕЮЩЕЙ СТАЛИ 304 (ЖЕЛЕЗО/СТАЛЬ), ПЕРЕПЛЕТЕННАЯ, СВЕРХПРОЧНАЯ, ДЛЯ ОКН И ДВЕРЕЙ, ЗАЩИТА ОТ НАСЕКОМЫХ И ГРЫЗУНОВ. ТОЛЩИНА 0.23 ММ, РАЗМЕРЫ РУЛОНА: 1.6*30 М, 1.4*30 М. ТМ: ОТСУТСТВУЕТ. | CD | описание для графы 31 | расчет |

- _item_audit: 2

### `document`: Transit Declaration
- `uqi_prefix`: non_formalized.td
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\Транзитка 10719110_300526_5086483_reg.md
- `file_name`: Транзитка 10719110_300526_5086483_reg.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs_post_code | 10719110 | CD | код таможни отправления | Транзитка 10719110_300526_5086483_reg.md |
| 02 | customs_post_name | МАПП Забайкальск | CD | наим. таможни отправления | Транзитка 10719110_300526_5086483_reg.md |
| 03 | transport_reg_number | M869OM67/AM015667 | CD | номера ТС по ТД | Транзитка 10719110_300526_5086483_reg.md |
| 04 | doc_gr44 | true | CD | константа | |
| 05 | doc_code | 09013 | CD | константа | |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | константа | |
| 07 | doc_number | 10719110/300526/5086483 | CD | номер ТД | Транзитка 10719110_300526_5086483_reg.md |
| 08 | doc_date | 30.05.2026 | CD | дата ТД | Транзитка 10719110_300526_5086483_reg.md |

- _audit: 8
- `doc_status`: confirmed

### `document`: Storage Report
- `uqi_prefix`: non_formalized.svh
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\ДО_ОТЧЕТ.md
- `file_name`: ДО_ОТЧЕТ.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер свидетельства СВХ | ДО_ОТЧЕТ.md |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата свидетельства СВХ | ДО_ОТЧЕТ.md |
| 03 | actual_gross_weight | 3460 | CD | фактический вес брутто | ДО_ОТЧЕТ.md |
| 04 | actual_places | 206 | CD | фактическое число мест | ДО_ОТЧЕТ.md |
| 05 | transport_reg_number | М 869 ОМ 67, АМ 0156 67 | CD | номера ТС по отчету СВХ | ДО_ОТЧЕТ.md |
| 06 | doc_gr44 | false | CD | константа | |

- _audit: 6
- `doc_status`: confirmed

#### 3.5 СВХ Товары Массив: goods[3]
- _array_audit: 3

#### 3.5 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 5804101000 | CD | код товара | ДО_ОТЧЕТ.md |
| 02 | places | 197 | CD | кол-во мест строки | ДО_ОТЧЕТ.md |
| 03 | gross_weight_kg | 3092.2 | CD | брутто строки | ДО_ОТЧЕТ.md |
| 04 | cost | 78114 | CD | стоимость по строке | ДО_ОТЧЕТ.md |
| 05 | currency_code | CNY | CD | валюта строки | ДО_ОТЧЕТ.md |

- _item_audit: 5

#### 3.5 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 6303921000 | CD | код товара | ДО_ОТЧЕТ.md |
| 02 | places | 8 | CD | кол-во мест строки | ДО_ОТЧЕТ.md |
| 03 | gross_weight_kg | 151.8 | CD | брутто строки | ДО_ОТЧЕТ.md |
| 04 | cost | 4185 | CD | стоимость по строке | ДО_ОТЧЕТ.md |
| 05 | currency_code | CNY | CD | валюта строки | ДО_ОТЧЕТ.md |

- _item_audit: 5

#### 3.5 Элемент массива: goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tnved | 7314490000 | CD | код товара | ДО_ОТЧЕТ.md |
| 02 | places | 1 | CD | кол-во мест строки | ДО_ОТЧЕТ.md |
| 03 | gross_weight_kg | 216 | CD | брутто строки | ДО_ОТЧЕТ.md |
| 04 | cost | 5668.44 | CD | стоимость по строке | ДО_ОТЧЕТ.md |
| 05 | currency_code | CNY | CD | валюта строки | ДО_ОТЧЕТ.md |

- _item_audit: 5

### `document`: Storage Report Additional Sheet
- `uqi_prefix`: non_formalized.svh_additional_sheet_1
- `path`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\МоскитнаяСеткаWuqiang\01\md\ДО_ДОБАВОЧНЫЙ_ЛИСТ.md
- `file_name`: ДО_ДОБАВОЧНЫЙ_ЛИСТ.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | number | 1 | CD | номер доп. листа СВХ | ДО_ДОБАВОЧНЫЙ_ЛИСТ.md |
| 02 | date | 05.06.2026 | CD | дата доп. листа СВХ | ДО_ДОБАВОЧНЫЙ_ЛИСТ.md |
| 03 | actual_gross_weight | 3460 | CD | фактический вес брутто | ДО_ДОБАВОЧНЫЙ_ЛИСТ.md |
| 04 | actual_places | 206 | CD | фактическое число мест | ДО_ДОБАВОЧНЫЙ_ЛИСТ.md |
| 05 | transport_reg_number | М 869 ОМ 67 | CD | номера ТС | ДО_ДОБАВОЧНЫЙ_ЛИСТ.md |
| 06 | doc_gr44 | false | CD | константа | |

- _audit: 6
- `doc_status`: confirmed

### Итогo, по файлу:

`total_unreliable_fields`: 0
`primary_status`: confirmed

## 5. Нерешенные вопросы (Issues)

(Все вопросы успешно разрешены на базе предоставленных оператором ответов и кросс-дока)

## 6. `unreliable_fields`:

