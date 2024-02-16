use super::*;
use chrono::Datelike;

pub fn is_first_day_of_month(month: &NaiveDate) -> bool {
    month.day() == 1
}

fn get_last_day_month(month: &NaiveDate) -> NaiveDate {
    let (year, month, _) = (month.year(), month.month(), 1); // Extract year and month
    let next_month = if month == 12 { 1 } else { month + 1 }; // Get the next month, considering year change
    if let Some(last_day_of_month) =
        NaiveDate::from_ymd_opt(year, next_month, 1).and_then(|d| d.pred_opt())
    {
        last_day_of_month
    } else {
        panic!("Invalid month");
    }
}

fn is_last_day_of_month(date: &NaiveDate) -> bool {
    let last_day_of_month = get_last_day_month(date);
    *date == last_day_of_month
}

fn is_full_month(start: &NaiveDate, end: &NaiveDate) -> bool {
    is_first_day_of_month(start) && is_last_day_of_month(end)
}

fn calc_num_days_month(month: NaiveDate) -> i32 {
    let start = NaiveDate::from_ymd_opt(month.year(), month.month(), 1).unwrap();
    let end = get_last_day_month(&month);
    let mut diff = end.signed_duration_since(start).num_days() as i32;
    diff + 1
}

pub fn calc_valid_days(start: NaiveDate, end: Option<NaiveDate>, month: NaiveDate) -> i32 {
    // Ensure start date is not before the beginning of the month
    let start = start.max(NaiveDate::from_ymd_opt(month.year(), month.month(), 1).unwrap());
    let mut end_date = get_last_day_month(&month);
    if let Some(end) = end {
        end_date = end_date.min(end);
    }

    // Calculate the difference in days between end and start
    let mut diff = end_date.signed_duration_since(start).num_days() as i32 + 1;

    diff
}

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

