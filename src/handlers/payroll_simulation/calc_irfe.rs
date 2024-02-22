use super::*;
use std::collections::HashSet;
use utils::*;

#[derive(Debug, PartialEq, Clone)]
pub struct ReceiptToPay {
    pub receipt_id: Uuid,
    pub value: f64,
}

pub fn calc_receipts_to_pay(
    receipts: Vec<&ReceiptsRes>,
    paid_receipts: Vec<&PaidReceiptsRes>,
    rci_fc_irfe: f64,
    city_fc_irfe: f64,
) -> Vec<ReceiptToPay> {
    let max_per_month = rci_fc_irfe * city_fc_irfe;
    let mut paid_receipt_ids: HashSet<Uuid> = HashSet::new();
    for paid_receipt in &paid_receipts {
        paid_receipt_ids.insert(paid_receipt.rf_receipt_id);
    }
    let mut vec_receipts = vec![];
    let mut paid_receipts: Vec<(NaiveDate, NaiveDate)> = paid_receipts
        .iter()
        .map(|&receipt| (receipt.start_date, receipt.end_date))
        .collect();
    for r in receipts {
        let mut value = 0.0;
        let is_not_paid = !paid_receipt_ids.contains(&r.rf_receipt_id);
        if is_not_paid {
            let mut percent = 1.0;
            for (pr_start, pr_end) in &paid_receipts {
                percent = calc_overlap_percentage(*pr_start, *pr_end, r.start_date, r.end_date);
            }
            let valid_months = calc_months_between(r.start_date, r.end_date) * percent;
            let max_value = valid_months * max_per_month;
            value = r.value * percent;
            if value > max_value {
                value = max_value;
            }
            paid_receipt_ids.insert(r.rf_receipt_id);
            paid_receipts.push((r.start_date, r.end_date));
            vec_receipts.push(ReceiptToPay {
                receipt_id: r.rf_receipt_id,
                value: ((value * r.rate * 100.0) + 0.5).floor() / 100.0,
            });
        }
    }

    vec_receipts
}

pub fn calc_irfe(value: f64, payroll_date: NaiveDate, person_id: Uuid) -> PayrollData {
    PayrollData {
        payroll_item: Uuid::parse_str(&var("ID_IRFE").unwrap()).unwrap(),
        person_id,
        value,
        date: payroll_date,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_calc_receipts_to_pay() {
        dotenv::from_filename("db.env").ok();
        let person_id = Uuid::parse_str("a8aaa402-957d-442d-b7f3-66c5c8e1e40e").unwrap();

        let rec_a = ReceiptsRes {
            rf_receipt_id: Uuid::parse_str("0575e238-dc3f-49ce-a5ba-413418f030ec").unwrap(),
            person_id,
            start_date: NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
            rate: 1.0,
            value: 7400.00,
        };
        let rec_b = ReceiptsRes {
            rf_receipt_id: Uuid::parse_str("51c1b668-5d50-443e-984d-7ce7378a0558").unwrap(),
            person_id,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            rate: 1.0,
            value: 7400.00,
        };
        let receipts = vec![&rec_a, &rec_b];
        let p_rec_a = PaidReceiptsRes {
            rf_receipt_id: Uuid::parse_str("0575e238-dc3f-49ce-a5ba-413418f030ec").unwrap(),
            person_id,
            start_date: NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
            payroll_closed_item_id: Uuid::parse_str("5f9cf6da-42e5-4fcd-bfdf-3608e556da60")
                .unwrap(),
        };
        let paid_receipts = vec![&p_rec_a];
        let expected = vec![ReceiptToPay {
            receipt_id: Uuid::parse_str("51c1b668-5d50-443e-984d-7ce7378a0558").unwrap(),
            value: 7400.00,
        }];
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let rci_fc_irfe = 150.00;
        let city_fc_irfe = 68.0;
        let result = calc_receipts_to_pay(
            receipts.clone(),
            paid_receipts.clone(),
            rci_fc_irfe,
            city_fc_irfe,
        );
        assert_eq!(result, expected);

        let rec_b = ReceiptsRes {
            rf_receipt_id: Uuid::parse_str("51c1b668-5d50-443e-984d-7ce7378a0558").unwrap(),
            person_id,
            start_date: NaiveDate::from_ymd_opt(2023, 12, 15).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            rate: 1.0,
            value: 11468.00,
        };
        let receipts = vec![&rec_a, &rec_b];
        let result = calc_receipts_to_pay(
            receipts.clone(),
            paid_receipts.clone(),
            rci_fc_irfe,
            city_fc_irfe,
        );
        let expected = vec![ReceiptToPay {
            receipt_id: Uuid::parse_str("51c1b668-5d50-443e-984d-7ce7378a0558").unwrap(),
            value: 7406.42,
        }];
        assert_eq!(result, expected);

        let rec_c = ReceiptsRes {
            rf_receipt_id: Uuid::parse_str("5e3adf28-09a2-4e9f-b112-f352c9f804dc").unwrap(),
            person_id,
            start_date: NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
            rate: 1.0,
            value: 15000.00,
        };
        let receipts = vec![&rec_a, &rec_b, &rec_c];
        let result = calc_receipts_to_pay(
            receipts.clone(),
            paid_receipts.clone(),
            rci_fc_irfe,
            city_fc_irfe,
        );
        let expected = vec![
            ReceiptToPay {
                receipt_id: Uuid::parse_str("51c1b668-5d50-443e-984d-7ce7378a0558").unwrap(),
                value: 7406.42,
            },
            ReceiptToPay {
                receipt_id: Uuid::parse_str("5e3adf28-09a2-4e9f-b112-f352c9f804dc").unwrap(),
                value: 10200.00,
            },
        ];
        assert_eq!(result, expected);

        let paid_receipts = vec![];
        let rec_a = ReceiptsRes {
            rf_receipt_id: Uuid::parse_str("0575e238-dc3f-49ce-a5ba-413418f030ec").unwrap(),
            person_id,
            start_date: NaiveDate::from_ymd_opt(2023, 12, 11).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
            rate: 1.0,
            value: 3800.00,
        };
        let rec_b = ReceiptsRes {
            rf_receipt_id: Uuid::parse_str("51c1b668-5d50-443e-984d-7ce7378a0558").unwrap(),
            person_id,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            rate: 1.0,
            value: 4200.00,
        };
        let receipts = vec![&rec_a, &rec_b];
        let result = calc_receipts_to_pay(
            receipts.clone(),
            paid_receipts.clone(),
            rci_fc_irfe,
            city_fc_irfe,
        );
        let expected = vec![
            ReceiptToPay {
                receipt_id: Uuid::parse_str("0575e238-dc3f-49ce-a5ba-413418f030ec").unwrap(),
                value: 3800.00,
            },
            ReceiptToPay {
                receipt_id: Uuid::parse_str("51c1b668-5d50-443e-984d-7ce7378a0558").unwrap(),
                value: 2845.16,
            },
        ];
        assert_eq!(result, expected);
    }
}
