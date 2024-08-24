use crate::timelog;
use crate::utils;

pub enum Fill {
    Padded,
    #[allow(dead_code)]
    Sparse,
}

pub fn by_date(log: &timelog::Log, fill: Fill) -> Vec<(chrono::NaiveDate, chrono::TimeDelta)> {
    let mut logs = log.by_date();
    if let Fill::Padded = fill {
        utils::pad_dates(&mut logs, None);
    }
    logs.drain(..)
        .map(|(date, log)| (date, log.sum_duration()))
        .collect::<Vec<_>>()
}

pub fn by_project(log: &timelog::Log) -> Vec<(String, chrono::TimeDelta)> {
    let mut logs = log.by_project();
    logs.drain(..)
        .map(|(project, log)| (project, log.sum_duration()))
        .collect::<Vec<_>>()
}

pub fn sum<T>(items: &[(T, chrono::TimeDelta)]) -> chrono::TimeDelta {
    items
        .iter()
        .fold(chrono::TimeDelta::zero(), |sum, val| sum + val.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn by_date_empty() {
        let list = timelog::Log(vec![]);
        let report: Vec<_> = by_date(&list, Fill::Sparse);
        assert!(report.is_empty());
        let report: Vec<_> = by_date(&list, Fill::Padded);
        assert!(report.is_empty());
    }

    #[test]
    fn by_date_one_date() {
        let list = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
        "});
        let report: Vec<_> = by_date(&list, Fill::Sparse);
        let expected = vec![(date(2024, 2, 13), chrono::TimeDelta::hours(3))];
        assert_eq!(expected, report);
    }

    #[test]
    fn by_date_multiple_dates() {
        let list = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
            ## 2024-02-14
            * 9-10 ABC
            * 10-11 DEF
        "});
        let report: Vec<_> = by_date(&list, Fill::Sparse);
        let expected = vec![
            (date(2024, 2, 13), chrono::TimeDelta::hours(3)),
            (date(2024, 2, 14), chrono::TimeDelta::hours(2)),
        ];
        assert_eq!(expected, report);
    }

    #[test]
    fn by_date_multiple_dates_sparse() {
        let list = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
            ## 2024-02-17
            * 9-10 ABC
            * 10-11 DEF
        "});
        let report: Vec<_> = by_date(&list, Fill::Sparse);
        let expected = vec![
            (date(2024, 2, 13), chrono::TimeDelta::hours(3)),
            (date(2024, 2, 17), chrono::TimeDelta::hours(2)),
        ];
        assert_eq!(expected, report);
    }

    #[test]
    fn by_date_multiple_dates_padded() {
        let list = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
            ## 2024-02-17
            * 9-10 ABC
            * 10-11 DEF
        "});
        let report: Vec<_> = by_date(&list, Fill::Padded);
        let expected = vec![
            (date(2024, 2, 13), chrono::TimeDelta::hours(3)),
            (date(2024, 2, 14), chrono::TimeDelta::zero()),
            (date(2024, 2, 15), chrono::TimeDelta::zero()),
            (date(2024, 2, 16), chrono::TimeDelta::zero()),
            (date(2024, 2, 17), chrono::TimeDelta::hours(2)),
        ];
        assert_eq!(expected, report);
    }

    #[test]
    fn by_project_empty() {
        let list = timelog::Log(vec![]);
        let report: Vec<_> = by_project(&list);
        assert!(report.is_empty());
        let report: Vec<_> = by_project(&list);
        assert!(report.is_empty());
    }

    #[test]
    fn by_project_one_project() {
        let list = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 ABC
            ## 2024-02-14
            * 9-10 ABC
        "});
        let report: Vec<_> = by_project(&list);
        let expected = vec![("ABC".to_string(), chrono::TimeDelta::hours(3))];
        assert_eq!(expected, report);
    }

    #[test]
    fn by_project_multiple_projects() {
        let list = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            ## 2024-02-14
            * 9-10 ABC
        "});
        let report: Vec<_> = by_project(&list);
        let expected = vec![
            ("ABC".to_string(), chrono::TimeDelta::hours(2)),
            ("DEF".to_string(), chrono::TimeDelta::hours(1)),
        ];
        assert_eq!(expected, report);
    }
}
