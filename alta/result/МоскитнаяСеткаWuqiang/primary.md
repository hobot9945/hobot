# Первичные данные

## 1. meta:
- `название кейса`: МоскитнаяСеткаWuqiang
- `путь к папке поставки`: alta\source\МоскитнаяСеткаWuqiang\01\
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 14 товаров (по инвойсам)
- `источники данных:` md + master_data.md

## 2. master_data:

### `document`: Contract
  - `uqi_prefix`: master_data.contract
  - `path`: alta\source\МоскитнаяСеткаWuqiang\master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 02 | doc_code | 03011 | CD | код вида документа | |
| 03 | doc_name | КОНТРАКТ | CD | наименование документа | |
| 04 | doc_number | 26HL-1103 | CD | номер документа | |
| 05 | doc_date | 31.03.2026 | CD | дата документа | |
_audit: 5
- `doc_status`: confirmed

### `document`: EGRUL
  - `uqi_prefix`: master_data.egrul
  - `path`: alta\source\МоскитнаяСеткаWuqiang\master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | наименование организации | |
| 02 | ShortName | ООО "СКИФ" | CD | краткое наименование | |
| 03 | OGRN | 1201600020390 | CD | ОГРН | |
| 04 | INN | 1650389298 | CD | ИНН | |
| 05 | KPP | 165001001 | CD | КПП | |
| 06 | Address_PostalCode | 423800 | CD | индекс | |
| 07 | Address_CountryCode | RU | CD | страна alpha-2 | |
| 08 | Address_CounryName | РОССИЯ | CD | страна текст | |
| 09 | Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 10 | Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 11 | Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | |
| 12 | Phone | +7 (843) 207 18 90 | CD | телефон | |
| 13 | Email | PROM_TAT@MAIL.RU | CD | e-mail | |
| 14 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 15 | doc_code | 04011 | CD | код вида документа | |
| 16 | doc_name | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование документа | |
| 17 | doc_number | ЮЭ9965-25-106893283 | CD | номер документа | |
| 18 | doc_date | 14.07.2025 | CD | дата документа | |
_audit: 18
- `doc_status`: confirmed

### `document`: Personal Passport
  - `uqi_prefix`: master_data.passport
  - `path`: alta\source\МоскитнаяСеткаWuqiang\master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | PersonSurname | АРБУЗОВА | CD | фамилия | |
| 02 | PersonName | АНАСТАСИЯ | CD | имя | |
| 03 | PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество | |
| 04 | CardSeries | 63 09 | CD | серия паспорта | |
| 05 | CardNumber | 449948 | CD | номер паспорта | |
| 06 | CardDate | 11.03.2010 | CD | дата выдачи | |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан | |
| 08 | Phone | +7 927-222-0500 | CD | телефон | |
| 09 | Email | A.K.ARBUZOVA@YANDEX.RU | CD | e-mail | |
| 10 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 11 | doc_code | 11001 | CD | код вида документа | |
| 12 | doc_name | ПАСПОРТ | CD | наименование документа | |
| 13 | doc_number | 63 09 449948 | CD | номер документа | |
| 14 | doc_date | 11.03.2010 | CD | дата документа | |
_audit: 14
- `doc_status`: confirmed

### `document`: Letter of Attorney
  - `uqi_prefix`: master_data.letter_of_attorney
  - `path`: alta\source\МоскитнаяСеткаWuqiang\master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentNumber | 1 | CD | номер доверенности | |
| 02 | DocumentDate | 01.02.2026 | CD | дата доверенности | |
| 03 | EndDate | 31.12.2026 | CD | действительна до | |
| 04 | EmpoweredPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | должность | |
| 05 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 06 | doc_code | 11004 | CD | код вида документа | |
| 07 | doc_name | ДОВЕРЕННОСТЬ | CD | наименование документа | |
| 08 | doc_number | 1 | CD | номер документа | |
| 09 | doc_date | 01.02.2026 | CD | дата документа | |
_audit: 9
- `doc_status`: confirmed

### `document`: Transport Contract
  - `uqi_prefix`: master_data.transport_contract
  - `path`: alta\source\МоскитнаяСеткаWuqiang\master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 02 | doc_code | 04033 | CD | код вида документа | |
| 03 | doc_name | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование документа | |
| 04 | doc_number | КООО/26651/М | CD | номер документа | |
| 05 | doc_date | 13.05.2025 | CD | дата документа | |
_audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter
  - `uqi_prefix`: master_data.exemption_letter
  - `path`: alta\source\МоскитнаяСеткаWuqiang\master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 02 | doc_code | 09023 | CD | код вида документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | |
| 05 | doc_date | 20.08.2025 | CD | дата документа | |
_audit: 5
- `doc_status`: confirmed

### `document`: Exemption Letter (source)
  - `uqi_prefix`: master_data.exemption_letter_source
  - `path`: alta\source\МоскитнаяСеткаWuqiang\master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 02 | doc_code | 09999 | CD | код вида документа | |
| 03 | doc_name | ОТКАЗНОЕ ПИСЬМО | CD | наименование документа | |
| 04 | doc_number | 24968/МЛ10 | CD | номер документа | |
| 05 | doc_date | 20.08.2025 | CD | дата документа | |
_audit: 5
- `doc_status`: confirmed

### `document`: UNK
  - `uqi_prefix`: master_data.unk
  - `path`: alta\source\МоскитнаяСеткаWuqiang\master_data.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 02 | doc_code | 03031 | CD | код вида документа | |
| 03 | doc_name | УНК | CD | наименование документа | |
| 04 | doc_number | | pending | номер документа | |
| 05 | doc_date | | pending | дата документа | |
_audit: 5
- `doc_status`: pending

## 3. formalized:

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\CL 26HL-1103.md
  - `file_name`: CL 26HL-1103.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | CurrencyCode | CNY | CD | валюта инвойса | |
