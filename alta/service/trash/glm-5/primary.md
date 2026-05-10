# primary.md — МоскитнаяСетка

## Метаданные

- **название кейса**: МоскитнаяСетка
- **путь к папке поставки**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- **тип поставки**: 1 ДТ, 7 товаров (по инвойсу)
- **источник данных**: md-файлы (этап 0 выполнен), operator_provided_data.md

---

# РАЗДЕЛ I: formalized

---

## document: contract

- **uqi_prefix**: formalized.contract_1
- **xml_target_root**: AltaE2CONT
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\контракт\SALES CONTRACT NoLM-2553.pdf
- **file_name**: SALES CONTRACT NoLM-2553.pdf
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| DocumentCode | 03011 | код вида документа для графы 44 | confirmed_document | |
| ContractRegistration_PrDocumentNumber | LM-2553 | № контракта | confirmed_document | |
| ContractRegistration_PrDocumentDate | 2025-07-02 | дата контракта | confirmed_document | |
| ContractTerms_Amount | 41904.30 | общая сумма контракта (исходная) | confirmed_document | уточнена доп. соглашением |
| ContractTerms_CurrencyCode | 156 | цифровой код валюты (CNY) | confirmed_operator | из operator_provided_data |
| ContractTerms_LastDate | 2026-12-31 | срок действия | confirmed_document | |
| ContractTerms_OtherTerms | EXW HEBEI | условия поставки / Incoterms | confirmed_operator | из operator_provided_data |
| ContractTerms_ContractText | link:...SALES CONTRACT NoLM-2553.pdf | текст контракта | confirmed_document | |
| ContractTerms_DealSign | 1 | системный признак Альты | confirmed_operator | |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | продавец | confirmed_document | |
| ForeignPerson_Address_CountryCode | CN | страна продавца alpha-2 | confirmed_document | |
| ForeignPerson_Address_CounryName | Китай | страна продавца | confirmed_document | |
| ForeignPerson_Address_Region | Hebei | регион продавца | confirmed_document | |
| ForeignPerson_Address_City | Shijiazhuang | город продавца | confirmed_document | |
| ForeignPerson_Address_StreetHouse | No. 5 Gaodong Street, Xinhua District | адрес продавца | confirmed_document | |
| RussianPerson_OrganizationName | ООО «СКИФ» | покупатель | confirmed_document | |
| RussianPerson_OGRN | 1201600020390 | ОГРН покупателя | confirmed_document | из ЕГРЮЛ |
| RussianPerson_INN | 1650389298 | ИНН покупателя | confirmed_document | |
| RussianPerson_KPP | 165001001 | КПП покупателя | confirmed_document | |
| RussianPerson_Address_PostalCode | 423800 | индекс покупателя | confirmed_document | |
| RussianPerson_Address_CountryCode | RU | страна покупателя alpha-2 | confirmed_document | |
| RussianPerson_Address_CounryName | Россия | страна покупателя | confirmed_document | |
| RussianPerson_Address_Region | Республика Татарстан | регион покупателя | confirmed_document | |
| RussianPerson_Address_City | г. Набережные Челны | город покупателя | confirmed_document | |
| RussianPerson_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | адрес покупателя | confirmed_document | |

---

## document: supplementary_contract

- **uqi_prefix**: formalized.supplementary_contract_1
- **xml_target_root**: AltaSupplementaryContract
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\контракт\1 Supplementary agreement to the contract.docx
- **file_name**: 1 Supplementary agreement to the contract.docx
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| DocumentNumber | 1 | № доп. соглашения | confirmed_document | |
| IssueDate | 2025-11-25 | дата доп. соглашения | confirmed_document | |
| ContractDescription_Amount | 270000.00 | новая сумма контракта | confirmed_document | |
| ContractDescription_CurrencyCode | 156 | цифровой код валюты (CNY) | confirmed_operator | |
| ContractDescription_LastDate | 2026-12-31 | срок действия | confirmed_document | |
| ContractDescription_ContractText | link:...1 Supplementary agreement to the contract.docx | текст доп. соглашения | confirmed_document | |
| ContractDescription_DealSign | 1 | системный признак | confirmed_operator | |
| ContractDescription_StockCategorySign | 0 | системный признак | confirmed_operator | |
| ContractDescription_BuyerLimitationSign | 0 | системный признак | confirmed_operator | |
| ContractDescription_InsuranceSign | 0 | системный признак | confirmed_operator | |
| RussianPerson_OrganizationName | ООО «СКИФ» | российская сторона | confirmed_document | |
| RussianPerson_ShortName | ООО «СКИФ» | краткое наименование | confirmed_document | |
| RussianPerson_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| RussianPerson_INN | 1650389298 | ИНН | confirmed_document | |
| RussianPerson_KPP | 165001001 | КПП | confirmed_document | |
| RussianPerson_Address_PostalCode | 423800 | индекс | confirmed_document | |
| RussianPerson_Address_CountryCode | RU | страна alpha-2 | confirmed_document | |
| RussianPerson_Address_CounryName | Россия | страна | confirmed_document | |
| RussianPerson_Address_Region | Республика Татарстан | регион | confirmed_document | |
| RussianPerson_Address_City | г. Набережные Челны | город | confirmed_document | |
| RussianPerson_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | адрес | confirmed_document | |
| ForeignPerson_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | иностранная сторона | confirmed_document | |
| ForeignPerson_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | краткое наименование | confirmed_operator | совпадает с полным |
| ForeignPerson_Address_CountryCode | CN | страна alpha-2 | confirmed_document | |
| ForeignPerson_Address_CounryName | Китай | страна | confirmed_document | |
| ForeignPerson_Address_Region | Hebei | регион | confirmed_document | |
| ForeignPerson_Address_City | Shijiazhuang | город | confirmed_document | |
| ForeignPerson_Address_StreetHouse | No. 5 Gaodong Street, Xinhua District | адрес | confirmed_document | |

### ContractSignedPerson

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| PersonSurname | Li | фамилия подписанта | confirmed_operator | |
| PersonName | Jing | имя подписанта | confirmed_operator | |
| PersonMiddleName | | отчество | confirmed_operator | отсутствует |

