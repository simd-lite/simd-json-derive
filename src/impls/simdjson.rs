use crate::Serialize;
use simd_json::{BorrowedValue, OwnedValue};
use value_trait::Writable;

impl Serialize for OwnedValue {
    fn json_write<W>(&self, writer: &mut W) -> crate::Result
    where
        W: std::io::Write,
    {
        self.write(writer)
    }
}
impl<'value> Serialize for BorrowedValue<'value> {
    fn json_write<W>(&self, writer: &mut W) -> crate::Result
    where
        W: std::io::Write,
    {
        self.write(writer)
    }
}