// Function to calculate the number of years served abroad
pub fn calc_num_years_tsa(periods: Vec<TimeServedAbroadRes>, payroll_date: NaiveDate) -> i32 {
    let mut days = 0.0;
    for period in periods {
        let start_date = period.boarding_date;
        let mut end_date = get_last_day_month(&payroll_date);
        if let Some(end) = period.end_date {
            end_date = end_date.min(end);
        }
        let diff = end_date.signed_duration_since(start_date).num_days() as f64;
        days += diff;
    }
    (days / 365.0).floor() as i32
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

pub fn calc_num_years(start_date: NaiveDate, end_date: NaiveDate) -> f64 {
    let diff = end_date.signed_duration_since(start_date).num_days() as f64;
    diff / 365.0
}

pub fn get_years_from_date(start_date: NaiveDate, years: i32) -> NaiveDate {
    NaiveDate::from_ymd_opt(
        start_date.year() + years,
        start_date.month(),
        start_date.day(),
    )
    .unwrap()
}

pub fn calc_af(
    dependents: Vec<DependentsRes>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_is_first_day_of_month() {
        let day = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
        assert_eq!(is_first_day_of_month(&day), false);

        let day = NaiveDate::from_ymd_opt(2020, 10, 1).unwrap();
        assert_eq!(is_first_day_of_month(&day), true);
    }

    #[test]
    fn test_get_last_day_month() {
        let day = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let last = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(get_last_day_month(&day), last);

        let day = NaiveDate::from_ymd_opt(2024, 1, 29).unwrap();
        let last = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        assert_eq!(get_last_day_month(&day), last);
    }

    #[test]
    fn test_is_last_day_of_month() {
        let day = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
        assert_eq!(is_last_day_of_month(&day), false);

        let day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(is_last_day_of_month(&day), true);
    }

    #[test]
    fn test_is_full_month() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 2).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(is_full_month(&start, &end), false);

        let start = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
        assert_eq!(is_full_month(&start, &end), false);

        let start = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(is_full_month(&start, &end), true);
    }

    #[test]
    fn test_calc_num_days_monthh() {
        let month = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
        assert_eq!(calc_num_days_month(month), 29);

        let month = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        assert_eq!(calc_num_days_month(month), 31);
    }

    #[test]
    fn test_calc_valid_days() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
        let end = Some(NaiveDate::from_ymd_opt(2024, 2, 20)).unwrap();
        let month = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(calc_valid_days(start, end, month), 11);

        let start = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
        let end = Some(NaiveDate::from_ymd_opt(2024, 2, 15)).unwrap();
        let month = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(calc_valid_days(start, end, month), 6);

        let start = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
        let end = Some(NaiveDate::from_ymd_opt(2024, 2, 10)).unwrap();
        let month = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(calc_valid_days(start, end, month), 1);

        let start = NaiveDate::from_ymd_opt(2024, 2, 20).unwrap();
        let end = Some(NaiveDate::from_ymd_opt(2024, 2, 25)).unwrap();
        let month = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(calc_valid_days(start, end, month), 6);

        let start = NaiveDate::from_ymd_opt(2024, 2, 20).unwrap();
        let end = None;
        let month = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(calc_valid_days(start, end, month), 10);

        let start = NaiveDate::from_ymd_opt(2023, 2, 20).unwrap();
        let end = None;
        let month = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(calc_valid_days(start, end, month), 29);

        let start = NaiveDate::from_ymd_opt(2023, 4, 20).unwrap();
        let end = Some(NaiveDate::from_ymd_opt(2024, 4, 20).unwrap());
        let month = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(calc_valid_days(start, end, month), 29);
    }

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
    fn test_calc_num_years_tsa() {
        let payroll_date = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let periods = vec![
            TimeServedAbroadRes {
                boarding_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                end_date: None,
            },
            TimeServedAbroadRes {
                boarding_date: NaiveDate::from_ymd_opt(2018, 1, 1).unwrap(),
                end_date: Some(NaiveDate::from_ymd_opt(2021, 12, 1).unwrap()),
            },
        ];
        assert_eq!(calc_num_years_tsa(periods, payroll_date), 6);
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
        let dependents = vec![
            DependentsRes {
                person_id,
                birth_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                end_date: None,
                ir: true,
                type_id: Uuid::parse_str(&var("ID_SPOUSE").unwrap()).unwrap(),
                value: 0.1,
            },
            DependentsRes {
                person_id,
                birth_date: NaiveDate::from_ymd_opt(2010, 1, 1).unwrap(),
                start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                end_date: None,
                ir: true,
                type_id: Uuid::parse_str(&var("ID_SON_UNDER_21").unwrap()).unwrap(),
                value: 0.05,
            },
        ];
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let irex_value = 791.74;
        assert_eq!(
            calc_af(dependents, payroll_date, irex_value, person_id).value,
            ((irex_value * 0.15 * 100.0) + 0.5).floor() / 100.0
        );

        let dependents = vec![DependentsRes {
            person_id,
            birth_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end_date: None,
            ir: true,
            type_id: Uuid::parse_str(&var("ID_SPOUSE").unwrap()).unwrap(),
            value: 0.1,
        }];
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let irex_value = 791.74;
        assert_eq!(
            calc_af(dependents, payroll_date, irex_value, person_id).value,
            ((irex_value * 0.1 * 100.0) + 0.5).floor() / 100.0
        );

        let dependents = vec![DependentsRes {
            person_id,
            birth_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            start_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end_date: Some(NaiveDate::from_ymd_opt(2023, 12, 29).unwrap()),
            ir: true,
            type_id: Uuid::parse_str(&var("ID_SPOUSE").unwrap()).unwrap(),
            value: 0.1,
        }];
        let payroll_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let irex_value = 791.74;
        assert_eq!(
            calc_af(dependents, payroll_date, irex_value, person_id).value,
            0.0
        );
    }
}
