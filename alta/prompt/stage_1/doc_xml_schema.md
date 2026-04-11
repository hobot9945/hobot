# doc_xml_schema.md — Stage 1 (формализованные документы → XML для импорта в Альту)
Версия: draft (собрано по эталонным выгрузкам `alta\reference\МоскитнаяСетка\выгрузки\*.xml`)

## Назначение
Этот документ задаёт:
1) **структуру XML** формализованных документов (корневые теги, вложенные узлы, массивы);
2) **маппинг** полей из `alta\stage_1_result\<case>\primary.md` в XML-теги;
3) **правила форматов** (даты, числа, кодировки, переносы строк).

Цель: на входе `primary.md` (этап 1) → на выходе набор XML в `alta\stage_1_result\<case>\formilized_docs\*.xml`
для импорта в Альту.

---

## Общие правила генерации XML

### Кодировка / декларация
- Всегда: `<?xml version="1.0" encoding="windows-1251"?>`

### Форматы дат
- В XML все даты в формате: `YYYY-MM-DD`
- В `primary.md` даты могут быть `dd.mm.yyyy` → при генерации конвертировать.

### Числа / десятичные
- Суммы денег обычно `NNNNN.NN` (2 знака), см. `TotalCost`, `PaymentAmount`, `TotalServiceCost`.
- Веса в эталонах иногда целые (`3500`, `3302`), иногда с 3 знаками (`855.000`).
  Рекомендация для устойчивости:
    - если в `primary.md` есть десятичная часть — сохранять её;
    - иначе писать как целое без `.00` (как в эталонах).

### Переносы строк в текстах
- Внутри текстовых тегов Альта использует `&#13;&#10;` как перевод строки.
- Если в `primary.md` многострочный `value` → заменять `\n` на `&#13;&#10;` (или генерировать как текст с CRLF,
  при сериализации в cp1251 контролировать результат).

### Заголовочные атрибуты корневого тега
В эталонах у корня есть атрибуты (`time`, `user`, `Version`, `FileName`, `EDVer`, `Comment`).
Для импорта обычно критична **структура и теги**, не эти атрибуты.
Рекомендация:
- можно либо копировать статический набор атрибутов,
- либо опускать атрибуты, если импорт Альты это допускает (нужно проверить в импорте).
  Если сомневаемся — генерировать с атрибутами, но значения можно ставить дефолтные.

---

## Нотация маппинга
- `UQI`: путь из `primary.md`, например `formalized.invoice_1.number`
- `XML`: путь тега в документе
- Для массивов:
    - `formalized.invoice_1.goods_[n]` → повторяющийся блок `<InvoiceGoods>...</InvoiceGoods>`
    - `formalized.cmr_1.goods_[n]` → `<CMRGoods>...</CMRGoods>`
    - `formalized.service_invoice_1.services_[n]` → `<ServiceDescription>...</ServiceDescription>`
    - `formalized.packing_list_1.transport_[n]` → `<TransportMeans>...</TransportMeans>` (эталон показывает 2 записи)

---

# 1) Invoice (код документа 04021)
Эталон: `Invoice_LM-2591.xml`
Корень: `<AltaE2I>`

