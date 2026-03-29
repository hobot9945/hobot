# facts_schema

## Meta

- case_name: ПриточнаяВентиляция

- source_folder: C:\hobot\alta\source\ПриточнаяВентиляция

- dt_scope: 1 ДТ / 1 товар

- status: in_progress

- ready_for_next_step: partial

- unresolved_conflicts_count: 2

- unresolved_missing_critical_data_count: 6

- note: facts собран из первички; mapping и расчеты отделены от shipment facts

## Documents package

### Core documents

- contract:
  
  - file: SALES CONTRACT No25AZC003.pdf
  - status: confirmed_document
  - number: 25AZC003
  - number_status: confirmed_document
  - date: 10.04.2025
  - date_status: confirmed_document
  - role: core
  - source_class: document

- invoice:
  
  - file: Инвойс 25AZC003B.pdf
  - status: confirmed_document
  - number: 25AZC003B
  - number_status: confirmed_document
  - date: 10.04.2025
  - date_status: confirmed_document
  - role: core
  - source_class: document

- packing_list:
  
  - file: PL 25AZC003B.pdf
  - status: confirmed_document
  - number: БН
  - number_status: confirmed_document
  - date: 10.04.2025
  - date_status: confirmed_document
  - role: core
  - source_class: document

- transport_doc:
  
  - file: СМР.pdf
  - status: confirmed_document
  - kind: CMR
  - kind_status: confirmed_document
  - number: 12327
  - number_status: confirmed_document
  - date: 01.07.2025
  - date_status: confirmed_document
  - role: core
  - source_class: document

- payment_doc:
  
  - file: Платежка.pdf
  - status: confirmed_document
  - number: 1
  - number_status: confirmed_document
  - date: 21.05.2025
  - date_status: confirmed_document
  - role: core
  - source_class: document

- tech_description:
  
  - file: техничка КИВ 125.pdf
  
  - status: confirmed_document
  
  - number: 1СК1004
  
  - number_status: confirmed_document
  
  - date: 10.04.2025
  
  - date_status: confirmed_document
  
  - role: core
  
  - source_class: document

### Supporting documents

- supporting_doc_1:
  
  - file: Заявка номер 1 от 21.05.2025.pdf
  - doc_kind: transport_request
  - number: 1
  - number_status: confirmed_document
  - date: 21.05.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: содержит маршрут, EXW, фрахт 1200 USD + 30000 RUB, адрес разгрузки

- supporting_doc_2:
  
  - file: Счет_№25-12327-k_от_22-05-2025 (2).pdf
  - doc_kind: transport_invoice
  - number: 25-12327-k
  - number_status: confirmed_document
  - date: 22.05.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: подтверждает 624 USD до границы и 576 USD после границы

- supporting_doc_3:
  
  - file: ТД_12327.pdf
  - doc_kind: transit_doc
  - number: 10719110/060725/5070039
  - number_status: confirmed_document
  - date: 06.07.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: содержит транспорт, transit-route, customs destination, graph44 candidates

- supporting_doc_4:
  
  - file: ВТамПостНабережныхЧелнов.pdf
  - doc_kind: svh_doc
  - number: БН
  - number_status: confirmed_document
  - date: 14.07.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: уведомление о регистрации документов для помещения на временное хранение

- supporting_doc_5:
  
  - file: до.pdf
  - doc_kind: svh_doc
  - number: 0000478
  - number_status: confirmed_document
  - date: 14.07.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: отчет о принятии товаров на хранение; содержит складской номер, зону, ячейку

- supporting_doc_6:
  
  - file: 1 Supplementary agreement to the _25AZC003.pdf
  - doc_kind: supplementary_agreement
  - number: pending
  - number_status: not_found
  - date: pending
  - date_status: not_found
  - role: supporting
  - source_class: document
  - note: файл дозагружен, но содержимое в текущем сеансе не распознано

