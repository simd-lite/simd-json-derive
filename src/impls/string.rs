use crate::*;
impl Serialize for String {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> Result
    where
        W: Write,
    {
        DummyGenerator(writer).write_string(self)
    }
}

impl Serialize for str {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> Result
    where
        W: Write,
    {
        DummyGenerator(writer).write_string(self)
    }
}
