# Первичные данные

## 1. meta:
- `название кейса`: Kyland
- `путь к папке поставки`: C:\Users\su144\RustroverProjects\rustdev\hobot\alta\source\Kyland\01
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 7 товаров
- `источники данных:` md + operator_provided_data.md + master_data.md

## 2. formalized/master_data/non_formalized:

### `document`: Contract
  - `uqi_prefix`: master_data.contract
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 03011 | CD | константа | |
| 03 | doc_name | КОНТРАКТ | CD | константа | |
| 04 | doc_number | Im191018/Kyl | CD | номер контракта | master_data.md |
| 05 | doc_date | 19.10.2018 | CD | дата контракта | master_data.md |

- _audit: 5

### `document`: Supplementary Contract
  - `uqi_prefix`: master_data.supplementary_contract_1
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 03012 | CD | константа | |
| 03 | doc_name | ДОПОЛНИТЕЛЬНОЕ СОГЛАШЕНИЕ | CD | константа | |
| 04 | doc_number | 221211 | CD | номер документа | master_data.md |
| 05 | doc_date | 11.12.2022 | CD | дата документа | master_data.md |

- _audit: 5

### `document`: UNK
  - `uqi_prefix`: master_data.unk
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 03031 | CD | константа | |
| 03 | doc_name | УНК | CD | константа | |
| 04 | doc_number | 18100214110000097211 | CD | номер документа | master_data.md |
| 05 | doc_date | 25.10.2018 | CD | дата документа | master_data.md |

- _audit: 5

### `document`: EGRUL
  - `uqi_prefix`: master_data.egrul
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СИМАНИТРОН" | CD | наименование организации | master_data.md |
| 02 | ShortName | ООО "СИМАНИТРОН" | CD | краткое наименование | master_data.md |
| 03 | OGRN | 1087746277740 | CD | ОГРН | master_data.md |
| 04 | INN | 7720609470 | CD | ИНН | master_data.md |
| 05 | KPP | 772001001 | CD | КПП | master_data.md |
| 06 | Address_PostalCode | 111675 | CD | индекс | master_data.md |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 08 | Address_CounryName | РОССИЯ | CD | страна, текст | master_data.md |
| 09 | Address_Region | ГОРОД МОСКВА | CD | регион | master_data.md |
| 10 | Address_City | МОСКВА | CD | город | master_data.md |
| 11 | Address_StreetHouse | УЛИЦА РУДНЁВКА, Д. 17, К. 103 | CD | улица/дом/офис | master_data.md |
| 12 | Phone | +7495 981-62-44 | CD | телефон | master_data.md |
| 13 | Email | info@symanitron.ru | CD | e-mail | master_data.md |
| 14 | doc_gr44 | true | CD | константа | |
| 15 | doc_code | 04011 | CD | константа | |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | константа | |
| 17 | doc_number | ЮЭ9965-19-16744108 | CD | номер документа | master_data.md |
| 18 | doc_date | 14.02.2019 | CD | дата документа | master_data.md |

- _audit: 18

### `document`: Personal Passport
  - `uqi_prefix`: master_data.passport
  - `path`: master_data.md
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
| 10 | doc_gr44 | true | CD | константа | |
| 11 | doc_code | 11001 | CD | константа | |
| 12 | doc_name | ПАСПОРТ | CD | константа | |
| 13 | doc_number | 63 09 449948 | CD | номер документа | master_data.md |
| 14 | doc_date | 11.03.2010 | CD | дата документа | master_data.md |

- _audit: 14

### `document`: Letter of Attorney
  - `uqi_prefix`: master_data.letter_of_attorney
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentNumber | 1 | CD | номер доверенности | master_data.md |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | master_data.md |
| 03 | EndDate | 31.12.2026 | CD | действительна до | master_data.md |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность | master_data.md |
| 05 | doc_gr44 | true | CD | константа | |
| 06 | doc_code | 11004 | CD | константа | |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | константа | |
| 08 | doc_number | 1 | CD | номер документа | master_data.md |
| 09 | doc_date | 01.02.2026 | CD | дата документа | master_data.md |

- _audit: 9

### `document`: Transport Contract
  - `uqi_prefix`: master_data.transport_contract
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 04033 | CD | константа | |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | константа | |
| 04 | doc_number | КООО/26651/М | CD | номер документа | master_data.md |
| 05 | doc_date | 13.05.2025 | CD | дата документа | master_data.md |

- _audit: 5