- supporting_doc_7:
  
  - file: ZENGO Proforma Invoice 25AZC003.pdf
  
  - doc_kind: other
  
  - number: 25AZC004
  
  - number_status: confirmed_document
  
  - date: 09.04.2025
  
  - date_status: confirmed_document
  
  - role: supporting
  
  - source_class: document
  
  - note: proforma invoice; useful as supporting/background, not primary truth for shipment facts

### Reference-only documents

- reference_doc_1:
  
  - file: ПриточнаяВентиляцияДТВыгрузка.txt
  - role: reference_only
  - status: sample_only
  - source_class: sample
  - note: использовать только для структуры, completeness и mapping analysis

- reference_doc_2:
  
  - file: GTD_10418010_150725_5103886.pdf
  - role: reference_only
  - status: sample_only
  - source_class: sample
  - note: старая ДТ; использовать только как reference for mapping and presentation

---

# shipment_facts

## General shipment facts

- declaration_basis:
  
  - value: import
  - status: derived
  - source: комплект документов поставки и транзитная декларация
  - source_class: derived
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- incoterms_code:
  
  - value: EXW
  - status: confirmed_document
  - source: invoice, packing list, transport request
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- incoterms_place:
  
  - value: Ningbo
  - status: confirmed_document
  - source: invoice, packing list
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- invoice_currency_alpha:
  
  - value: CNY
  - status: confirmed_document
  - source: invoice, payment, transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- invoice_currency_numeric:
  
  - value: 156
  - status: derived
  - source: alpha currency CNY
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- invoice_amount_total:
  
  - value: 13600.00
  - currency: CNY
  - status: confirmed_document
  - source: invoice, payment, transit declaration, svh report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- package_count_total:
  
  - value: 2
  - status: confirmed_document
  - source: packing list, cmr, transit declaration, svh report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- gross_weight_total:
  
  - value: 383
  - unit: kg
  - status: confirmed_document
  - source: packing list, cmr, transit declaration, svh report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- net_weight_total:
  
  - value: 312.5
  - unit: kg
  - status: confirmed_document
  - source: packing list
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- trade_country_name:
  
  - value: Китай
  - status: derived
  - source: страна продавца и страна отправления
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- trade_country_code:
  
  - value: CN
  - status: derived
  - source: shipment country facts
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- dispatch_country_name:
  
  - value: Китай
  - status: confirmed_document
  - source: invoice, packing list, transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- dispatch_country_code:
  
  - value: CN
  - status: confirmed_document
  - source: transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- origin_country_name:
  
  - value: Китай
  - status: confirmed_document
  - source: invoice, tech description
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- origin_country_code:
  
  - value: CN
  - status: derived
  - source: origin country name China
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- destination_country_name:
  
  - value: Россия
  - status: confirmed_document
  - source: invoice, transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- destination_country_code:
  
  - value: RU
  - status: confirmed_document
  - source: transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- commercial_shipment_point:
  
  - value: Ningbo
  
  - status: confirmed_document
  
  - source: invoice, packing list
  
  - source_class: document
  
  - transfer_relevance: needed_for_review
  
  - blocking_for_next_step: no
  
  - note: коммерческое место отгрузки; не смешивать с фактическим departure_place

## Parties

### Sender

- name:
  
  - value: NINGBO ZENTEC Air Conditioning & Refrigeration Co., Ltd
  - status: confirmed_document
  - source: contract, invoice, packing list, cmr
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- country_code:
  
  - value: CN
  - status: derived
  - source: seller address China
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- country_name:
  
  - value: Китай
  - status: derived
  - source: seller address
  - source_class: derived
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- postcode:
  
  - value: 315175
  - status: confirmed_document
  - source: contract, invoice, packing list, cmr
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- region_or_area:
  
  - value: Ningbo
  - status: confirmed_document
  - source: sender address
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- city_or_locality:
  
  - value: Haishu District
  - status: confirmed_document
  - source: sender address
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- street_address:
  
  - value: D4-109, Liangzhu Culture Park
  - status: confirmed_document
  - source: sender address
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- house:
  
  - value: pending
  - status: not_found
  - source: sender address
  - source_class: document
  - transfer_relevance: optional
  - blocking_for_next_step: no

