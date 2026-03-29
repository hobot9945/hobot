# facts

## Meta

- case_name: ПриточнаяВентиляция
- source_folder: C:\hobot\alta\source\ПриточнаяВентиляция
- dt_scope: 1 ДТ / 1 товар
- status: in_progress
- ready_for_next_step: partial
- unresolved_conflicts_count: 0
- unresolved_missing_critical_data_count: 8
- note: migrated from older facts version into current facts_schema; approximate structure migration, not full rebuild 
  from primary docs

---

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
    - note: sales contract

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
    - note: маршрут, EXW, фрахт 1200 USD + 30000 RUB, адрес разгрузки

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
    - note: transit-route, customs data, graph44 candidates

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
    - note: отчет о принятии товаров на хранение; складской номер, зона, ячейка

- supporting_doc_6:
    - file: 1 Supplementary agreement to the _25AZC003.pdf
    - doc_kind: supplementary_agreement
    - number: pending
    - number_status: not_found
    - date: pending
    - date_status: not_found
    - role: supporting
    - source_class: document
    - note: файл есть, содержимое в текущей итерации не распознано

- supporting_doc_7:
    - file: ZENGO Proforma Invoice 25AZC003.pdf
    - doc_kind: other
    - number: 25AZC004
    - number_status: confirmed_document
    - date: 09.04.2025
    - date_status: confirmed_document
    - role: supporting
    - source_class: document
    - note: supporting/background only, not primary truth for shipment facts

### Reference-only documents

- reference_doc_1:
    - file: ПриточнаяВентиляцияДТВыгрузка.xml
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

- reference_doc_3:
    - file: СкриншотыДТ\дт1.png .. дт7.png
    - role: reference_only
    - status: sample_only
    - source_class: sample
    - note: UI/presentation reference only

### Noise / excluded materials

- noise_doc_1:
    - file: База Ningbo ZENTEC .xlsx
    - role: noise
    - note: вспомогательный материал, не использован как прямой источник shipment facts

- noise_doc_2:
    - file: Хлам\25AZC003B CI & PL.pdf
    - role: noise
    - note: дублирующий/вспомогательный материал

---

# shipment_facts

## General shipment facts

- declaration_basis:
    - value: import
    - status: derived
    - source: комплект документов поставки и transit docs
    - source_class: derived
    - transfer_relevance: needed_for_review
    - blocking_for_next_step: no
    - note: business-level characterization, not final graph 1 code

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
    - source: sender country / dispatch country
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
    - source: origin country name
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
    - note: коммерческое место отгрузки, не фактический departure_place

---

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
    - source: sender address
    - source_class: derived
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: no

- country_name:
    - value: Китай
    - status: derived
    - source: sender address
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
    - transfer_relevance: needed_for_xml
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
    - value: ООО "СКИФ"
    - status: confirmed_document
    - source: contract, invoice, packing list, payment, transit declaration
    - source_class: document
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: no
    - note: migrated normalization from OCR-like form ООО ?СКИФ?

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
    - source: shipment docs
    - source_class: document
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- country_code:
    - value: RU
    - status: derived
    - source: consignee address
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
    - note: shipment-doc phone; do not auto-replace with reference/master-data values

- email:
    - value: pending
    - status: not_found
    - source: shipment docs current case
    - source_class: document
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

---

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
    - note: not equal to consignee legal address and not equal to warehouse address

- carrier_name:
    - value: ООО "АСД-ТРАНС"
    - status: confirmed_document
    - source: transit declaration, svh report
    - source_class: document
    - transfer_relevance: needed_for_review
    - blocking_for_next_step: no
    - note: migrated normalization from OCR-like form

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

### Transport / customs candidates observed in documents or reference

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
    - note: interpretation pending; do not merge process-code and normal country code

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

---

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
    - source: svh docs
    - source_class: document
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
    - note: business fact, not final graph30 code

- document_kind_code:
    - value: pending
    - status: sample_only
    - source: reference
    - source_class: sample
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: yes
    - note: left as migrated candidate only, not production-confirmed rule

- warehouse_document_number:
    - value: 0000478
    - status: confirmed_document
    - source: svh report
    - source_class: document
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: no
    - note: report number; final G_30_1 may differ

- warehouse_document_date:
    - value: 14.07.2025
    - status: confirmed_document
    - source: svh report
    - source_class: document
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: no
    - note: report date; final G_30_DATE may differ

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
    - source: transit declaration destination customs and svh context
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
    - note: presentation-layer candidate only

