mod daterange;

pub use daterange::DateRange;
use indexmap::map::IndexMap;

pub fn pad_dates<T: std::default::Default>(
    map: &mut IndexMap<chrono::NaiveDate, T>,
    range: Option<DateRange>,
) {
    if map.is_empty() {
        return;
    }
    let range = range.unwrap_or_else(|| {
        let start = map.first().unwrap().0;
        let end = map.last().unwrap().0;
        DateRange(*start, *end)
    });
    map.retain(|date, _| range.contains(date));
    for date in range {
        if map.get(&date).is_none() {
            map.insert(date, Default::default());
        }
    }
    map.sort_keys();
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::map::IndexMap;

    fn date(y: i32, m: u32, d: u32) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn pad_dates_empty() {
        let mut map: IndexMap<chrono::NaiveDate, &str> = IndexMap::new();
        let expected: IndexMap<chrono::NaiveDate, &str> = IndexMap::new();
        pad_dates(&mut map, None);
        assert_eq!(expected, map)
    }

    #[test]
    fn pad_dates_consecutive() {
        let mut map = IndexMap::from([
            (date(2024, 2, 13), ""),
            (date(2024, 2, 14), ""),
            (date(2024, 2, 15), ""),
        ]);
        let expected = IndexMap::from([
            (date(2024, 2, 13), ""),
            (date(2024, 2, 14), ""),
            (date(2024, 2, 15), ""),
        ]);
        pad_dates(&mut map, None);
        assert_eq!(expected, map)
    }

    #[test]
    fn pad_dates_gap() {
        let mut map = IndexMap::from([
            (date(2024, 2, 13), ""),
            // 2024-02-14 is missing
            (date(2024, 2, 15), ""),
        ]);
        let expected = IndexMap::from([
            (date(2024, 2, 13), ""),
            (date(2024, 2, 14), ""),
            (date(2024, 2, 15), ""),
        ]);
        pad_dates(&mut map, None);
        assert_eq!(expected, map)
    }

    #[test]
    fn pad_dates_multiple_gaps() {
        let mut map = IndexMap::from([
            // 2024-02-12 is missing
            (date(2024, 2, 13), ""),
            // 2024-02-14 is missing
            (date(2024, 2, 15), ""),
            // 2024-02-16 is missing
            // 2024-02-17 is missing
            (date(2024, 2, 18), ""),
        ]);
        let expected = IndexMap::from([
            // 2024-02-12 is not expected
            (date(2024, 2, 13), ""),
            (date(2024, 2, 14), ""),
            (date(2024, 2, 15), ""),
            (date(2024, 2, 16), ""),
            (date(2024, 2, 17), ""),
            (date(2024, 2, 18), ""),
        ]);
        pad_dates(&mut map, None);
        assert_eq!(expected, map)
    }

    #[test]
    fn pad_dates_custom_range() {
        let range = DateRange(date(2024, 2, 12), date(2024, 2, 18));
        let mut map = IndexMap::from([
            // 2024-02-12 is missing
            (date(2024, 2, 13), ""),
            // 2024-02-14 is missing
            (date(2024, 2, 15), ""),
            // 2024-02-16 is missing
            // 2024-02-17 is missing
            // 2024-02-18 is missing
            (date(2024, 2, 18), ""),
        ]);
        let expected = IndexMap::from([
            (date(2024, 2, 12), ""),
            (date(2024, 2, 13), ""),
            (date(2024, 2, 14), ""),
            (date(2024, 2, 15), ""),
            (date(2024, 2, 16), ""),
            (date(2024, 2, 17), ""),
            (date(2024, 2, 18), ""),
        ]);
        pad_dates(&mut map, Some(range));
        assert_eq!(expected, map)
    }

    #[test]
    fn pad_dates_input_outside_custom_range() {
        let range = DateRange(date(2024, 2, 12), date(2024, 2, 16));
        let mut map = IndexMap::from([
            (date(2024, 2, 11), ""), // outside range
            // 2024-02-12 is missing
            (date(2024, 2, 13), ""),
            // 2024-02-14 is missing
            (date(2024, 2, 15), ""),
            // 2024-02-16 is missing
            // 2024-02-17 is outside range
            // 2024-02-18 is outside range
            (date(2024, 2, 18), ""), // outside range
        ]);
        let expected = IndexMap::from([
            (date(2024, 2, 12), ""),
            (date(2024, 2, 13), ""),
            (date(2024, 2, 14), ""),
            (date(2024, 2, 15), ""),
            (date(2024, 2, 16), ""),
        ]);
        pad_dates(&mut map, Some(range));
        assert_eq!(expected, map)
    }
}