## 1.1 Заголовок документа
| XML тег | UQI в primary.md | Комментарий |
|---|---|---|
| `DocumentCode` | (константа) | `04021` |
| `CurrencyRate` | `formalized.invoice_1.exchange_rate` | строка/число |
| `CurrencyCode` | `formalized.invoice_1.currency_code` | В эталоне `CNY` (не `RMB`). В `primary.md` хранить буквенный код. |
| `PlacesQuantity` | `formalized.invoice_1.places_quantity` | |
| `PlacesDescription` | `formalized.invoice_1.places_description` | |
| `GrossWeightQuantity` | `formalized.invoice_1.total_gross_weight` | |
| `NetWeightQuantity` | `formalized.invoice_1.total_net_weight` | |
| `GCost` | `formalized.invoice_1.total_amount` | в эталоне дублируется с `TotalCost` |
| `TotalCost` | `formalized.invoice_1.total_amount` | |
| `DeliveryTerms_DispatchCountryCode` | `formalized.invoice_1.dispatch_country_code` | `CN` |
| `DeliveryTerms_DestinationCountryCode` | `formalized.invoice_1.destination_country_code` | `RU` |
| `Registration_PrDocumentName` | (константа/или derived) | в эталоне: `ИНВОЙС (СЧЕТ-ФАКТУРА) К ДОГОВОРУ` |
| `Registration_PrDocumentNumber` | `formalized.invoice_1.number` | |
| `Registration_PrDocumentDate` | `formalized.invoice_1.date` | дата `YYYY-MM-DD` |
| `Contract_PrDocumentNumber` | `formalized.invoice_1.contract_ref_number` | |
| `Contract_PrDocumentDate` | `formalized.invoice_1.contract_ref_date` | дата `YYYY-MM-DD` |

### Реквизиты покупателя/продавца/грузоотправителя/грузополучателя
В эталоне присутствуют большие блоки (`Buyer_*`, `Seler_*`, `Consignor_*`, `Consignee_*`).
В `primary_schema.md` **нет** полного шаблона реквизитов для Invoice.
Правило:
- на этапе 1 можно заполнять минимально необходимое (как в текущих шаблонах),
- но для приближения к эталону лучше подтягивать реквизиты из:
    - Contract (`formalized.contract_1.*`),
    - LetterOfAttorney/стабильных данных,
    - либо расширить `primary_schema.md` позже.

Минимальный набор для импорта нужно выявить экспериментом: какие теги Альта требует обязательно.

## 1.2 Товарные строки
Повторяющийся блок: `<InvoiceGoods>...</InvoiceGoods>`

| XML тег внутри InvoiceGoods | UQI | Комментарий |
|---|---|---|
| `GoodsCode` | `formalized.invoice_1.goods_[n].tnved` | |
| `GoodsDescription` | `formalized.invoice_1.goods_[n].description` | в эталоне верхний регистр, русский; это presentation слой |
| `GoodsQuantity` | `formalized.invoice_1.goods_[n].quantity` | |
| `MeasureUnitQualifierName` | `formalized.invoice_1.goods_[n].unit` | в эталоне `М2` |
| `GrossWeightQuantity` | `formalized.invoice_1.goods_[n].gross_weight` | |
| `NetWeightQuantity` | `formalized.invoice_1.goods_[n].net_weight` | |
| `Price` | `formalized.invoice_1.goods_[n].price` | |
| `TotalCost` | `formalized.invoice_1.goods_[n].amount` | |
| `OriginCountryCode` | `formalized.invoice_1.goods_[n].origin_country_code` | **ВАЖНО:** в эталоне `156` (цифровой код страны), не `CN`. |
| `AdditionalGoodsDescription_Manufacturer` | `formalized.invoice_1.goods_[n].manufacturer` | |
| `AdditionalGoodsDescription_TradeMark` | `formalized.invoice_1.goods_[n].trade_mark` | |
| `AdditionalGoodsDescription_GoodsMark` | `formalized.invoice_1.goods_[n].goods_mark` | |
| `AdditionalGoodsDescription_GoodsModel` | `formalized.invoice_1.goods_[n].model` | в эталоне “АНТИКОТ 1.4 * 30”, не “1.4*30” |

---

# 2) Contract (код 03011)
Эталон: `Contract_LM-2553.xml`
Корень: `<AltaE2CONT>`

