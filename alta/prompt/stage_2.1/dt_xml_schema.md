# dt_xml_schema.md — Stage 2.1 (DT XML)

## Назначение
Этот документ задаёт правила генерации **одного XML-файла ДТ** (декларации на товары) для импорта в Альту.

Вход этапа 2.1:
- `alta\\stage_2.0_result\\<case>\\dt_fields.yaml`

Выход этапа 2.1:
- `alta\\stage_2.1_result\\<case>\\dt.xml` (или `<case>_dt.xml`) в кодировке `windows-1251`.

## 1. Базовые принципы преобразования

1. **Источник данных:** значения берутся из `dt_fields.yaml` (stage 2.0).
2. **Whitelist-теги:** генератор выводит **только** теги и структуры, перечисленные в разделе «2) Структура и маппинг». 
   Никаких дополнительных системных тегов.
3. **Кодировка:** XML генерируется строго в `windows-1251`. Декларация:
   `<?xml version="1.0" encoding="windows-1251"?>`.
4. **Экранирование:** все текстовые значения должны быть XML-экранированы (`&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`).
5. **Даты:** писать в формате, ожидаемом Альтой для соответствующего тега:
   - для большинства дат: `YYYY-MM-DD` (пример: `2026-02-05`)
   - если в конкретном поле используются короткие форматы (как в некоторых печатных строках) — это явно оговаривается 
     в `dt_fields.yaml` и переносится как есть.
6. **Числа:** числовые поля писать как строковое представление числа **без принудительного округления** 
   (как в `doc_xml_schema.md`). Если в `dt_fields.yaml` дано `97260.00` — так и писать.

## 2. Структура файла dt.xml

### 2.1. Корневой элемент

Корень:
- `<AltaGTD> ... </AltaGTD>`

Допускается наличие атрибутов у корня (например `time`, `Version`, `EDVer`), но **в этой схеме мы их не генерируем**, 
если они не перечислены явно в whitelist.

### 2.2. Скалярные теги верхнего уровня (заголовок ДТ)

Правило: каждое поле `dt_fields.<path>` маппится в одноимённый XML-тег графы (G-код) по таблице ниже.

#### 2.2.1. Графа 1
| XML тег  | dt_fields path                | Комментарий  |
|----------|-------------------------------|--------------|
| `G_1_1`  | `declaration.direction.value` | Направление  |
| `G_1_2`  | `declaration.procedure.value` | Процедура    |
| `G_1_31` | `declaration.form.value`      | Форма подачи |

#### 2.2.2. Графа 2 (Отправитель)
| XML тег   | dt_fields path              | Комментарий       |
|-----------|-----------------------------|-------------------|
| `G_2_50`  | `sender.country_name.value` | страна (текст)    |
| `G_2_7`   | `sender.country_code.value` | alpha-2           |
| `G_2_NAM` | `sender.name.value`         | наименование      |
| `G_2_SUB` | `sender.region.value`       | регион/подраздел  |
| `G_2_CIT` | `sender.city.value`         | город             |
| `G_2_STR` | `sender.street.value`       | улица/дом строкой |

Примечание: блок `G_2` (вложенный `<G_2><NAME>...`) не генерируем, если он не задан явно в dt_fields.

#### 2.2.3. Графы 5–6
| XML тег | dt_fields path                      | Комментарий            |
|---------|-------------------------------------|------------------------|
| `G_5_1` | `shipment.total_goods_number.value` | число товарных позиций |
| `G_6_0` | `shipment.packages_flag.value`      | признак подсчёта мест  |
| `G_6_1` | `shipment.total_packages.value`     | общее количество мест  |

