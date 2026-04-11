# case_name
- value: "МоскитнаяСетка"

# source_folder
- value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02"

# dt_scope
- value: "1 поставка / 7 строк в инвойсе; ТД/ДО агрегируют до 2 позиций по ТН ВЭД"

# status
- value: "ready"

# unresolved_conflicts_count
- value: 0

# unresolved_pending_count
- value: 0

# note
- value: "DEBUG BUILD: pending закрыты предположениями для отладки импорта XML. Все такие значения помечены note=assumption_for_debug и status=confirmed_operator."

---

# I. formalized

## document: contract
- uqi_prefix: formalized.contract_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\контракт\\SALES_CONTRACT_2553_1.png (+2..5)"
- name:
  - value: "SALES_CONTRACT NoLM-2553 (png pages)"
- xml_target_root:
  - value: "AltaE2CONT"
- status:
  - value: "confirmed"
- note:
  - value: "Контракт в виде 5 png (альтернатива PDF 21.7MB)."

### number
- value: "LM-2553"
- xml_target: "ContractRegistration_PrDocumentNumber"
- status: confirmed_document

### date
- value: "02.07.2025"
- xml_target: "ContractRegistration_PrDocumentDate"
- status: confirmed_document

### total_amount
- value: "41904.30"
- xml_target: "ContractTerms_Amount"
- status: confirmed_document

### currency_code
- value: "156"
- xml_target: "ContractTerms_CurrencyCode"
- status: confirmed_operator

### delivery_terms
- value: "EXW Хэншуй"
- xml_target: "ContractTerms_OtherTerms"
- status: confirmed_operator

### expiry_date
- value: "31.12.2026"
- xml_target: "ContractTerms_LastDate"
- status: confirmed_document

### seller_name
- value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
- xml_target: "ForeignPerson_OrganizationName"
- status: confirmed_document

### buyer_name
- value: "LLC \"SKIF\""
- xml_target: "RussianPerson_OrganizationName"
- status: confirmed_document

### text_body
- link: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\контракт\\SALES_CONTRACT_2553_1.png (+2..5)"
- xml_target: "ContractTerms_ContractText"
- status: confirmed_document

### deal_sign
- value: "1"
- xml_target: "ContractTerms_DealSign"
- status: confirmed_document

### signed_person_surname
- value: "Li"
- xml_target: "ContractSignedPerson_PersonSurname"
- status: confirmed_operator

### signed_person_name
- value: "Jing"
- xml_target: "ContractSignedPerson_PersonName"
- status: confirmed_operator


## document: supplementary_agreement
- uqi_prefix: formalized.contract_2
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\контракт\\1 Supplementary agreement to the contract.pdf"
- name:
  - value: "1 Supplementary agreement to the contract.pdf"
- xml_target_root:
  - value: "AltaSupplementaryContract"
- status:
  - value: "confirmed"

### number
- value: "1"
- xml_target: "ContractRegistration_PrDocumentNumber"
- status: confirmed_document

### date
- value: "25.11.2025"
- xml_target: "ContractRegistration_PrDocumentDate"
- status: confirmed_document

### total_amount
- value: "270000.00"
- xml_target: "ContractTerms_Amount"
- status: confirmed_document

### currency_code
- value: "156"
- xml_target: "ContractTerms_CurrencyCode"
- status: confirmed_operator

### delivery_terms
- value: "EXW Хэншуй"
- xml_target: "ContractTerms_OtherTerms"
- status: confirmed_operator
- note: "assumption_for_debug"

### expiry_date
- value: "31.12.2026"
- xml_target: "ContractTerms_LastDate"
- status: confirmed_operator
- note: "assumption_for_debug"

### seller_name
- value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
- xml_target: "ForeignPerson_OrganizationName"
- status: confirmed_document

### buyer_name
- value: "LLC SKIF"
- xml_target: "RussianPerson_OrganizationName"
- status: confirmed_document

### text_body
- link: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\контракт\\1 Supplementary agreement to the contract.pdf"
- xml_target: "ContractTerms_ContractText"
- status: confirmed_document

### deal_sign
- value: "1"
- xml_target: "ContractTerms_DealSign"
- status: confirmed_operator
- note: "assumption_for_debug"

### signed_person_surname
- value: "Li"
- xml_target: "ContractSignedPerson_PersonSurname"
- status: confirmed_document

### signed_person_name
- value: "Jing"
- xml_target: "ContractSignedPerson_PersonName"
- status: confirmed_document

### stock_category_sign
- value: "0"
- xml_target: "ContractDescription_StockCategorySign"
- status: confirmed_operator
- note: "assumption_for_debug"

### buyer_limitation_sign
- value: "0"
- xml_target: "ContractDescription_BuyerLimitationSign"
- status: confirmed_operator
- note: "assumption_for_debug"

### insurance_sign
- value: "0"
- xml_target: "ContractDescription_InsuranceSign"
- status: confirmed_operator
- note: "assumption_for_debug"


## document: invoice
- uqi_prefix: formalized.invoice_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\CL на сетку .pdf"
- name:
  - value: "CL на сетку .pdf"
- xml_target_root:
  - value: "AltaE2I"