---

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
    - blocking_for_next_step: yes
    - note: left as confirmed from migrated facts; final classification still should be treated cautiously in future 
      rebuild if needed

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
    - note: migrated as provisional code candidate only

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
    - note: final graph31 packaging line still needs composition

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
        - 1000 шт., 2 места, 383 кг брутто, 312.5 кг нетто
    - status: derived
    - source: goods facts and tech description
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
    - note: material set for graph 31 composition

- description_31_main_draft:
    - value: pending
    - status: sample_only
    - source: reference and source facts
    - source_class: sample
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: yes
    - note: migrated placeholder only, not a ready shipment fact

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
    - note: structure/presentation hint only

- group_description:
    - value: КИВ-125 - клапан инфильтрации воздуха
    - status: derived
    - source: goods facts, tech description
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no

---

# alta_master_data_requirements

## Consignee / declarant / financial responsible master data

### consignee_profile:

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
    - note: shipment phone exists separately; this field is master-data profile phone

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
    - note: representation mode, not business fact

### financial_responsible_profile:

- name:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- inn_kpp:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- ogrn_or_registration_id:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- country_code:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- country_name:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: needed_for_review
    - blocking_for_next_step: no

- postcode:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- region_or_area:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- city_or_locality:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- street:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- house:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- office:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- phone:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- email:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / document
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
    - note: representation mode, not business fact

### declarant_profile:

- name:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- inn_kpp:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- ogrn_or_registration_id:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- country_code:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- country_name:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: needed_for_review
    - blocking_for_next_step: no

- address_raw:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- postcode:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- region_or_area:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- city_or_locality:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- street:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- house:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- office:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

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

### representative_profile:

- last_name:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- first_name:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- middle_name:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- phone:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- email:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- role_or_status:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: needed_for_review
    - blocking_for_next_step: no

- customs_representative_code:
    - value: pending
    - status: pending
    - source: operator / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- passport_type:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- passport_series:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- passport_number:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- passport_date:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- passport_issuer:
    - value: pending
    - status: pending
    - source: operator / alta_master_data / authority docs
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- authority_doc_name:
    - value: pending
    - status: pending
    - source: operator / authority docs / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- authority_doc_number:
    - value: pending
    - status: pending
    - source: operator / authority docs / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- authority_doc_date_from:
    - value: pending
    - status: pending
    - source: operator / authority docs / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- authority_doc_date_to:
    - value: pending
    - status: pending
    - source: operator / authority docs / alta_master_data
    - source_class: alta_master_data
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- printed_block_candidate:
    - value: pending
    - status: pending
    - source: representative fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
    - note: migrated placeholder only

---

# calculated_requirements

## Declaration mode / procedure / transport codes

- declaration_direction:
    - value: pending
    - status: pending
    - source: operator / process rule / sample
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes
    - note: not rebuilt yet; mapping side exists separately

- declaration_procedure_code:
    - value: pending
    - status: pending
    - source: operator / process rule / sample
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- electronic_declaration_flag:
    - value: pending
    - status: pending
    - source: operator / system / process rule
    - source_class: calculated
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- forms_main:
    - value: pending
    - status: pending
    - source: calculated / operator
    - source_class: calculated
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- forms_additional:
    - value: pending
    - status: pending
    - source: calculated / operator
    - source_class: calculated
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- border_transport_mode_code:
    - value: pending
    - status: pending
    - source: operator / process rule / sample / logistics candidates
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- departure_transport_mode_code:
    - value: pending
    - status: pending
    - source: operator / process rule / sample / logistics candidates
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- transport_identification_for_declaration:
    - value: A488ОУ67 / A6726I5
    - status: confirmed_document
    - source: cmr / transit declaration
    - source_class: document
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- transport_registration_country_code:
    - value: pending
    - status: pending
    - source: operator / document / sample
    - source_class: calculated
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- container_flag:
    - value: pending
    - status: pending
    - source: operator / document / sample
    - source_class: calculated
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- border_customs_code:
    - value: 10719110
    - status: confirmed_document
    - source: transit declaration
    - source_class: document
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: no

- border_customs_name:
    - value: т/п МАПП Забайкальск
    - status: confirmed_document
    - source: transit declaration
    - source_class: document
    - transfer_relevance: conditional_for_ui
    - blocking_for_next_step: no

## Valuation inputs and outputs

- transport_cost_to_border:
    - value: 624.00
    - currency: USD
    - status: confirmed_document
    - source: transport invoice
    - source_class: document
    - transfer_relevance: needed_for_calculation
    - blocking_for_next_step: yes

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
    - note: docs mention insurance conditionally, actual insurance fact/amount not confirmed

