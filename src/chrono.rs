use std::fmt::{self, Display, Formatter, Write};

use chrono::{format::{Fixed, Item, Numeric, Pad}, TimeZone};

use super::MySqlLiteral;

const DATE_FORMAT: &[Item<'static>] = &[
    Item::Numeric(Numeric::Year, Pad::Zero),
    Item::Literal("-"),
    Item::Numeric(Numeric::Month, Pad::Zero),
    Item::Literal("-"),
    Item::Numeric(Numeric::Day, Pad::Zero),
];

const TIME_FORMAT: &[Item<'static>] = &[
    Item::Numeric(Numeric::Hour, Pad::Zero),
    Item::Literal(":"),
    Item::Numeric(Numeric::Minute, Pad::Zero),
    Item::Literal(":"),
    Item::Numeric(Numeric::Second, Pad::Zero),
];

const TIME_ZONE_FORMAT: &[Item<'static>] = &[
    Item::Fixed(Fixed::TimezoneOffsetColon)
];

fn date(f: &mut Formatter<'_>, date: chrono::NaiveDate) -> fmt::Result {
    date.format_with_items(DATE_FORMAT.iter()).fmt(f)
}

fn time(f: &mut Formatter<'_>, time: chrono::NaiveTime) -> fmt::Result {
    time.format_with_items(TIME_FORMAT.iter()).fmt(f)
}

impl MySqlLiteral for chrono::NaiveDate {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('\'')?;
        date(f, *self)?;
        f.write_char('\'')
    }
}

impl MySqlLiteral for chrono::NaiveTime {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('\'')?;
        time(f, *self)?;
        f.write_char('\'')
    }
}

impl MySqlLiteral for chrono::NaiveDateTime {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('\'')?;
        date(f, self.date())?;
        f.write_str(" ")?;
        time(f, self.time())?;
        f.write_char('\'')
    }
}

impl<Tz> MySqlLiteral for chrono::DateTime<Tz>
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let naive = self.naive_utc();
        f.write_char('\'')?;
        date(f, naive.date())?;
        f.write_str(" ")?;
        time(f, naive.time())?;
        self.format_with_items(TIME_ZONE_FORMAT.iter()).fmt(f)?;
        f.write_char('\'')
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, FixedOffset};

    use crate::Safe;

    #[test]
    fn test() {
        use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Utc};

        let date = NaiveDate::from_ymd_opt(1995, 8, 9).unwrap();
        let time = NaiveTime::from_hms_opt(12, 4, 6).unwrap();
        let datetime = NaiveDateTime::new(date, time);
        let utc_datetime = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
        let jst_datetime = DateTime::<FixedOffset>::from_naive_utc_and_offset(datetime, FixedOffset::east_opt(9 * 3600).unwrap());

        assert_eq!(Safe(date).to_string(), "'1995-08-09'");
        assert_eq!(Safe(time).to_string(), "'12:04:06'");
        assert_eq!(Safe(datetime).to_string(), "'1995-08-09 12:04:06'");
        assert_eq!(Safe(utc_datetime).to_string(), "'1995-08-09 12:04:06+00:00'");
        assert_eq!(Safe(jst_datetime).to_string(), "'1995-08-09 12:04:06+09:00'");
    }
}