| 02 | DocumentCode | 04021 | CD | код вида документа | |
| 03 | PlacesQuantity | 201 | CD | кол-во грузовых мест | |
| 04 | PlacesDescription | pcs | CD | описание мест | |
| 05 | GrossWeightQuantity | 3460 | CD | общий вес брутто | copied_from:formalized.cmr.CMRGoodsWeight_GrossWeightQuantity |
| 06 | NetWeightQuantity | 3200 | CD | общий вес нетто | copied_from:formalized.packing_list.NetWeightQuantity |
| 07 | GCost | 72607.44 | CD | системное поле | |
| 08 | TotalCost | 72607.44 | CD | итого по инвойсу | |
| 09 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 15 | Registration_PrDocumentName | Commercial invoice | CD | наименование документа | |
| 16 | Registration_PrDocumentNumber | 26HL-1103 | CD | номер инвойса | |
| 17 | Registration_PrDocumentDate | 31.03.2026 | CD | дата инвойса | |
| 18 | Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта-ссылки | |
| 19 | Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта-ссылки | |
| 20 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | |
| 21 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | |
| 22 | Buyer_Name | LLC «SKIF» | CD | наименование покупателя | |
| 23 | Buyer_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | |
| 25 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя текст | |
| 26 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 27 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 28 | Buyer_PostalAddress_StreetHouse | KHLEBNY PASSAGE, HAUSE 30, OFFICE 211 | CD | улица/дом/офис | |
| 29 | Seler_Name | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | продавец | |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца alpha-2 | |
| 31 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца текст | |
| 32 | Seler_PostalAddress_Region | HEBEI | CD | регион продавца | |
| 33 | Seler_PostalAddress_City | WUQIANG, HENGSHUI | CD | город/район продавца | |
| 34 | Seler_PostalAddress_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом продавца | |
| 35 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | нормализация: consignor=seller |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 37 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя текст | |
| 38 | Consignor_Address_Region | HEBEI | CD | регион | |
| 39 | Consignor_Address_City | WUQIANG, HENGSHUI | CD | город/район | |
| 40 | Consignor_Address_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом | |
| 41 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОТВЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 42 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 43 | Consignee_INN | 1650389298 | CD | ИНН | master_data.md |
| 44 | Consignee_KPP | 165001001 | CD | КПП | master_data.md |
| 45 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | master_data.md |
| 46 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 47 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | master_data.md |
| 48 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data.md |
| 49 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data.md |
| 50 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_data.md |
| 51 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 52 | doc_code | 04021 | CD | код вида документа | |
| 53 | doc_name | ИНВОЙС | CD | наименование документа | |
| 54 | doc_number | 26HL-1103 | CD | номер документа | |
| 55 | doc_date | 31.03.2026 | CD | дата документа | |
_audit: 55

#### InvoiceGoods Массив: InvoiceGoods[13]
- _array_audit: 13

#### formalized.invoice_1.InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Polyester Pleated Mesh 1.4m x 30m Black | CD | описание товара | |
| 03 | GoodsQuantity | 10 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 420 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 42 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 39 | CD | нетто | |
| 09 | Price | 2.30 | CD | цена | |
| 10 | TotalCost | 966.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Polyester Pleated Mesh 1.4m x 30m | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Polyester Pleated Mesh 1.6m x 30m Black | CD | описание товара | |
| 03 | GoodsQuantity | 10 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 480 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 36.4 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 34 | CD | нетто | |
| 09 | Price | 2.30 | CD | цена | |
| 10 | TotalCost | 1104.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Polyester Pleated Mesh 1.6m x 30m | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | European Pleated Mesh 1.4m x 30m Black | CD | описание товара | |
| 03 | GoodsQuantity | 10 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 420 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 39 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 36 | CD | нетто | |
| 09 | Price | 2.35 | CD | цена | |
| 10 | TotalCost | 987.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | European Pleated Mesh 1.4m x 30m | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 6303921000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | European Pleated Mesh 1.6m x 30m Black | CD | описание товара | |
| 03 | GoodsQuantity | 10 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 480 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 34.4 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 31.5 | CD | нетто | |
| 09 | Price | 2.35 | CD | цена | |
| 10 | TotalCost | 1128.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | European Pleated Mesh 1.6m x 30m | CD | модель | |
| 16 | dt_item_index | 1 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 220g 1.4m x 30m Grey | CD | описание товара | |
| 03 | GoodsQuantity | 30 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 1260 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 303 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 277 | CD | нетто | |
| 09 | Price | 4.70 | CD | цена | |
| 10 | TotalCost | 5922.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 220g 1.4m x 30m | CD | модель | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Grey | CD | описание товара | |
| 03 | GoodsQuantity | 50 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 2400 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 800 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 768 | CD | нетто | |
| 09 | Price | 5.80 | CD | цена | |
| 10 | TotalCost | 13920.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.6m x 30m | CD | модель | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.4 m x 30m Grey | CD | описание товара | |
| 03 | GoodsQuantity | 50 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 2100 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 710 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 672 | CD | нетто | |
| 09 | Price | 5.80 | CD | цена | |
| 10 | TotalCost | 12180.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.4 m x 30m | CD | модель | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 3 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[8]
- _element_num: 8

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Black | CD | описание товара | |
| 03 | GoodsQuantity | 30 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 1440 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 480 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 461 | CD | нетто | |
| 09 | Price | 5.80 | CD | цена | |
| 10 | TotalCost | 8352.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.6m x 30m | CD | модель | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 4 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[9]
- _element_num: 9

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti-Cat Mesh 320g 1.4m x 30m Black | CD | описание товара | |
| 03 | GoodsQuantity | 30 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 1260 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 426 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 403 | CD | нетто | |
| 09 | Price | 5.80 | CD | цена | |
| 10 | TotalCost | 7308.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti-Cat Mesh 320g 1.4m x 30m | CD | модель | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 5 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[10]
- _element_num: 10

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti Dust Mesh 30g 1,6*50 M2 Black | CD | описание товара | |
| 03 | GoodsQuantity | 10 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 800 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 45.46 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 33 | CD | нетто | |
| 09 | Price | 9.42 | CD | цена | |
| 10 | TotalCost | 7536.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti Dust Mesh 30g 1,6*50 M2 | CD | модель | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 6 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[11]
- _element_num: 11

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti Dust Mesh 30g 1,6*30 M2 Black | CD | описание товара | |
| 03 | GoodsQuantity | 5 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 800 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 22.74 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 16.5 | CD | нетто | |
| 09 | Price | 9.42 | CD | цена | |
| 10 | TotalCost | 7536.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti Dust Mesh 30g 1,6*30 M2 | CD | модель | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 7 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[12]
- _element_num: 12

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7314490000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Mesh 18 Mesh 0.18mm SS304 | CD | описание товара | |
| 03 | GoodsQuantity | 3 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 144 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 51 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 43 | CD | нетто | |
| 09 | Price | 7.457338 | CD | цена | |
| 10 | TotalCost | 1073.86 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Mesh 18 Mesh 0.18mm SS304 | CD | модель | |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 1 | CD | индекс позиции | |
- _item_audit: 17

