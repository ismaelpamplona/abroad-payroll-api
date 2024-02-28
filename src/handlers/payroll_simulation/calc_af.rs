use super::*;
use chrono::Datelike;
use utils::*;

pub fn calc_af(
    dependents: Vec<&DependentsRes>,
    payroll_date: NaiveDate,
    irex_value: f64,
    person_id: Uuid,
) -> PayrollDataWithReceipt {
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
    PayrollDataWithReceipt {
        payroll_item: Uuid::parse_str(&var("ID_AF").unwrap()).unwrap(),
        person_id,
        value: ((percent * irex_value * 100.0) + 0.5).floor() / 100.0,
        date: payroll_date,
        receipt_id: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

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
}
