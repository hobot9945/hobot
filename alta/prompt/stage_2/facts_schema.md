# Схема facts.md

## Назначение

`facts.md` — это **слой фактов и зависимостей**, на котором строятся:

- `review.md`
- `ui_input.md`
- `xml_import.md`
  Главный принцип:
  **в `facts.md` храним то, что нужно для построения новой ДТ с явным указанием происхождения данных и явным разделением:**
- shipment_facts,
- alta_master_data_requirements,
- calculated_requirements,
- mapping_rules,
- reference-only наблюдений,
- system-only полей.

---

## Ключевое разделение слоев

Внутри `facts.md` должны быть **четко разделены** 6 классов данных. Эти классы взаимоисключающие.
Одно и то же значение нельзя одновременно относить к нескольким классам.

1. **shipment_facts** 
   Факты, извлеченные из первичных документов поставки.

2. **alta_master_data_requirements** 
   Данные, которые нужны для новой ДТ, обычно берутся из:
   - карточек контрагентов,
   - карточек декларанта,
   - карточек представителя,
   - настроек / справочников Альты,
   - ранее заведенных реквизитов компании.

3. **calculated_requirements**
   Данные, которые возникают в результате:
  - расчета;
  - вывода производного значения;
  - применения формализованного правила к данным текущего кейса.

   Если значение может однозначно следовать из ясного предиката, то его 
   нужно хранить в `mapping_rules`, а не в `calculated_requirements`.

4. **fact_composition**
   Составные значения, которые собираются из shipment_facts для целей UI, XML, печатной формы или сверки.

5. **mapping_rules** 
   Условные представления и process-константы, которые:
   - нужны для корректного построения `ui_input.md` и `xml_import.md`;
   - имеют ясно сформулированное и однозначное условие применения;
   - задают результат, который должен быть помещен в поле Альты или тег XML;
   
6. **reference_observed / system_only** 
   Наблюдения из эталона и системные поля, которые нельзя смешивать с фактами новой поставки.

---

## Практический приоритет

Для всех существенных полей желательно сохранять:

- `status`
- `source`
- `source_class`
- `transfer_relevance`
- `blocking_for_next_step`
  Поле `blocking_for_next_step` критично, потому что именно оно позволяет переходить от `facts.md`
  к `review.md`, `ui_input.md` и `xml_import.md` без потери управляемости по этапам.

---

## Поля любого типа

- `pending` — пока не подтверждено.

---

## Статусы значений

- `confirmed_document` — подтверждено документами поставки.
- `confirmed_operator` — подтверждено оператором.
- `confirmed_mapping_rule` — подтверждено как правило проекции / process-rule.
- `confirmed_case_pattern` — подтверждено сравнением generated vs reference на эталонном кейсе.
- `derived` — выведено из подтвержденных данных.
- `sample_only` — наблюдается только в эталоне / reference.
- `not_applicable` — поле осознанно неприменимо.
- `not_found` — искали, но не нашли.

---

## source_class

Использовать только:

- `document`
- `operator`
- `derived`
- `sample`
- `system`
- `alta_master_data`
- `calculated`
- `composed`
- `mapping_rule`

---

## transfer_relevance

Для каждого существенного поля желательно указывать:

- `needed_for_review`
- `needed_for_ui`
- `needed_for_xml`
- `needed_for_calculation`
- `needed_for_mapping`
- `reference_only`
- `do_not_transfer`
- `optional`
- `conditional_for_ui`
- `conditional_for_xml`

## note

### Сокращения для поля:

- для mapping_rule:
  - `reference_not_clear`: в эталоне значение присутствует, но для формулировки правила требуется консультация специалиста

---

# Метаданные

- case_name: <название кейса>
- source_folder: <путь к папке поставки>
- dt_scope: <например: 1 ДТ / 1 товар>
- status: <draft / in_progress / blocked / ready_for_projection>
- ready_for_next_step: <yes / partial / no>
- unresolved_conflicts_count: <число>
- unresolved_missing_critical_data_count: <число>
- note: <если нужно короткое пояснение по стадии>

---

# Пакет документов. Префикс: `documents_package`

- contract:
  
  - file: <имя файла>
  - status: <confirmed_document / not_found>
  - number: <номер>
  - number_status: <status>
  - date: <дата>
  - date_status: <status>
  - role: core
  - source_class: document
  - note: <если нужно>

- invoice:
  
  - file: <имя файла>
  - status: <confirmed_document / not_found>
  - number: <номер>
  - number_status: <status>
  - date: <дата>
  - date_status: <status>
  - role: core
  - source_class: document

- packing_list:
  
  - file: <имя файла>
  - status: <confirmed_document / not_found>
  - number: <номер / БН>
  - number_status: <status>
  - date: <дата>
  - date_status: <status>
  - role: core
  - source_class: document

- transport_doc:
  
  - file: <имя файла>
  - status: <confirmed_document / not_found>
  - kind: <CMR / AWB / rail waybill / etc>
  - kind_status: <status>
  - number: <номер>
  - number_status: <status>
  - date: <дата>
  - date_status: <status>
  - role: core
  - source_class: document

- payment_doc:
  
  - file: <имя файла>
  - status: <confirmed_document / not_found>
  - number: <номер>
  - number_status: <status>
  - date: <дата>
  - date_status: <status>
  - role: core
  - source_class: document

- tech_description:
  - file: <имя файла>
  - status: <confirmed_document / not_found>
  - number: <номер / БН>
  - number_status: <status>
  - date: <дата>
  - date_status: <status>
  - role: core
  - source_class: document
    
## Документы поддержки

