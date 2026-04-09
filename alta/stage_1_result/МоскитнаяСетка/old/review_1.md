# review_1.md

## 1. Метаданные и статус
- **Кейс:** МоскитнаяСетка
- **Статус готовности:** Partial
- **Всего обработано документов:** 13
- **Количество конфликтов:** 2
- **Количество недостающих данных (Pending):** 3

## 2. Сводка по документам
- [Contract] — [SALES CONTRACT NoLM-2553.pdf] — [Статус: Внимание (часть полей pending, условия изменены допсоглашением)]
- [Supplementary Contract] — [1 Supplementary agreement to the contract.pdf] — [Статус: Внимание (есть незаполненные служебные признаки для XML)]
- [Invoice] — [CL на сетку .pdf] — [Статус: Внимание (часть заголовочных полей pending)]
- [Packing List] — [PL на сетку .pdf] — [Статус: OK]
- [Transit Declaration] — [ТД 10719110_240126_5011363_reg 00378тд (1).pdf] — [Статус: OK]
- [Storage Report / ДО-1] — [ДО 14431420260204161621.pdf] — [Статус: OK]
- [Storage Report / Доп. лист] — [ДО доп 14431520260204161645.pdf] — [Статус: OK]
- [CMR] — [СМР от СВХ.pdf] — [Статус: Внимание (скан низкого качества, часть полей pending)]
- [Service Invoice] — [Счет_№26-00378-tl_от_27-01-2026.pdf] — [Статус: OK]
- [Insurance Document] — [Счет_№26-00378-tl_1_от_14-01-2026.pdf] — [Статус: OK]
- [Tech Description] — [техничка .pdf] — [Статус: Внимание (документ шумный, есть конфликт по коду Антимошки)]
- [Letter of Attorney] — [LetterOfAttorney_1.xml] — [Статус: OK]
- [Passport] — [Passport_63_09_449948.xml] — [Статус: OK]
- [Transport Contract] — [FreeDoc_КООО_26651_М.xml] — [Статус: OK]
- [EGRUL] — [FreeDoc_ЮЭ9965-25-106893283.xml] — [Статус: OK]

## 3. Выявленные конфликты (ТРЕБУЕТСЯ РЕШЕНИЕ)
- **Конфликт #1**: Расхождение по коду ТН ВЭД товара Антимошка
  - Поле (UQI): `formalized.invoice_1.goods_5.tnved / formalized.invoice_1.goods_6.tnved`
  - Документ 1 (`техничка .pdf`): `6307909800 или 5903909000`
  - Документ 2 (`CL на сетку .pdf`): `7019900095`
  - Документ 3 (`ТД 10719110_240126_5011363_reg 00378тд (1).pdf`): `7019900095`
  - Документ 4 (`ДО 14431420260204161621.pdf`): `7019900095`
  - **Вопрос оператору:** Какой код ТН ВЭД считать приоритетным для формирования ДТ?

- **Конфликт #2**: Исходный предмет контракта отличается от фактической поставки
  - Поле (UQI): `formalized.contract_1.non_xml_fields.original_commodity`
  - Документ 1 (`SALES CONTRACT NoLM-2553.pdf`): `Anti-cat mosquito net 320 grams; SS 304 metal mosquito net`
  - Документ 2 (`1 Supplementary agreement to the contract.pdf`): `mosquito nets`
  - Документ 3 (`CL на сетку .pdf`): `7 строк москитных сеток, без SS 304 metal mosquito net`
  - **Вопрос оператору:** Считать ли, что допсоглашение №1 полностью корректно заменяет исходный предмет договора для 
    текущей поставки?

## 4. Недостающие данные / Pending (ТРЕБУЕТСЯ ОТВЕТ)
- **Вопрос #1**: Цифровой код валюты контракта и допсоглашения
  - Документ/Сущность: `contract_1`, `supplementary_contract_1`
  - Поле (UQI): `formalized.contract_1.currency_code`, `formalized.supplementary_contract_1.currency_code`
  - **Вопрос оператору:** Подтверди цифровой код валюты для RMB / Chinese yuan.

- **Вопрос #2**: Недостающие технические поля CMR
  - Документ/Сущность: `cmr_1`
  - Поле (UQI): `formalized.cmr_1.number`, `formalized.cmr_1.taking_cargo_country_code`, 
    `formalized.cmr_1.delivery_country_code`, `formalized.cmr_1.language_code`, `formalized.cmr_1.cmr_choice`
  - **Вопрос оператору:** Нужно ли заполнять эти поля по внешнему подтверждению, если на скане CMR они не читаются?

- **Вопрос #3**: Заголовочные поля инвойса, не читаемые напрямую в OCR
  - Документ/Сущность: `invoice_1`
  - Поле (UQI): `formalized.invoice_1.exchange_rate`, `formalized.invoice_1.total_gross_weight`, 
    `formalized.invoice_1.total_net_weight`, `formalized.invoice_1.dispatch_country_code`, 
    `formalized.invoice_1.destination_country_code`
  - **Вопрос оператору:** Подтверди, нужно ли оставлять эти поля pending до этапа 2/3 или можно выводить их из packing list и географии документа.

## 5. Инструкция для оператора
Пожалуйста, ответьте на вопросы и разрешите конфликты в формате:
- `Конфликт #1: использовать значение 7019900095 из инвойса и ТД`
- `Конфликт #2: да, допсоглашение полностью заменяет исходный предмет договора для текущей поставки`
- `Вопрос #1: код валюты 156`
- `Вопрос #2: оставить pending`
- `Вопрос #3: вывести dispatch_country_code=CN, destination_country_code=RU, gross/net из packing list`

После получения ответов можно обновить `primary.md`, заполнить раздел `operator` и довести этап до состояния `ready_for_next_step`.
