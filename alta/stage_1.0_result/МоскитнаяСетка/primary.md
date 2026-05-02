## meta:
- `название кейса`: МоскитнаяСетка
- `путь к папке поставки`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- `direction`: ИМ
- `тип поставки`: 1 ДТ / 7 товаров
- `источники данных`: md + operator_provided_data + stable_source (xml)

## formalized

### `document`: Contract
  - `uqi_prefix`: formalized.contract_1
  - `xml_target_root`: AltaE2CONT
  - `path`: контракт\SALES CONTRACT NoLM-2553.pdf
  - `file_name`: SALES CONTRACT NoLM-2553.md
  - `note`: Договор купли-продажи № LM-2553 от 02.07.2025

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 03011 | CD | код вида документа для графы 44: G44/G441 | |
| 02 | ContractRegistration_PrDocumentNumber | LM-2553 | CD | № контракта; графа 44: G44/G442 | |
| 03 | ContractRegistration_PrDocumentDate | 02.07.2025 | CD | дата контракта; графа 44: G44/G443 | |
| 04 | ContractTerms_Amount | 270000.00 | CO | общая сумма контракта | нормализация: по доп. соглашению №1 |
| 05 | ContractTerms_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator_provided_data |
| 06 | ContractTerms_LastDate | 31.12.2026 | CO | срок действия/исполнения | нормализация: по доп. соглашению №1 |
| 07 | ContractTerms_OtherTerms | EXW | CO | условия поставки / Incoterms | operator_provided_data |
| 08 | ContractTerms_ContractText | link:контракт\SALES CONTRACT NoLM-2553.pdf | CD | текст контракта | link на файл-источник |
| 09 | ContractTerms_DealSign | 1 | CO | системный признак Альты | operator_provided_data |
| 10 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец/сторона контракта | |
| 11 | ForeignPerson_Address_CountryCode | CN | CO | страна продавца alpha-2 | нормализация по cb:country; operator_provided_data |
| 12 | ForeignPerson_Address_CounryName | КИТАЙ | CD | страна продавца, текст | из контракта стр. 4 |
| 13 | ForeignPerson_Address_Region | Hebei | CD | регион/область продавца | |
| 14 | ForeignPerson_Address_City | Shijiazhuang | CD | город/район продавца | |
| 15 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District | CD | улица/дом продавца одной строкой | |
| 16 | RussianPerson_OrganizationName | ООО «СКИФ» | CD | покупатель/сторона контракта | из контракта стр. 1 |
| 17 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН покупателя | cross-doc: master_data, operator_provided_data |
| 18 | RussianPerson_INN | 1650389298 | CO | ИНН покупателя | cross-doc: master_data, operator_provided_data |
| 19 | RussianPerson_KPP | 165001001 | CO | КПП покупателя | cross-doc: master_data, operator_provided_data |
| 20 | RussianPerson_Address_PostalCode | 423800 | CD | индекс покупателя | |
| 21 | RussianPerson_Address_CountryCode | RU | CD | страна покупателя alpha-2 | нормализация по cb:country |
| 22 | RussianPerson_Address_CounryName | РОССИЯ | CD | страна покупателя, текст | |
| 23 | RussianPerson_Address_Region | Республика Татарстан | CD | регион покупателя | |
| 24 | RussianPerson_Address_City | Набережные Челны | CD | город покупателя | |
| 25 | RussianPerson_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом/офис одной строкой | |

#### Итого, по документу:
- `doc_fields`: 25 из 25
- `doc_formalization_status`: confirmed


### `document`: Supplementary Contract
  - `uqi_prefix`: formalized.supplementary_contract_1
  - `xml_target_root`: AltaSupplementaryContract
  - `path`: контракт\1 Supplementary agreement to the contract.pdf
  - `file_name`: 1 Supplementary agreement to the contract.md
  - `note`: Дополнительное соглашение №1 к LM-2553 от 25.11.2025

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentNumber | 1 | CD | № доп. соглашения; графа 44: G44/G442 | |
| 02 | IssueDate | 25.11.2025 | CD | дата доп. соглашения; графа 44: G44/G443 | |
| 03 | ContractDescription_Amount | 270000.00 | CD | новая/уточненная сумма контракта | |
| 04 | ContractDescription_CurrencyCode | 156 | CO | цифровой код валюты ISO 4217 numeric | operator_provided_data |
| 05 | ContractDescription_LastDate | 31.12.2026 | CO | новый срок действия/исполнения | operator_provided_data |
| 06 | ContractDescription_ContractText | link:контракт\1 Supplementary agreement to the contract.pdf | CD | текст доп. соглашения | link на файл-источник |
| 07 | ContractDescription_DealSign | 1 | CO | системный признак Альты | operator_provided_data |
| 08 | ContractDescription_StockCategorySign | 0 | CO | системный признак Альты | operator_provided_data |
| 09 | ContractDescription_BuyerLimitationSign | 0 | CO | системный признак Альты | operator_provided_data |
| 10 | ContractDescription_InsuranceSign | 0 | CO | системный признак Альты | operator_provided_data |
| 11 | RussianPerson_OrganizationName | ООО «СКИФ» | CD | российская сторона; покупатель | |
| 12 | RussianPerson_ShortName | ООО «СКИФ» | CO | краткое наименование | |
| 13 | RussianPerson_OGRN | 1201600020390 | CO | ОГРН | cross-doc: master_data |
| 14 | RussianPerson_INN | 1650389298 | CO | ИНН | cross-doc: master_data |
| 15 | RussianPerson_KPP | 165001001 | CO | КПП | cross-doc: master_data |
| 16 | RussianPerson_Address_PostalCode | 423800 | CD | индекс | |
| 17 | RussianPerson_Address_CountryCode | RU | CD | страна alpha-2 | нормализация по cb:country |
| 18 | RussianPerson_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 19 | RussianPerson_Address_Region | Республика Татарстан | CO | регион | cross-doc: contract_1 |
| 20 | RussianPerson_Address_City | Набережные Челны | CO | город | cross-doc: contract_1 |
| 21 | RussianPerson_Address_StreetHouse | проезд Хлебный, 30, 211 | CO | улица/дом одной строкой | cross-doc: LetterOfAttorney_1 |
| 22 | ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | иностранная сторона; продавец | |
| 23 | ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator_provided_data: short_name_equals_full |
| 24 | ForeignPerson_Address_CountryCode | CN | CO | страна alpha-2 | operator_provided_data |
| 25 | ForeignPerson_Address_CounryName | КИТАЙ | CD | страна, текст | |
| 26 | ForeignPerson_Address_Region | Hebei | CO | регион | cross-doc: contract_1, operator_provided_data |
| 27 | ForeignPerson_Address_City | Shijiazhuang | CO | город/район | cross-doc: contract_1, operator_provided_data |
| 28 | ForeignPerson_Address_StreetHouse | No.5 Gaodong Street, Xinhua District | CO | улица/дом одной строкой | cross-doc: contract_1, operator_provided_data |
| 29 | PersonSurname | Li | CO | фамилия подписанта | operator_provided_data |
| 30 | PersonName | Jing | CO | имя подписанта | operator_provided_data |
| 31 | PersonMiddleName | | CO | отчество подписанта | operator_provided_data: пусто |