- exchange_rate:
    - value: pending
    - status: pending
    - source: system / operator / calculation rule
    - source_class: calculated
    - transfer_relevance: needed_for_calculation
    - blocking_for_next_step: yes

- exchange_rate_date_basis:
    - value: pending
    - status: pending
    - source: system / operator / calculation rule
    - source_class: calculated
    - transfer_relevance: needed_for_calculation
    - blocking_for_next_step: yes

- customs_value_total:
    - value: pending
    - currency: RUB
    - status: pending
    - source: calculation
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- statistical_value_total:
    - value: pending
    - currency: USD
    - status: pending
    - source: calculation
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

### GoodsCalculated[1]

- item_no:
    - value: 1
    - status: derived
    - source: goods linkage
    - source_class: derived
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: no

- preference_code:
    - value: pending
    - status: pending
    - source: operator / calculation / reference
    - source_class: calculated
    - transfer_relevance: conditional_for_xml
    - blocking_for_next_step: yes

- procedure_code:
    - value: pending
    - status: pending
    - source: operator / calculation / reference
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- customs_value:
    - value: pending
    - currency: RUB
    - status: pending
    - source: calculation
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- statistical_value:
    - value: pending
    - currency: USD
    - status: pending
    - source: calculation
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

### Payments[1]

- payment_code:
    - value: pending
    - status: pending
    - source: calculation / tariff rule
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- payment_base:
    - value: pending
    - status: pending
    - source: calculation
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- payment_rate:
    - value: pending
    - status: pending
    - source: tariff rule / calculation
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- payment_amount:
    - value: pending
    - status: pending
    - source: calculation
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- payment_sp:
    - value: pending
    - status: pending
    - source: operator / process rule / calculation
    - source_class: calculated
    - transfer_relevance: needed_for_xml
    - blocking_for_next_step: yes

- payment_summary_line:
    - value: pending
    - status: pending
    - source: payments / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
    - note: migrated placeholder

---

# mapping_rules

## MappingRule

- rule_name: graph30_type_code_rule
    - value: 11
    - status: confirmed_case_pattern
    - source: reference xml / txt / old dt
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - confidence: medium
    - note: observed graph30 type code for temporary storage case

- rule_name: graph30_document_kind_code_rule
    - value: 2
    - status: confirmed_case_pattern
    - source: reference xml / txt / old dt
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - confidence: medium
    - note: observed graph30 document kind code

- rule_name: graph30_registry_document_number_rule
    - value: 10404/141210/10092/04
    - status: confirmed_case_pattern
    - source: cmr / svh registry plus reference final graph30
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - confidence: medium
    - note: final graph30 uses registry number, not svh report number

- rule_name: graph30_registry_document_date_rule
    - value: 21.08.2019
    - status: confirmed_case_pattern
    - source: cmr / svh registry plus reference final graph30
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - confidence: medium
    - note: final graph30 uses registry date, not svh report date

## Declaration header mapping rules

- declaration_direction_code_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: not migrated in detail yet

- declaration_procedure_code_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: not migrated in detail yet

- electronic_declaration_flag_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- forms_main_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- forms_additional_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

## Transport / border mapping rules

- graph18_kind_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: not migrated in detail yet

- graph18_registration_country_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- graph19_container_flag_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- graph21_kind_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- graph25_transport_mode_rule:
    - value: pending
    - status: pending
    - source: reference / operator / process
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- graph26_transport_mode_rule:
    - value: pending
    - status: pending
    - source: reference / operator / process
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- graph29_customs_code_rule:
    - value: pending
    - status: pending
    - source: reference / operator / process
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- graph29_customs_name_rule:
    - value: pending
    - status: pending
    - source: reference / operator / directory
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

## Warehouse / graph 30 mapping rules

- graph30_type_code_rule:
    - value: 11
    - status: confirmed_case_pattern
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: code of graph 30 type

- graph30_document_kind_code_rule:
    - value: 2
    - status: confirmed_case_pattern
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: graph 30 document kind code

- graph30_registry_document_number_rule:
    - value: 10404/141210/10092/04
    - status: confirmed_case_pattern
    - source: reference xml / screenshot / operator / registry docs
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: final G_30_1 may differ from svh report number

- graph30_registry_document_date_rule:
    - value: 21.08.2019
    - status: confirmed_case_pattern
    - source: reference xml / screenshot / operator / registry docs
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: final G_30_DATE may differ from svh report date

- graph30_customs_code_rule:
    - value: 10404083
    - status: confirmed_case_pattern
    - source: reference xml / screenshot / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: migrated from warehouse customs candidate / reference pattern