#### 2.2.4. Графа 8 (Получатель)
| XML тег     | dt_fields path                      |
|-------------|-------------------------------------|
| `G_8_1`     | `consignee.ogrn.value`              |
| `G_8_50`    | `consignee.country_name.value`      |
| `G_8_6`     | `consignee.inn_kpp.value`           |
| `G_8_7`     | `consignee.country_code.value`      |
| `G_8_NAM`   | `consignee.name.value`              |
| `G_8_POS`   | `consignee.postcode.value`          |
| `G_8_SUB`   | `consignee.region.value`            |
| `G_8_CIT`   | `consignee.city.value`              |
| `G_8_STR`   | `consignee.street.value`            |
| `G_8_BLD`   | `consignee.building.value`          |
| `G_8_ROM`   | `consignee.room.value`              |
| `G_8_SM14`  | `consignee.same_as_declarant.value` |
| `G_8_PHONE` | `consignee.phone.value`             |
| `G_8_EMAIL` | `consignee.email.value`             |

Блок `G_8/NAME`:
- генерировать узел:
  ```xml
  <G_8><NAME>...</NAME></G_8>
  ```
  где значение берётся из `consignee.name_display.value`.

#### 2.2.5. Графа 9 (Финансовое урегулирование)
| XML тег     | dt_fields path                      |
|-------------|-------------------------------------|
| `G_9_1`     | `financial.ogrn.value`              |
| `G_9_4`     | `financial.inn_kpp.value`           |
| `G_9_NAM`   | `financial.name.value`              |
| `G_9_CC`    | `financial.country_code.value`      |
| `G_9_CN`    | `financial.country_name.value`      |
| `G_9_POS`   | `financial.postcode.value`          |
| `G_9_SUB`   | `financial.region.value`            |
| `G_9_CIT`   | `financial.city.value`              |
| `G_9_STR`   | `financial.street.value`            |
| `G_9_BLD`   | `financial.building.value`          |
| `G_9_ROM`   | `financial.room.value`              |
| `G_9_SM14`  | `financial.same_as_declarant.value` |
| `G_9_7`     | `financial.country_code_alt.value`  |
| `G_9_PHONE` | `financial.phone.value`             |
| `G_9_EMAIL` | `financial.email.value`             |

Блок `G_9/NAME`:
- генерировать узел:
  ```xml
  <G_9><NAME>...</NAME></G_9>
  ```
  где значение берётся из `financial.name_display.value`.

#### 2.2.6. Графа 11
| XML тег  | dt_fields path                      |
|----------|-------------------------------------|
| `G_11_1` | `shipment.trade_country_code.value` |

#### 2.2.7. Графа 14 (Декларант)
| XML тег      | dt_fields path                 |
|--------------|--------------------------------|
| `G_14_1`     | `declarant.ogrn.value`         |
| `G_14_4`     | `declarant.inn_kpp.value`      |
| `G_14_NAM`   | `declarant.name.value`         |
| `G_14_CC`    | `declarant.country_code.value` |
| `G_14_CN`    | `declarant.country_name.value` |
| `G_14_POS`   | `declarant.postcode.value`     |
| `G_14_SUB`   | `declarant.region.value`       |
| `G_14_CIT`   | `declarant.city.value`         |
| `G_14_STR`   | `declarant.street.value`       |
| `G_14_BLD`   | `declarant.building.value`     |
| `G_14_ROM`   | `declarant.room.value`         |
| `G_14_PHONE` | `declarant.phone.value`        |
| `G_14_EMAIL` | `declarant.email.value`        |

Блок `G_14/NAME`:
- генерировать:
  ```xml
  <G_14><NAME>...</NAME></G_14>
  ```
  где значение берётся из `declarant.name_display.value`.

#### 2.2.8. Графы 15–17 (страны)
| XML тег   | dt_fields path                            |
|-----------|-------------------------------------------|
| `G_15_1`  | `shipment.dispatch_country_name.value`    |
| `G_15A_1` | `shipment.dispatch_country_code.value`    |
| `G_16_1`  | `shipment.origin_country_name.value`      |
| `G_16_2`  | `shipment.origin_country_code.value`      |
| `G_17_1`  | `shipment.destination_country_name.value` |
| `G_17A_1` | `shipment.destination_country_code.value` |

#### 2.2.9. Графа 18–21 (транспорт)
| XML тег  | dt_fields path                              |
|----------|---------------------------------------------|
| `G_18_0` | `transport.vehicles_count.value`        |
| `G_18`   | `transport.identification.value`            |
| `G_18_2` | `transport.registration_country_code.value` |
| `G_19_1` | `transport.container_flag.value`            |
| `G_21_0` | `transport.border_mode.value`               |

