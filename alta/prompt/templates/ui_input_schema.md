# ui_input_schema

## Назначение

`ui_input.md` — это **операционная карта ручного ввода, проверки и подтверждения process / mapping rules в интерфейсе Альты**.
Это не:

- отчет по кейсу;
- XML-слой;
- эталонная выгрузка;
- место для системных полей;
- место для хранения “всего, что попалось в reference”.
  `ui_input.md` должен отвечать на вопросы:
1. Что нужно реально ввести в форму Альты?
2. Что нужно добрать из карточек / master data Альты?
3. Что должно появиться после расчета и подлежит проверке, а не ручному вводу?
4. Что видно в эталоне, но вводить как факт новой поставки нельзя?
5. Что должно быть подтверждено как **process / mapping rule**, а не как shipment fact?

---

## Ключевое разделение

Все поля UI должны быть разделены по происхождению:

- `shipment_input` — вводится по документам поставки;
- `master_data_input` — берется из карточек / профиля / справочников Альты;
- `calculated_verify` — появляется после расчета и подлежит проверке;
- `mapping_verify` — не является shipment fact, но должно быть подтверждено как process / mapping rule;
- `reference_only` — видно в эталоне, но не вводится автоматически;
- `do_not_input` — системное / служебное / не является ручным вводом.

---

## Статусы значений

Использовать только:

- `input`
- `verify`
- `mapping_verify`
- `conditional`
- `pending`
- `master_data_required`
- `calculation_required`
- `sample_only`
- `do_not_input`
  
  ### Пояснение по статусам
- `input` — поле реально вводится руками.
- `verify` — поле нужно сверить, но не обязательно вводить вручную с нуля.
- `mapping_verify` — поле должно быть подтверждено как process / mapping rule.
- `master_data_required` — поле должно быть добрано из карточек / профилей.
- `calculation_required` — поле должно появиться после расчета.
- `sample_only` — видно только в эталоне.
- `do_not_input` — нельзя вводить руками как рабочее поле новой ДТ.
  
  ### Практическое правило выбора статуса
- `input` — если оператор должен реально ввести значение руками как данные новой поставки;
- `verify` — если значение уже может быть подставлено / собрано, и оператор должен его только проверить;
- `mapping_verify` — если значение не является фактом новой поставки, а должно быть подтверждено как process / projection rule.
  Нельзя использовать `input` для process-констант только потому, что они визуально находятся в UI.

---

## Правила включения полей

В `ui_input.md` должны попадать только поля, которые реально нужны оператору в одном из режимов:

1. **ввести**;
2. **добрать из карточки / справочника**;
3. **проверить после автоподстановки / расчета**;
4. **подтвердить как mapping / process rule**.
   Запрещено включать как обычные поля ручного ввода:
- регистрационные номера старой ДТ;
- ED_ID / ED_STAT / GUID;
- системные XML-поля;
- служебные флаги выгрузки;
- поля истории документа в Альте;
- печатные представления, если оператор их не вводит руками;
- reference-only значения без подтверждения для новой ДТ;
- process-константы как будто это shipment facts.

---

## Meta

- case_name: <название кейса>
- source_facts: facts.md
- source_mapping: projection_mapping.md
- ready_for_manual_input: <yes / partial / no>
- manual_input_blockers:
  - <блокер 1>
  - <блокер 2>

---

# Раздел 1. Shipment input — ввод по документам поставки

## 1 Декларация

### Важно по графе 1

Поля графы 1 находятся в UI и визуально похожи на обычный ввод, но в большинстве кейсов они
являются не shipment facts, а process / mapping rules.
Поэтому:

- не заполнять их вслепую как `shipment_input`;
- не переносить из эталона как факт;
- подтверждать через `mapping_verify`, если правило уже стабилизировано.
- 1 Декларация — тип:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: <mapping_verify / reference_only>
  - source: <mapping rule / operator / reference>
  - note: это не shipment fact; поле подтверждается как process-rule
- 1 Декларация — процедура:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: <mapping_verify / reference_only>
  - source: <mapping rule / operator / reference>
- 1 Декларация — ЭД:
  - value: <значение>
  - status: <mapping_verify / conditional / verify / pending>
  - source_layer: <mapping_verify / calculated_verify>
  - source: <process rule / operator / system>
- 3 Формы — 1:
  - value: <значение>
  - status: <mapping_verify / verify / conditional / pending>
  - source_layer: <mapping_verify / calculated_verify>
  - source: <calculation / rule>