- office:
  
  - value: pending
  - status: not_found
  - source: sender address
  - source_class: document
  - transfer_relevance: optional
  - blocking_for_next_step: no

- phone:
  
  - value: +86 574 8907 9907
  - status: confirmed_document
  - source: contract, invoice, tech description
  - source_class: document
  - transfer_relevance: optional
  - blocking_for_next_step: no

- email:
  
  - value: pending
  
  - status: not_found
  
  - source: seller docs
  
  - source_class: document
  
  - transfer_relevance: optional
  
  - blocking_for_next_step: no

### Consignee

- name:
  
  - value: ООО ?СКИФ?
  - status: confirmed_document
  - source: contract, invoice, packing list, payment, transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- inn_kpp:
  
  - value: 1650389298/165001001
  - status: confirmed_document
  - source: payment, transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- ogrn:
  
  - value: pending
  - status: not_found
  - source: поставочная первичка
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- country_code:
  
  - value: RU
  - status: derived
  - source: consignee address Russia
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- country_name:
  
  - value: Россия
  - status: derived
  - source: consignee address
  - source_class: derived
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- postcode:
  
  - value: 423800
  - status: confirmed_document
  - source: contract, invoice, packing list, payment, transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- region_or_area:
  
  - value: Республика Татарстан
  - status: confirmed_document
  - source: consignee address
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- city_or_locality:
  
  - value: Набережные Челны
  - status: confirmed_document
  - source: consignee address
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- street:
  
  - value: Хлебный проезд
  - status: confirmed_document
  - source: consignee address
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- house:
  
  - value: 30
  - status: confirmed_document
  - source: consignee address
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- office:
  
  - value: 211
  - status: confirmed_document
  - source: consignee address
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- phone:
  
  - value: +7 937 779-26-56
  - status: confirmed_document
  - source: contract, invoice, packing list
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no
  - note: differs from reference/master-data phone; do not replace with sample-only value

- email:
  
  - value: pending
  
  - status: not_found
  
  - source: поставочная первичка текущей партии
  
  - source_class: document
  
  - transfer_relevance: conditional_for_xml
  
  - blocking_for_next_step: no

## Logistics and transport

- transport_doc_number:
  
  - value: 12327
  - status: confirmed_document
  - source: cmr, transit declaration, svh notice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- transport_doc_date:
  
  - value: 01.07.2025
  - status: confirmed_document
  - source: cmr, transit declaration, svh notice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- departure_place:
  
  - value: Маньчжурия, Китай
  - status: confirmed_document
  - source: cmr
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- delivery_place:
  
  - value: Набережные Челны
  - status: confirmed_document
  - source: invoice, packing list
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - note: do not confuse with consignee address and warehouse address

- carrier_name:
  
  - value: ООО ?АСД-ТРАНС?
  - status: confirmed_document
  - source: transit declaration, svh report
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- carrier_tax_id:
  
  - value: 6732148782/673201001
  - status: confirmed_document
  - source: transit declaration
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- border_transport_description:
  
  - value: A488ОУ67 / A6726I5
  - status: confirmed_document
  - source: cmr, transit declaration, svh report
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- transport_identification_departure_candidate:
  
  - value: A488ОУ67 / A6726I5
  - status: confirmed_document
  - source: cmr, transit declaration
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- transport_registration_country_code_candidate:
  
  - value: RU / BY context observed
  - status: pending
  - source: transit declaration
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - note: requires interpretation; do not treat process-code and normal country code as same thing

- container_flag_candidate:
  
  - value: 0
  - status: sample_only
  - source: reference
  - source_class: sample
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- border_transport_mode_code_candidate:
  
  - value: 31
  - status: sample_only
  - source: reference
  - source_class: sample
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- departure_transport_mode_code_candidate:
  
  - value: 31
  - status: sample_only
  - source: reference
  - source_class: sample
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- border_customs_code_candidate:
  
  - value: 10719110
  - status: confirmed_document
  - source: transit declaration
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- border_customs_name_candidate:
  
  - value: т/п МАПП Забайкальск
  
  - status: confirmed_document
  
  - source: transit declaration
  
  - source_class: document
  
  - transfer_relevance: needed_for_review
  
  - blocking_for_next_step: no

