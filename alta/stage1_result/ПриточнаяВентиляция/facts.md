# facts.md

## Meta

- case_name: ПриточнаяВентиляция
- source_folder: alta\source\ПриточнаяВентиляция
- dt_scope: 1 ДТ / 1 товар
- status: in_progress
- ready_for_next_step: partial
- unresolved_conflicts_count: 2
- unresolved_missing_critical_data_count: 4
- note: собран основной слой первички по контракту, инвойсу, PL, платежке, CMR, перевозке, СВХ, техописанию; 
  часть mapping/calculation еще не закрыта

---

## Documents package

### documents_package

- contract:
  - file: SALES CONTRACT No25AZC003.pdf
  - status: confirmed_document
  - number: 25AZC003
  - number_status: confirmed_document
  - date: 10.04.2025
  - date_status: confirmed_document
  - role: core
  - source_class: document
  - note: рамочный договор; конкретные quantity/price по поставке отнесены в invoice

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
  - number_status: not_found
  - date: 10.04.2025
  - date_status: confirmed_document
  - role: core
  - source_class: document
  - note: связан с invoice 25AZC003B

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
  - file: тех описание\техничка КИВ 125.pdf
  - status: confirmed_document
  - number: 1СК1004
  - number_status: confirmed_document
  - date: 10.04.2025
  - date_status: confirmed_document
  - role: core
  - source_class: document

### Supporting documents

- supporting_doc_supplementary_agreement:
  - file: 1 Supplementary agreement to the _25AZC003.pdf
  - doc_kind: supplementary_agreement
  - number: 25AZC003
  - number_status: confirmed_document
  - date: 10.04.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: содержит банковские реквизиты сторон; не заменяет основной контракт

- supporting_doc_transport_request:
  - file: документы от Любы\Заявка номер 1 от 21.05.2025.pdf
  - doc_kind: transport_request
  - number: 1
  - number_status: confirmed_document
  - date: 21.05.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: заявка на перевозку по договору КООО/26651/М

- supporting_doc_transport_invoice_usd:
  - file: документы от Любы\Счет_№25-12327-k_от_22-05-2025 (2).pdf
  - doc_kind: transport_invoice
  - number: 25-12327-k
  - number_status: confirmed_document
  - date: 22.05.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: международное и пограничное/междугороднее плечо, 1200 USD

- supporting_doc_transport_invoice_rub:
  - file: документы от Любы\Счет_№25-12327-k_1_от_22-05-2025 (3).pdf
  - doc_kind: transport_invoice
  - number: 25-12327-k/1
  - number_status: confirmed_document
  - date: 22.05.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: локальное плечо внутри Набережных Челнов, 30000 RUB

- supporting_doc_transit_doc:
  - file: документы от Любы\ТД_12327.pdf
  - doc_kind: transit_doc
  - number: 10719110/060725/5070039
  - number_status: confirmed_document
  - date: 06.07.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: транзитная декларация; использовать как supporting/reference для логистики и graph 44

- supporting_doc_svh_report:
  - file: СВХ\до.pdf
  - doc_kind: svh_doc
  - number: 0000478
  - number_status: confirmed_document
  - date: 14.07.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: отчет о принятии товаров на хранение

- supporting_doc_svh_notification:
  - file: СВХ\ВТамПостНабережныхЧелнов.pdf
  - doc_kind: svh_doc
  - number: not_found
  - number_status: not_found
  - date: 14.07.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: уведомление на регистрацию документов для помещения на временное хранение

- supporting_doc_proforma_invoice:
  - file: доки от Клары\ZENGO Proforma Invoice 25AZC003.pdf
  - doc_kind: other
  - number: 25AZC004
  - number_status: confirmed_document
  - date: 09.04.2025
  - date_status: confirmed_document
  - role: supporting
  - source_class: document
  - note: предварительный коммерческий документ; не использовать как основной источник фактов поставки поверх invoice/payment

### Reference-only documents

- reference_doc_old_dt:
  - file: 1\GTD_10418010_150725_5103886.pdf
  - role: reference_only
  - status: sample_only
  - source_class: sample
  - note: использовать только для структуры, полноты, mapping и process-rule analysis

---

# shipment_facts

## General shipment facts

- declaration_basis:
  - value: import
  - status: derived
  - source: комплект документов поставки / транзит / характер сделки
  - source_class: derived
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - note: бизнес-уровень; финальный код графы 1 не выводить отсюда напрямую

