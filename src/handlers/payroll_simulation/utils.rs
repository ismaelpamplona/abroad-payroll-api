use super::*;
use chrono::{Datelike, Duration};
use std::cmp::{max, min};

pub fn is_first_day_of_month(month: &NaiveDate) -> bool {
    month.day() == 1
}

pub fn get_last_day_month(date: &NaiveDate) -> NaiveDate {
    let year = date.year();
    let month = date.month();
    // Adjust the year when wrapping from December to January.
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    // Attempt to create a date for the first day of the next month and then subtract one day to get the last day of the current month.
    if let Some(last_day_of_month) =
        NaiveDate::from_ymd_opt(next_year, next_month, 1).and_then(|d| d.pred_opt())
    {
        last_day_of_month
    } else {
        panic!("Invalid date operation");
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

pub fn calculate_overlap_percentage(
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

fn get_month_before_date(d: NaiveDate) -> NaiveDate {
    if is_last_day_of_month(&d) {
        return NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap();
    }
    let (year, month) = if d.month() == 1 {
        (d.year() - 1, 12)
    } else {
        (d.year(), d.month() - 1)
    };

    let day = if get_last_day_month(&d) == d {
        1
    } else {
        d.day() + 1
    };

    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

fn get_month_after_date(d: NaiveDate) -> NaiveDate {
    // Increment month, handling December to January transition.
    if d.day() == 1 {
        return get_last_day_month(&d);
    }
    let (year, month) = if d.month() == 12 {
        (d.year() + 1, 1)
    } else {
        (d.year(), d.month() + 1)
    };
    if is_last_day_of_month(&d) {
        let next_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let last_day_next_month = get_last_day_month(&next_month);
        return NaiveDate::from_ymd_opt(year, month, last_day_next_month.day() - 1).unwrap();
    }
    NaiveDate::from_ymd_opt(year, month, d.day() - 1).unwrap()
}

pub fn calc_months_between(start: NaiveDate, end: NaiveDate) -> f64 {
    if start >= end {
        return 0.0; // Ensure start date is before end date
    }

    let mut months = (end.year() - start.year()) * 12 + (end.month() as i32 - start.month() as i32);
    let day_start = start.day();
    let day_end = end.day();

    // If the end day is on or before the start day, consider it as ending the last month
    if day_end <= day_start {
        months -= 1;
    }

    let penultimate_month = get_month_before_date(end);
    let day_start_penultimate_month = NaiveDate::from_ymd_opt(
        penultimate_month.year(),
        penultimate_month.month(),
        start.day(),
    )
    .unwrap();

    let full = (get_month_after_date(day_start_penultimate_month) - day_start_penultimate_month)
        .num_days();

    let partial = (end - day_start_penultimate_month).num_days();

    let mut fraction_of_month = (partial as f64 / full as f64);

    months as f64 + fraction_of_month
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

        let day = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let last = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
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
        assert_eq!(
            ((non_overlapped_percentage * 100.0) + 0.5).floor() / 100.0,
            1.0
        ); // 100% non-overlapped

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
        assert_eq!(
            ((non_overlapped_percentage * 100.0) + 0.5).floor() / 100.0,
            0.0
        ); // 100% non-overlapped

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
        assert_eq!(
            ((non_overlapped_percentage * 100.0) + 0.5).floor() / 100.0,
            0.83
        ); // Approximately 82.76% non-overlapped

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
        assert_eq!(
            ((non_overlapped_percentage * 100.0) + 0.5).floor() / 100.0,
            0.68
        ); // Approximately 67.86% non-overlapped, considering leap year

        // partial_overlap_end_test()
        let already_paid_start = NaiveDate::from_ymd_opt(2023, 12, 1).unwrap();
        let already_paid_end = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let unpaid_start = NaiveDate::from_ymd_opt(2023, 12, 15).unwrap();
        let unpaid_end = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        let non_overlapped_percentage = calculate_overlap_percentage(
            already_paid_start,
            already_paid_end,
            unpaid_start,
            unpaid_end,
        );
        assert_eq!(
            ((non_overlapped_percentage * 100.0) + 0.5).floor() / 100.0,
            0.65
        ); // Approximately 67.86% non-overlapped, considering leap year
    }

    #[test]
    fn test_get_month_before_date() {
        let end_date = NaiveDate::from_ymd_opt(2024, 1, 14).unwrap();
        let expected = NaiveDate::from_ymd_opt(2023, 12, 15).unwrap();
        assert_eq!(get_month_before_date(end_date), expected);

        let end_date = NaiveDate::from_ymd_opt(2024, 2, 14).unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        assert_eq!(get_month_before_date(end_date), expected);

        let end_date = NaiveDate::from_ymd_opt(2024, 5, 10).unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 4, 11).unwrap();
        assert_eq!(get_month_before_date(end_date), expected);

        let end_date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert_eq!(get_month_before_date(end_date), expected);

        assert_eq!(get_month_before_date(end_date), expected);
    }

    #[test]
    fn test_get_month_after_date() {
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 2, 14).unwrap();
        assert_eq!(get_month_after_date(start_date), expected);

        let start_date = NaiveDate::from_ymd_opt(2023, 12, 15).unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 1, 14).unwrap();
        assert_eq!(get_month_after_date(start_date), expected);

        let start_date = NaiveDate::from_ymd_opt(2024, 5, 10).unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 6, 9).unwrap();
        assert_eq!(get_month_after_date(start_date), expected);

        let start_date = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 5, 31).unwrap();
        assert_eq!(get_month_after_date(start_date), expected);

        let start_date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        let expected = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
        assert_eq!(get_month_after_date(start_date), expected);
    }

    #[test]
    fn test_calc_months_between() {
        // Same month
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        assert_eq!(calc_months_between(start, end), 1.0);
        //
        // Full month difference
        let start = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 2, 14).unwrap();
        assert_eq!(calc_months_between(start, end), 1.0);

        // Partial month difference
        let start = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 10).unwrap();
        assert_eq!(
            ((calc_months_between(start, end) * 100.0) + 0.5).floor() / 100.0,
            1.86
        );

        // Multiple months difference
        let start = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 4, 14).unwrap();
        assert_eq!(calc_months_between(start, end), 3.0);

        // Cross-year difference
        let start = NaiveDate::from_ymd_opt(2023, 12, 15).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 14).unwrap();
        assert_eq!(calc_months_between(start, end), 1.0);

        // Longer period
        let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        assert_eq!(
            ((calc_months_between(start, end) * 100.0) + 0.5).floor() / 100.0,
            12.0
        );

        // Longer period
        let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert_eq!(
            ((calc_months_between(start, end) * 100.0) + 0.5).floor() / 100.0,
            12.03
        );
    }
}
