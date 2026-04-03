# projection_mapping

## Назначение

`projection_mapping.md` — это карта перехода от `facts.md` к:

- `xml_import.md`
- `ui_input.md`

Документ фиксирует:

- из какого слоя берется значение;
- допускается ли автозаполнение или требуется ручная проверка;
- является ли значение business data, master data, calculated value, composition field или mapping rule.

---

## Базовое правило

Поле допускается в UI/XML только если одновременно понятны:

1. `semantic_source_layer`
2. `projection_rule`
3. `new_dt_policy`

Поля из `reference_observed` и `system_only` не являются источником новой ДТ, кроме случаев использования как structural / mapping hint.

---

## Смысловые слои

В карте используются слои:

- `documents_package`
- `shipment_facts`
- `alta_master_data_requirements`
- `calculated_requirements`
- `fact_composition`
- `mapping_rules`
- `reference_observed`
- `system_only`

`fact_composition` — это слой составных значений, собираемых из подтвержденных facts для UI/XML/presentation.
`mapping_rules` — это process / projection layer: значения, которые не являются фактами поставки, master data или расчетом, но нужны для корректного построения UI/XML.

---

## Уровни зрелости rules

| Уровень            | Смысл                                      | Автоподстановка |
| ------------------ | ------------------------------------------ | --------------- |
| `candidate`        | предварительное наблюдение без закрепления | нет             |
| `case-pattern`     | правило, наблюдавшееся на кейсе            | нет             |
| `production-ready` | утвержденное рабочее правило               | да              |

Если поле имеет `mapping_status = needs_validation`, оно не считается production-ready.

---

## Формат строки карты

| fact_path | semantic_source_layer | meaning | xml_target | ui_target | transform | multiplicity | required_for | allowed_source_class | projection_rule | reference_presence | new_dt_policy | mapping_status | notes |
| --------- | --------------------- | ------- | ---------- | --------- | --------- | ------------ | ------------ | -------------------- | --------------- | ------------------ | ------------- | -------------- | ----- |

---

## Поля карты

- `fact_path` — путь к полю в `facts.md` или имя process/mapping rule.
- `semantic_source_layer` — слой происхождения:
  - `documents_package`
  - `shipment_facts`
  - `alta_master_data_requirements`
  - `calculated_requirements`
  - `fact_composition`
  - `mapping_rules`
  - `reference_observed`
  - `system_only`
- `meaning` — краткий смысл поля.
- `xml_target` — целевое поле XML.
- `ui_target` — целевое поле UI / графа / подполе.
- `transform` — способ преобразования:
  - `direct`
  - `direct_if_present`
  - `split`
  - `compose`
  - `derived`
  - `conditional`
  - `manual_review`
  - `repeat_block`
  - `lookup`
  - `calculation_result`
  - `mapping_constant`
  - `mapping_lookup`
- `multiplicity` — кратность использования:
  - `single`
  - `repeat_goods`
  - `repeat_documents44`
  - `repeat_payments`
  - `repeat_other`
- `required_for` — где поле обязательно:
  - `both`
  - `xml`
  - `ui`
  - `optional`
  - `conditional`
  - `calculation_only`
  - `mapping_only`
- `allowed_source_class` — допустимые классы происхождения:
  - `document`
  - `operator`
  - `derived`
  - `alta_master_data`
  - `calculated`
  - `composed`
  - `mapping_rule`
  - `sample`
  - `system`
- `projection_rule` — правило допустимости проекции:
  - `allow_direct`
  - `allow_if_confirmed`
  - `allow_if_derived_from_confirmed`
  - `allow_if_from_alta_master_data`
  - `allow_if_calculated`
  - `allow_if_composed_from_confirmed`
  - `allow_if_mapping_rule_confirmed`
  - `allow_if_reference_used_as_mapping_hint`
  - `manual_review_only`
  - `never_from_sample_only`
  - `system_only`
- `reference_presence` — где поле наблюдалось в reference:
  - `seen_in_ui`
  - `seen_in_xml`
  - `seen_in_both`
  - `not_verified`
- `new_dt_policy` — политика использования в новой ДТ:
  - `must_fill_for_new_dt`
  - `conditional_for_new_dt`
  - `calculated_for_new_dt`
  - `master_data_for_new_dt`
  - `mapping_rule_for_new_dt`
  - `reference_only`
  - `do_not_transfer`
- `mapping_status` — уровень подтвержденности / зрелости правила:
  - `verified_by_operator_rule`
  - `verified_by_process`
  - `verified_by_reference`
  - `verified_by_ui`
  - `verified_by_xml`
  - `verified_by_case_pattern`
  - `inferred`
  - `needs_validation`

---

# Раздел 1. Shipment facts → business projection

## Общие реквизиты

| fact_path                               | semantic_source_layer | meaning                            | xml_target | ui_target                              | transform | multiplicity | required_for | allowed_source_class      | projection_rule                 | reference_presence | new_dt_policy        | mapping_status        | notes |
| --------------------------------------- | --------------------- | ---------------------------------- | ---------- | -------------------------------------- | --------- | ------------ | ------------ | ------------------------- | ------------------------------- | ------------------ | -------------------- | --------------------- | ----- |
| shipment_facts.incoterms_code           | shipment_facts        | Код условий поставки               | G_20_20    | 20 Условия поставки (код)              | direct    | single       | both         | document,operator         | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.incoterms_place          | shipment_facts        | Географический пункт Incoterms     | G_20_21    | 20 Условия поставки (место)            | direct    | single       | both         | document,operator         | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.invoice_currency_numeric | shipment_facts        | Цифровой код валюты                | G_22_1     | 22 Валюта — цифровой код               | direct    | single       | both         | document,derived          | allow_if_derived_from_confirmed | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.invoice_currency_alpha   | shipment_facts        | Буквенный код валюты               | G_22_3     | 22 Валюта — буквенный код              | direct    | single       | both         | document                  | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.invoice_amount_total     | shipment_facts        | Общая сумма по счету               | G_22_2     | 22 Общая сумма по счету                | direct    | single       | both         | document                  | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.package_count_total      | shipment_facts        | Общее количество мест              | G_6_1      | 6 Всего мест                           | direct    | single       | both         | document                  | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.gross_weight_total       | shipment_facts        | Суммарный вес брутто               | VES_BR_S   | Суммарный вес брутто                   | direct    | single       | both         | document                  | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.net_weight_total         | shipment_facts        | Суммарный вес нетто                | VES_NT_S   | Суммарный вес нетто                    | direct    | single       | both         | document                  | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.trade_country_code       | shipment_facts        | Торгующая страна, код              | G_11_1     | 11 Торгующая страна                    | direct    | single       | both         | document,derived          | allow_if_derived_from_confirmed | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.dispatch_country_name    | shipment_facts        | Страна отправления, наименование   | G_15_1     | 15 Страна отправления (наименование)   | direct    | single       | both         | document,derived          | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.dispatch_country_code    | shipment_facts        | Страна отправления, код            | G_15A_1    | 15 Страна отправления (код)            | direct    | single       | both         | document,derived          | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.origin_country_name      | shipment_facts        | Страна происхождения, наименование | G_16_1     | 16 Страна происхождения (наименование) | direct    | single       | both         | document,derived          | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.origin_country_code      | shipment_facts        | Страна происхождения, код          | G_16_2     | 16 Страна происхождения (код)          | direct    | single       | both         | document,derived          | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.destination_country_name | shipment_facts        | Страна назначения, наименование    | G_17_1     | 17 Страна назначения (наименование)    | direct    | single       | both         | document,derived,operator | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |
| shipment_facts.destination_country_code | shipment_facts        | Страна назначения, код             | G_17A_1    | 17 Страна назначения (код)             | direct    | single       | both         | document,derived,operator | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt | verified_by_reference |       |