- incoterms_code:
  - value: EXW
  - status: confirmed_document
  - source: invoice / packing_list / transport_request
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- incoterms_place:
  - value: Ningbo
  - status: confirmed_document
  - source: invoice / packing_list
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- invoice_currency_alpha:
  - value: CNY
  - status: confirmed_document
  - source: invoice / payment_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- invoice_currency_numeric:
  - value: 156
  - status: derived
  - source: invoice_currency_alpha=CNY
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- invoice_amount_total:
  - value: 13600.00
  - currency: CNY
  - status: confirmed_document
  - source: invoice / payment_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- package_count_total:
  - value: 2
  - status: confirmed_document
  - source: packing_list / CMR / transit_doc / svh_report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- gross_weight_total:
  - value: 383.00
  - unit: kg
  - status: confirmed_document
  - source: packing_list / CMR / transit_doc / svh_report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- net_weight_total:
  - value: 312.50
  - unit: kg
  - status: confirmed_document
  - source: packing_list
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- trade_country_name:
  - value: Китай
  - status: derived
  - source: продавец / invoice / origin / dispatch
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- trade_country_code:
  - value: CN
  - status: derived
  - source: trade_country_name
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- dispatch_country_name:
  - value: Китай
  - status: confirmed_document
  - source: invoice / CMR / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- dispatch_country_code:
  - value: CN
  - status: derived
  - source: dispatch_country_name
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- origin_country_name:
  - value: Китай
  - status: confirmed_document
  - source: invoice / tech_description / proforma
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- origin_country_code:
  - value: CN
  - status: derived
  - source: origin_country_name
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- destination_country_name:
  - value: Россия
  - status: confirmed_document
  - source: invoice / CMR / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- destination_country_code:
  - value: RU
  - status: derived
  - source: destination_country_name
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- commercial_shipment_point:
  - value: Ningbo, China
  - status: confirmed_document
  - source: invoice / packing_list / proforma
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - note: коммерческое место отгрузки; не равно фактическому месту отправки по CMR

---

## Parties

### Sender

- name:
  - value: Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd
  - status: confirmed_document
  - source: contract / invoice / CMR / tech_description
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- country_code:
  - value: CN
  - status: derived
  - source: sender address / China
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- country_name:
  - value: Китай
  - status: confirmed_document
  - source: sender address
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- postcode:
  - value: 315175
  - status: confirmed_document
  - source: contract / invoice / CMR
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- region_or_area:
  - value: Haishu District
  - status: confirmed_document
  - source: contract / invoice / CMR
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- city_or_locality:
  - value: Ningbo
  - status: confirmed_document
  - source: contract / invoice / CMR
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- street_address:
  - value: D4-109, Liangzhu Culture Park
  - status: confirmed_document
  - source: contract / invoice / CMR
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- house:
  - value: D4-109
  - status: derived
  - source: sender address
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- office:
  - value: not_found
  - status: not_found
  - source: documents reviewed
  - source_class: document
  - transfer_relevance: optional
  - blocking_for_next_step: no

- phone:
  - value: +86 574 8907 9907
  - status: confirmed_document
  - source: contract / invoice / tech_description
  - source_class: document
  - transfer_relevance: optional
  - blocking_for_next_step: no

- email:
  - value: not_found
  - status: not_found
  - source: documents reviewed
  - source_class: document
  - transfer_relevance: optional
  - blocking_for_next_step: no

### Consignee

- name:
  - value: ООО «СКИФ»
  - status: confirmed_document
  - source: contract / invoice / CMR / payment / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- inn_kpp:
  - value: 1650389298/165001001
  - status: confirmed_document
  - source: transit_doc / transport invoices
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- ogrn:
  - value: 1201600020390
  - status: sample_only
  - source: reference old DT
  - source_class: sample
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes
  - note: не подтвержден поставочной первичкой; нужен как master data/operator if required

- country_code:
  - value: RU
  - status: derived
  - source: consignee address
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- country_name:
  - value: Россия
  - status: confirmed_document
  - source: consignee address
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- postcode:
  - value: 423800
  - status: confirmed_document
  - source: contract / invoice / CMR / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- region_or_area:
  - value: Республика Татарстан
  - status: confirmed_document
  - source: contract / invoice / CMR / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- city_or_locality:
  - value: Набережные Челны
  - status: confirmed_document
  - source: contract / invoice / CMR / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- street:
  - value: проезд Хлебный
  - status: confirmed_document
  - source: contract / invoice / CMR / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- house:
  - value: 30
  - status: confirmed_document
  - source: contract / invoice / CMR / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- office:
  - value: 211
  - status: confirmed_document
  - source: contract / invoice / CMR / transit_doc
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- phone:
  - value: +7 937 779-26-56
  - status: confirmed_document
  - source: contract / invoice / supplementary_agreement
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- email:
  - value: prom_tat@mail.ru
  - status: sample_only
  - source: proforma / reference old DT
  - source_class: sample
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes
  - note: не подтверждено базовой первичкой как реквизит для новой ДТ

