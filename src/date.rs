//! Utilities for working with dates.

use chrono::{ Datelike, TimeZone, UTC };

/// Date
pub type Date = ::chrono::Date<UTC>;

/// Which week in the year the date belongs to. Week number start at zero.
fn week_number(date: &Date) -> u32 {
    date.isoweekdate().1 - 1
}

pub fn weekday(date: Date) -> u32 {
    date.weekday().num_days_from_monday()
}

//------------------------------------------------------------------------------

/// Range of dates.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DateRange {
    pub start: Date,
    pub end:   Date
}

impl DateRange {
    pub fn new(start: Date, end: Date) -> DateRange {
        DateRange{ start: start, end: end }
    }

    fn empty(&self) -> bool {
        self.start >= self.end
    }

    pub fn by_month(self) -> ByMonth {
        self.group_by(Date::month)
    }

    pub fn by_week(self) -> ByWeek {
        self.group_by(week_number)
    }

    fn group_by<K, F>(self, key: F) -> GroupBy<F>
        where F: FnMut(&Date) -> K, K: PartialEq
    {
        GroupBy{ dates: self, key: key }
    }
}

impl Iterator for DateRange {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        if self.empty() {
            None
        } else {
            let result = Some(self.start);
            self.start = self.start.succ();
            result
        }
    }
}

//------------------------------------------------------------------------------

struct GroupBy<F> {
    dates: DateRange,
    key: F
}

pub type ByMonth = GroupBy<fn(&Date) -> u32>;
pub type ByWeek  = GroupBy<fn(&Date) -> u32>;

impl<K, F> Iterator for GroupBy<F> where F: FnMut(&Date) -> K, K: PartialEq {
    type Item = DateRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dates.empty() { return None; }

        let start     = self.dates.start;
        let start_key = (self.key)(&start);

        let end = self.dates.skip_while(|&d| (self.key)(&d) == start_key)
                            .next()
                            .unwrap_or(self.dates.end);

        self.dates = DateRange::new(end, self.dates.end);

        Some(DateRange::new(start, end))
    }
}

/// Returns a range of all dates in the given year.
pub fn dates(year: i32) -> DateRange {
    DateRange::new(UTC.ymd(year, 1, 1), UTC.ymd(year + 1, 1, 1))
}

//------------------------------------------------------------------------------

#[test]
fn week_number_returns_week_number_of_the_date() {
    assert_eq!(week_number(&UTC.ymd(2015, 1,  1)), 0);
    assert_eq!(week_number(&UTC.ymd(2015, 1,  2)), 0);
    assert_eq!(week_number(&UTC.ymd(2015, 1,  3)), 0);
    assert_eq!(week_number(&UTC.ymd(2015, 1,  4)), 0);

    assert_eq!(week_number(&UTC.ymd(2015, 1,  5)), 1);
    assert_eq!(week_number(&UTC.ymd(2015, 1, 13)), 2);
}

#[test]
fn date_range_can_be_iterated() {
    let range = DateRange::new(UTC.ymd(2015, 1, 1), UTC.ymd(2015, 1, 4));

    let actual = range.collect::<Vec<_>>();
    let expected = vec![UTC.ymd(2015, 1, 1),
                        UTC.ymd(2015, 1, 2),
                        UTC.ymd(2015, 1, 3)];

    assert_eq!(actual, expected);
}

#[test]
fn by_month_groups_date_range_by_months() {
    let     range  = DateRange::new(UTC.ymd(2015, 1, 10), UTC.ymd(2015, 3, 10));
    let mut months = range.by_month();

    assert_eq!(months.next().unwrap(),
               DateRange::new(UTC.ymd(2015, 1, 10), UTC.ymd(2015, 2, 1)));

    assert_eq!(months.next().unwrap(),
               DateRange::new(UTC.ymd(2015, 2, 1), UTC.ymd(2015, 3, 1)));

    assert_eq!(months.next().unwrap(),
               DateRange::new(UTC.ymd(2015, 3, 1), UTC.ymd(2015, 3, 10)));

    assert_eq!(months.next(), None);
}

#[test]
fn by_week_groups_date_range_by_weeks() {
    let range     = DateRange::new(UTC.ymd(2015, 1, 1), UTC.ymd(2015, 1, 17));
    let mut weeks = range.by_week();

    assert_eq!(weeks.next().unwrap(),
               DateRange::new(UTC.ymd(2015, 1, 1), UTC.ymd(2015, 1, 5)));

    assert_eq!(weeks.next().unwrap(),
               DateRange::new(UTC.ymd(2015, 1, 5), UTC.ymd(2015, 1, 12)));

    assert_eq!(weeks.next().unwrap(),
               DateRange::new(UTC.ymd(2015, 1, 12), UTC.ymd(2015, 1, 17)));

    assert_eq!(weeks.next(), None);
}

#[test]
fn dates_returns_all_dates_in_a_year() {
    let range = dates(2015);

    let actual = range.take(4).collect::<Vec<_>>();
    let expected = vec![ UTC.ymd(2015, 1, 1)
                       , UTC.ymd(2015, 1, 2)
                       , UTC.ymd(2015, 1, 3)
                       , UTC.ymd(2015, 1, 4)];
    assert_eq!(actual, expected);

    let actual = range.last().unwrap();
    let expected = UTC.ymd(2015, 12, 31);
    assert_eq!(actual, expected);
}
