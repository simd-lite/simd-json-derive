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

impl<'input> Deserialize<'input> for String {
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::String(s)) => Ok(String::from(s)),
            _ => Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedString,
            )),
        }
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

// Figure this out.
// impl<'input> Deserialize<'input> for str {
//     #[inline]
//     fn from_tape(
//         tape: &mut Tape<'input>,
//     ) -> simd_json::Result<Self>
//     where
//         Self: std::marker::Sized + 'input,
//     {
//         match tape.next() {
//             Some(simd_json::Node::String(s)) => Ok(s),
//             _ => Err(simd_json::Error::generic(
//                 simd_json::ErrorType::ExpectedString,
//             )),
//         }
//     }
// }