---

## Logistics and transport

- transport_doc_number:
  - value: 12327
  - status: confirmed_document
  - source: CMR / transit_doc / svh_notification
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- transport_doc_date:
  - value: 01.07.2025
  - status: confirmed_document
  - source: CMR / transit_doc / svh_notification
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- departure_place:
  - value: Маньчжурия, Китай
  - status: confirmed_document
  - source: CMR
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- delivery_place:
  - value: Набережные Челны, Производственный пр-д, д. 45, Россия
  - status: confirmed_document
  - source: CMR / svh docs
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- carrier_name:
  - value: ООО «АСД-ТРАНС»
  - status: confirmed_document
  - source: CMR stamp / transit_doc / svh_report
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- carrier_tax_id:
  - value: 6732148782
  - status: confirmed_document
  - source: transit_doc
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- border_transport_description:
  - value: авто
  - status: confirmed_document
  - source: transport_request / transport_invoice_usd / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

### Transport / customs candidates observed in documents or reference

- transport_identification_departure_candidate:
  - value: A488OY67 / A6726I5
  - status: confirmed_document
  - source: transit_doc / CMR / svh_report
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no
  - note: candidate for graph 18/21, not final process value

- transport_registration_country_code_candidate:
  - value: RU
  - status: confirmed_document
  - source: transit_doc
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- container_flag_candidate:
  - value: 0
  - status: confirmed_document
  - source: transit_doc
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- border_transport_mode_code_candidate:
  - value: 31
  - status: sample_only
  - source: reference old DT
  - source_class: sample
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: yes

- departure_transport_mode_code_candidate:
  - value: 31
  - status: sample_only
  - source: reference old DT
  - source_class: sample
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: yes

- border_customs_code_candidate:
  - value: 10404083
  - status: confirmed_document
  - source: CMR / transit_doc / svh docs
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- border_customs_name_candidate:
  - value: ОТО и ТК №3 т/п Набережночелнинский
  - status: confirmed_document
  - source: CMR / transit_doc
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

---

## Warehouse / goods location (document-confirmed only)

- warehouse_document_exists:
  - value: yes
  - status: confirmed_document
  - source: svh_report / svh_notification / CMR
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- location_type:
  - value: temporary_storage_warehouse
  - status: confirmed_document
  - source: svh docs
  - source_class: document
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - note: бизнес-факт хранения на СВХ, не код поля Альты

- document_kind_code:
  - value: pending
  - status: pending
  - source: operator / mapping rule
  - source_class: operator
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: yes
  - note: код вида документа графы 30 не подтвержден как rule

- warehouse_document_number:
  - value: 0000478
  - status: confirmed_document
  - source: svh_report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no
  - note: номер отчета о принятии на хранение

- warehouse_document_date:
  - value: 14.07.2025
  - status: confirmed_document
  - source: svh_report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- warehouse_registry_number:
  - value: 0000478
  - status: pending
  - source: svh_report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes
  - note: сильный кандидат, но финальное использование в графе 30 не закреплено; reference old DT показывает 0000487

- warehouse_registry_date:
  - value: 14.07.2025
  - status: pending
  - source: svh_report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes
  - note: сильный кандидат, но rule финального использования не закреплен

- warehouse_address_raw:
  - value: 423800, Республика Татарстан, г. Набережные Челны, Производственный пр-д, д. 45
  - status: confirmed_document
  - source: CMR / svh docs
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- warehouse_country_code:
  - value: RU
  - status: derived
  - source: warehouse_address_raw
  - source_class: derived
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- warehouse_region_or_area:
  - value: Республика Татарстан
  - status: derived
  - source: warehouse_address_raw
  - source_class: derived
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- warehouse_city_or_locality:
  - value: Набережные Челны
  - status: derived
  - source: warehouse_address_raw
  - source_class: derived
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- warehouse_street_address:
  - value: Производственный пр-д, д. 45
  - status: derived
  - source: warehouse_address_raw
  - source_class: derived
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- warehouse_customs_code_candidate:
  - value: 10404083
  - status: confirmed_document
  - source: CMR / transit_doc / svh docs
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

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
  - value: приточный клапан
  - status: confirmed_document
  - source: tech_description / CMR
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no
  - note: техописание имеет приоритет над более общими коммерческими формулировками

