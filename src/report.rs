use crate::timelog;
use crate::utils;
use colored::Colorize;
use std::fmt;

pub enum Fill {
    Padded,
    #[allow(dead_code)]
    Sparse,
}

#[derive(Debug, PartialEq)]
pub struct Report {
    pub title: String,
    pub entries: Option<Vec<(String, chrono::TimeDelta)>>,
    pub total: Option<chrono::TimeDelta>,
}

pub struct TextReport(Report);

impl Report {
    #[cfg(test)]
    pub fn by_date(log: &timelog::Log, fill: Fill) -> Self {
        let mut logs = log.by_date();
        if let Fill::Padded = fill {
            utils::pad_dates(&mut logs, None);
        }
        Self {
            title: "By date".to_owned(),
            entries: Some(
                logs.drain(..)
                    .map(|(date, log)| (date.to_string(), log.sum_duration()))
                    .collect::<Vec<_>>(),
            ),
            total: None,
        }
    }

    #[cfg(test)]
    pub fn by_project(log: &timelog::Log) -> Self {
        let mut logs = log.by_project();
        Self {
            title: "By project".to_owned(),
            entries: Some(
                logs.drain(..)
                    .map(|(project, log)| (project, log.sum_duration()))
                    .collect::<Vec<_>>(),
            ),
            total: None,
        }
    }

    #[cfg(test)]
    pub fn total(log: &timelog::Log) -> Self {
        Self {
            title: "Total".to_owned(),
            entries: None,
            total: Some(log.sum_duration()),
        }
    }

