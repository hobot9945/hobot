# Данные от оператора (МоскитнаяСетка) — актуализация по чату

updated_at: 2026-04-30
case_name: МоскитнаяСетка

## contract (LM-2553)
- formalized.contract_1.currency_code_numeric: "156" (CNY)
- formalized.contract_1.delivery_terms: "EXW HEBEI" (решение оператора)
- formalized.contract_1.deal_sign: "1" (решение оператора)
- formalized.contract_1.foreign_person_country_code_alpha2: "CN" (подтверждено оператором)

## supplementary_contract (№1 к LM-2553)
- formalized.supplementary_contract_1.currency_code_numeric: "156" (CNY)
- formalized.supplementary_contract_1.expiry_date: "31.12.2026"
- formalized.supplementary_contract_1.deal_sign: "1"
- formalized.supplementary_contract_1.stock_category_sign: "0"
- formalized.supplementary_contract_1.buyer_limitation_sign: "0"
- formalized.supplementary_contract_1.insurance_sign: "0"
- formalized.supplementary_contract_1.signed_person_surname: "Li"
- formalized.supplementary_contract_1.signed_person_name: "Jing"
- formalized.supplementary_contract_1.signed_person_middle_name: "" (пусто, решение оператора)
- formalized.supplementary_contract_1.foreign_person_short_name_equals_full: "true" (решение оператора)
- formalized.supplementary_contract_1.foreign_person_country_code_alpha2: "CN" (подтверждено оператором)

## invoice (LM-2591)
- formalized.invoice_1.exchange_rate: "10.9430"
- formalized.invoice_1.currency_code: "CNY" (решение оператора)
- formalized.invoice_1.dispatch_country_code: "CN"
- formalized.invoice_1.trading_country_code: "CN" (решение оператора)
- formalized.invoice_1.destination_country_code: "RU"
- formalized.invoice_1.delivery_terms_string: "EXW"
- formalized.invoice_1.delivery_terms_numeric: "01" (решение оператора)
- formalized.invoice_1.places_description: "Поддон"
- formalized.invoice_1.total_gross_weight: "3500.00" (из PL totals)
- formalized.invoice_1.total_net_weight: "3302.00" (из PL totals)
- formalized.invoice_1.total_cost: "97260.00" (из инвойса)
- formalized.invoice_1.gcost: "97260.00" (решение оператора: =TotalCost)
- formalized.invoice_1.seller_country_code_alpha2: "CN" (подтверждено оператором)
- formalized.invoice_1.consignor_equals_seller: "true"
- formalized.invoice_1.consignee_equals_buyer: "true"

### invoice goods (общие решения)
- formalized.invoice_1.goods_all.trade_mark: "ОТСУТСТВУЕТ" (решение оператора)
- formalized.invoice_1.goods_all.goods_mark: "ОТСУТСТВУЕТ" (решение оператора)
- formalized.invoice_1.goods_all.origin_country_code_numeric: "156"
- formalized.invoice_1.goods_all.manufacturer: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD"
- formalized.invoice_1.goods_all.model: "NOT APPLICABLE"

### invoice goods weights (from packing_list goods)
- formalized.invoice_1.goods_1.gross_weight: "855.00"
- formalized.invoice_1.goods_1.net_weight: "806.60"
- formalized.invoice_1.goods_2.gross_weight: "490.00"
- formalized.invoice_1.goods_2.net_weight: "460.80"
- formalized.invoice_1.goods_3.gross_weight: "265.00"
- formalized.invoice_1.goods_3.net_weight: "252.00"
- formalized.invoice_1.goods_4.gross_weight: "155.00"
- formalized.invoice_1.goods_4.net_weight: "144.00"
- formalized.invoice_1.goods_5.gross_weight: "520.00"
- formalized.invoice_1.goods_5.net_weight: "491.40"
- formalized.invoice_1.goods_6.gross_weight: "1190.00"
- formalized.invoice_1.goods_6.net_weight: "1123.20"
- formalized.invoice_1.goods_7.gross_weight: "25.00"
- formalized.invoice_1.goods_7.net_weight: "24.00"

## packing list (LM-2591)
- formalized.packing_list_1.consignor_shortname_equals_full: "true"
- formalized.packing_list_1.consignee_shortname_equals_full: "true"
- formalized.packing_list_1.consignor_country_code_alpha2: "CN"
- formalized.packing_list_1.registration_doc_name: "Упаковочный лист"
- formalized.packing_list_1.registration_doc_number: "LM-2591"
- formalized.packing_list_1.registration_doc_date: "30.10.2025"

### packing list: transport (тягач/прицеп)
- formalized.packing_list_1.transport_1.number: "О157АО774"
- formalized.packing_list_1.transport_1.mode_code: "31"
- formalized.packing_list_1.transport_1.nationality_code: "000"
- formalized.packing_list_1.transport_1.mover_indicator: "true"
- formalized.packing_list_1.transport_2.number: "ВТ374974"
- formalized.packing_list_1.transport_2.mode_code: "31"
- formalized.packing_list_1.transport_2.nationality_code: "000"
- formalized.packing_list_1.transport_2.mover_indicator: "false"