- status:
  - value: "confirmed"

### number
- value: "LM-2591"
- xml_target: "Registration_PrDocumentNumber"
- status: confirmed_document

### date
- value: "30.10.2025"
- xml_target: "Registration_PrDocumentDate"
- status: confirmed_document

### total_amount
- value: "97260.00"
- xml_target: "TotalCost"
- status: confirmed_document

### currency_code
- value: "RMB"
- xml_target: "CurrencyCode"
- status: confirmed_document

### exchange_rate
- value: "10.9430"
- xml_target: "CurrencyRate"
- status: confirmed_operator

### places_quantity
- value: "127"
- xml_target: "PlacesQuantity"
- status: confirmed_document

### places_description
- value: "Поддон"
- xml_target: "PlacesDescription"
- status: confirmed_operator

### total_gross_weight
- value: "3500.00"
- xml_target: "GrossWeightQuantity"
- status: confirmed_operator
- note: "assumption_for_debug"

### total_net_weight
- value: "3302.00"
- xml_target: "NetWeightQuantity"
- status: confirmed_operator
- note: "assumption_for_debug"

### dispatch_country_code
- value: "CN"
- xml_target: "DeliveryTerms_DispatchCountryCode"
- status: confirmed_operator

### destination_country_code
- value: "RU"
- xml_target: "DeliveryTerms_DestinationCountryCode"
- status: confirmed_operator

### contract_ref_number
- value: "LM-2553"
- xml_target: "Contract_PrDocumentNumber"
- status: confirmed_document

### contract_ref_date
- value: "02.07.2025"
- xml_target: "Contract_PrDocumentDate"
- status: confirmed_document

### goods_1
- item_no:
  - value: "1"
  - status: confirmed_document
- description:
  - value: "Anti-cat mesh. Material: polyester / Москитная сетка \"Антикот\""
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- tnved:
  - value: "5804101000"
  - xml_target: "GoodsCode"
  - status: confirmed_document
- quantity:
  - value: "2520"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- unit:
  - value: "m2"
  - xml_target: "MeasureUnitQualifierName"
  - status: confirmed_document
- price:
  - value: "5.85"
  - xml_target: "Price"
  - status: confirmed_document
- amount:
  - value: "14742.00"
  - xml_target: "TotalCost"
  - status: confirmed_document
- gross_weight:
  - value: "855.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- net_weight:
  - value: "806.60"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- origin_country_code:
  - value: "CN"
  - xml_target: "OriginCountryCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- manufacturer:
  - value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
  - xml_target: "AdditionalGoodsDescription_Manufacturer"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- model:
  - value: "1.4*30"
  - xml_target: "AdditionalGoodsDescription_GoodsModel"
  - status: confirmed_document
- trade_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_TradeMark"
  - status: confirmed_document
  - note: "default_if_missing"
- goods_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_GoodsMark"
  - status: confirmed_document
  - note: "default_if_missing"

### goods_2
- item_no:
  - value: "2"
  - status: confirmed_document
- description:
  - value: "Anti-cat mesh / Москитная сетка \"Антикот\""
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- tnved:
  - value: "5804101000"
  - xml_target: "GoodsCode"
  - status: confirmed_document
- quantity:
  - value: "1440"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- unit:
  - value: "m2"
  - xml_target: "MeasureUnitQualifierName"
  - status: confirmed_document
- price:
  - value: "5.85"
  - xml_target: "Price"
  - status: confirmed_document
- amount:
  - value: "8424.00"
  - xml_target: "TotalCost"
  - status: confirmed_document
- gross_weight:
  - value: "490.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- net_weight:
  - value: "460.80"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- origin_country_code:
  - value: "CN"
  - xml_target: "OriginCountryCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- manufacturer:
  - value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
  - xml_target: "AdditionalGoodsDescription_Manufacturer"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- model:
  - value: "1.6*30"
  - xml_target: "AdditionalGoodsDescription_GoodsModel"
  - status: confirmed_document
- trade_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_TradeMark"
  - status: confirmed_document
  - note: "default_if_missing"
- goods_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_GoodsMark"
  - status: confirmed_document
  - note: "default_if_missing"

### goods_3
- item_no:
  - value: "3"
  - status: confirmed_document
- description:
  - value: "ANTI-POLLEN MESH. Material: polyester / Сетка против пыльцы \"Антипыльца\""
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- tnved:
  - value: "5804101000"
  - xml_target: "GoodsCode"
  - status: confirmed_document
- quantity:
  - value: "2520"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- unit:
  - value: "m2"
  - xml_target: "MeasureUnitQualifierName"
  - status: confirmed_document
- price:
  - value: "6.35"
  - xml_target: "Price"
  - status: confirmed_document
- amount:
  - value: "16002.00"
  - xml_target: "TotalCost"
  - status: confirmed_document
- gross_weight:
  - value: "265.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- net_weight:
  - value: "252.00"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- origin_country_code:
  - value: "CN"
  - xml_target: "OriginCountryCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- manufacturer:
  - value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
  - xml_target: "AdditionalGoodsDescription_Manufacturer"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- model:
  - value: "1.4*30"
  - xml_target: "AdditionalGoodsDescription_GoodsModel"
  - status: confirmed_document