- 3 Формы — 2:
  - value: <значение>
  - status: <mapping_verify / verify / conditional / pending>
  - source_layer: <mapping_verify / calculated_verify>
  - source: <calculation / rule>
- 5 Всего товаров:
  - value: <значение>
  - status: <verify / input>
  - source_layer: shipment_input
  - source: <derived>
- 6 Всего мест:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <источник>

---

## 2 Отправитель

- 2 Отправитель — наименование:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 2 Отправитель — Страна:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 2 Отправитель — Почтовый код:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 2 Отправитель — Область, район:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
  - note: проверить корректность разложения адреса по полям Альты
- 2 Отправитель — Населенный пункт:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
  - note: проверить корректность разложения адреса по полям Альты
- 2 Отправитель — Улица / адресная строка:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
  - note: проверить, нужно ли делить адрес на части
- 2 Отправитель — Дом:
  - value: <значение>
  - status: <conditional / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 2 Отправитель — Офис:
  - value: <значение>
  - status: <conditional / pending>
  - source_layer: shipment_input
  - source: <источник>
- 2 Отправитель — Телефон:
  - value: <значение>
  - status: <conditional / pending>
  - source_layer: shipment_input
  - source: <источник>
- 2 Отправитель — E-mail:
  - value: <значение>
  - status: <conditional / pending>
  - source_layer: shipment_input
  - source: <источник>

---

## 11 / 15 / 16 / 17 / 20 / 22 / 23

- 11 Торгующая страна:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 15 Страна отправления (код):
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 15 Страна отправления (наименование):
  - value: <значение>
  - status: <verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 16 Страна происхождения (код):
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 16 Страна происхождения (наименование):
  - value: <значение>
  - status: <verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 17 Страна назначения (код):
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 17 Страна назначения (наименование):
  - value: <значение>
  - status: <verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 20 Условия поставки (код):
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 20 Условия поставки (место):
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 22 Валюта — цифровой код:
  - value: <значение>
  - status: <verify / pending>
  - source_layer: shipment_input
  - source: <derived / document>
- 22 Валюта — буквенный код:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 22 Общая сумма по счету:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 23 Курс валюты:
  - value: <значение>
  - status: <calculation_required / verify / pending>
  - source_layer: calculated_verify
  - source: <system / calculation>
- 23 Курс вал. для расч.:
  - value: <значение>
  - status: <calculation_required / verify / pending>
  - source_layer: calculated_verify
  - source: <system / calculation>

---

## 18 / 19 / 21 / 25 / 26 / 29 / 30

### Важно по графе 30

Для графы 30 нельзя автоматически считать, что:

- номер документа СВХ = итоговое поле "Номер документа";
- дата документа СВХ = итоговое поле "Дата";
- факт хранения = код типа / код вида документа.
  Графа 30 требует одновременно:
1. shipment-level подтверждения:
   - факт хранения,
   - документ СВХ,
   - адрес;
2. mapping-level подтверждения:
   - тип,
   - вид документа,
   - итоговый номер,
   - итоговая дата,
   - код таможни.
- 18 Вид / режим поля:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: <mapping_verify / reference_only>
  - source: <mapping rule / reference>
- 18 Идентификация тр. ср-ва:
  - value: <значение>
  - status: <conditional / verify / pending / sample_only>
  - source_layer: <shipment_input / calculated_verify / reference_only>
  - source: <источник>
- 18 Страна регистрации / код:
  - value: <значение>
  - status: <mapping_verify / verify / pending / sample_only>
  - source_layer: <mapping_verify / calculated_verify / reference_only>
  - source: <источник>
  - note: если это process-код, не вводить как буквенный код страны
- 19 Конт.:
  - value: <значение>
  - status: <mapping_verify / verify / pending / sample_only>
  - source_layer: <mapping_verify / calculated_verify / reference_only>
  - source: <источник>
- 21 Вид / режим поля:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: <mapping_verify / reference_only>
  - source: <mapping rule / reference>
- 21 Идентификация активного тр. ср-ва на границе:
  - value: <значение>
  - status: <conditional / verify / pending>
  - source_layer: calculated_verify
  - source: <operator / rule>
- 25 Код вида транспорта на границе:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: <shipment_input / calculated_verify>
  - source: <operator / rule>
- 26 Код вида транспорта при отправлении:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: <shipment_input / calculated_verify>
  - source: <operator / rule>
