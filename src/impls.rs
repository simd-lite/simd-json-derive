mod array;
#[cfg(feature = "impl-chrono")]
mod chrono;
mod collections;
mod deref;
mod primitives;
mod simdjson;
mod string;
mod tpl;
use crate::*;
use value_trait::generator::BaseGenerator;

struct DummyGenerator<W: Write>(W);
impl<W: Write> BaseGenerator for DummyGenerator<W> {
    type T = W;
    #[inline]
    fn get_writer(&mut self) -> &mut <Self as BaseGenerator>::T {
        &mut self.0
    }
    #[inline]
    fn write_min(&mut self, _: &[u8], _: u8) -> io::Result<()> {
        unimplemented!()
    }
}

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
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
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
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Object { len: 1, .. }) = tape.next() {
            match tape.next() {
                Some(simd_json::Node::String("Ok")) => Ok(Ok(TOk::from_tape(tape)?)),
                Some(simd_json::Node::String("Err")) => Ok(Err(TErr::from_tape(tape)?)),
                Some(simd_json::Node::String("ok")) => Ok(Ok(TOk::from_tape(tape)?)),
                Some(simd_json::Node::String("err")) => Ok(Err(TErr::from_tape(tape)?)),
                _ => Err(simd_json::Error::generic(simd_json::ErrorType::ExpectedMap)),
            }
        } else {
            Err(simd_json::Error::generic(simd_json::ErrorType::ExpectedMap))
        }
    }
}