---

## Отправитель

| fact_path                                      | semantic_source_layer | meaning                             | xml_target        | ui_target                                         | transform         | multiplicity | required_for | allowed_source_class | projection_rule                 | reference_presence | new_dt_policy          | mapping_status           | notes                                     |
| ---------------------------------------------- | --------------------- | ----------------------------------- | ----------------- | ------------------------------------------------- | ----------------- | ------------ | ------------ | -------------------- | ------------------------------- | ------------------ | ---------------------- | ------------------------ | ----------------------------------------- |
| shipment_facts.parties.sender.name             | shipment_facts        | Наименование отправителя            | G_2_NAM           | 2 Отправитель — наименование                      | direct            | single       | both         | document             | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference    |                                           |
| shipment_facts.parties.sender.country_code     | shipment_facts        | Код страны отправителя              | G_2_7             | 2 Отправитель — Страна                            | direct            | single       | both         | document,derived     | allow_if_derived_from_confirmed | seen_in_both       | must_fill_for_new_dt   | verified_by_reference    |                                           |
| shipment_facts.parties.sender.country_name     | shipment_facts        | Наименование страны отправителя     | G_2_50            | 2 Отправитель — страна, печатное представление    | direct            | single       | optional     | document,derived     | allow_if_derived_from_confirmed | seen_in_both       | conditional_for_new_dt | verified_by_reference    |                                           |
| shipment_facts.parties.sender.postcode         | shipment_facts        | Почтовый индекс отправителя         | G_2_POS           | 2 Отправитель — Почтовый код                      | direct            | single       | both         | document             | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference    |                                           |
| shipment_facts.parties.sender.region_or_area   | shipment_facts        | Регион / область отправителя        | G_2_SUB / G_2_CIT | 2 Отправитель — Область, район / Населенный пункт | split             | single       | both         | document             | manual_review_only              | seen_in_both       | must_fill_for_new_dt   | verified_by_case_pattern | разложение адреса требует проверки        |
| shipment_facts.parties.sender.city_or_locality | shipment_facts        | Населенный пункт отправителя        | G_2_SUB / G_2_CIT | 2 Отправитель — Область, район / Населенный пункт | split             | single       | both         | document             | manual_review_only              | seen_in_both       | must_fill_for_new_dt   | verified_by_case_pattern | требуется контроль распределения по полям |
| shipment_facts.parties.sender.street_address   | shipment_facts        | Улица / адресная строка отправителя | G_2_STR           | 2 Отправитель — Улица / адресная строка           | direct            | single       | both         | document             | manual_review_only              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference    | проверить разложение адреса               |
| shipment_facts.parties.sender.house            | shipment_facts        | Дом отправителя                     | G_2_BLD           | 2 Отправитель — Дом                               | direct_if_present | single       | conditional  | document,derived     | allow_if_derived_from_confirmed | seen_in_ui         | conditional_for_new_dt | needs_validation         |                                           |
| shipment_facts.parties.sender.office           | shipment_facts        | Офис отправителя                    | G_2_ROM           | 2 Отправитель — Офис                              | direct_if_present | single       | conditional  | document,derived     | allow_if_derived_from_confirmed | seen_in_ui         | conditional_for_new_dt | needs_validation         |                                           |
| shipment_facts.parties.sender.phone            | shipment_facts        | Телефон отправителя                 | G_2_PHONE         | 2 Отправитель — Телефон                           | direct_if_present | single       | conditional  | document,operator    | allow_if_confirmed              | seen_in_ui         | conditional_for_new_dt | inferred                 |                                           |
| shipment_facts.parties.sender.email            | shipment_facts        | Email отправителя                   | G_2_EMAIL         | 2 Отправитель — E-mail                            | direct_if_present | single       | conditional  | document,operator    | allow_if_confirmed              | seen_in_ui         | conditional_for_new_dt | inferred                 |                                           |

---

## Получатель

| fact_path                                                            | semantic_source_layer         | meaning                              | xml_target | ui_target                                     | transform         | multiplicity | required_for | allowed_source_class               | projection_rule                 | reference_presence | new_dt_policy          | mapping_status        | notes                                                   |
| -------------------------------------------------------------------- | ----------------------------- | ------------------------------------ | ---------- | --------------------------------------------- | ----------------- | ------------ | ------------ | ---------------------------------- | ------------------------------- | ------------------ | ---------------------- | --------------------- | ------------------------------------------------------- |
| shipment_facts.parties.consignee.name                                | shipment_facts                | Наименование получателя              | G_8_NAM    | 8 Получатель — наименование                   | direct            | single       | both         | document,operator,alta_master_data | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference |                                                         |
| shipment_facts.parties.consignee.inn_kpp                             | shipment_facts                | ИНН/КПП получателя                   | G_8_6      | 8 Получатель — ИНН/КПП                        | direct            | single       | both         | document,operator,alta_master_data | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference |                                                         |
| alta_master_data_requirements.consignee_profile.registration_id      | alta_master_data_requirements | ОГРН / рег. идентификатор получателя | G_8_1      | 8 Получатель — ОГРН / рег. идентификатор      | direct_if_present | single       | conditional  | operator,alta_master_data          | allow_if_from_alta_master_data  | seen_in_both       | master_data_for_new_dt | verified_by_reference | часто не из первички                                    |
| shipment_facts.parties.consignee.country_code                        | shipment_facts                | Код страны получателя                | G_8_7      | 8 Получатель — Страна                         | direct            | single       | both         | document,derived,operator          | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference |                                                         |
| shipment_facts.parties.consignee.country_name                        | shipment_facts                | Наименование страны получателя       | G_8_50     | 8 Получатель — страна, печатное представление | direct            | single       | optional     | document,derived,operator          | allow_if_derived_from_confirmed | seen_in_both       | conditional_for_new_dt | verified_by_reference |                                                         |
| shipment_facts.parties.consignee.postcode                            | shipment_facts                | Индекс получателя                    | G_8_POS    | 8 Получатель — Почтовый код                   | direct            | single       | both         | document,operator,alta_master_data | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference |                                                         |
| shipment_facts.parties.consignee.region_or_area                      | shipment_facts                | Регион получателя                    | G_8_SUB    | 8 Получатель — Область, район                 | direct            | single       | both         | document,operator,alta_master_data | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference |                                                         |
| shipment_facts.parties.consignee.city_or_locality                    | shipment_facts                | Населенный пункт получателя          | G_8_CIT    | 8 Получатель — Населенный пункт               | direct            | single       | both         | document,operator,alta_master_data | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference |                                                         |
| shipment_facts.parties.consignee.street                              | shipment_facts                | Улица получателя                     | G_8_STR    | 8 Получатель — Улица                          | direct            | single       | both         | document,operator,alta_master_data | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt   | verified_by_reference |                                                         |
| shipment_facts.parties.consignee.house                               | shipment_facts                | Дом получателя                       | G_8_BLD    | 8 Получатель — Дом                            | direct_if_present | single       | conditional  | document,operator,alta_master_data | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt | verified_by_reference |                                                         |
| shipment_facts.parties.consignee.office                              | shipment_facts                | Офис получателя                      | G_8_ROM    | 8 Получатель — Офис                           | direct_if_present | single       | conditional  | document,operator,alta_master_data | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt | verified_by_reference |                                                         |
| alta_master_data_requirements.consignee_profile.phone                | alta_master_data_requirements | Телефон получателя                   | G_8_PHONE  | 8 Получатель — Телефон                        | direct_if_present | single       | conditional  | operator,alta_master_data          | allow_if_from_alta_master_data  | seen_in_both       | master_data_for_new_dt | verified_by_reference | не брать автоматически из shipment docs как master data |
| alta_master_data_requirements.consignee_profile.email                | alta_master_data_requirements | Email получателя                     | G_8_EMAIL  | 8 Получатель — E-mail                         | direct_if_present | single       | conditional  | operator,alta_master_data          | allow_if_from_alta_master_data  | seen_in_both       | master_data_for_new_dt | verified_by_reference | не брать автоматически из shipment docs как master data |
| alta_master_data_requirements.consignee_profile.same_as_graph14_mode | alta_master_data_requirements | Режим "см. графу 14"                 | G_8_SM14   | 8 Получатель — ссылка на графу 14             | conditional       | single       | conditional  | operator,alta_master_data          | allow_if_from_alta_master_data  | seen_in_both       | master_data_for_new_dt | verified_by_reference | это режим представления, не факт о компании             |