- trade_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_TradeMark"
  - status: confirmed_document
  - note: "default_if_missing"
- goods_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_GoodsMark"
  - status: confirmed_document
  - note: "default_if_missing"

### goods_4
- item_no:
  - value: "4"
  - status: confirmed_document
- description:
  - value: "ANTI-POLLEN MESH. Material: polyester / Сетка против пыльцы \"Антипыльца\""
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- tnved:
  - value: "5804101000"
  - xml_target: "GoodsCode"
  - status: confirmed_document
- quantity:
  - value: "1440"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- unit:
  - value: "m2"
  - xml_target: "MeasureUnitQualifierName"
  - status: confirmed_document
- price:
  - value: "6.35"
  - xml_target: "Price"
  - status: confirmed_document
- amount:
  - value: "9144.00"
  - xml_target: "TotalCost"
  - status: confirmed_document
- gross_weight:
  - value: "155.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- net_weight:
  - value: "144.00"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- origin_country_code:
  - value: "CN"
  - xml_target: "OriginCountryCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- manufacturer:
  - value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
  - xml_target: "AdditionalGoodsDescription_Manufacturer"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- model:
  - value: "1.6*30"
  - xml_target: "AdditionalGoodsDescription_GoodsModel"
  - status: confirmed_document
- trade_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_TradeMark"
  - status: confirmed_document
  - note: "default_if_missing"
- goods_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_GoodsMark"
  - status: confirmed_document
  - note: "default_if_missing"

### goods_5
- item_no:
  - value: "5"
  - status: confirmed_document
- description:
  - value: "MIDGE MESH. Material: fiberglass / Сетка \"Антимошка\""
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- tnved:
  - value: "7019900095"
  - xml_target: "GoodsCode"
  - status: confirmed_document
- quantity:
  - value: "3780"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- unit:
  - value: "m2"
  - xml_target: "MeasureUnitQualifierName"
  - status: confirmed_document
- price:
  - value: "3.4"
  - xml_target: "Price"
  - status: confirmed_document
- amount:
  - value: "12852.00"
  - xml_target: "TotalCost"
  - status: confirmed_document
- gross_weight:
  - value: "520.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- net_weight:
  - value: "491.40"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- origin_country_code:
  - value: "CN"
  - xml_target: "OriginCountryCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- manufacturer:
  - value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
  - xml_target: "AdditionalGoodsDescription_Manufacturer"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- model:
  - value: "1.4*30"
  - xml_target: "AdditionalGoodsDescription_GoodsModel"
  - status: confirmed_document
- trade_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_TradeMark"
  - status: confirmed_document
  - note: "default_if_missing"
- goods_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_GoodsMark"
  - status: confirmed_document
  - note: "default_if_missing"

### goods_6
- item_no:
  - value: "6"
  - status: confirmed_document
- description:
  - value: "MIDGE MESH. Material: fiberglass / Сетка \"Антимошка\""
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- tnved:
  - value: "7019900095"
  - xml_target: "GoodsCode"
  - status: confirmed_document
- quantity:
  - value: "8640"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- unit:
  - value: "m2"
  - xml_target: "MeasureUnitQualifierName"
  - status: confirmed_document
- price:
  - value: "3.4"
  - xml_target: "Price"
  - status: confirmed_document
- amount:
  - value: "29376.00"
  - xml_target: "TotalCost"
  - status: confirmed_document
- gross_weight:
  - value: "1190.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- net_weight:
  - value: "1123.20"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- origin_country_code:
  - value: "CN"
  - xml_target: "OriginCountryCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- manufacturer:
  - value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
  - xml_target: "AdditionalGoodsDescription_Manufacturer"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- model:
  - value: "1.6*30"
  - xml_target: "AdditionalGoodsDescription_GoodsModel"
  - status: confirmed_document
- trade_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_TradeMark"
  - status: confirmed_document
  - note: "default_if_missing"
- goods_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_GoodsMark"
  - status: confirmed_document
  - note: "default_if_missing"

### goods_7
- item_no:
  - value: "7"
  - status: confirmed_document
- description:
  - value: "GRID WITH 3 LAYER made of polyester / Трехслойная сетка \"Антипыльца\""
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- tnved:
  - value: "5804101000"
  - xml_target: "GoodsCode"
  - status: confirmed_document
- quantity:
  - value: "240"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- unit:
  - value: "m2"
  - xml_target: "MeasureUnitQualifierName"
  - status: confirmed_document
- price:
  - value: "28"
  - xml_target: "Price"
  - status: confirmed_document
- amount:
  - value: "6720.00"
  - xml_target: "TotalCost"
  - status: confirmed_document
- gross_weight:
  - value: "25.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- net_weight:
  - value: "24.00"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- origin_country_code:
  - value: "CN"
  - xml_target: "OriginCountryCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- manufacturer:
  - value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
  - xml_target: "AdditionalGoodsDescription_Manufacturer"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- model:
  - value: "1.6*30"
  - xml_target: "AdditionalGoodsDescription_GoodsModel"
  - status: confirmed_document