#### Итого, по документу:
- `doc_fields`: 31 из 31
- `doc_formalization_status`: confirmed

### `document`: Invoice
  - `uqi_prefix`: formalized.invoice_1
  - `xml_target_root`: AltaE2I
  - `path`: md\CL на сетку.md
  - `file_name`: CL на сетку.md
  - `note`: Инвойс № LM-2591 от 30.10.2025

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | CurrencyRate | 10.9430 | CO | курс валюты; графа 23: G_23_1, G_23_2 | operator_provided_data |
| 02 | CurrencyCode | CNY | CO | валюта инвойса ISO 4217 alpha-3 | operator_provided_data |
| 03 | DocumentCode | 04021 | CD | код вида документа для графы 44: G44/G441 | |
| 04 | PlacesQuantity | 127 | CO | кол-во грузовых мест по инвойсу | operator_provided_data: из PL |
| 05 | PlacesDescription | Поддон | CO | описание мест | operator_provided_data |
| 06 | GrossWeightQuantity | 3500.00 | CO | общий вес брутто по инвойсу | operator_provided_data: из PL totals |
| 07 | NetWeightQuantity | 3302.00 | CO | общий вес нетто по инвойсу | operator_provided_data: из PL totals |
| 08 | GCost | 97260.00 | CO | системное поле Альты; дубль TotalCost | operator_provided_data: =TotalCost |
| 09 | TotalCost | 97260.00 | CD | итого по инвойсу; графа 22: G_22_2 | |
| 10 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 11 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий поставки | operator_provided_data |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator_provided_data |
| 13 | DeliveryTerms_DispatchCountryCode | CN | CO | страна отправления alpha-2 | operator_provided_data |
| 14 | DeliveryTerms_TradingCountryCode | CN | CO | торгующая страна alpha-2 | operator_provided_data |
| 15 | DeliveryTerms_DestinationCountryCode | RU | CO | страна назначения alpha-2 | operator_provided_data |
| 16 | Registration_PrDocumentName | КОММЕРЧЕСКИЙ ИНВОЙС | CD | наименование документа; графа 44: G44/G444 | |
| 17 | Registration_PrDocumentNumber | LM-2591 | CD | номер инвойса; графа 44: G44/G442 | |
| 18 | Registration_PrDocumentDate | 30.10.2025 | CD | дата инвойса; графа 44: G44/G443 | |
| 19 | Contract_PrDocumentNumber | LM-2553 | CD | № контракта-ссылки; графа 44: G44/G442 | |
| 20 | Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта-ссылки; графа 44: G44/G443 | |
| 21 | Buyer_CompanyID | 1650389298 | CO | ИНН покупателя | cross-doc: master_data |
| 22 | Buyer_KPPCode | 165001001 | CO | КПП покупателя | cross-doc: master_data |
| 23 | Buyer_Name | ООО «СКИФ» | CD | наименование покупателя | |
| 24 | Buyer_PostalAddress_PostalCode | 423800 | CD | индекс покупателя | |
| 25 | Buyer_PostalAddress_CountryCode | RU | CD | страна покупателя alpha-2 | нормализация по cb:country |
| 26 | Buyer_PostalAddress_CounryName | РОССИЯ | CD | страна покупателя, текст | |
| 27 | Buyer_PostalAddress_Region | Республика Татарстан | CD | регион | |
| 28 | Buyer_PostalAddress_City | Набережные Челны | CD | город | |
| 29 | Buyer_PostalAddress_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом/офис одной строкой | |
| 30 | Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | продавец | |
| 31 | Seler_PostalAddress_CountryCode | CN | CO | страна продавца alpha-2 | нормализация по cb:country |
| 32 | Seler_PostalAddress_CounryName | КИТАЙ | CD | страна продавца, текст | |
| 33 | Seler_PostalAddress_Region | Hebei | CD | регион продавца | |
| 34 | Seler_PostalAddress_City | Shijiazhuang | CD | город/район продавца | |
| 35 | Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | |
| 36 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | грузоотправитель | нормализация: consignor=seller |
| 37 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | нормализация: consignor=seller |
| 38 | Consignor_Address_CounryName | КИТАЙ | CO | страна грузоотправителя, текст | нормализация: consignor=seller |
| 39 | Consignor_Address_Region | Hebei | CO | регион | нормализация: consignor=seller |
| 40 | Consignor_Address_City | Shijiazhuang | CO | город/район | нормализация: consignor=seller |
| 41 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CO | улица/дом одной строкой | нормализация: consignor=seller |
| 42 | Consignee_OrganizationName | ООО «СКИФ» | CO | грузополучатель | нормализация: consignee=buyer |
| 43 | Consignee_OGRN | 1201600020390 | CO | ОГРН | cross-doc: master_data |
| 44 | Consignee_INN | 1650389298 | CO | ИНН | cross-doc: master_data |
| 45 | Consignee_KPP | 165001001 | CO | КПП | cross-doc: master_data |
| 46 | Consignee_Address_PostalCode | 423800 | CD | индекс | |
| 47 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | |
| 48 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 49 | Consignee_Address_Region | Республика Татарстан | CD | регион | |
| 50 | Consignee_Address_City | Набережные Челны | CD | город | |
| 51 | Consignee_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом/офис одной строкой | |