---

# Раздел 2. Alta master data → projection

## Графа 9

| fact_path                                                                           | semantic_source_layer         | meaning                           | xml_target     | ui_target                     | transform         | multiplicity | required_for | allowed_source_class      | projection_rule                | reference_presence | new_dt_policy          | mapping_status        | notes |
| ----------------------------------------------------------------------------------- | ----------------------------- | --------------------------------- | -------------- | ----------------------------- | ----------------- | ------------ | ------------ | ------------------------- | ------------------------------ | ------------------ | ---------------------- | --------------------- | ----- |
| alta_master_data_requirements.financial_responsible_profile.name                    | alta_master_data_requirements | Наименование лица графы 9         | G_9_NAM        | 9 — наименование              | direct            | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.financial_responsible_profile.inn_kpp                 | alta_master_data_requirements | ИНН/КПП графы 9                   | G_9_4          | 9 — ИНН/КПП                   | direct            | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.financial_responsible_profile.ogrn_or_registration_id | alta_master_data_requirements | ОГРН / рег. идентификатор графы 9 | G_9_1          | 9 — ОГРН / рег. идентификатор | direct_if_present | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.financial_responsible_profile.country_code            | alta_master_data_requirements | Код страны графы 9                | G_9_7 / G_9_CC | 9 — Страна                    | direct            | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.financial_responsible_profile.phone                   | alta_master_data_requirements | Телефон графы 9                   | G_9_PHONE      | 9 — Телефон                   | direct_if_present | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.financial_responsible_profile.email                   | alta_master_data_requirements | Email графы 9                     | G_9_EMAIL      | 9 — E-mail                    | direct_if_present | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.financial_responsible_profile.same_as_graph14_mode    | alta_master_data_requirements | Режим "см. графу 14" для графы 9  | G_9_SM14       | 9 — ссылка на графу 14        | conditional       | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |

## Графа 14

| fact_path                                                               | semantic_source_layer         | meaning                              | xml_target | ui_target                                | transform         | multiplicity | required_for | allowed_source_class      | projection_rule                | reference_presence | new_dt_policy          | mapping_status        | notes |
| ----------------------------------------------------------------------- | ----------------------------- | ------------------------------------ | ---------- | ---------------------------------------- | ----------------- | ------------ | ------------ | ------------------------- | ------------------------------ | ------------------ | ---------------------- | --------------------- | ----- |
| alta_master_data_requirements.declarant_profile.name                    | alta_master_data_requirements | Наименование декларанта              | G_14_NAM   | 14 Декларант — наименование              | direct            | single       | both         | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.declarant_profile.inn_kpp                 | alta_master_data_requirements | ИНН/КПП декларанта                   | G_14_4     | 14 Декларант — ИНН/КПП                   | direct            | single       | both         | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.declarant_profile.ogrn_or_registration_id | alta_master_data_requirements | ОГРН / рег. идентификатор декларанта | G_14_1     | 14 Декларант — ОГРН / рег. идентификатор | direct_if_present | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.declarant_profile.country_code            | alta_master_data_requirements | Код страны декларанта                | G_14_CC    | 14 Декларант — Страна                    | direct            | single       | both         | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.declarant_profile.phone                   | alta_master_data_requirements | Телефон декларанта                   | G_14_PHONE | 14 Декларант — Телефон                   | direct_if_present | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.declarant_profile.email                   | alta_master_data_requirements | Email декларанта                     | G_14_EMAIL | 14 Декларант — E-mail                    | direct_if_present | single       | conditional  | operator,alta_master_data | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |

## Графа 54

| fact_path                                                                        | semantic_source_layer         | meaning                       | xml_target | ui_target                       | transform         | multiplicity | required_for | allowed_source_class               | projection_rule                | reference_presence | new_dt_policy          | mapping_status        | notes |
| -------------------------------------------------------------------------------- | ----------------------------- | ----------------------------- | ---------- | ------------------------------- | ----------------- | ------------ | ------------ | ---------------------------------- | ------------------------------ | ------------------ | ---------------------- | --------------------- | ----- |
| alta_master_data_requirements.representative_profile.last_name                   | alta_master_data_requirements | Фамилия представителя         | G_54_3     | 54 — Фамилия                    | direct            | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.first_name                  | alta_master_data_requirements | Имя представителя             | G_54_3NM   | 54 — Имя                        | direct            | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.middle_name                 | alta_master_data_requirements | Отчество представителя        | G_54_3MD   | 54 — Отчество                   | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.phone                       | alta_master_data_requirements | Телефон представителя         | G_54_21    | 54 — Телефон                    | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.email                       | alta_master_data_requirements | Email представителя           | G_54_EMAIL | 54 — E-mail                     | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.customs_representative_code | alta_master_data_requirements | Код представителя             | G_54_8     | 54 — код представителя          | direct_if_present | single       | conditional  | operator,alta_master_data          | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.authority_doc_name          | alta_master_data_requirements | Документ полномочий           | G_54_4     | 54 — документ полномочий        | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.authority_doc_number        | alta_master_data_requirements | Номер документа полномочий    | G_54_5     | 54 — номер документа полномочий | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.authority_doc_date_from     | alta_master_data_requirements | Начало действия полномочий    | G_54_60    | 54 — документ полномочий от     | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.authority_doc_date_to       | alta_master_data_requirements | Окончание действия полномочий | G_54_61    | 54 — документ полномочий до     | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.passport_type               | alta_master_data_requirements | Вид документа личности        | G_54_9     | 54 — документ личности, вид     | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.passport_series             | alta_master_data_requirements | Серия документа личности      | G_54_12    | 54 — серия документа            | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.passport_number             | alta_master_data_requirements | Номер документа личности      | G_54_100   | 54 — номер документа            | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.passport_date               | alta_master_data_requirements | Дата документа личности       | G_54_101   | 54 — дата документа             | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |
| alta_master_data_requirements.representative_profile.passport_issuer             | alta_master_data_requirements | Кем выдан документ личности   | G_54_13    | 54 — кем выдан                  | direct_if_present | single       | conditional  | operator,alta_master_data,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference |       |

---

# Раздел 3. Mapping rules → projection constants and process rules

## Графа 1 и общие режимы декларации