---

## document: invoice

- **uqi_prefix**: formalized.invoice_1
- **xml_target_root**: AltaE2I
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\CL на сетку .xlsx
- **file_name**: CL на сетку .xlsx
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| DocumentCode | 04021 | код вида документа для графы 44 | confirmed_document | |
| CurrencyRate | 10.9430 | курс валюты | confirmed_operator | из operator_provided_data |
| CurrencyCode | CNY | валюта инвойса ISO alpha-3 | confirmed_operator | |
| PlacesQuantity | 127 | кол-во грузовых мест | confirmed_document | |
| PlacesDescription | Поддон | описание мест | confirmed_operator | |
| GrossWeightQuantity | 3500.00 | общий вес брутто | confirmed_document | из PL totals |
| NetWeightQuantity | 3302.00 | общий вес нетто | confirmed_document | из PL totals |
| GCost | 97260.00 | системное поле Альты | confirmed_operator | =TotalCost |
| TotalCost | 97260.00 | итого по инвойсу | confirmed_document | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | место поставки | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | числовой код условий поставки | confirmed_operator | EXW |
| DeliveryTerms_DeliveryTermsStringCode | EXW | строковый код условий | confirmed_operator | |
| DeliveryTerms_DispatchCountryCode | CN | страна отправления alpha-2 | confirmed_operator | |
| DeliveryTerms_TradingCountryCode | CN | торгующая страна alpha-2 | confirmed_operator | |
| DeliveryTerms_DestinationCountryCode | RU | страна назначения alpha-2 | confirmed_operator | |
| Registration_PrDocumentName | Commercial Invoice | наименование документа | confirmed_document | |
| Registration_PrDocumentNumber | LM-2591 | номер инвойса | confirmed_document | |
| Registration_PrDocumentDate | 2025-10-30 | дата инвойса | confirmed_document | |
| Contract_PrDocumentNumber | LM-2553 | № контракта-ссылки | confirmed_document | |
| Contract_PrDocumentDate | 2025-07-02 | дата контракта-ссылки | confirmed_document | |
| Buyer_CompanyID | 1650389298 | ИНН покупателя | confirmed_document | |
| Buyer_KPPCode | 165001001 | КПП покупателя | confirmed_document | |
| Buyer_Name | ООО «СКИФ» | наименование покупателя | confirmed_document | |
| Buyer_PostalAddress_PostalCode | 423800 | индекс покупателя | confirmed_document | |
| Buyer_PostalAddress_CountryCode | RU | страна покупателя alpha-2 | confirmed_document | |
| Buyer_PostalAddress_CounryName | Россия | страна покупателя | confirmed_document | |
| Buyer_PostalAddress_Region | Республика Татарстан | регион покупателя | confirmed_document | |
| Buyer_PostalAddress_City | г. Набережные Челны | город покупателя | confirmed_document | |
| Buyer_PostalAddress_StreetHouse | Хлебный пр-д, д. 30, офис 211 | адрес покупателя | confirmed_document | |
| Seler_Name | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | продавец | confirmed_document | опечатка тега: Seler |
| Seler_PostalAddress_CountryCode | CN | страна продавца alpha-2 | confirmed_document | |
| Seler_PostalAddress_CounryName | Китай | страна продавца | confirmed_document | |
| Seler_PostalAddress_Region | Hebei | регион продавца | confirmed_document | |
| Seler_PostalAddress_City | Shijiazhuang | город продавца | confirmed_document | |
| Seler_PostalAddress_StreetHouse | No. 5 Gaodong street | адрес продавца | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | грузоотправитель | confirmed_operator | совпадает с продавцом |
| Consignor_Address_CountryCode | CN | страна грузоотправителя alpha-2 | confirmed_document | |
| Consignor_Address_CounryName | Китай | страна грузоотправителя | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street | адрес | confirmed_document | |
| Consignee_OrganizationName | ООО «СКИФ» | грузополучатель | confirmed_operator | совпадает с покупателем |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна alpha-2 | confirmed_document | |
| Consignee_Address_CounryName | Россия | страна | confirmed_document | |
| Consignee_Address_Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_Address_City | г. Набережные Челны | город | confirmed_document | |
| Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | адрес | confirmed_document | |