#### InvoiceGoods_1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД; графа 33: G_33_1 | |
| 02 | GoodsDescription | Anti-cat mesh. Roll size 1.4 * 30 Material: polyester / Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке в «основной» единице (наборы) | |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм для ДТ (M2) | неформализуемое поле |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм из cb:unit | код 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества для ДТ | TOVG/NAME_EDI |
| 07 | GrossWeightQuantity | 855.00 | CO | брутто по строке | operator_provided_data: из PL |
| 08 | NetWeightQuantity | 806.60 | CO | нетто по строке | operator_provided_data: из PL |
| 09 | Price | 5.85 | CD | цена за единицу (за M2) | |
| 10 | TotalCost | 14742.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data, нормализация alpha-2: CN |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator_provided_data |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД; графа 33: G_33_1 | |
| 02 | GoodsDescription | Anti-cat mesh Roll size 1.6 * 30 / Москитная сетка «Антикот» Размер рулона 1,6*30 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке в «основной» единице (наборы) | |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм для ДТ (M2) | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм из cb:unit | код 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества для ДТ | |
| 07 | GrossWeightQuantity | 490.00 | CO | брутто по строке | operator_provided_data: из PL |
| 08 | NetWeightQuantity | 460.80 | CO | нетто по строке | operator_provided_data: из PL |
| 09 | Price | 5.85 | CD | цена за единицу (за M2) | |
| 10 | TotalCost | 8424.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator_provided_data |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД; графа 33: G_33_1 | |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 / Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2 / Материал: полиэстер | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 60 | CD | кол-во по строке в «основной» единице (наборы) | |
| 04 | goods_supplementary_quantity | 2520 | CD | количество в доп.ед.изм для ДТ (M2) | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм из cb:unit | код 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества для ДТ | |
| 07 | GrossWeightQuantity | 265.00 | CO | брутто по строке | operator_provided_data: из PL |
| 08 | NetWeightQuantity | 252.00 | CO | нетто по строке | operator_provided_data: из PL |
| 09 | Price | 6.35 | CD | цена за единицу (за M2) | |
| 10 | TotalCost | 16002.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator_provided_data |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД; графа 33: G_33_1 | |
| 02 | GoodsDescription | ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2 / Сетка против пыльцы Антипыльца" из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 30 | CD | кол-во по строке в «основной» единице (наборы) | |
| 04 | goods_supplementary_quantity | 1440 | CD | количество в доп.ед.изм для ДТ (M2) | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм из cb:unit | код 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества для ДТ | |
| 07 | GrossWeightQuantity | 155.00 | CO | брутто по строке | operator_provided_data: из PL |
| 08 | NetWeightQuantity | 144.00 | CO | нетто по строке | operator_provided_data: из PL |
| 09 | Price | 6.35 | CD | цена за единицу (за M2) | |
| 10 | TotalCost | 9144.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator_provided_data |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД; графа 33: G_33_1 | |
| 02 | GoodsDescription | MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 90 | CD | кол-во по строке в «основной» единице (наборы) | |
| 04 | goods_supplementary_quantity | 3780 | CD | количество в доп.ед.изм для ДТ (M2) | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм из cb:unit | код 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества для ДТ | |
| 07 | GrossWeightQuantity | 520.00 | CO | брутто по строке | operator_provided_data: из PL |
| 08 | NetWeightQuantity | 491.40 | CO | нетто по строке | operator_provided_data: из PL |
| 09 | Price | 3.4 | CD | цена за единицу (за M2) | |
| 10 | TotalCost | 12852.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator_provided_data |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 7019900095 | CD | код ТН ВЭД; графа 33: G_33_1 | |
| 02 | GoodsDescription | MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 : Fiberglass / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 180 | CD | кол-во по строке в «основной» единице (наборы) | |
| 04 | goods_supplementary_quantity | 8640 | CD | количество в доп.ед.изм для ДТ (M2) | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм из cb:unit | код 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества для ДТ | |
| 07 | GrossWeightQuantity | 1190.00 | CO | брутто по строке | operator_provided_data: из PL |
| 08 | NetWeightQuantity | 1123.20 | CO | нетто по строке | operator_provided_data: из PL |
| 09 | Price | 3.4 | CD | цена за единицу (за M2) | |
| 10 | TotalCost | 29376.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator_provided_data |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### InvoiceGoods_7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsCode | 5804101000 | CD | код ТН ВЭД; графа 33: G_33_1 | |
| 02 | GoodsDescription | GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2 / Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | CD | описание товара как в инвойсе | |
| 03 | GoodsQuantity | 5 | CD | кол-во по строке в «основной» единице (наборы) | |
| 04 | goods_supplementary_quantity | 240 | CD | количество в доп.ед.изм для ДТ (M2) | |
| 05 | goods_supplementary_uom_name | м² | CD | наименование доп.ед.изм из cb:unit | код 055 |
| 06 | MeasureUnitQualifierName | м² | CD | единица измерения доп.количества для ДТ | |
| 07 | GrossWeightQuantity | 25.00 | CO | брутто по строке | operator_provided_data: из PL |
| 08 | NetWeightQuantity | 24.00 | CO | нетто по строке | operator_provided_data: из PL |
| 09 | Price | 28 | CD | цена за единицу (за M2) | |
| 10 | TotalCost | 6720.00 | CD | стоимость по строке | |
| 11 | OriginCountryCode | 156 | CO | цифровой код страны происхождения | operator_provided_data |
| 12 | AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | производитель | operator_provided_data |
| 13 | AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | CO | товарная марка/ТМ | operator_provided_data |
| 14 | AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | CO | товарный знак/маркировка | operator_provided_data |
| 15 | AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | CO | модель/модификация | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 15 из 15

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 105 из 15 * 7
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 52 из 52
- `doc_formalization_status`: confirmed

### `document`: Packing List
  - `uqi_prefix`: formalized.packing_list_1
  - `xml_target_root`: AltaE2PACK
  - `path`: md\PL на сетку.md
  - `file_name`: PL на сетку.md
  - `note`: Упаковочный лист к инвойсу LM-2591 от 30.10.2025

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GrossWeightQuantity | 3500.00 | CD | общий вес брутто по упаковочному | |
| 02 | NetWeightQuantity | 3302.00 | CD | общий вес нетто по упаковочному | |
| 03 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | грузоотправитель | |
| 04 | Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | краткое наименование | operator_provided_data: shortname_equals_full |
| 05 | Consignor_Address_CountryCode | CN | CO | страна грузоотправителя alpha-2 | нормализация по cb:country |
| 06 | Consignor_Address_CounryName | КИТАЙ | CD | страна грузоотправителя, текст | |
| 07 | Consignor_Address_Region | Hebei | CD | регион | |
| 08 | Consignor_Address_City | Shijiazhuang | CD | город/район | |
| 09 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | |
| 10 | Consignee_OrganizationName | ООО «СКИФ» | CD | грузополучатель | |
| 11 | Consignee_ShortName | ООО «СКИФ» | CO | краткое наименование | operator_provided_data: shortname_equals_full |
| 12 | Consignee_OGRN | 1201600020390 | CO | ОГРН | cross-doc: master_data |
| 13 | Consignee_INN | 1650389298 | CO | ИНН | cross-doc: master_data |
| 14 | Consignee_KPP | 165001001 | CO | КПП | cross-doc: master_data |
| 15 | Consignee_Address_PostalCode | 423800 | CD | индекс | |
| 16 | Consignee_Address_CountryCode | RU | CD | страна alpha-2 | |
| 17 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 18 | Consignee_Address_Region | Республика Татарстан | CD | регион | |
| 19 | Consignee_Address_City | Набережные Челны | CD | город | |
| 20 | Consignee_Address_StreetHouse | проезд Хлебный, дом 30, офис 211 | CD | улица/дом/офис одной строкой | |
| 21 | DeliveryTerms_DeliveryPlace | HEBEI | CD | место поставки по Incoterms | |
| 22 | DeliveryTerms_DeliveryTermsNumericCode | 01 | CO | числовой код условий | operator_provided_data |
| 23 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | строковый код условий | operator_provided_data |
| 24 | DeliveryTerms_Contract_PrDocumentName | ДОГОВОР КУПЛИ-ПРОДАЖИ | CD | наименование контракта для печати | |
| 25 | DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | CD | № контракта | |
| 26 | DeliveryTerms_Contract_PrDocumentDate | 02.07.2025 | CD | дата контракта | |
| 27 | DeliveryTerms_Invoice_PrDocumentName | КОММЕРЧЕСКИЙ ИНВОЙС | CD | наименование инвойса для печати | |
| 28 | DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | CD | № инвойса | |
| 29 | DeliveryTerms_Invoice_PrDocumentDate | 30.10.2025 | CD | дата инвойса | |
| 30 | DeliveryTerms_Registration_PrDocumentName | УПАКОВОЧНЫЙ ЛИСТ | CD | наименование упаковочного | |
| 31 | DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | CO | № упаковочного | operator_provided_data |
| 32 | DeliveryTerms_Registration_PrDocumentDate | 30.10.2025 | CO | дата упаковочного | operator_provided_data |
| 33 | registration_doc_name | УПАКОВОЧНЫЙ ЛИСТ | CO | наименование документа для графы 44 | неформализуемое; operator_provided_data |
| 34 | registration_doc_number | LM-2591 | CO | номер документа для графы 44 | неформализуемое; operator_provided_data |
| 35 | registration_doc_date | 30.10.2025 | CO | дата документа для графы 44 | неформализуемое; operator_provided_data |