- supporting_doc_<имя документа>:
  - file: <имя файла>
  - doc_kind: <transport_request / transport_invoice / svh_doc / supplementary_agreement / authority_doc / other>
  - number: <значение>
  - number_status: <status>
  - date: <значение>
  - date_status: <status>
  - role: supporting
  - source_class: document
  - note: <зачем документ нужен>
    
## Документы эталонов

- reference_doc_<имя документа>:
  - file: <имя файла>
  - role: reference_only
  - status: sample_only
  - source_class: sample
  - note: использовать только для структуры / полноты / mapping / process-rule analysis
    
---

# Поставка. Префикс: `shipment_facts`

- declaration_basis:
  - value: <import / export / transit / etc if document-confirmed>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / derived>
  - transfer_relevance: <needed_for_review / needed_for_ui / needed_for_xml>
  - blocking_for_next_step: <yes / no>
  - note: использовать только если реально подтверждается комплектом документов или оператором
- incoterms_code:
  - value: <EXW / FCA / ...>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- incoterms_place:
  - value: <место>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- invoice_currency_alpha:
  - value: <CNY / USD / EUR ...>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- invoice_currency_numeric:
  - value: <156 / 840 / ...>
  - status: <status>
  - source: <источник>
  - source_class: <derived / document>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- invoice_amount_total:
  - value: <сумма>
  - currency: <валюта>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- package_count_total:
  - value: <число мест>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- gross_weight_total:
  - value: <значение>
  - unit: kg
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- net_weight_total:
  - value: <значение>
  - unit: kg
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- trade_country_name:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- trade_country_code:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- dispatch_country_name:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- dispatch_country_code:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- origin_country_name:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- origin_country_code:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- destination_country_name:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- destination_country_code:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
- commercial_shipment_point:
  - value: <например Ningbo>
  - status: <status>
  - source: <invoice / PL / contract>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>
  - note: коммерческое место отгрузки, не всегда совпадает с фактическим departure_place

## Стороны. Префикс: `shipment_facts.parties`

### Грузоотправитель. Префикс: `shipment_facts.parties.sender`

- name:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- country_code:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- country_name:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- postcode:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- region_or_area:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- city_or_locality:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- street_address:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- house:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- office:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: optional
  - blocking_for_next_step: <yes / no>

- phone:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator>
  - transfer_relevance: optional
  - blocking_for_next_step: <yes / no>

- email:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator>
  - transfer_relevance: optional
  - blocking_for_next_step: <yes / no>

### Грузополучатель. Префикс: `shipment_facts.parties.consignee`

- name:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- inn_kpp:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- ogrn:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- country_code:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- country_name:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- postcode:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- region_or_area:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- city_or_locality:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- street:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- house:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- office:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- phone:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- email:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / alta_master_data>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

## Логистика и транспорт. Префикс: `shipment_facts.logistics_and_transport`

- transport_doc_number:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- transport_doc_date:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- departure_place:
  - value: <фактическое место отправки>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- delivery_place:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- carrier_name:
  - value: <значение>
  - status: <status>
  - source: <CMR / transport doc / transport request>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- carrier_tax_id:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- border_transport_description:
  - value: <если читается прямо из документов>
  - status: <status>
  - source: <источник>
  - source_class: <document / sample>
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

### Неподтверждённые наблюдения.

Потенциально полезные данные, обнаруженные в первичке или эталонах. Требуют внимания оператора.

- <candidate_id>:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / sample>
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>
  - note: наблюдаемое значение; не считать автоматически готовым расчетным полем

---

## Склад / размещение товаров (только подтвержденные первичкой).

- warehouse_document_exists:
  - value: <yes / no / pending>
  - status: <status>
  - source: <SVH docs / CMR / operator>
  - source_class: <document / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
  
- location_type:
  - value: <значение>
  - status: <status>
  - source: <SVH docs / operator / reference>
  - source_class: <document / operator / sample>
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: <yes / no>
  - note: тип местонахождения / тип заполнения графы 30; это еще не код поля Альты

- document_kind_code:
  - value: <значение>
  - status: <status>
  - source: <SVH docs / operator / reference>
  - source_class: <document / operator / sample>
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: <yes / no>
  - note: код вида документа графы 30, если реально подтвержден как rule, а не просто подсмотрен

- warehouse_document_number:
  - value: <значение>
  - status: <status>
  - source: <SVH docs>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
  - note: это номер документа СВХ

- warehouse_document_date:
  - value: <значение>
  - status: <status>
  - source: <SVH docs>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
  - note: это дата документа СВХ

- warehouse_registry_number:
  - value: <значение>
  - status: <status>
  - source: <SVH docs>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
  - note: реестровый складской номер / номер из реквизитов владельца СВХ; может использоваться в итоговом `G_30_1`

- warehouse_registry_date:
  - value: <значение>
  - status: <status>
  - source: <SVH docs>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
  - note: дата, связанная с реестровым складским номером / реквизитами владельца СВХ; может использоваться в 
    итоговом `G_30_DATE` 

- warehouse_address_raw:
  - value: <адрес / местонахождение товара>
  - status: <status>
  - source: <CMR / SVH docs>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- warehouse_country_code:
  - value: <значение>
  - status: <status>
  - source: <SVH docs / derived>
  - source_class: <document / derived>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- warehouse_region_or_area:
  - value: <значение>
  - status: <status>
  - source: <SVH docs / derived>
  - source_class: <document / derived>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- warehouse_city_or_locality:
  - value: <значение>
  - status: <status>
  - source: <SVH docs / derived>
  - source_class: <document / derived>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- warehouse_street_address:
  - value: <значение>
  - status: <status>
  - source: <SVH docs / derived>
  - source_class: <document / derived>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- warehouse_customs_code_candidate:
  - value: <значение>
  - status: <status>
  - source: <SVH docs / sample>
  - source_class: <document / sample>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