| fact_path                                      | semantic_source_layer | meaning                    | xml_target | ui_target                | transform        | multiplicity | required_for | allowed_source_class | projection_rule                 | reference_presence | new_dt_policy           | mapping_status           | notes                                                           |
| ---------------------------------------------- | --------------------- | -------------------------- | ---------- | ------------------------ | ---------------- | ------------ | ------------ | -------------------- | ------------------------------- | ------------------ | ----------------------- | ------------------------ | --------------------------------------------------------------- |
| mapping_rules.declaration_direction_code_rule  | mapping_rules         | Код направления декларации | G_1_1      | 1 Декларация — тип       | mapping_constant | single       | both         | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | для импортного кейса наблюдается `ИМ`; не считать shipment fact |
| mapping_rules.declaration_procedure_code_rule  | mapping_rules         | Код процедуры              | G_1_2      | 1 Декларация — процедура | mapping_constant | single       | both         | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | для кейса наблюдается `40`                                      |
| mapping_rules.electronic_declaration_flag_rule | mapping_rules         | Признак ЭД                 | G_1_31     | 1 Декларация — ЭД        | mapping_constant | single       | conditional  | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | использовать как process rule                                   |
| mapping_rules.forms_main_rule                  | mapping_rules         | Основная форма             | G_3_1      | 3 Формы — 1              | mapping_constant | single       | conditional  | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | needs_validation         | в reference наблюдается `1`, но rule еще не production-ready    |
| mapping_rules.forms_additional_rule            | mapping_rules         | Доп. форма                 | G_3_2      | 3 Формы — 2              | mapping_constant | single       | conditional  | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | needs_validation         | в reference наблюдается `1`, но rule еще не production-ready    |

## Графа 18 / 19 / 21 / 25 / 26 / 29

| fact_path                                                        | semantic_source_layer   | meaning                                    | xml_target | ui_target                                       | transform        | multiplicity | required_for | allowed_source_class         | projection_rule                         | reference_presence | new_dt_policy           | mapping_status        | notes                                          |
| ---------------------------------------------------------------- | ----------------------- | ------------------------------------------ | ---------- | ----------------------------------------------- | ---------------- | ------------ | ------------ | ---------------------------- | --------------------------------------- | ------------------ | ----------------------- | --------------------- | ---------------------------------------------- |
| calculated_requirements.transport_identification_for_declaration | calculated_requirements | Идентификация ТС для декларации            | G_18       | 18 Идентификация тр. ср-ва                      | conditional      | single       | conditional  | operator,document,calculated | manual_review_only                      | seen_in_both       | conditional_for_new_dt  | verified_by_reference |                                                |
| mapping_rules.graph18_kind_rule                                  | mapping_rules           | Вид/режим заполнения поля 18               | G_18_0     | 18 Вид ТС / код режима                          | mapping_constant | single       | conditional  | mapping_rule                 | allow_if_mapping_rule_confirmed         | seen_in_both       | mapping_rule_for_new_dt | needs_validation      | в кейсе наблюдается `2`                        |
| mapping_rules.graph18_registration_country_rule                  | mapping_rules           | Код страны регистрации ТС по process-layer | G_18_2     | 18 Страна регистрации                           | mapping_constant | single       | conditional  | mapping_rule                 | allow_if_reference_used_as_mapping_hint | seen_in_both       | mapping_rule_for_new_dt | needs_validation      | не путать process-code и business country code |
| mapping_rules.graph19_container_flag_rule                        | mapping_rules           | Признак контейнера                         | G_19_1     | 19 Конт.                                        | mapping_constant | single       | conditional  | mapping_rule                 | allow_if_mapping_rule_confirmed         | seen_in_both       | mapping_rule_for_new_dt | needs_validation      | в кейсе наблюдается `0`                        |
| mapping_rules.graph21_kind_rule                                  | mapping_rules           | Вид/режим активного ТС на границе          | G_21_0     | 21 Идентификация активного тр. ср-ва на границе | mapping_constant | single       | conditional  | mapping_rule                 | allow_if_mapping_rule_confirmed         | seen_in_both       | mapping_rule_for_new_dt | needs_validation      | в кейсе наблюдается `1`                        |
| calculated_requirements.border_transport_mode_code               | calculated_requirements | Код вида транспорта на границе             | G_25_1     | 25 Код вида транспорта                          | direct           | single       | conditional  | operator,document,calculated | allow_if_confirmed                      | seen_in_both       | conditional_for_new_dt  | verified_by_reference |                                                |
| calculated_requirements.departure_transport_mode_code            | calculated_requirements | Код вида транспорта при отправлении        | G_26_1     | 26 Код вида транспорта                          | direct           | single       | conditional  | operator,document,calculated | allow_if_confirmed                      | seen_in_both       | conditional_for_new_dt  | verified_by_reference |                                                |
| calculated_requirements.border_customs_code                      | calculated_requirements | Код таможни на границе                     | G_29_1     | 29 Код таможни                                  | direct           | single       | conditional  | operator,document,calculated | allow_if_confirmed                      | seen_in_both       | conditional_for_new_dt  | verified_by_reference |                                                |
| calculated_requirements.border_customs_name                      | calculated_requirements | Наименование таможни на границе            | G_29_2     | 29 Наименование таможни                         | lookup           | single       | conditional  | operator,system,calculated   | manual_review_only                      | seen_in_both       | conditional_for_new_dt  | verified_by_reference |                                                |

## Графа 30

### Важно: графа 30 строится из двух уровней

Для графы 30 нужно разделять:

1. `shipment_facts`
   
   - факт хранения
   - номер и дата документа СВХ / реестрового документа
   - адрес местонахождения

2. `mapping_rules`
   
   - код типа
   - код вида документа
   - финальный номер / дата
   - код таможни

Shipment-layer и process-layer нельзя смешивать.

| fact_path                                       | semantic_source_layer | meaning                                    | xml_target         | ui_target                  | transform        | multiplicity | required_for | allowed_source_class     | projection_rule                 | reference_presence | new_dt_policy           | mapping_status           | notes                                                   |
| ----------------------------------------------- | --------------------- | ------------------------------------------ | ------------------ | -------------------------- | ---------------- | ------------ | ------------ | ------------------------ | ------------------------------- | ------------------ | ----------------------- | ------------------------ | ------------------------------------------------------- |
| shipment_facts.location_type                    | shipment_facts        | Тип местонахождения / тип заполнения       | G_30_0             | 30 Тип                     | conditional      | single       | conditional  | document,operator,sample | manual_review_only              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    | shipment fact сам по себе не равен коду поля            |
| mapping_rules.graph30_type_code_rule            | mapping_rules         | Код типа местонахождения товара            | G_30_0             | 30 Тип                     | mapping_constant | single       | both         | mapping_rule             | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | needs_validation         | candidate `11`, до production закреплять явно           |
| mapping_rules.graph30_document_kind_code_rule   | mapping_rules         | Код вида документа графы 30                | G_30_10            | 30 Вид / 1/2/3             | mapping_constant | single       | both         | mapping_rule             | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | needs_validation         | candidate `2`, до production закреплять явно            |
| shipment_facts.warehouse_registry_number        | shipment_facts        | Номер реестрового документа / регистрац. № | G_30_1             | 30 Номер документа         | direct           | single       | both         | document                 | manual_review_only              | seen_in_ui         | conditional_for_new_dt  | verified_by_reference    | по текущей схеме это источник итогового номера графы 30 |
| shipment_facts.warehouse_document_date          | shipment_facts        | Дата документа СВХ                         | G_30_DATE          | 30 Дата                    | direct           | single       | both         | document                 | manual_review_only              | seen_in_ui         | conditional_for_new_dt  | verified_by_reference    | по текущей схеме это источник итоговой даты графы 30    |
| shipment_facts.warehouse_country_code           | shipment_facts        | Страна местонахождения                     | G_30_CC            | 30 Страна                  | direct           | single       | conditional  | document,derived         | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    |                                                         |
| shipment_facts.warehouse_region_or_area         | shipment_facts        | Регион местонахождения                     | G_30_SUB           | 30 Область, район          | direct           | single       | conditional  | document,derived         | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    |                                                         |
| shipment_facts.warehouse_city_or_locality       | shipment_facts        | Населенный пункт местонахождения           | G_30_CIT           | 30 Населенный пункт        | direct           | single       | conditional  | document,derived         | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    |                                                         |
| shipment_facts.warehouse_street_address         | shipment_facts        | Улица / адрес местонахождения              | G_30_STR           | 30 Улица / адрес           | direct           | single       | conditional  | document,derived         | manual_review_only              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    |                                                         |
| shipment_facts.warehouse_customs_code_candidate | shipment_facts        | Candidate-код таможни места хранения       | G_30_12_candidate  | 30 Код таможни (candidate) | direct           | single       | conditional  | document,sample          | manual_review_only              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    | не путать candidate и финальный process-rule            |
| mapping_rules.graph30_customs_code_rule         | mapping_rules         | Финальный код таможни графы 30             | G_30_12            | 30 Код таможни             | mapping_constant | single       | both         | mapping_rule             | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | наблюдалось значение `10404083`; закреплять как rule    |
| fact_composition.warehouse_printed_address      | fact_composition      | Печатная строка графы 30                   | G_30P_1            | 30 Печатная форма          | compose          | single       | xml          | composed                 | manual_review_only              | seen_in_both       | reference_only          | verified_by_reference    | presentation-field                                      |
| shipment_facts.warehouse_address_raw            | shipment_facts        | Сырой адрес местонахождения                | G_30_STR / G_30P_1 | 30 Адрес / печатная форма  | split,compose    | single       | conditional  | document,operator        | manual_review_only              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    | использовать как источник декомпозиции                  |