- product_name_en:
  - value: air infiltration valve
  - status: confirmed_document
  - source: invoice
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- model:
  - value: КИВ-125
  - status: confirmed_document
  - source: tech_description / packing_list
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- article:
  - value: not_found
  - status: not_found
  - source: documents reviewed
  - source_class: document
  - transfer_relevance: optional
  - blocking_for_next_step: no

- hs_code:
  - value: 8481309908
  - status: confirmed_document
  - source: invoice / CMR / transit_doc / tech_description
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- manufacturer:
  - value: Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd
  - status: confirmed_document
  - source: invoice / tech_description
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- trademark:
  - value: отсутствует
  - status: pending
  - source: reference old DT / absence in current docs
  - source_class: sample
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes
  - note: в текущей первичке не подтверждено явно как business fact

- brand:
  - value: отсутствует
  - status: pending
  - source: reference old DT / absence in current docs
  - source_class: sample
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes
  - note: в текущей первичке не подтверждено явно как business fact

- origin_country_code:
  - value: CN
  - status: derived
  - source: invoice / tech_description
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- quantity:
  - value: 1000
  - unit: pcs
  - status: confirmed_document
  - source: invoice / packing_list / proforma
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- unit_code:
  - value: 796
  - status: pending
  - source: operator / classifier
  - source_class: operator
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes
  - note: код единицы измерения не подтвержден первичкой; требуется справочник/оператор

- unit_name:
  - value: шт
  - status: confirmed_document
  - source: quantity context / proforma / xlsx support
  - source_class: document
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no

- package_places_count:
  - value: 2
  - status: confirmed_document
  - source: packing_list / CMR
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- package_places_marking:
  - value: PK-2
  - status: confirmed_document
  - source: transit_doc
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- gross_weight:
  - value: 383.00
  - unit: kg
  - status: confirmed_document
  - source: packing_list / CMR / transit_doc / svh_report
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- net_weight:
  - value: 312.50
  - unit: kg
  - status: confirmed_document
  - source: packing_list
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- invoice_value:
  - value: 13600.00
  - currency: CNY
  - status: confirmed_document
  - source: invoice / payment_doc
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

- description_31_source_facts:
  - value:
    - приточный клапан
    - модель КИВ-125
    - предназначен для подачи свежего воздуха в помещение
    - регулируемая подача воздуха
    - фильтрация
    - тепло- и шумоизоляция
    - пластиковая труба с оголовком
    - устанавливается в стену
    - диаметр воздуховода 125 мм
    - производитель Ningbo ZENTEC Air Conditioning & Refrigeration Co., Ltd
    - 1000 шт
  - status: derived
  - source: tech_description / invoice / packing_list
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no

---

# alta_master_data_requirements

## consignee_profile

- registration_id:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- phone:
  - value: +7 937 779-26-56
  - status: confirmed_document
  - source: contract / invoice
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no
  - note: можно использовать как candidate, но для master data лучше подтвердить карточкой/оператором

- email:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- same_as_graph14_mode:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

## financial_responsible_profile

- name:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- inn_kpp:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
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
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- country_name:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: needed_for_review
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
  - blocking_for_next_step: yes

- phone:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- email:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- same_as_graph14_mode:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

## declarant_profile

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
  - blocking_for_next_step: yes

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
  - blocking_for_next_step: yes

- phone:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- email:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

## representative_profile

- last_name:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- first_name:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- middle_name:
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
  - blocking_for_next_step: yes

- email:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- role_or_status:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: yes

- customs_representative_code:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- passport_type:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- passport_series:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- passport_number:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- passport_date:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- passport_issuer:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- authority_doc_name:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- authority_doc_number:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- authority_doc_date_from:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- authority_doc_date_to:
  - value: pending
  - status: pending
  - source: operator / alta_master_data
  - source_class: alta_master_data
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

---

# calculated_requirements

