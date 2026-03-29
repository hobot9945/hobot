# xml_import_schema

## Назначение

`xml_import.md` — это **подготовленный набор значений для генерации новой `DT.xml`**, а не:

- финальный XML;
- эталонная выгрузка старой ДТ;
- отчет по кейсу;
- свалка системных и регистрационных полей.
  Документ должен отвечать на вопрос:
  **какие значения уже допускаются к генерации новой ДТ, из какого слоя они получены и что еще блокирует генерацию.**

---

## Главный принцип

В `xml_import.md` должны быть **разделены** 5 типов данных:

1. **xml_business_data** 
   Значения, готовые к генерации новой ДТ и происходящие из shipment facts.
2. **xml_master_data_dependencies** 
   Значения, которые нужны для новой ДТ, но происходят из:
   - карточек контрагентов,
   - карточки декларанта,
   - карточки представителя,
   - настроек / master data Альты.
3. **xml_calculated_data** 
   Значения, получаемые только после расчета:
   - курс,
   - таможенная стоимость,
   - статистическая стоимость,
   - графа 47,
   - иные расчетные поля.
4. **xml_mapping_dependencies** 
   Значения, которые не являются фактами поставки и не являются собственно расчетами, но нужны для генерации XML как:
   - process-константы,
   - коды режимов,
   - коды представления,
   - коды графы 30,
   - коды графы 44,
   - признаки UI/XML-логики,
   - presentation / process rules.
5. **reference_observed_xml_fields / system_do_not_generate** 
   Поля эталона и системные поля, которые не должны автоматически попадать в новую ДТ.

---

## Статусы значений

Использовать только:

- `ready`
- `pending`
- `conditional`
- `master_data_required`
- `calculation_required`
- `mapping_required`
- `reference_only`
- `do_not_generate`
  
  ### Пояснения
- `ready` — значение допускается к генерации.
- `conditional` — допускается при наличии дополнительных условий.
- `master_data_required` — нужно из master data.
- `calculation_required` — нужно получить из расчета.
- `mapping_required` — нужно подтвердить как process / mapping rule.
- `reference_only` — найдено только в эталоне, не переносится в рабочий слой.
- `do_not_generate` — системное / служебное / запрещено к генерации.

---

## Правила допуска в XML

1. В `xml_business_data` попадают только значения, для которых понятны:
   - источник;
   - слой происхождения;
   - правило допуска к проекции.
2. Значения из `reference_observed` не должны напрямую попадать в рабочий слой XML как business data.
3. Значения из `reference_observed` могут использоваться только:
   - как structural hint,
   - как completeness hint,
   - как mapping hint,
   - как process-rule confirmation,
     если это явно отражено в `xml_mapping_dependencies`.
4. Системные поля:
   - регистрационные,
   - ED_ID / ED_STAT / ED_TYP,
   - GUID,
   - BACK / FACE,
   - PARENT_*,
   - CREATEDATE,
   - FileName / user / time 
     не входят в генерацию новой ДТ.
5. Значения со статусами:
   - `pending`
   - `reference_only`
   - `do_not_generate`
     не должны маскироваться под готовые данные генерации.
6. Mapping-константы нельзя маскировать под shipment facts.

---

## Практический запрет на ложную готовность

Если значение видно в reference XML, это **НЕ** означает, что оно готово к генерации новой ДТ.
Для включения поля в рабочий XML-слой должно быть ясно:

1. это business data, master data, calculated value или mapping dependency;
2. reference используется как факт или только как mapping hint;
3. есть ли подтвержденная логика переноса в новую ДТ.
   Если этого нет — поле не должно получать статус `ready`.

---

## Meta

- case_name: <название кейса>
- source_facts: facts.md
- source_mapping: projection_mapping.md
- ready_for_xml_generation: <yes / partial / no>
- xml_generation_blockers:
  - <блокер 1>
  - <блокер 2>
  - <блокер 3>

---

## XML root settings

- xml_root:
  - value: AltaGTD
  - status: ready
- xml_encoding:
  - value: windows-1251
  - status: ready
- version:
  - value: <версия XML формата>
  - status: <ready / pending / reference_only>