### packing list: PackingInfo.PakingQuantity (решение оператора: =GoodsQuantity)
- formalized.packing_list_1.goods_1.paking_quantity: "60"
- formalized.packing_list_1.goods_2.paking_quantity: "30"
- formalized.packing_list_1.goods_3.paking_quantity: "6"
- formalized.packing_list_1.goods_4.paking_quantity: "3"
- formalized.packing_list_1.goods_5.paking_quantity: "9"
- formalized.packing_list_1.goods_6.paking_quantity: "18"
- formalized.packing_list_1.goods_7.paking_quantity: "1"

## cmr (№00378)
- formalized.cmr_1.language_code: "RU"
- formalized.cmr_1.cmr_choice: "1"
- formalized.cmr_1.registration_place: "Маньчжурия"
- formalized.cmr_1.taking_cargo_country_code_alpha2: "CN"
- formalized.cmr_1.delivery_country_code_alpha2: "RU"
- formalized.cmr_1.consignor_shortname_equals_full: "true"
- formalized.cmr_1.consignee_shortname_equals_full: "true"
- formalized.cmr_1.delivery_terms_string: "EXW"
- formalized.cmr_1.delivery_terms_place: "Naberezhnye Chelny"
- formalized.cmr_1.consignor_guarantee_all: "ОТСУТСТВУЕТ"
- formalized.cmr_1.goods_1.packing_quantity: "127"

## payment orders (currency_transfer_*)
- formalized.payment_order_all.document_code: "04023"
- formalized.payment_order_all.transaction_kind: "01"
- formalized.payment_order_all.payment_mode_code: "0"
- formalized.payment_order_all.priority: "5"
- formalized.payment_order_1.payer_kpp: "165001001"
- formalized.payment_order_2.payer_kpp: "165001001"
- formalized.payment_order_all.payer_sign.surname: "Саранов"
- formalized.payment_order_all.payer_sign.name: "Дмитрий"

## service invoice (26-00378-tl)
- formalized.service_invoice_1.document_sign: "1"
- formalized.service_invoice_1.signature_choice: "1"
- formalized.service_invoice_1.payment_document_number: "ОТСУТСТВУЕТ"
- formalized.service_invoice_1.payment_document_date: "ОТСУТСТВУЕТ"
- formalized.service_invoice_1.consignor_decision: "seller" (решение оператора: Consignor_* = seller)
- formalized.service_invoice_1.consignee_house: "30"
- formalized.service_invoice_1.consignee_room: "211"
- formalized.service_invoice_1.service_1.service_name: "ОТСУТСТВУЕТ"
- formalized.service_invoice_1.service_2.service_name: "ОТСУТСТВУЕТ"

## insurance document (26-00378-tl/1, 04111)
- formalized.insurance_document_1.textpara_storage: "link"

## master data
- non_formalized.master_data_1.declarant_email: ""
- non_formalized.master_data_1.representative_email: ""

## tech_description defaults
- formalized.tech_description_1.date: "30.10.2025"
- formalized.tech_description_1.number: "Б/Н"

## decisions_from_chat (2026-05-01)
- allow_cross_doc_master_data_to_contract_invoice: "true" (разрешено подставлять ОГРН/ИНН/КПП из non_formalized.master_data_1 в formalized.contract_1 и formalized.invoice_1, status=CO)
- service_invoice_1.consignor_address_from_seller: "true" (решение оператора: в Service Invoice consignor=seller, адрес продавца подставлять; PostalCode оставлять пустым если отсутствует)


## decisions_from_chat (2026-05-01, continued)
- formalized.supplementary_contract_1.foreign_person_address_from_contract: "true" (разрешено подставлять адрес продавца из formalized.contract_1 в supplementary_contract_1, status=CO)
- formalized.cmr_1.consignee_ogrn_from_master_data: "true" (разрешено подставить ОГРН 1201600020390 из master_data в CMR Consignee_OGRNID, status=CO)
- formalized.service_invoice_1.consignee_ogrn_from_master_data: "true" (разрешено подставить ОГРН 1201600020390 из master_data в ServiceInvoice Consignee_RFOrganizationFeatures_OGRN, status=CO)
- formalized.service_invoice_1.signatures_confirmed: "true" (подтверждено: Director=Климович Л.А.; ChiefAccountant=Лехно О.А.)
- non_formalized.svh_additional_sheet_1.address_from_cmr: "true" (разрешено заполнить адрес СВХ из CMR п.3, status=CD)
- non_formalized.svh_1.actual_totals_from_svh_additional_sheet: "true" (разрешено считать итоги мест/веса из доп.листа ДО, status=CD)