#### formalized.invoice_1.InvoiceGoods[13]
- _element_num: 13

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7314490000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Mesh 20 0.17 mm SS304 | CD | описание товара | |
| 03 | GoodsQuantity | 10 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 420 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 165 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 146 | CD | нетто | |
| 09 | Price | 10.939484 | CD | цена | |
| 10 | TotalCost | 4594.58 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Mesh 20 0.17 mm SS304 | CD | модель | |
| 16 | dt_item_index | 3 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 2 | CD | индекс позиции | |
- _item_audit: 17

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_2
  - `xml_target_root`: AltaE2I
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\CL 26HL-1103-A.md
  - `file_name`: CL 26HL-1103-A.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | CurrencyCode | CNY | CD | валюта инвойса | |
| 02 | DocumentCode | 04021 | CD | код вида документа | |
| 03 | PlacesQuantity | 5 | CD | кол-во грузовых мест | |
| 04 | PlacesDescription | pcs | CD | описание мест | |
| 05 | GrossWeightQuantity | 305 | CD | общий вес брутто | copied_from:спецификация.md |
| 06 | NetWeightQuantity | 240 | CD | общий вес нетто | copied_from:спецификация.md |
| 07 | GCost | 15360.00 | CD | системное поле | |
| 08 | TotalCost | 15360.00 | CD | итого по инвойсу | |
| 09 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | |
| 10 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | |
| 11 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 12 | DeliveryTerms_DispatchCountryCode | CN | CD | страна отправления | |
| 13 | DeliveryTerms_TradingCountryCode | CN | CD | торгующая страна | |
| 14 | DeliveryTerms_DestinationCountryCode | RU | CD | страна назначения | |
| 15 | Registration_PrDocumentName | Commercial invoice | CD | наименование документа | |
| 16 | Registration_PrDocumentNumber | 26HL-1103-A | CD | номер инвойса | |
| 17 | Registration_PrDocumentDate | 31.03.2026 | CD | дата инвойса | |
| 18 | Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта-ссылки | |
| 19 | Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта-ссылки | |
| 20 | Buyer_CompanyID | 1650389298 | CD | ИНН покупателя | |
| 21 | Buyer_KPPCode | 165001001 | CD | КПП покупателя | |
| 22 | Buyer_Name | LLC «SKIF» | CD | наименование покупателя | |
| 23 | Buyer_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | |
| 24 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | |
| 25 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя текст | |
| 26 | Buyer_PostalAddress_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 27 | Buyer_PostalAddress_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 28 | Buyer_PostalAddress_StreetHouse | KHLEBNY PASSAGE, HAUSE 30, OFFICE 211 | CD | улица/дом/офис | |
| 29 | Seler_Name | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | продавец | |
| 30 | Seler_PostalAddress_CountryCode | CN | CD | страна продавца alpha-2 | |
| 31 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца текст | |
| 32 | Seler_PostalAddress_Region | HEBEI | CD | регион продавца | |
| 33 | Seler_PostalAddress_City | WUQIANG, HENGSHUI | CD | город/район продавца | |
| 34 | Seler_PostalAddress_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом продавца | |
| 35 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | нормализация: consignor=seller |
| 36 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя | |
| 37 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя текст | |
| 38 | Consignor_Address_Region | HEBEI | CD | регион | |
| 39 | Consignor_Address_City | WUQIANG, HENGSHUI | CD | город/район | |
| 40 | Consignor_Address_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом | |
| 41 | Consignee_OrganizationName | ОБЩЕСТВО С ОГРАНИЧЕННОЙ ОБЕТСТВЕННОСТЬЮ "СКИФ" | CD | грузополучатель | master_data.md |
| 42 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 43 | Consignee_INN | 1650389298 | CD | ИНН | master_data.md |
| 44 | Consignee_KPP | 165001001 | CD | КПП | master_data.md |
| 45 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | master_data.md |
| 46 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | master_data.md |
| 47 | Consignee_Address_CounryName | РОССИЯ | CD | страна текст | master_data.md |
| 48 | Consignee_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | master_data.md |
| 49 | Consignee_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | master_data.md |
| 50 | Consignee_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, Д. 30, ОФИС 211 | CD | улица/дом/офис | master_data.md |
| 51 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 52 | doc_code | 04021 | CD | код вида документа | |
| 53 | doc_name | ИНВОЙС | CD | наименование документа | |
| 54 | doc_number | 26HL-1103-A | CD | номер документа | |
| 55 | doc_date | 31.03.2026 | CD | дата документа | |
_audit: 55
- `doc_status`: confirmed

#### InvoiceGoods Массив: InvoiceGoods[1]
- _array_audit: 1

