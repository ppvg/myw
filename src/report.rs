use crate::timelog;
use indexmap::map::IndexMap;

pub enum Fill {
    Sparse,
    Padded,
}

pub fn by_date(log: &timelog::Log, fill: Fill) -> Vec<(chrono::NaiveDate, chrono::TimeDelta)> {
    let timelog::Log(entries) = log;
    if entries.is_empty() {
        return vec![];
    }
    let mut map: IndexMap<chrono::NaiveDate, chrono::TimeDelta> = IndexMap::new();
    for entry in entries {
        map.entry(entry.from.date().clone())
            .and_modify(|d| *d += entry.duration())
            .or_insert(entry.duration());
    }
    map.sort_keys();
    if let Fill::Padded = fill {
        let start = map.first().unwrap().0;
        let end = map.last().unwrap().0;
        for date in DateRange(*start, *end) {
            if let None = map.get(&date) {
                map.insert(date, chrono::TimeDelta::zero());
            }
        }
        map.sort_keys();
    }
    map.drain(..).collect::<Vec<_>>()
}

pub fn by_project(log: &timelog::Log) -> Vec<(String, chrono::TimeDelta)> {
    let timelog::Log(entries) = log;
    if entries.is_empty() {
        return vec![];
    }
    let mut map: IndexMap<String, chrono::TimeDelta> = IndexMap::new();
    for entry in entries {
        map.entry(entry.project.clone())
            .and_modify(|d| *d += entry.duration())
            .or_insert(entry.duration());
    }
    map.sort_keys();
    map.drain(..).collect::<Vec<_>>()
}

pub fn sum<T>(items: &Vec<(T, chrono::TimeDelta)>) -> chrono::TimeDelta {
    items.iter().fold(chrono::TimeDelta::zero(), |sum, val| sum + val.1)
}

struct DateRange(chrono::NaiveDate, chrono::NaiveDate);

impl Iterator for DateRange {
    type Item = chrono::NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0.checked_add_days(chrono::Days::new(1))?;
            Some(std::mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
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
        let expected = vec![(date(2024, 02, 13), chrono::TimeDelta::hours(3))];
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
            (date(2024, 02, 13), chrono::TimeDelta::hours(3)),
            (date(2024, 02, 14), chrono::TimeDelta::hours(2)),
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
            (date(2024, 02, 13), chrono::TimeDelta::hours(3)),
            (date(2024, 02, 17), chrono::TimeDelta::hours(2)),
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
            (date(2024, 02, 13), chrono::TimeDelta::hours(3)),
            (date(2024, 02, 14), chrono::TimeDelta::zero()),
            (date(2024, 02, 15), chrono::TimeDelta::zero()),
            (date(2024, 02, 16), chrono::TimeDelta::zero()),
            (date(2024, 02, 17), chrono::TimeDelta::hours(2)),
        ];
        assert_eq!(expected, report);
    }

    #[test]
    fn by_date_unsorted_sparse() {
        let list = timelog::Log(vec![
            timelog::Entry::parse("10-11 ABC", &date(2024, 02, 17)).unwrap(),
            timelog::Entry::parse("10-12 ABC", &date(2024, 02, 13)).unwrap(),
            timelog::Entry::parse("10-13 ABC", &date(2024, 02, 14)).unwrap(),
        ]);
        let report: Vec<_> = by_date(&list, Fill::Sparse);
        let expected = vec![
            (date(2024, 02, 13), chrono::TimeDelta::hours(2)),
            (date(2024, 02, 14), chrono::TimeDelta::hours(3)),
            (date(2024, 02, 17), chrono::TimeDelta::hours(1)),
        ];
        assert_eq!(expected, report);
    }

    #[test]
    fn by_date_unsorted_padded() {
        let list = timelog::Log(vec![
            timelog::Entry::parse("10-11 ABC", &date(2024, 02, 17)).unwrap(),
            timelog::Entry::parse("10-12 ABC", &date(2024, 02, 13)).unwrap(),
            timelog::Entry::parse("10-13 ABC", &date(2024, 02, 14)).unwrap(),
        ]);
        let report: Vec<_> = by_date(&list, Fill::Padded);
        let expected = vec![
            (date(2024, 02, 13), chrono::TimeDelta::hours(2)),
            (date(2024, 02, 14), chrono::TimeDelta::hours(3)),
            (date(2024, 02, 15), chrono::TimeDelta::zero()),
            (date(2024, 02, 16), chrono::TimeDelta::zero()),
            (date(2024, 02, 17), chrono::TimeDelta::hours(1)),
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

    #[test]
    fn by_project_unsorted() {
        let list = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 DEF
            * 10-11 ABC
            ## 2024-02-14
            * 9-10 GHI
            * 10-11 ABC
        "});
        let report: Vec<_> = by_project(&list);
        let expected = vec![
            ("ABC".to_string(), chrono::TimeDelta::hours(2)),
            ("DEF".to_string(), chrono::TimeDelta::hours(1)),
            ("GHI".to_string(), chrono::TimeDelta::hours(1)),
        ];
        assert_eq!(expected, report);
    }
}
