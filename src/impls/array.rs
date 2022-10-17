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
    ($($len:tt)+) => {
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
        )+
    }
}

array_impls! {
    1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
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
        assert_eq!((&s).json_string().unwrap(), "[]");
        assert_eq!((&[1]).json_string().unwrap(), "[1]");
        assert_eq!((&[1, 2]).json_string().unwrap(), "[1,2]");
        assert_eq!((&[1, 2, 3]).json_string().unwrap(), "[1,2,3]");
    }
}