### Decision-rule для графы 30

1. Business-input и process-rule не смешиваются.
2. `warehouse_*_candidate` — это candidate-layer, а не финальные XML-значения.
3. Финальные `G_30_0`, `G_30_10`, `G_30_12` можно автозаполнять только при production-approved rule.
4. `G_30_1` и `G_30_DATE` берутся из подтвержденных shipment facts (`warehouse_registry_number`, `warehouse_document_date`),
   но до стабилизации кейс-практики требуют manual review.
5. Если production-rule не закреплен:
   - `ui_input.md` допускается только как manual-review representation;
   - `xml_import.md` не должен автозаполнять графу 30 как окончательную.

---

# Раздел 4. Goods and valuation projection

## Товарный блок

| fact_path                                                   | semantic_source_layer   | meaning                          | xml_target                      | ui_target                   | transform          | multiplicity | required_for | allowed_source_class       | projection_rule                 | reference_presence | new_dt_policy           | mapping_status           | notes                                                               |
|-------------------------------------------------------------|-------------------------|----------------------------------|---------------------------------|-----------------------------|--------------------|--------------|--------------|----------------------------|---------------------------------|--------------------|-------------------------|--------------------------|---------------------------------------------------------------------|
| shipment_facts.Goods[].item_no                              | shipment_facts          | Номер товара                     | BLOCK[].G_32_1                  | 32 Товар                    | direct             | repeat_goods | both         | derived,document           | allow_if_derived_from_confirmed | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |
| fact_composition.description_31_main_draft                  | fact_composition        | Основное описание графы 31       | BLOCK[].G_31.NAME               | 31 Основное описание        | compose            | repeat_goods | both         | composed                   | manual_review_only              | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    | черновик допустим как материал, не слепой перенос                   |
| fact_composition.description_31_lines                       | fact_composition        | Дополнительные строки графы 31   | BLOCK[].TXT[]                   | 31 Дополнительные строки    | repeat_block       | repeat_goods | conditional  | composed                   | manual_review_only              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    |                                                                     |
| shipment_facts.Goods[].description_31_source_facts          | shipment_facts          | Набор фактов для сборки графы 31 | BLOCK[].G_31 / BLOCK[].TXT[]    | 31 Основное описание / доп. | compose            | repeat_goods | both         | document,operator,derived  | manual_review_only              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    | material set, а не готовая строка                                   |
| shipment_facts.Goods[].package_places_count                 | shipment_facts          | Количество мест по товару        | BLOCK[].G_31.PLACE              | 31 Количество мест          | direct_if_present  | repeat_goods | conditional  | document                   | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    |                                                                     |
| shipment_facts.Goods[].package_places_marking               | shipment_facts          | Маркировка / упаковка            | BLOCK[].G_31.PLACE2             | 31 Маркировка / упаковка    | direct_if_present  | repeat_goods | conditional  | document,operator          | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    | финальная строка требует ручной проверки                            |
| shipment_facts.Goods[].hs_code                              | shipment_facts          | Код ТН ВЭД                       | BLOCK[].G_33_1                  | 33 ТН ВЭД                   | direct             | repeat_goods | both         | document,operator,derived  | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |
| shipment_facts.Goods[].origin_country_code                  | shipment_facts          | Страна происхождения товара      | BLOCK[].G_34_1                  | 34 Страна происхождения     | direct             | repeat_goods | both         | document,derived           | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |
| shipment_facts.Goods[].gross_weight                         | shipment_facts          | Вес брутто товара                | BLOCK[].G_35_1                  | 35 Вес брутто               | direct             | repeat_goods | both         | document                   | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |
| calculated_requirements.GoodsCalculated[].preference_code   | calculated_requirements | Преференция                      | BLOCK[].G_36_2                  | 36 Преференция              | direct             | repeat_goods | conditional  | operator,calculated,sample | allow_if_mapping_rule_confirmed | seen_in_both       | conditional_for_new_dt  | verified_by_case_pattern | process/mapping ориентир                                            |
| calculated_requirements.GoodsCalculated[].procedure_code    | calculated_requirements | Код процедуры по товару          | BLOCK[].G_37_1                  | 37 Процедура                | direct             | repeat_goods | both         | operator,calculated,sample | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    | рабочее значение для текущего кейса                                 |
| shipment_facts.Goods[].net_weight                           | shipment_facts          | Вес нетто товара                 | BLOCK[].G_38_1                  | 38 Вес нетто                | direct             | repeat_goods | both         | document                   | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |
| shipment_facts.Goods[].invoice_value                        | shipment_facts          | Фактурная стоимость товара       | BLOCK[].G_42_1                  | 42 Цена товара              | direct             | repeat_goods | both         | document                   | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |
| mapping_rules.graph42_value_in_dts_rule                     | mapping_rules           | Признак "В ДТС"                  | G_42_2                          | 42 Признак "В ДТС"          | mapping_constant   | single       | conditional  | mapping_rule               | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | это process rule                                                    |
| calculated_requirements.GoodsCalculated[].customs_value     | calculated_requirements | Таможенная стоимость товара      | BLOCK[].G_45_0 / BLOCK[].G_45_1 | 45 Таможенная стоимость     | calculation_result | repeat_goods | both         | calculated                 | allow_if_calculated             | seen_in_both       | calculated_for_new_dt   | verified_by_reference    |                                                                     |
| calculated_requirements.GoodsCalculated[].statistical_value | calculated_requirements | Статистическая стоимость товара  | BLOCK[].G_46_1                  | 46 Статистическая стоимость | calculation_result | repeat_goods | both         | calculated                 | allow_if_calculated             | seen_in_both       | calculated_for_new_dt   | verified_by_reference    |                                                                     |
| fact_composition.group_description                          | fact_composition        | Описание группы                  | BLOCK[].TOVG[].G31_1            | 31 Таблица — описание       | compose            | repeat_goods | conditional  | composed                   | manual_review_only              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    | presentation-поле                                                   |
| shipment_facts.Goods[].manufacturer                         | shipment_facts          | Производитель                    | BLOCK[].TOVG[].G31_11           | 31 Таблица — производитель  | direct             | repeat_goods | both         | document,operator          | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |
| shipment_facts.Goods[].trademark                            | shipment_facts          | Товарный знак                    | BLOCK[].TOVG[].G31_12           | 31 Таблица — товарный знак  | direct_if_present  | repeat_goods | conditional  | document,operator,derived  | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt  | verified_by_reference    |                                                                     |
| shipment_facts.Goods[].brand                                | shipment_facts          | Марка / бренд                    | BLOCK[].TOVG[].G31_14           | 31 Таблица — марка          | direct_if_present  | repeat_goods | conditional  | document,operator,derived  | allow_if_confirmed              | seen_in_both       | conditional_for_new_dt  | verified_by_xml          | значение `ОТСУТСТВУЕТ` возможно как оформленное presentation-value  |
| shipment_facts.Goods[].model                                | shipment_facts          | Модель                           | BLOCK[].TOVG[].G31_15_MOD       | 31 Таблица — модель         | direct             | repeat_goods | both         | document,operator          | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |
| shipment_facts.Goods[].quantity                             | shipment_facts          | Количество                       | BLOCK[].TOVG[].KOLVO            | 31 Таблица — количество     | direct             | repeat_goods | both         | document                   | allow_if_confirmed              | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |
| shipment_facts.Goods[].unit_code                            | shipment_facts          | Код единицы измерения            | BLOCK[].TOVG[].CODE_EDI         | 31 Таблица — код ед. изм.   | direct             | repeat_goods | both         | document,derived,operator  | allow_if_derived_from_confirmed | seen_in_both       | must_fill_for_new_dt    | needs_validation         | до production обязательно подтвердить код справочником / оператором |
| shipment_facts.Goods[].unit_name                            | shipment_facts          | Наименование единицы измерения   | BLOCK[].TOVG[].NAME_EDI         | 31 Таблица — ед. изм.       | direct             | repeat_goods | both         | document,derived           | allow_if_derived_from_confirmed | seen_in_both       | must_fill_for_new_dt    | verified_by_reference    |                                                                     |

