use super::{BaseGenerator, DummyGenerator};
use crate::*;
use chrono::{DateTime, FixedOffset, TimeZone};
use std::{fmt, io};

impl<Tz: TimeZone> Serialize for DateTime<Tz> {
    /// Serialize into a rfc3339 time string
    ///
    /// See [the `serde` module](./serde/index.html) for alternate
    /// serializations.
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        struct FormatWrapped<'a, D: 'a> {
            inner: &'a D,
        }

        impl<'a, D: fmt::Debug> fmt::Display for FormatWrapped<'a, D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.inner.fmt(f)
            }
        }

        // Debug formatting is correct RFC3339, and it allows Zulu.
        DummyGenerator(writer).write_string(&format!("{}", FormatWrapped { inner: &self }))
    }
}

impl Deserialize for DateTime<FixedOffset> {
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::String(s)) => DateTime::parse_from_rfc2822(s)
                .map_err(|_| simd_json::Error::generic(simd_json::ErrorType::ExpectedString)),
            _ => Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedString,
            )),
        }
    }
}