#### Goods_1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16 | CD | описание строки как в документе | |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | |
| 03 | GrossWeightQuantity | 855.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 806.60 | CD | нетто по строке | |
| 05 | PakingQuantity | 60 | CO | кол-во упаковок/мест в упаковке | operator_provided_data: =GoodsQuantity |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Anti-cat mesh / Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | CD | описание строки как в документе | |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц | |
| 03 | GrossWeightQuantity | 490.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 460.80 | CD | нетто по строке | |
| 05 | PakingQuantity | 30 | CO | кол-во упаковок/мест в упаковке | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_3

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | ANTI-POLLEN MESH / Сетка против пыльцы Антипыльца " из полиэстера. Размер рулона 1,42*0,64*0,22 | CD | описание строки как в документе | |
| 02 | GoodsQuantity | 60 | CD | количество мест/грузовых единиц | |
| 03 | GrossWeightQuantity | 265.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 252.00 | CD | нетто по строке | |
| 05 | PakingQuantity | 6 | CO | кол-во упаковок/мест в упаковке | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_4

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | ANTI-POLLEN MESH / Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,62*0,64*0,23 | CD | описание строки как в документе | |
| 02 | GoodsQuantity | 30 | CD | количество мест/грузовых единиц | |
| 03 | GrossWeightQuantity | 155.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 144.00 | CD | нетто по строке | |
| 05 | PakingQuantity | 3 | CO | кол-во упаковок/мест в упаковке | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_5

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | MIDGE MEHS / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,42*0,55*0,18 | CD | описание строки как в документе | |
| 02 | GoodsQuantity | 90 | CD | количество мест/грузовых единиц | |
| 03 | GrossWeightQuantity | 520.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 491.40 | CD | нетто по строке | |
| 05 | PakingQuantity | 9 | CO | кол-во упаковок/мест в упаковке | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_6

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | MIDGE MEHS / СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка" из стекловолокна. Размер рулона 1,62*0,55*18 | CD | описание строки как в документе | |
| 02 | GoodsQuantity | 180 | CD | количество мест/грузовых единиц | |
| 03 | GrossWeightQuantity | 1190.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 1123.20 | CD | нетто по строке | |
| 05 | PakingQuantity | 18 | CO | кол-во упаковок/мест в упаковке | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Goods_7

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | GRID WITH 3 LAYER / Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,72*0,35* 0,31*1 | CD | описание строки как в документе | |
| 02 | GoodsQuantity | 5 | CD | количество мест/грузовых единиц | |
| 03 | GrossWeightQuantity | 25.00 | CD | брутто по строке | |
| 04 | NetWeightQuantity | 24.00 | CD | нетто по строке | |
| 05 | PakingQuantity | 1 | CO | кол-во упаковок/мест в упаковке | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 7
- `item_fields`: всего полей 35 из 5 * 7
- `array_status`: confirmed

#### TransportMeans_1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | Number | О157АО774 | CO | регистрационный номер | operator_provided_data |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator_provided_data |
| 03 | NationalityCode | 000 | CO | код «национальности» ТС | operator_provided_data |
| 04 | MoverIndicator | true | CO | тягач | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### TransportMeans_2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | Number | ВТ374974 | CO | регистрационный номер | operator_provided_data |
| 02 | ModeCode | 31 | CO | код вида транспорта | operator_provided_data |
| 03 | NationalityCode | 000 | CO | код «национальности» ТС | operator_provided_data |
| 04 | MoverIndicator | false | CO | прицеп | operator_provided_data |

#### Итого, по элементу массива:
- `item_fields`: 4 из 4

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 8 из 4 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 37 из 37
- `doc_formalization_status`: confirmed