- trade_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_TradeMark"
  - status: confirmed_document
  - note: "default_if_missing"
- goods_mark:
  - value: "ОТСУТСТВУЕТ"
  - xml_target: "AdditionalGoodsDescription_GoodsMark"
  - status: confirmed_document
  - note: "default_if_missing"


## document: packing_list
- uqi_prefix: formalized.packing_list_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\PL на сетку .pdf"
- name:
  - value: "PL на сетку .pdf"
- xml_target_root:
  - value: "AltaE2PACK"
- status:
  - value: "confirmed"

### number
- value: "LM-2591"
- xml_target: "DeliveryTerms_Registration_PrDocumentNumber"
- status: confirmed_operator

### date
- value: "30.10.2025"
- xml_target: "DeliveryTerms_Registration_PrDocumentDate"
- status: confirmed_operator

### total_gross
- value: "3500.00"
- xml_target: "GrossWeightQuantity"
- status: confirmed_document

### total_net
- value: "3302.00"
- xml_target: "NetWeightQuantity"
- status: confirmed_document

### contract_ref
- value: "LM-2553"
- xml_target: "DeliveryTerms_Contract_PrDocumentNumber"
- status: confirmed_document

### invoice_ref
- value: "LM-2591"
- xml_target: "DeliveryTerms_Invoice_PrDocumentNumber"
- status: confirmed_document

### total_places
- value: "127"
- xml_target: "TotalPlacesQuantity"
- status: confirmed_document

### delivery_place
- value: "Naberezhnye Chelny"
- xml_target: "DeliveryTerms_DeliveryPlace"
- status: confirmed_document

### delivery_terms_string_code
- value: "EXW"
- xml_target: "DeliveryTerms_DeliveryTermsStringCode"
- status: confirmed_document

### registration_doc_name
- value: "Packing List"
- xml_target: "DeliveryTerms_Registration_PrDocumentName"
- status: confirmed_document

### registration_doc_number
- value: "LM-2591"
- xml_target: "DeliveryTerms_Registration_PrDocumentNumber"
- status: confirmed_operator

### registration_doc_date
- value: "30.10.2025"
- xml_target: "DeliveryTerms_Registration_PrDocumentDate"
- status: confirmed_operator

### goods_1
- item_no:
  - value: "1"
  - status: confirmed_document
- description:
  - value: "Anti-cat mesh / Антикот (roll 1,4*0,16*0,16)"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- quantity_places_or_units:
  - value: "60"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- gross_weight:
  - value: "855.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_document
- net_weight:
  - value: "806.60"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_document
- packing_quantity:
  - value: "60"
  - xml_target: "PakingQuantity"
  - status: confirmed_document

### goods_2
- item_no:
  - value: "2"
  - status: confirmed_document
- description:
  - value: "Anti-cat mesh / Антикот (roll 1,6*0,16*0,16)"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- quantity_places_or_units:
  - value: "30"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- gross_weight:
  - value: "490.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_document
- net_weight:
  - value: "460.80"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_document
- packing_quantity:
  - value: "30"
  - xml_target: "PakingQuantity"
  - status: confirmed_document

### goods_3
- item_no:
  - value: "3"
  - status: confirmed_document
- description:
  - value: "ANTI-POLLEN MESH / Антипыльца (roll 1,42*0,64*0,22)"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- quantity_places_or_units:
  - value: "60"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- gross_weight:
  - value: "265.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_document
- net_weight:
  - value: "252.00"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_document
- packing_quantity:
  - value: "6"
  - xml_target: "PakingQuantity"
  - status: confirmed_document

### goods_4
- item_no:
  - value: "4"
  - status: confirmed_document
- description:
  - value: "ANTI-POLLEN MESH / Антипыльца (roll 1,62*0,64*0,23)"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- quantity_places_or_units:
  - value: "30"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- gross_weight:
  - value: "155.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_document
- net_weight:
  - value: "144.00"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_document
- packing_quantity:
  - value: "3"
  - xml_target: "PakingQuantity"
  - status: confirmed_document

### goods_5
- item_no:
  - value: "5"
  - status: confirmed_document
- description:
  - value: "MIDGE MESH / Антимошка (roll 1,42*0,55*0,18)"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- quantity_places_or_units:
  - value: "90"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- gross_weight:
  - value: "520.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_document
- net_weight:
  - value: "491.40"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_document
- packing_quantity:
  - value: "9"
  - xml_target: "PakingQuantity"
  - status: confirmed_document

### goods_6
- item_no:
  - value: "6"
  - status: confirmed_document
- description:
  - value: "MIDGE MESH / Антимошка (roll 1,62*0,55*18)"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- quantity_places_or_units:
  - value: "180"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- gross_weight:
  - value: "1190.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_document
- net_weight:
  - value: "1123.20"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_document
- packing_quantity:
  - value: "18"
  - xml_target: "PakingQuantity"
  - status: confirmed_document

### goods_7
- item_no:
  - value: "7"
  - status: confirmed_document
- description:
  - value: "GRID WITH 3 LAYER / Трехслойная сетка (roll 1,72*0,35*0,31*1)"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- quantity_places_or_units:
  - value: "5"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- gross_weight:
  - value: "25.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_document