- ed_version:
  - value: <версия ED>
  - status: <ready / pending / reference_only>

---

# xml_business_data

## Header fields

### Поля, заполняемые из shipment data

- G_5_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <facts shipment layer / derived>
- G_6_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <facts shipment layer>
- G_11_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / derived>
- G_15_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / document / derived>
- G_15A_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / derived>
- G_16_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / document / derived>
- G_16_2:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / derived>
- G_17_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / document / derived>
- G_17A_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / derived>
- G_20_20:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_20_21:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_22_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / derived>
- G_22_2:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_22_3:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- VES_BR_S:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- VES_NT_S:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
    
    ### Поля заголовка, зависящие от process / mapping rules
    
    См. раздел `xml_mapping_dependencies`:
- G_1_1
- G_1_2
- G_1_31
- G_3_1
- G_3_2

---

## G_2 Sender

- G_2_NAM:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_2_50:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <shipment / derived>
- G_2_7:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / derived>
- G_2_POS:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_2_SUB:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_2_CIT:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_2_STR:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_2_BLD:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <shipment / derived>
- G_2_ROM:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <shipment / derived>
- G_2_PHONE:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <shipment / operator>
- G_2_EMAIL:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <shipment / operator>
- G_2:
  - NAME:
  - value: <значение>
  - status: <ready / pending>
  - ADDRESS:
  - value: <собранная адресная строка>
  - status: <ready / pending>
  - note: составное представление из подтвержденных компонентов

---

## G_8 Consignee

- G_8_6:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / master_data>
- G_8_7:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / derived>
- G_8_50:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <shipment / derived / master_data>
- G_8_NAM:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / master_data>
- G_8_POS:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / master_data>
- G_8_SUB:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / master_data>
- G_8_CIT:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / master_data>
- G_8_STR:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / master_data>
- G_8_BLD:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <shipment / master_data>
- G_8_ROM:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <shipment / master_data>
- G_8:
  - NAME:
  - value: <значение или "СМ. ГРАФУ 14 ДТ">
  - status: <ready / conditional / pending>
  - note: это представление, а не самостоятельный business fact

---

# xml_master_data_dependencies

## G_8 Consignee

- G_8_1:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_8_PHONE:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_8_EMAIL:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_8_SM14:
  - value: <true / false>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator>
  - note: режим представления, а не business fact
    
    ## G_9 Financial responsible
- G_9_1:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_4:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_7:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_CC:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_CN:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_NAM:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_POS:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_SUB:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_CIT:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_STR:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_BLD:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_ROM:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_PHONE:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_EMAIL:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_9_SM14:
  - value: <true / false>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator>
- G_9:
  - NAME:
  - value: <значение или "СМ. ГРАФУ 14 ДТ">
  - status: <ready / conditional / pending>
    
    ## G_14 Declarant
- G_14_1:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator>
- G_14_4:
  - value: <значение>
  - status: <ready / master_data_required / pending>
  - source: <master_data / operator>
- G_14_CC:
  - value: <значение>
  - status: <ready / master_data_required / pending>
  - source: <master_data / operator>
- G_14_CN:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator>
- G_14_NAM:
  - value: <значение>
  - status: <ready / master_data_required / pending>
  - source: <master_data / operator>
- G_14_POS:
  - value: <значение>
  - status: <ready / master_data_required / pending>
  - source: <master_data / operator>
- G_14_SUB:
  - value: <значение>
  - status: <ready / master_data_required / pending>
  - source: <master_data / operator>
- G_14_CIT:
  - value: <значение>
  - status: <ready / master_data_required / pending>
  - source: <master_data / operator>
- G_14_STR:
  - value: <значение>
  - status: <ready / master_data_required / pending>
  - source: <master_data / operator>
- G_14_BLD:
  - value: <значение>
  - status: <ready / master_data_required / pending>
  - source: <master_data / operator>
- G_14_ROM:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator>
- G_14_PHONE:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator>
- G_14_EMAIL:
  - value: <значение>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator>
