use crate::*;
use abi_stable::std_types::{
    RBox, RHashMap,
    ROption::{self, RNone, RSome},
    RString, RVec, Tuple2,
};

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

impl<K, V, H> Serialize for RHashMap<K, V, H>
where
    K: SerializeAsKey,
    V: Serialize,
    H: std::hash::BuildHasher,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> Result
    where
        W: Write,
    {
        let mut i = self.iter();
        if let Some(Tuple2(k, v)) = i.next() {
            if let Err(e) = writer.write_all(b"{") {
                return Err(e);
            };
            if let Err(e) = k.json_write(writer) {
                return Err(e);
            };
            if let Err(e) = writer.write_all(b":") {
                return Err(e);
            };
            if let Err(e) = v.json_write(writer) {
                return Err(e);
            };
            for Tuple2(k, v) in i {
                if let Err(e) = writer.write_all(b",") {
                    return Err(e);
                };
                if let Err(e) = k.json_write(writer) {
                    return Err(e);
                };
                if let Err(e) = writer.write_all(b":") {
                    return Err(e);
                };
                if let Err(e) = v.json_write(writer) {
                    return Err(e);
                };
            }
            writer.write_all(b"}")
        } else {
            writer.write_all(b"{}")
        }
    }
}

impl<'input, K, V, H> Deserialize<'input> for RHashMap<K, V, H>
where
    K: Deserialize<'input> + std::hash::Hash + Eq,
    V: Deserialize<'input>,
    H: std::hash::BuildHasher + Default,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Object(size, _)) = tape.next() {
            let mut v = RHashMap::with_capacity_and_hasher(size, H::default());
            for _ in 0..size {
                let k = K::from_tape(tape)?;
                v.insert(k, V::from_tape(tape)?);
            }
            Ok(v)
        } else {
            Err(simd_json::Error::generic(simd_json::ErrorType::ExpectedMap))
        }
    }
}

impl<'input, T> Deserialize<'input> for RBox<T>
where
    T: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        Ok(RBox::new(T::from_tape(tape)?))
    }
}