    pub fn by_date_by_project(log: &timelog::Log, fill: Fill) -> Vec<Self> {
        let mut logs = log.by_date();
        if let Fill::Padded = fill {
            utils::pad_dates(&mut logs, None);
        }
        logs.drain(..)
            .map(|(date, log)| {
                let mut logs = log.by_project();
                let total = log.sum_duration();
                Self {
                    title: date.to_string(),
                    entries: Some(
                        logs.drain(..)
                            .map(|(project, log)| (project, log.sum_duration()))
                            .collect::<Vec<_>>(),
                    ),
                    total: Some(total),
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn text(self) -> TextReport {
        TextReport(self)
    }
}

impl fmt::Display for TextReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let report = &self.0;
        let title = report.title.bold();
        if let Some(total) = report.total {
            let total = format_hours(&total);
            writeln!(f, "{}: {}", title, total)?;
        } else {
            writeln!(f, "{}", report.title.bold())?;
        }
        if let Some(entries) = &report.entries {
            for (name, hours) in entries {
                let hours = format_hours(hours);
                writeln!(f, "{}: {}", name, hours)?;
            }
        }
        Ok(())
    }
}

fn format_hours(td: &chrono::TimeDelta) -> String {
    format!(
        "{}",
        ((td.num_minutes() as f32) / 60.0 * 100.0).round() / 100.0
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn by_date_empty() {
        let log = timelog::Log(vec![]);
        let report = Report::by_date(&log, Fill::Sparse);
        assert_eq!(
            Report {
                title: "By date".to_owned(),
                entries: Some(vec![]),
                total: None
            },
            report
        );
        let report = Report::by_date(&log, Fill::Padded);
        assert_eq!(
            Report {
                title: "By date".to_owned(),
                entries: Some(vec![]),
                total: None
            },
            report
        );
    }

    #[test]
    fn by_date_one_date() {
        let log = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
        "});
        let report = Report::by_date(&log, Fill::Sparse);
        assert_eq!(
            Report {
                title: "By date".to_owned(),
                entries: Some(vec![(
                    date(2024, 2, 13).to_string(),
                    chrono::TimeDelta::hours(3)
                )]),
                total: None
            },
            report
        );
    }

    #[test]
    fn by_date_multiple_dates() {
        let log = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
            ## 2024-02-14
            * 9-10 ABC
            * 10-11 DEF
        "});
        let report = Report::by_date(&log, Fill::Sparse);
        assert_eq!(
            Report {
                title: "By date".to_owned(),
                entries: Some(vec![
                    (date(2024, 2, 13).to_string(), chrono::TimeDelta::hours(3)),
                    (date(2024, 2, 14).to_string(), chrono::TimeDelta::hours(2)),
                ]),
                total: None
            },
            report
        );
    }

    #[test]
    fn by_date_multiple_dates_sparse() {
        let log = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
            ## 2024-02-17
            * 9-10 ABC
            * 10-11 DEF
        "});
        let report = Report::by_date(&log, Fill::Sparse);
        assert_eq!(
            Report {
                title: "By date".to_owned(),
                entries: Some(vec![
                    (date(2024, 2, 13).to_string(), chrono::TimeDelta::hours(3)),
                    (date(2024, 2, 17).to_string(), chrono::TimeDelta::hours(2)),
                ]),
                total: None
            },
            report
        );
    }

    #[test]
    fn by_date_multiple_dates_padded() {
        let log = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            * 11-12 ABC
            ## 2024-02-17
            * 9-10 ABC
            * 10-11 DEF
        "});
        let report = Report::by_date(&log, Fill::Padded);
        assert_eq!(
            Report {
                title: "By date".to_owned(),
                entries: Some(vec![
                    (date(2024, 2, 13).to_string(), chrono::TimeDelta::hours(3)),
                    (date(2024, 2, 14).to_string(), chrono::TimeDelta::zero()),
                    (date(2024, 2, 15).to_string(), chrono::TimeDelta::zero()),
                    (date(2024, 2, 16).to_string(), chrono::TimeDelta::zero()),
                    (date(2024, 2, 17).to_string(), chrono::TimeDelta::hours(2)),
                ]),
                total: None
            },
            report
        );
    }

    #[test]
    fn by_project_empty() {
        let log = timelog::Log(vec![]);
        let report = Report::by_project(&log);
        assert_eq!(
            Report {
                title: "By project".to_owned(),
                entries: Some(vec![]),
                total: None
            },
            report
        );
        let report = Report::by_project(&log);
        assert_eq!(
            Report {
                title: "By project".to_owned(),
                entries: Some(vec![]),
                total: None
            },
            report
        );
    }

    #[test]
    fn by_project_one_project() {
        let log = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 ABC
            ## 2024-02-14
            * 9-10 ABC
        "});
        let report = Report::by_project(&log);
        assert_eq!(
            Report {
                title: "By project".to_owned(),
                entries: Some(vec![("ABC".to_string(), chrono::TimeDelta::hours(3))]),
                total: None
            },
            report
        );
    }

    #[test]
    fn by_project_multiple_projects() {
        let log = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            ## 2024-02-14
            * 9-10 ABC
        "});
        let report = Report::by_project(&log);
        assert_eq!(
            Report {
                title: "By project".to_owned(),
                entries: Some(vec![
                    ("ABC".to_string(), chrono::TimeDelta::hours(2)),
                    ("DEF".to_string(), chrono::TimeDelta::hours(1)),
                ]),
                total: None
            },
            report
        );
    }

    #[test]
    fn total() {
        let log = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            ## 2024-02-14
            * 9-10 ABC
        "});
        let report = Report::total(&log);
        assert_eq!(
            Report {
                title: "Total".to_owned(),
                entries: None,
                total: Some(chrono::TimeDelta::hours(3))
            },
            report
        );
    }

    #[test]
    fn by_date_by_project_sparse() {
        let log = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            ## 2024-02-16
            * 9-10 ABC
        "});
        let report = Report::by_date_by_project(&log, Fill::Sparse);
        assert_eq!(
            vec![
                Report {
                    title: "2024-02-13".to_owned(),
                    entries: Some(vec![
                        ("ABC".to_string(), chrono::TimeDelta::hours(1)),
                        ("DEF".to_string(), chrono::TimeDelta::hours(1)),
                    ]),
                    total: Some(chrono::TimeDelta::hours(2))
                },
                Report {
                    title: "2024-02-16".to_owned(),
                    entries: Some(vec![("ABC".to_string(), chrono::TimeDelta::hours(1)),]),
                    total: Some(chrono::TimeDelta::hours(1))
                }
            ],
            report
        );
    }

    #[test]
    fn by_date_by_project_padded() {
        let log = timelog::Log::parse(indoc::indoc! {"
            ## 2024-02-13
            * 9-10 ABC
            * 10-11 DEF
            ## 2024-02-16
            * 9-10 ABC
        "});
        let report = Report::by_date_by_project(&log, Fill::Padded);
        assert_eq!(
            vec![
                Report {
                    title: "2024-02-13".to_owned(),
                    entries: Some(vec![
                        ("ABC".to_string(), chrono::TimeDelta::hours(1)),
                        ("DEF".to_string(), chrono::TimeDelta::hours(1)),
                    ]),
                    total: Some(chrono::TimeDelta::hours(2))
                },
                Report {
                    title: "2024-02-14".to_owned(),
                    entries: Some(vec![]),
                    total: Some(chrono::TimeDelta::hours(0))
                },
                Report {
                    title: "2024-02-15".to_owned(),
                    entries: Some(vec![]),
                    total: Some(chrono::TimeDelta::hours(0))
                },
                Report {
                    title: "2024-02-16".to_owned(),
                    entries: Some(vec![("ABC".to_string(), chrono::TimeDelta::hours(1)),]),
                    total: Some(chrono::TimeDelta::hours(1))
                }
            ],
            report
        );
    }

    #[test]
    fn fmt_as_text_emtpy() {
        let report = Report {
            title: "By project".to_owned(),
            entries: None,
            total: None,
        };
        let result = format!("{}", report.text());
        let expected = indoc::indoc! {"
            \u{1b}[1mBy project\u{1b}[0m
        "};
        assert_eq!(expected, result);
    }

    #[test]
    fn fmt_as_text_with_entries() {
        let report = Report {
            title: "By project".to_owned(),
            entries: Some(vec![
                ("ABC".to_string(), chrono::TimeDelta::hours(2)),
                ("DEF".to_string(), chrono::TimeDelta::hours(1)),
            ]),
            total: None,
        };
        let result = format!("{}", report.text());
        let expected = indoc::indoc! {"
            \u{1b}[1mBy project\u{1b}[0m
            ABC: 2
            DEF: 1
        "};
        assert_eq!(expected, result);
    }

    #[test]
    fn fmt_as_text_with_total() {
        let report = Report {
            title: "Total".to_owned(),
            entries: None,
            total: Some(chrono::TimeDelta::hours(3)),
        };
        let result = format!("{}", report.text());
        let expected = indoc::indoc! {"
            \u{1b}[1mTotal\u{1b}[0m: 3
        "};
        assert_eq!(expected, result);
    }

    #[test]
    fn fmt_as_text_with_entries_and_total() {
        let report = Report {
            title: "By project".to_owned(),
            entries: Some(vec![
                ("ABC".to_string(), chrono::TimeDelta::hours(2)),
                ("DEF".to_string(), chrono::TimeDelta::hours(1)),
            ]),
            total: Some(chrono::TimeDelta::hours(3)),
        };
        let result = format!("{}", report.text());
        let expected = indoc::indoc! {"
            \u{1b}[1mBy project\u{1b}[0m: 3
            ABC: 2
            DEF: 1
        "};
        assert_eq!(expected, result);
    }
}