---

## Товары. Префикс: `shipment_facts.Goods[n]`

- item_no:
  - value: <значение>
  - status: <status>
  - source: <derived / document>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- product_name_ru:
  - value: <значение>
  - status: <status>
  - source: <tech description / invoice / operator>
  - source_class: <document / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- product_name_en:
  - value: <значение>
  - status: <status>
  - source: <invoice / PL / transport docs>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- model:
  - value: <значение>
  - status: <status>
  - source: <tech description / PL / invoice>
  - source_class: <document / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- article:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator>
  - transfer_relevance: optional
  - blocking_for_next_step: <yes / no>

- hs_code:
  - value: <значение>
  - status: <status>
  - source: <tech description / invoice / operator / classification>
  - source_class: <document / operator / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- manufacturer:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- trademark:
  - value: <значение / отсутствует>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / derived>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- brand:
  - value: <значение / отсутствует>
  - status: <status>
  - source: <источник>
  - source_class: <document / operator / derived>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- origin_country_code:
  - value: <значение>
  - status: <status>
  - source: <источник>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- quantity:
  - value: <значение>
  - unit: <как в документе>
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- unit_code:
  - value: <код ед. изм.>
  - status: <status>
  - source: <derived / operator / document>
  - source_class: <document / operator / derived>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- unit_name:
  - value: <наименование ед. изм.>
  - status: <status>
  - source: <derived / document>
  - source_class: <document / derived>
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: <yes / no>

- package_places_count:
  - value: <значение>
  - status: <status>
  - source: <PL / CMR>
  - source_class: document
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- package_places_marking:
  - value: <значение>
  - status: <status>
  - source: <PL / marking docs / sample>
  - source_class: <document / sample>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>
  - note: нельзя подтверждать по sample-only без документной опоры

- gross_weight:
  - value: <значение>
  - unit: kg
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- net_weight:
  - value: <значение>
  - unit: kg
  - status: <status>
  - source: <источник>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- invoice_value:
  - value: <значение>
  - currency: <валюта>
  - status: <status>
  - source: <invoice / payment>
  - source_class: document
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- description_31_source_facts:
  - value:
  - <факт 1>
  - <факт 2>
  - <факт 3>
  - status: <status>
  - source: <goods facts>
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: <yes / no>
  - note: это не готовая строка графы 31, а набор фактов для ее сборки

---

# Требования к master data Альты. Префикс: `alta_master_data_requirements`

## Профиль грузополучателя. Префикс: `alta_master_data_requirements.consignee_profile`

- registration_id:
  - value: <например ОГРН / иной регистрационный идентификатор>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- phone:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- email:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- same_as_graph14_mode:
  - value: <yes / no>
  - status: <status>
  - source: <operator / alta_master_data / sample>
  - source_class: <alta_master_data / operator / sample>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
  - note: это не свойство компании, а режим представления

## Профиль лица графы 9. Префикс: `alta_master_data_requirements.financial_responsible_profile`

- name:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- inn_kpp:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- ogrn_or_registration_id:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- country_code:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- country_name:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- postcode:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- region_or_area:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- city_or_locality:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- street:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- house:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- office:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- phone:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- email:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / document>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- same_as_graph14_mode:
  - value: <yes / no>
  - status: <status>
  - source: <operator / alta_master_data / sample>
  - source_class: <alta_master_data / operator / sample>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>
  - note: это режим представления, а не бизнес-факт

## Профиль декларанта. Префикс: `alta_master_data_requirements.declarant_profile`

- name:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- inn_kpp:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- ogrn_or_registration_id:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- country_code:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- country_name:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- address_raw:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- postcode:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- region_or_area:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- city_or_locality:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- street:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- house:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- office:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- phone:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- email:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

## Профиль представителя. Префикс: `alta_master_data_requirements.representative_profile`

- last_name:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- first_name:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- middle_name:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- phone:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- email:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- role_or_status:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: <yes / no>

- customs_representative_code:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data>
  - source_class: <alta_master_data / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- passport_type:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- passport_series:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- passport_number:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- passport_date:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- passport_issuer:
  - value: <значение>
  - status: <status>
  - source: <operator / alta_master_data / authority docs>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- authority_doc_name:
  - value: <значение>
  - status: <status>
  - source: <operator / authority docs / alta_master_data>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- authority_doc_number:
  - value: <значение>
  - status: <status>
  - source: <operator / authority docs / alta_master_data>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- authority_doc_date_from:
  - value: <значение>
  - status: <status>
  - source: <operator / authority docs / alta_master_data>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- authority_doc_date_to:
  - value: <значение>
  - status: <status>
  - source: <operator / authority docs / alta_master_data>
  - source_class: <alta_master_data / operator / document>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

---

# Расчётные данные

## Назначение слоя

Этот раздел нужен для хранения **правил расчета значений**, которые нужны для корректного заполнения полей Альты и 
генерации XML;

## Общий шаблон расчетных данных.

Если правило нужно для следующего этапа и помечено `needed_for_mapping`, но не может быть надежно подтверждено или
применено из-за отсутствия фактов, недостатка данных или невозможности надежного вывода, оно материализуется в `facts.md`
со статусом `pending`.
Такое правило должно быть отражено в review как незакрытая mapping-зависимость и оценено как блокер или частичный блокер
следующего этапа.