## Warehouse / goods location (document-confirmed only)

- warehouse_document_exists:
  
  - value: yes
  - status: confirmed_document
  - source: svh notice, svh report, cmr
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- location_type:
  
  - value: временное хранение на СВХ
  - status: confirmed_document
  - source: svh documents
  - source_class: document
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - note: business fact; not equal to final graph30 code

- document_kind_code:
  
  - value: pending
  - status: sample_only
  - source: reference
  - source_class: sample
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: yes

- warehouse_document_number:
  
  - value: 0000478
  - status: confirmed_document
  - source: svh report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no
  - note: это номер отчета СВХ, final G_30_1 может отличаться

- warehouse_document_date:
  
  - value: 14.07.2025
  - status: confirmed_document
  - source: svh report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no
  - note: это дата отчета СВХ, final G_30_DATE может отличаться

- warehouse_address_raw:
  
  - value: Республика Татарстан, г. Набережные Челны, Производственный пр-д, д. 45
  - status: confirmed_document
  - source: cmr, svh context
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- warehouse_country_code:
  
  - value: RU
  - status: derived
  - source: warehouse address
  - source_class: derived
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- warehouse_region_or_area:
  
  - value: Республика Татарстан
  - status: confirmed_document
  - source: warehouse address
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- warehouse_city_or_locality:
  
  - value: Набережные Челны
  - status: confirmed_document
  - source: warehouse address
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- warehouse_street_address:
  
  - value: Производственный пр-д, д. 45
  - status: confirmed_document
  - source: warehouse address
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- warehouse_customs_code_candidate:
  
  - value: 10404083
  - status: confirmed_document
  - source: transit declaration destination customs & svh context
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- warehouse_printed_address:
  
  - value: pending
  
  - status: sample_only
  
  - source: reference
  
  - source_class: sample
  
  - transfer_relevance: needed_for_ui
  
  - blocking_for_next_step: no
  
  - note: final printed graph30 string is presentation-layer, not primary fact

## Goods

### Goods[1]

- item_no:
  
  - value: 1
  - status: derived
  - source: single-item shipment
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- product_name_ru:
  
  - value: Клапан инфильтрации воздуха / приточный клапан КИВ-125
  - status: confirmed_document
  - source: tech description, invoice, packing list
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- product_name_en:
  
  - value: Plastic air vent / air infiltration valve
  - status: confirmed_document
  - source: invoice, packing list
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- model:
  
  - value: КИВ-125
  - status: confirmed_document
  - source: tech description, packing list
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- article:
  
  - value: VPRR
  - status: confirmed_document
  - source: proforma, package photo
  - source_class: document
  - transfer_relevance: optional
  - blocking_for_next_step: no

- hs_code:
  
  - value: 8481309908
  - status: confirmed_document
  - source: invoice, cmr, transit declaration, tech description, svh report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- manufacturer:
  
  - value: Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd
  - status: confirmed_document
  - source: invoice, tech description, contract
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- trademark:
  
  - value: pending
  - status: not_found
  - source: shipment documents
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- brand:
  
  - value: pending
  - status: not_found
  - source: shipment documents
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- origin_country_code:
  
  - value: CN
  - status: derived
  - source: invoice, tech description
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- quantity:
  
  - value: 1000
  - unit: шт
  - status: confirmed_document
  - source: invoice, packing list, package photo, proforma
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- unit_code:
  
  - value: 796
  - status: sample_only
  - source: reference
  - source_class: sample
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- unit_name:
  
  - value: ШТ
  - status: confirmed_document
  - source: invoice, packing list, proforma
  - source_class: document
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no

- package_places_count:
  
  - value: 2
  - status: confirmed_document
  - source: packing list, transit declaration, svh report
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- package_places_marking:
  
  - value: 24 PCS per carton; lot 25AZC003B; observed package labels present
  - status: confirmed_document
  - source: packing list, package photo
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no
  - note: final graph31 packaging string still needs manual composition

