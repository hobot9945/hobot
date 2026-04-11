# Данные от оператора (МоскитнаяСетка)

- formalized.contract_1.currency_code: "156" (CNY)
- formalized.contract_1.delivery_terms: "EXW Хэншуй"
- formalized.contract_1.signed_person_surname: "Li"
- formalized.contract_1.signed_person_name: "Jing"
- formalized.contract_2.currency_code: "156" (CNY)
- formalized.invoice_1.exchange_rate: "10.9430"
- formalized.invoice_1.dispatch_country_code: "CN"
- formalized.invoice_1.destination_country_code: "RU"
- formalized.invoice_1.places_description: "Поддон"
- formalized.packing_list_1.number: "LM-2591"
- formalized.packing_list_1.date: "30.10.2025"
- formalized.cmr_1.taking_cargo_country_code: "CN"
- formalized.cmr_1.delivery_country_code: "RU"
- formalized.cmr_1.language_code: "RU"
- formalized.cmr_1.cmr_choice: "1"
- formalized.cmr_1.goods_1.packing_code: "PX"
- formalized.payment_order_1.payer_kpp: "165001001"
- formalized.payment_order_2.payer_kpp: "165001001"
- formalized.payment_order_1.transaction_kind: "01"
- formalized.payment_order_2.transaction_kind: "01"
- formalized.letter_of_attorney_1.subject: "link: alta\stable_source\LetterOfAttorney_1.xml"
- formalized.td_1.seals_info: "ОТСУТСТВУЕТ"

# --- assumption_for_debug (разрешено оператором) ---

# invoice totals (from packing_list totals)
- formalized.invoice_1.total_gross_weight: "3500.00" (from packing_list_1.total_gross)
- formalized.invoice_1.total_net_weight: "3302.00" (from packing_list_1.total_net)

# invoice goods weights (from packing_list goods)
- formalized.invoice_1.goods_1.gross_weight: "855.00" (from packing_list_1.goods_1.gross_weight)
- formalized.invoice_1.goods_1.net_weight: "806.60" (from packing_list_1.goods_1.net_weight)
- formalized.invoice_1.goods_2.gross_weight: "490.00" (from packing_list_1.goods_2.gross_weight)
- formalized.invoice_1.goods_2.net_weight: "460.80" (from packing_list_1.goods_2.net_weight)
- formalized.invoice_1.goods_3.gross_weight: "265.00" (from packing_list_1.goods_3.gross_weight)
- formalized.invoice_1.goods_3.net_weight: "252.00" (from packing_list_1.goods_3.net_weight)
- formalized.invoice_1.goods_4.gross_weight: "155.00" (from packing_list_1.goods_4.gross_weight)
- formalized.invoice_1.goods_4.net_weight: "144.00" (from packing_list_1.goods_4.net_weight)
- formalized.invoice_1.goods_5.gross_weight: "520.00" (from packing_list_1.goods_5.gross_weight)
- formalized.invoice_1.goods_5.net_weight: "491.40" (from packing_list_1.goods_5.net_weight)
- formalized.invoice_1.goods_6.gross_weight: "1190.00" (from packing_list_1.goods_6.gross_weight)
- formalized.invoice_1.goods_6.net_weight: "1123.20" (from packing_list_1.goods_6.net_weight)
- formalized.invoice_1.goods_7.gross_weight: "25.00" (from packing_list_1.goods_7.gross_weight)
- formalized.invoice_1.goods_7.net_weight: "24.00" (from packing_list_1.goods_7.net_weight)

# invoice goods: origin/manufacturer (assumptions)
# NOTE: в эталонном XML OriginCountryCode — цифровой код страны (например, Китай=156).
- formalized.invoice_1.goods_1.origin_country_code: "156" (assumption_for_debug; numeric country code)
- formalized.invoice_1.goods_2.origin_country_code: "156" (assumption_for_debug; numeric country code)
- formalized.invoice_1.goods_3.origin_country_code: "156" (assumption_for_debug; numeric country code)
- formalized.invoice_1.goods_4.origin_country_code: "156" (assumption_for_debug; numeric country code)
- formalized.invoice_1.goods_5.origin_country_code: "156" (assumption_for_debug; numeric country code)
- formalized.invoice_1.goods_6.origin_country_code: "156" (assumption_for_debug; numeric country code)
- formalized.invoice_1.goods_7.origin_country_code: "156" (assumption_for_debug; numeric country code)

- formalized.invoice_1.goods_1.manufacturer: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD" (assumption_for_debug)
- formalized.invoice_1.goods_2.manufacturer: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD" (assumption_for_debug)
- formalized.invoice_1.goods_3.manufacturer: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD" (assumption_for_debug)
- formalized.invoice_1.goods_4.manufacturer: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD" (assumption_for_debug)
- formalized.invoice_1.goods_5.manufacturer: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD" (assumption_for_debug)
- formalized.invoice_1.goods_6.manufacturer: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD" (assumption_for_debug)
- formalized.invoice_1.goods_7.manufacturer: "HEBEI LANGMAI IMPORT AND EXPORT CO., LTD" (assumption_for_debug)

# packing list transport (assumptions)
- formalized.packing_list_1.transport_1.number: "О157АО774 / BT374974" (from cmr_1; assumption_for_debug)
- formalized.packing_list_1.transport_1.mode_code: "31" (assumption_for_debug)
- formalized.packing_list_1.transport_1.nationality_code: "000" (assumption_for_debug)
- formalized.packing_list_1.transport_1.mover_indicator: "true" (assumption_for_debug)

- formalized.packing_list_1.transport_2.number: "BT374974"
- formalized.packing_list_1.transport_2.mode_code: "31"
- formalized.packing_list_1.transport_2.nationality_code: "000"
- formalized.packing_list_1.transport_2.mover_indicator: "false"

# cmr goods_* details (assumptions; если CMR агрегирован в одну строку)
- formalized.cmr_1.goods_1.tnved: "5804101000" (assumption_for_debug)
- formalized.cmr_1.goods_1.packing_quantity: "127" (assumption_for_debug)
- formalized.cmr_1.goods_1.packing_description: "ПОДДОН" (assumption_for_debug)

# payment order: PaymentModeCode (ВАЖНО: это не валюта; в эталонах встречается 0)
- formalized.payment_order_1.currency_mode_code: "0" (assumption_for_debug; matches typical export)
- formalized.payment_order_2.currency_mode_code: "0" (assumption_for_debug; matches typical export)

# supplementary agreement additional fields (assumptions_for_debug)
- formalized.contract_2.delivery_terms: "EXW Хэншуй" (assumption_for_debug)
- formalized.contract_2.expiry_date: "31.12.2026" (assumption_for_debug)
- formalized.contract_2.deal_sign: "1" (assumption_for_debug)
- formalized.contract_2.stock_category_sign: "0" (assumption_for_debug)
- formalized.contract_2.buyer_limitation_sign: "0" (assumption_for_debug)
- formalized.contract_2.insurance_sign: "0" (assumption_for_debug)

# tech_description defaults
- formalized.tech_description_1.date: "30.10.2025" (assumption_for_debug)
- formalized.tech_description_2.date: "30.10.2025" (assumption_for_debug)
- formalized.tech_description_1.document_sign: "0" (assumption_for_debug)
- formalized.tech_description_2.document_sign: "0" (assumption_for_debug)