### InvoiceGoods_1

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер | описание товара | confirmed_document | |
| GoodsQuantity | 60 | кол-во наборов | confirmed_document | |
| goods_supplementary_quantity | 2520 | количество в доп.ед.изм (м2) | confirmed_document | |
| goods_supplementary_uom_name | м2 | наименование доп.ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м2 | единица измерения доп.количества | confirmed_document | |
| GrossWeightQuantity | 855.00 | брутто по строке | confirmed_document | из PL |
| NetWeightQuantity | 806.60 | нетто по строке | confirmed_document | из PL |
| Price | 5.85 | цена за м2 | confirmed_document | |
| TotalCost | 14742.00 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | цифровой код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | товарная марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_2

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Москитная сетка «Антикот» Размер рулона 1,6*30 | описание товара | confirmed_document | |
| GoodsQuantity | 30 | кол-во наборов | confirmed_document | |
| goods_supplementary_quantity | 1440 | количество в доп.ед.изм (м2) | confirmed_document | |
| goods_supplementary_uom_name | м2 | наименование доп.ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м2 | единица измерения доп.количества | confirmed_document | |
| GrossWeightQuantity | 490.00 | брутто по строке | confirmed_document | из PL |
| NetWeightQuantity | 460.80 | нетто по строке | confirmed_document | из PL |
| Price | 5.85 | цена за м2 | confirmed_document | |
| TotalCost | 8424.00 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | цифровой код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | товарная марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_3

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,4*30 M2 | описание товара | confirmed_document | |
| GoodsQuantity | 60 | кол-во наборов | confirmed_document | |
| goods_supplementary_quantity | 2520 | количество в доп.ед.изм (м2) | confirmed_document | |
| goods_supplementary_uom_name | м2 | наименование доп.ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м2 | единица измерения доп.количества | confirmed_document | |
| GrossWeightQuantity | 265.00 | брутто по строке | confirmed_document | из PL |
| NetWeightQuantity | 252.00 | нетто по строке | confirmed_document | из PL |
| Price | 6.35 | цена за м2 | confirmed_document | |
| TotalCost | 16002.00 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | цифровой код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | товарная марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_4

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | описание товара | confirmed_document | |
| GoodsQuantity | 30 | кол-во наборов | confirmed_document | |
| goods_supplementary_quantity | 1440 | количество в доп.ед.изм (м2) | confirmed_document | |
| goods_supplementary_uom_name | м2 | наименование доп.ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м2 | единица измерения доп.количества | confirmed_document | |
| GrossWeightQuantity | 155.00 | брутто по строке | confirmed_document | из PL |
| NetWeightQuantity | 144.00 | нетто по строке | confirmed_document | из PL |
| Price | 6.35 | цена за м2 | confirmed_document | |
| TotalCost | 9144.00 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | цифровой код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | товарная марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_5

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsCode | 7019900095 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Сетка среднего размера "Антимошка" из стекловолокна. Размер рулона 1,4*30 M2 | описание товара | confirmed_document | |
| GoodsQuantity | 90 | кол-во наборов | confirmed_document | |
| goods_supplementary_quantity | 3780 | количество в доп.ед.изм (м2) | confirmed_document | |
| goods_supplementary_uom_name | м2 | наименование доп.ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м2 | единица измерения доп.количества | confirmed_document | |
| GrossWeightQuantity | 520.00 | брутто по строке | confirmed_document | из PL |
| NetWeightQuantity | 491.40 | нетто по строке | confirmed_document | из PL |
| Price | 3.4 | цена за м2 | confirmed_document | |
| TotalCost | 12852.00 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | цифровой код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | товарная марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_6

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsCode | 7019900095 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Сетка среднего размера "Антимошка" из стекловолокна. Размер рулона 1,6*30 M2 | описание товара | confirmed_document | |
| GoodsQuantity | 180 | кол-во наборов | confirmed_document | |
| goods_supplementary_quantity | 8640 | количество в доп.ед.изм (м2) | confirmed_document | |
| goods_supplementary_uom_name | м2 | наименование доп.ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м2 | единица измерения доп.количества | confirmed_document | |
| GrossWeightQuantity | 1190.00 | брутто по строке | confirmed_document | из PL |
| NetWeightQuantity | 1123.20 | нетто по строке | confirmed_document | из PL |
| Price | 3.4 | цена за м2 | confirmed_document | |
| TotalCost | 29376.00 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | цифровой код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | товарная марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

### InvoiceGoods_7

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsCode | 5804101000 | код ТН ВЭД | confirmed_document | |
| GoodsDescription | Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,6*30 M2 | описание товара | confirmed_document | |
| GoodsQuantity | 5 | кол-во наборов | confirmed_document | |
| goods_supplementary_quantity | 240 | количество в доп.ед.изм (м2) | confirmed_document | |
| goods_supplementary_uom_name | м2 | наименование доп.ед.изм | confirmed_document | |
| MeasureUnitQualifierName | м2 | единица измерения доп.количества | confirmed_document | |
| GrossWeightQuantity | 25.00 | брутто по строке | confirmed_document | из PL |
| NetWeightQuantity | 24.00 | нетто по строке | confirmed_document | из PL |
| Price | 28 | цена за м2 | confirmed_document | |
| TotalCost | 6720.00 | стоимость по строке | confirmed_document | |
| OriginCountryCode | 156 | цифровой код страны происхождения | confirmed_operator | |
| AdditionalGoodsDescription_Manufacturer | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | производитель | confirmed_operator | |
| AdditionalGoodsDescription_TradeMark | ОТСУТСТВУЕТ | товарная марка | confirmed_operator | |
| AdditionalGoodsDescription_GoodsMark | ОТСУТСТВУЕТ | товарный знак | confirmed_operator | |
| AdditionalGoodsDescription_GoodsModel | NOT APPLICABLE | модель | confirmed_operator | |

---

## document: packing_list

- **uqi_prefix**: formalized.packing_list_1
- **xml_target_root**: AltaE2PACK
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\PL на сетку .xlsx
- **file_name**: PL на сетку .xlsx
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GrossWeightQuantity | 3500.00 | общий вес брутто | confirmed_document | |
| NetWeightQuantity | 3302.00 | общий вес нетто | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | грузоотправитель | confirmed_document | |
| Consignor_ShortName | HEBEI LANGMAI IMPORT AND EXPORT CO.,LTD. | краткое наименование | confirmed_operator | совпадает с полным |
| Consignor_Address_CountryCode | CN | страна грузоотправителя alpha-2 | confirmed_document | |
| Consignor_Address_CounryName | Китай | страна грузоотправителя | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street | адрес | confirmed_document | |
| Consignee_OrganizationName | ООО «СКИФ» | грузополучатель | confirmed_document | |
| Consignee_ShortName | ООО «СКИФ» | краткое наименование | confirmed_operator | совпадает с полным |
| Consignee_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_Address_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_Address_CountryCode | RU | страна alpha-2 | confirmed_document | |
| Consignee_Address_CounryName | Россия | страна | confirmed_document | |
| Consignee_Address_Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_Address_City | г. Набережные Челны | город | confirmed_document | |
| Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | адрес | confirmed_document | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | место поставки | confirmed_document | |
| DeliveryTerms_DeliveryTermsNumericCode | 01 | числовой код условий | confirmed_operator | |
| DeliveryTerms_DeliveryTermsStringCode | EXW | строковый код условий | confirmed_operator | |
| DeliveryTerms_Contract_PrDocumentName | SALES CONTRACT | наименование контракта | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentNumber | LM-2553 | № контракта | confirmed_document | |
| DeliveryTerms_Contract_PrDocumentDate | 2025-07-02 | дата контракта | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentName | Commercial Invoice | наименование инвойса | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentNumber | LM-2591 | № инвойса | confirmed_document | |
| DeliveryTerms_Invoice_PrDocumentDate | 2025-10-30 | дата инвойса | confirmed_document | |
| DeliveryTerms_Registration_PrDocumentName | Упаковочный лист | наименование упаковочного | confirmed_operator | |
| DeliveryTerms_Registration_PrDocumentNumber | LM-2591 | № упаковочного | confirmed_operator | |
| DeliveryTerms_Registration_PrDocumentDate | 2025-10-30 | дата упаковочного | confirmed_operator | |
| registration_doc_name | Упаковочный лист | наименование для графы 44 | confirmed_operator | |
| registration_doc_number | LM-2591 | номер для графы 44 | confirmed_operator | |
| registration_doc_date | 2025-10-30 | дата для графы 44 | confirmed_operator | |