- gross_weight:
  
  - value: 383
  - unit: kg
  - status: confirmed_document
  - source: packing list, cmr, transit declaration, svh report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- net_weight:
  
  - value: 312.5
  - unit: kg
  - status: confirmed_document
  - source: packing list
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- invoice_value:
  
  - value: 13600.00
  - currency: CNY
  - status: confirmed_document
  - source: invoice, payment, transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- description_31_source_facts:
  
  - value:
  - приточный клапан / клапан инфильтрации воздуха
  - модель КИВ-125
  - предназначен для подачи свежего воздуха в помещение
  - регулируемая подача воздуха, фильтрация, тепло- и шумоизоляция
  - пластиковая труба с оголовком, устанавливаемая в стену
  - производитель Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd
  - 1000 шт, 2 места, 383 кг брутто, 312.5 кг нетто
  - status: derived
  - source: goods facts and tech description
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no

- description_31_main_draft:
  
  - value: pending
  - status: sample_only
  - source: reference and source facts
  - source_class: sample
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: yes

- description_31_lines:
  
  - value:
  - УСТАНАВЛИВАЕМУЮ В СТЕНУ
  - АРТ: КИВ-125 - 1000 ШТ
  - КИВ-125 - КЛАПАН ИНФИЛЬТРАЦИИ ВОЗДУХА
  - status: sample_only
  - source: reference
  - source_class: sample
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - note: useful for structure, not direct shipment fact

- group_description:
  
  - value: КИВ-125 - клапан инфильтрации воздуха
  - status: derived
  - source: goods facts, tech description
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no

---

# alta_master_data_requirements

- consignee_profile:
  
  - registration_id:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes
  - phone:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no
  - email:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no
  - same_as_graph14_mode:
  - value: pending
  - status: pending
  - source: operator / alta_master_data / sample
  - source_class: alta_master_data
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- financial_responsible_profile:
  
  - name:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- declarant_profile:
  
  - name:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- representative_profile:
  
  - last_name:
  - value: pending
  - status: pending
  - source: operator / alta_master_data / authority docs
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

---

# calculated_requirements

- transport_cost_to_border:
  - value: 624.00
  - currency: USD
  - status: confirmed_document
  - source: transport invoice
  - source_class: document
  - transfer_relevance: needed_for_calculation
  - blocking_for_next_step: no
- post_border_transport_cost:
  - value: 576.00
  - currency: USD
  - status: confirmed_document
  - source: transport invoice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
- local_delivery_cost:
  - value: 30000
  - currency: RUB
  - status: confirmed_document
  - source: transport request
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
- insurance:
  - value: pending
  - status: pending
  - source: transport request, proforma
  - source_class: document
  - transfer_relevance: needed_for_calculation
  - blocking_for_next_step: yes
  - note: request says insurance if needed; proforma says buyer covers insurance; actual insurance fact not confirmed
- exchange_rate:
  - value: pending
  - status: pending
  - source: system / operator / calculation rule
  - source_class: calculated
  - transfer_relevance: needed_for_calculation
  - blocking_for_next_step: yes

---

# mapping_rules

- graph30_type_code_rule:
  - value: 11
  - status: confirmed_case_pattern
  - source: reference xml/txt and old dt
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - applies_when: warehouse facts point to temporary storage at warehouse of goods
  - confidence: medium
- graph30_document_kind_code_rule:
  - value: 2
  - status: confirmed_case_pattern
  - source: reference xml/txt and old dt
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - applies_when: graph30 built from warehouse registry document
  - confidence: medium
- graph30_registry_document_number_rule:
  - value: 10404/141210/10092/04
  - status: confirmed_case_pattern
  - source: cmr/svh registry plus reference final graph30
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - applies_when: final graph30 uses warehouse registry, not svh report number
  - confidence: medium
- graph30_registry_document_date_rule:
  - value: 21.08.2019
  - status: confirmed_case_pattern
  - source: cmr/svh registry plus reference final graph30
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - applies_when: final graph30 uses warehouse registry date, not svh report date
  - confidence: medium