### `document`: CMR
  - `uqi_prefix`: formalized.cmr_1
  - `xml_target_root`: AltaE3CMR
  - `path`: md\СМР от СВХ.md
  - `file_name`: СМР от СВХ.md
  - `note`: Международная товарно-транспортная накладная CMR № 00378 от 20.01.2026

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | LanguageCode | RU | CO | язык документа | operator_provided_data |
| 02 | CMR_Choice | 1 | CO | системный выбор/вариант Альты | operator_provided_data |
| 03 | RegistrationDocument_RegID | 00378 | CD | номер CMR; графа 44: G44/G442 | |
| 04 | RegistrationDocument_DateInf | 20.01.2026 | CD | дата CMR; графа 44: G44/G443 | из п. 4/21 CMR |
| 05 | RegistrationDocument_Place | Маньчжурия | CO | место составления | operator_provided_data |
| 06 | TrakingCargo_TakingCargoDate | 20.01.2026 | CO | дата CMR (п. 4) | копия RegistrationDocument_DateInf |
| 07 | TrakingCargo_TakingCargoPlace_CountryCode | CN | CO | страна принятия груза alpha-2 | нормализация по cb:country |
| 08 | TrakingCargo_TakingCargoPlace_CounryName | КИТАЙ | CD | страна принятия груза, текст | |
| 09 | DeliveryPlace_CountryCode | RU | CO | страна доставки alpha-2 | operator_provided_data |
| 10 | DeliveryPlace_CounryName | РОССИЯ | CD | страна доставки, текст | |
| 11 | DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | CO | место поставки по Incoterms | operator_provided_data |
| 12 | DeliveryTerms_DeliveryTermsStringCode | EXW | CO | условия поставки | operator_provided_data |
| 13 | GoodsQuantity | 127 | CD | общее количество грузовых мест/упаковок | п. 7 CMR |
| 14 | CMRGoodsWeight_GrossWeightQuantity | 3500.00 | CD | общий вес брутто | п. 11 CMR |
| 15 | CMRTransport_PrimeMoverStateSignID | О157АО774 | CD | гос. номер тягача | из п. 25 CMR |
| 16 | CMRTransport_TrailerStateSignID | ВТ374974 | CD | гос. номер прицепа | из п. 25 CMR |
| 17 | Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CD | наименование отправителя | п. 1 CMR |
| 18 | Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | CO | краткое наименование | operator_provided_data: shortname_equals_full |
| 19 | Consignor_PostalAddress_CountryCode | CN | CD | страна alpha-2 | нормализация по cb:country |
| 20 | Consignor_Address_CounryName | КИТАЙ | CD | страна, текст | |
| 21 | Consignor_Address_Region | Hebei | CD | регион | |
| 22 | Consignor_Address_City | Shijiazhuang | CD | город/район | |
| 23 | Consignor_Address_StreetHouse | No. 5 Gaodong street | CD | улица/дом одной строкой | |
| 24 | Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | CO | наименование гаранта | operator_provided_data: все поля гаранта отсутствуют |
| 25 | Consignor_Guarantee_ShortName | ОТСУТСТВУЕТ | CO | краткое наименование | operator_provided_data |
| 26 | Consignor_Guarantee_Address_CountryCode | ОТСУТСТВУЕТ | CO | страна alpha-2 | operator_provided_data |
| 27 | Consignor_Guarantee_Address_CounryName | ОТСУТСТВУЕТ | CO | страна, текст | operator_provided_data |
| 28 | Consignor_Guarantee_Address_Region | ОТСУТСТВУЕТ | CO | регион | operator_provided_data |
| 29 | Consignor_Guarantee_Address_City | ОТСУТСТВУЕТ | CO | город/район | operator_provided_data |
| 30 | Consignor_Guarantee_Address_StreetHouse | ОТСУТСТВУЕТ | CO | улица/дом одной строкой | operator_provided_data |
| 31 | Consignee_NameInf | ООО «Скиф» | CD | наименование получателя | п. 2 CMR |
| 32 | Consignee_ShortName | ООО «СКИФ» | CO | краткое наименование | operator_provided_data: shortname_equals_full |
| 33 | Consignee_OGRNID | 1201600020390 | CO | ОГРН | operator_provided_data: из master_data |
| 34 | Consignee_INNID | 1650389298 | CD | ИНН | п. 2 CMR |
| 35 | Consignee_KPPCode | 165001001 | CD | КПП | п. 2 CMR |
| 36 | Consignee_PostalAddress_PostalCode | 423800 | CD | индекс | п. 2 CMR |
| 37 | Consignee_PostalAddress_CountryCode | RU | CD | страна alpha-2 | нормализация по cb:country |
| 38 | Consignee_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 39 | Consignee_Address_Region | Республика Татарстан | CD | регион | |
| 40 | Consignee_Address_City | Набережные Челны | CD | город | |
| 41 | Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | CD | улица/дом/офис одной строкой | |

#### CMRGoods_1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsNumeric | 1 | CD | порядковый номер строки | авто-нумерация единственной строки |
| 02 | GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | CO | описание груза/товара | исключение CMRGoodsDescription — источник п. 6 CMR |
| 03 | PakingQuantity | 127 | CO | кол-во упаковок/мест | operator_provided_data: равен общему кол-ву мест |

#### Итого, по элементу массива:
- `item_fields`: 3 из 3

#### Итого, по массиву:
- `array_elements`: 1
- `item_fields`: всего полей 3 из 3 * 1
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 42 из 42
- `doc_formalization_status`: confirmed

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_1
  - `xml_target_root`: AltaPaymentOrder
  - `path`: платежки\currency_transfer_1_13.01.2026.pdf
  - `file_name`: currency_transfer_1_13.01.2026.md
  - `note`: Заявление на перевод № 1 от 13.01.2026

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CO | код вида документа; графа 44: G44/G441 | operator_provided_data |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator_provided_data |
| 03 | PaymentAmount | 63219.00 | CD | сумма платежа | CNY |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator_provided_data |
| 05 | Priority | 5 | CO | очередность | operator_provided_data |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер платежного поручения; графа 44: G44/G442 | |
| 09 | DocumentReference_PrDocumentDate | 13.01.2026 | CD | дата платежного поручения; графа 44: G44/G443 | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator_provided_data |
| 13 | Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО), 044525411 | CD | банк плательщика | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX, SHANGHAI TOWER, RM. 2503-2505 FLOOR 25, 501 MIDDLE YINCHENG ROAD, PUDONG SHANGHAI, CN | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator_provided_data |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator_provided_data |

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### `document`: Payment Order
  - `uqi_prefix`: formalized.payment_order_2
  - `xml_target_root`: AltaPaymentOrder
  - `path`: платежки\currency_transfer_7_28.11.2025.pdf
  - `file_name`: currency_transfer_7_28.11.2025.md
  - `note`: Заявление на перевод № 7 от 28.11.2025

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04023 | CO | код вида документа; графа 44: G44/G441 | operator_provided_data |
| 02 | PaymentModeCode | 0 | CO | системный код способа платежа | operator_provided_data |
| 03 | PaymentAmount | 34041.00 | CD | сумма платежа | CNY |
| 04 | TransactionKind | 01 | CO | вид операции/код | operator_provided_data |
| 05 | Priority | 5 | CO | очередность | operator_provided_data |
| 06 | Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | CD | назначение платежа | |
| 07 | ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | CD | сумма прописью | |
| 08 | DocumentReference_PrDocumentNumber | 7 | CD | номер платежного поручения; графа 44: G44/G442 | |
| 09 | DocumentReference_PrDocumentDate | 28.11.2025 | CD | дата платежного поручения; графа 44: G44/G443 | |
| 10 | Payer_OrganizationName | LLC SKIF | CD | плательщик | |
| 11 | Payer_INN | 1650389298 | CD | ИНН плательщика | |
| 12 | Payer_KPP | 165001001 | CO | КПП плательщика | operator_provided_data |
| 13 | Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО), 044525411 | CD | банк плательщика | |
| 14 | Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CD | получатель платежа | |
| 15 | Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX, SHANGHAI TOWER, RM. 2503-2505 FLOOR 25, 501 MIDDLE YINCHENG ROAD, PUDONG SHANGHAI, CN | CD | банк получателя | |
| 16 | PersonSurname | Саранов | CO | фамилия подписанта | operator_provided_data |
| 17 | PersonName | Дмитрий | CO | имя подписанта | operator_provided_data |

#### Итого, по документу:
- `doc_fields`: 17 из 17
- `doc_formalization_status`: confirmed

