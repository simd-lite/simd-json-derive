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
                        if let Err(e) = writer.write_all(b"["){
                            return Err(e);
                        };
                        if let Err(e) = first.json_write(writer){
                            return Err(e);
                        };
                        for e in i {
                            if let Err(e) = writer.write_all(b","){
                                return Err(e);
                            };
                            if let Err(e) = e.json_write(writer){
                                return Err(e);
                            };
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
    01 02 03 04 05 06 07 08 09
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