### Decision-rule для `unit_code`

1. `unit_name` — business-level поле, `unit_code` — code-layer.
2. Если `unit_code` имеет `needs_validation`, его нельзя автопереносить в финальный `xml_import.md` до подтверждения.

---

# Раздел 5. Documents for graph 44

## Candidate → final graph 44 mapping

### Важно

В текущем `facts_schema.md` нет отдельного материализованного слоя `documents_for_graph44_candidates`.
Поэтому candidate-level для графы 44 в этой карте понимается как **логический слой**, собираемый из:

- `documents_package.*`
- подтвержденных реквизитов документа;
- `mapping_rules.graph44_*`
- `fact_composition.graph44_doc_text`

То есть сначала определяется business-role документа, затем применяется mapping rule `role -> final code`.

| fact_path                                                                    | semantic_source_layer         | meaning                               | xml_target                | ui_target                   | transform         | multiplicity       | required_for | allowed_source_class               | projection_rule                | reference_presence | new_dt_policy          | mapping_status        | notes                                           |
|------------------------------------------------------------------------------| ----------------------------- | ------------------------------------- |---------------------------|-----------------------------|-------------------| ------------------ |--------------| ---------------------------------- | ------------------------------ |--------------------| ---------------------- | --------------------- |-------------------------------------------------|
| documents_package.transport_doc.number                                       | documents_package             | Номер CMR / транспортного документа   | BLOCK[].G44[].G442        | 44 — номер                  | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=cmr                           |
| documents_package.transport_doc.date                                         | documents_package             | Дата CMR / транспортного документа    | BLOCK[].G44[].G443        | 44 — дата                   | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=cmr                           |
| documents_package.contract.number                                            | documents_package             | Номер контракта                       | BLOCK[].G44[].G442        | 44 — номер                  | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=contract                      |
| documents_package.contract.date                                              | documents_package             | Дата контракта                        | BLOCK[].G44[].G443        | 44 — дата                   | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=contract                      |
| documents_package.invoice.number                                             | documents_package             | Номер инвойса                         | BLOCK[].G44[].G442        | 44 — номер                  | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=invoice                       |
| documents_package.invoice.date                                               | documents_package             | Дата инвойса                          | BLOCK[].G44[].G443        | 44 — дата                   | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=invoice                       |
| documents_package.packing_list.number                                        | documents_package             | Номер packing list                    | BLOCK[].G44[].G442        | 44 — номер                  | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=packing_list                  |
| documents_package.packing_list.date                                          | documents_package             | Дата packing list                     | BLOCK[].G44[].G443        | 44 — дата                   | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=packing_list                  |
| documents_package.payment_doc.number                                         | documents_package             | Номер платежного документа            | BLOCK[].G44[].G442        | 44 — номер                  | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=payment                       |
| documents_package.payment_doc.date                                           | documents_package             | Дата платежного документа             | BLOCK[].G44[].G443        | 44 — дата                   | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=payment                       |
| documents_package.tech_description.number                                    | documents_package             | Номер техописания                     | BLOCK[].G44[].G442        | 44 — номер                  | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=tech_description              |
| documents_package.tech_description.date                                      | documents_package             | Дата техописания                      | BLOCK[].G44[].G443        | 44 — дата                   | direct            | repeat_documents44 | both         | document                           | allow_if_confirmed             | seen_in_both       | must_fill_for_new_dt   | verified_by_reference | для business_role=tech_description              |
| alta_master_data_requirements.representative_profile.passport_number         | alta_master_data_requirements | Номер документа личности представителя| BLOCK[].G44[].G442        | 44 — номер                  | direct            | repeat_documents44 | conditional  | alta_master_data,operator,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference | для business_role=representative_identity_doc   |
| alta_master_data_requirements.representative_profile.passport_date           | alta_master_data_requirements | Дата документа личности представителя | BLOCK[].G44[].G443        | 44 — дата                   | direct            | repeat_documents44 | conditional  | alta_master_data,operator,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference | для business_role=representative_identity_doc   |
| alta_master_data_requirements.representative_profile.authority_doc_number    | alta_master_data_requirements | Номер доверенности                    | BLOCK[].G44[].G442        | 44 — номер                  | direct            | repeat_documents44 | conditional  | alta_master_data,operator,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference | для business_role=authority_doc                 |
| alta_master_data_requirements.representative_profile.authority_doc_date_from | alta_master_data_requirements | Дата доверенности / начало действия   | BLOCK[].G44[].G443 / G446 | 44 — дата / срок действия с | direct            | repeat_documents44 | conditional  | alta_master_data,operator,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference | для business_role=authority_doc                 |
| alta_master_data_requirements.representative_profile.authority_doc_date_to   | alta_master_data_requirements | Окончание действия доверенности       | BLOCK[].G44[].G447        | 44 — срок действия по       | direct_if_present | repeat_documents44 | conditional  | alta_master_data,operator,document | allow_if_from_alta_master_data | seen_in_both       | master_data_for_new_dt | verified_by_reference | для business_role=authority_doc                 |
| fact_composition.graph44_doc_text                                            | fact_composition              | Сводная строка документа графы 44     | BLOCK[].G44[].DOCTEXT     | 44 Итоговая строка          | compose           | repeat_documents44 | xml          | composed                           | manual_review_only             | seen_in_both       | reference_only         | verified_by_reference | presentation-field                              |
| system_only.SystemField[].value                                              | system_only                   | Системные ED-связки                   | BACK / FACE / ED_*        | 44 Служебные признаки       | direct            | repeat_documents44 | optional     | system                             | system_only                    | seen_in_xml        | do_not_transfer        | verified_by_xml       | переносить нельзя                               |