- declaration_direction:
  - value: pending
  - status: pending
  - source: operator / process rule / reference
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- declaration_procedure_code:
  - value: pending
  - status: pending
  - source: operator / process rule / reference
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- electronic_declaration_flag:
  - value: pending
  - status: pending
  - source: operator / process rule / reference
  - source_class: calculated
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- forms_main:
  - value: pending
  - status: pending
  - source: operator / reference
  - source_class: calculated
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- forms_additional:
  - value: pending
  - status: pending
  - source: operator / reference
  - source_class: calculated
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- border_transport_mode_code:
  - value: pending
  - status: pending
  - source: operator / process rule / reference
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- departure_transport_mode_code:
  - value: pending
  - status: pending
  - source: operator / process rule / reference
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- transport_identification_for_declaration:
  - value: A488OY67 / A6726I5
  - status: confirmed_document
  - source: CMR / transit_doc / svh_report
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- transport_registration_country_code:
  - value: pending
  - status: pending
  - source: operator / process rule / reference
  - source_class: calculated
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- container_flag:
  - value: 0
  - status: confirmed_document
  - source: transit_doc
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: no

- border_customs_code:
  - value: pending
  - status: pending
  - source: operator / process rule / reference
  - source_class: calculated
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: yes

- border_customs_name:
  - value: pending
  - status: pending
  - source: operator / directory / reference
  - source_class: calculated
  - transfer_relevance: conditional_for_ui
  - blocking_for_next_step: yes

## Valuation inputs and outputs

- transport_cost_to_border:
  - value: 624.00
  - currency: USD
  - status: confirmed_document
  - source: transport_invoice_usd
  - source_class: document
  - transfer_relevance: needed_for_calculation
  - blocking_for_next_step: no

- post_border_transport_cost:
  - value: 576.00
  - currency: USD
  - status: confirmed_document
  - source: transport_invoice_usd
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- local_delivery_cost:
  - value: 30000.00
  - currency: RUB
  - status: confirmed_document
  - source: transport_invoice_rub
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- insurance:
  - value: pending
  - status: pending
  - source: transport_request / proforma
  - source_class: document
  - transfer_relevance: needed_for_calculation
  - blocking_for_next_step: yes
  - note: есть only contractual/request-level observations; фактическое наличие/отсутствие не подтверждено

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
  - source: operator / reference
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

---

# Fact Composition

- warehouse_printed_address:
  - composition_rule: собрать печатную строку графы 30 из кода таможни, адреса местонахождения товара и реквизитов СВХ
  - value: 10404083, Республика Татарстан, г. Набережные Челны, Производственный пр-д, д. 45, СВХ ООО "ЛОГИКАМ", 
    отчет 0000478 от 14.07.2025
  - status: derived
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - compose_from:
    - shipment_facts.warehouse_goods_location.warehouse_customs_code_candidate
    - shipment_facts.warehouse_goods_location.warehouse_address_raw
    - shipment_facts.warehouse_goods_location.warehouse_document_number
    - shipment_facts.warehouse_goods_location.warehouse_document_date
  - note: presentation-only candidate, не финальный xml-value

- description_31_main_draft:
  - composition_rule: собрать черновик основной строки графы 31 из подтвержденных описательных facts товара
  - value: ПРИТОЧНЫЙ КЛАПАН КАНАЛЬНОГО ТИПА КИВ-125, ПРЕДНАЗНАЧЕН ДЛЯ ПОДАЧИ СВЕЖЕГО ВОЗДУХА В ПОМЕЩЕНИЕ, 
    УСТАНАВЛИВАЕТСЯ В СТЕНУ, ДИАМЕТР 125 ММ, ПРОИЗВОДИТЕЛЬ NINGBO ZENTEC AIR CONDITIONING & REFRIGERATION CO., LTD, 1000 ШТ.
  - status: derived
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - compose_from:
    - shipment_facts.goods.Goods[1].description_31_source_facts
    - shipment_facts.goods.Goods[1].product_name_ru
    - shipment_facts.goods.Goods[1].model
    - shipment_facts.goods.Goods[1].manufacturer
    - shipment_facts.goods.Goods[1].quantity
    - shipment_facts.goods.Goods[1].unit_name
  - note: draft only

- description_31_lines:
  - composition_rule: разложить описание графы 31 на presentation-строки
  - value:
    - ПРИТОЧНЫЙ КЛАПАН КАНАЛЬНОГО ТИПА, ГРАЖДАНСКОГО НАЗНАЧЕНИЯ
    - ПРЕДНАЗНАЧЕН ДЛЯ ПОДАЧИ СВЕЖЕГО ВОЗДУХА В ПОМЕЩЕНИЕ
    - УСТАНАВЛИВАЕТСЯ В СТЕНУ
    - МОДЕЛЬ: КИВ-125
    - ПРОИЗВОДИТЕЛЬ: NINGBO ZENTEC AIR CONDITIONING & REFRIGERATION CO., LTD
    - 1000 ШТ
  - status: derived
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - compose_from:
    - shipment_facts.goods.Goods[1].description_31_source_facts
    - fact_composition.description_31_main_draft

