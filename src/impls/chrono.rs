use super::{BaseGenerator, DummyGenerator};
use crate::Serialize;
use chrono::{DateTime, TimeZone};
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
