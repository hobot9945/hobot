## 1. Контекст и статус

- Кейс: МоскитнаяСетка
- Статус готовности этапа 2.0: Partial
- Готовность к этапу 2.1 (dt.xml): partial

Коротко:
- Всего полей в схеме: 235
- Pending полей: 11
- Полей с пометкой `птп`: 11

## 2. Сводка по заполнению (по разделам ДТ)

- 3.1 Заголовок: confirmed ✅
- 3.2 Отправитель: confirmed ✅
- 3.3 Количество товаров и мест: confirmed ✅
- 3.4 Получатель: confirmed ✅
- 3.5 Графа 9: confirmed ✅
- 3.6 Торгующая страна: confirmed ✅
- 3.7 Декларант: confirmed ✅
- 3.8 Страны: confirmed ✅
- 3.9 Условия поставки: confirmed ✅
- 3.10 Транспорт: confirmed ✅
- 3.11 Валюта и стоимость: confirmed ✅
- 3.12 Курс валюты: confirmed ✅
- 3.13 Вид транспорта: confirmed ✅
- 3.14 Таможня на границе: confirmed ✅
- 3.15 Местонахождение товаров: confirmed ✅
- 3.16 Товары: pending ⚠️ (8 полей pending, 6 птп)
- 3.17 После товаров: pending ⚠️ (2 поля pending: g42_2, representative.date; printed_block птп)

## 3. Pending (требуются данные / решения)

### 3.1 Нужно от оператора
- `goods[*].tnved.flag_1`: литеры после кода ТН ВЭД (G_33_4)
- `goods[*].tnved.flag_2`: литеры после кода ТН ВЭД (G_33_5)
- `goods[*].preference`: код преференции (G_36_2), например ОД
- `goods[*].procedure_code`: подтвердить 4000000 (ИМ40)
- `goods[*].mos_code_main`: код МОС (G_43_1)
- `goods[*].mos_code_extra`: доп. код МОС (G_43_2)
- `declaration.g42_2`: доп. признак графы 42, например «В ДТС»
- `representative.date`: дата подачи ДТ (G_54_20)
- `goods[*].customs_value`: таможенная стоимость (G_45) — считается Альтой
- `goods[*].statistical_value`: статистическая стоимость (G_46) — считается Альтой

### 3.2 Нужно вернуть в stage 1 (добавить факты в primary.md)
Отсутствуют.

## 4. Правила, требующие подтверждения (птп)

- `goods[*].tnved.flag_1`, `goods[*].tnved.flag_2`: нужны ли литеры
- `goods[*].preference`: ОД или иной код
- `goods[*].procedure_code`: 4000000
- `goods[*].mos_code_main`, `goods[*].mos_code_extra`: метод определения стоимости
- `declaration.g42_2`: «В ДТС» или пусто
- `representative.date`: дата подачи
- `representative.printed_block`: формат печатного блока

## 5. Блокеры перехода к этапу 2.1

- 11 pending-полей, большинство — птп, требующие решения оператора
- customs_value и statistical_value может оставить pending (Альта считает сама)

## 6. Практический план

1) Оператор подтверждает/уточняет коды: флаги ТН ВЭД, преференцию, процедуру, МОС, g42_2, дату подачи
2) AI закрывает pending в dt_fields.md
3) Критерий «можно запускать stage 2.1»: все поля кроме customs_value/statistical_value имеют статус CP/CO/D