- group_description:
  - composition_rule: собрать краткое табличное описание товарной группы из наименования товара и модели
  - value: Приточный клапан КИВ-125
  - status: derived
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - compose_from:
    - shipment_facts.goods.Goods[1].product_name_ru
    - shipment_facts.goods.Goods[1].model

---

# mapping_rules

- declaration_direction_code_rule:
  - if: declaration_basis=import
  - value: ИМ
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- declaration_procedure_code_rule:
  - if: import / выпуск для внутреннего потребления
  - value: 40
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- electronic_declaration_flag_rule:
  - if: декларация подается в электронном виде
  - value: ЭД
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- forms_main_rule:
  - if: всегда
  - value: 1
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- forms_additional_rule:
  - if: всегда для данного кейса
  - value: 1
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph18_kind_rule:
  - if: pending
  - value: 2
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph18_registration_country_rule:
  - if: pending
  - value: 00
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph19_container_flag_rule:
  - if: container absent
  - value: 0
  - status: confirmed_case_pattern
  - source: transit_doc / reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph21_kind_rule:
  - if: pending
  - value: 1
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph25_transport_mode_rule:
  - if: автотранспорт на границе
  - value: 31
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph26_transport_mode_rule:
  - if: автотранспорт внутри страны
  - value: 31
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph29_customs_code_rule:
  - if: въезд через текущий транзитный маршрут
  - value: 10719110
  - status: confirmed_case_pattern
  - source: transit_doc / reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph29_customs_name_rule:
  - if: графа 29 = 10719110
  - value: т/п МАПП Забайкальск
  - status: confirmed_case_pattern
  - source: transit_doc / reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph30_type_code_rule:
  - if: хранение на СВХ
  - value: 11
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph30_document_kind_code_rule:
  - if: pending
  - value: pending
  - status: pending
  - source: reference old DT / operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph30_customs_code_rule:
  - if: хранение на СВХ ООО "ЛОГИКАМ" по текущему кейсу
  - value: 10404083
  - status: confirmed_case_pattern
  - source: CMR / transit_doc / reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph44_cmr_primary:
  - if: business_role=cmr
  - value:
    - business_role: cmr
    - code: 02015
    - subcode: 0
    - doc_number: documents_package.transport_doc.number
    - doc_date: documents_package.transport_doc.date
    - doc_name: СМР
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph44_contract_primary:
  - if: business_role=contract
  - value:
    - business_role: contract
    - code: 03011
    - subcode: 0
    - doc_number: documents_package.contract.number
    - doc_date: documents_package.contract.date
    - doc_name: КОНТРАКТ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph44_invoice_primary:
  - if: business_role=invoice
  - value:
    - business_role: invoice
    - code: 04021
    - subcode: 0
    - doc_number: documents_package.invoice.number
    - doc_date: documents_package.invoice.date
    - doc_name: КОММЕРЧЕСКИЙ ИНВОЙС
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph44_payment_primary:
  - if: business_role=payment
  - value:
    - business_role: payment
    - code: 04023
    - subcode: 0
    - doc_number: documents_package.payment_doc.number
    - doc_date: documents_package.payment_doc.date
    - doc_name: ЗАЯВЛЕНИЕ НА ПЕРЕВОД
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph44_transport_invoice_primary:
  - if: business_role=transport_invoice
  - value:
    - business_role: transport_invoice
    - code: 04031
    - subcode: 0
    - doc_number: supporting_doc_transport_invoice_usd.number
    - doc_date: supporting_doc_transport_invoice_usd.date
    - doc_name: СЧЕТ ЗА ПЕРЕВОЗКУ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph44_transport_contract_primary:
  - if: business_role=transport_contract
  - value:
    - business_role: transport_contract
    - code: 04033
    - subcode: 0
    - doc_number: supporting_doc_transport_request.note
    - doc_date: 13.05.2025
    - doc_name: ДОГОВОР ПО ПЕРЕВОЗКЕ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: transport docs / reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph44_packing_list_primary:
  - if: business_role=packing_list
  - value:
    - business_role: packing_list
    - code: 04131
    - subcode: 0
    - doc_number: БН
    - doc_date: documents_package.packing_list.date
    - doc_name: УПАКОВОЧНЫЙ ЛИСТ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph44_tech_description_primary:
  - if: business_role=tech_description
  - value:
    - business_role: tech_description
    - code: 05999
    - subcode: 0
    - doc_number: documents_package.tech_description.number
    - doc_date: documents_package.tech_description.date
    - doc_name: ТЕХ ОПИСАНИЕ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph44_transit_doc_primary:
  - if: business_role=transit_doc
  - value:
    - business_role: transit_doc
    - code: 09013
    - subcode: 0
    - doc_number: supporting_doc_transit_doc.number
    - doc_date: supporting_doc_transit_doc.date
    - doc_name: ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: transit_doc / reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- graph44_svh_report_primary:
  - if: business_role=svh_doc
  - value:
    - business_role: svh_doc
    - code: 09026
    - subcode: 0
    - doc_number: pending
    - doc_date: 14.07.2025
    - doc_name: ОТЧЕТ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: pending
  - source: svh_report / reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: old DT shows 0000487, current svh_report reads 0000478; exact doc number for mapping unresolved

