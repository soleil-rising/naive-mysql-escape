use std::fmt::{self, Formatter, Write};

use time::{Date, Time};

use super::MySqlLiteral;

fn date(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    write!(
        f,
        "{:04}-{:02}-{:02}",
        date.year(),
        date.month() as u8,
        date.day()
    )
}

fn time(f: &mut Formatter<'_>, time: Time) -> fmt::Result {
    write!(
        f,
        "{:02}:{:02}:{:02}",
        time.hour(),
        time.minute(),
        time.second()
    )
}

impl MySqlLiteral for time::Date {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('\'')?;
        date(f, *self)?;
        f.write_char('\'')
    }
}

impl MySqlLiteral for time::Time {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('\'')?;
        time(f, *self)?;
        f.write_char('\'')
    }
}

impl MySqlLiteral for time::PrimitiveDateTime {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('\'')?;
        date(f, self.date())?;
        f.write_str(" ")?;
        time(f, self.time())?;
        f.write_char('\'')
    }
}

impl MySqlLiteral for time::OffsetDateTime {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('\'')?;
        date(f, self.date())?;
        f.write_str(" ")?;
        time(f, self.time())?;
        let (h, m, _) = self.offset().as_hms();
        write!(f, "{:+03}:{:02}", h, m)?;
        f.write_char('\'')
    }
}

#[cfg(test)]
mod tests {
    use crate::Safe;

    use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

    #[test]
    fn test() {
        let date = Date::from_calendar_date(1995, Month::August, 9).unwrap();
        let time = Time::from_hms(12, 4, 6).unwrap();
        let datetime = PrimitiveDateTime::new(date, time);
        let utc_datetime = OffsetDateTime::new_utc(date, time);
        let jst_datetime =
            OffsetDateTime::new_in_offset(date, time, UtcOffset::from_hms(9, 0, 0).unwrap());

        assert_eq!(Safe(date).to_string(), "'1995-08-09'");
        assert_eq!(Safe(time).to_string(), "'12:04:06'");
        assert_eq!(Safe(datetime).to_string(), "'1995-08-09 12:04:06'");
        assert_eq!(
            Safe(utc_datetime).to_string(),
            "'1995-08-09 12:04:06+00:00'"
        );
        assert_eq!(
            Safe(jst_datetime).to_string(),
            "'1995-08-09 12:04:06+09:00'"
        );
    }
}