- G_14:
  - NAME:
  - value: <собранная строка декларанта>
  - status: <ready / master_data_required / pending>
  - note: составное представление, не исходный бизнес-факт поставки
    
    ## G_54 Representative
- G_54_3:
  - value: <фамилия>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_3NM:
  - value: <имя>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_3MD:
  - value: <отчество>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_4:
  - value: <наименование документа полномочий>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_5:
  - value: <номер документа полномочий>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_7:
  - value: <роль>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator>
- G_54_8:
  - value: <код представителя>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator>
- G_54_9:
  - value: <вид документа личности>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_12:
  - value: <серия документа>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_13:
  - value: <кем выдан>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_21:
  - value: <телефон>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_60:
  - value: <YYYY-MM-DD>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_61:
  - value: <YYYY-MM-DD>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_100:
  - value: <номер документа>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_101:
  - value: <YYYY-MM-DD>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54_EMAIL:
  - value: <email>
  - status: <ready / master_data_required / conditional / pending>
  - source: <master_data / operator / document>
- G_54P:
  - value: <печатный блок>
  - status: <ready / conditional / pending>
  - source: <composed>
  - note: производное представление, не самостоятельный источник
    
    ## Master data checklist
- consignee_master_data_ready:
  - value: <yes / partial / no>
  - status: <ready / pending>
- financial_responsible_master_data_ready:
  - value: <yes / partial / no>
  - status: <ready / pending>
- declarant_master_data_ready:
  - value: <yes / partial / no>
  - status: <ready / pending>
- representative_master_data_ready:
  - value: <yes / partial / no>
  - status: <ready / pending>
- master_data_blockers:
  - <блокер 1>
  - <блокер 2>

---

# xml_calculated_data

## Exchange rate and valuation

- G_12_0:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <calculated>
- G_12_1:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <calculated>
- G_23_1:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <calculated>
- G_23_2:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <calculated>
- exchange_rate:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <system / operator / calculation>
- exchange_rate_date_basis:
  - value: <правило / дата>
  - status: <ready / calculation_required / pending>
  - source: <system / operator / calculation>
- transport_cost_to_border:
  - value: <значение>
  - currency: <валюта>
  - status: <ready / pending>
  - source: <document / operator>
  - note: это не прямое поле XML, а вход расчета
- post_border_transport_cost:
  - value: <значение>
  - currency: <валюта>
  - status: <ready / conditional / pending>
  - source: <document>
- local_delivery_cost:
  - value: <значение>
  - currency: <валюта>
  - status: <ready / conditional / pending>
  - source: <document>
- insurance_value:
  - value: <значение / not_applicable>
  - status: <ready / calculation_required / pending>
  - source: <document / operator / calculation>
    
    ## BLOCKS
    
    ### BLOCK[1]
    
    #### Main goods fields
- G_32_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / derived>
- G_33_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / operator>
- G_34_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment / derived>
- G_35_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_36_2:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <operator / calculated>
- G_37_1:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <operator / calculated>
- G_38_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_42_1:
  - value: <значение>
  - status: <ready / pending>
  - source: <shipment>
- G_42_2:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <operator / calculated / mapping>
- G_45_0:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <calculated>
- G_45_1:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <calculated>
- G_46_1:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - source: <calculated>
    
    #### G_31 structured block
- G_31:
  - NAME:
  - value: <основной текст графы 31>
  - status: <ready / pending>
  - source: <composed from shipment facts>
  - FIRMA:
  - value: <производитель>
  - status: <ready / pending>
  - source: <shipment>
  - TM:
  - value: <товарный знак / отсутствует>
  - status: <ready / conditional / pending>
  - source: <shipment / derived / operator / mapping>
  - PLACE:
  - value: <количество мест>
  - status: <ready / conditional / pending>
  - source: <shipment>
  - PLACE2:
  - value: <маркировка / упаковка>
  - status: <ready / conditional / pending>
  - source: <shipment / operator / mapping>
    
    #### TXT lines
- TXT[1]:
  - TEXT:
  - value: <строка>
  - status: <ready / pending>
  - source: <composed>
- TXT[2]:
  - TEXT:
  - value: <строка>
  - status: <ready / pending>
  - source: <composed>
