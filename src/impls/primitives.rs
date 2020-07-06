use crate::*;
use std::convert::TryFrom;

impl Serialize for bool {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> Result
    where
        W: Write,
    {
        if *self {
            writer.write_all(b"true")
        } else {
            writer.write_all(b"false")
        }
    }
}

macro_rules! itoa {
    ($t:ty) => {
        impl Serialize for $t {
            #[inline]
            fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
            where
                W: Write,
            {
                itoa::write(writer, *self).map(|_| ())
            }
        }

        impl Deserialize for $t {
            fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
            where
                Self: std::marker::Sized + 'input,
            {
                match tape.next() {
                    Some(simd_json::Node::Static(simd_json::StaticNode::I64(i))) => {
                        <$t>::try_from(i).map_err(|_| {
                            simd_json::Error::generic(simd_json::ErrorType::ExpectedInteger)
                        })
                    }
                    Some(simd_json::Node::Static(simd_json::StaticNode::U64(i))) => {
                        <$t>::try_from(i).map_err(|_| {
                            simd_json::Error::generic(simd_json::ErrorType::ExpectedInteger)
                        })
                    }
                    #[cfg(feature = "128bit")]
                    Some(simd_json::Node::Static(simd_json::StaticNode::U128(i))) => {
                        <$t>::try_from(i).map_err(|_| {
                            simd_json::Error::generic(simd_json::ErrorType::ExpectedInteger)
                        })
                    }
                    #[cfg(feature = "128bit")]
                    Some(simd_json::Node::Static(simd_json::StaticNode::UI28(i))) => {
                        <$t>::try_from(i).map_err(|_| {
                            simd_json::Error::generic(simd_json::ErrorType::ExpectedInteger)
                        })
                    }
                    _ => Err(simd_json::Error::generic(
                        simd_json::ErrorType::ExpectedString,
                    )),
                }
            }
        }
    };
}

itoa!(i8);
itoa!(u8);
itoa!(i16);
itoa!(u16);
itoa!(i32);
itoa!(u32);
itoa!(i64);
itoa!(u64);
itoa!(usize);
itoa!(i128);
itoa!(u128);

macro_rules! ryu {
    ($t:ty) => {
        impl Serialize for $t {
            #[inline]
            fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
            where
                W: Write,
            {
                let mut buffer = ryu::Buffer::new();
                let s = buffer.format_finite(*self);
                writer.write_all(s.as_bytes())
            }
        }
    };
}
ryu!(f64);
ryu!(f32);

impl Deserialize for f64 {
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::Static(simd_json::StaticNode::F64(i))) => Ok(i),
            Some(simd_json::Node::Static(simd_json::StaticNode::I64(i))) => Ok(i as f64),
            Some(simd_json::Node::Static(simd_json::StaticNode::U64(i))) => Ok(i as f64),
            #[cfg(feature = "128bit")]
            Some(simd_json::Node::Static(simd_json::StaticNode::U128(i))) => Ok(i as f64),
            #[cfg(feature = "128bit")]
            Some(simd_json::Node::Static(simd_json::StaticNode::UI28(i))) => Ok(i as f64),
            _ => Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedString,
            )),
        }
    }
}

impl Deserialize for f32 {
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        match tape.next() {
            Some(simd_json::Node::Static(simd_json::StaticNode::F64(i))) => Ok(i as f32),
            Some(simd_json::Node::Static(simd_json::StaticNode::I64(i))) => Ok(i as f32),
            Some(simd_json::Node::Static(simd_json::StaticNode::U64(i))) => Ok(i as f32),
            #[cfg(feature = "128bit")]
            Some(simd_json::Node::Static(simd_json::StaticNode::U128(i))) => Ok(i as f32),
            #[cfg(feature = "128bit")]
            Some(simd_json::Node::Static(simd_json::StaticNode::UI28(i))) => Ok(i as f32),
            _ => Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedString,
            )),
        }
    }
}