#### formalized.invoice_2.InvoiceGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД | |
| 02 | GoodsDescription | Anti polent Mesh 100g Black | CD | описание товара | |
| 03 | GoodsQuantity | 50 | CD | кол-во в осн.ед. | |
| 04 | goods_supplementary_quantity | 2400 | CD | кол-во в доп.ед. | |
| 05 | goods_supplementary_uom_name | M2 | CD | доп.ед.изм код | |
| 06 | MeasureUnitQualifierName | Кв.м | CD | доп.ед.изм текст | |
| 07 | GrossWeightQuantity | 305 | CD | брутто | copied_from:спецификация.md |
| 08 | NetWeightQuantity | 240 | CD | нетто | |
| 09 | Price | 6.40 | CD | цена | |
| 10 | TotalCost | 15360.00 | CD | стоимость | |
| 11 | OriginCountryCode | CN | CD | страна происхождения | |
| 12 | AdditionalGoodsDescription_Manufacturer | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | производитель | |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CD | ТМ | |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CD | маркировка | |
| 15 | AdditionalGoodsDescription_GoodsModel | Anti polent Mesh 100g | CD | модель | |
| 16 | dt_item_index | 2 | CD | индекс товара ДТ | |
| 17 | dt_tovg_index | 8 | CD | индекс позиции | |
- _item_audit: 17

### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list
  - `xml_target_root`: AltaE2PACK
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\PL.md
  - `file_name`: PL.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GrossWeightQuantity | 3460 | CD | общий вес брутто | |
| 02 | NetWeightQuantity | 3200 | CD | общий вес нетто | |
| 03 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | |
| 04 | Consignor_ShortName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | краткое наименование | |
| 05 | Consignor_Address_CountryCode | CN | CD | страна грузоотправителя alpha-2 | |
| 06 | Consignor_Address_CounryName | China | CD | страна грузоотправителя текст | |
| 07 | Consignor_Address_Region | Hebei | CD | регион | |
| 08 | Consignor_Address_City | Wuqiang, Hengshui | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | Haozhuang Industrial Zone | CD | улица/дом | |
| 10 | Consignee_OrganizationName | LLC «SKIF» | CD | грузополучатель | |
| 11 | Consignee_ShortName | ООО "СКиФ" | CD | краткое наименование | |
| 12 | Consignee_OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 13 | Consignee_INN | 1650389298 | CD | ИНН | master_data.md |
| 14 | Consignee_KPP | 165001001 | CD | КПП | master_data.md |
| 15 | Consignee_Address_PostalCode | 423800 | CD | почтовый индекс | |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | |
| 17 | Consignee_Address_CounryName | Russia | CD | страна текст | |
| 18 | Consignee_Address_Region | Republic of Tatarstan | CD | регион | |
| 19 | Consignee_Address_City | Naberezhnye Chelny | CD | город | |
| 20 | Consignee_Address_StreetHouse | Khlebny Passage, hause 30, office 211 | CD | улица/дом/офис | |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CD | числовой код условий | |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | строковый код условий | |
| 24 | DeliveryTerms_Contract_PrDocumentName | КОНТРАКТ | CD | наименование контракта | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | 26HL-1103 | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 31.03.2026 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | ИНВОЙС | CD | наименование инвойса | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | 26HL-1103 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 31.03.2026 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | | pending | № упаковочного | |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 31.03.2026 | CD | дата упаковочного | |
| 33 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 34 | doc_code | 04131 | CD | код вида документа | |
| 35 | doc_name | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование документа | |
| 36 | doc_number | | pending | номер документа | |
| 37 | doc_date | 31.03.2026 | CD | дата документа | |
_audit: 37
- `doc_status`: pending

#### Goods Массив: Goods[14]
- _array_audit: 14

#### formalized.packing_list.Goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Polyester Pleated Mesh 1.4m x 30m Black | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 42 | CD | брутто | |
| 04 | NetWeightQuantity | 39 | CD | нетто | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Polyester Pleated Mesh 1.6m x 30m Black | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 36.4 | CD | брутто | |
| 04 | NetWeightQuantity | 34 | CD | нетто | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | European Pleated Mesh 1.4m x 30m Black | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 39 | CD | брутто | |
| 04 | NetWeightQuantity | 36 | CD | нетто | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[4]
- _element_num: 4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | European Pleated Mesh 1.6m x 30m. Black | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 34.4 | CD | брутто | |
| 04 | NetWeightQuantity | 31.5 | CD | нетто | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[5]
- _element_num: 5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-Cat Mesh 220g 1.4m x 30m Grey | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест | |
| 03 | GrossWeightQuantity | 303 | CD | брутто | |
| 04 | NetWeightQuantity | 277 | CD | нетто | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[6]
- _element_num: 6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Grey | CD | описание строки | |
| 02 | GoodsQuantity | 50 | CD | количество мест | |
| 03 | GrossWeightQuantity | 800 | CD | брутто | |
| 04 | NetWeightQuantity | 768 | CD | нетто | |
| 05 | PakingQuantity | 50 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[7]
- _element_num: 7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.4m x 30m Grey | CD | описание строки | |
| 02 | GoodsQuantity | 50 | CD | количество мест | |
| 03 | GrossWeightQuantity | 710 | CD | брутто | |
| 04 | NetWeightQuantity | 672 | CD | нетто | |
| 05 | PakingQuantity | 50 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[8]
- _element_num: 8

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.6m x 30m Black | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест | |
| 03 | GrossWeightQuantity | 480 | CD | брутто | |
| 04 | NetWeightQuantity | 461 | CD | нетто | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[9]
- _element_num: 9

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-Cat Mesh 320g 1.4m x 30m Black | CD | описание строки | |
| 02 | GoodsQuantity | 30 | CD | количество мест | |
| 03 | GrossWeightQuantity | 426 | CD | брутто | |
| 04 | NetWeightQuantity | 403 | CD | нетто | |
| 05 | PakingQuantity | 30 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[10]
- _element_num: 10

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*50 M2.Black | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 45.46 | CD | брутто | |
| 04 | NetWeightQuantity | 33 | CD | нетто | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[11]
- _element_num: 11

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti Dust Mesh 30g polyester Roll size 1,6*30 M2.Black | CD | описание строки | |
| 02 | GoodsQuantity | 2 | CD | количество мест | |
| 03 | GrossWeightQuantity | 22.74 | CD | брутто | |
| 04 | NetWeightQuantity | 16.5 | CD | нетто | |
| 05 | PakingQuantity | 5 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[12]
- _element_num: 12

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Mesh 18 Mesh 0.18mm SS304 | CD | описание строки | |
| 02 | GoodsQuantity | 1 | CD | количество мест | |
| 03 | GrossWeightQuantity | 51 | CD | брутто | |
| 04 | NetWeightQuantity | 43 | CD | нетто | |
| 05 | PakingQuantity | 3 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[13]
- _element_num: 13

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Mesh 20 0.17 mm SS304 | CD | описание строки | |
| 02 | GoodsQuantity | 1 | CD | количество мест | |
| 03 | GrossWeightQuantity | 165 | CD | брутто | |
| 04 | NetWeightQuantity | 146 | CD | нетто | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |
- _item_audit: 5