- TXT[3]:
  - TEXT:
  - value: <строка>
  - status: <ready / pending>
  - source: <composed>
    
    #### TOVG
- TOVG[1].G32G:
  - value: <значение>
  - status: <ready / pending>
- TOVG[1].G31_1:
  - value: <описание группы>
  - status: <ready / pending>
- TOVG[1].G31_11:
  - value: <производитель>
  - status: <ready / pending>
- TOVG[1].G31_12:
  - value: <товарный знак>
  - status: <ready / conditional / pending>
- TOVG[1].G31_14:
  - value: <марка>
  - status: <ready / conditional / pending>
- TOVG[1].G31_15_MOD:
  - value: <модель>
  - status: <ready / pending>
- TOVG[1].KOLVO:
  - value: <количество>
  - status: <ready / pending>
- TOVG[1].CODE_EDI:
  - value: <код единицы>
  - status: <ready / pending>
- TOVG[1].NAME_EDI:
  - value: <единица>
  - status: <ready / pending>
- TOVG[1].G31_35:
  - value: <вес брутто>
  - status: <ready / pending>
- TOVG[1].G31_38:
  - value: <вес нетто>
  - status: <ready / pending>
- TOVG[1].G31_42:
  - value: <стоимость>
  - status: <ready / pending>
- TOVG[1].INVOICCOST:
  - value: <стоимость>
  - status: <ready / pending>
    
    #### G47 payments
- G_47_1:
  - code:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - base:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - rate:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - amount:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - sp:
  - value: <значение>
  - status: <ready / calculation_required / pending>
- G_47_2:
  - code:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - base:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - rate:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - amount:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - sp:
  - value: <значение>
  - status: <ready / calculation_required / pending>
- G_47_3:
  - code:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - base:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - rate:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - amount:
  - value: <значение>
  - status: <ready / calculation_required / pending>
  - sp:
  - value: <значение>
  - status: <ready / calculation_required / pending>

---

# xml_mapping_dependencies

## Назначение слоя

Этот раздел нужен для XML-полей, которые:

- не являются фактами поставки;
- не являются чистыми master data;
- не являются просто расчетной величиной;
- но нужны для генерации XML как process / projection rules.
  `xml_mapping_dependencies` не заменяет:
- shipment_facts,
- documents_for_graph44_candidates,
- warehouse_* candidates,
- transport_* candidates.
  Этот слой хранит не сами бизнес-факты кейса, а правила,
  по которым факты и кандидаты превращаются в итоговые XML-поля.
  
  ## Важно: business value vs mapping dependency
  
  Если поле в XML нужно для новой ДТ, это еще не значит, что оно относится к business-data.
  Нужно различать:
1. `xml_business_data`:
   - значение происходит из shipment facts;
2. `xml_master_data_dependencies`:
   - значение происходит из master data;
3. `xml_calculated_data`:
   - значение получается из расчета;
4. `xml_mapping_dependencies`:
   - значение является process / projection rule.
     Одно и то же поле XML может иметь:
- business-level источник для конкретного кейса;
- и отдельное mapping-rule объяснение для шаблона.

---

## Header / declaration process rules

- G_1_1:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference>
  - note: не shipment fact, а process-константа
- G_1_2:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference>
- G_1_31:
  - value: <значение>
  - status: <ready / conditional / mapping_required / pending>
  - source: <mapping rule / operator / reference>
- G_3_1:
  - value: <значение>
  - status: <ready / conditional / mapping_required / pending>
  - source: <mapping rule / operator / reference>
- G_3_2:
  - value: <значение>
  - status: <ready / conditional / mapping_required / pending>
  - source: <mapping rule / operator / reference>
    
    ## Transport / customs / warehouse dependent fields
- G_18_0:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference>
- G_18:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <shipment / document / operator>
- G_18_2:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference>
  - note: может быть process-кодом, а не кодом страны в обычном смысле
- G_19_1:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference>
- G_21_0:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference>
- G_25_1:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <operator / calculation / process rule>
- G_26_1:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <operator / calculation / process rule>
- G_29_1:
  - value: <значение>
  - status: <ready / conditional / mapping_required / pending>
  - source: <operator / document / calculation / mapping>