---

# documents_for_graph44_candidates

- Graph44Candidate_1:
  - business_role: contract
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: contract
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: SALES CONTRACT No25AZC003.pdf
  - doc_name:
  - value: Контракт
  - status: confirmed_document
  - source: contract
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: 25AZC003
  - date: 10.04.2025
  - source: contract
  - source_class: document
  - transfer_relevance: needed_for_xml
- Graph44Candidate_2:
  - business_role: invoice
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: invoice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: Инвойс 25AZC003B.pdf
  - doc_name:
  - value: Коммерческий инвойс
  - status: confirmed_document
  - source: invoice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: 25AZC003B
  - date: 10.04.2025
  - source: invoice
  - source_class: document
  - transfer_relevance: needed_for_xml
- Graph44Candidate_3:
  - business_role: packing_list
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: packing list
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: PL 25AZC003B.pdf
  - doc_name:
  - value: Упаковочный лист
  - status: confirmed_document
  - source: packing list
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: БН
  - date: 10.04.2025
  - source: packing list
  - source_class: document
  - transfer_relevance: needed_for_xml
- Graph44Candidate_4:
  - business_role: cmr
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: cmr
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: СМР.pdf
  - doc_name:
  - value: CMR
  - status: confirmed_document
  - source: cmr
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: 12327
  - date: 01.07.2025
  - source: cmr
  - source_class: document
  - transfer_relevance: needed_for_xml
- Graph44Candidate_5:
  - business_role: payment
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: payment
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: Платежка.pdf
  - doc_name:
  - value: Заявление на перевод
  - status: confirmed_document
  - source: payment
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: 1
  - date: 21.05.2025
  - source: payment
  - source_class: document
  - transfer_relevance: needed_for_xml
- Graph44Candidate_6:
  - business_role: transport_invoice
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: transport invoice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: Счет_№25-12327-k_от_22-05-2025 (2).pdf
  - doc_name:
  - value: Счет за перевозку
  - status: confirmed_document
  - source: transport invoice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: 25-12327-k
  - date: 22.05.2025
  - source: transport invoice
  - source_class: document
  - transfer_relevance: needed_for_xml
- Graph44Candidate_7:
  - business_role: transport_contract
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: transport request, transport invoice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: transport docs
  - doc_name:
  - value: Договор по перевозке
  - status: confirmed_document
  - source: transport request, transport invoice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: КООО/26651/М
  - date: 13.05.2025
  - source: transport request, transport invoice
  - source_class: document
  - transfer_relevance: needed_for_xml
- Graph44Candidate_8:
  - business_role: tech_description
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: tech description
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: техничка КИВ 125.pdf
  - doc_name:
  - value: Техническое описание
  - status: confirmed_document
  - source: tech description
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: 1СК1004
  - date: 10.04.2025
  - source: tech description
  - source_class: document
  - transfer_relevance: needed_for_xml
- Graph44Candidate_9:
  - business_role: transit_doc
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: transit declaration
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: ТД_12327.pdf
  - doc_name:
  - value: Транзитная декларация
  - status: confirmed_document
  - source: transit declaration
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: 10719110/060725/5070039
  - date: 06.07.2025
  - source: transit declaration
  - source_class: document
  - transfer_relevance: needed_for_xml
- Graph44Candidate_10:
  - business_role: svh_doc
  - item_scope:
  - value: all_dt
  - status: confirmed_document
  - source: svh report
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - file_or_source: до.pdf
  - doc_name:
  - value: Отчет о принятии товаров на хранение
  - status: confirmed_document
  - source: svh report
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - doc_code_candidate: pending
  - doc_code_candidate_status: pending
  - number: 0000478
  - date: 14.07.2025
  - source: svh report
  - source_class: document
  - transfer_relevance: needed_for_xml

---

# reference_observed