### Goods_1

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16. Материал полиэстер | описание строки | confirmed_document | |
| GoodsQuantity | 60 | кол-во мест/рулонов | confirmed_document | |
| GrossWeightQuantity | 855.00 | брутто по строке | confirmed_document | |
| NetWeightQuantity | 806.60 | нетто по строке | confirmed_document | |
| PackingInfo.PakingQuantity | 60 | кол-во упаковок | confirmed_operator | =GoodsQuantity |

### Goods_2

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16 | описание строки | confirmed_document | |
| GoodsQuantity | 30 | кол-во мест/рулонов | confirmed_document | |
| GrossWeightQuantity | 490.00 | брутто по строке | confirmed_document | |
| NetWeightQuantity | 460.80 | нетто по строке | confirmed_document | |
| PackingInfo.PakingQuantity | 30 | кол-во упаковок | confirmed_operator | =GoodsQuantity |

### Goods_3

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Сетка против пыльцы "Антипыльца" из полиэстера. Размер рулона 1,42*0,64*0,22 | описание строки | confirmed_document | |
| GoodsQuantity | 6 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 265.00 | брутто по строке | confirmed_document | |
| NetWeightQuantity | 252.00 | нетто по строке | confirmed_document | |
| PackingInfo.PakingQuantity | 6 | кол-во упаковок | confirmed_operator | =GoodsQuantity |

### Goods_4

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Сетка против пыльцы "Антипыльца" из полиэстера Размер рулона 1,62*0,64*0,23 | описание строки | confirmed_document | |
| GoodsQuantity | 3 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 155.00 | брутто по строке | confirmed_document | |
| NetWeightQuantity | 144.00 | нетто по строке | confirmed_document | |
| PackingInfo.PakingQuantity | 3 | кол-во упаковок | confirmed_operator | =GoodsQuantity |

### Goods_5

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Сетка среднего размера "Антимошка" из стекловолокна. Размер рулона 1,42*0,55*0,18 | описание строки | confirmed_document | |
| GoodsQuantity | 9 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 520.00 | брутто по строке | confirmed_document | |
| NetWeightQuantity | 491.40 | нетто по строке | confirmed_document | |
| PackingInfo.PakingQuantity | 9 | кол-во упаковок | confirmed_operator | =GoodsQuantity |

### Goods_6

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Сетка среднего размера "Антимошка" из стекловолокна. Размер рулона 1,62*0,55*18 | описание строки | confirmed_document | |
| GoodsQuantity | 18 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 1190.00 | брутто по строке | confirmed_document | |
| NetWeightQuantity | 1123.20 | нетто по строке | confirmed_document | |
| PackingInfo.PakingQuantity | 18 | кол-во упаковок | confirmed_operator | =GoodsQuantity |

### Goods_7

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Трехслойные сетки "Антипыльца" из полиэстера Размер рулона 1,72*0,35*0,31*1 | описание строки | confirmed_document | |
| GoodsQuantity | 1 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 25.00 | брутто по строке | confirmed_document | |
| NetWeightQuantity | 24.00 | нетто по строке | confirmed_document | |
| PackingInfo.PakingQuantity | 1 | кол-во упаковок | confirmed_operator | =GoodsQuantity |

### TransportMeans_1

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| Number | О157АО774 | регистрационный номер ТС | confirmed_operator | тягач |
| ModeCode | 31 | код вида транспорта | confirmed_operator | |
| NationalityCode | 000 | код национальности | confirmed_operator | |
| MoverIndicator | true | признак тягача | confirmed_operator | |

### TransportMeans_2

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| Number | ВТ374974 | регистрационный номер ТС | confirmed_operator | прицеп |
| ModeCode | 31 | код вида транспорта | confirmed_operator | |
| NationalityCode | 000 | код национальности | confirmed_operator | |
| MoverIndicator | false | признак прицепа | confirmed_operator | |

---

## document: cmr

