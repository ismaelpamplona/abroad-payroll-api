use super::*;
use chrono::Datelike;
use std::collections::HashSet;
use utils::*;

// rb or irex
pub fn calc_item(
    rci_fc: f64,
    city_fc: f64,
    start: NaiveDate,
    end: Option<NaiveDate>,
    payroll_date: NaiveDate,
    item_id: Uuid,
    person_id: Uuid,
) -> PayrollData {
    let month_days = calc_num_days_month(payroll_date) as f64;
    let valid_days = calc_valid_days(start, end, payroll_date) as f64;
    let value = rci_fc * city_fc / month_days * valid_days;
    PayrollData {
        payroll_item: item_id,
        person_id,
        value: ((value * 100.0) + 0.5).floor() / 100.0,
        date: payroll_date,
    }
}

pub fn calc_gets(
    periods: Vec<TimeServedAbroadRes>,
    payroll_date: NaiveDate,
    rb_value: f64,
    person_id: Uuid,
) -> PayrollData {
    let percent = calc_num_years_tsa(periods, payroll_date) as f64 / 100.0;
    PayrollData {
        payroll_item: Uuid::parse_str(&var("ID_GETS").unwrap()).unwrap(),
        person_id,
        value: ((percent * rb_value * 100.0) + 0.5).floor() / 100.0,
        date: payroll_date,
    }
}

pub fn calc_af(
    dependents: Vec<&DependentsRes>,
    payroll_date: NaiveDate,
    irex_value: f64,
    person_id: Uuid,
) -> PayrollData {
    let mut percent = 0.0;
    for d in dependents {
        let first_day_payroll =
            NaiveDate::from_ymd_opt(payroll_date.year(), payroll_date.month(), 1).unwrap();
        let last_day_payroll = get_last_day_month(&payroll_date);
        let is_under_21 = d.type_id == Uuid::parse_str(&var("ID_SON_UNDER_21").unwrap()).unwrap();
        let is_student = d.type_id == Uuid::parse_str(&var("ID_SON_STUDENT").unwrap()).unwrap();
        let is_spouse = d.type_id == Uuid::parse_str(&var("ID_SPOUSE").unwrap()).unwrap();
        let is_daughter =
            d.type_id == Uuid::parse_str(&var("ID_SINGLE_DAUGHTER").unwrap()).unwrap();
        let is_mother = d.type_id == Uuid::parse_str(&var("ID_WIDOW_MOTHER").unwrap()).unwrap();
        let is_single_woman =
            d.type_id == Uuid::parse_str(&var("ID_SINGLE_WOMAN").unwrap()).unwrap();
        let end_date = match d.end_date {
            Some(data) => data,
            None => get_last_day_month(&payroll_date),
        };
        if (d.start_date <= last_day_payroll
            && end_date >= first_day_payroll
            && end_date <= last_day_payroll)
            && ((get_years_from_date(d.birth_date, 21) >= first_day_payroll && is_under_21)
                || (get_years_from_date(d.birth_date, 24) >= first_day_payroll && is_student)
                || (is_spouse || is_daughter || is_mother || is_single_woman))
        {
            percent += d.value;
        }
    }
    PayrollData {
        payroll_item: Uuid::parse_str(&var("ID_AF").unwrap()).unwrap(),
        person_id,
        value: ((percent * irex_value * 100.0) + 0.5).floor() / 100.0,
        date: payroll_date,
    }
}

#[derive(Debug, PartialEq)]
pub struct ReceiptToPay {
    pub receipt_id: Uuid,
    pub value: f64,
}