- graph44_tech_description_attachment:
  - if: business_role=tech_description
  - value:
    - business_role: tech_description
    - code: 09023
    - subcode: 0
    - doc_number: documents_package.tech_description.number
    - doc_date: documents_package.tech_description.date
    - doc_name: ТЕХ ОПИСАНИЕ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph44_invoice_attachment:
  - if: business_role=invoice
  - value:
    - business_role: invoice
    - code: 09023
    - subcode: 0
    - doc_number: documents_package.invoice.number
    - doc_date: documents_package.invoice.date
    - doc_name: КОММЕРЧЕСКИЙ ИНВОЙС
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph44_transport_request_attachment:
  - if: business_role=transport_request
  - value:
    - business_role: transport_request
    - code: 09023
    - subcode: 0
    - doc_number: supporting_doc_transport_request.number
    - doc_date: supporting_doc_transport_request.date
    - doc_name: ЗАЯВКА
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph44_payment_attachment:
  - if: business_role=payment
  - value:
    - business_role: payment
    - code: 09023
    - subcode: 0
    - doc_number: documents_package.payment_doc.number
    - doc_date: documents_package.payment_doc.date
    - doc_name: ЗАЯВЛЕНИЕ НА ПЕРЕВОД
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph44_transport_invoice_attachment:
  - if: business_role=transport_invoice
  - value:
    - business_role: transport_invoice
    - code: 09023
    - subcode: 0
    - doc_number: supporting_doc_transport_invoice_usd.number
    - doc_date: supporting_doc_transport_invoice_usd.date
    - doc_name: СЧЕТ ЗА ПЕРЕВОЗКУ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph44_cmr_attachment:
  - if: business_role=cmr
  - value:
    - business_role: cmr
    - code: 09023
    - subcode: 0
    - doc_number: documents_package.transport_doc.number
    - doc_date: documents_package.transport_doc.date
    - doc_name: СМР
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph44_svh_report_attachment:
  - if: business_role=svh_doc
  - value:
    - business_role: svh_doc
    - code: 09023
    - subcode: 0
    - doc_number: pending
    - doc_date: 14.07.2025
    - doc_name: ОТЧЕТ СВХ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: pending
  - source: reference old DT / svh_report
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low

- graph8_same_as_graph14_rule:
  - value: yes
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

- graph9_same_as_graph14_rule:
  - value: yes
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

- graph42_value_in_dts_rule:
  - value: pending
  - status: pending
  - source: operator / reference
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

- graph31_absent_trademark_representation_rule:
  - value: ОТСУТСТВУЕТ
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

- graph31_absent_brand_representation_rule:
  - value: pending
  - status: pending
  - source: operator / reference
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

- payment_pattern_rule:
  - value: 1010 / 2010 / 5010
  - status: confirmed_case_pattern
  - source: reference old DT
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - note: pattern only, not applied calculation

---

# reference_observed

## Reference[1]

- field_name: old_dt_graph30_type
- observed_in: old_dt_pdf
- value: 11
- status: sample_only
- source_class: sample
- transfer_relevance: reference_only
- relevance_for_next_step: useful_for_mapping
- comment: candidate process-rule for graph 30 type

## Reference[2]

- field_name: old_dt_graph25_graph26
- observed_in: old_dt_pdf
- value: 31 / 31
- status: sample_only
- source_class: sample
- transfer_relevance: reference_only
- relevance_for_next_step: useful_for_mapping
- comment: candidate transport mode rules

## Reference[3]