- Reference_1:
  - field_name: graph44_role_to_code_matrix
  - observed_in: xml_export / txt_export / old_dt_pdf
  - value: role-to-code mapping observed for contract, invoice, payment, transport invoice, transport contract, packing list, tech description, transit declaration, svh report
  - status: sample_only
  - source_class: sample
  - transfer_relevance: reference_only
  - relevance_for_next_step: useful_for_mapping
  - comment: do not use as shipment fact

---

# Conflicts

- Conflict_1:
  - field: contract_number_reference_in_documents
  - status: unresolved
  - source_1: contract
  - value_1: 25AZC003
  - source_2: invoice / packing_list
  - value_2: 25AZC003B
  - impact: needs explicit separation of contract number and invoice number in downstream projection
  - action_required: treat 25AZC003 as contract and 25AZC003B as invoice unless operator says otherwise
- Conflict_2:
  - field: consignee_contact_data
  - status: unresolved
  - source_1: contract / invoice / packing list
  - value_1: +7 937 779-26-56
  - source_2: reference old dt
  - value_2: +7 (843) 207 18 90, prom_tat@mail.ru
  - impact: master-data fields for Alta must not be filled from reference as shipment facts
  - action_required: obtain current master data from operator / Alta cards

---

# Missing critical data

- MissingCriticalData_1:
  - field: insurance
  - status: unresolved
  - expected_source: insurance policy / operator confirmation / explicit no-insurance confirmation
  - source_class_expected: operator
  - reason: docs mention insurance conditionally but do not confirm actual insurance fact or amount
  - impact: blocks valuation calculation
  - action_required: confirm insured / not insured and amount if applicable
- MissingCriticalData_2:
  - field: consignee_master_data_registration_id
  - status: unresolved
  - expected_source: Alta card / operator
  - source_class_expected: alta_master_data
  - reason: not present in shipment docs
  - impact: blocks graph 8 completeness
  - action_required: provide OGRN / registration id from master data
- MissingCriticalData_3:
  - field: declarant_profile
  - status: unresolved
  - expected_source: Alta card / operator
  - source_class_expected: alta_master_data
  - reason: shipment docs do not supply valid declarant master data for new dt
  - impact: blocks graph 14 completeness
  - action_required: provide declarant card data
- MissingCriticalData_4:
  - field: representative_profile
  - status: unresolved
  - expected_source: authority docs / Alta card / operator
  - source_class_expected: alta_master_data
  - reason: reference has old dt representative data only; current-case master-data not confirmed for new dt
  - impact: blocks graph 54 completeness
  - action_required: provide actual representative data for new dt
- MissingCriticalData_5:
  - field: unit_code
  - status: unresolved
  - expected_source: directory / operator / validated reference mapping
  - source_class_expected: operator
  - reason: unit name is documented as PCS/ШТ, but Alta code 796 comes from reference layer only
  - impact: blocks safe xml projection
  - action_required: confirm unit code from directory or operator
- MissingCriticalData_6:
  - field: graph30_final_projection_rule
  - status: unresolved
  - expected_source: mapping analysis / operator confirmation
  - source_class_expected: mapping_rule
  - reason: docs show svh report number/date and registry number/date; final graph30 target field selection still depends on mapping rule
  - impact: blocks safe xml projection for graph 30
  - action_required: lock rule registry-number/date vs svh-report-number/date

---

# Ready-to-use summary

- facts_confirmed_enough_for_review: yes
- facts_confirmed_enough_for_ui_projection: partial
- facts_confirmed_enough_for_xml_projection: partial
- facts_confirmed_enough_for_mapping_rule_extraction: yes
- blocker_list:
  - insurance fact / amount unresolved
  - master data for graphs 8/9/14/54 unresolved
  - graph30 final rule unresolved
  - unit code not confirmed from non-reference source
- high_risk_fields:
  - insurance
  - graph30 final number/date
  - consignee master data contacts
  - graph31 final composed wording
- data_source_gaps:
  - shipment_docs_gap: страхование и финальная упаковочная строка graph31 не подтверждены полностью
  - alta_master_data_gap: графы 8/9/14/54
  - calculation_gap: курс, таможенная стоимость, статистическая стоимость, graph47
  - mapping_rule_gap: graph1/30/44/47 final rules