## Нормализованные правила кодов графы 44

| fact_path                                          | semantic_source_layer | meaning                                 | xml_target         | ui_target          | transform        | multiplicity       | required_for | allowed_source_class | projection_rule                 | reference_presence | new_dt_policy           | mapping_status           | notes                                             |
| -------------------------------------------------- | --------------------- | --------------------------------------- | ------------------ | ------------------ | ---------------- | ------------------ | ------------ | -------------------- | ------------------------------- | ------------------ | ----------------------- | ------------------------ | ------------------------------------------------- |
| mapping_rules.graph44_contract_primary             | mapping_rules         | Код графы 44 для контракта              | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `03011`                                           |
| mapping_rules.graph44_cmr_primary                  | mapping_rules         | Код графы 44 для CMR                    | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `02015`                                           |
| mapping_rules.graph44_invoice_primary              | mapping_rules         | Код графы 44 для коммерческого инвойса  | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `04021`                                           |
| mapping_rules.graph44_payment_primary              | mapping_rules         | Код графы 44 для заявления на перевод   | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `04023`                                           |
| mapping_rules.graph44_transport_invoice_primary    | mapping_rules         | Код графы 44 для счета за перевозку     | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `04031`                                           |
| mapping_rules.graph44_transport_contract_primary   | mapping_rules         | Код графы 44 для договора по перевозке  | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `04033`                                           |
| mapping_rules.graph44_packing_list_primary         | mapping_rules         | Код графы 44 для упаковочного листа     | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `04131`                                           |
| mapping_rules.graph44_tech_description_primary     | mapping_rules         | Код графы 44 для техописания            | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `05999`; binary-copy не заменяет основной код     |
| mapping_rules.graph44_transit_doc_primary          | mapping_rules         | Код графы 44 для транзитной декларации  | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `09013`                                           |
| mapping_rules.graph44_tech_description_attachment  | mapping_rules         | Binary-copy для техописания             | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `09023`; не заменяет основной business-code       |
| mapping_rules.graph44_invoice_attachment           | mapping_rules         | Binary-copy для инвойса                 | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `09023`; не заменяет основной business-code       |
| mapping_rules.graph44_transport_request_attachment | mapping_rules         | Binary-copy для заявки                  | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `09023`; attachment pattern                       |
| mapping_rules.graph44_payment_attachment           | mapping_rules         | Binary-copy для платежного документа    | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `09023`; attachment pattern                       |
| mapping_rules.graph44_transport_invoice_attachment | mapping_rules         | Binary-copy для счета за перевозку      | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `09023`; attachment pattern                       |
| mapping_rules.graph44_cmr_attachment               | mapping_rules         | Binary-copy для CMR                     | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `09023`; attachment pattern                       |
| mapping_rules.graph44_svh_report_attachment        | mapping_rules         | Binary-copy для отчета СВХ              | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `09023`; attachment pattern                       |
| mapping_rules.graph44_svh_report_primary           | mapping_rules         | Основной код для отчета СВХ             | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `09026`; использовать только если требует процесс |
| mapping_rules.graph44_passport_primary             | mapping_rules         | Код графы 44 для паспорта представителя | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `11001`                                           |
| mapping_rules.graph44_power_of_attorney_primary    | mapping_rules         | Код графы 44 для доверенности           | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `11004`                                           |
| mapping_rules.graph44_registry_extract_primary     | mapping_rules         | Код графы 44 для выписки ЕГРЮЛ          | BLOCK[].G44[].G441 | 44 — код документа | mapping_constant | repeat_documents44 | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | `04011`                                           |

### Decision-rule для графы 44

1. Final code нельзя брать напрямую из reference как факт.
2. Сначала определяется business-role документа и подтверждаются его business-реквизиты.
3. Затем выбирается final code по `mapping_rules.graph44_*`.
4. Дополнительные process-коды (`09023`, `09026`, `11001`, `11004`, `04011`) не заменяют основной business-code документа, если не являются самостоятельной business-role записью.
5. Если matrix role → code не production-approved:
   - `ui_input.md` может содержать suggested code;
   - `xml_import.md` не должен безусловно заполнять `G441`.

---

# Раздел 6. Calculated fields → payments and valuation

### Важно: calculation result vs calculation pattern

Нужно различать:

1. расчет текущего кейса (`calculated_requirements`);
2. pattern строк и кодов (`mapping_rules`).

Pattern не равен расчету и не должен автоматически подменять applied calculation.

| fact_path                                         | semantic_source_layer   | meaning                    | xml_target            | ui_target                     | transform          | multiplicity    | required_for     | allowed_source_class         | projection_rule     | reference_presence | new_dt_policy         | mapping_status        | notes                                            |
| ------------------------------------------------- | ----------------------- | -------------------------- | --------------------- | ----------------------------- | ------------------ | --------------- | ---------------- | ---------------------------- | ------------------- | ------------------ | --------------------- | --------------------- | ------------------------------------------------ |
| calculated_requirements.exchange_rate             | calculated_requirements | Курс валюты                | G_23_1 / G_23_2       | 23 Курс валюты                | calculation_result | single          | both             | calculated,system,operator   | allow_if_calculated | seen_in_both       | calculated_for_new_dt | verified_by_reference |                                                  |
| calculated_requirements.customs_value_total       | calculated_requirements | Общая таможенная стоимость | G_12_0 / G_12_1       | 12 Общая таможенная стоимость | calculation_result | single          | both             | calculated                   | allow_if_calculated | seen_in_both       | calculated_for_new_dt | verified_by_reference |                                                  |
| calculated_requirements.transport_cost_to_border  | calculated_requirements | Расходы до границы         | влияет на G_45 / G_47 | расчет ДТС / стоимости        | derived            | single          | calculation_only | document,operator,calculated | allow_if_confirmed  | seen_in_ui         | calculated_for_new_dt | verified_by_ui        | не прямое поле XML, а вход расчета               |
| calculated_requirements.insurance                 | calculated_requirements | Страхование                | влияет на G_45 / G_47 | расчет ДТС / стоимости        | derived            | single          | calculation_only | document,operator,calculated | allow_if_confirmed  | seen_in_ui         | calculated_for_new_dt | verified_by_ui        | без подтверждения не считать нулем автоматически |
| calculated_requirements.Payments[].payment_code   | calculated_requirements | Вид платежа                | BLOCK[].G_47_*_1      | 47 — Вид                      | calculation_result | repeat_payments | both             | calculated                   | allow_if_calculated | seen_in_both       | calculated_for_new_dt | verified_by_reference |                                                  |
| calculated_requirements.Payments[].payment_base   | calculated_requirements | Основа начисления          | BLOCK[].G_47_*_2      | 47 — Основа начисления        | calculation_result | repeat_payments | both             | calculated                   | allow_if_calculated | seen_in_both       | calculated_for_new_dt | verified_by_reference |                                                  |
| calculated_requirements.Payments[].payment_rate   | calculated_requirements | Ставка                     | BLOCK[].G_47_*_3      | 47 — Ставка                   | calculation_result | repeat_payments | both             | calculated                   | allow_if_calculated | seen_in_both       | calculated_for_new_dt | verified_by_reference |                                                  |
| calculated_requirements.Payments[].payment_amount | calculated_requirements | Сумма                      | BLOCK[].G_47_*_4      | 47 — Сумма                    | calculation_result | repeat_payments | both             | calculated                   | allow_if_calculated | seen_in_both       | calculated_for_new_dt | verified_by_reference |                                                  |
| calculated_requirements.Payments[].payment_sp     | calculated_requirements | СП                         | BLOCK[].G_47_*_5      | 47 — СП                       | direct             | repeat_payments | both             | operator,calculated          | allow_if_confirmed  | seen_in_both       | calculated_for_new_dt | verified_by_reference |                                                  |
| fact_composition.payment_summary_line             | fact_composition        | Итоговая строка платежей   | B_* / B_7             | Блок итогов платежей          | compose            | single          | optional         | composed                     | manual_review_only  | seen_in_both       | reference_only        | verified_by_reference | presentation-field                               |