- **uqi_prefix**: formalized.cmr_1
- **xml_target_root**: AltaE3CMR
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\СМР от СВХ.PNG
- **file_name**: СМР от СВХ.PNG
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| LanguageCode | RU | язык документа | confirmed_operator | |
| CMR_Choice | 1 | системный выбор Альты | confirmed_operator | |
| RegistrationDocument_RegID | 00378 | номер CMR | confirmed_document | |
| RegistrationDocument_DateInf | 2026-01-20 | дата CMR | confirmed_document | |
| RegistrationDocument_Place | Маньчжурия | место составления | confirmed_operator | |
| TrakingCargo_TakingCargoDate | 2026-01-20 | дата принятия груза | confirmed_document | |
| TrakingCargo_TakingCargoPlace_CountryCode | CN | страна принятия груза alpha-2 | confirmed_operator | |
| TrakingCargo_TakingCargoPlace_CounryName | Китай | страна принятия груза | confirmed_operator | |
| DeliveryPlace_CountryCode | RU | страна доставки alpha-2 | confirmed_operator | |
| DeliveryPlace_CounryName | Россия | страна доставки | confirmed_operator | |
| DeliveryTerms_DeliveryPlace | Naberezhnye Chelny | место поставки | confirmed_operator | |
| DeliveryTerms_DeliveryTermsStringCode | EXW | условия поставки | confirmed_operator | |
| GoodsQuantity | 127 | общее кол-во мест | confirmed_document | |
| CMRGoodsWeight_GrossWeightQuantity | 3500.00 | общий вес брутто | confirmed_document | |
| CMRTransport_PrimeMoverStateSignID | О157АО774 | гос. номер тягача | confirmed_document | |
| CMRTransport_TrailerStateSignID | ВТ374974 | гос. номер прицепа | confirmed_document | |
| Consignor_NameInf | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | наименование отправителя | confirmed_document | |
| Consignor_ShortName | HEBEI LANGMAI IMPORT & EXPORT CO., LTD. | краткое наименование | confirmed_operator | совпадает с полным |
| Consignor_PostalAddress_CountryCode | CN | страна alpha-2 | confirmed_document | |
| Consignor_Address_CounryName | Китай | страна | confirmed_document | |
| Consignor_Address_Region | Hebei | регион | confirmed_document | |
| Consignor_Address_City | Shijiazhuang | город | confirmed_document | |
| Consignor_Address_StreetHouse | No. 5 Gaodong street | адрес | confirmed_document | |
| Consignor_Guarantee_OrganizationName | ОТСУТСТВУЕТ | гарант отправителя | confirmed_operator | не требуется |
| Consignor_Guarantee_ShortName | | краткое наименование | confirmed_operator | отсутствует |
| Consignor_Guarantee_Address_CountryCode | | страна alpha-2 | confirmed_operator | отсутствует |
| Consignor_Guarantee_Address_CounryName | | страна | confirmed_operator | отсутствует |
| Consignor_Guarantee_Address_Region | | регион | confirmed_operator | отсутствует |
| Consignor_Guarantee_Address_City | | город | confirmed_operator | отсутствует |
| Consignor_Guarantee_Address_StreetHouse | | адрес | confirmed_operator | отсутствует |
| Consignee_NameInf | ООО «Скиф» | наименование получателя | confirmed_document | |
| Consignee_ShortName | ООО «Скиф» | краткое наименование | confirmed_operator | совпадает с полным |
| Consignee_OGRNID | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_INNID | 1650389298 | ИНН | confirmed_document | |
| Consignee_KPPCode | 165001001 | КПП | confirmed_document | |
| Consignee_PostalAddress_PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_PostalAddress_CountryCode | RU | страна alpha-2 | confirmed_document | |
| Consignee_Address_CounryName | Россия | страна | confirmed_document | |
| Consignee_Address_Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_Address_City | г. Набережные Челны | город | confirmed_document | |
| Consignee_Address_StreetHouse | Хлебный пр-д, д. 30, офис 211 | адрес | confirmed_document | |

### CMRGoods_1

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsNumeric | 1 | порядковый номер строки | confirmed_document | |
| GoodsDescription | Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025 | описание груза | confirmed_document | |
| GoodsNomenclatureCode | 5804101000 | код товара (ТН ВЭД) | confirmed_operator | |
| GoodsQuantity | 127 | кол-во мест | confirmed_document | |
| GrossWeightQuantity | 3500.00 | брутто | confirmed_document | |

#### GoodsPackingInfo

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| PackingCode | PX | код вида упаковки | confirmed_operator | |
| PakingQuantity | 127 | кол-во упаковок | confirmed_operator | |
| PackingDescription | ПОДДОН | описание упаковки | confirmed_operator | |

---

## document: payment_order_1

- **uqi_prefix**: formalized.payment_order_1
- **xml_target_root**: AltaPaymentOrder
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\платежки\currency_transfer_1_13.01.2026.pdf
- **file_name**: currency_transfer_1_13.01.2026.pdf
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| DocumentCode | 04023 | код вида документа для графы 44 | confirmed_operator | |
| PaymentModeCode | 0 | системный код способа платежа | confirmed_operator | |
| PaymentAmount | 63219.00 | сумма платежа (CNY) | confirmed_document | |
| TransactionKind | 01 | вид операции | confirmed_operator | |
| Priority | 5 | очередность | confirmed_operator | |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553, DATE:JULY 02.2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | назначение платежа | confirmed_document | |
| ValueSpelledOut | Шестьдесят три тысячи двести девятнадцать юаней 00/100 | сумма прописью | confirmed_document | |
| DocumentReference_PrDocumentNumber | 1 | номер платежного поручения | confirmed_document | |
| DocumentReference_PrDocumentDate | 2026-01-13 | дата платежного поручения | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | плательщик | confirmed_document | |
| Payer_INN | 1650389298 | ИНН плательщика | confirmed_document | |
| Payer_KPP | 165001001 | КПП плательщика | confirmed_operator | |
| Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) 044525411 | банк плательщика | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | получатель платежа | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SWIFT: VTBRCNSHXXX | банк получателя | confirmed_document | |

### PayerSign

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| PersonSurname | | фамилия | pending | не указана в документе |
| PersonName | | имя | pending | не указано в документе |

---

## document: payment_order_2

- **uqi_prefix**: formalized.payment_order_2
- **xml_target_root**: AltaPaymentOrder
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\платежки\currency_transfer_7_28.11.2025.pdf
- **file_name**: currency_transfer_7_28.11.2025.pdf
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| DocumentCode | 04023 | код вида документа для графы 44 | confirmed_operator | |
| PaymentModeCode | 0 | системный код способа платежа | confirmed_operator | |
| PaymentAmount | 34041.00 | сумма платежа (CNY) | confirmed_document | |
| TransactionKind | 01 | вид операции | confirmed_operator | |
| Priority | 5 | очередность | confirmed_operator | |
| Purpose | PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553, DATE:JULY 02.2025, INVOICE NO.: LM-2591, DATE: 2025.10.30 | назначение платежа | confirmed_document | |
| ValueSpelledOut | Тридцать четыре тысячи сорок один юань 00/100 | сумма прописью | confirmed_document | |
| DocumentReference_PrDocumentNumber | 7 | номер платежного поручения | confirmed_document | |
| DocumentReference_PrDocumentDate | 2025-11-28 | дата платежного поручения | confirmed_document | |
| Payer_OrganizationName | LLC SKIF | плательщик | confirmed_document | |
| Payer_INN | 1650389298 | ИНН плательщика | confirmed_document | |
| Payer_KPP | 165001001 | КПП плательщика | confirmed_operator | |
| Payer_Bank_BankName | ФИЛИАЛ "ЦЕНТРАЛЬНЫЙ" БАНКА ВТБ (ПАО) 044525411 | банк плательщика | confirmed_document | |
| Payee_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | получатель платежа | confirmed_document | |
| Payee_Bank_BankName | VTB BANK (PJSC) SHANGHAI BRANCH, SWIFT: VTBRCNSHXXX | банк получателя | confirmed_document | |