- graph30_printed_address_rule:
    - value: pending
    - status: pending
    - source: reference xml / screenshot
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: presentation composition rule not migrated in detail

## Graph 44 mapping rules

### Graph44MappingRule[1]

- business_role: contract
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

- business_role: invoice
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

- business_role: packing_list
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

- business_role: cmr
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

- business_role: payment
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

- business_role: transport_invoice
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

- business_role: transport_contract
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

- business_role: tech_description
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

- business_role: transit_doc
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

- business_role: svh_doc
    - xml_code: pending
    - xml_subcode: pending
    - status: pending
    - source: reference xml / screenshot / operator / diff analysis
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: matrix not migrated in detail yet

## Representation rules

- graph8_same_as_graph14_rule:
    - value: pending
    - status: pending
    - source: operator / reference ui
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: not migrated in detail yet

- graph9_same_as_graph14_rule:
    - value: pending
    - status: pending
    - source: operator / reference ui
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- graph42_value_in_dts_rule:
    - value: pending
    - status: pending
    - source: reference ui / operator / process
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- graph31_absent_trademark_representation_rule:
    - value: pending
    - status: pending
    - source: reference ui / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- graph31_absent_brand_representation_rule:
    - value: pending
    - status: pending
    - source: reference ui / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping

- payment_pattern_rule:
    - value: pending
    - status: pending
    - source: reference ui / xml / operator
    - source_class: mapping_rule
    - transfer_relevance: needed_for_mapping
    - note: not migrated in detail yet

---

# documents_for_graph44_candidates

## Graph44Candidate[1]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: 25AZC003
- date: 10.04.2025
- valid_from: pending
- valid_to: pending
- source: contract
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

## Graph44Candidate[2]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: 25AZC003B
- date: 10.04.2025
- valid_from: pending
- valid_to: pending
- source: invoice
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

## Graph44Candidate[3]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: БН
- date: 10.04.2025
- valid_from: pending
- valid_to: pending
- source: packing list
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

## Graph44Candidate[4]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: 12327
- date: 01.07.2025
- valid_from: pending
- valid_to: pending
- source: cmr
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

## Graph44Candidate[5]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: 1
- date: 21.05.2025
- valid_from: pending
- valid_to: pending
- source: payment
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

## Graph44Candidate[6]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: 25-12327-k
- date: 22.05.2025
- valid_from: pending
- valid_to: pending
- source: transport invoice
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

## Graph44Candidate[7]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: КООО/26651/М
- date: 13.05.2025
- valid_from: pending
- valid_to: pending
- source: transport request, transport invoice
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

## Graph44Candidate[8]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: 1СК1004
- date: 10.04.2025
- valid_from: pending
- valid_to: pending
- source: tech description
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

## Graph44Candidate[9]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: 10719110/060725/5070039
- date: 06.07.2025
- valid_from: pending
- valid_to: pending
- source: transit declaration
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

## Graph44Candidate[10]

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
- doc_subcode_candidate: pending
- doc_subcode_candidate_status: pending
- number: 0000478
- date: 14.07.2025
- valid_from: pending
- valid_to: pending
- source: svh report
- source_class: document
- transfer_relevance: needed_for_xml
- doc_text:
    - value: pending
    - status: pending
    - source: candidate fields / reference
    - source_class: derived
    - transfer_relevance: needed_for_ui
    - blocking_for_next_step: no
- system_observed_binding:
    - value: pending
    - status: not_found
    - source: reference xml / txt
    - source_class: system
    - transfer_relevance: do_not_transfer
    - blocking_for_next_step: no
- note: migrated from previous facts

---

# reference_observed

## Reference[1]

- field_name: graph44_role_to_code_matrix
- observed_in: xml_export / txt_export / old_dt_pdf
- value: role-to-code mapping observed for contract, invoice, payment, transport invoice, transport contract, 
  packing list, tech description, transit declaration, svh report
- status: sample_only
- source_class: sample
- transfer_relevance: reference_only
- relevance_for_next_step: useful_for_mapping
- comment: do not use as shipment fact

## Reference[2]

- field_name: unit_code_reference_candidate
- observed_in: xml_export / txt_export / old_dt_pdf
- value: 796
- status: sample_only
- source_class: sample
- transfer_relevance: reference_only
- relevance_for_next_step: useful_for_mapping
- comment: observed code for unit ШТ/PCS in reference, not yet validated as production input

## Reference[3]

