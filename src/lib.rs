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

impl Serialize for () {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        writer.write_all(b"null")
    }
}

impl Serialize for bool {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
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
                    if let Err(e) = writer.write_all(b"[") {
                        return Err(e);
                    };
                    if let Err(e) = first.json_write(writer) {
                        return Err(e);
                    };
                    for e in i {
                        if let Err(e) = writer.write_all(b",") {
                            return Err(e);
                        };
                        if let Err(e) = e.json_write(writer) {
                            return Err(e);
                        };
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
            if let Err(e) = writer.write_all(b"[") {
                return Err(e);
            };
            if let Err(e) = first.json_write(writer) {
                return Err(e);
            };
            for e in i {
                if let Err(e) = writer.write_all(b",") {
                    return Err(e);
                };
                if let Err(e) = e.json_write(writer) {
                    return Err(e);
                };
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
            for (k, v) in i {
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
            for (k, v) in i {
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

// Taken from https://docs.serde.rs/src/serde/ser/impls.rs.html#378
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
deref_impl!(<T: ?Sized> Serialize for Box<T> where T: Serialize);
deref_impl!(<'a, T: ?Sized> Serialize for std::borrow::Cow<'a, T> where T: Serialize + ToOwned);

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

// takenn from https://docs.serde.rs/src/serde/ser/impls.rs.html#306

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> Serialize for ($($name,)+)
            where
                $($name: Serialize,)+
            {
                #[inline]
                fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
                where
                    W: Write,
                {
                    if let Err(e) = writer.write_all(b"["){
                        return Err(e);
                    };
                    $(
                        if $n == 0 {
                            if let Err(e) = writer.write_all(b","){
                                return Err(e);
                            };
                        }
                        if let Err(e) = self.$n.json_write(writer){
                            return Err(e);
                        };
                    )+
                    writer.write_all(b"]")
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
