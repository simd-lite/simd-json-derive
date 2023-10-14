use crate::*;

impl Serialize for () {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> Result
    where
        W: Write,
    {
        writer.write_all(b"null")
    }
}

impl<'input> Deserialize<'input> for () {
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Static(simd_json::StaticNode::Null)) = tape.next() {
            Ok(())
        } else {
            Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedNull,
            ))
        }
    }
}

// takenn from https://docs.serde.rs/src/serde/ser/impls.rs.html#306

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> Serialize for ($($name,)+)
            where
                $($name: Serialize,)+
            {
                #[inline]
                fn json_write<W>(&self, writer: &mut W) -> Result
                where
                    W: Write,
                {
                    writer.write_all(b"[")?;
                    $(
                        if $n != 0 {
                            writer.write_all(b",")?;
                        }
                        self.$n.json_write(writer)?;
                    )+
                    writer.write_all(b"]")
                }
            }
            impl<'input, $($name),+> Deserialize<'input> for ($($name,)+)
            where
                $($name: Deserialize<'input>,)+
            {
                #[inline]
                fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
                where
                    Self: std::marker::Sized + 'input,
                {
                    if let Some(simd_json::Node::Array{len: $len, ..}) = tape.next() {
                        Ok((
                            $($name::from_tape(tape)?,)+
                        ))
                    } else {
                        Err(simd_json::Error::generic(
                            simd_json::ErrorType::ExpectedArray,
                        ))
                    }
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn tpl() {
        assert_eq!((1).json_string().unwrap(), "1");
        assert_eq!((1, 2).json_string().unwrap(), "[1,2]");
        assert_eq!((1, 2, 3).json_string().unwrap(), "[1,2,3]");
    }
}
