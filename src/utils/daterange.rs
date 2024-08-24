pub struct DateRange(pub chrono::NaiveDate, pub chrono::NaiveDate);

impl DateRange {
    pub fn contains(&self, reference: &chrono::NaiveDate) -> bool {
        reference >= &self.0 && reference <= &self.1
    }
}

static ONE_DAY: chrono::Days = chrono::Days::new(1);

impl Iterator for DateRange {
    type Item = chrono::NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + ONE_DAY;
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
    fn contains() {
        let range = DateRange(date(2024, 2, 12), date(2024, 2, 16));
        assert!(!range.contains(&date(2024, 2, 11)));
        assert!(range.contains(&date(2024, 2, 12)));
        assert!(range.contains(&date(2024, 2, 14)));
        assert!(range.contains(&date(2024, 2, 16)));
        assert!(!range.contains(&date(2024, 2, 17)));
    }

    #[test]
    fn iterator() {
        let range = DateRange(date(2024, 2, 12), date(2024, 2, 16));
        let result: Vec<String> = range.into_iter().map(|date| date.to_string()).collect();
        let expected = vec![
            "2024-02-12",
            "2024-02-13",
            "2024-02-14",
            "2024-02-15",
            "2024-02-16",
        ];
        assert_eq!(expected, result);
    }
}
