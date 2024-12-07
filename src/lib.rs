use std::{
    fmt::{self, Display, Formatter, Write},
    num::Wrapping,
    ops::Deref,
};

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "time")]
mod time;

const LOOKUP_TABLE: [u8; 128] = {
    let mut table: [u8; 128] = [0; 128];
    table[b'"' as usize] = b'"';
    table[b'\0' as usize] = b'0';
    table[b'\'' as usize] = b'\'';
    table[b'\\' as usize] = b'\\';
    table[b'\n' as usize] = b'n';
    table[b'\r' as usize] = b'r';
    table[26] = b'Z';
    table
};

pub struct EscapeStr<'a>(&'a str);

impl Display for EscapeStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for chr in self.0.chars() {
            if chr.is_ascii() {
                let c = LOOKUP_TABLE[chr as usize];
                if c != 0 {
                    f.write_char('\\')?;
                    f.write_char(c as char)?;
                    continue;
                }
            }
            f.write_char(chr)?;
        }
        Ok(())
    }
}

pub trait MySqlLiteral {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result;
}

impl MySqlLiteral for str {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('\'')?;
        EscapeStr(self).fmt(f)?;
        f.write_char('\'')?;
        Ok(())
    }
}

impl MySqlLiteral for String {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.as_str().to_mysql_literal(f)
    }
}

impl MySqlLiteral for char {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('\'')?;
        let mut buf = [0; 4];
        let s = self.encode_utf8(&mut buf);
        EscapeStr(s).fmt(f)?;
        f.write_char('\'')?;
        Ok(())
    }
}

impl<T> MySqlLiteral for &'_ T
where
    T: MySqlLiteral + ?Sized,
{
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <T as MySqlLiteral>::to_mysql_literal(self, f)
    }
}

impl MySqlLiteral for bool {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(if *self { "TRUE" } else { "FALSE" })?;
        Ok(())
    }
}

impl MySqlLiteral for () {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("NULL")?;
        Ok(())
    }
}

impl MySqlLiteral for f32 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_nan() || self.is_infinite() {
            f.write_str("NULL")
        } else {
            self.fmt(f)
        }
    }
}

impl MySqlLiteral for f64 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_nan() || self.is_infinite() {
            f.write_str("NULL")
        } else {
            self.fmt(f)
        }
    }
}

impl MySqlLiteral for i8 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for i16 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for i32 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for i64 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for i128 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for isize {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for u8 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for u16 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for u32 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for u64 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for u128 {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl MySqlLiteral for usize {
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl<T> MySqlLiteral for Wrapping<T>
where
    T: MySqlLiteral,
{
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.to_mysql_literal(f)
    }
}

impl<T> MySqlLiteral for [T]
where
    T: MySqlLiteral,
{
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut iter = self.iter();
        if let Some(value) = iter.next() {
            value.to_mysql_literal(f)?;
            for value in iter {
                f.write_str(", ")?;
                value.to_mysql_literal(f)?;
            }
        }
        Ok(())
    }
}

impl<const N: usize, T> MySqlLiteral for [T; N]
where
    T: MySqlLiteral,
{
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.as_slice().to_mysql_literal(f)
    }
}

impl<T> MySqlLiteral for Option<T>
where
    T: MySqlLiteral,
{
    fn to_mysql_literal(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(value) = self {
            value.to_mysql_literal(f)
        } else {
            ().to_mysql_literal(f)
        }
    }
}

pub struct Safe<T>(pub T);

impl<T> Display for Safe<T>
where
    T: MySqlLiteral,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.to_mysql_literal(f)
    }
}

pub struct Blob<T>(pub T);

impl<T> Display for Blob<T>
where
    T: Deref<Target = [u8]>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            f.write_str("''")
        } else {
            f.write_str("0x")?;
            for byte in &*self.0 {
                write!(f, "{:02X}", byte)?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Blob, Safe};

    #[test]
    fn test() {
        let sql = format!(
            "select * from table where column = {}",
            Safe("unsafe' value")
        );
        assert_eq!(sql, "select * from table where column = 'unsafe\\' value'");

        let sql = format!("select * from table where column = {}", Safe(true));
        assert_eq!(sql, "select * from table where column = TRUE");

        let sql = format!("select * from table where column = {}", Safe(None::<i32>));
        assert_eq!(sql, "select * from table where column = NULL");

        let sql = format!("select * from table where column = {}", Safe(Some(42)));
        assert_eq!(sql, "select * from table where column = 42");

        let sql = format!("select * from table where column in ({})", Safe([1, 2, 3]));
        assert_eq!(sql, "select * from table where column in (1, 2, 3)");

        let sql = format!(
            "select * from table where column in ({})",
            Safe(["a", "b", "c"])
        );
        assert_eq!(sql, "select * from table where column in ('a', 'b', 'c')");

        let sql = format!(
            "select * from table where column in ({})",
            Safe([String::from("foo")])
        );
        assert_eq!(sql, "select * from table where column in ('foo')");

        let sql = format!(
            "select * from table where column = {}",
            Blob(&[0, 1, 2, 3][..])
        );
        assert_eq!(sql, "select * from table where column = 0x00010203");

        let sql = format!("select * from table where column = {}", Blob(&[][..]));
        assert_eq!(sql, "select * from table where column = ''");

        let sql = format!(
            "select * from table where column = {}",
            Blob(vec![0, 1, 2, 3])
        );
        assert_eq!(sql, "select * from table where column = 0x00010203");
    }
}
