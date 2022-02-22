//! Useful date and time related functions
use chrono::{offset::TimeZone, Date, Datelike, Local, NaiveDate};

/// Resolve first and last date for the month of given date
pub fn month_first_and_last_dates(date: &Date<Local>) -> (Date<Local>, Date<Local>) {
    let year = date.year();
    let month = date.month();
    let start_date = NaiveDate::from_ymd(year, month, 1);
    let end_date = if month == 12 {
        NaiveDate::from_ymd(year + 1, 1, 1).pred()
    } else {
        NaiveDate::from_ymd(year, month + 1, 1).pred()
    };

    (
        Local.from_local_date(&start_date).unwrap(),
        Local.from_local_date(&end_date).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    mod current_month_dates {
        use super::super::*;
        use chrono::NaiveDate;
        #[test]
        fn given_day_in_middle_of_month() {
            let wednesday = Local
                .from_local_date(&NaiveDate::from_ymd(2022, 01, 12))
                .unwrap();
            let (start_date, end_date) = month_first_and_last_dates(&wednesday);

            assert_eq!(start_date.naive_local(), NaiveDate::from_ymd(2022, 1, 1));
            assert_eq!(end_date.naive_local(), NaiveDate::from_ymd(2022, 1, 31));
        }
        #[test]
        fn last_day_in_middle_of_week() {
            let monday = Local
                .from_local_date(&NaiveDate::from_ymd(2022, 1, 31))
                .unwrap();
            let (start_date, end_date) = month_first_and_last_dates(&monday);

            assert_eq!(start_date.naive_local(), NaiveDate::from_ymd(2022, 1, 1));
            assert_eq!(end_date.naive_local(), NaiveDate::from_ymd(2022, 1, 31));
        }
        #[test]
        fn first_day_in_middle_of_week() {
            let tuesday = Local
                .from_local_date(&NaiveDate::from_ymd(2022, 2, 1))
                .unwrap();
            let (start_date, end_date) = month_first_and_last_dates(&tuesday);

            assert_eq!(start_date.naive_local(), NaiveDate::from_ymd(2022, 2, 1));
            assert_eq!(end_date.naive_local(), NaiveDate::from_ymd(2022, 2, 28));
        }
        #[test]
        fn given_day_in_december() {
            let tuesday = Local
                .from_local_date(&NaiveDate::from_ymd(2021, 12, 07))
                .unwrap();
            let (start_date, end_date) = month_first_and_last_dates(&tuesday);

            assert_eq!(start_date.naive_local(), NaiveDate::from_ymd(2021, 12, 1));
            assert_eq!(end_date.naive_local(), NaiveDate::from_ymd(2021, 12, 31));
        }
    }
}
