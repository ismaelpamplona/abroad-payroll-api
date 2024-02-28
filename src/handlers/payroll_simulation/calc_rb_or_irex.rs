use super::*;
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
) -> PayrollDataWithReceipt {
    let month_days = calc_num_days_month(payroll_date) as f64;
    let valid_days = calc_valid_days(start, end, payroll_date) as f64;
    let value = rci_fc * city_fc / month_days * valid_days;
    PayrollDataWithReceipt {
        payroll_item: item_id,
        person_id,
        value: ((value * 100.0) + 0.5).floor() / 100.0,
        date: payroll_date,
        receipt_id: None,
    }
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
}