#### 2.2.10. Графа 20 (условия поставки)
| XML тег   | dt_fields path              |
|-----------|-----------------------------|
| `G_20_20` | `delivery.terms_code.value` |
| `G_20_21` | `delivery.place_name.value` |

#### 2.2.11. Графы 22–23 (валюта/курс)
| XML тег  | dt_fields path                            |
|----------|-------------------------------------------|
| `G_22_1` | `shipment.invoice_currency_numeric.value` |
| `G_22_2` | `shipment.invoice_amount.value`           |
| `G_22_3` | `shipment.invoice_currency_alpha.value`   |
| `G_23_1` | `shipment.currency_rate.value`            |
| `G_23_2` | `shipment.currency_rate.value`            |

#### 2.2.12. Графы 25–26
| XML тег  | dt_fields path                            |
|----------|-------------------------------------------|
| `G_25_1` | `transport.border_transport_code.value`   |
| `G_26_1` | `transport.internal_transport_code.value` |

#### 2.2.13. Графа 29 (таможня на границе)
| XML тег  | dt_fields path              |
|----------|-----------------------------|
| `G_29_1` | `customs.border_code.value` |
| `G_29_2` | `customs.border_name.value` |

#### 2.2.14. Графа 30 (местонахождение)
| XML тег     | dt_fields path                        |
|-------------|---------------------------------------|
| `G_30_0`    | `location.type.value`                 |
| `G_30_10`   | `location.document_kind.value`        |
| `G_30_1`    | `location.document_number.value`      |
| `G_30_DATE` | `location.document_date.value`        |
| `G_30_CC`   | `location.address.country_code.value` |
| `G_30_SUB`  | `location.address.region.value`       |
| `G_30_CIT`  | `location.address.city.value`         |
| `G_30_STR`  | `location.address.street.value`       |
| `G_30_12`   | `location.customs_code.value`         |
| `G_30P_1`   | `location.printed.value`              |

#### 2.2.15. Графа 42 (доп. признак)
| XML тег  | dt_fields path            |
|----------|---------------------------|
| `G_42_2` | `declaration.g42_2.value` |

#### 2.2.16. Графа 54 (уполномоченное лицо)
| XML тег      | dt_fields path                                 |
|--------------|------------------------------------------------|
| `G_54_20`    | `representative.date.value`                    |
| `G_54_21`    | `representative.phone.value`                   |
| `G_54_EMAIL` | `representative.email.value`                   |
| `G_54_3`     | `representative.last_name.value`               |
| `G_54_3NM`   | `representative.first_name.value`              |
| `G_54_3MD`   | `representative.middle_name.value`             |
| `G_54_4`     | `representative.authority_doc_name.value`      |
| `G_54_5`     | `representative.authority_doc_number.value`    |
| `G_54_60`    | `representative.authority_doc_date_from.value` |
| `G_54_61`    | `representative.authority_doc_date_to.value`   |
| `G_54_7`     | `representative.position.value`                |
| `G_54_8`     | `representative.passport_code.value`           |
| `G_54_9`     | `representative.passport_name.value`           |
| `G_54_100`   | `representative.passport_number.value`         |
| `G_54_101`   | `representative.passport_date.value`           |
| `G_54_12`    | `representative.passport_series.value`         |
| `G_54_13`    | `representative.passport_issuer.value`         |

Блок `G_54P`:
- генерировать:
  ```xml
  <G_54P><N2>...</N2></G_54P>
  ```
  где значение берётся из `representative.printed_block.value`.

### 2.3. Товары (повторяющиеся BLOCK)

Правило:
- каждый элемент массива `dt_fields.goods[]` → один `<BLOCK>...</BLOCK>`.

#### 2.3.1. Поля внутри BLOCK (по товару)