- rule_name: <имя правила>
  - rule: <формализованное/произвольное описание правила расчета значения> (не материализуется)
  - value: <значение>
  - status: <confirmed_mapping_rule / confirmed_case_pattern / pending>
  - source: <reference xml / screenshot / operator / diff analysis>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - derive_from: (не материализуется)
    - <full_fact_path>[n]
  - confidence: <high / medium / low>
  - note: <(опционально) в `facts_schema.md` пояснение к правилу, в `facts.md` пояснение к факту>

## Префикс: `calculated_requirements`

- declaration_direction:
  - value: <значение>
  - status: <status>
  - source: <operator / process rule / sample>
  - source_class: <operator / calculated / sample>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes
  - note: если это не фактический shipment data, а устойчивое process rule, лучше выносить в `mapping_rules`

- declaration_procedure_code:
  - value: <значение>
  - status: <status>
  - source: <operator / process rule / sample>
  - source_class: <operator / calculated / sample>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes
  - note: если это подтвержденный process-код, лучше хранить rule отдельно в `mapping_rules`

- electronic_declaration_flag:
  - value: <значение>
  - status: <status>
  - source: <operator / system / process rule>
  - source_class: <operator / system / calculated>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- forms_main:
  - value: <значение>
  - status: <status>
  - source: <calculated / operator>
  - source_class: <calculated / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- forms_additional:
  - value: <значение>
  - status: <status>
  - source: <calculated / operator>
  - source_class: <calculated / operator>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- border_transport_mode_code:
  - value: <значение>
  - status: <status>
  - source: <operator / process rule / sample / logistics candidates>
  - source_class: <operator / calculated / sample / document>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- departure_transport_mode_code:
  - value: <значение>
  - status: <status>
  - source: <operator / process rule / sample / logistics candidates>
  - source_class: <operator / calculated / sample / document>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- transport_identification_for_declaration:
  - value: <значение>
  - status: <status>
  - source: <operator / document / sample>
  - source_class: <operator / document / sample>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- transport_registration_country_code:
  - value: <значение>
  - status: <status>
  - source: <operator / document / sample>
  - source_class: <operator / document / sample>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>
  - note: если это код process-режима, а не буквенный код страны, лучше отражать это явно

- container_flag:
  - value: <0 / 1 / etc>
  - status: <status>
  - source: <operator / document / sample>
  - source_class: <operator / document / sample>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- border_customs_code:
  - value: <значение>
  - status: <status>
  - source: <operator / SVH docs / sample / process rule>
  - source_class: <operator / document / sample / calculated>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- border_customs_name:
  - value: <значение>
  - status: <status>
  - source: <operator / sample / directory>
  - source_class: <operator / sample / system>
  - transfer_relevance: conditional_for_ui
  - blocking_for_next_step: <yes / no>

## Входы и результаты расчёта стоимости. Префикс: `calculated_requirements`

- transport_cost_to_border:
  - value: <значение>
  - currency: <валюта>
  - status: <status>
  - source: <transport invoice / operator>
  - source_class: <document / operator>
  - transfer_relevance: needed_for_calculation
  - blocking_for_next_step: yes

- post_border_transport_cost:
  - value: <значение>
  - currency: <валюта>
  - status: <status>
  - source: <transport invoice>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- local_delivery_cost:
  - value: <значение>
  - currency: <валюта>
  - status: <status>
  - source: <transport invoice>
  - source_class: document
  - transfer_relevance: needed_for_review
  - blocking_for_next_step: no

- insurance:
  - value: <значение / not_applicable>
  - status: <status>
  - source: <operator / transport docs / invoice>
  - source_class: <document / operator / calculated>
  - transfer_relevance: needed_for_calculation
  - blocking_for_next_step: yes
  - note: нужно явно различать `pending` и `not_applicable`

- exchange_rate:
  - value: <значение>
  - status: <status>
  - source: <system / operator / calculation rule>
  - source_class: <system / operator / calculated>
  - transfer_relevance: needed_for_calculation
  - blocking_for_next_step: yes

- exchange_rate_date_basis:
  - value: <дата / правило>
  - status: <status>
  - source: <system / operator / calculation rule>
  - source_class: <system / operator / calculated>
  - transfer_relevance: needed_for_calculation
  - blocking_for_next_step: yes

- customs_value_total:
  - value: <значение>
  - currency: RUB
  - status: <status>
  - source: <calculation>
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- statistical_value_total:
  - value: <значение>
  - currency: <USD / other as required>
  - status: <status>
  - source: <calculation>
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

## Расчётные данные по товару. Префикс: `calculated_requirements.GoodsCalculated[1]`

- item_no:
  - value: <номер товара>
  - status: <status>
  - source: <goods linkage>
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- preference_code:
  - value: <значение>
  - status: <status>
  - source: <operator / calculation / reference>
  - source_class: <operator / calculated / sample>
  - transfer_relevance: conditional_for_xml
  - blocking_for_next_step: <yes / no>

- procedure_code:
  - value: <значение>
  - status: <status>
  - source: <operator / calculation / reference>
  - source_class: <operator / calculated / sample>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- customs_value:
  - value: <значение>
  - currency: RUB
  - status: <status>
  - source: <calculation>
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

- statistical_value:
  - value: <значение>
  - currency: <USD / other as required>
  - status: <status>
  - source: <calculation>
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: <yes / no>

## Платёж. Префикс: `calculated_requirements.Payments[1]`

