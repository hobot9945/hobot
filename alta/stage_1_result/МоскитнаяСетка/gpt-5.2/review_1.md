# Отчет по этапу 1: Сбор и формализация первичных данных (DEBUG BUILD)

## 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **Режим:** DEBUG BUILD (заполнение для отладки импорта XML)
- **Статус готовности:** Ready (для целей отладки)
- **Всего обработано документов:** 15
- **Количество конфликтов:** 0
- **Количество недостающих данных (Pending):** 0

## 2. Важное предупреждение
Этот `primary.md` **не отражает подтвержденные факты поставки**.

По явному разрешению оператора были нарушены правила этапа 1:
- "никаких догадок";
- "документная изоляция".

Часть полей была заполнена значениями, полученными переносом между документами (например, PL → Invoice, CMR → PL) и/или предположениями.
Все такие значения помечены в `primary.md` как:
- `status: confirmed_operator`
- `note: "assumption_for_debug"`

Цель: получить полный набор полей для генерации/отладки XML формализованных документов для импорта в Альту.

## 3. Сводка по документам

### Раздел I (Формализуемые)
- `Contract` — SALES_CONTRACT_2553_1.png (+2..5) — OK
- `Supplementary Agreement` — 1 Supplementary agreement to the contract.pdf — OK (частично с assumption_for_debug)
- `Invoice` — CL на сетку .pdf — OK (веса, origin, manufacturer заполнены assumption_for_debug)
- `Packing List` — PL на сетку .pdf — OK (transport заполнен assumption_for_debug)
- `CMR` — СМР от СВХ.pdf — OK (tnved/packing заполнены assumption_for_debug)
- `Transit Declaration` — ТД 10719110_240126_5011363_reg 00378тд (1).pdf — OK
- `Payment Order #1` — currency_transfer_1_13.01.2026.pdf — OK
- `Payment Order #7` — currency_transfer_7_28.11.2025.pdf — OK
- `Service Invoice` — Счет_№26-00378-tl_от_27-01-2026.pdf — OK
- `Insurance Document` — Счет_№26-00378-tl_1_от_14-01-2026.pdf — OK
- `Tech Description #1` — техничка .pdf — OK (date/document_sign заполнены assumption_for_debug)
- `Tech Description #2` — техничка Антикот, антипыльца антимошка .pdf — OK (date/document_sign заполнены assumption_for_debug)
- `Passport` — Passport_63_09_449948.xml (stable_source) — OK
- `Letter of Attorney` — LetterOfAttorney_1.xml (stable_source) — OK

### Раздел II (Неформализуемые)
- `СВХ (ДО-1)` — ДО 14431420260204161621.pdf — OK
- `СВХ доп. лист` — ДО доп 14431520260204161645.pdf — OK

## 4. Pending / вопросы к оператору

Отсутствуют (pending закрыты предположениями для отладки).

## 5. Примечание по воспроизводимости
Дополнения для debug-режима записаны в:
- `alta\\source\\operator\\operator_provided_data.md` (блок `assumption_for_debug`)