### `document`: Service Invoice
  - `uqi_prefix`: formalized.service_invoice_1
  - `xml_target_root`: AltaServiceInvoice
  - `path`: md\Счет_№26-00378-tl_от_27-01-2026.md
  - `file_name`: Счет_№26-00378-tl_от_27-01-2026.md
  - `note`: Счет на оплату № 26-00378-tl от 27.01.2026 за транспортно-экспедиционные услуги

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentSign | 1 | CO | системный признак документа Альты | operator_provided_data |
| 02 | TotalServiceCost | 2700.00 | CD | итого по услугам | USD |
| 03 | Currency | USD | CD | валюта итого ISO 4217 alpha-3 | |
| 04 | ServiceProvider_Name | ООО «Трансимпериал» | CD | исполнитель услуг/перевозчик | |
| 05 | BankName | АО "Райффайзенбанк", БИК 044525700, Сч. 30101810200000000700 | CD | банк исполнителя | |
| 06 | ContractDetails_PrDocumentNumber | КООО/26651/М | CD | № договора на услуги/перевозку; графа 44: G44/G442 | |
| 07 | ContractDetails_PrDocumentDate | 13.05.2025 | CD | дата договора на услуги/перевозку; графа 44: G44/G443 | |
| 08 | PrDocumentNumber | ОТСУТСТВУЕТ | CO | номер связанного документа/заказа | operator_provided_data |
| 09 | PrDocumentDate | ОТСУТСТВУЕТ | CO | дата связанного документа/заказа | operator_provided_data |
| 10 | Registration_PrDocumentName | Счет на оплату | CD | наименование счета; графа 44: G44/G444 | |
| 11 | Registration_PrDocumentNumber | 26-00378-tl | CD | номер счета; графа 44: G44/G442 | |
| 12 | Registration_PrDocumentDate | 27.01.2026 | CD | дата счета; графа 44: G44/G443 | |
| 13 | Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | CO | грузоотправитель | operator_provided_data: consignor=seller |
| 14 | PostalCode | | CO | индекс грузоотправителя | operator_provided_data: пусто, если отсутствует |
| 15 | CountryCode | CN | CO | страна alpha-2 | operator_provided_data: consignor_address_from_seller |
| 16 | CounryName | КИТАЙ | CO | страна, текст | operator_provided_data |
| 17 | Region | Hebei | CO | регион | operator_provided_data |
| 18 | Town | Shijiazhuang | CO | город/район | operator_provided_data |
| 19 | StreetHouse | No. 5 Gaodong Street, Xinhua District | CO | улица/дом одной строкой | operator_provided_data |
| 20 | Consignee_OrganizationName | ООО «СКИФ» | CD | грузополучатель | |
| 21 | Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | CO | ОГРН | operator_provided_data: из master_data |
| 22 | Consignee_RFOrganizationFeatures_INN | 1650389298 | CD | ИНН | |
| 23 | Consignee_RFOrganizationFeatures_KPP | 165001001 | CD | КПП | |
| 24 | PostalCode | 423800 | CD | индекс | |
| 25 | CountryCode | RU | CD | страна alpha-2 | нормализация по cb:country |
| 26 | CounryName | РОССИЯ | CD | страна, текст | |
| 27 | Region | Республика Татарстан | CO | регион | cross-doc: master_data |
| 28 | Town | Набережные Челны | CO | город | cross-doc: master_data |
| 29 | StreetHouse | проезд Хлебный | CO | улица | cross-doc: master_data |
| 30 | House | 30 | CO | дом | operator_provided_data |
| 31 | Room | 211 | CO | офис/кв | operator_provided_data |
| 32 | Signature_Choice | 1 | CO | вариант подписи | operator_provided_data |
| 33 | SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | CO | фамилия руководителя | operator_provided_data |
| 34 | SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | CO | инициалы/имя руководителя | operator_provided_data |
| 35 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | CO | фамилия бухгалтера | operator_provided_data |
| 36 | SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | CO | инициалы/имя бухгалтера | operator_provided_data |

#### ServiceDescription_1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | CD | многострочное описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки ISO alpha-3 | |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator_provided_data |
| 04 | TaxRate | 0 | CD | ставка налога | НДС 0% |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1404.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### ServiceDescription_2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | CD | многострочное описание услуги | |
| 02 | CurrencyCode | USD | CD | валюта строки ISO alpha-3 | |
| 03 | ServiceName | ОТСУТСТВУЕТ | CO | наименование/маршрут | operator_provided_data |
| 04 | TaxRate | 0 | CD | ставка налога | НДС 0% |
| 05 | TaxSum | 0.00 | CD | сумма налога | |
| 06 | ServiceCost_Amount | 1296.00 | CD | стоимость строки | |
| 07 | ServiceCost_Currency | USD | CD | валюта стоимости строки | |

#### Итого, по элементу массива:
- `item_fields`: 7 из 7

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 14 из 7 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 37 из 37
- `doc_formalization_status`: confirmed

### `document`: Transport Contract
  - `uqi_prefix`: formalized.transport_contract_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\stable_source\FreeDoc_КООО_26651_М.xml
  - `file_name`: FreeDoc_КООО_26651_М.xml
  - `note`: Договор транспортной экспедиции № КООО/26651/М от 13.05.2025

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04033 | CD | код вида документа для графы 44: G44/G441 | из stable_source xml |
| 02 | DocumentHead_DocumentName | ДОГОВОР ПО ПЕРЕВОЗКЕ | CD | наименование договора; графа 44: G44/G444 | |
| 03 | DocumentHead_DocumentDate | 13.05.2025 | CD | дата договора; графа 44: G44/G443 | |
| 04 | DocumentHead_DocumentNumber | КООО/26651/М | CD | номер договора; графа 44: G44/G442 | |
| 05 | TextPara_[n] | link:alta\stable_source\FreeDoc_КООО_26651_М.xml | CD | текст договора | link на файл-источник |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

### `document`: EGRUL
  - `uqi_prefix`: formalized.egrul_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml
  - `file_name`: FreeDoc_ЮЭ9965-25-106893283.xml
  - `note`: Выписка из ЕГРЮЛ № ЮЭ9965-25-106893283 от 14.07.2025

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04011 | CD | код вида документа для графы 44: G44/G441 | из stable_source xml |
| 02 | DocumentHead_DocumentName | ВЫПИСКА ИЗ ЕГРЮЛ | CD | наименование выписки; графа 44: G44/G444 | |
| 03 | DocumentHead_DocumentDate | 14.07.2025 | CD | дата выписки; графа 44: G44/G443 | |
| 04 | DocumentHead_DocumentNumber | ЮЭ9965-25-106893283 | CD | номер выписки; графа 44: G44/G442 | |
| 05 | TextPara_[n] | link:alta\stable_source\FreeDoc_ЮЭ9965-25-106893283.xml | CD | текст выписки | link на файл-источник |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