- 29 Код таможни:
  - value: <значение>
  - status: <mapping_verify / input / verify / pending>
  - source_layer: <mapping_verify / calculated_verify>
  - source: <operator / document / rule>
- 29 Наименование таможни:
  - value: <значение>
  - status: <verify / pending>
  - source_layer: calculated_verify
  - source: <system / directory / rule>
- 30 Тип:
  - value: <значение>
  - status: <mapping_verify / input / pending>
  - source_layer: <mapping_verify / shipment_input>
  - source: <mapping rule / документы СВХ>
  - note: факт хранения и код поля — не одно и то же
- 30 Вид / 1/2/3:
  - value: <значение>
  - status: <mapping_verify / pending>
  - source_layer: mapping_verify
  - source: <mapping rule / reference / operator>
- 30 Номер документа:
  - value: <значение>
  - status: <mapping_verify / input / verify / pending>
  - source_layer: <mapping_verify / shipment_input>
  - source: <documents СВХ / registry rule>
  - note: итоговое поле может быть не равно номеру документа СВХ
- 30 Дата:
  - value: <значение>
  - status: <mapping_verify / input / verify / pending>
  - source_layer: <mapping_verify / shipment_input>
  - source: <documents СВХ / registry rule>
  - note: итоговое поле может быть не равно дате документа СВХ
- 30 Код таможни:
  - value: <значение>
  - status: <mapping_verify / verify / pending>
  - source_layer: mapping_verify
  - source: <documents СВХ / operator / reference>
- 30 Страна:
  - value: <значение>
  - status: <conditional / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 30 Область, район:
  - value: <значение>
  - status: <conditional / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 30 Населенный пункт:
  - value: <значение>
  - status: <conditional / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 30 Улица / адрес:
  - value: <значение>
  - status: <conditional / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 30 Печатная форма:
  - value: <значение>
  - status: <verify / do_not_input / sample_only>
  - source_layer: calculated_verify
  - source: <compose from graph 30>
  - note: оператор обычно не должен вручную вводить готовую печатную строку целиком

---

## 31 Грузовые места и описание товаров

### Товар 1

- 31 Основное описание:
  - value: <текст>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <собрано из shipment facts>
- 31 Дополнительные строки:
  - value:
  - <строка 1>
  - <строка 2>
  - <строка 3>
  - status: <verify / pending>
  - source_layer: shipment_input
  - source: <derived from shipment facts>
- 31 Количество мест:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 31 Маркировка / упаковка:
  - value: <значение>
  - status: <input / verify / pending / sample_only>
  - source_layer: <shipment_input / reference_only>
  - source: <источник>
  - note: нельзя переносить из эталона без подтверждения первичкой
    
    #### 31 Табличная часть
- 31 Описание группы:
  - value: <значение>
  - status: <verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 31 Производитель:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 31 Товарный знак:
  - value: <значение>
  - status: <conditional / verify / mapping_verify / pending>
  - source_layer: <shipment_input / mapping_verify>
  - source: <источник / operator / reference>
  - note: если подтверждено представление "ОТСУТСТВУЕТ", это mapping/presentation rule
- 31 Марка:
  - value: <значение>
  - status: <conditional / verify / mapping_verify / pending>
  - source_layer: <shipment_input / mapping_verify>
  - source: <источник / operator / reference>
- 31 Модель:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 31 Количество:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 31 Единица измерения:
  - value: <значение>
  - status: <verify / pending>
  - source_layer: shipment_input
  - source: <derived / document>
- 31 Код единицы измерения:
  - value: <значение>
  - status: <verify / pending>
  - source_layer: shipment_input
  - source: <derived / document>
- 31 Фактурная стоимость:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>

---

## 32 / 33 / 34 / 35 / 36 / 37 / 38 / 42 / 45 / 46

### Товар 1

- 32 Товар:
  - value: <значение>
  - status: <verify / input>
  - source_layer: shipment_input
  - source: <derived>
- 33 ТН ВЭД:
  - value: <значение>
  - status: <input / pending / sample_only>
  - source_layer: <shipment_input / reference_only>
  - source: <источник>
- 34 Страна происхождения:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 35 Вес брутто:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 36 Преференция:
  - value: <значение>
  - status: <mapping_verify / calculation_required / pending>
  - source_layer: <mapping_verify / calculated_verify>
  - source: <rule / calculation / reference>