- payment_code:
  - value: <значение>
  - status: <status>
  - source: <calculation / tariff rule>
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- payment_base:
  - value: <значение>
  - status: <status>
  - source: <calculation>
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- payment_rate:
  - value: <значение>
  - status: <status>
  - source: <tariff rule / calculation>
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- payment_amount:
  - value: <значение>
  - status: <status>
  - source: <calculation>
  - source_class: calculated
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

- payment_sp:
  - value: <значение>
  - status: <status>
  - source: <operator / process rule / calculation>
  - source_class: <operator / calculated>
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: yes

---

# Составные данные

## Назначение слоя

В выходных документах `ui_input.md`, `xml_import.md` присутствуют поля, скомпонованные из нескольких `shipment_facts`.
Этот раздел хранит объекты, которые обеспечивают генерацию данных для таких полей.

## Общий шаблон. Префикс: `fact_composition`

- composition_name:
  - composition_rule: <формализованное/описательное правило композиции> (не материализуется)
  - value: <значение>
  - status: <status>
  - source: composed
  - source_class: derived
  - transfer_relevance: <needed_for_ui / needed_for_xml / optional>
  - blocking_for_next_step: <yes / no>
  - compose_from: (не материализуется)
    - <full_fact_path>[n]
  - confidence: <high / medium / low>
  - note: <(опционально) в facts_schema.md пояснение к правилу, в facts.md пояснение к факту>

## Составные данные для графы 30. Префикс: `fact_composition`

- warehouse_printed_address:
  - composition_rule: собрать печатную строку графы 30 из кода таможни, адресных компонентов местонахождения товара и
    выбранных реквизитов документа графы 30; пустые компоненты пропускать без лишних разделителей
  - value: <собранная печатная строка>
  - status: <status>
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - compose_from:
    - shipment_facts.warehouse_goods_location.warehouse_customs_code_candidate
    - shipment_facts.warehouse_goods_location.warehouse_region_or_area
    - shipment_facts.warehouse_goods_location.warehouse_city_or_locality
    - shipment_facts.warehouse_goods_location.warehouse_street_address
    - shipment_facts.warehouse_goods_location.warehouse_document_number
    - shipment_facts.warehouse_goods_location.warehouse_document_date
  - confidence: <high / medium / low>
  - note: составное представление для сверки графы 30

## Составные данные для графы 31. Префикс: `fact_composition`

- description_31_main_draft:
  - composition_rule: собрать черновик основной строки графы 31 из подтвержденных описательных facts товара по устойчивому
    порядку описания; не добавлять сведения, не подтвержденные source facts
  - value: <черновик основной строки графы 31>
  - status: <status>
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: <yes / no>
  - compose_from:
    - shipment_facts.Goods[n].description_31_source_facts
    - shipment_facts.Goods[n].product_name_ru
    - shipment_facts.Goods[n].model
    - shipment_facts.Goods[n].manufacturer
    - shipment_facts.Goods[n].quantity
    - shipment_facts.Goods[n].unit_name
  - confidence: <high / medium / low>
  - note: составной черновик для сверки и доработки

- description_31_lines:
  - composition_rule: разложить описание графы 31 на строки presentation-формата по подтвержденным source facts; порядок
    строк должен быть устойчивым внутри кейса
  - value:
    - <строка 1>
    - <строка 2>
    - <строка 3>
  - status: <status>
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: <yes / no>
  - compose_from:
    - shipment_facts.Goods[n].description_31_source_facts
    - fact_composition.description_31_main_draft
  - confidence: <high / medium / low>
  - note: промежуточное presentation-представление для сверки графы 31

- group_description:
  - composition_rule: собрать краткое табличное описание товарной группы из наименования товара и модели
  - value: <значение>
  - status: <status>
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: <yes / no>
  - compose_from:
    - shipment_facts.Goods[n].product_name_ru
    - shipment_facts.Goods[n].model
  - confidence: <high / medium / low>
  - note: составное табличное описание

## Составные данные для графы 44. Префикс: `fact_composition`

Используется в `graph44_mapping_rule` как функция для сборки поля `graph44_doc_text` без материализации. По этой причине
опущены некоторые поля общего шаблона и изменена логика `composition_rule`.

- graph44_doc_text:
  - composition_rule: собрать presentation-строку текущей записи graph44 из полей `code`, `subcode`, `doc_number`,
    `doc_date`, `doc_name`, `valid_from`, `valid_to`; отсутствующие компоненты пропускать
  - value: <собранное текстовое представление>
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_xml
  - blocking_for_next_step: no

## Составные данные для графы 54. Префикс: `fact_composition`

- representative_printed_block:
  - composition_rule: собрать печатный блок представителя из ФИО, статуса, контактных данных, документа личности и документа
    полномочий; пустые части пропускать
  - value: <значение>
  - status: <status>
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - compose_from:
    - alta_master_data_requirements.representative_profile.last_name
    - alta_master_data_requirements.representative_profile.first_name
    - alta_master_data_requirements.representative_profile.middle_name
    - alta_master_data_requirements.representative_profile.role_or_status
    - alta_master_data_requirements.representative_profile.phone
    - alta_master_data_requirements.representative_profile.email
    - alta_master_data_requirements.representative_profile.passport_type
    - alta_master_data_requirements.representative_profile.passport_series
    - alta_master_data_requirements.representative_profile.passport_number
    - alta_master_data_requirements.representative_profile.passport_date
    - alta_master_data_requirements.representative_profile.passport_issuer
    - alta_master_data_requirements.representative_profile.authority_doc_name
    - alta_master_data_requirements.representative_profile.authority_doc_number
    - alta_master_data_requirements.representative_profile.authority_doc_date_from
    - alta_master_data_requirements.representative_profile.authority_doc_date_to
  - confidence: <high / medium / low>
  - note: печатное представление для сверки

