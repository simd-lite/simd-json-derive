use crate::{de, BaseGenerator, Deserialize, DummyGenerator, Result, Serialize, Tape, Write};

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
    fn from_tape(tape: &mut Tape<'input>) -> de::Result<Self>
    where
        Self: Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::String(s)) => Ok(String::from(s)),
            _ => Err(de::Error::expected_string()),
        }
    }
}

impl<'input> Deserialize<'input> for &'input str {
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> de::Result<Self>
    where
        Self: Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::String(s)) => Ok(s),
            _ => Err(de::Error::expected_string()),
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

// "Figure this out". <-- PS. you cant as no one manages str's memory,
// and its also dynamically sized so you cant really allocate it on the stack risking overflow
// even if you should dynamically allocate on the stack which rust can't
// you could do a Box<str> though
//
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
//             _ => Err(de::Error::simd(
//                 de::Error::expected_string(),
//             )),
//         }
//     }
// }

impl<'input> Deserialize<'input> for Box<str> {
    fn from_tape(tape: &mut Tape<'input>) -> de::Result<Self>
    where
        Self: Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::String(s)) => Ok(Box::from(s)),
            _ => Err(de::Error::expected_string()),
        }
    }
}
