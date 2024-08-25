mod entry;

pub use entry::Entry;
use indexmap::map::IndexMap;
use std::sync::LazyLock;
use markdown::mdast;
use regex::Regex;

static DATE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b\d{4}-\d{2}-\d{2}\b").unwrap());

#[derive(Debug, Default, PartialEq)]
pub struct Log(pub Vec<Entry>);

impl Log {
    pub fn parse(input: &str) -> Self {
        let ast = parse_md(input);
        let nodes = ast.children().unwrap();
        let mut date: Option<chrono::NaiveDate> = None;
        let mut entries: Vec<Entry> = vec![];
        for node in nodes.iter() {
            if let mdast::Node::Heading(_) = node {
                date = parse_heading(&node.to_string());
                continue;
            }
            let mdast::Node::List(mdast::List { children, .. }) = node else {
                continue;
            };
            let Some(ref date) = date else {
                continue;
            };
            for list_item in children.iter() {
                let Some(item_text) = list_item.children().unwrap().first() else {
                    continue;
                };
                if let Some(entry) = Entry::parse(&item_text.to_string(), date) {
                    entries.push(entry);
                };
            }
        }
        entries.sort();
        Self(entries)
    }

    pub fn by_date(&self) -> IndexMap<chrono::NaiveDate, Self> {
        let Self(entries) = self;
        if entries.is_empty() {
            return IndexMap::new();
        }
        let mut map: IndexMap<chrono::NaiveDate, Self> = IndexMap::new();
        for entry in entries {
            map.entry(entry.from.date())
                .or_default()
                .0
                .push(entry.clone())
        }
        map
    }

    pub fn by_project(&self) -> IndexMap<String, Self> {
        let Self(entries) = self;
        if entries.is_empty() {
            return IndexMap::new();
        }
        let mut map: IndexMap<String, Self> = IndexMap::new();
        for entry in entries {
            map.entry(entry.project.clone())
                .or_default()
                .0
                .push(entry.clone())
        }
        map
    }

    pub fn sum_duration(self) -> chrono::TimeDelta {
        self.0
            .into_iter()
            .fold(chrono::TimeDelta::zero(), |sum, entry| {
                sum + entry.duration()
            })
    }
}

fn parse_heading(s: &str) -> Option<chrono::NaiveDate> {
    let cap = DATE_RE.captures(s)?;
    chrono::NaiveDate::parse_from_str(&cap[0], "%Y-%m-%d").ok()
}