pub fn calc_receipts_to_pay(
    receipts: Vec<&ReceiptsRes>,
    paid_receipts: Vec<&PaidReceiptsRes>,
    payroll_date: NaiveDate,
    rci_fc_irfe: f64,
    city_fc_irfe: f64,
) -> Vec<ReceiptToPay> {
    let max_per_month = rci_fc_irfe * city_fc_irfe;
    let mut paid_receipt_ids: HashSet<Uuid> = HashSet::new();
    for paid_receipt in &paid_receipts {
        paid_receipt_ids.insert(paid_receipt.rf_receipt_id);
    }
    let mut vec_receipts = vec![];

    for r in receipts {
        let mut value = 0.0;
        let is_not_paid = !paid_receipt_ids.contains(&r.rf_receipt_id);
        if is_not_paid {
            for pr in &paid_receipts {
                let percent = calculate_overlap_percentage(
                    pr.start_date,
                    pr.end_date,
                    r.start_date,
                    r.end_date,
                );
                let valid_months = calc_months_between(r.start_date, r.end_date) * percent;
                let max_value = valid_months * max_per_month;
                value = r.value * percent;
                if value > max_value {
                    value = max_value;
                }
            }
            vec_receipts.push(ReceiptToPay {
                receipt_id: r.rf_receipt_id,
                value: ((value * r.rate * 100.0) + 0.5).floor() / 100.0,
            });
        }
    }

    vec_receipts
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_calc_item() {
        dotenv::from_filename("db.env").ok();
        let rci_fc_rb = 94.0;
        let city_fc_rb = 76.70;
        let start = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 2, 29);
        let payroll_date = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let result = 7209.80;
        let item_id = Uuid::parse_str(&var("ID_RB").unwrap()).unwrap();
        let person_id = Uuid::parse_str("0575e238-dc3f-49ce-a5ba-413418f030ec").unwrap();
        assert_eq!(
            calc_item(
                rci_fc_rb,
                city_fc_rb,
                start,
                end,
                payroll_date,
                item_id,
                person_id
            )
            .value,
            result
        );

        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 4);
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = 930.30;
        assert_eq!(
            calc_item(
                rci_fc_rb,
                city_fc_rb,
                start,
                end,
                payroll_date,
                item_id,
                person_id
            )
            .value,
            result
        );

        let city_fc_rb = 92.82;
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 4);
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = 1125.82;
        assert_eq!(
            calc_item(
                rci_fc_rb,
                city_fc_rb,
                start,
                end,
                payroll_date,
                item_id,
                person_id
            )
            .value,
            result
        );

        let rci_fc_irex = 80.0;
        let city_fc_irex = 76.70;
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 4);
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = 791.74;
        let item_id = Uuid::parse_str(&var("ID_IREX").unwrap()).unwrap();
        assert_eq!(
            calc_item(
                rci_fc_irex,
                city_fc_irex,
                start,
                end,
                payroll_date,
                item_id,
                person_id
            )
            .value,
            result
        );

        let city_fc_irex = 57.12;
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 4);
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = 589.63;
        assert_eq!(
            calc_item(
                rci_fc_irex,
                city_fc_irex,
                start,
                end,
                payroll_date,
                item_id,
                person_id
            )
            .value,
            result
        );
    }

    #[test]
    fn test_calc_gets() {
        dotenv::from_filename("db.env").ok();
        let periods = vec![TimeServedAbroadRes {
            boarding_date: NaiveDate::from_ymd_opt(2021, 1, 4).unwrap(),
            end_date: Some(NaiveDate::from_ymd_opt(2024, 1, 4).unwrap()),
        }];
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let rb_value = 930.30;
        let person_id = Uuid::parse_str("0575e238-dc3f-49ce-a5ba-413418f030ec").unwrap();
        assert_eq!(
            calc_gets(periods.clone(), payroll_date, rb_value, person_id).value,
            27.91
        );

        let rb_value = 1125.82;
        assert_eq!(
            calc_gets(periods, payroll_date, rb_value, person_id).value,
            33.77
        );
    }

    #[test]
    fn test_calc_af() {
        dotenv::from_filename("db.env").ok();
        let person_id = Uuid::parse_str("0575e238-dc3f-49ce-a5ba-413418f030ec").unwrap();
        let res_a = DependentsRes {
            person_id,
            birth_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end_date: None,
            ir: true,
            type_id: Uuid::parse_str(&var("ID_SPOUSE").unwrap()).unwrap(),
            value: 0.1,
        };
        let res_b = DependentsRes {
            person_id,
            birth_date: NaiveDate::from_ymd_opt(2010, 1, 1).unwrap(),
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end_date: None,
            ir: true,
            type_id: Uuid::parse_str(&var("ID_SON_UNDER_21").unwrap()).unwrap(),
            value: 0.05,
        };
        let dependents = vec![&res_a, &res_b];
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let irex_value = 791.74;
        assert_eq!(
            calc_af(dependents, payroll_date, irex_value, person_id).value,
            ((irex_value * 0.15 * 100.0) + 0.5).floor() / 100.0
        );

        let res_a = DependentsRes {
            person_id,
            birth_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end_date: None,
            ir: true,
            type_id: Uuid::parse_str(&var("ID_SPOUSE").unwrap()).unwrap(),
            value: 0.1,
        };
        let dependents = vec![&res_a];
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let irex_value = 791.74;
        assert_eq!(
            calc_af(dependents, payroll_date, irex_value, person_id).value,
            ((irex_value * 0.1 * 100.0) + 0.5).floor() / 100.0
        );

        let res_a = DependentsRes {
            person_id,
            birth_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end_date: Some(NaiveDate::from_ymd_opt(2023, 12, 29).unwrap()),
            ir: true,
            type_id: Uuid::parse_str(&var("ID_SPOUSE").unwrap()).unwrap(),
            value: 0.1,
        };
        let dependents = vec![&res_a];
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let irex_value = 791.74;
        assert_eq!(
            calc_af(dependents, payroll_date, irex_value, person_id).value,
            0.0
        );
    }

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
            payroll_date,
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
            payroll_date,
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
            payroll_date,
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
    }
}