- G_29_2:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <operator / system / calculation>
    
    ### Важно: графа 30 не равна документу СВХ "как есть"
    
    Для XML графы 30 нужно различать:
- документно подтвержденный факт хранения;
- документ СВХ как источник business-facts;
- и итоговые XML-поля графы 30,
  которые могут использовать:
  - код типа,
  - код вида документа,
  - реестровый номер,
  - реестровую дату,
  - код таможни.
    Нельзя автоматически считать, что:
- номер документа СВХ = `G_30_1`;
- дата документа СВХ = `G_30_DATE`.
- G_30_0:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference>
  - note: код типа графы 30
- G_30_10:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference>
  - note: код вида документа графы 30
- G_30_1:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / document / operator / reference>
  - note: итоговое поле может быть не равно номеру документа СВХ
- G_30_DATE:
  - value: <YYYY-MM-DD>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / document / operator / reference>
  - note: итоговое поле может быть не равно дате документа СВХ
- G_30_CC:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <document / derived>
- G_30_SUB:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <document / derived>
- G_30_CIT:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <document / derived>
- G_30_STR:
  - value: <значение>
  - status: <ready / conditional / pending>
  - source: <document / derived>
- G_30_12:
  - value: <значение>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference>
- G_30P_1:
  - value: <собранная печатная строка>
  - status: <ready / conditional / pending>
  - source: <compose from confirmed graph 30 parts>
  - note: presentation-field, не самостоятельный факт
    
    ## Graph 44 code mapping
    
    ### Важно: graph 44 строится в два этапа
1. Определяются business-документы кейса:
   - контракт,
   - инвойс,
   - packing list,
   - CMR,
   - платежка,
   - техописание,
   - транспортные документы,
   - доверенность и т.д.
2. Затем для них подбираются:
   - XML-код документа,
   - подкод / доп. реквизит,
   - сроки действия,
   - presentation-строка.
     Нельзя считать, что reference graph 44 = готовый набор документов новой ДТ.
     
     ### G44[1]
- role:
  - value: <contract / invoice / packing_list / cmr / payment / transport_invoice / authority_doc / other>
- item_scope:
  - value: <all_dt / goods_1 / goods_n>
  - status: <ready / conditional / pending>
- G4403:
  - value: <значение>
  - status: <ready / conditional / mapping_required / pending>
  - source: <mapping rule / operator / reference>
- G441:
  - value: <код>
  - status: <ready / mapping_required / pending>
  - source: <mapping rule / operator / reference / document>
  - note: код должен быть подтвержден mapping-логикой
- G441A:
  - value: <подкод>
  - status: <ready / conditional / mapping_required / pending>
  - source: <mapping rule / operator / reference>
- G442:
  - value: <номер>
  - status: <ready / pending>
  - source: <document / operator>
- G443:
  - value: <YYYY-MM-DD>
  - status: <ready / pending>
  - source: <document / operator>
- G444:
  - value: <наименование>
  - status: <ready / conditional / pending>
  - source: <document / operator / derived>
- G446:
  - value: <YYYY-MM-DD>
  - status: <ready / conditional / pending>
  - source: <document / operator / master_data>
- G447:
  - value: <YYYY-MM-DD>
  - status: <ready / conditional / pending>
  - source: <document / operator / master_data>
- DOCTEXT:
  - value: <собранное представление>
  - status: <ready / conditional / pending>
  - source: <composed>
  - note: производное представление строки
    
    ## Representation / process rules
- G_8_SM14:
  - value: <true / false>
  - status: <ready / mapping_required / master_data_required / pending>
  - source: <master_data / operator / reference>
  - note: режим представления
- G_9_SM14:
  - value: <true / false>
  - status: <ready / mapping_required / master_data_required / pending>
  - source: <master_data / operator / reference>
  - note: режим представления
- G_42_2:
  - value: <значение>
  - status: <ready / mapping_required / conditional / pending>
  - source: <mapping rule / valuation / operator / reference>
  - note: признак process-уровня