#### formalized.packing_list.Goods[14]
- _element_num: 14

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti polent Mesh 100g polyester Black | CD | описание строки | |
| 02 | GoodsQuantity | 5 | CD | количество мест | |
| 03 | GrossWeightQuantity | 305 | CD | брутто | |
| 04 | NetWeightQuantity | 240 | CD | нетто | |
| 05 | PakingQuantity | 10 | CD | кол-во упаковок | |
- _item_audit: 5

#### TransportMeans Массив: TransportMeans[2]
- _array_audit: 2

#### formalized.packing_list.TransportMeans[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | Number | M869OM67 | CD | гос. номер тягача | copied_from:formalized.cmr.CMRTransport_PrimeMoverStateSignID |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 000 | CD | код национальности | |
| 04 | MoverIndicator | true | CD | признак тягача | |
- _item_audit: 4

#### formalized.packing_list.TransportMeans[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | Number | AM015667 | CD | гос. номер прицепа | copied_from:formalized.cmr.CMRTransport_TrailerStateSignID |
| 02 | ModeCode | 31 | CD | код вида транспорта | |
| 03 | NationalityCode | 000 | CD | код национальности | |
| 04 | MoverIndicator | false | CD | признак тягача | |
- _item_audit: 4

### `document`: CMR
  - `uqi_prefix`: formalized.cmr
  - `xml_target_root`: AltaE3CMR
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\СМР.md
  - `file_name`: СМР.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | LanguageCode | RU | CD | язык документа | |
| 02 | CMR_Choice | 1 | CD | системный выбор | |
| 03 | RegistrationDocument_RegID | 09225 | CD | номер CMR | |
| 04 | RegistrationDocument_DateInf | 27.05.2026 | CD | дата CMR | |
| 05 | RegistrationDocument_Place | МАНЬЧЖУРИЯ | CD | место составления | |
| 06 | TrakingCargo_TakingCargoDate | 27.05.2026 | CD | дата принятия груза | |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CD | страна принятия груза alpha-2 | |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза текст | |
| 09 | DeliveryPlace_CountryCode | RU | CD | страна доставки alpha-2 | |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки текст | |
| 11 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryPlace |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CD | условия поставки | copied_from:formalized.invoice_1.DeliveryTerms_DeliveryTermsStringCode |
| 13 | GoodsQuantity | 206 | CD | общее число мест | |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3460 | CD | общий вес брутто | |
| 15 | CMRTransport_PrimeMoverStateSignID | M869OM67 | CD | гос. номер тягача | |
| 16 | CMRTransport_TrailerStateSignID | AM015667 | CD | гос. номер прицепа | |
| 17 | Consignor_NameInf | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD. | CD | отправитель | |
| 18 | Consignor_ShortName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD. | CD | краткое наименование | |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна alpha-2 | |
| 20 | Consignor_Address_CounryName | CHINA | CD | страна текст | |
| 21 | Consignor_Address_Region | HEBEI | CD | регион | |
| 22 | Consignor_Address_City | WUQIANG, HENGSHUI | CD | город/район | |
| 23 | Consignor_Address_StreetHouse | HAOZHUANG INDUSTRIAL ZONE | CD | улица/дом | |
| 24 | Consignor_Guarantee_OrganizationName | | pending | наименование гаранта | |
| 25 | Consignor_Guarantee_ShortName | | pending | краткое наименование | |
| 26 | Consignor_Guarantee_Address_CountryCode | | pending | страна alpha-2 | |
| 27 | Consignor_Guarantee_Address_CounryName | | pending | страна текст | |
| 28 | Consignor_Guarantee_Address_Region | | pending | регион | |
| 29 | Consignor_Guarantee_Address_City | | pending | город/район | |
| 30 | Consignor_Guarantee_Address_StreetHouse | | pending | улица/дом | |
| 31 | Consignee_NameInf | LLC «SKIF» | CD | получатель | |
| 32 | Consignee_ShortName | ООО "СКиФ" | CD | краткое наименование | |
| 33 | Consignee_OGRNID | 1201600020390 | CD | ОГРН | master_data.md |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | master_data.md |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | master_data.md |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | почтовый индекс | |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | |
| 38 | Consignee_Address_CounryName | REPUBLIC OF TATARSTAN, RUSSIA | CD | страна текст | |
| 39 | Consignee_Address_Region | REPUBLIC OF TATARSTAN | CD | регион | |
| 40 | Consignee_Address_City | NABEREZHNYE CHELNY | CD | город | |
| 41 | Consignee_Address_StreetHouse | KHLEBNY PASSAGE, HAUSE 30, OFFICE 211 | CD | улица/дом/офис | |
| 42 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 43 | doc_code | 02015 | CD | код вида документа | |
| 44 | doc_name | CMR | CD | наименование документа | |
| 45 | doc_number | 09225 | CD | номер документа | |
| 46 | doc_date | 27.05.2026 | CD | дата документа | |
_audit: 46
- `doc_status`: pending

