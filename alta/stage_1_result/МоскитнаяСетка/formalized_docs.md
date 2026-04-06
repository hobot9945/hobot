# Формализованные документы

## Метаданные

- case_name: МоскитнаяСетка
- source_folder: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02
- dt_scope: 1 ДТ
- status: in_progress
- ready_for_next_step: yes
- unresolved_conflicts_count: 0
- unresolved_missing_critical_data_count: 1
- note: Оригинал контракта отсутствует в виде PDF. Формализован на основе ссылок в Инвойсе и Платежках.

## Файл:

- `document`: contract
  - `full_path`: pending
  - `name`: pending
  - `xml_target`: AltaE2CONT
  - `status`: pending
  - `blocking_for_next_step`: no

## Файл:

- `document`: invoice
  
  - `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\CL на сетку .pdf
  - `name`: CL на сетку .pdf
  - `xml_target`: AltaE2I
  - `status`: confirmed_document
  - `blocking_for_next_step`: no

- `number`
  
  - `value`: LM-2591
  - `xml_target`: Registration_PrDocumentNumber
  - `status`: confirmed_document

- `date`
  
  - `value`: 30.10.2025
  - `xml_target`: Registration_PrDocumentDate
  - `status`: confirmed_document

- `total_amount`
  
  - `value`: 97260.00
  - `xml_target`: TotalCost
  - `status`: confirmed_document

- `currency_code`
  
  - `value`: RMB
  - `xml_target`: CurrencyCode
  - `status`: confirmed_document

- `exchange_rate`
  
  - `value`: pending
  - `xml_target`: CurrencyRate
  - `status`: pending

### Товарные позиции (InvoiceGoods):

**Позиция 1**

- `item_no`: 1
- `description`: Anti-cat mesh. Roll size 1.4 * 30 Material: polyester/ Москитная сетка «Антикот» Размер рулона 1,4*30 Материал полиэстер
- `tnved`: 5804101000
- `quantity`: 60
- `unit`: Sets/Кол-во наборов
- `price`: 245.7
- `amount`: 14742.00

**Позиция 2**

- `item_no`: 2
- `description`: Anti-cat mesh Roll size 1.6 *30 /Москитная сетка «Антикот» Размер рулона 1,6*30
- `tnved`: 5804101000
- `quantity`: 30
- `unit`: Sets/Кол-во наборов
- `price`: 280.8
- `amount`: 8424.00

**Позиция 3**

- `item_no`: 3
- `description`: ANTI-POLLEN MESH. Material: polyester 1,4*30 M2 /Сетка против пыльцы "Антипыльца " из полиэстера . Размер рулона 1,4*30 M2/Материал: полиэстер
- `tnved`: 5804101000
- `quantity`: 60
- `unit`: Sets/Кол-во наборов
- `price`: 266.7
- `amount`: 16002.00

**Позиция 4**

- `item_no`: 4
- `description`: ANTI-POLLEN MESH. Material: polyeste 1,6*30 M2/Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,6*30 M2 Материал: полиэстер
- `tnved`: 5804101000
- `quantity`: 30
- `unit`: Sets/Кол-во наборов
- `price`: 304.8
- `amount`: 9144.00

**Позиция 5**

- `item_no`: 5
- `description`: MIDGE MEHS Material: Fiberglass. Roll size: 1,4*30 M2 /СЕТКА СРЕДНЕГО РАЗМЕРА " Антимошка " из стекловолокна. Размер рулона 1,4*30 M2
- `tnved`: 7019900095
- `quantity`: 90
- `unit`: Sets/Кол-во наборов
- `price`: 142.8
- `amount`: 12852.00

**Позиция 6**

- `item_no`: 6
- `description`: MIDGE MESH Material: Fiberglass. Roll size: 1,6*30 M2 : Fiberglass /СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна. Размер рулона 1,6*30 M2
- `tnved`: 7019900095
- `quantity`: 180
- `unit`: Sets/Кол-во наборов
- `price`: 163.2
- `amount`: 29376.00

**Позиция 7**

- `item_no`: 7
- `description`: GRID WITH 3 LAYER made of polyester Roll size 1,6*30 M2/Трехслойные сетки "Антипыльца " из полиэстера Размер рулона 1,6*30 M2
- `tnved`: 5804101000
- `quantity`: 5
- `unit`: Sets/Кол-во наборов
- `price`: 1344.0
- `amount`: 6720.00

## Файл:

- `document`: PackingList
  
  - `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\PL на сетку .pdf
  - `name`: PL на сетку .pdf
  - `xml_target`: AltaE2PACK
  - `status`: confirmed_document
  - `blocking_for_next_step`: no