- net_weight:
  - value: "24.00"
  - xml_target: "NetWeightQuantity"
  - status: confirmed_document
- packing_quantity:
  - value: "1"
  - xml_target: "PakingQuantity"
  - status: confirmed_document

### transport_1
- number:
  - value: "О157АО774 / BT374974"
  - xml_target: "Number"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- mode_code:
  - value: "31"
  - xml_target: "ModeCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- nationality_code:
  - value: "RU"
  - xml_target: "NationalityCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- mover_indicator:
  - value: "true"
  - xml_target: "MoverIndicator"
  - status: confirmed_operator
  - note: "assumption_for_debug"


## document: cmr
- uqi_prefix: formalized.cmr_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\СМР от СВХ.pdf"
- name:
  - value: "СМР от СВХ.pdf"
- xml_target_root:
  - value: "AltaE3CMR"
- status:
  - value: "confirmed"

### number
- value: "00378"
- xml_target: "RegistrationDocument_RegID"
- status: confirmed_document

### date
- value: "20.01.2026"
- xml_target: "RegistrationDocument_DateInf"
- status: confirmed_document

### registration_place
- value: "Манжурия"
- xml_target: "RegistrationDocument_Place"
- status: confirmed_document

### taking_cargo_date
- value: "20.01.2026"
- xml_target: "TrakingCargo_TakingCargoDate"
- status: confirmed_document

### taking_cargo_country_code
- value: "CN"
- xml_target: "TrakingCargo_TakingCargoPlace_CountryCode"
- status: confirmed_operator

### delivery_country_code
- value: "RU"
- xml_target: "DeliveryPlace_CountryCode"
- status: confirmed_operator

### language_code
- value: "RU"
- xml_target: "LanguageCode"
- status: confirmed_operator

### cmr_choice
- value: "1"
- xml_target: "CMR_Choice"
- status: confirmed_operator

### truck_number
- value: "О157АО774"
- xml_target: "CMRTransport_PrimeMoverStateSignID"
- status: confirmed_document

### trailer_number
- value: "BT374974"
- xml_target: "CMRTransport_TrailerStateSignID"
- status: confirmed_document

### total_places
- value: "127"
- xml_target: "GoodsQuantity"
- status: confirmed_document

### total_gross_weight
- value: "3500.00"
- xml_target: "CMRGoodsWeight_GrossWeightQuantity"
- status: confirmed_document

### goods_1
- item_no:
  - value: "1"
  - xml_target: "GoodsNumeric"
  - status: confirmed_document
- description:
  - value: "Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- tnved:
  - value: "5804101000"
  - xml_target: "GoodsNomenclatureCode"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- quantity_places_or_units:
  - value: "127"
  - xml_target: "GoodsQuantity"
  - status: confirmed_document
- gross_weight:
  - value: "3500.00"
  - xml_target: "GrossWeightQuantity"
  - status: confirmed_document
- packing_code:
  - value: "PX"
  - xml_target: "PackingCode"
  - status: confirmed_operator
- packing_quantity:
  - value: "127"
  - xml_target: "PakingQuantity"
  - status: confirmed_operator
  - note: "assumption_for_debug"
- packing_description:
  - value: "Поддон"
  - xml_target: "PackingDescription"
  - status: confirmed_operator
  - note: "assumption_for_debug"


## document: td
- uqi_prefix: formalized.td_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\ТД 10719110_240126_5011363_reg 00378тд (1).pdf"
- name:
  - value: "ТД 10719110_240126_5011363_reg 00378тд (1).pdf"
- xml_target_root:
  - value: "AltaTD"
- status:
  - value: "confirmed"

### number
- value: "10719110/240126/5011363"
- xml_target: "TransitRegistrationNumber"
- status: confirmed_document

### date
- value: "24.01.2026"
- xml_target: "RegistrationDate"
- status: confirmed_document

### total_gross_weight
- value: "3500"
- xml_target: "TotalGrossWeight"
- status: confirmed_document

### total_places
- value: "127"
- xml_target: "TotalPackageQuantity"
- status: confirmed_document

### seals_info
- value: "ОТСУТСТВУЕТ"
- xml_target: "SealsNumber"
- status: confirmed_operator

### customs_office_code
- value: "10719110"
- xml_target: "CustomsCode"
- status: confirmed_document

### destination_customs_code
- value: "10404083"
- xml_target: "DestinationCustomsCode"
- status: confirmed_document


## document: payment_order
- uqi_prefix: formalized.payment_order_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\платежки\\currency_transfer_1_13.01.2026.pdf"
- name:
  - value: "currency_transfer_1_13.01.2026.pdf"
- xml_target_root:
  - value: "AltaPaymentOrder"
- status:
  - value: "confirmed"

### number
- value: "1"
- xml_target: "DocumentReference_PrDocumentNumber"
- status: confirmed_document

### date
- value: "13.01.2026"
- xml_target: "DocumentReference_PrDocumentDate"
- status: confirmed_document

### amount
- value: "63219.00"
- xml_target: "PaymentAmount"
- status: confirmed_document

### payment_mode_code
- value: "CNY"
- xml_target: "PaymentModeCode"
- status: confirmed_document