#### CMRGoods Массив: CMRGoods[1]
- _array_audit: 1

#### formalized.cmr.CMRGoods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsNumeric | 1 | CD | порядковый номер | авто-нумерация единственной строки |
| 02 | GoodsDescription | ТОВАР ЗАГРУЖЕН, СОГЛАСНО СПЕЦИФИКАЦИИ К INVOICE № 26HL-1103-A ОТ 31.03.2026; INVOICE № 26HL-1103 ОТ 31.03.2026 | CD | описание груза | |
| 03 | PakingQuantity | 206 | CD | кол-во упаковок | |
- _item_audit: 3

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\currency_transfer_5_03.04.2026.md
  - `file_name`: currency_transfer_5_03.04.2026.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 1 | CD | способ платежа | |
| 03 | PaymentAmount | 72607.44 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN MESH. CONTRACT NO.: 26HL-1103, DATE: MARCH 31ST, 2026, INVOICE NO: 26HL-1103, DATE: 31.03.2026 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Семьдесят две тысячи шестьсот семь юаней 44/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 5 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 03.04.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | | pending | КПП плательщика | |
| 13 | Payer_Bank_BankName | ВТБ, счет 40702156216150000051 | CD | банк плательщика | |
| 14 | Payee_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD, счет 40807156200610025308 | CD | получатель | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | |
| 17 | PersonName | Дмитрий | CD | имя подписанта | |
| 18 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 19 | doc_code | 04023 | CD | код вида документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 5 | CD | номер документа | |
| 22 | doc_date | 03.04.2026 | CD | дата документа | |
_audit: 22
- `doc_status`: pending

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\currency_transfer_6_08.04.2026.md
  - `file_name`: currency_transfer_6_08.04.2026.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CD | код вида документа | |
| 02 | PaymentModeCode | 1 | CD | способ платежа | |
| 03 | PaymentAmount | 15360.00 | CD | сумма платежа | |
| 04 | TransactionKind | 01 | CD | вид операции | |
| 05 | Priority | . | CD | очередность | |
| 06 | Purpose | PURCHASE OF AN MESH. CONTRACT NO.: 26HL-1103, DATE: MARCH 31ST, 2026, INVOICE NO: 26HL-1103-A, DATE: 31.03.2026 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Пятнадцать тысяч триста шестьдесят юаней 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 6 | CD | номер платежного поручения | |
| 09 | DocumentReference_PrDocumentDate | 08.04.2026 | CD | дата платежного поручения | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | | pending | КПП плательщика | |
| 13 | Payer_Bank_BankName | ВТБ, счет 40702156216150000051 | CD | банк плательщика | |
| 14 | Payee_OrganizationName | WUQIANG COUNTY HUILI FIBERGLASS CO., LTD, счет 40807156200610025308 | CD | получатель | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CD | фамилия подписанта | |
| 17 | PersonName | Дмитрий | CD | имя подписанта | |
| 18 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 19 | doc_code | 04023 | CD | код вида документа | |
| 20 | doc_name | ПЛАТЕЖНОЕ ПОРУЧЕНИЕ | CD | наименование документа | |
| 21 | doc_number | 6 | CD | номер документа | |
| 22 | doc_date | 08.04.2026 | CD | дата документа | |
_audit: 22
- `doc_status`: pending

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice
  - `xml_target_root`: AltaServiceInvoice
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\Счет_№26-09225-tl_от_12-05-2026.md
  - `file_name`: Счет_№26-09225-tl_от_12-05-2026.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentSign | 1 | CD | системный признак | |
| 02 | TotalServiceCost | 2610.00 | CD | итого по услугам | |
| 03 | Currency | USD | CD | валюта итого | |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг | |
| 05 | BankName | АО "Альфа-Банк", БИК 044525593, Сч. № 40702810001600010931 | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | №КООО/26651/М | CD | № договора на услуги | |
| 07 | ContractDetails_PrDocumentDate | 13-05-2025 | CD | дата договора на услуги | |
| 08 | PrDocumentNumber | №26-09225-tl | CD | номер связанного заказа | |
| 09 | PrDocumentDate | 07.04.2026 | CD | дата связанного заказа | |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета | |
| 11 | Registration_PrDocumentNumber | 26-09225-tl | CD | номер счета | |
| 12 | Registration_PrDocumentDate | 12.05.2026 | CD | дата счета | |
| 13 | Consignor_OrganizationName | Wuqiang County Huili Fiberglass Co.,Ltd. | CD | грузоотправитель | copied_from:formalized.invoice_1.Seler_Name |
| 14 | PostalCode | | CD | почтовый индекс | |
| 15 | CountryCode | CN | CD | страна alpha-2 | |
| 16 | CounryName | China | CD | страна текст | |
| 17 | Region | Hebei | CD | регион | |
| 18 | Town | Wuqiang, Hengshui | CD | город/район | |
| 19 | StreetHouse | Haozhuang Industrial Zone | CD | улица/дом | |
| 20 | Consignee_OrganizationName | ООО "СКиФ" | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CD | ОГРН | master_data.md |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | |
| 24 | PostalCode | 423800 | CD | почтовый индекс | |
| 25 | CountryCode | RU | CD | страна alpha-2 | |
| 26 | CounryName | Россия | CD | страна текст | |
| 27 | Region | Республика Татарстан | CD | регион | |
| 28 | Town | г. Набережные Челны | CD | город | |
| 29 | StreetHouse | проезд Хлебный | CD | улица | |
| 30 | House | д. 30 | CD | дом | |
| 31 | Room | кв/оф. 211 | CD | офис/кв | |
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
| 42 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 43 | doc_code | 04031 | CD | код вида документа | |
| 44 | doc_name | СЧЕТ ЗА ПЕРЕВОЗКУ | CD | наименование документа | |
| 45 | doc_number | 26-09225-tl | CD | номер документа | |
| 46 | doc_date | 12.05.2026 | CD | дата документа | |
| 47 | transport_to_border | 1358.00 | CD | транспорт до границы | |
| 48 | transport_currency | USD | CD | валюта транспорта | |
_audit: 48
- `doc_status`: pending