- field_name: old_dt_svh_number
- observed_in: old_dt_pdf
- value: 0000487
- status: sample_only
- source_class: sample
- transfer_relevance: reference_only
- relevance_for_next_step: useful_for_mapping
- comment: conflicts with current svh_report visual reading 0000478

---

# system_only

## SystemField[1]

- field_name: ED timestamps / LNP / release metadata
- observed_in: old_dt_pdf / transit_doc
- value: various
- source_class: system
- transfer_relevance: do_not_transfer
- comment: служебные реквизиты обработки и выпуска

---

# Conflicts

## Conflict[1]

- field: contract_reference_in_invoice_and_packing_list
- status: unresolved
- source_1: contract_pdf / payment_doc
- value_1: 25AZC003
- source_2: invoice_pdf / packing_list_pdf / transit_doc_invoice_ref
- value_2: 25AZC003B
- impact: блокирует безусловное заполнение реквизита договора в связанных документах и graph 44 linkage logic
- action_required: уточнить у оператора, считать ли 25AZC003 базовым контрактом, а 25AZC003B номером партии/инвойса

## Conflict[2]

- field: svh_report_number_for_graph30_and_graph44
- status: unresolved
- source_1: svh_report_do_pdf
- value_1: 0000478 от 14.07.2025
- source_2: old_dt_pdf
- value_2: 0000487 от 14.07.2025
- impact: блокирует безусловное финальное заполнение номера/кода документа СВХ в graph 30 / graph 44
- action_required: подтвердить по первичному СВХ-документу/оператору правильный номер документа, используемый для ДТ

---

# Missing critical data

## MissingCriticalData[1]

- field: insurance_actual_status
- status: unresolved
- expected_source: operator / страховой документ / явное отсутствие страхования
- source_class_expected: document / operator
- reason: в заявке и proforma есть only condition-level mentions, но нет подтверждения фактического страхования или 
  not_applicable
- impact: блокирует valuation
- action_required: уточнить у оператора, было ли страхование; если нет — зафиксировать not_applicable

## MissingCriticalData[2]

- field: declarant_graph14_master_data
- status: unresolved
- expected_source: alta master data / operator
- source_class_expected: alta_master_data / operator
- reason: поставочная первичка не дает надежного полного профиля декларанта для новой ДТ
- impact: блокирует ui/xml projection
- action_required: получить карточку/реквизиты декларанта

## MissingCriticalData[3]

- field: financial_responsible_graph9_master_data
- status: unresolved
- expected_source: alta master data / operator
- source_class_expected: alta_master_data / operator
- reason: нет подтвержденного master data профиля для графы 9
- impact: блокирует ui/xml projection
- action_required: получить карточку/режим same_as_graph14 или отдельные реквизиты

## MissingCriticalData[4]

- field: graph30_mapping_rule_set
- status: unresolved
- expected_source: operator / approved mapping rule / validated reference analysis
- source_class_expected: mapping_rule
- reason: не закрыты финальные правила по type/document_kind/final_number_date
- impact: блокирует финальный xml_import по графе 30
- action_required: закрепить process-rule по графе 30

## MissingCriticalData[5]

- field: applied_valuation_and_payments
- status: unresolved
- expected_source: calculation
- source_class_expected: calculated
- reason: есть inputs, но нет расчета таможенной стоимости, статистической стоимости и графы 47
- impact: блокирует xml_import
- action_required: выполнить valuation и расчет платежей

---

# Ready-to-use summary

- facts_confirmed_enough_for_review: yes
- facts_confirmed_enough_for_ui_projection: partial
- facts_confirmed_enough_for_xml_projection: no
- facts_confirmed_enough_for_mapping_rule_extraction: partial
- blocker_list:
  - не закрыт фактический статус страхования
  - нет master data для граф 9/14/54
  - не закреплены финальные mapping rules графы 30
  - не выполнен valuation и расчет графы 47
- high_risk_fields:
  - contract reference 25AZC003 vs 25AZC003B
  - номер СВХ-документа 0000478 vs 0000487
  - страхование
  - unit_code
- data_source_gaps:
  - shipment_docs_gap: явного страхового документа нет; нет отдельного надежного документа, окончательно снимающего 
    конфликт номера СВХ для graph 30/44
  - alta_master_data_gap: графы 9/14/54 и часть contact/master реквизитов
  - calculation_gap: курс, база курса, таможенная стоимость, статистическая стоимость, графа 47
  - mapping_rule_gap: графа 30 финальный rule-set, часть process rules граф 18/21/42