- 37 Процедура:
  - value: <значение>
  - status: <mapping_verify / verify / pending>
  - source_layer: <mapping_verify / calculated_verify>
  - source: <operator / rule / reference>
- 38 Вес нетто:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 42 Цена товара / фактурная стоимость:
  - value: <значение>
  - status: <input / pending>
  - source_layer: shipment_input
  - source: <источник>
- 42 Признак "В ДТС":
  - value: <значение>
  - status: <mapping_verify / verify / calculation_required / pending>
  - source_layer: <mapping_verify / calculated_verify>
  - source: <operator / rule / valuation / reference>
- 45 Таможенная стоимость:
  - value: <значение>
  - status: <calculation_required / verify / pending>
  - source_layer: calculated_verify
  - source: <valuation>
- 46 Статистическая стоимость:
  - value: <значение>
  - status: <calculation_required / verify / pending>
  - source_layer: calculated_verify
  - source: <valuation>

---

## 44 Дополнительная информация / представляемые документы

### Важно по графе 44

Для графы 44 нужно различать:

1. business-реквизиты документа:
   - номер,
   - дата,
   - наименование,
   - срок действия;
2. mapping-реквизиты:
   - код документа,
   - подкод / доп. реквизит,
   - binary-copy patterns,
   - правило включения документа в итоговый набор.
     Нельзя считать, что эталонный набор graph 44 можно перенести в новую ДТ без отбора и mapping-логики.
     
     ### Документ 1
- 44 Код документа:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: <mapping_verify / reference_only>
  - source: <mapping rule / reference>
- 44 Доп. реквизит / подкод:
  - value: <значение>
  - status: <mapping_verify / conditional / pending / sample_only>
  - source_layer: <mapping_verify / reference_only>
  - source: <mapping rule / reference>
- 44 Номер:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 44 Дата:
  - value: <значение>
  - status: <input / verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 44 Наименование:
  - value: <значение>
  - status: <verify / pending>
  - source_layer: shipment_input
  - source: <источник>
- 44 Срок действия с:
  - value: <значение>
  - status: <conditional / verify / pending>
  - source_layer: <shipment_input / master_data_input>
  - source: <источник>
- 44 Срок действия по:
  - value: <значение>
  - status: <conditional / verify / pending>
  - source_layer: <shipment_input / master_data_input>
  - source: <источник>
- 44 Итоговая строка:
  - value: <значение>
  - status: <verify / do_not_input / pending>
  - source_layer: calculated_verify
  - source: <compose from graph44 fields>
  - note: это presentation-field, а не отдельный ручной ввод

---

# Раздел 2. Master data input — добрать из карточек / профилей Альты

## 8 Получатель — данные из карточки, если не подтверждены первичкой

- 8 Получатель — ОГРН / рег. идентификатор:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка контрагента / operator>
- 8 Получатель — Телефон:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка контрагента / operator>
- 8 Получатель — E-mail:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка контрагента / operator>
- 8 Получатель — признак "см. графу 14":
  - value: <yes / no>
  - status: <master_data_required / mapping_verify / pending>
  - source_layer: <master_data_input / mapping_verify>
  - source: <настройка заполнения / operator / reference>
  - note: это режим представления, а не shipment fact

---

## 9 Лицо, ответственное за финансовое урегулирование

- 9 — наименование:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — ИНН/КПП:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — ОГРН / рег. идентификатор:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — Страна:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — Почтовый код:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — Область, район:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — Населенный пункт:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — Улица:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — Дом:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — Офис:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — Телефон:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — E-mail:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / operator>
- 9 — признак "см. графу 14":
  - value: <yes / no>
  - status: <master_data_required / mapping_verify / pending>
  - source_layer: <master_data_input / mapping_verify>
  - source: <настройка / operator / reference>

---

## 14 Декларант

- 14 Декларант — наименование:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — ИНН/КПП:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — ОГРН / рег. идентификатор:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — Страна:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — Почтовый код:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — Область, район:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — Населенный пункт:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — Улица:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — Дом:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — Офис:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — Телефон:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>
- 14 Декларант — E-mail:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка декларанта / operator>

---

## 54 Представитель

- 54 Фамилия:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка представителя / operator>
- 54 Имя:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка представителя / operator>
- 54 Отчество:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка представителя / operator>
- 54 Телефон:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка представителя / operator>
- 54 E-mail:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка представителя / operator>
- 54 Статус / роль:
  - value: <значение>
  - status: <master_data_required / pending>
  - source_layer: master_data_input
  - source: <карточка представителя / operator>