### transaction_kind
- value: "01"
- xml_target: "TransactionKind"
- status: confirmed_operator

### purpose
- value: "PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30"
- xml_target: "Purpose"
- status: confirmed_document

### payer_name
- value: "LLC SKIF"
- xml_target: "Payer_OrganizationName"
- status: confirmed_document

### payer_inn
- value: "1650389298"
- xml_target: "Payer_INN"
- status: confirmed_document

### payer_kpp
- value: "165001001"
- xml_target: "Payer_KPP"
- status: confirmed_operator

### payee_name
- value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
- xml_target: "Payee_OrganizationName"
- status: confirmed_document

### payer_sign_surname
- value: "Саранов"
- xml_target: "PersonSurname"
- status: confirmed_document

### payer_sign_name
- value: "Дмитрий"
- xml_target: "PersonName"
- status: confirmed_document

### payer_bank_name
- value: "ФИЛИАЛ \"ЦЕНТРАЛЬНЫЙ\" БАНКА ВТБ (ПАО), БИК 044525411"
- xml_target: "Payer_Bank_BankName"
- status: confirmed_document

### payee_bank_name
- value: "VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX, CN767290000018"
- xml_target: "Payee_Bank_BankName"
- status: confirmed_document


## document: payment_order
- uqi_prefix: formalized.payment_order_2
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\платежки\\currency_transfer_7_28.11.2025.pdf"
- name:
  - value: "currency_transfer_7_28.11.2025.pdf"
- xml_target_root:
  - value: "AltaPaymentOrder"
- status:
  - value: "confirmed"

### number
- value: "7"
- xml_target: "DocumentReference_PrDocumentNumber"
- status: confirmed_document

### date
- value: "28.11.2025"
- xml_target: "DocumentReference_PrDocumentDate"
- status: confirmed_document

### amount
- value: "34041.00"
- xml_target: "PaymentAmount"
- status: confirmed_document

### currency_mode_code
- value: "CNY"
- xml_target: "PaymentModeCode"
- status: confirmed_document

### transaction_kind
- value: "01"
- xml_target: "TransactionKind"
- status: confirmed_operator

### purpose
- value: "PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30"
- xml_target: "Purpose"
- status: confirmed_document

### payer_name
- value: "LLC SKIF"
- xml_target: "Payer_OrganizationName"
- status: confirmed_document

### payer_inn
- value: "1650389298"
- xml_target: "Payer_INN"
- status: confirmed_document

### payer_kpp
- value: "165001001"
- xml_target: "Payer_KPP"
- status: confirmed_operator

### payee_name
- value: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
- xml_target: "Payee_OrganizationName"
- status: confirmed_document

### payer_sign_surname
- value: "Саранов"
- xml_target: "PersonSurname"
- status: confirmed_document

### payer_sign_name
- value: "Дмитрий"
- xml_target: "PersonName"
- status: confirmed_document

### payer_bank_name
- value: "ФИЛИАЛ \"ЦЕНТРАЛЬНЫЙ\" БАНКА ВТБ (ПАО), БИК 044525411"
- xml_target: "Payer_Bank_BankName"
- status: confirmed_document

### payee_bank_name
- value: "VTB BANK (PJSC) SHANGHAI BRANCH VTBRCNSHXXX, CN767290000018"
- xml_target: "Payee_Bank_BankName"
- status: confirmed_document


## document: service_invoice
- uqi_prefix: formalized.service_invoice_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\Счет_№26-00378-tl_от_27-01-2026.pdf"
- name:
  - value: "Счет_№26-00378-tl_от_27-01-2026.pdf"
- xml_target_root:
  - value: "AltaServiceInvoice"
- status:
  - value: "confirmed"

### number
- value: "26-00378-tl"
- xml_target: "Registration_PrDocumentNumber"
- status: confirmed_document

### date
- value: "27.01.2026"
- xml_target: "Registration_PrDocumentDate"
- status: confirmed_document

### total_amount
- value: "2700.00"
- xml_target: "TotalServiceCost"
- status: confirmed_document

### currency
- value: "USD"
- xml_target: "Currency"
- status: confirmed_document

### service_provider_name
- value: "ООО \"Трансимпериал\""
- xml_target: "ServiceProvider_Name"
- status: confirmed_document

### contract_ref_number
- value: "КООО/26651/М"
- xml_target: "ContractDetails_PrDocumentNumber"
- status: confirmed_document

### contract_ref_date
- value: "13.05.2025"
- xml_target: "ContractDetails_PrDocumentDate"
- status: confirmed_document

### service_provider_bank_name
- value: "АО \"Райффайзенбанк\", БИК 044525700, р/с 40702810400000233463"
- xml_target: "ServiceProvider_PaymentRequisitions_BankName"
- status: confirmed_document

### services_1
- item_no:
  - value: "1"
  - status: confirmed_document
- goods_description:
  - value: "Транспортно-экспедиционные услуги в международном сообщении"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- route_description:
  - value: "China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск)"
  - xml_target: "ServiceName"
  - status: confirmed_document