#### ServiceDescription Массив: ServiceDescription[2]
- _array_audit: 2

#### formalized.service_invoice.ServiceDescription[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу №26-09225-tl от 07.04.2026 по маршруту: China, Ningbo - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | China, Ningbo - граница РФ | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1358.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | |
- _item_audit: 7

#### formalized.service_invoice.ServiceDescription[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, г. Набережные Челны | CD | описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки | |
| 03 | ServiceName | граница РФ - г. Набережные Челны | CD | наименование/маршрут | |
| 04 | TaxRate | 0 | CD | ставка налога | |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1252.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости | |
- _item_audit: 7

### `document`: Insurance Invoice
  - `uqi_prefix`: formalized.insurance_invoice
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\Счет_№26-09225-tl_1_от_11-05-2026.md
  - `file_name`: Счет_№26-09225-tl_1_от_11-05-2026.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04111 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | Счет на оплату | CD | наименование документа | |
| 03 | DocumentHead_DocumentDate | 11.05.2026 | CD | дата документа | |
| 04 | DocumentHead_DocumentNumber | 26-09225-tl/1 | CD | номер документа | |
| 05 | TextPara | link:alta\source\МоскитнаяСеткаWuqiang\01\md\Счет_№26-09225-tl_1_от_11-05-2026.md | CD | ссылка на текст | |
| 06 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 07 | doc_code | 04111 | CD | код вида документа | |
| 08 | doc_name | СЧЕТ ЗА СТРАХОВКУ | CD | наименование документа | |
| 09 | doc_number | 26-09225-tl/1 | CD | номер документа | |
| 10 | doc_date | 11.05.2026 | CD | дата документа | |
| 11 | insurance_to_border | 798.04 | CD | страховка до границы | |
| 12 | insurance_currency | RUB | CD | валюта страховки | |
_audit: 12
- `doc_status`: confirmed

### `document`: Tech Description
  - `uqi_prefix`: formalized.tech_description
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\техничка Антикот, антипыльца нержавейка плесе  .md
  - `file_name`: техничка Антикот, антипыльца нержавейка плесе  .pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 05999 | CD | код вида документа | |
| 02 | DocumentHead_DocumentName | Технические характеристики | CD | наименование техописания | |
| 03 | DocumentHead_DocumentDate | 31.03.2026 | CD | дата техописания | |
| 04 | DocumentHead_DocumentNumber | 31032026 | CD | номер техописания | |
| 05 | TextPara | link:alta\source\МоскитнаяСеткаWuqiang\01\md\техничка Антикот, антипыльца нержавейка плесе  .md | CD | ссылка на текст | |
| 06 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 07 | doc_code | 05999 | CD | код вида документа | |
| 08 | doc_name | ТЕХНИЧЕСКОЕ ОПИСАНИЕ | CD | наименование документа | |
| 09 | doc_number | 31032026 | CD | номер документа | |
| 10 | doc_date | 31.03.2026 | CD | дата документа | |
_audit: 10
- `doc_status`: confirmed

## 4. non_formalized:

### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.td
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\Транзитка 10719110_300526_5086483_reg.md
  - `file_name`: Транзитка 10719110_300526_5086483_reg.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | customs_post_code | 10719110 | CD | код таможенного органа | |
| 02 | customs_post_name | МАПП Забайкальск | CD | наименование таможни | |
| 03 | transport_reg_number | M869OM67/AM015667 | CD | ТС по ТД | |
| 04 | doc_gr44 | true | CD | признак включения в гр.44 | |
| 05 | doc_code | 09013 | CD | код вида документа | |
| 06 | doc_name | ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ | CD | наименование документа | |
| 07 | doc_number | 10719110/300526/5086483 | CD | номер документа | |
| 08 | doc_date | 30.05.2026 | CD | дата документа | |
_audit: 8
- `doc_status`: confirmed

### `document`: Storage Report
  - `uqi_prefix`: non_formalized.svh
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\ДО_ОТЧЕТ.md
  - `file_name`: ДО.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии СВХ | |
| 02 | warehouse_license_date | 18.09.2025 | CD | дата лицензии СВХ | |
| 03 | actual_gross_weight | 3460 | CD | фактический вес брутто | copied_from:ДО_ДОБАВОЧНЫЙ_ЛИСТ.md |
| 04 | actual_places | 206 | CD | фактическое количество мест | copied_from:ДО_ДОБАВОЧНЫЙ_ЛИСТ.md |
| 05 | transport_reg_number | М 869 ОМ 67 | CD | номер ТС при въезде | |
| 06 | doc_gr44 | false | CD | признак включения в гр.44 | |
_audit: 6
- `doc_status`: confirmed

#### goods Массив: goods[2]
- _array_audit: 2

#### non_formalized.svh.goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tnved | 5804101000 | CD | код товара | |
| 02 | places | 197 | CD | кол-во мест по строке | |
| 03 | gross_weight_kg | 3092.2 | CD | вес брутто по строке | |
| 04 | cost | 78114 | CD | стоимость по строке | |
| 05 | currency_code | CNY | CD | код валюты | |
- _item_audit: 5

