//! Useful date and time related functions
use chrono::{Datelike, NaiveDate};

/// Resolve first and last date for the month of given date
pub fn month_first_and_last_dates(date: &NaiveDate) -> (NaiveDate, NaiveDate) {
    let year = date.year();
    let month = date.month();
    let start_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let end_date = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    };

    (start_date, end_date)
}

#[cfg(test)]
mod tests {
    mod current_month_dates {
        use super::super::*;
        use chrono::NaiveDate;
        #[test]
        fn given_day_in_middle_of_month() {
            let wednesday = &NaiveDate::from_ymd_opt(2022, 01, 12).unwrap();
            let (start_date, end_date) = month_first_and_last_dates(&wednesday);

            assert_eq!(start_date, NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
            assert_eq!(end_date, NaiveDate::from_ymd_opt(2022, 1, 31).unwrap());
        }
        #[test]
        fn last_day_in_middle_of_week() {
            let monday = NaiveDate::from_ymd_opt(2022, 1, 31).unwrap();
            let (start_date, end_date) = month_first_and_last_dates(&monday);

            assert_eq!(start_date, NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
            assert_eq!(end_date, NaiveDate::from_ymd_opt(2022, 1, 31).unwrap());
        }
        #[test]
        fn first_day_in_middle_of_week() {
            let tuesday = NaiveDate::from_ymd_opt(2022, 2, 1).unwrap();
            let (start_date, end_date) = month_first_and_last_dates(&tuesday);

            assert_eq!(start_date, NaiveDate::from_ymd_opt(2022, 2, 1).unwrap());
            assert_eq!(end_date, NaiveDate::from_ymd_opt(2022, 2, 28).unwrap());
        }
        #[test]
        fn given_day_in_december() {
            let tuesday = NaiveDate::from_ymd_opt(2021, 12, 07).unwrap();
            let (start_date, end_date) = month_first_and_last_dates(&tuesday);

            assert_eq!(start_date, NaiveDate::from_ymd_opt(2021, 12, 1).unwrap());
            assert_eq!(end_date, NaiveDate::from_ymd_opt(2021, 12, 31).unwrap());
        }
    }
}
