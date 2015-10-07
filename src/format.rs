//! Calendar formatting utilities.

use date::{ weekday, ByWeek, Date, DateRange };
use chrono::Datelike;
use std::iter::{ once, repeat, Chain, Map, Once, Repeat, Take };

type FnFormatWeek = fn(DateRange) -> String;
pub type MonthLayout =
    Chain<
        Once<String>,
        Chain<
            Map<ByWeek, FnFormatWeek>,
            Take<Repeat<String>>>>;

pub fn layout_month(month: DateRange) -> MonthLayout {
    let week_count   = month.by_week().count();
    let title        = once(month_title(month.start));
    let padding_item = repeat(" ").take(22).collect::<String>();
    let padding      = repeat(padding_item).take(6 - week_count);

    title.chain(month.by_week().map(format_week as FnFormatWeek).chain(padding))
}

fn format_day(date: Date) -> String {
    format!("{: >3}", date.day())
}

fn format_week(week: DateRange) -> String {
    let pad_left  = weekday(week.start) * 3;
    let pad_right = (6 - weekday(week.end.pred())) * 3;

    let mut result = String::with_capacity(22);
    result.extend(repeat(" ").take(pad_left as usize));
    result.extend(week.map(format_day));
    result.extend(repeat(" ").take(pad_right as usize));
    result.push_str(" ");

    result
}

fn month_title(date: Date) -> String {
    format!("{: ^22}", format!("{}", date.format("%B")))
}

//------------------------------------------------------------------------------

#[cfg(test)]
use chrono::{ TimeZone, UTC };

#[test]
fn layout_month_returns_an_iterator_of_formatted_weeks() {
    let month = DateRange::new(UTC.ymd(2015, 1, 1), UTC.ymd(2015, 2, 1));
    let mut layout = layout_month(month);

    assert_eq!(layout.next().unwrap(), "       January        ");
    assert_eq!(layout.next().unwrap(), "           1  2  3  4 ");
    assert_eq!(layout.next().unwrap(), "  5  6  7  8  9 10 11 ");
    assert_eq!(layout.next().unwrap(), " 12 13 14 15 16 17 18 ");
    assert_eq!(layout.next().unwrap(), " 19 20 21 22 23 24 25 ");
    assert_eq!(layout.next().unwrap(), " 26 27 28 29 30 31    ");
    assert_eq!(layout.next().unwrap(), "                      ");
    assert_eq!(layout.next(), None);

    let month = DateRange::new(UTC.ymd(2010, 2, 1), UTC.ymd(2010, 3, 1));
    let mut layout = layout_month(month);

    assert_eq!(layout.next().unwrap(), "       February       ");
    assert_eq!(layout.next().unwrap(), "  1  2  3  4  5  6  7 ");
    assert_eq!(layout.next().unwrap(), "  8  9 10 11 12 13 14 ");
    assert_eq!(layout.next().unwrap(), " 15 16 17 18 19 20 21 ");
    assert_eq!(layout.next().unwrap(), " 22 23 24 25 26 27 28 ");
    assert_eq!(layout.next().unwrap(), "                      ");
    assert_eq!(layout.next().unwrap(), "                      ");
    assert_eq!(layout.next(), None);
}

#[test]
fn format_day_formats_day() {
    assert_eq!(format_day(UTC.ymd(2015, 1,  1)), "  1");
    assert_eq!(format_day(UTC.ymd(2015, 2, 11)), " 11");
}

#[test]
fn format_week_formats_week() {
    let week0 = DateRange::new(UTC.ymd(2015, 1, 1),  UTC.ymd(2015, 1,  5));
    let week1 = DateRange::new(UTC.ymd(2015, 1, 5),  UTC.ymd(2015, 1, 12));
    let week4 = DateRange::new(UTC.ymd(2015, 1, 26), UTC.ymd(2015, 2, 1));

    assert_eq!(format_week(week0), "           1  2  3  4 ");
    assert_eq!(format_week(week1), "  5  6  7  8  9 10 11 ");
    assert_eq!(format_week(week4), " 26 27 28 29 30 31    ");
}

#[test]
fn month_title_formats_month_name() {
    assert_eq!(month_title(UTC.ymd(2015, 1, 1)), "       January        ");
}