### `document`: Personal Passport
  - `uqi_prefix`: formalized.passport_1
  - `xml_target_root`: AltaPassport
  - `path`: alta\stable_source\Passport_63_09_449948.xml
  - `file_name`: Passport_63_09_449948.xml
  - `note`: Паспорт 63 09 449948 Арбузовой Анастасии Константиновны

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 11001 | CD | код вида документа; графа 44 | |
| 02 | DocumentHead_DocumentName | ПАСПОРТ | CD | наименование документа; графа 44 | |
| 03 | DocumentHead_DocumentDate | 11.03.2010 | CD | дата документа; графа 44 | =CardDate |
| 04 | DocumentHead_DocumentNumber | 63 09 449948 | CD | номер документа; графа 44 | =CardSeries + CardNumber |
| 05 | CardSeries | 63 09 | CD | серия; графа 54: G_54_12 | |
| 06 | CardNumber | 449948 | CD | номер; графа 54: G_54_100 | |
| 07 | OrganizationName | ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан; графа 54: G_54_13 | |
| 08 | CardDate | 11.03.2010 | CD | дата выдачи; графа 54: G_54_101 | |
| 09 | PersonInfo_PersonSurname | АРБУЗОВА | CD | фамилия; графа 54: G_54_3 | |
| 10 | PersonInfo_PersonName | АНАСТАСИЯ | CD | имя; графа 54: G_54_3NM | |
| 11 | PersonInfo_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество; графа 54: G_54_3MD | |
| 12 | PersonInfo_Sex | 1 | CD | пол | |
| 13 | PersonInfo_Birthday | 25.07.1987 | CD | дата рождения | |
| 14 | PersonInfo_Birthplace | город Саратов | CD | место рождения | |
| 15 | ResidencePlace_PostalCode | 410052 | CD | индекс | |
| 16 | ResidencePlace_CountryCode | RU | CD | страна alpha-2 | |
| 17 | ResidencePlace_CounryName | РОССИЯ | CD | страна, текст | |
| 18 | ResidencePlace_Region | Саратовская область | CD | регион | |
| 19 | ResidencePlace_City | Саратов | CD | город | |
| 20 | ResidencePlace_StreetHouse | Ул. Одесская д 11 кв 160 | CD | адрес одной строкой | |

#### Итого, по документу:
- `doc_fields`: 20 из 20
- `doc_formalization_status`: confirmed

### `document`: Letter of Attorney
  - `uqi_prefix`: formalized.letter_of_attorney_1
  - `xml_target_root`: AltaLetterOfAttorney
  - `path`: alta\stable_source\LetterOfAttorney_1.xml
  - `file_name`: LetterOfAttorney_1.xml
  - `note`: Доверенность № 1 от 01.02.2026 на Арбузову А.К.

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 11004 | CD | код вида документа; графа 44 | |
| 02 | DocumentHead_DocumentName | ДОВЕРЕННОСТЬ | CD | наименование документа; графа 44 | =DocumentReference_PrDocumentName |
| 03 | DocumentHead_DocumentDate | 01.02.2026 | CD | дата документа; графа 44 | =DocumentReference_PrDocumentDate |
| 04 | DocumentHead_DocumentNumber | 1 | CD | номер документа; графа 44 | =DocumentReference_PrDocumentNumber |
| 05 | Subject | link:alta\stable_source\LetterOfAttorney_1.xml | CD | текст доверенности | link на файл-источник |
| 06 | EndDate | 31.12.2026 | CD | действительна до; графа 54: G_54_61 | |
| 07 | DocumentReference_PrDocumentName | ДОВЕРЕННОСТЬ | CD | наименование доверенности; графа 54: G_54_4 | |
| 08 | DocumentReference_PrDocumentNumber | 1 | CD | номер доверенности; графа 54: G_54_5 | |
| 09 | DocumentReference_PrDocumentDate | 01.02.2026 | CD | дата доверенности; графа 54: G_54_60 | |
| 10 | Organization_OrganizationName | ООО «СКИФ» | CD | выдавшая организация | |
| 11 | Organization_ShortName | ООО «СКИФ» | CD | краткое наименование | |
| 12 | Organization_OGRN | 1201600020390 | CD | ОГРН | |
| 13 | Organization_INN | 1650389298 | CD | ИНН | |
| 14 | Organization_KPP | 165001001 | CD | КПП | |
| 15 | Organization_Address_PostalCode | 423800 | CD | индекс | |
| 16 | Organization_Address_CountryCode | RU | CD | страна alpha-2 | |
| 17 | Organization_Address_CounryName | РОССИЯ | CD | страна, текст | |
| 18 | Organization_Address_Region | РЕСПУБЛИКА ТАТАРСТАН | CD | регион | |
| 19 | Organization_Address_City | НАБЕРЕЖНЫЕ ЧЕЛНЫ | CD | город | |
| 20 | Organization_Address_StreetHouse | ПРОЕЗД ХЛЕБНЫЙ, 30, 211 | CD | улица/дом одной строкой | |
| 21 | Organization_OrganizationPerson_PersonSurname | Саранов | CD | подписант от организации | |
| 22 | Organization_OrganizationPerson_PersonName | Дмитрий | CD | имя/инициалы | |
| 23 | Organization_OrganizationPerson_PersonMiddleName | Олегович | CD | отчество | |
| 24 | Organization_OrganizationPerson_PersonPost | Директор | CD | должность | |
| 25 | EmpoweredPerson_PersonSurname | АРБУЗОВА | CD | уполномоченное лицо; графа 54: G_54_3 | |
| 26 | EmpoweredPerson_PersonName | АНАСТАСИЯ | CD | имя; графа 54: G_54_3NM | |
| 27 | EmpoweredPerson_PersonMiddleName | КОНСТАНТИНОВНА | CD | отчество; графа 54: G_54_3MD | |
| 28 | EmpoweredPerson_PersonPost | УПОЛНОМОЧЕННОЕ ЛИЦО | CD | роль/должность; графа 54: G_54_7 | |
| 29 | EmpoweredPerson_Passport_IdentityCardCode | RU01001 | CD | код документа; графа 54: G_54_8 | |
| 30 | EmpoweredPerson_Passport_IdentityCardName | ПАСРФ | CD | наименование документа; графа 54: G_54_9 | |
| 31 | EmpoweredPerson_Passport_IdentityCardSeries | 63 09 | CD | серия; графа 54: G_54_12 | |
| 32 | EmpoweredPerson_Passport_IdentityCardNumber | 449948 | CD | номер; графа 54: G_54_100 | |
| 33 | EmpoweredPerson_Passport_IdentityCardDate | 11.03.2010 | CD | дата выдачи; графа 54: G_54_101 | |
| 34 | EmpoweredPerson_Passport_OrganizationName | ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА | CD | кем выдан; графа 54: G_54_13 | |

#### Итого, по документу:
- `doc_fields`: 34 из 34
- `doc_formalization_status`: confirmed

## non_formalized