- `number`
  
  - `value`: LM-2591
  - `xml_target`: DeliveryTerms_Registration_PrDocumentNumber
  - `status`: confirmed_document

- `date`
  
  - `value`: 30.10.2025
  - `xml_target`: DeliveryTerms_Registration_PrDocumentDate
  - `status`: confirmed_document

- `total_gross`
  
  - `value`: 3500.00
  - `xml_target`: GrossWeightQuantity
  - `status`: confirmed_document

- `total_net`
  
  - `value`: 3302.00
  - `xml_target`: NetWeightQuantity
  - `status`: confirmed_document

- `contract_ref`
  
  - `value`: LM-2553
  - `xml_target`: DeliveryTerms_Contract_PrDocumentNumber
  - `status`: confirmed_document

- `invoice_ref`
  
  - `value`: LM-2591
  - `xml_target`: DeliveryTerms_Invoice_PrDocumentNumber
  - `status`: confirmed_document

### Товарные позиции (Goods):

**Позиция 1**

- `item_no`: 1
- `description`: Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,4*0,16*0,16
- `quantity_places_or_units`: 60
- `gross_weight`: 855.00
- `net_weight`: 806.60
- `packing_quantity`: 60

**Позиция 2**

- `item_no`: 2
- `description`: Anti-cat mesh /Антивандальная москитная сетка «Антикот» Размер рулона 1,6*0,16*0,16
- `quantity_places_or_units`: 30
- `gross_weight`: 490.00
- `net_weight`: 460.80
- `packing_quantity`: 30

**Позиция 3**

- `item_no`: 3
- `description`: ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера . Размер рулона 1,42*0,64*0,22
- `quantity_places_or_units`: 60
- `gross_weight`: 265.00
- `net_weight`: 252.00
- `packing_quantity`: 6

**Позиция 4**

- `item_no`: 4
- `description`: ANTI-POLLEN MESH/Сетка против пыльцы Антипыльца " из полиэстера Размер рулона 1,62*0,64*0,23
- `quantity_places_or_units`: 30
- `gross_weight`: 155.00
- `net_weight`: 144.00
- `packing_quantity`: 3

**Позиция 5**

- `item_no`: 5
- `description`: MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА " Антимошка " из стекловолокна. Размер рулона 1,42*0,55*0,18
- `quantity_places_or_units`: 90
- `gross_weight`: 520.00
- `net_weight`: 491.40
- `packing_quantity`: 9

**Позиция 6**

- `item_no`: 6
- `description`: MIDGE MEHS/СЕТКА СРЕДНЕГО РАЗМЕРА "Антимошка " из стекловолокна. Размер рулона 1,62*0,55*18
- `quantity_places_or_units`: 180
- `gross_weight`: 1190.00
- `net_weight`: 1123.20
- `packing_quantity`: 18

**Позиция 7**

- `item_no`: 7
- `description`: GRID WITH 3 LAYER/Трехслойные сетки "Антипыльца " из полиэстера Размер рулона 1,72*0,35* 0,31*1
- `quantity_places_or_units`: 5
- `gross_weight`: 25.00
- `net_weight`: 24.00
- `packing_quantity`: 1

## Файл:

- `document`: CMR
  
  - `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\СМР от СВХ.pdf
  - `name`: СМР от СВХ.pdf
  - `xml_target`: AltaE3CMR
  - `status`: confirmed_document
  - `blocking_for_next_step`: no

- `number`
  
  - `value`: 00378
  - `xml_target`: RegistrationDocument_RegID
  - `status`: confirmed_document

- `date`
  
  - `value`: 20.01.2026
  - `xml_target`: RegistrationDocument_DateInf
  - `status`: confirmed_document

- `registration_place`
  
  - `value`: Маньчжурия
  - `xml_target`: RegistrationDocument_Place
  - `status`: confirmed_document

- `taking_cargo_date`
  
  - `value`: 20.01.2026
  - `xml_target`: TrakingCargo_TakingCargoDate
  - `status`: confirmed_document

- `taking_cargo_country_code`
  
  - `value`: CN
  - `xml_target`: TrakingCargo_TakingCargoPlace_CountryCode
  - `status`: confirmed_document

- `delivery_country_code`
  
  - `value`: RU
  - `xml_target`: DeliveryPlace_CountryCode
  - `status`: confirmed_document

- `truck_number`
  
  - `value`: О157АО774
  - `xml_target`: CMRTransport_PrimeMoverStateSignID
  - `status`: confirmed_document

- `trailer_number`
  
  - `value`: ВТ374974
  - `xml_target`: CMRTransport_TrailerStateSignID
  - `status`: confirmed_document

- `total_places`
  
  - `value`: 127
  - `xml_target`: GoodsQuantity
  - `status`: confirmed_document

- `total_gross_weight`
  
  - `value`: 3500.00
  - `xml_target`: CMRGoodsWeight_GrossWeightQuantity
  - `status`: confirmed_document

### Товарные позиции (CMRGoods):

**Позиция 1**

- `item_no`: 1
- `description`: Товар загружен согласно спецификации к Invoice № LM-2591 от 30.10.2025
- `quantity_places_or_units`: 127
- `gross_weight`: 3500.00

## Файл:

- `document`: ServiceInvoice
  
  - `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\Счет_№26-00378-tl_от_27-01-2026.pdf
  - `name`: Счет_№26-00378-tl_от_27-01-2026.pdf
  - `xml_target`: AltaServiceInvoice
  - `status`: confirmed_document
  - `blocking_for_next_step`: no
  - `note`: Счет за транспорт

- `number`
  
  - `value`: 26-00378-tl
  - `xml_target`: Registration_PrDocumentNumber
  - `status`: confirmed_document

- `date`
  
  - `value`: 27.01.2026
  - `xml_target`: Registration_PrDocumentDate
  - `status`: confirmed_document

- `total_amount`
  
  - `value`: 2700.00
  - `xml_target`: TotalServiceCost
  - `status`: confirmed_document

- `currency`
  
  - `value`: USD
  - `xml_target`: Currency
  - `status`: confirmed_document

- `service_provider_name`
  
  - `value`: ООО "Трансимпериал"
  - `xml_target`: ServiceProvider_Name
  - `status`: confirmed_document

- `contract_ref_number`
  
  - `value`: КООО/26651/М
  - `xml_target`: ContractDetails_PrDocumentNumber
  - `status`: confirmed_document

- `contract_ref_date`
  
  - `value`: 13.05.2025
  - `xml_target`: ContractDetails_PrDocumentDate
  - `status`: confirmed_document

### Услуги (ServiceDescription):

**Позиция 1**

- `item_no`: 1
- `route_description`: Транспортно-экспедиционные услуги в международном сообщении... по маршруту: China, Hengshui - граница РФ (п/п Маньчжурия/Забайкальск)
- `amount`: 1404.00
- `currency`: USD

**Позиция 2**

- `item_no`: 2
- `route_description`: Транспортно-экспедиционные услуги по маршруту: граница РФ (п/п Маньчжурия/Забайкальск) - Россия, Республика Татарстан, Набережные Челны
- `amount`: 1296.00
- `currency`: USD

## Файл:

- `document`: ServiceInvoice
  
  - `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\Счет_№26-00378-tl_1_от_14-01-2026.pdf
  - `name`: Счет_№26-00378-tl_1_от_14-01-2026.pdf
  - `xml_target`: AltaServiceInvoice
  - `status`: confirmed_document
  - `blocking_for_next_step`: no
  - `note`: Счет за страхование

- `number`
  
  - `value`: 26-00378-tl/1
  - `xml_target`: Registration_PrDocumentNumber
  - `status`: confirmed_document

- `date`
  
  - `value`: 14.01.2026
  - `xml_target`: Registration_PrDocumentDate
  - `status`: confirmed_document

- `total_amount`
  
  - `value`: 910.34
  - `xml_target`: TotalServiceCost
  - `status`: confirmed_document

- `currency`
  
  - `value`: RUB
  - `xml_target`: Currency
  - `status`: confirmed_document

- `service_provider_name`
  
  - `value`: ООО "Трансимпериал"
  - `xml_target`: ServiceProvider_Name
  - `status`: confirmed_document

- `contract_ref_number`
  
  - `value`: КООО/26651/М
  - `xml_target`: ContractDetails_PrDocumentNumber
  - `status`: confirmed_document

- `contract_ref_date`
  
  - `value`: 13.05.2025
  - `xml_target`: ContractDetails_PrDocumentDate
  - `status`: confirmed_document

### Услуги (ServiceDescription):

**Позиция 1**

- `item_no`: 1
- `route_description`: Возмещение за добровольное страхование груза
- `amount`: 910.34
- `currency`: RUB

## Файл:

- `document`: PaymentOrder
  
  - `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\платежки\currency_transfer_1_13.01.2026.pdf
  - `name`: currency_transfer_1_13.01.2026.pdf
  - `xml_target`: AltaPaymentOrder
  - `status`: confirmed_document
  - `blocking_for_next_step`: no