- amount:
  - value: "1404.00"
  - xml_target: "ServiceCost_Amount"
  - status: confirmed_document
- currency:
  - value: "USD"
  - xml_target: "ServiceCost_Currency"
  - status: confirmed_document
- tax_rate:
  - value: "0%"
  - xml_target: "TaxRate"
  - status: confirmed_document
- tax_sum:
  - value: "0.00"
  - xml_target: "TaxSum"
  - status: confirmed_document

### services_2
- item_no:
  - value: "2"
  - status: confirmed_document
- goods_description:
  - value: "Транспортно-экспедиционные услуги"
  - xml_target: "GoodsDescription"
  - status: confirmed_document
- route_description:
  - value: "граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны"
  - xml_target: "ServiceName"
  - status: confirmed_document
- amount:
  - value: "1296.00"
  - xml_target: "ServiceCost_Amount"
  - status: confirmed_document
- currency:
  - value: "USD"
  - xml_target: "ServiceCost_Currency"
  - status: confirmed_document
- tax_rate:
  - value: "0%"
  - xml_target: "TaxRate"
  - status: confirmed_document
- tax_sum:
  - value: "0.00"
  - xml_target: "TaxSum"
  - status: confirmed_document


## document: insurance_document
- uqi_prefix: formalized.insurance_document_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\Счет_№26-00378-tl_1_от_14-01-2026.pdf"
- name:
  - value: "Счет_№26-00378-tl_1_от_14-01-2026.pdf"
- xml_target_root:
  - value: "AltaFreeDoc"
- status:
  - value: "confirmed"

### number
- value: "26-00378-tl/1"
- xml_target: "DocumentHead_DocumentNumber"
- status: confirmed_document

### date
- value: "14.01.2026"
- xml_target: "DocumentHead_DocumentDate"
- status: confirmed_document

### amount
- value: "910.34"
- xml_target: "TotalCost"
- status: confirmed_document

### currency
- value: "RUB"
- xml_target: "CurrencyCode"
- status: confirmed_document

### text_body
- value: "Возмещение за добровольное страхование груза по договору №КООО/26651/М от 13-05-2025; по заявлению №26-00378-tl от 14.01.2026"
- xml_target: "TextPara"
- status: confirmed_document


## document: tech_description
- uqi_prefix: formalized.tech_description_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\техничка .pdf"
- name:
  - value: "техничка .pdf"
- xml_target_root:
  - value: "AltaFreeDoc"
- status:
  - value: "confirmed"

### doc_name
- value: "Технические характеристики"
- xml_target: "DocumentHead_DocumentName"
- status: confirmed_document

### number
- value: "БН"
- xml_target: "DocumentHead_DocumentNumber"
- status: confirmed_document

### date
- value: "30.10.2025"
- xml_target: "DocumentHead_DocumentDate"
- status: confirmed_operator
- note: "assumption_for_debug"

### text_body
- link: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\техничка .pdf"
- xml_target: "TextPara"
- status: confirmed_document

### document_sign
- value: "0"
- xml_target: "DocumentSign"
- status: confirmed_operator
- note: "assumption_for_debug"


## document: tech_description
- uqi_prefix: formalized.tech_description_2
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\техничка Антикот, антипыльца антимошка .pdf"
- name:
  - value: "техничка Антикот, антипыльца антимошка .pdf"
- xml_target_root:
  - value: "AltaFreeDoc"
- status:
  - value: "confirmed"

### doc_name
- value: "Технические характеристики"
- xml_target: "DocumentHead_DocumentName"
- status: confirmed_document

### number
- value: "БН"
- xml_target: "DocumentHead_DocumentNumber"
- status: confirmed_document

### date
- value: "30.10.2025"
- xml_target: "DocumentHead_DocumentDate"
- status: confirmed_operator
- note: "assumption_for_debug"

### text_body
- link: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\техничка Антикот, антипыльца антимошка .pdf"
- xml_target: "TextPara"
- status: confirmed_document

### document_sign
- value: "0"
- xml_target: "DocumentSign"
- status: confirmed_operator
- note: "assumption_for_debug"


## document: passport
- uqi_prefix: formalized.passport_1
- full_path:
  - value: "alta\\stable_source\\Passport_63_09_449948.xml"
- name:
  - value: "Passport_63_09_449948.xml"
- xml_target_root:
  - value: "AltaPassport"
- status:
  - value: "confirmed"

### series
- value: "63 09"
- xml_target: "CardSeries"
- status: confirmed_document

### number
- value: "449948"
- xml_target: "CardNumber"
- status: confirmed_document

### issue_date
- value: "2010-03-11"
- xml_target: "CardDate"
- status: confirmed_document

### issued_by
- value: "ОТДЕЛОМ УФМС РОССИИ ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА"
- xml_target: "OrganizationName"
- status: confirmed_document

### full_name
- value: "АРБУЗОВА АНАСТАСИЯ КОНСТАНТИНОВНА"
- xml_target: "PersonInfo_PersonSurname + PersonInfo_PersonName + PersonInfo_PersonMiddleName"
- status: confirmed_document

### birth_date
- value: "1987-07-25"
- xml_target: "PersonInfo_Birthday"
- status: confirmed_document

