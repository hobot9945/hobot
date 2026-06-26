# DOC_CONVERSION_REVIEW: Kyland 01

| NN | Basename | Status | Quality | Note |
|----|----------|--------|---------|------|
| 01 | CI, PL final | done | high | Invoice and Packing List extracted |
| 02 | АвиаНакладная | done | high | |
| 03 | ПЛАТЕЖКА | done | high | |
| 04 | Счет на оплату № 27611 от 27 декабря 2022 г | done | high | |
| 05 | Счет на оплату № VIG2227802 от 28.12.2022 | done | high | |
| 06 | Kyland SYM3000A... | done | high | 7 PDF files merged into TechDescription.md |
_audit: 6

## Критические точки (Верификация):

- **Стоимость (Invoice vs Payment):** USD 25,255.00 = USD 25,255.00. **OK**.
- **Количество (Invoice vs Packing):** 52 units = 52 units. **OK**.
- **Вес брутто (Packing vs AWB):** 90 kg = 90 kg. **OK**.
- **Грузовые места (Packing vs AWB):** 6 CTNS = 6 PCS. **OK**.

## Замечания:
- Дата инвойса в PL (23.12) отличается от даты в CI (12.12). В платежке указана дата 12.12.
- Разные адреса получателя (Научный проезд vs Рудневка).