# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСеткаWuqiang
- `путь к папке поставки`: alta\source\МоскитнаяСеткаWuqiang
- `direction`: ИМ
- `тип поставки`: 1 ДТ/3 товара
- `источники данных:` md + master_keys.md + master_data.md

## 2. formalized/master_data/non_formalized:

### `document`: Contract
- `uqi_prefix`: master_data.contract
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 02 | doc_code | 03011 | CD | код вида документа | master_data.md |
| 03 | doc_name | КОНТРАКТ | CD | наименование документа | master_data.md |
| 04 | doc_number | 26HL-1103 | CD | номер контракта | master_data.md |
| 05 | doc_date | 31.03.2026 | CD | дата контракта | master_data.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: EGRUL
- `uqi_prefix`: master_data.egrul
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование организации | master_data.md |
| 02 | ShortName | ООО "СКИФ" | CD | краткое наименование | master_data.md |
| 03 | OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 04 | INN | 1650389298 | CD | ИНН | master_data.md |
| 05 | KPP | 165001001 | CD | КПП | master_data.md |
| 06 | Address_PostalCode | 423800 | CD | индекс | master_data.md |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 08 | Address_CounryName | РОССИЯ | CD | страна, текст | master_data.md |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data.md |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data.md |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | master_data.md |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | master_data.md |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | master_data.md |
| 14 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 15 | doc_code | 04011 | CD | код вида документа | master_data.md |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | master_data.md |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер выписки | master_data.md |
| 18 | doc_date | 14.07.2025 | CD | дата выписки | master_data.md |

- _audit: 18
- `doc_status`: confirmed

### `document`: Personal Passport
- `uqi_prefix`: master_data.passport
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | master_data.md |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | master_data.md |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | master_data.md |
| 04 | CardSeries | 63 09 | CD | серия | master_data.md |
| 05 | CardNumber | 449948 | CD | номер | master_data.md |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | master_data.md |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | master_data.md |
| 08 | Phone | +7 927-222-0500 | CD | телефон | master_data.md |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | master_data.md |
| 10 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 11 | doc_code | 11001 | CD | код вида документа | master_data.md |
| 12 | doc_name | ПАСПОРТ | CD | наименование документа | master_data.md |
| 13 | doc_number | 63 09 449948 | CD | номер паспорта | master_data.md |
| 14 | doc_date | 11.03.2010 | CD | дата выдачи | master_data.md |

- _audit: 14
- `doc_status`: confirmed

### `document`: Letter of Attorney
- `uqi_prefix`: master_data.letter_of_attorney
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data.md |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_data.md |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_data.md |
| 05 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 06 | doc_code | 11004 | CD | код вида документа | master_data.md |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | master_data.md |
| 08 | doc_number | 1 | CD | номер доверенности | master_data.md |
| 09 | doc_date | 01.02.2026 | CD | дата доверенности | master_data.md |

- _audit: 9
- `doc_status`: confirmed