### `document`: Conformity Document
  - `uqi_prefix`: master_data.conformity_document_1
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 01402 | CD | константа | |
| 03 | doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | константа | |
| 04 | doc_number | ЕАЭС N RU Д-CN.РА01.В.65848/21 | CD | номер документа | master_data.md |
| 05 | doc_date | 14.05.2021 | CD | дата документа | master_data.md |
| 06 | date_start | 14.05.2021 | CD | дата начала действия | master_data.md |
| 07 | date_end | 12.05.2026 | CD | дата окончания действия | master_data.md |

- _audit: 7

### `document`: Conformity Document
  - `uqi_prefix`: master_data.conformity_document_2
  - `path`: master_data.md
  - `file_name`: master_data.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | true | CD | константа | |
| 02 | doc_code | 01402 | CD | константа | |
| 03 | doc_name | ДЕКЛАРАЦИЯ О СООТВЕТСТВИИ | CD | константа | |
| 04 | doc_number | ЕАЭС N RU Д-CN.РА03.В.59715/21 | CD | номер документа | master_data.md |
| 05 | doc_date | 17.12.2021 | CD | дата документа | master_data.md |
| 06 | date_start | 17.12.2021 | CD | дата начала действия | master_data.md |
| 07 | date_end | 15.12.2026 | CD | дата окончания действия | master_data.md |

- _audit: 7

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: md\CI, PL final_Invoice.md
  - `file_name`: CI, PL final.pdf

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | CurrencyCode | USD | CD | валюта инвойса | |
| 02 | DocumentCode | 04021 | CD | код вида документа | константа |
| 03 | PlacesQuantity | 6 | CD | кол-во грузовых мест | copied_from:formalized.awb.GoodsMovement_TotalPlacesQuantity |
| 04 | PlacesDescription | CTNS | CD | описание мест | |
| 05 | GrossWeightQuantity | 90 | CD | общий вес брутто | |
| 06 | NetWeightQuantity | 78 | CD | общий вес нетто | |
| 07 | GCost | 25255.00 | CD | системное поле | derived |
| 08 | TotalCost | 25255.00 | CD | итого по инвойсу | |
| 09 | DeliveryTerms_DeliveryPlace | Yichang | CD | место поставки | |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | EXW |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 15 | Registration_PrDocumentName | ИНВОЙС | CD | наименование документа | |
| 16 | Registration_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | номер инвойса | |
| 17 | Registration_PrDocumentDate | 12.12.2022 | CD | дата инвойса | подтверждено платежкой |
| 18 | Contract_PrDocumentNumber | Im191018/Kyl | CD | № контракта-ссылки | |
| 19 | Contract_PrDocumentDate | 19.10.2018 | CD | дата контракта-ссылки | |
| 20 | Buyer_CompanyID | 7720609470 | CD | ИНН покупателя | master_data.md |
| 21 | Buyer_KPPCode | 772001001 | CD | КПП покупателя | master_data.md |
| 22 | Buyer_Name | Symanitron LTD | CD | наименование покупателя | |
| 23 | Buyer_PostalAddress_PostalCode | 111675 | CD | почтовый индекс | |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя | |
| 25 | Buyer_PostalAddress_CounryName | RUSSIA | CD | страна покупателя, текст | |
| 26 | Buyer_PostalAddress_Region | MOSCOW | CD | регион | |
| 27 | Buyer_PostalAddress_City | MOSCOW | CD | город | |
| 28 | Buyer_PostalAddress_StreetHouse | Ap.103. 17 Rudnevka str. | CD | улица/дом/офис | |
| 29 | Seler_Name | Kyland Technology Co., Ltd. | CD | продавец | |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца | |
| 31 | Seler_PostalAddress_CounryName | CHINA | CD | страна продавца, текст | |
| 32 | Seler_PostalAddress_Region | Beijing | CD | регион продавца | |
| 33 | Seler_PostalAddress_City | Beijing | CD | город/район продавца | |
| 34 | Seler_PostalAddress_StreetHouse | Building No.2, Shixing Avenue 30# Shijingshan District | CD | улица/дом | |
| 35 | Consignor_OrganizationName | Kyland Technology Co., Ltd. | CD | грузоотправитель | нормализация: consignor=seller |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 37 | Consignor_Address_CounryName | CHINA | CD | страна грузоотправителя, текст | |
| 38 | Consignor_Address_Region | Beijing | CD | регион | |
| 39 | Consignor_Address_City | Beijing | CD | город/район | |
| 40 | Consignor_Address_StreetHouse | Building No.2, Shixing Avenue 30# Shijingshan District | CD | улица/дом | |
| 41 | Consignee_OrganizationName | Symanitron LTD | CD | грузополучатель | |
| 42 | Consignee_OGRN | 1087746277740 | CD | ОГРН | master_data.md |
| 43 | Consignee_INN | 7720609470 | CD | ИНН | |
| 44 | Consignee_KPP | 772001001 | CD | КПП | master_data.md |
| 45 | Consignee_Address_PostalCode | 117246 | CD | почтовый индекс | Ship-to address |
| 46 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | |
| 47 | Consignee_Address_CounryName | RUSSIAN FEDERATION | CD | страна, текст | |
| 48 | Consignee_Address_Region | Moscow | CD | регион | |
| 49 | Consignee_Address_City | Moscow | CD | город | |
| 50 | Consignee_Address_StreetHouse | Office 429, 1 building, 8 Nauchny proezd | CD | улица/дом/офис | |
| 51 | doc_gr44 | true | CD | константа | |
| 52 | doc_code | 04021 | CD | константа | |
| 53 | doc_name | ИНВОЙС | CD | константа | |
| 54 | doc_number | 1000059769 /60285 /60389 /60491 | CD | номер документа | |
| 55 | doc_date | 12.12.2022 | CD | дата документа | |