fn parse_md(input: &str) -> mdast::Node {
    let opts = &markdown::ParseOptions::default();
    markdown::to_mdast(input, opts).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cmp::Eq, hash::Hash};

    fn date(y: i32, m: u32, d: u32) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn datetime(y: i32, m: u32, d: u32, hours: u32, minutes: u32) -> chrono::NaiveDateTime {
        let date = chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap();
        let time = chrono::NaiveTime::from_hms_opt(hours, minutes, 0).unwrap();
        chrono::NaiveDateTime::new(date, time)
    }

    fn map<T: Hash + Eq, U>(vec: Vec<(T, U)>) -> IndexMap<T, U> {
        vec.into_iter().collect::<IndexMap<T, U>>()
    }

    #[test]
    fn parse_empty_file() {
        let log = Log::parse("");
        assert!(log.0.is_empty());
    }

    #[test]
    fn parse_unrelated_list() {
        let log = Log::parse(indoc::indoc! {"
            ## Just some list
            * A list
            * But no timelog entries
        "});
        assert!(log.0.is_empty());
    }

    #[test]
    fn parse_list_empty_items() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            *
            *
        "});
        assert!(log.0.is_empty());
    }

    #[test]
    fn parse_no_list() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            Just some notes, no list of timelog entries
        "});
        assert!(log.0.is_empty());
    }

    #[test]
    fn parse_list_without_entries() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * A list
            * But no timelog entries
        "});
        assert!(log.0.is_empty());
    }

    #[test]
    fn parse_simple_list() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
        "});
        let expected = Log(vec![
            Entry {
                from: datetime(2024, 2, 13, 9, 0),
                until: datetime(2024, 2, 13, 10, 0),
                project: "ABC".into(),
                notes: None,
            },
            Entry {
                from: datetime(2024, 2, 13, 10, 0),
                until: datetime(2024, 2, 13, 11, 0),
                project: "DEF".into(),
                notes: None,
            },
        ]);
        assert_eq!(expected, log);
    }

    #[test]
    fn parse_unrelated_paragraph_and_simple_list() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            Some unrelated text, followed by list with entries:
            * 9-10 ABC
            * 10-11 DEF
        "});
        let expected = Log(vec![
            Entry {
                from: datetime(2024, 2, 13, 9, 0),
                until: datetime(2024, 2, 13, 10, 0),
                project: "ABC".into(),
                notes: None,
            },
            Entry {
                from: datetime(2024, 2, 13, 10, 0),
                until: datetime(2024, 2, 13, 11, 0),
                project: "DEF".into(),
                notes: None,
            },
        ]);
        assert_eq!(expected, log);
    }

    #[test]
    fn parse_mixed_list() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            Some unrelated text, followed by list with entries mixed with notes:
            * 9-10 ABC
              * Notes about this entry
            * Some unrelated list item
            * 10-11 DEF
        "});
        let expected = Log(vec![
            Entry {
                from: datetime(2024, 2, 13, 9, 0),
                until: datetime(2024, 2, 13, 10, 0),
                project: "ABC".into(),
                notes: None,
            },
            Entry {
                from: datetime(2024, 2, 13, 10, 0),
                until: datetime(2024, 2, 13, 11, 0),
                project: "DEF".into(),
                notes: None,
            },
        ]);
        assert_eq!(expected, log);
    }

    #[test]
    fn parse_multiple_dates_mixed_content() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            Some unrelated text, followed by list with entries mixed with notes:
            * 9-10 ABC
              * Notes about this entry
            * Some unrelated list item
            * 10-11 DEF
            ## 2024-02-14
            * 9-10 ABC
        "});
        let expected = Log(vec![
            Entry {
                from: datetime(2024, 2, 13, 9, 0),
                until: datetime(2024, 2, 13, 10, 0),
                project: "ABC".into(),
                notes: None,
            },
            Entry {
                from: datetime(2024, 2, 13, 10, 0),
                until: datetime(2024, 2, 13, 11, 0),
                project: "DEF".into(),
                notes: None,
            },
            Entry {
                from: datetime(2024, 2, 14, 9, 0),
                until: datetime(2024, 2, 14, 10, 0),
                project: "ABC".into(),
                notes: None,
            },
        ]);
        assert_eq!(expected, log);
    }

    #[test]
    fn parse_multiple_dates_unordered_and_repeating() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            ## 2024-02-14
            * 9-10 ABC
            ## 2024-02-13
            * 11-12 GHI
        "});
        let expected = Log(vec![
            Entry {
                from: datetime(2024, 2, 13, 9, 0),
                until: datetime(2024, 2, 13, 10, 0),
                project: "ABC".into(),
                notes: None,
            },
            Entry {
                from: datetime(2024, 2, 13, 10, 0),
                until: datetime(2024, 2, 13, 11, 0),
                project: "DEF".into(),
                notes: None,
            },
            Entry {
                from: datetime(2024, 2, 13, 11, 0),
                until: datetime(2024, 2, 13, 12, 0),
                project: "GHI".into(),
                notes: None,
            },
            Entry {
                from: datetime(2024, 2, 14, 9, 0),
                until: datetime(2024, 2, 14, 10, 0),
                project: "ABC".into(),
                notes: None,
            },
        ]);
        assert_eq!(expected, log);
    }

    #[test]
    fn by_date_empty() {
        let log = Log::parse("");
        let report = Log::by_date(&log);
        assert!(report.is_empty());
    }

    #[test]
    fn by_date_one_date() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
        "});
        let report = Log::by_date(&log);
        let e1 = Entry::parse("9-10 ABC", &date(2024, 2, 13)).unwrap();
        let e2 = Entry::parse("10-11 DEF", &date(2024, 2, 13)).unwrap();
        let e3 = Entry::parse("11-12 ABC", &date(2024, 2, 13)).unwrap();
        let expected = map(vec![(date(2024, 2, 13), Log(vec![e1, e2, e3]))]);
        assert_eq!(expected, report);
    }

    #[test]
    fn by_date_multiple_dates() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
            ## 2024-02-14
            * 9-10 ABC
            * 10-11 DEF
        "});
        let report = Log::by_date(&log);
        let e1 = Entry::parse("9-10 ABC", &date(2024, 2, 13)).unwrap();
        let e2 = Entry::parse("10-11 DEF", &date(2024, 2, 13)).unwrap();
        let e3 = Entry::parse("11-12 ABC", &date(2024, 2, 13)).unwrap();
        let e4 = Entry::parse("9-10 ABC", &date(2024, 2, 14)).unwrap();
        let e5 = Entry::parse("10-11 DEF", &date(2024, 2, 14)).unwrap();
        let expected = map(vec![
            (date(2024, 2, 13), Log(vec![e1, e2, e3])),
            (date(2024, 2, 14), Log(vec![e4, e5])),
        ]);
        assert_eq!(expected, report);
    }

    #[test]
    fn by_project_empty() {
        let log = Log::parse("");
        let report = Log::by_project(&log);
        assert!(report.is_empty());
    }

    #[test]
    fn by_project_one_date() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
        "});
        let report = Log::by_project(&log);
        let e1 = Entry::parse("9-10 ABC", &date(2024, 2, 13)).unwrap();
        let e2 = Entry::parse("10-11 DEF", &date(2024, 2, 13)).unwrap();
        let e3 = Entry::parse("11-12 ABC", &date(2024, 2, 13)).unwrap();
        let expected = map(vec![
            ("ABC".to_owned(), Log(vec![e1, e3])),
            ("DEF".to_owned(), Log(vec![e2])),
        ]);
        assert_eq!(expected, report);
    }

    #[test]
    fn by_project_multiple_dates() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
            ## 2024-02-14
            * 9-10 ABC
            * 10-11 DEF
        "});
        let report = Log::by_project(&log);
        let e1 = Entry::parse("9-10 ABC", &date(2024, 2, 13)).unwrap();
        let e2 = Entry::parse("10-11 DEF", &date(2024, 2, 13)).unwrap();
        let e3 = Entry::parse("11-12 ABC", &date(2024, 2, 13)).unwrap();
        let e4 = Entry::parse("9-10 ABC", &date(2024, 2, 14)).unwrap();
        let e5 = Entry::parse("10-11 DEF", &date(2024, 2, 14)).unwrap();
        let expected = map(vec![
            ("ABC".to_owned(), Log(vec![e1, e3, e4])),
            ("DEF".to_owned(), Log(vec![e2, e5])),
        ]);
        assert_eq!(expected, report);
    }

    #[test]
    fn sum_duration_empty() {
        let log = Log::parse("");
        let duration = log.sum_duration();
        let expected = chrono::TimeDelta::zero();
        assert_eq!(expected, duration);
    }

    #[test]
    fn sum_duration_entries() {
        let log = Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
            ## 2024-02-14
            * 9-10 ABC
            * 10-11 DEF
        "});
        let duration = log.sum_duration();
        let expected = chrono::TimeDelta::hours(5);
        assert_eq!(expected, duration);
    }
}