### PayerSign

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| PersonSurname | | фамилия | pending | не указана в документе |
| PersonName | | имя | pending | не указано в документе |

---

## document: service_invoice_1

- **uqi_prefix**: formalized.service_invoice_1
- **xml_target_root**: AltaServiceInvoice
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\Счет_№26-00378-tl_от_27-01-2026.pdf
- **file_name**: Счет_№26-00378-tl_от_27-01-2026.pdf
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| DocumentSign | 1 | системный признак документа | confirmed_operator | |
| TotalServiceCost | 2700.00 | итого по услугам | confirmed_document | |
| Currency | USD | валюта итого | confirmed_document | |
| ServiceProvider_Name | ООО «Трансмипериал» | исполнитель услуг | confirmed_document | |
| ServiceProvider_PaymentRequisitions.BankName | АО "Райффайзенбанк", г. Москва | банк исполнителя | confirmed_document | |
| ContractDetails_PrDocumentNumber | КООО/26651/М | № договора на услуги | confirmed_document | |
| ContractDetails_PrDocumentDate | 2025-05-13 | дата договора на услуги | confirmed_document | |
| PaymentDocument.PrDocumentNumber | ОТСУТСТВУЕТ | номер заказа | confirmed_operator | |
| PaymentDocument.PrDocumentDate | ОТСУТСТВУЕТ | дата заказа | confirmed_operator | |
| Registration_PrDocumentName | Счет на оплату | наименование счета | confirmed_document | |
| Registration_PrDocumentNumber | 26-00378-tl | номер счета | confirmed_document | |
| Registration_PrDocumentDate | 2026-01-27 | дата счета | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | грузоотправитель | confirmed_operator | совпадает с продавцом |
| Consignor_SubjectAddressDetails.PostalCode | | индекс | pending | |
| Consignor_SubjectAddressDetails.CountryCode | CN | страна alpha-2 | confirmed_document | |
| Consignor_SubjectAddressDetails.CounryName | Китай | страна | confirmed_document | |
| Consignor_SubjectAddressDetails.Region | Hebei | регион | confirmed_document | |
| Consignor_SubjectAddressDetails.Town | Hengshui | город | confirmed_document | |
| Consignor_SubjectAddressDetails.StreetHouse | | адрес | pending | |
| Consignee_OrganizationName | ООО "СКиФ" | грузополучатель | confirmed_document | |
| Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_RFOrganizationFeatures_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_RFOrganizationFeatures_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_SubjectAddressDetails.PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_SubjectAddressDetails.CountryCode | RU | страна alpha-2 | confirmed_document | |
| Consignee_SubjectAddressDetails.CounryName | Россия | страна | confirmed_document | |
| Consignee_SubjectAddressDetails.Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_SubjectAddressDetails.Town | г. Набережные Челны | город | confirmed_document | |
| Consignee_SubjectAddressDetails.StreetHouse | проезд Хлебный, д. 30 | адрес | confirmed_document | |
| Consignee_SubjectAddressDetails.House | 30 | дом | confirmed_operator | |
| Consignee_SubjectAddressDetails.Room | 211 | офис | confirmed_operator | |
| Signature_Choice | 1 | вариант подписи | confirmed_operator | |
| SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | фамилия руководителя | confirmed_document | |
| SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | инициалы руководителя | confirmed_document | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | фамилия бухгалтера | confirmed_document | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | инициалы бухгалтера | confirmed_document | |

### ServiceDescription_1

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Транспортно-экспедиционные услуги в международном сообщении по договору №КООО/26651/М от 13-05-2025 по транспортному заказу № 26-00378-tl от 12.01.2026 по маршруту: China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск) перевозка автотранспортом | описание услуги | confirmed_document | |
| CurrencyCode | USD | валюта строки | confirmed_document | |
| ServiceName | ОТСУТСТВУЕТ | наименование/маршрут | confirmed_operator | |
| TaxRate | 0 | ставка налога | confirmed_document | |
| TaxSum | 0.00 | сумма налога | confirmed_document | |
| ServiceCost_Amount | 1404.00 | стоимость строки | confirmed_document | |
| ServiceCost_Currency | USD | валюта стоимости | confirmed_document | |

### ServiceDescription_2

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны | описание услуги | confirmed_document | |
| CurrencyCode | USD | валюта строки | confirmed_document | |
| ServiceName | ОТСУТСТВУЕТ | наименование/маршрут | confirmed_operator | |
| TaxRate | 0 | ставка налога | confirmed_document | |
| TaxSum | 0.00 | сумма налога | confirmed_document | |
| ServiceCost_Amount | 1296.00 | стоимость строки | confirmed_document | |
| ServiceCost_Currency | USD | валюта стоимости | confirmed_document | |

---

## document: service_invoice_2