## Составные данные по платежам. Префикс: `fact_composition`

- payment_summary_line:
  - composition_rule: собрать итоговую presentation-строку по платежам из рассчитанных полей платежа; отсутствующие
    компоненты пропускать
  - value: <значение>
  - status: <status>
  - source: composed
  - source_class: derived
  - transfer_relevance: needed_for_ui
  - blocking_for_next_step: no
  - compose_from:
    - calculated_requirements.Payments[n].payment_code
    - calculated_requirements.Payments[n].payment_base
    - calculated_requirements.Payments[n].payment_rate
    - calculated_requirements.Payments[n].payment_amount
    - calculated_requirements.Payments[n].payment_sp
  - confidence: <high / medium / low>
  - note: итоговая строка представления платежей

---

# Условные данные

## Назначение слоя

Этот раздел нужен для хранения **правил выбора значений**, которые нужны для корректного заполнения полей Альты и 
генерации XML.

Этот слой хранит:
- правила выбора значений;
- process-константы (условие `if: всегда`).

## Общий шаблон условных данных.

Если правило нужно для следующего этапа и помечено `needed_for_mapping`, но не может быть надежно подтверждено или
применено из-за отсутствия фактов, недостатка данных или невозможности надежного вывода, оно материализуется в `facts.md`
со статусом `pending`.
Такое правило должно быть отражено в review как незакрытая mapping-зависимость и оценено как блокер или частичный блокер
следующего этапа.

- rule_name: <имя правила>
  - if: <формализованное/описательное условие применения> (не материализуется)
  - value: <значение>
  - status: <confirmed_mapping_rule / confirmed_case_pattern / pending>
  - source: <reference xml / screenshot / operator / diff analysis>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: <high / medium / low>
  - note: <(опционально) в `facts_schema.md` пояснение к правилу, в `facts.md` пояснение к факту>

## Условные данные для заголовка декларации. Префикс: `mapping_rules`

- declaration_direction_code_rule:
  - if: declaration_basis=import
  - value: ИМ
  - status: confirmed_case_pattern
  - source: reference xml / screenshot / operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- declaration_procedure_code_rule:
  - if: declaration_basis=import; декларация оформляется в стандартной процедуре выпуска для внутреннего потребления
  - value: 40
  - status: confirmed_case_pattern
  - source: reference xml / screenshot / operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium

- electronic_declaration_flag_rule:
  - if: декларация подается в электронном виде
  - value: ЭД
  - status: confirmed_case_pattern
  - source: reference xml / screenshot / operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: признак способа подачи декларации

- forms_main_rule:
  - if: всегда
  - value: 1
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear

- forms_additional_rule:
  - if: всегда
  - value: 1
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear

## Условные данные для транспорта и таможни на границе. Префикс: `mapping_rules`

- graph18_kind_rule:
  - if: pending
  - value: 2
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear; это не номер ТС, а режим / код представления поля

- graph18_registration_country_rule:
  - if: pending
  - value: 00
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear; не смешивать с буквенным кодом страны регистрации

- graph19_container_flag_rule:
  - if: pending
  - value: 0
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear

- graph21_kind_rule:
  - if: pending
  - value: 1
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear

- graph25_transport_mode_rule:
  - if: pending
  - value: pending
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear

- graph26_transport_mode_rule:
  - if: pending
  - value: pending
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear

- graph29_customs_code_rule:
  - if: pending
  - value: pending
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear

- graph29_customs_name_rule:
  - if: pending
  - value: pending
  - status: pending
  - source: operator
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: reference_not_clear

## Условные данные для графы 30. Префикс: `mapping_rules`

- graph30_type_code_rule:
  - if: pending
  - value: pending
  - status: pending
  - source: <reference xml / screenshot / operator>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - note: код графы 30

- graph30_document_kind_code_rule:
  - if: pending
  - value: pending
  - status: pending
  - source: <reference xml / screenshot / operator>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - note: код вида документа графы 30

- graph30_customs_code_rule:
  - if: pending
  - value: pending
  - status: pending
  - source: <reference xml / screenshot / operator>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

## Условные данные для графы 44. Префикс: `mapping_rules`

### Общие принципы

1. Правила описывают, как нормализованный документ из `documents_package` проецируется в запись графы 44.
2. Правила не извлекают данные из сырых документов. Они используют только уже подтвержденные реквизиты документа.
3. `fact_composition.graph44_doc_text` используется только как правило композиции, без материализации значения в `facts.md`.

### Специализация общего шаблона условных представлений

- rule_name: <имя правила>
  - if: <условие применения>
  - value:
    - business_role: <role>
    - code: <код graph44>
    - subcode: <подкод graph44>
    - doc_number: <путь к полю номера документа>
    - doc_date: <путь к полю даты документа>
    - valid_from: <путь к полю даты начала действия, если применимо>
    - valid_to: <путь к полю даты окончания действия, если применимо>
    - doc_name: <имя документа>
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: <confirmed_mapping_rule / confirmed_case_pattern / pending>
  - source: <reference xml / screenshot / operator / diff analysis>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: <high / medium / low>
  - note: <(опционально) в `facts_schema.md` пояснение к правилу, в `facts.md` пояснение к факту>

### Набор правил проекции для графы 44. Префикс: `mapping_rules.graph44_*`

- rule_name: graph44_cmr_primary
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
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: основная запись графы 44 для CMR

- rule_name: graph44_contract_primary
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
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: основная запись графы 44 для контракта