### birth_place
- value: "город Саратов"
- xml_target: "PersonInfo_Birthplace"
- status: confirmed_document

### residence_address
- value: "410052, RU, РОССИЯ, Саратовская область, Саратов, Ул. Одесская д 11 кв 160"
- xml_target: "ResidencePlace_*"
- status: confirmed_document

### sex
- value: "1"
- xml_target: "PersonInfo_Sex"
- status: confirmed_document


## document: letter_of_attorney
- uqi_prefix: formalized.letter_of_attorney_1
- full_path:
  - value: "alta\\stable_source\\LetterOfAttorney_1.xml"
- name:
  - value: "LetterOfAttorney_1.xml"
- xml_target_root:
  - value: "AltaLetterOfAttorney"
- status:
  - value: "confirmed"

### number
- value: "1"
- xml_target: "DocumentReference_PrDocumentNumber"
- status: confirmed_document

### date
- value: "2026-02-01"
- xml_target: "DocumentReference_PrDocumentDate"
- status: confirmed_document

### valid_until
- value: "2026-12-31"
- xml_target: "EndDate"
- status: confirmed_document

### attorney_name
- value: "АРБУЗОВА АНАСТАСИЯ КОНСТАНТИНОВНА"
- xml_target: "EmpoweredPerson_*"
- status: confirmed_document

### attorney_passport_series
- value: "63 09"
- xml_target: "EmpoweredPerson_Passport_IdentityCardSeries"
- status: confirmed_document

### attorney_passport_number
- value: "449948"
- xml_target: "EmpoweredPerson_Passport_IdentityCardNumber"
- status: confirmed_document

### attorney_passport_issue_date
- value: "2010-03-11"
- xml_target: "EmpoweredPerson_Passport_IdentityCardDate"
- status: confirmed_document

### attorney_passport_issued_by
- value: "ОТДЕЛОМ УФМС ПО САРАТОВСКОЙ ОБЛАСТИ В ЛЕНИНСКОМ РАЙОНЕ ГОР. САРАТОВА"
- xml_target: "EmpoweredPerson_Passport_OrganizationName"
- status: confirmed_document

### issuer_name
- value: "ООО «СКИФ»"
- xml_target: "Organization_OrganizationName"
- status: confirmed_document

### issuer_inn
- value: "1650389298"
- xml_target: "Organization_INN"
- status: confirmed_document

### issuer_kpp
- value: "165001001"
- xml_target: "Organization_KPP"
- status: confirmed_document

### issuer_ogrn
- value: "1201600020390"
- xml_target: "Organization_OGRN"
- status: confirmed_document

### subject
- link: "alta\\stable_source\\LetterOfAttorney_1.xml"
- xml_target: "Subject"
- status: confirmed_operator

### issuer_director_surname
- value: "Саранов"
- xml_target: "Organization_OrganizationPerson_PersonSurname"
- status: confirmed_document

### issuer_director_name
- value: "Дмитрий"
- xml_target: "Organization_OrganizationPerson_PersonName"
- status: confirmed_document

### issuer_director_post
- value: "Директор"
- xml_target: "Organization_OrganizationPerson_PersonPost"
- status: confirmed_document


---

# II. non_formalized

## document: svh
- uqi_prefix: non_formalized.svh_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\ДО 14431420260204161621.pdf"
- name:
  - value: "ДО 14431420260204161621.pdf"
- status:
  - value: "confirmed"

### number
- value: "0000080"
- status: confirmed_document

### date
- value: "03.02.2026"
- status: confirmed_document

### warehouse_license
- value: "10404/141210/10092/5"
- status: confirmed_document

### actual_gross_weight
- value: "3500"
- status: confirmed_document

### actual_places
- value: "127"
- status: confirmed_document

### transport_reg_number
- value: "О157АО774 / BT374974"
- status: confirmed_document

### non_xml_fields
- value: "Перевозчик: ООО \"МАГСТИЛПРОМ\". Транспортная накладная №00378 от 20.01.2026. Инвойс №LM-2591 от 30.10.2025. ТД №10719110/240126/5011363."
- status: confirmed_document
- formalization_role: non_xml


## document: svh_additional_sheet
- uqi_prefix: non_formalized.svh_additional_sheet_1
- full_path:
  - value: "alta\\source\\МоскитнаяСетка\\HEBEI LANGMAI IMPORT AND EXPORT\\02\\ДО доп 14431520260204161645.pdf"
- name:
  - value: "ДО доп 14431520260204161645.pdf"
- status:
  - value: "confirmed"

### number
- value: "0000080"
- status: confirmed_document

### date
- value: "03.02.2026"
- status: confirmed_document

### actual_gross_weight
- value: "3500"
- status: confirmed_document

### actual_places
- value: "127"
- status: confirmed_document

### transport_reg_number
- value: "О157АО774 / BT374974"
- status: confirmed_document

### non_xml_fields
- value: "Итого 2 товара; стоимость 97260 CNY."
- status: confirmed_document
- formalization_role: non_xml


---

# III. unresolved_questions

- value: "none"
- note: "DEBUG BUILD: unresolved_questions очищены (pending закрыты предположениями)."