| XML тег | UQI | Комментарий |
|---|---|---|
| `DocumentCode` | (константа) | `03011` |
| `ContractTerms_Amount` | `formalized.contract_1.total_amount` | |
| `ContractTerms_CurrencyCode` | `formalized.contract_1.currency_code` | **цифровой** код (например, 156) |
| `ContractTerms_LastDate` | `formalized.contract_1.expiry_date` | `YYYY-MM-DD` |
| `ContractTerms_OtherTerms` | `formalized.contract_1.delivery_terms` | |
| `ContractTerms_ContractText` | `formalized.contract_1.text_body` или `link` | если `link` — нужно либо OCR/вставка текста, либо отдельный режим |
| `ContractTerms_DealSign` | `formalized.contract_1.deal_sign` | |
| `ForeignPerson_OrganizationName` | `formalized.contract_1.seller_name` | |
| `RussianPerson_OrganizationName` | `formalized.contract_1.buyer_name` | |
| `ContractRegistration_PrDocumentNumber` | `formalized.contract_1.number` | |
| `ContractRegistration_PrDocumentDate` | `formalized.contract_1.date` | `YYYY-MM-DD` |

Эталон содержит расширенные реквизиты сторон (INN/KPP/OGRN, адреса). Это кандидаты на расширение `primary_schema.md`.

---

# 3) Supplementary contract (доп. соглашение)
Эталон: `SupplementaryContract_1.xml`
Корень: `<AltaSupplementaryContract>`

| XML тег | UQI | Комментарий |
|---|---|---|
| `DocumentNumber` | `formalized.contract_2.number` | |
| `IssueDate` | `formalized.contract_2.date` | `YYYY-MM-DD` |
| `ContractDescription_Amount` | `formalized.contract_2.total_amount` | |
| `ContractDescription_CurrencyCode` | `formalized.contract_2.currency_code` | цифровой |
| `ContractDescription_LastDate` | `formalized.contract_2.expiry_date` | `YYYY-MM-DD` |
| `ContractDescription_ContractText` | `formalized.contract_2.text_body` или `link` | |
| `ContractDescription_DealSign` | `formalized.contract_2.deal_sign` | |
| `ContractDescription_StockCategorySign` | `formalized.contract_2.stock_category_sign` | |
| `ContractDescription_BuyerLimitationSign` | `formalized.contract_2.buyer_limitation_sign` | |
| `ContractDescription_InsuranceSign` | `formalized.contract_2.insurance_sign` | |
| `RussianPerson_OrganizationName` | `formalized.contract_2.buyer_name` | |
| `ForeignPerson_OrganizationName` | `formalized.contract_2.seller_name` | |
| `ContractSignedPerson/PersonSurname` | (нет прямого UQI) | в `primary_schema` для supplementary нет SignedPerson блока; сейчас не маппим |

---

# 4) Packing List (04131)
Эталон: `PackingList_БН.xml`
Корень: `<AltaE2PACK>`

## 4.1 Заголовок
| XML тег | UQI | Комментарий |
|---|---|---|
| `GrossWeightQuantity` | `formalized.packing_list_1.total_gross` | |
| `NetWeightQuantity` | `formalized.packing_list_1.total_net` | |
| `DeliveryTerms_DeliveryPlace` | `formalized.packing_list_1.delivery_place` | в эталоне `ХЭБЭЙ` |
| `DeliveryTerms_DeliveryTermsNumericCode` | (константа/derived) | в эталоне `01` |
| `DeliveryTerms_DeliveryTermsStringCode` | `formalized.packing_list_1.delivery_terms_string_code` | `EXW` |
| `DeliveryTerms_Contract_PrDocumentNumber` | `formalized.packing_list_1.contract_ref` | |
| `DeliveryTerms_Contract_PrDocumentDate` | (нужен UQI) | в `primary_schema` нет даты ссылки контракта в PL; можно брать из `formalized.contract_1.date` |
| `DeliveryTerms_Invoice_PrDocumentNumber` | `formalized.packing_list_1.invoice_ref` | |
| `DeliveryTerms_Invoice_PrDocumentDate` | (нужен UQI) | можно брать из `formalized.invoice_1.date` |
| `DeliveryTerms_Registration_PrDocumentName` | `formalized.packing_list_1.registration_doc_name` | в эталоне `УПАКОВОЧНЫЙ ЛИСТ` |
| `DeliveryTerms_Registration_PrDocumentNumber` | `formalized.packing_list_1.number` | в эталоне `БН` |
| `DeliveryTerms_Registration_PrDocumentDate` | `formalized.packing_list_1.date` | `YYYY-MM-DD` |