- _audit: 55

#### formalized.invoice_1 Массив: InvoiceGoods[7]
- _array_audit: 7

#### formalized.invoice_1 Элемент массива: InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator_provided_data.md |
| 02 | GoodsDescription | Промышленный управляемый коммутатор SYM3000A-4GX16GE-L2-L2 | CD | описание товара | |
| 03 | GoodsQuantity | 19 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | 19 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | шт | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | ШТУКА | CD | единица измерения | |
| 07 | GrossWeightQuantity | 44.240 | CD | брутто по строке | weight_rules: distributed |
| 08 | NetWeightQuantity | 38.340 | CD | нетто по строке | weight_rules: distributed |
| 09 | Price | 683.00 | CD | цена за единицу | |
| 10 | TotalCost | 12977.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | CN |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-4GX16GE-L2-L2 | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator_provided_data.md |
| 02 | GoodsDescription | Промышленный управляемый коммутатор SYM3000A-LITE-2GX8T-L3-L3 | CD | описание товара | |
| 03 | GoodsQuantity | 15 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | 15 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | шт | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | ШТУКА | CD | единица измерения | |
| 07 | GrossWeightQuantity | 15.442 | CD | брутто по строке | weight_rules: distributed |
| 08 | NetWeightQuantity | 13.383 | CD | нетто по строке | weight_rules: distributed |
| 09 | Price | 347.00 | CD | цена за единицу | |
| 10 | TotalCost | 5205.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | CN |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-LITE-2GX8T-L3-L3 | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator_provided_data.md |
| 02 | GoodsDescription | Промышленный управляемый коммутатор SYM3000A-4SFP8T-L2-L2 | CD | описание товара | |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | 5 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | шт | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | ШТУКА | CD | единица измерения | |
| 07 | GrossWeightQuantity | 9.191 | CD | брутто по строке | weight_rules: distributed |
| 08 | NetWeightQuantity | 7.966 | CD | нетто по строке | weight_rules: distributed |
| 09 | Price | 347.00 | CD | цена за единицу | |
| 10 | TotalCost | 1735.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | CN |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-4SFP8T-L2-L2 | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator_provided_data.md |
| 02 | GoodsDescription | Промышленный управляемый коммутатор SYM3000A-LITE-8T-L3-L3 | CD | описание товара | |
| 03 | GoodsQuantity | 1 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | 1 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | шт | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | ШТУКА | CD | единица измерения | |
| 07 | GrossWeightQuantity | 1.029 | CD | брутто по строке | weight_rules: distributed |
| 08 | NetWeightQuantity | 0.892 | CD | нетто по строке | weight_rules: distributed |
| 09 | Price | 215.00 | CD | цена за единицу | |
| 10 | TotalCost | 215.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | CN |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-LITE-8T-L3-L3 | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator_provided_data.md |
| 02 | GoodsDescription | Промышленный управляемый коммутатор SYM3000A-2GX16GE-L2-L2 | CD | описание товара | |
| 03 | GoodsQuantity | 1 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | 1 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | шт | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | ШТУКА | CD | единица измерения | |
| 07 | GrossWeightQuantity | 2.328 | CD | брутто по строке | weight_rules: distributed |
| 08 | NetWeightQuantity | 2.018 | CD | нетто по строке | weight_rules: distributed |
| 09 | Price | 604.00 | CD | цена за единицу | |
| 10 | TotalCost | 604.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | CN |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX16GE-L2-L2 | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 5 | CD | индекс позиции | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator_provided_data.md |
| 02 | GoodsDescription | Промышленный управляемый коммутатор SYM3000A-2GX8T-L2-L2 | CD | описание товара | |
| 03 | GoodsQuantity | 8 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | 8 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | шт | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | ШТУКА | CD | единица измерения | |
| 07 | GrossWeightQuantity | 10.785 | CD | брутто по строке | weight_rules: distributed |
| 08 | NetWeightQuantity | 9.347 | CD | нетто по строке | weight_rules: distributed |
| 09 | Price | 368.00 | CD | цена за единицу | |
| 10 | TotalCost | 2944.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | CN |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX8T-L2-L2 | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 6 | CD | индекс позиции | |

