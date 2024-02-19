use super::*;
use chrono::Datelike;
use std::cmp::{max, min};

pub fn is_first_day_of_month(month: &NaiveDate) -> bool {
    month.day() == 1
}

pub fn get_last_day_month(month: &NaiveDate) -> NaiveDate {
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

pub fn is_last_day_of_month(date: &NaiveDate) -> bool {
    let last_day_of_month = get_last_day_month(date);
    *date == last_day_of_month
}

pub fn is_full_month(start: &NaiveDate, end: &NaiveDate) -> bool {
    is_first_day_of_month(start) && is_last_day_of_month(end)
}

pub fn calc_num_days_month(month: NaiveDate) -> i32 {
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

fn calculate_overlap_percentage(
    start1: NaiveDate,
    end1: NaiveDate,
    start2: NaiveDate,
    end2: NaiveDate,
) -> f64 {
    let overlap_start = max(start1, start2);
    let overlap_end = min(end1, end2);

    // If there's no overlap
    if overlap_start > overlap_end {
        return 1.0; // 100% non-overlapped since there's no overlap
    }

    let overlap_duration = (overlap_end - overlap_start).num_days() + 1; // +1 to include both start and end days in the count
    let total_duration = (end2 - start2).num_days() + 1; // For the unpaid period

    // Calculate non-overlapped percentage
    let non_overlapped_duration = total_duration - overlap_duration;
    let non_overlapped_percentage = non_overlapped_duration as f64 / total_duration as f64;

    non_overlapped_percentage
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_calculate_overlap_percentage() {
        // no_overlap_test()
        let already_paid_start = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        let already_paid_end = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();
        let unpaid_start = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        let unpaid_end = NaiveDate::from_ymd_opt(2024, 4, 30).unwrap();
        let non_overlapped_percentage = calculate_overlap_percentage(
            already_paid_start,
            already_paid_end,
            unpaid_start,
            unpaid_end,
        );
        assert_eq!(non_overlapped_percentage, 1.0); // 100% non-overlapped

        // complete_overlap_test()
        let already_paid_start = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let already_paid_end = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let unpaid_start = already_paid_start;
        let unpaid_end = already_paid_end;
        let non_overlapped_percentage = calculate_overlap_percentage(
            already_paid_start,
            already_paid_end,
            unpaid_start,
            unpaid_end,
        );
        assert_eq!(non_overlapped_percentage, 0.0); // 0% non-overlapped

        // partial_overlap_start_test()
        let already_paid_start = NaiveDate::from_ymd_opt(2024, 1, 20).unwrap();
        let already_paid_end = NaiveDate::from_ymd_opt(2024, 2, 5).unwrap();
        let unpaid_start = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let unpaid_end = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let non_overlapped_percentage = calculate_overlap_percentage(
            already_paid_start,
            already_paid_end,
            unpaid_start,
            unpaid_end,
        );
        assert!((non_overlapped_percentage - 0.827586).abs() < f64::EPSILON); // Approximately 82.76% non-overlapped

        // partial_overlap_end_test()
        let already_paid_start = NaiveDate::from_ymd_opt(2024, 2, 20).unwrap();
        let already_paid_end = NaiveDate::from_ymd_opt(2024, 3, 10).unwrap();
        let unpaid_start = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let unpaid_end = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
        let non_overlapped_percentage = calculate_overlap_percentage(
            already_paid_start,
            already_paid_end,
            unpaid_start,
            unpaid_end,
        );
        assert!((non_overlapped_percentage - 0.678571).abs() < f64::EPSILON); // Approximately 67.86% non-overlapped, considering leap year
    }
}
