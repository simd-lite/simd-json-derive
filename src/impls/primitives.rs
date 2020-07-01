use crate::*;

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