- _item_audit: 17

#### formalized.invoice_1 Элемент массива: InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsCode | 8517620003 | CO | код ТН ВЭД | operator_provided_data.md |
| 02 | GoodsDescription | Промышленный управляемый коммутатор SYM3000A-2GX16T-L2-L2 | CD | описание товара | |
| 03 | GoodsQuantity | 3 | CD | кол-во по строке | |
| 04 | goods_supplementary_quantity | 3 | CD | количество в доп.ед.изм | |
| 05 | goods_supplementary_uom_name | шт | CD | наименование доп.ед.изм | |
| 06 | MeasureUnitQualifierName | ШТУКА | CD | единица измерения | |
| 07 | GrossWeightQuantity | 6.985 | CD | брутто по строке | weight_rules: distributed |
| 08 | NetWeightQuantity | 6.054 | CD | нетто по строке | weight_rules: distributed |
| 09 | Price | 525.00 | CD | цена за единицу | |
| 10 | TotalCost | 1575.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CD | код страны происхождения | CN |
| 12 | AdditionalGoodsDescription_Manufacturer | Kyland Technology Co., Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | Kyland | CD | товарная марка | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | SYM3000A-2GX16T-L2-L2 | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 7 | CD | индекс позиции | |

- _item_audit: 17

### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list
  - `xml_target_root`: AltaE2PACK
  - `path`: md\CI, PL final_PackingList.md
  - `file_name`: CI, PL final.pdf

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GrossWeightQuantity | 90 | CD | общий вес брутто | |
| 02 | NetWeightQuantity | 78 | CD | общий вес нетто | |
| 03 | Consignor_OrganizationName | Kyland Technology Co., Ltd. | CD | грузоотправитель | |
| 04 | Consignor_ShortName | Kyland | CD | краткое наименование | |
| 05 | Consignor_Address_CountryCode | CN | CD | страна alpha-2 | |
| 06 | Consignor_Address_CounryName | CHINA | CD | страна, текст | |
| 07 | Consignor_Address_Region | Beijing | CD | регион | |
| 08 | Consignor_Address_City | Beijing | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | Building No.2, Shixing Avenue 30# Shijingshan District | CD | улица/дом | |
| 10 | Consignee_OrganizationName | Symanitron LTD | CD | грузополучатель | |
| 11 | Consignee_ShortName | Symanitron | CD | краткое наименование | |
| 12 | Consignee_OGRN | 1087746277740 | CD | ОГРН | master_data.md |
| 13 | Consignee_INN | 7720609470 | CD | ИНН | |
| 14 | Consignee_KPP | 772001001 | CD | КПП | master_data.md |
| 15 | Consignee_Address_PostalCode | 117246 | CD | почтовый индекс | |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | |
| 17 | Consignee_Address_CounryName | RUSSIAN FEDERATION | CD | страна, текст | |
| 18 | Consignee_Address_Region | Moscow | CD | регион | |
| 19 | Consignee_Address_City | Moscow | CD | город | |
| 20 | Consignee_Address_StreetHouse | Office 429, 1 building, 8 Nauchny proezd | CD | улица/дом/офис | |
| 21 | DeliveryTerms_DeliveryPlace | Yichang | CD | место поставки | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | EXW |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наименование контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | lm191018/Kyl | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 19.10.2018 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 23.12.2022 | CD | дата инвойса | дата из PL |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | 1000059769 /60285 /60389 /60491 | CD | № упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 23.12.2022 | CD | дата упаковочного | |
| 33 | doc_gr44 | true | CD | константа | |
| 34 | doc_code | 04131 | CD | константа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | константа | |
| 36 | doc_number | 1000059769 /60285 /60389 /60491 | CD | номер документа | |
| 37 | doc_date | 23.12.2022 | CD | дата документа | |

