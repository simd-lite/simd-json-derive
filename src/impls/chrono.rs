use super::{BaseGenerator, DummyGenerator};
use crate::{de, Deserialize, Serialize, Tape, Write};
use chrono::{DateTime, FixedOffset, TimeZone};
use std::{fmt, io};

impl<Tz: TimeZone> Serialize for DateTime<Tz> {
    /// Serialize into a rfc3339 time string
    ///
    /// See [the `serde` module](./serde/index.html) for alternate
    /// serializations.
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        struct FormatWrapped<'a, D: 'a> {
            inner: &'a D,
        }

        impl<D: fmt::Debug> fmt::Display for FormatWrapped<'_, D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.inner.fmt(f)
            }
        }

        // Debug formatting is correct RFC3339, and it allows Zulu.
        DummyGenerator(writer).write_string(&format!("{}", FormatWrapped { inner: &self }))
    }
}

impl<'input> Deserialize<'input> for DateTime<FixedOffset> {
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> de::Result<Self>
    where
        Self: Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::String(s)) => DateTime::parse_from_rfc2822(s)
                .map_err(|e| de::Error::custom(format!("Invalid date string `{s}`: {e}"))),
            _ => Err(de::Error::expected_string()),
        }
    }
}