- rule_name: graph44_registry_extract_primary
  - if: business_role=registry_extract
  - value:
    - business_role: registry_extract
    - code: 04011
    - subcode: 0
    - doc_number: documents_package.supporting_doc_registry_extract.number
    - doc_date: documents_package.supporting_doc_registry_extract.date
    - doc_name: ВЫПИСКА ИЗ ЕГРЮЛ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: запись наблюдается в эталоне, но соответствующий документ ещё не оформлен в текущем documents_package

- rule_name: graph44_invoice_primary
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
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: основная запись графы 44 для коммерческого инвойса

- rule_name: graph44_payment_primary
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
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: основная запись графы 44 для платежного документа

- rule_name: graph44_transport_invoice_primary
  - if: business_role=transport_invoice
  - value:
    - business_role: transport_invoice
    - code: 04031
    - subcode: 0
    - doc_number: documents_package.supporting_doc_invoice.number
    - doc_date: documents_package.supporting_doc_invoice.date
    - doc_name: СЧЕТ ЗА ПЕРЕВОЗКУ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: основная запись графы 44 для счета за перевозку

- rule_name: graph44_transport_contract_primary
  - if: business_role=transport_contract
  - value:
    - business_role: transport_contract
    - code: 04033
    - subcode: 0
    - doc_number: documents_package.supporting_doc_transport_contract.number
    - doc_date: documents_package.supporting_doc_transport_contract.date
    - doc_name: ДОГОВОР ПО ПЕРЕВОЗКЕ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: запись наблюдается в эталоне; в текущем documents_package договор перевозки не оформлен как отдельный нормализованный документ

- rule_name: graph44_packing_list_primary
  - if: business_role=packing_list
  - value:
    - business_role: packing_list
    - code: 04131
    - subcode: 0
    - doc_number: documents_package.packing_list.number
    - doc_date: documents_package.packing_list.date
    - doc_name: УПАКОВОЧНЫЙ ЛИСТ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: основная запись графы 44 для упаковочного листа

- rule_name: graph44_tech_description_primary
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
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: основная запись графы 44 для технического описания

- rule_name: graph44_transit_doc_primary
  - if: business_role=transit_doc
  - value:
    - business_role: transit_doc
    - code: 09013
    - subcode: 0
    - doc_number: documents_package.supporting_doc_transit_doc.number
    - doc_date: documents_package.supporting_doc_transit_doc.date
    - doc_name: ТРАНЗИТНАЯ ДЕКЛАРАЦИЯ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: запись графы 44 для транзитной декларации

- rule_name: graph44_tech_description_attachment
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
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: дополнительная observed-запись по reference; общий принцип применения пока не зафиксирован

- rule_name: graph44_invoice_attachment
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
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: дополнительная observed-запись по reference; общий принцип применения пока не зафиксирован

- rule_name: graph44_transport_request_attachment
  - if: business_role=transport_request
  - value:
    - business_role: transport_request
    - code: 09023
    - subcode: 0
    - doc_number: documents_package.supporting_doc_transport_request.number
    - doc_date: documents_package.supporting_doc_transport_request.date
    - doc_name: ЗАЯВКА
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: дополнительная observed-запись по reference; общий принцип применения пока не зафиксирован

- rule_name: graph44_payment_attachment
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
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: дополнительная observed-запись по reference; общий принцип применения пока не зафиксирован

- rule_name: graph44_transport_invoice_attachment
  - if: business_role=transport_invoice
  - value:
    - business_role: transport_invoice
    - code: 09023
    - subcode: 0
    - doc_number: documents_package.supporting_doc_2.number
    - doc_date: documents_package.supporting_doc_2.date
    - doc_name: СЧЕТ ЗА ПЕРЕВОЗКУ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: дополнительная observed-запись по reference; общий принцип применения пока не зафиксирован

- rule_name: graph44_cmr_attachment
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
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: дополнительная observed-запись по reference; общий принцип применения пока не зафиксирован

- rule_name: graph44_svh_report_attachment
  - if: business_role=svh_doc
  - value:
    - business_role: svh_doc
    - code: 09023
    - subcode: 0
    - doc_number: documents_package.supporting_doc_svh_report.number
    - doc_date: documents_package.supporting_doc_svh_report.date
    - doc_name: ОТЧЕТ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: дополнительная observed-запись по reference; общий принцип применения пока не зафиксирован

- rule_name: graph44_svh_report_primary
  - if: business_role=svh_doc
  - value:
    - business_role: svh_doc
    - code: 09026
    - subcode: 0
    - doc_number: documents_package.supporting_doc_5.number
    - doc_date: documents_package.supporting_doc_5.date
    - doc_name: ОТЧЕТ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: low
  - note: observed-запись по reference; общий принцип применения пока не зафиксирован

- rule_name: graph44_passport_primary
  - if: business_role=representative_identity_doc
  - value:
    - business_role: representative_identity_doc
    - code: 11001
    - subcode: 3
    - doc_number: alta_master_data_requirements.representative_profile.passport_number
    - doc_date: alta_master_data_requirements.representative_profile.passport_date
    - doc_name: ПАСРФ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: запись графы 44 для документа, удостоверяющего личность представителя

- rule_name: graph44_power_of_attorney_primary
  - if: business_role=authority_doc
  - value:
    - business_role: authority_doc
    - code: 11004
    - subcode: 4
    - doc_number: alta_master_data_requirements.representative_profile.authority_doc_number
    - doc_date: alta_master_data_requirements.representative_profile.authority_doc_date_from
    - valid_from: alta_master_data_requirements.representative_profile.authority_doc_date_from
    - valid_to: alta_master_data_requirements.representative_profile.authority_doc_date_to
    - doc_name: ДОВЕРЕННОСТЬ
    - graph44_doc_text: fact_composition.graph44_doc_text.value
  - status: confirmed_case_pattern
  - source: reference xml / screenshot
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - confidence: medium
  - note: запись графы 44 для доверенности представителя