- _audit: 37

#### formalized.packing_list Массив: Goods[7]
- _array_audit: 7

#### formalized.packing_list Элемент массива: Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-4GX16GE-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 44.240 | CD | брутто по строке | weight_rules: distributed |
| 04 | NetWeightQuantity | 38.340 | CD | нетто по строке | weight_rules: distributed |
| 05 | PakingQuantity | 19 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-LITE-2GX8T-L3-L3 | CD | описание строки | |
| 02 | GoodsQuantity | | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 15.442 | CD | брутто по строке | weight_rules: distributed |
| 04 | NetWeightQuantity | 13.383 | CD | нетто по строке | weight_rules: distributed |
| 05 | PakingQuantity | 15 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-4SFP8T-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 9.191 | CD | брутто по строке | weight_rules: distributed |
| 04 | NetWeightQuantity | 7.966 | CD | нетто по строке | weight_rules: distributed |
| 05 | PakingQuantity | 5 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-LITE-8T-L3-L3 | CD | описание строки | |
| 02 | GoodsQuantity | | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 1.029 | CD | брутто по строке | weight_rules: distributed |
| 04 | NetWeightQuantity | 0.892 | CD | нетто по строке | weight_rules: distributed |
| 05 | PakingQuantity | 1 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX16GE-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 2.328 | CD | брутто по строке | weight_rules: distributed |
| 04 | NetWeightQuantity | 2.018 | CD | нетто по строке | weight_rules: distributed |
| 05 | PakingQuantity | 1 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX8T-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 10.785 | CD | брутто по строке | weight_rules: distributed |
| 04 | NetWeightQuantity | 9.347 | CD | нетто по строке | weight_rules: distributed |
| 05 | PakingQuantity | 8 | CD | кол-во упаковок | |

- _item_audit: 5

#### formalized.packing_list Элемент массива: Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | SYM3000A-2GX16T-L2-L2 | CD | описание строки | |
| 02 | GoodsQuantity | | CD | кол-во мест | |
| 03 | GrossWeightQuantity | 6.985 | CD | брутто по строке | weight_rules: distributed |
| 04 | NetWeightQuantity | 6.054 | CD | нетто по строке | weight_rules: distributed |
| 05 | PakingQuantity | 3 | CD | кол-во упаковок | |

- _item_audit: 5