- **uqi_prefix**: formalized.service_invoice_2
- **xml_target_root**: AltaServiceInvoice
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\Счет_№26-00378-tl_1_от_14-01-2026.pdf
- **file_name**: Счет_№26-00378-tl_1_от_14-01-2026.pdf
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| DocumentSign | 1 | системный признак документа | confirmed_operator | |
| TotalServiceCost | 910.34 | итого по услугам | confirmed_document | |
| Currency | RUB | валюта итого | confirmed_document | |
| ServiceProvider_Name | ООО «Транснипериал» | исполнитель услуг | confirmed_document | |
| ServiceProvider_PaymentRequisitions.BankName | АО "Райффайзенбанк", г. Москва | банк исполнителя | confirmed_document | |
| ContractDetails_PrDocumentNumber | КООО/26651/М | № договора на услуги | confirmed_document | |
| ContractDetails_PrDocumentDate | 2025-05-13 | дата договора на услуги | confirmed_document | |
| PaymentDocument.PrDocumentNumber | ОТСУТСТВУЕТ | номер заказа | confirmed_operator | |
| PaymentDocument.PrDocumentDate | ОТСУТСТВУЕТ | дата заказа | confirmed_operator | |
| Registration_PrDocumentName | Счет на оплату | наименование счета | confirmed_document | |
| Registration_PrDocumentNumber | 26-00378-tl/1 | номер счета | confirmed_document | |
| Registration_PrDocumentDate | 2026-01-14 | дата счета | confirmed_document | |
| Consignor_OrganizationName | HEBEI LANGMAI IMPORT AND EXPORT CO., LTD | грузоотправитель | confirmed_operator | совпадает с продавцом |
| Consignee_OrganizationName | ООО "СКиФ" | грузополучатель | confirmed_document | |
| Consignee_RFOrganizationFeatures_OGRN | 1201600020390 | ОГРН | confirmed_document | |
| Consignee_RFOrganizationFeatures_INN | 1650389298 | ИНН | confirmed_document | |
| Consignee_RFOrganizationFeatures_KPP | 165001001 | КПП | confirmed_document | |
| Consignee_SubjectAddressDetails.PostalCode | 423800 | индекс | confirmed_document | |
| Consignee_SubjectAddressDetails.CountryCode | RU | страна alpha-2 | confirmed_document | |
| Consignee_SubjectAddressDetails.CounryName | Россия | страна | confirmed_document | |
| Consignee_SubjectAddressDetails.Region | Республика Татарстан | регион | confirmed_document | |
| Consignee_SubjectAddressDetails.Town | г. Набережные Челны | город | confirmed_document | |
| Consignee_SubjectAddressDetails.StreetHouse | проезд Хлебный, д. 30 | адрес | confirmed_document | |
| Consignee_SubjectAddressDetails.House | 30 | дом | confirmed_operator | |
| Consignee_SubjectAddressDetails.Room | 211 | офис | confirmed_operator | |
| Signature_Choice | 1 | вариант подписи | confirmed_operator | |
| SignatureDirectorChiefAccountant_Director_PersonSurname | Климович | фамилия руководителя | confirmed_document | |
| SignatureDirectorChiefAccountant_Director_PersonName | Л.А. | инициалы руководителя | confirmed_document | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonSurname | Лехно | фамилия бухгалтера | confirmed_document | |
| SignatureDirectorChiefAccountant_ChiefAccountant_PersonName | О.А. | инициалы бухгалтера | confirmed_document | |

### ServiceDescription_1

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| GoodsDescription | Возмещение за добровольное страхование груза по договору №КООО/26651/М от 13-05-2025 по заявлению на страхование грузов №26-00378-tl от 14.01.2026 | описание услуги | confirmed_document | |
| CurrencyCode | RUB | валюта строки | confirmed_document | |
| ServiceName | ОТСУТСТВУЕТ | наименование | confirmed_operator | |
| TaxRate | 0 | ставка налога | confirmed_document | без НДС |
| TaxSum | 0.00 | сумма налога | confirmed_document | |
| ServiceCost_Amount | 910.34 | стоимость строки | confirmed_document | |
| ServiceCost_Currency | RUB | валюта стоимости | confirmed_document | |

---

## document: tech_description_1

- **uqi_prefix**: formalized.tech_description_1
- **xml_target_root**: AltaFreeDoc
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\техничка Антикот, антипыльца антимошка .docx
- **file_name**: техничка Антикот, антипыльца антимошка .docx
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| DocumentCode | 05999 | код вида документа для графы 44 | confirmed_document | |
| DocumentHead_DocumentName | Технические характеристики | наименование техописания | confirmed_document | |
| DocumentHead_DocumentDate | 2025-10-30 | дата техописания | confirmed_operator | |
| DocumentHead_DocumentNumber | Б/Н | номер техописания | confirmed_operator | |

### DocumentBody_TextSection

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| TextPara_1 | link:...техничка Антикот, антипыльца антимошка .docx | технический текст | confirmed_document | |

---

## document: tech_description_2

- **uqi_prefix**: formalized.tech_description_2
- **xml_target_root**: AltaFreeDoc
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\техничка .pdf
- **file_name**: техничка .pdf
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| DocumentCode | 05999 | код вида документа для графы 44 | confirmed_document | |
| DocumentHead_DocumentName | Технические характеристики | наименование техописания | confirmed_document | |
| DocumentHead_DocumentDate | 2025-10-30 | дата техописания | confirmed_operator | |
| DocumentHead_DocumentNumber | Б/Н | номер техописания | confirmed_operator | |

### DocumentBody_TextSection

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| TextPara_1 | link:...техничка .pdf | технический текст | confirmed_document | дублирует tech_description_1 |

---

# РАЗДЕЛ II: non_formalized

---

## document: svh

- **uqi_prefix**: non_formalized.svh_1
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\ДО 14431420260204161621.PNG
- **file_name**: ДО 14431420260204161621.PNG
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| number | 0000080 | № ДО-1 | confirmed_document | |
| date | 2026-02-03 | дата ДО-1 | confirmed_document | |
| warehouse_license_number | 10404/141210/10092/5 | номер лицензии СВХ | confirmed_document | |
| warehouse_license_date | 2025-09-18 | дата лицензии СВХ | confirmed_document | |
| actual_gross_weight | 3500.00 | фактический вес по весам | confirmed_document | |
| actual_places | 127 | фактическое количество мест | confirmed_document | |
| transport_reg_number | О157АО774 / ВТ374974 | номер ТС при въезде | confirmed_document | |

### goods_1

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| tnved | 7019900095 | код товара | confirmed_document | |
| places | 27 | кол-во грузовых мест | confirmed_document | |
| gross_weight_kg | 1710.00 | вес брутто | confirmed_document | |
| cost | 42228.00 | стоимость | confirmed_document | |
| currency_code | CNY | валюта | confirmed_document | |

### goods_2

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| tnved | 5804101000 | код товара | confirmed_document | |
| places | 100 | кол-во грузовых мест | confirmed_document | |
| gross_weight_kg | 1790.00 | вес брутто | confirmed_document | |
| cost | 55032.00 | стоимость | confirmed_document | |
| currency_code | CNY | валюта | confirmed_document | |

---

## document: svh_additional_sheet

