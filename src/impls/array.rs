use crate::*;
// Taken from https://docs.serde.rs/src/serde/ser/impls.rs.html#378

impl<T> Serialize for [T; 0] {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        writer.write_all(b"[]")
    }
}

macro_rules! array_impls {
    ($($len:expr => ($($n:tt)+))+) => {
        $(
            impl<T> Serialize for [T; $len]
            where
                T: Serialize,
            {
                #[inline]
                fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
                where
                    W: Write,
                {
                    let mut i = self.iter();
                    if let Some(first) = i.next() {
                        writer.write_all(b"[")?;
                        first.json_write(writer)?;
                        for e in i {
                            writer.write_all(b",")?;
                            e.json_write(writer)?;
                        }
                        writer.write_all(b"]")
                    } else {
                        unreachable!()
                    }
                }
            }
            impl<'input, T> Deserialize<'input> for [T; $len]
            where
                T: Deserialize<'input>,
            {
                #[inline]
                fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
                where
                    Self: std::marker::Sized + 'input,
                {
                    if let Some(simd_json::Node::Array($len, _)) = tape.next() {
                        Ok([
                            $(
                                if $n >= 0 {
                                    T::from_tape(tape)?
                                } else {
                                    unreachable!("we need this for the n")
                                },
                            )+
                        ])
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

array_impls! {
    1 => (0)
    2 => (0 1)
    3 => (0 1 2)
    4 => (0 1 2 3)
    5 => (0 1 2 3 4)
    6 => (0 1 2 3 4 5)
    7 => (0 1 2 3 4 5 6)
    8 => (0 1 2 3 4 5 6 7)
    9 => (0 1 2 3 4 5 6 7 8)
    10 => (0 1 2 3 4 5 6 7 8 9)
    11 => (0 1 2 3 4 5 6 7 8 9 10)
    12 => (0 1 2 3 4 5 6 7 8 9 10 11)
    13 => (0 1 2 3 4 5 6 7 8 9 10 11 12)
    14 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13)
    15 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14)
    16 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15)
    17 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16)
    18 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17)
    19 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18)
    20 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19)
    21 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20)
    22 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21)
    23 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22)
    24 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23)
    25 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24)
    26 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25)
    27 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26)
    28 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27)
    29 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28)
    30 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29)
    31 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30)
    32 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)
}
#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn arr() {
        let s: [u8; 0] = [];
        assert_eq!(s.json_string().unwrap(), "[]");
        assert_eq!([1].json_string().unwrap(), "[1]");
        assert_eq!([1, 2].json_string().unwrap(), "[1,2]");
        assert_eq!([1, 2, 3].json_string().unwrap(), "[1,2,3]");
    }
    #[test]
    fn slice() {
        let s: [u8; 0] = [];
        assert_eq!(s.json_string().unwrap(), "[]");
        assert_eq!([1].json_string().unwrap(), "[1]");
        assert_eq!([1, 2].json_string().unwrap(), "[1,2]");
        assert_eq!([1, 2, 3].json_string().unwrap(), "[1,2,3]");
    }
}