### `document`: Air Waybill
  - `uqi_prefix`: formalized.awb
  - `xml_target_root`: AltaE3AWB
  - `path`: md\АвиаНакладная.md
  - `file_name`: АвиаНакладная.jpg

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | AgreedValuation | N.V.D | CD | объявленная ценность | |
| 02 | AgreedValuationCurrencyCode | CNY | CD | код валюты | |
| 03 | Registration_AirlineIATACode | 876 | CD | IATA-код авиакомпании | |
| 04 | Registration_DocumentNumber | 41176586 | CD | номер авианакладной | |
| 05 | Registration_DateInf | 2023-01-02 | CD | дата выпуска | |
| 06 | Consignor_NameInf | KYLAND TECHNOLOGY CO., LTD. | CD | грузоотправитель | |
| 07 | Consignor_ShortName | KYLAND | CD | краткое наименование | |
| 08 | Consignor_PostalAddress_CountryCode | CN | CD | код страны отправителя | |
| 09 | Consignor_Address_CounryName | CHINA | CD | страна отправителя, текст | |
| 10 | Consignor_Address_City | BEIJING | CD | город отправителя | |
| 11 | Consignor_Address_StreetHouse | BUILDING NO.2, SHIXING AVENUE 30# | CD | улица, дом отправителя | |
| 12 | Consignee_NameInf | SYMANITRON LTD | CD | грузополучатель | |
| 13 | Consignee_ShortName | SYMANITRON | CD | краткое наименование | |
| 14 | Consignee_OGRNID | 1087746277740 | CD | ОГРН получателя | master_data.md |
| 15 | Consignee_INNID | 7720609470 | CD | ИНН получателя | |
| 16 | Consignee_KPPCode | 772001001 | CD | КПП получателя | master_data.md |
| 17 | Consignee_PostalAddress_PostalCode | 111675 | CD | индекс получателя | |
| 18 | Consignee_PostalAddres_CountryCode | RU | CD | код страны получателя | |
| 19 | Consignee_Address_CounryName | RUSSIA | CD | страна получателя, текст | |
| 20 | Consignee_Address_Region | MOSCOW | CD | регион получателя | |
| 21 | Consignee_Address_City | MOSCOW | CD | город получателя | |
| 22 | Consignee_Address_StreetHouse | AP.103. 17 RUDNEVKA STR. | CD | улица, дом, офис получателя | |
| 23 | GoodsMovement | | CD | сведения о движении | |
| 24 | HandlingInfo | | CD | информация по обработке | |
| 25 | IssueInfo_OrganizationName | CHINA SICHUAN AIRLINES | CD | авиакомпания | |
| 26 | IssueInfo_Address_CountryCode | CN | CD | код страны авиакомпании | |
| 27 | IssueInfo_Address_CounryName | CHINA | CD | страна авиакомпании, текст | |
| 28 | AWBGoodsInfo_TotalPlacesQuantity | 6 | CD | общее количество мест | |
| 29 | AWBGoodsInfo_WeightUnitQualifierCode | K | CD | код единицы веса | |
| 30 | AWBGoodsInfo_GrossWeightQuantity | 90 | CD | общий вес брутто | |
| 31 | doc_gr44 | true | CD | константа | |
| 32 | doc_code | 02017 | CD | константа | |
| 33 | doc_name | АВИАНАКЛАДНАЯ | CD | константа | |
| 34 | doc_number | 876-41176586 | CD | номер в формате IATA-Doc | |
| 35 | doc_date | 2023-01-02 | CD | дата документа | |

- _audit: 35

#### formalized.awb Массив: AWBGoodsInfo_AWBGoods[1]
- _array_audit: 1

#### formalized.awb Элемент массива: AWBGoodsInfo_AWBGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | PlacesQuantity | 6 | CD | количество мест | |
| 02 | WeightUnitQualifierCode | K | CD | единица измерения веса | |
| 03 | GrossWeightQuantity | 90 | CD | вес брутто позиции | |
| 04 | CommodityItemNum | 1 | CD | порядковый номер | |
| 05 | GoodsCommodityCode | 8517620003 | CO | код ТН ВЭД | operator_provided_data.md |
| 06 | FactPlacesQuantity | 6 | CD | фактическое кол-во мест | |
| 07 | GoodsDescription | INDUSTRIAL ETHERNET SWITCH | CD | наименование и описание | |

- _item_audit: 7

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: md\ПЛАТЕЖКА.md
  - `file_name`: ПЛАТЕЖКА.pdf

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04023 | CD | код вида документа | константа |
| 02 | PaymentModeCode | 1 | CD | код способа платежа | |
| 03 | PaymentAmount | 25255.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | константа |
| 05 | Priority | . | CD | очередность | константа |
| 06 | Purpose | SALE AND PURCHASE CONTRACT Im191018/Kyl date 19/10/18 Invoice 1000059769 /60285 /60389 /60491 date 12/12/22 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Двадцать пять тысяч двести пятьдесят пять долларов США 00 центов | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 30 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 15.12.2022 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | SYMANITRON | CD | плательщик | |
| 11 | Payer_INN | 7720609470 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 772001001 | CD | КПП плательщика | master_data.md |
| 13 | Payer_Bank_BankName | АО ЮниКредит Банк, счет 06037583USDCOCA101 | CD | банк плательщика | |
| 14 | Payee_OrganizationName | Kyland Technology Co., Ltd., счет 77012025000038811 | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | BANK OF NINGBO, SWIFT BKNBCN2NBEI, BEIJING CHINA | CD | банк получателя | |
| 16 | PersonSurname | | CD | фамилия | |
| 17 | PersonName | | CD | имя | |
| 18 | doc_gr44 | true | CD | константа | |
| 19 | doc_code | 04023 | CD | константа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | константа | |
| 21 | doc_number | 30 | CD | номер документа | |
| 22 | doc_date | 15.12.2022 | CD | дата документа | |