## 4.2 Goods в Packing List
Эталон: 2 блока `<Goods>...</Goods>` (агрегация), а в `primary_schema` PL содержит `goods_[1..7]`.
Это важное расхождение:
- либо Альта допускает 7 Goods,
- либо эталон агрегирует, а импорт может принять и 7.

Пока: маппить `primary.md` PL goods_[n] → `<Goods>` (n = 1..N), поля:
- `GoodsDescription` ← `formalized.packing_list_1.goods_[n].description`
- `GoodsQuantity` ← `formalized.packing_list_1.goods_[n].quantity_places_or_units`
- `GrossWeightQuantity` ← `formalized.packing_list_1.goods_[n].gross_weight`
- `NetWeightQuantity` ← `formalized.packing_list_1.goods_[n].net_weight`
- `PackingInfo/PakingQuantity` ← `formalized.packing_list_1.goods_[n].packing_quantity`

## 4.3 TransportMeans
Эталон содержит **2** блока `<TransportMeans>` (тягач и прицеп).
В `primary_schema` сейчас `transport_[n]` описан, но в текущем `primary.md` был один `transport_1`.
Для корректного генератора:
- поддержать массив `transport_[n]`.

Маппинг:
- `TransportMeans/Number` ← `formalized.packing_list_1.transport_[n].number`
- `TransportMeans/ModeCode` ← `formalized.packing_list_1.transport_[n].mode_code` (эталон 31)
- `TransportMeans/NationalityCode` ← `formalized.packing_list_1.transport_[n].nationality_code` (эталон `000`)
- `TransportMeans/MoverIndicator` ← `formalized.packing_list_1.transport_[n].mover_indicator` (`true`/`false`)

---

# 5) CMR (02015)
Эталон: `CMR_00378.xml`
Корень: `<AltaE3CMR>`

## 5.1 Заголовок
| XML тег | UQI | Комментарий |
|---|---|---|
| `LanguageCode` | `formalized.cmr_1.language_code` | |
| `GoodsQuantity` | `formalized.cmr_1.total_places` | |
| `RegistrationDocument_DateInf` | `formalized.cmr_1.date` | `YYYY-MM-DD` |
| `RegistrationDocument_RegID` | `formalized.cmr_1.number` | |
| `RegistrationDocument_Place` | `formalized.cmr_1.registration_place` | эталон: `КИТАЙ` |
| `DeliveryPlace_CountryCode` | `formalized.cmr_1.delivery_country_code` | |
| `CMRTransport_PrimeMoverStateSignID` | `formalized.cmr_1.truck_number` | |
| `CMRTransport_TrailerStateSignID` | `formalized.cmr_1.trailer_number` | |
| `TrakingCargo_TakingCargoDate` | `formalized.cmr_1.taking_cargo_date` | `YYYY-MM-DD` |
| `TrakingCargo_TakingCargoPlace_CountryCode` | `formalized.cmr_1.taking_cargo_country_code` | |
| `CMR_Choice` | `formalized.cmr_1.cmr_choice` | |
| `CMRGoodsWeight_GrossWeightQuantity` | `formalized.cmr_1.total_gross_weight` | |

