use std::{cmp, fmt};

lazy_static::lazy_static! {
    static ref ENTRY_RE: regex::Regex = regex::Regex::new(
        r#"(?x)^\s*
        (?<from_h>[012]?\d)(?::?(?<from_m>\d{2}))?
        (?:\s*-\s*|\s+)
        (?<until_h>[012]?\d)(?::?(?<until_m>\d{2}))?
        (?:\s*:\s*|\s+)
        (?:
            (?<project>\w{3,}+)
            |(?:"(?<quoted_project>.+?)")
        )
        \s*(?<notes>.+?)?\s*$"#
    )
    .unwrap();
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub from: chrono::NaiveDateTime,
    pub until: chrono::NaiveDateTime,
    pub project: String,
    pub notes: Option<String>,
}

impl Entry {
    pub fn parse(s: &str, date: &chrono::NaiveDate) -> Option<Self> {
        let cap = ENTRY_RE.captures(s)?;
        let from = time(&cap, "from_h", "from_m")?;
        let until = time(&cap, "until_h", "until_m")?;
        let project = cap.name("project").or(cap.name("quoted_project"))?;
        (from < until).then(|| Entry {
            from: chrono::NaiveDateTime::new(*date, from),
            until: chrono::NaiveDateTime::new(*date, until),
            project: project.as_str().into(),
            notes: cap.name("notes").map(|m| m.as_str().into()),
        })
    }
    pub fn duration(&self) -> chrono::TimeDelta {
        self.until.signed_duration_since(self.from)
    }
}

fn time(cap: &regex::Captures, h: &str, m: &str) -> Option<chrono::NaiveTime> {
    let h = time_part(cap.name(h))?;
    let m = time_part(cap.name(m))?;
    chrono::NaiveTime::from_hms_opt(h, m, 0)
}

fn time_part(s: Option<regex::Match>) -> Option<u32> {
    s.map_or(Some(0), |s| s.as_str().parse::<u32>().ok())
}

impl cmp::PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.until == other.until && self.project == other.project
    }
}

impl cmp::Eq for Entry {}

impl cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.from
            .cmp(&other.from)
            .then_with(|| self.until.cmp(&other.until))
            .then_with(|| self.project.cmp(&other.project))
    }
}

impl cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date = self.from.format("%Y-%m-%d");
        let from = self.from.format("%H:%M");
        let until = self.until.format("%H:%M");
        if let Some(notes) = &self.notes {
            write!(
                f,
                "{} | {} - {}: {} - {}",
                date, from, until, self.project, notes
            )
        } else {
            write!(f, "{} | {} - {}: {}", date, from, until, self.project)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static::lazy_static! {
        static ref DATE: chrono::NaiveDate = chrono::NaiveDate::from_ymd_opt(2024, 2, 13).unwrap();
    }

    fn datetime(h: u32, m: u32) -> chrono::NaiveDateTime {
        let time = chrono::NaiveTime::from_hms_opt(h, m, 0).unwrap();
        chrono::NaiveDateTime::new(*DATE, time)
    }

    #[test]
    fn unrelated() {
        let result = Entry::parse("some unrelated list item", &DATE);
        assert_eq!(None, result);
    }

    #[test]
    fn simple() {
        let result = Entry::parse("9:00 - 10:45: ABC", &DATE);
        assert_eq!(
            Some(Entry {
                from: datetime(9, 0),
                until: datetime(10, 45),
                project: "ABC".into(),
                notes: None
            }),
            result
        );
    }

    #[test]
    fn with_note() {
        let result = Entry::parse("9:00 - 10:45: DEF some notes here", &DATE);
        assert_eq!(
            Some(Entry {
                from: datetime(9, 0),
                until: datetime(10, 45),
                project: "DEF".into(),
                notes: Some("some notes here".into())
            }),
            result
        );
    }

    #[test]
    fn minimal() {
        let result = Entry::parse("9 1045 GHI", &DATE);
        assert_eq!(
            Some(Entry {
                from: datetime(9, 0),
                until: datetime(10, 45),
                project: "GHI".into(),
                notes: None
            }),
            result
        );
    }

    #[test]
    fn minimal_with_syntax() {
        let result = Entry::parse("9-1045:JKL", &DATE);
        assert_eq!(
            Some(Entry {
                from: datetime(9, 0),
                until: datetime(10, 45),
                project: "JKL".into(),
                notes: None
            }),
            result
        );
    }

    #[test]
    fn extra_whitespace() {
        let result = Entry::parse("   09:00  \t- \t10:45  :MNO", &DATE);
        assert_eq!(
            Some(Entry {
                from: datetime(9, 0),
                until: datetime(10, 45),
                project: "MNO".into(),
                notes: None
            }),
            result
        );
    }

    #[test]
    fn quoted_project() {
        let result = Entry::parse("0900 1045 \"Some project\" with notes", &DATE);
        assert_eq!(
            Some(Entry {
                from: datetime(9, 0),
                until: datetime(10, 45),
                project: "Some project".into(),
                notes: Some("with notes".into())
            }),
            result
        );
    }

    #[test]
    fn invalid_time() {
        let result = Entry::parse("9:00 - 24:15: ABC", &DATE);
        assert_eq!(None, result);
    }

    #[test]
    fn no_project() {
        let result = Entry::parse("9:00 - 10:45", &DATE);
        assert_eq!(None, result);
    }

    #[test]
    fn no_until() {
        let result = Entry::parse("9:00: ABC", &DATE);
        assert_eq!(None, result);
    }

    #[test]
    fn project_name_too_short() {
        let result = Entry::parse("9-10:AB", &DATE);
        assert_eq!(None, result);
    }

    #[test]
    fn cmp_from() {
        let a = Entry::parse("9-10:ABC", &DATE).unwrap();
        let b = Entry::parse("10-11:ABC", &DATE).unwrap();
        assert!(a < b);
        assert!(a <= b);
    }

    #[test]
    fn cmp_until() {
        let a = Entry::parse("9-10:ABC", &DATE).unwrap();
        let b = Entry::parse("9-11:ABC", &DATE).unwrap();
        assert!(a < b);
        assert!(a <= b);
    }

    #[test]
    fn cmp_project() {
        let a = Entry::parse("9-10:ABC", &DATE).unwrap();
        let b = Entry::parse("9-10:DEF", &DATE).unwrap();
        assert!(a < b);
        assert!(a <= b);
    }

    #[test]
    fn eq() {
        let a = Entry::parse("9-10:ABC", &DATE).unwrap();
        let b = Entry::parse("9-10:ABC", &DATE).unwrap();
        assert!(a == b);
        let a = Entry::parse("9-10:ABC", &DATE).unwrap();
        let b = Entry::parse("9:00 - 10:00: ABC", &DATE).unwrap();
        assert!(a == b);
        let a = Entry::parse("9-10:ABC", &DATE).unwrap();
        let b = Entry::parse("9-10:ABC notes are not taken into account", &DATE).unwrap();
        assert!(a == b);
    }
}