---

# reference_observed_xml_fields

## Поля эталонного XML, которые нельзя автоматически использовать для новой ДТ

### Важно

Наличие поля в эталонном XML не делает его:

- business data новой поставки;
- master data новой поставки;
- расчетным значением новой поставки;
- готовым значением генерации.
  Reference XML используется только:
- для проверки структуры;
- для поиска mapping dependencies;
- для diff analysis.
  
  ### Registration / system / customs result
- ___NUM:
  - value: <значение>
  - status: reference_only
- G_7_1:
  - value: <значение>
  - status: reference_only
- REGNUM:
  - value: <значение>
  - status: reference_only
- PARENT_ID:
  - value: <значение>
  - status: reference_only
- PARENT_DOC:
  - value: <значение>
  - status: reference_only
- CREATEDATE:
  - value: <значение>
  - status: reference_only
- D_CODE:
  - value: <значение>
  - status: reference_only
- D_DATE:
  - value: <значение>
  - status: reference_only
- D_TIME:
  - value: <значение>
  - status: reference_only
- D_LNP:
  - value: <значение>
  - status: reference_only
- D_RESOLUTIONDESCR:
  - value: <значение>
  - status: reference_only
- D_PERSON:
  - value: <значение>
  - status: reference_only
- D_DATE2:
  - value: <значение>
  - status: reference_only
- D_TIME2:
  - value: <значение>
  - status: reference_only
- D_G_CODE:
  - value: <значение>
  - status: reference_only
- D_G_DATE:
  - value: <значение>
  - status: reference_only
- D_G_TIME:
  - value: <значение>
  - status: reference_only
- D_G_LNP:
  - value: <значение>
  - status: reference_only
- D_G_RESOLUTIONDESCR:
  - value: <значение>
  - status: reference_only
- D_G_PERSON:
  - value: <значение>
  - status: reference_only
- FileName:
  - value: <значение>
  - status: reference_only
- user:
  - value: <значение>
  - status: reference_only
- time:
  - value: <значение>
  - status: reference_only
- ED_TYP / ED_ID / ED_STAT:
  - value: <значения>
  - status: reference_only
- BACK / FACE:
  - value: <значения>
  - status: reference_only
- ___RUSD / ___REUR / ___RR / ___DR / ___NEW44:
  - value: <значения>
  - status: reference_only

---

# system_do_not_generate

Ниже перечислены поля, которые не должны использоваться как входные данные для генерации новой ДТ:

- регистрационные номера и блоки регистрации;
- решения таможни;
- даты и время выпуска;
- ЛНП;
- системные ED_ID / ED_STAT / ED_TYP;
- BACK / FACE;
- PARENT_ID / PARENT_DOC / CREATEDATE;
- FileName / user / time;
- reference-значения, которые не подтверждены ни shipment-фактами, ни master data, ни расчетом, ни mapping-rule.

---

## Final readiness

- ready_for_xml_generation: <yes / partial / no>
- ready_for_partial_generation:
  - value: <yes / no>
  - note: можно ли строить partial / skeleton XML без закрытия всех блокеров
- generation_blockers:
  - <блокер 1>
  - <блокер 2>
  - <блокер 3>
- missing_master_data:
  - <master data field 1>
  - <master data field 2>
- missing_calculations:
  - <calc field 1>
  - <calc field 2>
- missing_mapping_rules:
  - <mapping field 1>
  - <mapping field 2>
- excluded_from_generation:
  - <поле 1>
  - <поле 2>
  - <поле 3>

---

## Практический принцип

Хороший `xml_import.md` должен позволять быстро ответить на 6 вопросов:

1. Какие поля уже готовы к генерации?
2. Какие поля еще зависят от master data Альты?
3. Какие поля еще зависят от расчета?
4. Какие поля еще зависят от mapping / process rules?
5. Какие поля видны только в эталоне и запрещены к переносу как факты новой поставки?
6. Можно ли уже строить:
   - полноценный `import.xml`,
   - partial export,
   - или пока только skeleton?
     Если документ не позволяет разделить business data, master data, calculated data и mapping dependencies — он недостаточно хорош.