## 5.2 Массив товаров CMRGoods
Эталон: 7 блоков `<CMRGoods>`.
Маппинг `formalized.cmr_1.goods_[n]` → `<CMRGoods>`:
- `GoodsDescription` ← `description`
- `GoodsNumeric` ← `item_no`
- `GoodsNomenclatureCode` ← `tnved`
- `GoodsQuantity` ← `quantity_places_or_units`
- `GrossWeightQuantity` ← `gross_weight`
- `GoodsPackingInfo/PackingCode` ← `packing_code`
- `GoodsPackingInfo/PakingQuantity` ← `packing_quantity`
- `GoodsPackingInfo/PackingDescription` ← `packing_description` (в эталоне `ПОДДОН`)

---

# 6) Payment Order (04023)
Эталоны: `PaymentOrder_1.xml`, `PaymentOrder_7.xml`
Корень: `<AltaPaymentOrder>`

| XML тег | UQI | Комментарий |
|---|---|---|
| `DocumentCode` | (константа) | `04023` |
| `PaymentModeCode` | `formalized.payment_order_[n].currency_mode_code` | **ВНИМАНИЕ:** эталон показывает `0`, а не `CNY`. Значит в `primary_schema` поле названо не тем смыслом. Нужна корректировка схемы/маппинга. |
| `PaymentAmount` | `formalized.payment_order_[n].amount` | |
| `TransactionKind` | `formalized.payment_order_[n].transaction_kind` | |
| `Purpose` | `formalized.payment_order_[n].purpose` | с `&#13;&#10;` |
| `DocumentReference_PrDocumentNumber` | `formalized.payment_order_[n].number` | |
| `DocumentReference_PrDocumentDate` | `formalized.payment_order_[n].date` | `YYYY-MM-DD` |
| `Payer_OrganizationName` | `formalized.payment_order_[n].payer_name` | |
| `Payer_INN` | `formalized.payment_order_[n].payer_inn` | |
| `Payer_KPP` | `formalized.payment_order_[n].payer_kpp` | |
| `Payee_OrganizationName` | `formalized.payment_order_[n].payee_name` | |
| `Payer_Bank_BankName` | `formalized.payment_order_[n].payer_bank_name` | |
| `Payee_Bank_BankName` | `formalized.payment_order_[n].payee_bank_name` | |
| `PayerSign/PersonSurname` | `formalized.payment_order_[n].payer_sign_surname` | |
| `PayerSign/PersonName` | `formalized.payment_order_[n].payer_sign_name` | |

---

# 7) Service Invoice (04031)
Эталон: `ServiceInvoice_26-00378-tl.xml`
Корень: `<AltaServiceInvoice>`

| XML тег | UQI | Комментарий |
|---|---|---|
| `DocumentSign` | `formalized.service_invoice_1.document_sign` (нет в primary_schema) | в эталоне `1` |
| `TotalServiceCost` | `formalized.service_invoice_1.total_amount` | |
| `Currency` | `formalized.service_invoice_1.currency` | |
| `ServiceProvider_Name` | `formalized.service_invoice_1.service_provider_name` | в эталоне обрезано (`ООО Трансимпериа`) |
| `ServiceProvider_PaymentRequisitions/BankName` | `formalized.service_invoice_1.service_provider_bank_name` | эталон хранит только имя банка, не реквизиты целиком |
| `ContractDetails_PrDocumentNumber` | `formalized.service_invoice_1.contract_ref_number` | |
| `ContractDetails_PrDocumentDate` | `formalized.service_invoice_1.contract_ref_date` | `YYYY-MM-DD` |
| `Registration_PrDocumentName` | (константа/derived) | |
| `Registration_PrDocumentNumber` | `formalized.service_invoice_1.number` | |
| `Registration_PrDocumentDate` | `formalized.service_invoice_1.date` | `YYYY-MM-DD` |

## 7.1 ServiceDescription (массив услуг)
Эталон: 2 блока `<ServiceDescription>`.
Маппинг `formalized.service_invoice_1.services_[n]` → `<ServiceDescription>`:
- `GoodsDescription` ← `goods_description` (в эталоне есть только в 1-й записи)
- `ServiceName` ← `route_description`
- `TaxRate` ← `tax_rate` (эталон `0.00`)
- `TaxSum` ← `tax_sum`
- `ServiceCost_Amount` ← `amount`
- `ServiceCost_Currency` ← `currency`
- `CurrencyCode` ← `currency` (в эталоне есть отдельным тегом)