- **uqi_prefix**: non_formalized.svh_additional_sheet_1
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\ДО доп 14431520260204161645.PNG
- **file_name**: ДО доп 14431520260204161645.PNG
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| number | 1 | № доп.листа | confirmed_document | |
| date | 2026-02-03 | дата доп.листа | confirmed_document | |
| actual_gross_weight | | фактический вес | confirmed_document | входит в основной ДО-1 |
| actual_places | | фактическое количество мест | confirmed_document | входит в основной ДО-1 |
| transport_reg_number | | номер ТС | confirmed_document | |
| svh_address_region | Республика Татарстан | регион СВХ | confirmed_document | |
| svh_address_city | г. Набережные Челны | город СВХ | confirmed_document | |
| svh_address_street_house | Производственный пр-д, д. 45 | адрес СВХ | confirmed_document | |
| svh_customs_code | 10404083 | код таможенного органа | confirmed_document | |

---

## document: td

- **uqi_prefix**: non_formalized.td_1
- **full_path**: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\ТД 10719110_240126_5011363_reg00378тд.pdf
- **file_name**: ТД 10719110_240126_5011363_reg00378тд.pdf
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| number | 10719110/240126/5011363 | номер ТД | confirmed_document | |
| date | 2026-01-24 | дата ТД | confirmed_document | |
| customs_post_code | 10404083 | код таможенного органа | confirmed_document | |
| customs_post_name | ОТО и ТК №3 т/п Набережночелнинский | наименование таможенного органа | confirmed_document | |
| transport_reg_number | 0157A0774 / B1734974 | ТС по ТД | confirmed_document | |

---

## document: master_data

- **uqi_prefix**: non_formalized.master_data_1
- **full_path**: alta\stable_source\ (паспорт, доверенность, ЕГРЮЛ)
- **status**: confirmed

| field | value | description | status | note |
|-------|-------|-------------|--------|------|
| declarant_name | ООО «СКИФ» | наименование декларанта | confirmed_document | |
| declarant_ogrn | 1201600020390 | ОГРН | confirmed_document | |
| declarant_inn | 1650389298 | ИНН | confirmed_document | |
| declarant_kpp | 165001001 | КПП | confirmed_document | |
| declarant_address_postal_code | 423800 | индекс | confirmed_document | |
| declarant_address_country_code | RU | страна alpha-2 | confirmed_document | |
| declarant_address_country_name | Россия | страна | confirmed_document | |
| declarant_address_region | Республика Татарстан | регион | confirmed_document | |
| declarant_address_city | г. Набережные Челны | город | confirmed_document | |
| declarant_address_street | проезд Хлебный | улица | confirmed_document | |
| declarant_address_building | 30 | дом | confirmed_document | |
| declarant_address_room | 211 | офис | confirmed_document | |
| declarant_phone | +7 937 779-26-56 | телефон | confirmed_document | из контракта |
| declarant_email | | e-mail | pending | |
| representative_last_name | Садыков | фамилия | confirmed_document | из доверенности |
| representative_first_name | Марсель | имя | confirmed_document | |
| representative_middle_name | Рамилевич | отчество | confirmed_document | |
| representative_position | менеджер ВЭД | должность | confirmed_document | |
| representative_phone | +7-927-030-70-07 | телефон | confirmed_document | |
| representative_email | | e-mail | pending | |
| representative_passport_code | RU01001 | код документа | confirmed_document | |
| representative_passport_name | ПАСРФ | наименование документа | confirmed_document | |
| representative_passport_series | 63 09 | серия | confirmed_document | |
| representative_passport_number | 449948 | номер | confirmed_document | |
| representative_passport_date | 2010-03-11 | дата выдачи | confirmed_document | |
| representative_passport_issuer | Отделением УФМС России по Саратовской области г. Саратов | кем выдан | confirmed_document | |
| representative_authority_doc_name | Доверенность | наименование документа полномочий | confirmed_document | |
| representative_authority_doc_number | 1 | № доверенности | confirmed_document | |
| representative_authority_doc_date_from | 2026-02-01 | дата начала | confirmed_document | |
| representative_authority_doc_date_to | 2026-12-31 | дата окончания | confirmed_document | |
| note | Данные из доверенности №1 от 01.02.2026, паспорт, ЕГРЮЛ | пояснения | confirmed_document | |

---

# РАЗДЕЛ III: Нерешенные вопросы

## Pending поля

### formalized.payment_order_1.PayerSign.PersonSurname
- **question**: В заявлении на перевод не указана фамилия подписанта. Требуется ли заполнить для импорта в Альту?

### formalized.payment_order_1.PayerSign.PersonName
- **question**: В заявлении на перевод не указано имя подписанта. Требуется ли заполнить для импорта в Альту?

### formalized.payment_order_2.PayerSign.PersonSurname
- **question**: В заявлении на перевод не указана фамилия подписанта. Требуется ли заполнить для импорта в Альту?

### formalized.payment_order_2.PayerSign.PersonName
- **question**: В заявлении на перевод не указано имя подписанта. Требуется ли заполнить для импорта в Альту?

### formalized.service_invoice_1.Consignor_SubjectAddressDetails.PostalCode
- **question**: Индекс грузоотправителя (Hengshui, China) не указан в счете. Требуется ли?

### formalized.service_invoice_1.Consignor_SubjectAddressDetails.StreetHouse
- **question**: Адрес грузоотправителя (Hengshui, China) не указан в счете. Требуется ли?

### non_formalized.master_data_1.declarant_email
- **question**: E-mail декларанта не указан в документах. Требуется ли для ДТ?

### non_formalized.master_data_1.representative_email
- **question**: E-mail представителя не указан в документах. Требуется ли для ДТ?

---

## [Общий]

### Структура товаров для ДТ
- **question**: Инвойс содержит 7 товарных строк, но ТД и ДО-1 агрегируют товары по 2 кодам ТН ВЭД (5804101000 и 7019900095). Подтверждаешь, что целевая структура ДТ — 7 товаров как в инвойсе?

### Кодировка stable_source
- **question**: XML-файлы из stable_source (паспорт, доверенность, ЕГРЮЛ) содержат нечитаемые символы (кодировка windows-1251). Данные извлечены из md-версий и контракта. Требуется ли перечитать оригинальные XML с явным указанием кодировки?