### `document`: Storage Report (ДО-1)
  - `uqi_prefix`: non_formalized.svh_1
  - `path`: md\ДО 14431420260204161621.md
  - `file_name`: ДО 14431420260204161621.md
  - `note`: Отчет № 0000080 от 03.02.2026 о принятии товаров на хранение

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | number | 0000080 | CD | № ДО-1; графа 30: G_30P_1 | |
| 02 | date | 03.02.2026 | CD | дата ДО-1; графа 30: G_30P_1 | |
| 03 | warehouse_license_number | 10404/141210/10092/5 | CD | номер лицензии/свидетельства СВХ; графа 30: G_30_1 | |
| 04 | warehouse_license_date | 18.09.2025 | CD | дата лицензии/свидетельства СВХ; графа 30: G_30_DATE | |
| 05 | actual_gross_weight | 3500 | CO | фактический вес по весам; графа 35: G_35_1 | operator_provided_data: из доп.листа ДО |
| 06 | actual_places | 127 | CO | фактическое количество мест; графа 6: G_6_1 (приоритет #1) | operator_provided_data: из доп.листа ДО |
| 07 | transport_reg_number | O157AO774 (прицеп: BT374974) | CD | номер ТС при въезде | |

#### goods_1

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tnved | 7019900095 | CD | код товара | строка 1 ДО |
| 02 | places | 27 | CD | кол-во грузовых мест | строка 1 ДО |
| 03 | gross_weight_kg | 1710 | CD | вес брутто по строке | строка 1 ДО |
| 04 | cost | 42228 | CD | стоимость по строке | строка 1 ДО, CNY |
| 05 | currency_code | CNY | CD | буквенный код валюты | строка 1 ДО |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### goods_2

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | tnved | 5804101000 | CD | код товара | строка 2 ДО |
| 02 | places | 100 | CD | кол-во грузовых мест | строка 2 ДО |
| 03 | gross_weight_kg | 1790 | CD | вес брутто по строке | строка 2 ДО |
| 04 | cost | 55032 | CD | стоимость по строке | строка 2 ДО, CNY |
| 05 | currency_code | CNY | CD | буквенный код валюты | строка 2 ДО |

#### Итого, по элементу массива:
- `item_fields`: 5 из 5

#### Итого, по массиву:
- `array_elements`: 2
- `item_fields`: всего полей 10 из 5 * 2
- `array_status`: confirmed

#### Итого, по документу:
- `doc_fields`: 8 из 8


### `document`: Insurance Document
  - `uqi_prefix`: formalized.insurance_document_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\Счет_№26-00378-tl_1_от_14-01-2026.md
  - `file_name`: Счет_№26-00378-tl_1_от_14-01-2026.md
  - `note`: Счет на оплату № 26-00378-tl/1 от 14.01.2026 — возмещение за добровольное страхование груза

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 04111 | CD | код вида документа для графы 44: G44/G441 | |
| 02 | DocumentHead_DocumentName | Счет на оплату (страхование) | CD | наименование документа; графа 44: G44/G444 | |
| 03 | DocumentHead_DocumentDate | 14.01.2026 | CD | дата документа; графа 44: G44/G443 | |
| 04 | DocumentHead_DocumentNumber | 26-00378-tl/1 | CD | номер документа; графа 44: G44/G442 | |
| 05 | TextPara_[n] | link:md\Счет_№26-00378-tl_1_от_14-01-2026.md | CD | основной текст/условия | link на файл-источник |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

### `document`: TechDescription
  - `uqi_prefix`: formalized.tech_description_1
  - `xml_target_root`: AltaFreeDoc
  - `path`: md\техничка Антикот, антипыльца антимошка.md
  - `file_name`: техничка Антикот, антипыльца антимошка.md
  - `note`: Техническое описание москитных сеток от 30.10.2025

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | DocumentCode | 05999 | CD | код вида документа для графы 44: G44/G441 | |
| 02 | DocumentHead_DocumentName | ТЕХНИЧЕСКИЕ ХАРАКТЕРИСТИКИ | CD | наименование техописания; графа 44: G44/G444 | |
| 03 | DocumentHead_DocumentDate | 30.10.2025 | CO | дата техописания; графа 44: G44/G443 | operator_provided_data |
| 04 | DocumentHead_DocumentNumber | Б/Н | CO | номер техописания; графа 44: G44/G442 | operator_provided_data |
| 05 | TextPara_[n] | link:md\техничка Антикот, антипыльца антимошка.md | CD | технический текст | link на файл-источник |

#### Итого, по документу:
- `doc_fields`: 5 из 5
- `doc_formalization_status`: confirmed

### `document`: Storage Report Additional Sheet
  - `uqi_prefix`: non_formalized.svh_additional_sheet_1
  - `path`: md\ДО доп 14431520260204161645.md
  - `file_name`: ДО доп 14431520260204161645.md
  - `note`: Добавочный лист № 1 к отчету № 0000080 от 03.02.2026

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | number | 1 | CD | № доп.листа; графа 30: G_30P_1 | |
| 02 | date | 03.02.2026 | CD | дата доп.листа; графа 30: G_30P_1 | |
| 03 | actual_gross_weight | 3500 | CD | фактический вес по весам | из доп.листа: итого |
| 04 | actual_places | 127 | CD | фактическое количество мест | из доп.листа: итого |
| 05 | transport_reg_number | O157AO774 (прицеп: BT374974) | CD | номер ТС при въезде | |
| 06 | svh_address_region | Республика Татарстан | CD | регион СВХ; графа 30: G_30_SUB | из CMR п. 3, решение оператора |
| 07 | svh_address_city | Набережные Челны | CD | город СВХ; графа 30: G_30_CIT | из CMR п. 3, решение оператора |
| 08 | svh_address_street_house | Производственный пр-д, д. 45 | CD | улица/дом СВХ; графа 30: G_30_STR | из CMR п. 3, решение оператора |
| 09 | svh_customs_code | 10404083 | CD | код таможенного органа; графа 30: G_30_12 | из CMR п. 13 |

#### Итого, по документу:
- `doc_fields`: 9 из 9

### `document`: Transit Declaration
  - `uqi_prefix`: non_formalized.td_1
  - `path`: md\ТД 10719110_240126_5011363_reg00378тд.md
  - `file_name`: ТД 10719110_240126_5011363_reg00378тд.md
  - `note`: Транзитная декларация № 10719110/240126/5011363

| num | field | value | status | description | note |
|-----|-------|-------|--------|-------------|------|
| 01 | number | 10719110/240126/5011363 | CD | номер ТД | |
| 02 | date | 24.01.2026 | CD | дата ТД | дата выпуска |
| 03 | customs_post_code | 10719110 | CD | код таможенного органа; графа 29: G_29_1 | т/п МАПП Забайкальск |
| 04 | customs_post_name | ТАМОЖЕННЫЙ ПОСТ МАПП ЗАБАЙКАЛЬСК | CD | наименование таможенного органа; графа 29: G_29_2 | |
| 05 | transport_reg_number | О157АО774/ВТ374974 | CD | ТС по ТД; сверка с графой 18: G_18 | |

#### Итого, по документу:
- `doc_fields`: 5 из 5

---

### Итогo, по файлу:

`total_doc_fields` — 17 документов (включая non_formalized), 251 поле без учета массивов
`total_fields` — 251 поле документов + 105 (InvoiceGoods) + 35 (PL Goods) + 8 (PL TransportMeans) + 3 (CMRGoods) + 14 (ServiceDescription) + 10 (SVH goods) = 426 полей
`formalization_status` — confirmed (все формализуемые документы подтверждены)
