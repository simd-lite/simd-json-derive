mod array;
#[cfg(feature = "impl-chrono")]
mod chrono;
mod collections;
mod deref;
mod primitives;
mod simdjson;
mod string;
mod tpl;
use crate::{de, io, Deserialize, DummyGenerator, Serialize, Tape, Write};
use value_trait::generator::BaseGenerator;

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        if let Some(e) = self {
            e.json_write(writer)
        } else {
            writer.write_all(b"null")
        }
    }
}

impl<'input, T> Deserialize<'input> for Option<T>
where
    T: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> de::Result<Self>
    where
        Self: Sized + 'input,
    {
        if let Some(simd_json::Node::Static(simd_json::StaticNode::Null)) = tape.peek() {
            tape.next();
            Ok(None)
        } else {
            Ok(Some(T::from_tape(tape)?))
        }
    }
}

impl<TOk, TErr> Serialize for std::result::Result<TOk, TErr>
where
    TOk: Serialize,
    TErr: Serialize,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        match self {
            Ok(e) => {
                writer.write_all(b"{\"Ok\":")?;
                e.json_write(writer)?;
                writer.write_all(b"}")
            }
            Err(e) => {
                writer.write_all(b"{\"Err\":")?;
                e.json_write(writer)?;
                writer.write_all(b"}")
            }
        }
    }
}

impl<'input, TOk, TErr> Deserialize<'input> for std::result::Result<TOk, TErr>
where
    TOk: Deserialize<'input>,
    TErr: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> de::Result<Self>
    where
        Self: Sized + 'input,
    {
        if let Some(simd_json::Node::Object { len: 1, .. }) = tape.next() {
            match tape.next() {
                Some(simd_json::Node::String("Ok")) => Ok(Ok(TOk::from_tape(tape)?)),
                Some(simd_json::Node::String("Err")) => Ok(Err(TErr::from_tape(tape)?)),
                Some(simd_json::Node::String("ok")) => Ok(Ok(TOk::from_tape(tape)?)),
                Some(simd_json::Node::String("err")) => Ok(Err(TErr::from_tape(tape)?)),
                _ => Err(de::Error::custom("result not `Ok` or `Err`")),
            }
        } else {
            Err(de::Error::InvalidStructRepresentation)
        }
    }
}