- 54 Код представителя:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка представителя / operator>
- 54 Документ личности — вид:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / документ / operator>
- 54 Документ личности — серия:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / документ / operator>
- 54 Документ личности — номер:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / документ / operator>
- 54 Документ личности — дата:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / документ / operator>
- 54 Документ личности — кем выдан:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / документ / operator>
- 54 Документ полномочий — наименование:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / документ / operator>
- 54 Документ полномочий — номер:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / документ / operator>
- 54 Документ полномочий — от:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / документ / operator>
- 54 Документ полномочий — до:
  - value: <значение>
  - status: <master_data_required / conditional / pending>
  - source_layer: master_data_input
  - source: <карточка / документ / operator>
- 54 Печатная форма:
  - value: <значение>
  - status: <verify / do_not_input / pending>
  - source_layer: calculated_verify
  - source: <compose from representative fields>
  - note: печатное представление не вводится как отдельный факт

---

# Раздел 3. Calculated verify — поля, которые не вводятся вслепую, а проверяются после расчета

## Стоимость и платежи

### Важно: calculation result vs process rule

В этом разделе могут встречаться два типа значений:

1. собственно результаты расчета:
   - курс,
   - стоимость,
   - база,
   - сумма;
2. process / valuation rules, которые проявляются в расчетном блоке UI:
   - признак "В ДТС",
   - pattern строк графы 47,
   - отдельные признаки режимов.
     Если значение относится ко второму типу, его нужно также отражать в `mapping_verify`.
- Расходы до границы:
  - value: <значение>
  - status: <verify / calculation_required / pending>
  - source_layer: calculated_verify
  - source: <transport invoices / valuation>
- Страхование:
  - value: <значение / not_applicable>
  - status: <verify / calculation_required / pending>
  - source_layer: calculated_verify
  - source: <transport docs / operator / valuation>
- 12 Общая таможенная стоимость:
  - value: <значение>
  - status: <verify / calculation_required / pending>
  - source_layer: calculated_verify
  - source: <valuation>
- 42 Признак "В ДТС":
  - value: <значение>
  - status: <verify / mapping_verify / calculation_required / pending>
  - source_layer: <calculated_verify / mapping_verify>
  - source: <rule / valuation / reference>
- 45 Таможенная стоимость:
  - value: <значение>
  - status: <verify / calculation_required / pending>
  - source_layer: calculated_verify
  - source: <valuation>
- 46 Статистическая стоимость:
  - value: <значение>
  - status: <verify / calculation_required / pending>
  - source_layer: calculated_verify
  - source: <valuation>
    
    ### 47 Исчисление платежей
    
    #### Платеж 1
- Вид:
  - value: <значение>
  - status: <verify / calculation_required / mapping_verify / pending>
  - source_layer: <calculated_verify / mapping_verify>
  - source: <calculation / rule / reference>
- Основа начисления:
  - value: <значение>
  - status: <verify / calculation_required / pending>
  - source_layer: calculated_verify
  - source: <calculation>
- Ставка:
  - value: <значение>
  - status: <verify / calculation_required / mapping_verify / pending>
  - source_layer: <calculated_verify / mapping_verify>
  - source: <calculation / rule / reference>
- Сумма:
  - value: <значение>
  - status: <verify / calculation_required / pending>
  - source_layer: calculated_verify
  - source: <calculation>
- СП:
  - value: <значение>
  - status: <verify / calculation_required / mapping_verify / pending>
  - source_layer: <calculated_verify / mapping_verify>
  - source: <calculation / operator / reference>
    
    #### Итог по платежам
- В платежи / итог:
  - value: <значение>
  - status: <verify / do_not_input / pending>
  - source_layer: calculated_verify
  - source: <compose from payments>
  - note: итоговое представление, а не отдельный ручной ввод

---

# Раздел 4. Mapping verify — поля и константы, которые нужно подтверждать как process / projection rules

## Назначение

Этот раздел нужен для полей, которые:

- не должны вводиться как shipment facts;
- но должны быть подтверждены как часть логики заполнения Альты;
- могут быть выявлены из:
  - эталонного XML,
  - эталонного UI,
  - diff generated vs reference,
  - операторских правил.
    
    ### Важно: сначала candidate / business layer, потом mapping verification
    
    `mapping_verify` не заменяет:
- shipment_input,
- master_data_input,
- calculated_verify.
  Сначала должны быть собраны:
- факты поставки,
- master data,
- расчетные кандидаты,
- document candidates,
  и только потом подтверждаются process / mapping rules.
  
  ## Declaration / process rules
- Graph 1 — declaration kind:
  - value: <например ИМ>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
- Graph 1 — declaration procedure:
  - value: <например 40>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
- Graph 1 — electronic declaration:
  - value: <например ЭД>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
- Graph 3 — forms main:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
- Graph 3 — forms additional:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
    
    ## Transport / graph 18 / 19 / 21 rules
- Graph 18 — kind rule:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
- Graph 18 — registration country code rule:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
- Graph 19 — container flag rule:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
- Graph 21 — kind rule:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
    
    ## Graph 30 rules
- Graph 30 — type code rule:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
- Graph 30 — document kind code rule:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
- Graph 30 — registry number rule:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
  - note: может отличаться от номера документа СВХ
- Graph 30 — registry date rule:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
  - note: может отличаться от даты документа СВХ
- Graph 30 — customs code rule:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / operator>
    
    ## Graph 44 rules
- Graph44 — role to code mapping:
  - value: <например contract -> 03011, invoice -> 04021>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / diff analysis>
  - note: это rule, а не shipment fact
- Graph44 — role to subcode mapping:
  - value: <например G4403 / G441A patterns>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference xml / screenshot / diff analysis>
    
    ## Representation rules
- Graph 8 — same as graph 14:
  - value: <yes / no>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference ui / xml / operator>
- Graph 9 — same as graph 14:
  - value: <yes / no>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference ui / xml / operator>
- Graph 42 — value in DTS:
  - value: <значение>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference ui / xml / operator>
- Graph 31 — absent trademark presentation:
  - value: <например ОТСУТСТВУЕТ>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference ui / operator>
- Graph 31 — absent brand presentation:
  - value: <например ОТСУТСТВУЕТ>
  - status: <mapping_verify / pending / sample_only>
  - source_layer: mapping_verify
  - source: <reference ui / operator>

---

# Раздел 5. Reference-only — наблюдается в эталоне, но не вводится автоматически

### Важно

Наличие значения в эталонном UI не делает его:

- ручным вводом новой поставки;
- shipment fact;
- master data текущего кейса.
  Reference используется только:
- для проверки completeness;
- для поиска mapping rules;
- для diff analysis.
- направление и процедура, если известны только из эталона и еще не переведены в mapping_verify;
- идентификация ТС, если известна только из эталона;
- страна регистрации ТС, если известна только из эталона;
- reference-набор документов графы 44;
- reference-значения граф 29 / 30 / 54, если они не подтверждены как rules или master data;
- любые поля со статусом `sample_only`.

---

# Раздел 6. Do not input — не вводить вручную

### Служебные и системные поля

- регистрационный номер ДТ;
- внутренний / временный номер старой ДТ;
- решения таможни;
- даты и время выпуска;
- ЛНП;
- GUID;
- ED_ID / ED_STAT / ED_TYP;
- FileName / time / user / PARENT_* / CREATEDATE;
- BACK / FACE;
- системные строки выгрузки;
- печатные блоки, если они составляются автоматически;
- любые поля со статусом `do_not_input`.

---

## Раздел 7. Итоговая готовность к ручному вводу

- ready_for_manual_input:
  - value: <yes / partial / no>
- ready_for_mapping_verification_session:
  - value: <yes / partial / no>
  - note: достаточно ли данных, чтобы не вводить, а именно сверять process / mapping rules по UI
- blockers:
  - <блокер 1>
  - <блокер 2>
  - <блокер 3>
- missing_master_data:
  - <поле 1>
  - <поле 2>
- missing_calculations:
  - <поле 1>
  - <поле 2>
- missing_mapping_rules:
  - <поле 1>
  - <поле 2>
- fields_requiring_manual_verification:
  - <поле 1>
  - <поле 2>
  - <поле 3>

---

## Практический принцип

Хороший `ui_input.md` должен сразу показывать оператору:

1. Что вводим как факты новой поставки;
2. Что добираем из карточек / профилей Альты;
3. Что появится только после расчета и требует проверки;
4. Что подтверждаем как process / mapping rules;
5. Что видно в эталоне, но не должно вводиться руками как факт новой поставки.
   Если документ этого не показывает — он недостаточно полезен для реального ручного ввода.
