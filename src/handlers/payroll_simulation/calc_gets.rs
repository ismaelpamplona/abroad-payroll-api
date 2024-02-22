use super::*;
use utils::*;

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

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

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
}
