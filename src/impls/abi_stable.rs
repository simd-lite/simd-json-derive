use crate::*;
use abi_stable::std_types::{RVec, RString, ROption::{self, RSome, RNone}};

impl Serialize for RString {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> Result
    where
        W: Write,
    {
        DummyGenerator(writer).write_string(self)
    }
}

impl<'input> Deserialize<'input> for RString {
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::String(s)) => Ok(RString::from(s)),
            _ => Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedString,
            )),
        }
    }
}

impl<T> Serialize for ROption<T>
where
    T: Serialize,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        if let RSome(e) = self {
            e.json_write(writer)
        } else {
            writer.write_all(b"null")
        }
    }
}

impl<'input, T> Deserialize<'input> for ROption<T>
where
    T: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Static(simd_json::StaticNode::Null)) = tape.peek() {
            tape.next();
            Ok(RNone)
        } else {
            Ok(RSome(T::from_tape(tape)?))
        }
    }
}

impl<'input, T> Deserialize<'input> for RVec<T>
where
    T: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::Array(size, _)) => {
                let mut v = RVec::with_capacity(size);
                for _ in 0..size {
                    v.push(T::from_tape(tape)?)
                }
                Ok(v)
            }
            _other => Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedArray,
            )),
        }
    }
}