---

# 8) Insurance document (04111) — в эталонах отсутствует отдельный XML
В выгрузках по кейсу нет явного `InsuranceDocument_*.xml` как отдельного root-типа.
По текущей схеме этапа 1 страхование у нас оформлено как `AltaFreeDoc` (или отдельный тип).
Нужно уточнить эталон: возможно страхование выгружено как `FreeDoc_26-00378-TL_1.xml` или как `FreeBinaryDoc_*`.
(Пока в doc_xml_schema.md не фиксируем, пока не прочитан конкретный эталон страхования.)

---

# 9) FreeDoc / TechDescription (05999)
Эталон: `FreeDoc_БН.xml`
Корень: `<AltaFreeDoc>`

| XML тег | UQI | Комментарий |
|---|---|---|
| `DocumentHead_DocumentName` | `formalized.tech_description_[n].doc_name` | |
| `DocumentHead_DocumentDate` | `formalized.tech_description_[n].date` | `YYYY-MM-DD` |
| `DocumentHead_DocumentNumber` | `formalized.tech_description_[n].number` | |
| `DocumentCode` | (константа) | `05999` |
| `DocumentBody_TextSection/TextPara` | `formalized.tech_description_[n].text_body` или `link` | если `link` — нужен режим “вложить файл” (`FreeBinaryDoc`) либо OCR |
| `DocumentSign` | `formalized.tech_description_[n].document_sign` | в эталоне пусто |

---

# 10) Passport (11001)
Эталон: `Passport_63_09_449948.xml`
Корень: `<AltaPassport>`

Маппинг прямой 1:1 с `primary_schema.md`:
- `CardSeries` ← `formalized.passport_1.series`
- `CardNumber` ← `formalized.passport_1.number`
- `OrganizationName` ← `formalized.passport_1.issued_by`
- `CardDate` ← `formalized.passport_1.issue_date`
- `PersonInfo_*` ← `formalized.passport_1.full_name` (нужно разложение) или отдельные поля
- `ResidencePlace_*` ← `formalized.passport_1.residence_address` (лучше разложить по компонентам)

---

# 11) Letter of Attorney (11004)
Эталон: `LetterOfAttorney_1.xml`
Корень: `<AltaLetterOfAttorney>`

Ключевые теги:
- `Subject` ← `formalized.letter_of_attorney_1.subject` (в эталоне большой текст с `&#13;&#10;`)
- `EndDate` ← `valid_until` (YYYY-MM-DD)
- `DocumentReference_PrDocumentName/Number/Date` ← number/date + константа имени
- `Organization_*` ← issuer_*
- `Organization_OrganizationPerson_*` ← директор
- `EmpoweredPerson_*` ← attorney_*
- `EmpoweredPerson_Passport_*` ← паспортные данные

---

## Открытые вопросы (для дальнейшей отладки промптов/схем)
1) `PaymentModeCode` в `AltaPaymentOrder`:
    - в `primary_schema` сейчас `currency_mode_code` хранит `CNY`,
    - в эталоне `PaymentModeCode=0`.
      Нужно уточнить: это код режима/вида платежа, не валюта.

2) Packing List:
    - в эталоне товары агрегированы в 2 строки, а в primary_schema — 7 goods.
      Нужно решить, какой формат ожидать генератору.

3) Insurance document:
    - найти конкретный эталон в `FreeDoc_26-00378-TL_1.xml` / `FreeBinaryDoc_*` и добавить сюда.

4) Заполнение реквизитов Buyer/Seller/Consignor/Consignee:
    - primary_schema пока не покрывает всех нужных полей, но эталонные XML их содержат.