- field_name: graph30_final_fields_reference_pattern
- observed_in: xml_export / txt_export / old_dt_pdf
- value: type=11; doc_kind=2; customs_code=10404083; registry number/date used instead of svh report number/date
- status: sample_only
- source_class: sample
- transfer_relevance: reference_only
- relevance_for_next_step: useful_for_mapping
- comment: pattern only, not shipment fact

## Reference[4]

- field_name: graph31_presentation_lines
- observed_in: screenshot / old_dt_pdf / xml_export
- value: migrated sample lines for graph 31 wording/composition
- status: sample_only
- source_class: sample
- transfer_relevance: reference_only
- relevance_for_next_step: useful_for_process_rule
- comment: use only for structure/presentation hints

---

# system_only

## SystemField[1]

- field_name: reference_xml_system_bindings
- observed_in: xml_export / txt_export / screenshot
- value: ED_ID / ED_STAT / BACK / FACE / registration-related bindings not migrated into working facts
- source_class: system
- transfer_relevance: do_not_transfer
- comment: system layer excluded from new DT

---

# Conflicts

- Конфликтов по подтвержденным документам не выявлено.
- note: previous pseudo-conflicts were not preserved as real conflicts during migration

---

# Missing critical data

## MissingCriticalData[1]

- field: insurance
- status: unresolved
- expected_source: insurance policy / operator confirmation / explicit no-insurance confirmation
- source_class_expected: operator
- reason: docs mention insurance conditionally but do not confirm actual insurance fact or amount
- impact: blocks valuation calculation
- action_required: confirm insured / not insured and amount if applicable

## MissingCriticalData[2]

- field: consignee_master_data_registration_id
- status: unresolved
- expected_source: Alta card / operator
- source_class_expected: alta_master_data
- reason: not present in shipment docs
- impact: blocks graph 8 completeness
- action_required: provide OGRN / registration id from master data

## MissingCriticalData[3]

- field: financial_responsible_profile
- status: unresolved
- expected_source: Alta card / operator
- source_class_expected: alta_master_data
- reason: graph 9 profile not reconstructed in migrated facts
- impact: blocks graph 9 completeness
- action_required: provide financial responsible card data

## MissingCriticalData[4]

- field: declarant_profile
- status: unresolved
- expected_source: Alta card / operator
- source_class_expected: alta_master_data
- reason: shipment docs do not supply valid declarant master data for new dt
- impact: blocks graph 14 completeness
- action_required: provide declarant card data

## MissingCriticalData[5]

- field: representative_profile
- status: unresolved
- expected_source: authority docs / Alta card / operator
- source_class_expected: alta_master_data
- reason: current-case representative master data not confirmed for new dt
- impact: blocks graph 54 completeness
- action_required: provide actual representative data for new dt

## MissingCriticalData[6]

- field: unit_code
- status: unresolved
- expected_source: directory / operator / validated mapping
- source_class_expected: operator
- reason: unit name is documented, but code 796 currently comes only from reference layer
- impact: blocks safe xml projection
- action_required: confirm unit code from directory or operator

## MissingCriticalData[7]

- field: graph30_final_projection_rule
- status: unresolved
- expected_source: mapping analysis / operator confirmation
- source_class_expected: mapping_rule
- reason: docs show svh report number/date and registry number/date; final graph30 target selection still depends 
  on mapping rule
- impact: blocks safe xml projection for graph 30
- action_required: lock rule registry-number/date vs svh-report-number/date

## MissingCriticalData[8]

- field: valuation_and_payments_block
- status: unresolved
- expected_source: calculation
- source_class_expected: calculated
- reason: exchange rate, customs value, statistical value and graph47 payments not built in migrated facts
- impact: blocks final xml-ready state
- action_required: perform calculation stage after schema stabilization

---

# Ready-to-use summary

- facts_confirmed_enough_for_review: yes
- facts_confirmed_enough_for_ui_projection: partial
- facts_confirmed_enough_for_xml_projection: partial
- facts_confirmed_enough_for_mapping_rule_extraction: partial
- blocker_list:
    - insurance fact / amount unresolved
    - master data for graphs 8/9/14/54 unresolved
    - graph30 final rule unresolved
    - unit code not confirmed from non-reference source
    - valuation / graph47 not built
- high_risk_fields:
    - insurance
    - graph30 final number/date
    - unit_code
    - graph31 final composed wording
    - graph44 final role-to-code matrix
- data_source_gaps:
    - shipment_docs_gap: страхование и финальная graph31 composition не подтверждены полностью
    - alta_master_data_gap: графы 8/9/14/54
    - calculation_gap: курс, таможенная стоимость, статистическая стоимость, graph47
    - mapping_rule_gap: graph1/18/19/21/25/26/29/30/44/47 final rules