- _audit: 22

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice
  - `xml_target_root`: AltaServiceInvoice
  - `path`: md\Счет на оплату № VIG2227802 от 28.12.2022.md
  - `file_name`: Счет на оплату № VIG2227802 от 28.12.2022.pdf

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentSign | 1 | CD | системный признак | константа |
| 02 | TotalServiceCost | 1615.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого | |
| 04 | ServiceProvider_Name | ООО "ВиАйДжи Кастомс" | CD | исполнитель услуг | |
| 05 | BankName | СМОЛЕНСКОЕ ОТДЕЛЕНИЕ N8609 ПАО СБЕРБАНК | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | 279 | CD | № договора на услуги | |
| 07 | ContractDetails_PrDocumentDate | 16.07.2019 | CD | дата договора на услуги | |
| 08 | PrDocumentNumber | BDE000168 | CD | номер заказа | |
| 09 | PrDocumentDate | | CD | дата заказа | |
| 10 | Registration_PrDocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | VIG2227802 | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 28.12.2022 | CD | дата счета | |
| 13 | Consignor_OrganizationName | ООО "ВиАйДжи Кастомс" | CD | грузоотправитель | |
| 14 | PostalCode | 121087 | CD | почтовый индекс | |
| 15 | CountryCode | RU | CD | страна alpha-2 | |
| 16 | CounryName | РОССИЯ | CD | страна, текст | |
| 17 | Region | МОСКВА Г | CD | регион | |
| 18 | Town | МОСКВА Г | CD | город/район | |
| 19 | StreetHouse | БАРКЛАЯ УЛ, ДОМ 6, СТРОЕНИЕ 5, ОФИС 22, ЭТАЖ 3 | CD | улица/дом | |
| 20 | Consignee_OrganizationName | ООО "СИМАНИТРОН" | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1087746277740 | CD | ОГРН | master_data.md |
| 22 | Consignee_RFOrganizationFeatures_INN | 7720609470 | CD | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 772001001 | CD | КПП | master_data.md |
| 24 | PostalCode | 111675 | CD | почтовый индекс | |
| 25 | CountryCode | RU | CD | страна alpha-2 | |
| 26 | CounryName | РОССИЯ | CD | страна, текст | |
| 27 | Region | МОСКВА Г | CD | регион | |
| 28 | Town | МОСКВА Г | CD | город | |
| 29 | StreetHouse | РУДНЁВКА УЛ | CD | улица | |
| 30 | House | 17 | CD | дом | |
| 31 | Room | 103 | CD | офис/кв | |
| 32 | Signature_Choice | 2 | CD | вариант подписи | |
| 33 | IndividualEntrepreneur_PersonSurname | | CD | фамилия ИП | |
| 34 | IndividualEntrepreneur_PersonName | | CD | имя ИП | |
| 35 | IndividualEntrepreneur_PersonMiddleName | | CD | отчество ИП | |
| 36 | SignatureDirectorChiefAccountant_Director_PersonSurname | МИНАКОВА | CD | фамилия руководителя | |
| 37 | SignatureDirectorChiefAccountant_Director_PersonName | Н. | CD | имя руководителя | |
| 38 | SignatureDirectorChiefAccountant_Director_PersonMiddleName | В. | CD | отчество руководителя | |
| 39 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | ЕМЕЦ | CD | фамилия бухгалтера | |
| 40 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | М. | CD | имя бухгалтера | |
| 41 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonMiddleName | В. | CD | отчество бухгалтера | |
| 42 | doc_gr44 | true | CD | константа | |
| 43 | doc_code | 04031 | CD | константа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | константа | |
| 45 | doc_number | VIG2227802 | CD | номер документа | |
| 46 | doc_date | 28.12.2022 | CD | дата документа | |
| 47 | transport_to_border | 1615.00 | CD | до границы | |
| 48 | transport_currency | USD | CD | валюта | |

- _audit: 48

#### formalized.service_invoice Массив: ServiceDescription[1]
- _array_audit: 1

#### formalized.service_invoice Элемент массива: ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | GoodsDescription | Организация перевозки груза по маршруту: Йичанг, КИТАЙ - Аэропорт Шереметьево - Авианакладная № 876-41176586 | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | Йичанг - Шереметьево | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1615.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | |

- _item_audit: 7