## Правила представления. Префикс: `mapping_rules`

- graph8_same_as_graph14_rule:
  - value: <yes / no>
  - status: <status>
  - source: <operator / reference ui>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - note: это режим представления, не свойство компании

- graph9_same_as_graph14_rule:
  - value: <yes / no>
  - status: <status>
  - source: <operator / reference ui>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

- graph42_value_in_dts_rule:
  - value: <например В ДТС>
  - status: <status>
  - source: <reference ui / operator / process>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

- graph31_absent_trademark_representation_rule:
  - value: <например ОТСУТСТВУЕТ>
  - status: <status>
  - source: <reference ui / operator>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

- graph31_absent_brand_representation_rule:
  - value: <например ОТСУТСТВУЕТ>
  - status: <status>
  - source: <reference ui / operator>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping

- payment_pattern_rule:
  - value: <например 1010 / 2010 / 5010>
  - status: <status>
  - source: <reference ui / xml / operator>
  - source_class: mapping_rule
  - transfer_relevance: needed_for_mapping
  - note: это правило расчетного оформления, а не shipment fact

---

# Эталонные наблюдения. Префикс: `reference_observed`

## Эталонное наблюдение. Префикс: `reference_observed.Reference[1]`

- field_name: <имя поля>
- observed_in: <xml_export / txt_export / screenshot / old_dt_pdf / transit_declaration>
- value: <значение>
- status: sample_only
- source_class: sample
- transfer_relevance: reference_only
- relevance_for_next_step: <useful_for_structure / useful_for_mapping / useful_for_process_rule /
  useful_for_completeness_check / likely_noise>
- comment: <зачем сохраняем>

### Важно

Если reference-наблюдение используется не просто как справка, а как стабильное правило проекции,
его нужно **не оставлять только в `reference_observed`**, а дополнительно поднимать в `mapping_rules`.

---

# Системные поля. Префикс: `system_only`

## Системное поле. Префикс: `system_only.SystemField[1]`

- field_name: <имя поля>
- observed_in: <xml_export / txt_export / screenshot>
- value: <значение>
- source_class: system
- transfer_relevance: do_not_transfer
- comment: <почему это системное поле>

---

# Конфликты

- field: <имя поля>
- status: unresolved
- source_1: <документ / источник>
- value_1: <значение>
- source_2: <документ / источник>
- value_2: <значение>
- impact: <что блокирует>
- action_required: <что нужно для снятия>

---

# Критичные пробелы

- field: <имя поля>
- status: unresolved
- expected_source: <откуда это должно быть получено>
- source_class_expected: <document / alta_master_data / calculated / operator / mapping_rule>
- reason: <почему данных нет>
- impact: <что блокирует>
- action_required: <что нужно сделать>

---

# Сводка готовности

- facts_confirmed_enough_for_review: <yes / no>
- facts_confirmed_enough_for_ui_projection: <yes / partial / no>
- facts_confirmed_enough_for_xml_projection: <yes / partial / no>
- facts_confirmed_enough_for_mapping_rule_extraction: <yes / partial / no>
- blocker_list:
  - <блокер 1>
  - <блокер 2>
  - <блокер 3>
- high_risk_fields:
  - <поле 1>
  - <поле 2>
- data_source_gaps:
  - shipment_docs_gap: <что именно не хватает из первички>
  - alta_master_data_gap: <что именно не хватает из карточек>
  - calculation_gap: <что именно не хватает для расчета>
  - mapping_rule_gap: <какие process / projection rules еще не закреплены>

---

## Что запрещено делать в facts_schema

Нельзя:
- смешивать shipment facts и alta master data в одном поле без пометки происхождения;
- хранить рассчитанную таможенную стоимость как будто это первичный факт;
- хранить режимы представления (`см. графу 14`) как будто это свойства компании;
- переносить из эталона телефоны, email, коды представителя, графу 54 и графу 47 как подтвержденные факты;
- считать графу 44 из эталона готовым набором документов новой ДТ;
- смешивать системные ED_ID / ED_STAT / BACK / FACE / GUID с данными для новой ДТ;
- хранить process / mapping rules как shipment facts;
- хранить коды графы 30, графы 44, графы 1 и другие process-константы, подтвержденные только эталоном,
  как `confirmed_document` факты поставки;
- подменять номер/дату документа СВХ итоговыми полями графы 30 без отдельного условного представления;
- нельзя удалять рабочее значение из `calculated_requirements` только потому, что для него уже выделено соответствующее
  `mapping_rule`;

---

## Что разрешено использовать из эталона

Разрешено использовать эталон для:
- проверки структуры XML/UI;
- проверки полноты generated-результатов;
- поиска устойчивых mapping-констант;
- выявления process-rules;
- улучшения шаблонов;
- сравнения generated vs reference;
- формирования `mapping_rules`.

Но эталон **не должен** автоматически превращаться в источник shipment facts новой поставки.

---

## Практический принцип

Хороший `facts.md` должен позволять отдельно и без смешения слоев ответить на 5 вопросов:

1. **Что подтверждено по документам поставки?**
2. **Что еще нужно взять из карточек / справочников Альты?**
3. **Что еще нужно рассчитать?**
4. **Какие process / mapping rules нужны для корректной проекции?**
5. **Что видно только в эталоне и нельзя переносить автоматически как факт новой поставки?**