#### non_formalized.svh.goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tnved | 6303921000 | CD | код товара | |
| 02 | places | 8 | CD | кол-во мест по строке | |
| 03 | gross_weight_kg | 151.8 | CD | вес брутто по строке | |
| 04 | cost | 4185 | CD | стоимость по строке | |
| 05 | currency_code | CNY | CD | код валюты | |
- _item_audit: 5

### `document`: Storage Report Additional Sheet
  - `uqi_prefix`: non_formalized.svh_additional_sheet_1
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\ДО_ДОБАВОЧНЫЙ_ЛИСТ.md
  - `file_name`: ДО.pdf

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | number | 1 | CD | № доп.листа | |
| 02 | date | 05.06.2026 | CD | дата доп.листа | |
| 03 | actual_gross_weight | 3460 | CD | фактический вес брутто | |
| 04 | actual_places | 206 | CD | фактическое количество мест | |
| 05 | transport_reg_number | М 869 ОМ 67 | CD | номер ТС при въезде | copied_from:non_formalized.svh.transport_reg_number |
| 06 | svh_address_region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион СВХ | copied_from:formalized.cmr.Consignee_Address_Region |
| 07 | svh_address_city | Г. НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город СВХ | copied_from:formalized.cmr.Consignee_Address_City |
| 08 | svh_address_street_house | ПРОИЗВОДСТВЕННЫЙ ПР-Д, Д. 45 | CD | улица/дом СВХ | copied_from:formalized.cmr.Consignee_Address_StreetHouse |
| 09 | svh_customs_code | 10404083 | CD | код таможни СВХ | copied_from:formalized.cmr.SenderInstructions |
| 10 | doc_gr44 | false | CD | признак включения в гр.44 | |
_audit: 10
- `doc_status`: pending

### `document`: Goods Description
  - `uqi_prefix`: non_formalized.goods_description_1
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\CL 26HL-1103.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | false | CD | признак включения в гр.44 | |
_audit: 1
- `doc_status`: confirmed

#### goods Массив: goods[3]
- _array_audit: 3

#### non_formalized.goods_description_1.goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tn_ved | 6303921000 | CD | ТН ВЭД | |
| 02 | description | Сетка москитная плиссированная (внутренние шторы) из синтетических нитей (полиэстер), из нетканых материалов, однотонная, без узора. Предназначена для защиты помещений от насекомых, устанавливается на окна и двери. Размеры: 1.4м х 30м, 1.6м х 30м. | CD | описание товара | |
- _item_audit: 2

#### non_formalized.goods_description_1.goods[2]
- _element_num: 2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tn_ved | 5804101000 | CD | ТН ВЭД | |
| 02 | description | Сетка москитная (сетчатое полотно) из полиэстера, одноцветная, без узора, не трикотажная, не тканая. Используется для защиты от насекомых, пыли и пыльцы (модели 'Антикот', 'Антипыльца'). Размеры рулонов: 1.4м х 30м, 1.6м х 30м, 1.6м х 50м. | CD | описание товара | |
- _item_audit: 2

#### non_formalized.goods_description_1.goods[3]
- _element_num: 3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tn_ved | 7314490000 | CD | ТН ВЭД | |
| 02 | description | Сетка москитная из нержавеющей стали (черные металлы), тканая, полотняного переплетения, из проволоки диаметром 0.17-0.18 мм, размер ячейки 0.8-1.2 мм. Предназначена для защиты от насекомых и мелких грызунов. Размеры рулонов: 1.4м х 30м. | CD | описание товара | |
- _item_audit: 2

### `document`: Goods Description
  - `uqi_prefix`: non_formalized.goods_description_2
  - `path`: alta\source\МоскитнаяСеткаWuqiang\01\md\CL 26HL-1103-A.md

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | doc_gr44 | false | CD | признак включения в гр.44 | |
_audit: 1
- `doc_status`: confirmed

#### goods Массив: goods[1]
- _array_audit: 1

#### non_formalized.goods_description_2.goods[1]
- _element_num: 1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tn_ved | 5804101000 | CD | ТН ВЭД | |
| 02 | description | Сетка москитная (сетчатое полотно) из полиэстера, одноцветная, без узора, не трикотажная, не тканая. Используется для защиты от насекомых, пыли и пыльцы (модели 'Антикот', 'Антипыльца'). Размеры рулонов: 1.4м х 30м, 1.6м х 30м, 1.6м х 50м. | CD | описание товара | |
- _item_audit: 2

### Итого, по файлу:

`total_unreliable_fields`: 0
`primary_status`: pending

## 5. Нерешенные вопросы (Issues)

**Для полей:**
- `master_data.unk.doc_number`
  - `question`: Уникальный номер контракта (УНК) отсутствует в master_data.md. Требуется подтверждение оператора.
- `formalized.packing_list.DeliveryTerms_Registration_PrDocumentNumber`
  - `question`: Номер упаковочного листа отсутствует в PL.pdf. Требуется подтверждение оператора.
- `non_formalized.svh_additional_sheet_1.transport_reg_number`
  - `question`: Номер ТС при въезде отсутствует в добавочном листе ДО. Требуется подтверждение оператора.
- `non_formalized.svh_additional_sheet_1.svh_address_region`
  - `question`: Регион СВХ отсутствует в добавочном листе ДО. Требуется подтверждение оператора.
- `non_formalized.svh_additional_sheet_1.svh_address_city`
  - `question`: Город СВХ отсутствует в добавочном листе ДО. Требуется подтверждение оператора.
- `non_formalized.svh_additional_sheet_1.svh_address_street_house`
  - `question`: Улица/дом СВХ отсутствуют в добавочном листе ДО. Требуется подтверждение оператора.
- `non_formalized.svh_additional_sheet_1.svh_customs_code`
  - `question`: Код таможни СВХ отсутствует в добавочном листе ДО. Требуется подтверждение оператора.

## 6. `unreliable_fields`:
(Нет недостоверно распознанных полей)