- `number`
  
  - `value`: 1
  - `xml_target`: DocumentReference_PrDocumentNumber
  - `status`: confirmed_document

- `date`
  
  - `value`: 13.01.2026
  - `xml_target`: DocumentReference_PrDocumentDate
  - `status`: confirmed_document

- `amount`
  
  - `value`: 63219.00
  - `xml_target`: PaymentAmount
  - `status`: confirmed_document

- `currency_mode_code`
  
  - `value`: CNY
  - `xml_target`: PaymentModeCode
  - `status`: confirmed_document

- `purpose`
  
  - `value`: PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30
  - `xml_target`: Purpose
  - `status`: confirmed_document

- `payer_name`
  
  - `value`: LLC SKIF
  - `xml_target`: Payer_OrganizationName
  - `status`: confirmed_document

- `payee_name`
  
  - `value`: HEBEI LANGMAI IMPORT AND EXPORT CO., LTD
  - `xml_target`: Payee_OrganizationName
  - `status`: confirmed_document

## Файл:

- `document`: PaymentOrder
  
  - `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\платежки\currency_transfer_7_28.11.2025.pdf
  - `name`: currency_transfer_7_28.11.2025.pdf
  - `xml_target`: AltaPaymentOrder
  - `status`: confirmed_document
  - `blocking_for_next_step`: no

- `number`
  
  - `value`: 7
  - `xml_target`: DocumentReference_PrDocumentNumber
  - `status`: confirmed_document

- `date`
  
  - `value`: 28.11.2025
  - `xml_target`: DocumentReference_PrDocumentDate
  - `status`: confirmed_document

- `amount`
  
  - `value`: 34041.00
  - `xml_target`: PaymentAmount
  - `status`: confirmed_document

- `currency_mode_code`
  
  - `value`: CNY
  - `xml_target`: PaymentModeCode
  - `status`: confirmed_document

- `purpose`
  
  - `value`: PURCHASE OF A MOSQUITO NET. CONTRACT NO.:LM-2553,DATE:JULY 02,2025, INVOICE NO.: LM-2591, DATE: 2025.10.30
  - `xml_target`: Purpose
  - `status`: confirmed_document

- `payer_name`
  
  - `value`: LLC SKIF
  - `xml_target`: Payer_OrganizationName
  - `status`: confirmed_document

- `payee_name`
  
  - `value`: HEBEI LANGMAI IMPORT AND EXPORT CO., LTD
  - `xml_target`: Payee_OrganizationName
  - `status`: confirmed_document

## Файл:

- `document`: FreeDoc
  
  - `full_path`: alta\source\МоскитнаяСетка\HEBEI LANGMAI IMPORT AND EXPORT\02\техничка Антикот, антипыльца антимошка .pdf
  - `name`: техничка Антикот, антипыльца антимошка .pdf
  - `xml_target`: AltaFreeDoc
  - `status`: confirmed_document
  - `blocking_for_next_step`: no

- `doc_name`
  
  - `value`: Технические характеристики
  - `xml_target`: DocumentHead_DocumentName
  - `status`: confirmed_document

- `number`
  
  - `value`: pending
  - `xml_target`: DocumentHead_DocumentNumber
  - `status`: pending

- `date`
  
  - `value`: pending
  - `xml_target`: DocumentHead_DocumentDate
  - `status`: pending

- `text_body`
  
  - `value`: Сетки из полиэстера 5804101000. 1. Москитная сетка "Антикот". 2. Москитная сетка на окно "Антипыльца". 3. Трехслойные сетки "Антипыльца ". Сетки из Стекловолокна 7019900095. 1. Москитная сетка Антимошка.
  - `xml_target`: TextPara
  - `status`: confirmed_document