### `document`: Transport Contract
- `uqi_prefix`: master_data.transport_contract
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 02 | doc_code | 04033 | CD | код вида документа | master_data.md |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование документа | master_data.md |
| 04 | doc_number | КООО/26651/М | CD | номер договора | master_data.md |
| 05 | doc_date | 13.05.2025 | CD | дата договора | master_data.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter
- `uqi_prefix`: master_data.exemption_letter
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 02 | doc_code | 09023 | CD | код вида документа | master_data.md |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | master_data.md |
| 04 | doc_number | 24968/МЛ10 | CD | номер отказного письма | master_data.md |
| 05 | doc_date | 20.08.2025 | CD | дата отказного письма | master_data.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter (source)
- `uqi_prefix`: master_data.exemption_letter_source
- `path`: alta\master_data.md
- `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | признак включения в графу 44 | master_data.md |
| 02 | doc_code | 09999 | CD | код вида документа | master_data.md |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | master_data.md |
| 04 | doc_number | 24968/МЛ10 | CD | номер отказного письма | master_data.md |
| 05 | doc_date | 20.08.2025 | CD | дата отказного письма | master_data.md |

- _audit: 5
- `doc_status`: confirmed

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_1
- `xml_target_root`: AltaE2I
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103 .md
- `file_name`: CL 26HL-1103 .md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyRate | | pending | курс валюты | |
| 02 | CurrencyCode | CNY | CD | валюта инвойса | |
| 03 | DocumentCode | 04021 | CD | код вида документа | |
| 04 | PlacesQuantity | 201 | CD | кол-во грузовых мест | |
| 05 | PlacesDescription | ПАКЕТ | CD | описание мест | |
| 06 | GrossWeightQuantity | 3175.00 | CD | общий вес брутто | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 07 | NetWeightQuantity | 2960.00 | CD | общий вес нетто | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | GCost | 72607.44 | CD | системное поле Альты | |
| 09 | TotalCost | 72607.44 | CD | итого по инвойсу | |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий поставки | |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 14 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 16 | Registration_PrDocumentName | КОММЕРЧЕСКИЙ ИНВОЙС | CD | наименование документа | |
| 17 | Registration_PrDocumentNumber | 26HL-1103 | CD | номер инвойса | |
| 18 | Registration_PrDocumentDate | 31.03.2026 | CD | дата инвойса | |
| 19 | Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта-ссылки | |
| 20 | Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта-ссылки | |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | copied_from:master_data.egrul.INN (alta\master_data.md) |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | copied_from:master_data.egrul.KPP (alta\master_data.md) |
| 23 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование покупателя | copied_from:master_data.egrul.OrganizationName (alta\master_data.md) |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | copied_from:master_data.egrul.Address_PostalCode (alta\master_data.md) |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | copied_from:master_data.egrul.Address_CountryCode (alta\master_data.md) |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя, текст | copied_from:master_data.egrul.Address_CounryName (alta\master_data.md) |
| 27 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | copied_from:master_data.egrul.Address_Region (alta\master_data.md) |
| 28 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | copied_from:master_data.egrul.Address_City (alta\master_data.md) |
| 29 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | copied_from:master_data.egrul.Address_StreetHouse (alta\master_data.md) |
| 30 | Seler_Name | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | продавец | |
| 31 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца alpha-2 | |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | |
| 33 | Seler_PostalAddress_Region | HEBEI | CD | регион продавца | |
| 34 | Seler_PostalAddress_City | WUQIANG, HENGSHUI | CD | город/район продавца | |
| 35 | Seler_PostalAddress_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом одной строкой | |
| 36 | Consignor_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | грузоотправитель | нормализация: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | нормализация: consignor=seller |
| 38 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | нормализация: consignor=seller |
| 39 | Consignor_Address_Region | HEBEI | CD | регион | нормализация: consignor=seller |
| 40 | Consignor_Address_City | WUQIANG, HENGSHUI | CD | город/район | нормализация: consignor=seller |
| 41 | Consignor_Address_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом одной строкой | нормализация: consignor=seller |
| 42 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | copied_from:master_data.egrul.OrganizationName (alta\master_data.md) |
| 43 | Consignee_OGRN | 1201600020390 | CD | ОГРН | copied_from:master_data.egrul.OGRN (alta\master_data.md) |
| 44 | Consignee_INN | 1650389298 | CD | ИНН | copied_from:master_data.egrul.INN (alta\master_data.md) |
| 45 | Consignee_KPP | 165001001 | CD | КПП | copied_from:master_data.egrul.KPP (alta\master_data.md) |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | copied_from:master_data.egrul.Address_PostalCode (alta\master_data.md) |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | copied_from:master_data.egrul.Address_CountryCode (alta\master_data.md) |
| 48 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | copied_from:master_data.egrul.Address_CounryName (alta\master_data.md) |
| 49 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | copied_from:master_data.egrul.Address_Region (alta\master_data.md) |
| 50 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | copied_from:master_data.egrul.Address_City (alta\master_data.md) |
| 51 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | copied_from:master_data.egrul.Address_StreetHouse (alta\master_data.md) |
| 52 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 53 | doc_code | 04021 | CD | код вида документа | |
| 54 | doc_name | ИНВОЙС | CD | наименование документа | |
| 55 | doc_number | 26HL-1103 | CD | номер инвойса | |
| 56 | doc_date | 31.03.2026 | CD | дата инвойса | |

- _audit: 56
- `doc_status`: confirmed

#### formalized.invoice_1 Массив: InvoiceGoods[13]
- _array_audit: 13

#### formalized.invoice_1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.4m x 30m Black / Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм. 1.4м х 30м. Черная | CD | описание товара | |
| 03 | GoodsQuantity | 420 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 42 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 39 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 2.3 | CD | цена за единицу | |
| 10 | TotalCost | 966.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.4m x 30m Black | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.6m x 30m Black / Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм 1.6м х 30м. Черная | CD | описание товара | |
| 03 | GoodsQuantity | 480 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 36.4 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 34 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 2.3 | CD | цена за единицу | |
| 10 | TotalCost | 1104.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.6m x 30m Black | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | European Pleated Mesh 16mm 1.4m x 30m Black / Европейская плиссированная сетка 16 мм 1.4м х 30м. Черная | CD | описание товара | |
| 03 | GoodsQuantity | 420 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 39 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 36 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 2.35 | CD | цена за единицу | |
| 10 | TotalCost | 987.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | European Pleated Mesh 16mm 1.4m x 30m Black | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | European Pleated Mesh 16mm 1.6m x 30m Black / Европейская плиссированная сетка 16 мм 1.6м х 30м. Черная | CD | описание товара | |
| 03 | GoodsQuantity | 480 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 34.4 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 31.5 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 2.35 | CD | цена за единицу | |
| 10 | TotalCost | 1128.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | European Pleated Mesh 16mm 1.6m x 30m Black | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 220g 1.4m x 30m Grey / Сетка от кошек 220 гр "Антикот" 1.4м х 30м. Серая | CD | описание товара | |
| 03 | GoodsQuantity | 1260 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 313 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 277 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 4.7 | CD | цена за единицу | |
| 10 | TotalCost | 5922.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 220g 1.4m x 30m Grey | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Grey / Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Серая | CD | описание товара | |
| 03 | GoodsQuantity | 2400 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 800 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 768 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 5.8 | CD | цена за единицу | |
| 10 | TotalCost | 13920.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.6m x 30m Grey | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.4 m x 30m Grey / Сетка от кошек 320 гр "Антикот" 1.4 м х 30м. Серая | CD | описание товара | |
| 03 | GoodsQuantity | 2100 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 710 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 672 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 5.8 | CD | цена за единицу | |
| 10 | TotalCost | 12180.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.4 m x 30m Grey | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Black / Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Черная | CD | описание товара | |
| 03 | GoodsQuantity | 1440 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 490 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 461 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 5.8 | CD | цена за единицу | |
| 10 | TotalCost | 8352.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.6m x 30m Black | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.4m x 30m Black / Сетка от кошек 320 гр "Антикот" 1.4м х 30м. Черная | CD | описание товара | |
| 03 | GoodsQuantity | 1260 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 426 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 403 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 5.8 | CD | цена за единицу | |
| 10 | TotalCost | 7308.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.4m x 30m Black | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 5 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*50 M2.Black / Пылезащитная сетка 30 г "Антипыльца" Черная из полиэстера Размер рулона 1,6*50 М2 | CD | описание товара | |
| 03 | GoodsQuantity | 800 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 45.46 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 33 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 9.42 | CD | цена за единицу | |
| 10 | TotalCost | 7536.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti Dust Mesh 30g polyester Roll size 1.6*50 M2, Black | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 6 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*30 M2.Black. Roll size: 1.6*100 m2 / Пылезащитная сетка 30 г "Антипыльца" Черная из полиэстера Размер рулона 1,6*30 М2 | CD | описание товара | |
| 03 | GoodsQuantity | 800 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 22.74 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 16.5 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 9.42 | CD | цена за единицу | |
| 10 | TotalCost | 7536.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti Dust Mesh 30g polyester Roll size 1.6*30 M2, Black | CD | модель/модификация | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 7 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7314490000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Mesh 18 Mesh 0.18mm материал SS304 material SS304 Roll size: 1.6*30 m2 Original / сетка 18, размер ячейки 0,18 мм материал SS304. Размер рулона 1,6*30 М2 цвет оригинальный | CD | описание товара | |
| 03 | GoodsQuantity | 144 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 51 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 43 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 7.46 | CD | цена за единицу | |
| 10 | TotalCost | 1073.86 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Mesh 18 Mesh 0.18mm material SS304 Roll size: 1.4*30 m2 | CD | модель/модификация | |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 7314490000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Mesh 20 0.17 mm материал SS304 material SS304 Roll size: 1.4*30 m black / сетка17 мм материал SS304. Размер рулона 1,4*30 М2 Черный | CD | описание товара | |
| 03 | GoodsQuantity | 420 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 165 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 146 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 10.94 | CD | цена за единицу | |
| 10 | TotalCost | 4594.58 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Mesh 20 0.17 mm material SS304 Roll size: 1.4*30 m | CD | модель/модификация | |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции внутри товара | |

- _item_audit: 17

### `document`: Invoice
- `uqi_prefix`: formalized.invoice_2
- `xml_target_root`: AltaE2I
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103-A.md
- `file_name`: CL 26HL-1103-A.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyRate | | pending | курс валюты | |
| 02 | CurrencyCode | CNY | CD | валюта инвойса | |
| 03 | DocumentCode | 04021 | CD | код вида документа | |
| 04 | PlacesQuantity | 5 | CD | кол-во грузовых мест | |
| 05 | PlacesDescription | ПАКЕТ | CD | описание мест | |
| 06 | GrossWeightQuantity | 305.00 | CD | общий вес брутто | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 07 | NetWeightQuantity | 240.00 | CD | общий вес нетто | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | GCost | 15360.00 | CD | системное поле Альты | |
| 09 | TotalCost | 15360.00 | CD | итого по инвойсу | |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий поставки | |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 14 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 16 | Registration_PrDocumentName | КОММЕРЧЕСКИЙ ИНВОЙС | CD | наименование документа | |
| 17 | Registration_PrDocumentNumber | 26HL-1103-A | CD | номер инвойса | |
| 18 | Registration_PrDocumentDate | 31.03.2026 | CD | дата инвойса | |
| 19 | Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта-ссылки | |
| 20 | Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта-ссылки | |
| 21 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | copied_from:master_data.egrul.INN (alta\master_data.md) |
| 22 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | copied_from:master_data.egrul.KPP (alta\master_data.md) |
| 23 | Buyer_Name | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование покупателя | copied_from:master_data.egrul.OrganizationName (alta\master_data.md) |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | copied_from:master_data.egrul.Address_PostalCode (alta\master_data.md) |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | copied_from:master_data.egrul.Address_CountryCode (alta\master_data.md) |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя, текст | copied_from:master_data.egrul.Address_CounryName (alta\master_data.md) |
| 27 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | copied_from:master_data.egrul.Address_Region (alta\master_data.md) |
| 28 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | copied_from:master_data.egrul.Address_City (alta\master_data.md) |
| 29 | Buyer_PostalAddress_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | copied_from:master_data.egrul.Address_StreetHouse (alta\master_data.md) |
| 30 | Seler_Name | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | продавец | |
| 31 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца alpha-2 | |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | |
| 33 | Seler_PostalAddress_Region | HEBEI | CD | регион продавца | |
| 34 | Seler_PostalAddress_City | WUQIANG, HENGSHUI | CD | город/район продавца | |
| 35 | Seler_PostalAddress_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом одной строкой | |
| 36 | Consignor_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | грузоотправитель | нормализация: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | нормализация: consignor=seller |
| 38 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | нормализация: consignor=seller |
| 39 | Consignor_Address_Region | HEBEI | CD | регион | нормализация: consignor=seller |
| 40 | Consignor_Address_City | WUQIANG, HENGSHUI | CD | город/район | нормализация: consignor=seller |
| 41 | Consignor_Address_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом одной строкой | нормализация: consignor=seller |
| 42 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | copied_from:master_data.egrul.OrganizationName (alta\master_data.md) |
| 43 | Consignee_OGRN | 1201600020390 | CD | ОГРН | copied_from:master_data.egrul.OGRN (alta\master_data.md) |
| 44 | Consignee_INN | 1650389298 | CD | ИНН | copied_from:master_data.egrul.INN (alta\master_data.md) |
| 45 | Consignee_KPP | 165001001 | CD | КПП | copied_from:master_data.egrul.KPP (alta\master_data.md) |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | copied_from:master_data.egrul.Address_PostalCode (alta\master_data.md) |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | copied_from:master_data.egrul.Address_CountryCode (alta\master_data.md) |
| 48 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | copied_from:master_data.egrul.Address_CounryName (alta\master_data.md) |
| 49 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | copied_from:master_data.egrul.Address_Region (alta\master_data.md) |
| 50 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | copied_from:master_data.egrul.Address_City (alta\master_data.md) |
| 51 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | copied_from:master_data.egrul.Address_StreetHouse (alta\master_data.md) |
| 52 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 53 | doc_code | 04021 | CD | код вида документа | |
| 54 | doc_name | ИНВОЙС | CD | наименование документа | |
| 55 | doc_number | 26HL-1103-A | CD | номер инвойса | |
| 56 | doc_date | 31.03.2026 | CD | дата инвойса | |

- _audit: 56
- `doc_status`: confirmed

#### formalized.invoice_2 Массив: InvoiceGoods[1]
- _array_audit: 1

#### formalized.invoice_2 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti polent Mesh 100g polyester Black 1,6 м*30 м / Антипыльца 100 г черная 1,6 м*30 м | CD | описание товара | |
| 03 | GoodsQuantity | 2400 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | | CD | количество в доп.ед. | |
| 05 | goods_supplementary_uom_name | | CD | наименование доп.ед. | |
| 06 | MeasureUnitQualifierName | | CD | единица измерения доп.количества | |
| 07 | GrossWeightQuantity | 305 | CD | брутто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 08 | NetWeightQuantity | 240 | CD | нетто по строке | copied_from:specification (alta\source\МоскитнаяСеткаWuqiang\md\спецификация .md) |
| 09 | Price | 6.4 | CD | цена за единицу | |
| 10 | TotalCost | 15360.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | цифровой код страны происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | товарный знак | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti polent Mesh 100g polyester Black 1.6m*30m | CD | модель/модификация | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции внутри товара | |

- _item_audit: 17

### `document`: Packing List
- `uqi_prefix`: formalized.packing_list
- `xml_target_root`: AltaE2PACK
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\PL.md
- `file_name`: PL.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 3460.00 | CD | общий вес брутто | |
| 02 | NetWeightQuantity | 3200.00 | CD | общий вес нетто | |
| 03 | Consignor_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | грузоотправитель | |
| 04 | Consignor_ShortName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | краткое наименование | |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | |
| 07 | Consignor_Address_Region | HEBEI | CD | регион | |
| 08 | Consignor_Address_City | WUQIANG, HENGSHUI | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом одной строкой | |
| 10 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | copied_from:master_data.egrul.OrganizationName (alta\master_data.md) |
| 11 | Consignee_ShortName | ООО "СКИФ" | CD | краткое наименование | copied_from:master_data.egrul.ShortName (alta\master_data.md) |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | copied_from:master_data.egrul.OGRN (alta\master_data.md) |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | copied_from:master_data.egrul.INN (alta\master_data.md) |
| 14 | Consignee_KPP | 165001001 | CD | КПП | copied_from:master_data.egrul.KPP (alta\master_data.md) |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | copied_from:master_data.egrul.Address_PostalCode (alta\master_data.md) |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | copied_from:master_data.egrul.Address_CountryCode (alta\master_data.md) |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | copied_from:master_data.egrul.Address_CounryName (alta\master_data.md) |
| 18 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | copied_from:master_data.egrul.Address_Region (alta\master_data.md) |
| 19 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | copied_from:master_data.egrul.Address_City (alta\master_data.md) |
| 20 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис одной строкой | copied_from:master_data.egrul.Address_StreetHouse (alta\master_data.md) |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | внутренний числовой код условий | |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наименование контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 26HL-1103 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 31.03.2026 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | 26HL-1103 | CD | № упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 31.03.2026 | CD | дата упаковочного | |
| 33 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 34 | doc_code | 04131 | CD | код вида документа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | |
| 36 | doc_number | 26HL-1103 | CD | номер документа | |
| 37 | doc_date | 31.03.2026 | CD | дата документа | |

- _audit: 37
- `doc_status`: confirmed

#### formalized.packing_list Массив: Goods[14]
- _array_audit: 14

#### formalized.packing_list Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.4m x 30m Black / Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм 1.4м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 42 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 39 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Polyester Pleated Mesh 16x16 Mesh, 16mm 1.6m x 30m Black / Сетчатая ткань из полиэстера с плиссировкой Сетка 16x16, 16 мм 1.6м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 36.4 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 34 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | European Pleated Mesh 16mm 1.4m x 30m Black / Европейская плиссированная сетка 16 мм 1.4м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 39 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 36 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | European Pleated Mesh 16mm 1.6m x 30m Black / Европейская плиссированная сетка 16 мм 1.6м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 34.4 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 31.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 220g 1.4m x 30m Grey / Сетка от кошек 220 гр "Антикот" 1.4м х 30м. Серая | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест | |
| 03 | GrossWeightQuantity | 303 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 277 | CD | нетто по строке | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Grey / Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Серая | CD | описание строки | |
| 02 | GoodsQuantity | 50 | CD | количество мест | |
| 03 | GrossWeightQuantity | 800 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 768 | CD | нетто по строке | |
| 05 | PakingQuantity | 50 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.4 m x 30m Grey / Сетка от кошек 320 гр "Антикот" 1.4 м х 30м. Серая | CD | описание строки | |
| 02 | GoodsQuantity | 50 | CD | количество мест | |
| 03 | GrossWeightQuantity | 710 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 672 | CD | нетто по строке | |
| 05 | PakingQuantity | 50 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[8]
- _element_num: 8

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Black / Сетка от кошек 320 гр "Антикот" 1.6м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест | |
| 03 | GrossWeightQuantity | 480 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 461 | CD | нетто по строке | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[9]
- _element_num: 9

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.4m x 30m Black / Сетка от кошек 320 гр "Антикот" 1.4м х 30м. Черная | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест | |
| 03 | GrossWeightQuantity | 426 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 403 | CD | нетто по строке | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[10]
- _element_num: 10

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*50 M2.Black / Пылезащитная сетка 30 г "Антипыльца" Черная из полиэстера Размер рулона 1,6*50 М2 | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 45.46 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 33 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[11]
- _element_num: 11

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*30 M2.Black. Roll size: 1.6*100 m2 / Пылезащитная сетка 30 г "Антипыльца" Черная из полиэстера Размер рулона 1,6*30 М2 | CD | описание строки | |
| 02 | GoodsQuantity | 1 | CD | количество мест | |
| 03 | GrossWeightQuantity | 22.74 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 16.5 | CD | нетто по строке | |
| 05 | PakingQuantity | 5 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[12]
- _element_num: 12

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Mesh 18 Mesh 0.18mm материал SS304 / сетка 18, размер ячейки 0,18 мм материал SS304. Размер рулона 1,4*30 М2 | CD | описание строки | |
| 02 | GoodsQuantity | 1 | CD | количество мест | |
| 03 | GrossWeightQuantity | 51 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 43 | CD | нетто по строке | |
| 05 | PakingQuantity | 3 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[13]
- _element_num: 13

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Mesh 20 0.17 mm материал SS304 / сетка 17 мм материал SS304. Размер рулона 1,4*30 М2 | CD | описание строки | |
| 02 | GoodsQuantity | 1 | CD | количество мест | |
| 03 | GrossWeightQuantity | 165 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 146 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[14]
- _element_num: 14

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Anti polent Mesh 100g polyester Black / Антипыльца 100 г черная | CD | описание строки | |
| 02 | GoodsQuantity | 5 | CD | количество мест | |
| 03 | GrossWeightQuantity | 305 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 240 | CD | нетто по строке | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Массив: TransportMeans[2]
- _array_audit: 2

#### formalized.packing_list Элемент массива: TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | М869ОМ67 | CD | регистрационный номер | перевод в кириллицу |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 643 | CD | код национальности ТС | |
| 04 | MoverIndicator | true | CD | признак тягача | |

- _item_audit: 4

#### formalized.packing_list Элемент массива: TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | Number | АМ015667 | CD | регистрационный номер | перевод в кириллицу |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 643 | CD | код национальности ТС | |
| 04 | MoverIndicator | false | CD | признак тягача | |

- _item_audit: 4

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_1
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\currency_transfer_5_03.04.2026.md
- `file_name`: currency_transfer_5_03.04.2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 10 | CD | код способа платежа | |
| 03 | PaymentAmount | 72607.44 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN MESH. CONTRACT NO.: 26HL-1103, DATE: MARCH 31ST, 2026, INVOICE NO: 26HL-1103, DATE: 31.03.2026 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Семьдесят две тысячи шестьсот семь юаней 44/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 5 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 03.04.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | плательщик | copied_from:master_data.egrul.OrganizationName (alta\master_data.md) |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | copied_from:master_data.egrul.KPP (alta\master_data.md) |
| 13 | Payer_Bank_BankName | Филиал "Центральный" Банка ВТБ (ПАО), БИК 044525411, счет 40702156216150000051 | CD | реквизиты банка плательщика | |
| 14 | Payee_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SWIFT: VTBRCNSHXXX, CNAPS: 767290000018 | CD | реквизиты банка получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | |
| 17 | PersonName | Дмитрий | CD | имя подписанта | |
| 18 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 19 | doc_code | 04023 | CD | код вида документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 5 | CD | номер документа | |
| 22 | doc_date | 03.04.2026 | CD | дата документа | |

- _audit: 22
- `doc_status`: confirmed

### `document`: Payment Order
- `uqi_prefix`: formalized.payment_order_2
- `xml_target_root`: AltaPaymentOrder
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\currency_transfer_6_08.04.2026.md
- `file_name`: currency_transfer_6_08.04.2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 10 | CD | код способа платежа | |
| 03 | PaymentAmount | 15360.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN MESH. CONTRACT NO.: 26HL-1103, DATE: MARCH 31ST, 2026, INVOICE NO: 26HL-1103-A, DATE: 31.03.2026 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Пятнадцать тысяч триста шестьдесят юаней 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 6 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 08.04.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | плательщик | copied_from:master_data.egrul.OrganizationName (alta\master_data.md) |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CD | КПП плательщика | copied_from:master_data.egrul.KPP (alta\master_data.md) |
| 13 | Payer_Bank_BankName | Филиал "Центральный" Банка ВТБ (ПАО), БИК 044525411, счет 40702156216150000051 | CD | реквизиты банка плательщика | |
| 14 | Payee_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SWIFT: VTBRCNSHXXX, CNAPS: 767290000018 | CD | реквизиты банка получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | |
| 17 | PersonName | Дмитрий | CD | имя подписанта | |
| 18 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 19 | doc_code | 04023 | CD | код вида документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 6 | CD | номер документа | |
| 22 | doc_date | 08.04.2026 | CD | дата документа | |

- _audit: 22
- `doc_status`: confirmed

### `document`: Service Invoice
- `uqi_prefix`: formalized.service_invoice
- `xml_target_root`: AltaServiceInvoice
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\Счет_№26-09225-tl_от_12-05-2026.md
- `file_name`: Счет_№26-09225-tl_от_12-05-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CD | системный признак документа | |
| 02 | TotalServiceCost | 2610.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого | |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг | |
| 05 | BankName | АО "Альфа-Банк", БИК 044525593, счет 40702810001600010931 | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги | |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги | |
| 08 | PrDocumentNumber | 26-09225-tl | CD | номер связанного заказа | |
| 09 | PrDocumentDate | 07.04.2026 | CD | дата связанного заказа | |
| 10 | Registration_PrDocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | 26-09225-tl | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 12.05.2026 | CD | дата счета | |
| 13 | Consignor_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD | CD | грузоотправитель | copied_from:formalized.invoice_1.Seler_Name (alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103 .md) |
| 14 | PostalCode | 053300 | CO | индекс грузоотправителя | operator:service_invoice.PostalCode |
| 15 | CountryCode | CN | CD | страна грузоотправителя alpha-2 | copied_from:formalized.invoice_1.Seler_PostalAddress_CountryCode (alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103 .md) |
| 16 | CounryName | КИТАЙ | CD | страна грузоотправителя, текст | copied_from:formalized.invoice_1.Seler_PostalAddress_CounryName (alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103 .md) |
| 17 | Region | HEBEI | CD | регион грузоотправителя | copied_from:formalized.invoice_1.Seler_PostalAddress_Region (alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103 .md) |
| 18 | Town | WUQIANG, HENGSHUI | CD | город/район грузоотправителя | copied_from:formalized.invoice_1.Seler_PostalAddress_City (alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103 .md) |
| 19 | StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом грузоотправителя | copied_from:formalized.invoice_1.Seler_PostalAddress_StreetHouse (alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103 .md) |
| 20 | Consignee_OrganizationName | Общество с ограниченной ответственностью "СкиФ" | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CD | ОГРН грузополучателя | copied_from:master_data.egrul.OGRN (alta\master_data.md) |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН грузополучателя | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП грузополучателя | |
| 24 | PostalCode | 423800 | CD | индекс грузополучателя | copied_from:master_data.egrul.Address_PostalCode (alta\master_data.md) |
| 25 | CountryCode | RU | CD | страна грузополучателя alpha-2 | copied_from:master_data.egrul.Address_CountryCode (alta\master_data.md) |
| 26 | CounryName | РОССИЯ | CD | страна грузополучателя, текст | copied_from:master_data.egrul.Address_CounryName (alta\master_data.md) |
| 27 | Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион грузополучателя | copied_from:master_data.egrul.Address_Region (alta\master_data.md) |
| 28 | Town | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город грузополучателя | copied_from:master_data.egrul.Address_City (alta\master_data.md) |
| 29 | StreetHouse | ПРОЕЗД ХЛЕБНЫЙ | CD | улица грузополучателя | copied_from:master_data.egrul.Address_StreetHouse (alta\master_data.md) |
| 30 | House | Д. 30 | CD | дом грузополучателя | copied_from:master_data.egrul.Address_StreetHouse (alta\master_data.md) |
| 31 | Room | ОФИС 211 | CD | офис грузополучателя | copied_from:master_data.egrul.Address_StreetHouse (alta\master_data.md) |
| 32 | Signature_Choice | 2 | CD | вариант подписи | |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | |
| 34 | IndividualEntrepreneur_PersonName | | CD | имя ИП | |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CD | фамилия руководителя | |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Л. | CD | имя руководителя | |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | А. | CD | отчество руководителя | |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CD | фамилия бухгалтера | |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О. | CD | имя бухгалтера | |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | А. | CD | отчество бухгалтера | |
| 42 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 43 | doc_code | 04031 | CD | код вида документа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | |
| 45 | doc_number | 26-09225-tl | CD | номер документа | |
| 46 | doc_date | 12.05.2026 | CD | дата документа | |
| 47 | transport_to_border | 1358.00 | CD | стоимость до границы | |
| 48 | transport_currency | USD | CD | валюта стоимости | |

- _audit: 48
- `doc_status`: confirmed

#### formalized.service_invoice Массив: ServiceDescription[2]
- _array_audit: 2

#### formalized.service_invoice Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу №26-09225-tl от 07.04.2026 по маршруту: China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | Транспортно-экспедиционные услуги | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1358.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | |

- _item_audit: 7

#### formalized.service_invoice Элемент массива: ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, г. Набережные Челны | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | Транспортно-экспедиционные услуги | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1252.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | |

- _item_audit: 7

### `document`: Insurance Services Invoice
- `uqi_prefix`: formalized.insurance_invoice
- `xml_target_root`: AltaFreeDoc
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\Счет_№26-09225-tl_1_от_11-05-2026.md
- `file_name`: Счет_№26-09225-tl_1_от_11-05-2026.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 11.05.2026 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | 26-09225-tl/1 | CD | номер документа | |
| 05 | TextPara | Возмещение за добровольное страхование груза по договору №КООО/26651/М от 13-05-2025 по заявлению на страхование грузов 26-09225-tl от 11.05.2026 | CD | основной текст | |
| 06 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 07 | doc_code | 04111 | CD | код вида документа | |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | |
| 09 | doc_number | 26-09225-tl/1 | CD | номер документа | |
| 10 | doc_date | 11.05.2026 | CD | дата документа | |
| 11 | insurance_to_border | 798.04 | CD | стоимость страхования | |
| 12 | insurance_currency | RUB | CD | валюта страхования | |

- _audit: 12
- `doc_status`: confirmed

### `document`: Transit Declaration
- `uqi_prefix`: non_formalized.td
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\Транзитка 10719110_300526_5086483_reg.md
- `file_name`: Транзитка 10719110_300526_5086483_reg.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | customs_post_code | 10719110 | CD | код таможенного органа | |
| 02 | customs_post_name | МАПП Забайкальск | CD | наименование таможенного органа | |
| 03 | transport_reg_number | М869ОМ67/АМ015667 | CD | ТС по ТД | перевод в кириллицу |
| 04 | doc_gr44 | true | CD | признак включения в графу 44 | |
| 05 | doc_code | 09013 | CD | код вида документа | |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | наименование документа | |
| 07 | doc_number | 10719110/300526/5086483 | CD | номер документа | |
| 08 | doc_date | 30.05.2026 | CD | дата документа | |

- _audit: 8
- `doc_status`: confirmed

### `document`: Goods Description
- `uqi_prefix`: non_formalized.goods_description_1
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103 .md
- `file_name`: CL 26HL-1103 .md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | признак включения в графу 44 | |

- _audit: 1
- `doc_status`: confirmed

#### non_formalized.goods_description_1 Массив: goods[3]
- _array_audit: 3

#### non_formalized.goods_description_1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 6303921000 | CD | ТН ВЭД | |
| 02 | description | СЕТКА ПЛИССИРОВАННАЯ ИЗ СИНТЕТИЧЕСКИХ НИТЕЙ (100% ПОЛИЭСТЕР), ИЗ НЕТКАНЫХ МАТЕРИАЛОВ, ПРЕДНАЗНАЧЕНА ДЛЯ ИЗГОТОВЛЕНИЯ ВНУТРЕННИХ ШТОР И МОСКИТНЫХ СЕТОК. РАЗМЕР ЯЧЕЙКИ 16Х16, ШИРИНА 1.4М И 1.6М, ДЛИНА РУЛОНА 30М. ЦВЕТ ЧЕРНЫЙ. БЕЗ РИСУНКА. | CD | описание для графы 31 | |

- _item_audit: 2

#### non_formalized.goods_description_1 Элемент массива: goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 5804101000 | CD | ТН ВЭД | |
| 02 | description | СЕТКА МОСКИТНАЯ И ПЫЛЕЗАЩИТНАЯ (ТЮЛЬ И ПРОЧИЕ СЕТЧАТЫЕ ПОЛОТНА), ИЗ СИНТЕТИЧЕСКИХ НИТЕЙ (100% ПОЛИЭСТЕР), ОДНОЦВЕТНАЯ, БЕЗ УЗОРА, НЕ ТРИКОТАЖНАЯ, НЕ ТКАНАЯ. МОДЕЛИ: АНТИКОТ (220Г/320Г), АНТИПЫЛЬЦА (30Г/100Г). ПРЕДНАЗНАЧЕНА ДЛЯ ЗАЩИТЫ ОТ НАСЕКОМЫХ И ПЫЛИ. | CD | описание для графы 31 | |

- _item_audit: 2

#### non_formalized.goods_description_1 Элемент массива: goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 7314490000 | CD | ТН ВЭД | |
| 02 | description | СЕТКА МЕТАЛЛИЧЕСКАЯ ИЗ ПРОВОЛОКИ ИЗ КОРРОЗИОННОСТОЙКОЙ СТАЛИ (МАРКА SS304 / ЧЕРНЫЕ МЕТАЛЛЫ), НЕ СВАРНАЯ, НЕ ОЦИНКОВАННАЯ, БЕЗ ПОКРЫТИЯ. РАЗМЕР ЯЧЕЙКИ 0.17ММ И 0.18ММ. РАЗМЕР РУЛОНА 1.4МХ30М И 1.6МХ30М. ПРЕДНАЗНАЧЕНА ДЛЯ МОСКИТНЫХ РАМ. | CD | описание для графы 31 | |

- _item_audit: 2

### `document`: Goods Description
- `uqi_prefix`: non_formalized.goods_description_2
- `path`: alta\source\МоскитнаяСеткаWuqiang\md\CL 26HL-1103-A.md
- `file_name`: CL 26HL-1103-A.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | признак включения в графу 44 | |

- _audit: 1
- `doc_status`: confirmed

#### non_formalized.goods_description_2 Массив: goods[1]
- _array_audit: 1

#### non_formalized.goods_description_2 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 5804101000 | CD | ТН ВЭД | |
| 02 | description | СЕТКА МОСКИТНАЯ И ПЫЛЕЗАЩИТНАЯ (ТЮЛЬ И ПРОЧИЕ СЕТЧАТЫЕ ПОЛОТНА), ИЗ СИНТЕТИЧЕСКИХ НИТЕЙ (100% ПОЛИЭСТЕР), ОДНОЦВЕТНАЯ, БЕЗ УЗОРА, НЕ ТРИКОТАЖНАЯ, НЕ ТКАНАЯ. МОДЕЛИ: АНТИКОТ (220Г/320Г), АНТИПЫЛЬЦА (30Г/100Г). ПРЕДНАЗНАЧЕНА ДЛЯ ЗАЩИТЫ ОТ НАСЕКОМЫХ И ПЫЛИ. | CD | описание для графы 31 | |

- _item_audit: 2

### Итого, по файлу:

`total_unreliable_fields`: 0
`primary_status`: pending

## 5. Нерешенные вопросы (Issues)

**Для полей:**
- `formalized.invoice_1.CurrencyRate`
  - `question`: Курс валюты CNY отсутствует в первичных документах. Какое значение использовать?
- `formalized.invoice_2.CurrencyRate`
  - `question`: Курс валюты CNY отсутствует в первичных документах. Какое значение использовать?
- `formalized.service_invoice.PostalCode`
  - `question`: Почтовый индекс грузоотправителя (WUQIANG COUNTY HUILI FIBERGLASS CO., LTD) отсутствует в счете за перевозку и инвойсах. Какое значение использовать?

**Для общих вопросов:**
- `[Общий]`
  - `question`: Нет.

## 6. `unreliable_fields`:
Нет.
