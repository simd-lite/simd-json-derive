pub use simd_json_derive_int::*;
use std::collections;
use std::io::{self, Write};
use value_trait::generator::BaseGenerator;
pub trait Serialize {
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write;

    #[inline]
    fn json_vec(&self) -> io::Result<Vec<u8>> {
        let mut v = Vec::with_capacity(512);
        self.json_write(&mut v)?;
        Ok(v)
    }
    #[inline]
    fn json_string(&self) -> io::Result<String> {
        self.json_vec()
            .map(|v| unsafe { String::from_utf8_unchecked(v) })
    }
}

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

impl Serialize for String {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        DummyGenerator(writer).write_string(self)
    }
}

impl Serialize for str {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        DummyGenerator(writer).write_string(self)
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

macro_rules! vec_like {
    ($t:ty) => {
        impl<T> Serialize for $t
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
                    writer.write_all(b"[]")
                }
            }
        }
    };
}
vec_like!(Vec<T>);
vec_like!([T]);
vec_like!(collections::VecDeque<T>);
vec_like!(collections::BinaryHeap<T>);
vec_like!(collections::BTreeSet<T>);
vec_like!(collections::LinkedList<T>);
impl<T, H> Serialize for collections::HashSet<T, H>
where
    T: Serialize,
    H: std::hash::BuildHasher,
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
            writer.write_all(b"[]")
        }
    }
}

impl<K, V, H> Serialize for collections::HashMap<K, V, H>
where
    K: Serialize,
    V: Serialize,
    H: std::hash::BuildHasher,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        let mut i = self.iter();
        if let Some((k, v)) = i.next() {
            writer.write_all(b"{")?;
            k.json_write(writer)?;
            writer.write_all(b":")?;
            v.json_write(writer)?;
            for (k, v) in i {
                writer.write_all(b",")?;
                k.json_write(writer)?;
                writer.write_all(b":")?;
                v.json_write(writer)?;
            }
            writer.write_all(b"}")
        } else {
            writer.write_all(b"{}")
        }
    }
}

impl<K, V> Serialize for collections::BTreeMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        let mut i = self.iter();
        if let Some((k, v)) = i.next() {
            writer.write_all(b"{")?;
            k.json_write(writer)?;
            writer.write_all(b":")?;
            v.json_write(writer)?;
            for (k, v) in i {
                writer.write_all(b",")?;
                k.json_write(writer)?;
                writer.write_all(b":")?;
                v.json_write(writer)?;
            }
            writer.write_all(b"}")
        } else {
            writer.write_all(b"{}")
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

// Talen from https://docs.serde.rs/src/serde/ser/impls.rs.html#378
macro_rules! deref_impl {
    (
        $(#[doc = $doc:tt])*
        <$($desc:tt)+
    ) => {
        $(#[doc = $doc])*
        impl <$($desc)+ {
            #[inline]
            fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
            where
                W: Write,
            {
                (**self).json_write(writer)
            }
        }
    };
}

deref_impl!(<'a, T> Serialize for &'a T where T: ?Sized + Serialize);
deref_impl!(<'a, T> Serialize for &'a mut T where T: ?Sized + Serialize);