## Минимальная расчетная схема для process-layer

| fact_path                          | semantic_source_layer | meaning         | xml_target     | ui_target    | transform        | multiplicity    | required_for | allowed_source_class | projection_rule                 | reference_presence | new_dt_policy           | mapping_status           | notes                                                    |
| ---------------------------------- | --------------------- | --------------- | -------------- | ------------ | ---------------- | --------------- | ------------ | -------------------- | ------------------------------- | ------------------ | ----------------------- | ------------------------ | -------------------------------------------------------- |
| mapping_rules.payment_pattern_rule | mapping_rules         | Шаблон графы 47 | BLOCK[].G_47_* | 47 — pattern | mapping_constant | repeat_payments | mapping_only | mapping_rule         | allow_if_mapping_rule_confirmed | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | pattern `1010 / 2010 / 5010`; не заменяет applied расчет |

### Decision-rule для графы 47

1. `payment_pattern_rule` — это только pattern-layer.
2. Финальный источник для `xml_import.md` — только applied calculation текущего кейса.
3. Если pattern есть, а расчет не построен:
   - в `ui_input.md` можно показывать только предполагаемый состав строк;
   - в `xml_import.md` нельзя выдавать финальные суммы / базы / ставки как готовые.

---

# Раздел 7. Presentation / composed fields

## Эти поля не должны быть каноническим источником, но могут собираться для UI/XML

| fact_path                                                                        | semantic_source_layer         | meaning                           | xml_target            | ui_target                 | transform | multiplicity       | required_for | allowed_source_class      | projection_rule    | reference_presence | new_dt_policy  | mapping_status        | notes          |
| -------------------------------------------------------------------------------- | ----------------------------- | --------------------------------- | --------------------- | ------------------------- | --------- | ------------------ | ------------ | ------------------------- | ------------------ | ------------------ | -------------- | --------------------- | -------------- |
| alta_master_data_requirements.consignee_profile.same_as_graph14_mode             | alta_master_data_requirements | Режим представления графы 8       | G_8.NAME              | 8 Печатное представление  | compose   | single             | xml          | operator,alta_master_data | manual_review_only | seen_in_both       | reference_only | verified_by_reference | не бизнес-факт |
| alta_master_data_requirements.financial_responsible_profile.same_as_graph14_mode | alta_master_data_requirements | Режим представления графы 9       | G_9.NAME              | 9 Печатное представление  | compose   | single             | xml          | operator,alta_master_data | manual_review_only | seen_in_both       | reference_only | verified_by_reference |                |
| alta_master_data_requirements.declarant_profile.name                             | alta_master_data_requirements | Развернутая строка декларанта     | G_14.NAME             | 14 Печатное представление | compose   | single             | xml          | operator,alta_master_data | manual_review_only | seen_in_both       | reference_only | verified_by_reference |                |
| fact_composition.representative_printed_block                                    | fact_composition              | Печатная строка представителя     | G_54P                 | 54 Печатная форма         | compose   | single             | xml          | composed                  | manual_review_only | seen_in_both       | reference_only | verified_by_reference |                |
| fact_composition.graph44_doc_text                                                | fact_composition              | Сводная строка документа графы 44 | BLOCK[].G44[].DOCTEXT | 44 Итоговая строка        | compose   | repeat_documents44 | xml          | composed                  | manual_review_only | seen_in_both       | reference_only | verified_by_reference |                |
| fact_composition.warehouse_printed_address                                       | fact_composition              | Печатная строка местонахождения   | G_30P_1               | 30 Печатная форма         | compose   | single             | xml          | composed                  | manual_review_only | seen_in_both       | reference_only | verified_by_reference |                |
| fact_composition.payment_summary_line                                            | fact_composition              | Итоговая строка платежей          | B_* / B_7             | Блок итогов платежей      | compose   | single             | ui           | composed                  | manual_review_only | seen_in_both       | reference_only | verified_by_reference |                |

---

# Раздел 8. Reference-only и system-only

## Эти поля не являются источником для новой ДТ

| fact_path                                            | semantic_source_layer | meaning                                              | xml_target                                                                              | ui_target | transform      | multiplicity | required_for | allowed_source_class | projection_rule                         | reference_presence | new_dt_policy           | mapping_status           | notes                                          |
| ---------------------------------------------------- | --------------------- | ---------------------------------------------------- | --------------------------------------------------------------------------------------- | --------- | -------------- | ------------ | ------------ | -------------------- | --------------------------------------- | ------------------ | ----------------------- | ------------------------ | ---------------------------------------------- |
| reference_observed.Reference[].value                 | reference_observed    | Поля, найденные только в reference                   | varies                                                                                  | varies    | direct         | varies       | optional     | sample               | never_from_sample_only                  | seen_in_both       | reference_only          | inferred                 | использовать только как подсказку структуры    |
| reference_observed.Reference[].value_as_mapping_hint | reference_observed    | Поля reference, используемые только как mapping hint | varies                                                                                  | varies    | mapping_lookup | varies       | mapping_only | sample,operator      | allow_if_reference_used_as_mapping_hint | seen_in_both       | mapping_rule_for_new_dt | verified_by_case_pattern | не факт поставки, а правило/константа проекции |
| system_only.SystemField[].value                      | system_only           | Системные / служебные поля                           | ED_ID / ED_STAT / BACK / FACE / REGNUM / PARENT_* / CREATEDATE / FileName / user / time | —         | direct         | varies       | optional     | system               | system_only                             | seen_in_xml        | do_not_transfer         | verified_by_xml          | не вход новой ДТ                               |

---

## Поля с обязательным ручным контролем

`manual_review_only` означает:

- безусловная автоподстановка запрещена;
- совпадение с reference недостаточно;
- поле допустимо только как материал для проверки.

---

## Правила использования mapping

1. Сначала формируется `facts.md`, затем — `review.md`, `ui_input.md`, `xml_import.md`.
2. `reference_observed` не является источником business/master/calculated data новой ДТ.
3. `system_only` никогда не переносится в новую ДТ.
4. `alta_master_data_requirements` допустим только как master data, а не как shipment facts.
5. `calculated_requirements` допустим только после расчета.
6. `fact_composition` хранит составные представления из подтвержденных данных и не подменяет source facts.
7. `mapping_rules` не являются shipment facts, но могут быть обязательны для UI/XML.
8. `needs_validation` и `manual_review_only` запрещают безусловную production-автоподстановку.

---

## Практический контрольный список

Хорошая карта проекции позволяет быстро ответить:

1. Это нужно для новой ДТ?
2. Это shipment fact, master data, calculation, composition или mapping rule?
3. Это можно копировать напрямую или нужно собирать / рассчитывать?
4. Это значение production-ready или только candidate / case-pattern / needs_validation?
5. Это можно переносить или только использовать как hint