| XML тег  | dt_fields path (для goods[i]) |
|----------|-------------------------------|
| `G_32_1` | `item_no.value`               |
| `G_33_1` | `tnved_code.value`            |
| `G_33_4` | `tnved.flag_1.value`          |
| `G_33_5` | `tnved.flag_2.value`          |
| `G_34_1` | `origin_country_code.value`   |
| `G_35_1` | `gross_weight.value`          |
| `G_36_2` | `preference.value`            |
| `G_37_1` | `procedure_code.value`        |
| `G_38_1` | `net_weight.value`            |
| `G_42_1` | `invoice_cost.value`          |
| `G_43_1` | `mos_code_main.value`         |
| `G_43_2` | `mos_code_extra.value`        |
| `G_44`   | `g44_text.value`              |
| `G_45_0` | `customs_value.value`         |
| `G_45_1` | `customs_value.value`         |
| `G_46_1` | `statistical_value.value`     |

#### 2.3.2. Блок G_31 (описание товара)

Генерировать:
```xml
<G_31>
  <NAME Pref="1-">...</NAME>
  <FIRMA Pref="ПРОИЗВ.:">...</FIRMA>
  <TM Pref="(ТМ)">...</TM>
  <PL Pref="2-"/>
  <PLACE>...</PLACE>
</G_31>
```

Примечание: 
    `PLACE` — количество мест по товару (если `goods[i].places` отсутствует или pending — тег `<PLACE>` не генерировать).

Маппинг:
- `NAME` ← `goods[i].g31.name.value`
- `FIRMA` ← `goods[i].g31.manufacturer.value`
- `TM` ← `goods[i].g31.trademark.value`
- `PL` ← `goods[i].g31.pl.value`
- `PLACE` ← `goods[i].places.value`

#### 2.3.3. TXT (дополнение к графе 31)

Правило:
- каждый элемент `goods[i].txt[]` → отдельный узел:
  ```xml
  <TXT><TEXT>...</TEXT></TXT>
  ```

Маппинг:
- `TEXT` ← `goods[i].txt[j].text.value`

#### 2.3.4. TOVG (таблица описания)

Правило:
- каждый элемент `goods[i].tovg[]` → отдельный узел `<TOVG>...</TOVG>`.

Маппинг:

| XML тег (внутри TOVG) | dt_fields path (для goods[i].tovg[j]) |
|-----------------------|---------------------------------------|
| `G32G`                | `line_no.value`                       |
| `G31_1`               | `description.value`                   |
| `G31_11`              | `manufacturer.value`                  |
| `G31_12`              | `trade_mark.value`                    |
| `G31_14`              | `goods_mark.value`                    |
| `G31_15_MOD`          | `model.value`                         |
| `KOLVO`               | `quantity.value`                      |
| `CODE_EDI`            | `unit_code.value`                     |
| `NAME_EDI`            | `unit_name.value`                     |
| `G31_35`              | `gross_weight.value`                  |
| `G31_38`              | `net_weight.value`                    |
| `G31_42`              | `invoice_cost.value`                  |
| `INVOICCOST`          | `invoice_cost.value`                  |

#### 2.3.5. G_47 (платежи)

Правило:
- каждый элемент `goods[i].payments[k]` разворачивается в набор тегов вида:
  - `G_47_<n>_1` = `payment_code`
  - `G_47_<n>_2` = `tax_base`
  - `G_47_<n>_3` = `rate`
  - `G_47_<n>_4` = `amount`
  - `G_47_<n>_5` = `payment_method`

Где `<n>` — порядковый номер платежа по товару, начиная с 1.

## 3. Верификация (обязательная)

После генерации `dt.xml` AI обязан проверить:
1) Файл читается обратно как `windows-1251` без кракозябр.
2) XML well-formed: корректно закрыты все теги.
3) Количество `<BLOCK>` равно `len(dt_fields.goods)`.
4) Для каждого `goods[i]` количество `<TOVG>` равно `len(goods[i].tovg)` и количество `<TXT>` равно `len(goods[i].txt)`.

## 4. Выходные файлы

Сохранять результат в:
- `alta\\stage_2.1_result\\<case>\\dt.xml`

Запись выполнять командой Хобота `write_file` с кодировкой `windows-1251`.