### `document`: Insurance Invoice
  - `uqi_prefix`: formalized.insurance_invoice
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\Счет на оплату № 27611 от 27 декабря 2022 г.md
  - `file_name`: Счет на оплату № 27611 от 27 декабря 2022 г.pdf

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 04111 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | СЧЕТ НА ОПЛАТУ | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 27.12.2022 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | VIG2227611 | CD | номер документа | |
| 05 | TextPara | link:md\Счет на оплату № 27611 от 27 декабря 2022 г.md | CD | текст документа | |
| 06 | doc_gr44 | true | CD | константа | |
| 07 | doc_code | 04111 | CD | константа | |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | константа | |
| 09 | doc_number | VIG2227611 | CD | номер документа | |
| 10 | doc_date | 27.12.2022 | CD | дата документа | |
| 11 | insurance_to_border | 5186.02 | CD | стоимость страхования | |
| 12 | insurance_currency | RUB | CD | валюта страхования | |

- _audit: 12

### `document`: Tech Description
  - `uqi_prefix`: formalized.tech_description
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\TechDescription.md
  - `file_name`: TechDescription.md

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | DocumentCode | 05999 | CD | код вида документа | константа |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКОЕ ОПИСАНИЕ ГРУЗА ОБОРУДОВАНИЕ KYLAND | CD | наименование | |
| 03 | DocumentHead_DocumentDate | 12.12.2022 | CD | дата техописания | из инвойса |
| 04 | DocumentHead_DocumentNumber | БН | CD | номер техописания | константа |
| 05 | TextPara | link:md\TechDescription.md | CD | ссылка на источник | |
| 06 | doc_gr44 | true | CD | константа | |
| 07 | doc_code | 05999 | CD | константа | |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | константа | |
| 09 | doc_number | БН | CD | номер документа | |
| 10 | doc_date | 12.12.2022 | CD | дата документа | |

- _audit: 10

### `document`: Goods Description
  - `uqi_prefix`: non_formalized.goods_description_1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | doc_gr44 | false | CD | константа | |

- _audit: 1

#### non_formalized.goods_description_1 Массив: goods[1]
- _array_audit: 1

#### non_formalized.goods_description_1 Элемент массива: goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | tn_ved | 8517620003 | CO | ТН ВЭД | |
| 02 | description | ПРОМЫШЛЕННЫЕ УПРАВЛЯЕМЫЕ КОММУТАТОРЫ KYLAND СЕРИИ SYM3000A (МОДЕЛИ: SYM3000A-4GX16GE-L2-L2, SYM3000A-LITE-2GX8T-L3-L3, SYM3000A-4SFP8T-L2-L2, SYM3000A-LITE-8T-L3-L3, SYM3000A-2GX16GE-L2-L2, SYM3000A-2GX8T-L2-L2, SYM3000A-2GX16T-L2-L2). ПРЕДНАЗНАЧЕНЫ ДЛЯ ПЕРЕДАЧИ ДАННЫХ В ПРОМЫШЛЕННЫХ СЕТЯХ ETHERNET. КОРПУС ИЗ МЕТАЛЛА, БЕЗ ВЕНТИЛЯТОРА, КРЕПЛЕНИЕ НА DIN-РЕЙКУ. ПОДДЕРЖИВАЮТ ПРОТОКОЛЫ DT-RING, VLAN, SNMP. ПИТАНИЕ 24VDC. НЕ ЯВЛЯЮТСЯ СРЕДСТВАМИ ПОЖАРНОЙ АВТОМАТИКИ. НЕ ДЛЯ ШИФРОВАНИЯ. | CD | описание товара | |

- _item_audit: 2

### `document`: Storage Report
  - `uqi_prefix`: non_formalized.svh

| num | field | value | status | description | note |
|---|---|---|---|---|---|
| 01 | warehouse_license_number | 10005/060917/10048/2 | CO | номер лицензии СВХ | operator_provided_data.md |
| 02 | actual_gross_weight | 90 | CD | фактический вес | из AWB |
| 03 | actual_places | 6 | CD | фактическое кол-во мест | из AWB |
| 04 | transport_reg_number | | CD | номер ТС | |
| 05 | doc_gr44 | false | CD | константа | |

- _audit: 5

### Итогo, по файлу:

`total_unreliable_fields`: 0
`primary_status`: confirmed
**Для общих вопросов:**
- `[Общий]`
  - `question`: Дата инвойса в Packing List (23.12.2022) отличается от даты в самом Инвойсе (12.12.2022). Использована дата 12.12.2022.
- `[Веса]`
  - `question`: Веса нетто в Packing List (например, 28.5 кг для 19 шт) физически меньше веса «голого» оборудования по тех. описаниям (36.1 кг). Данные PL признаны недостоверными, применены расчетные веса по weight_rules для всех документов.
