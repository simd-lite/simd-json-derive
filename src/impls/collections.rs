use std::ops::Range;

use crate::*;

macro_rules! vec_like {
    ($t:ty) => {
        impl<T> Serialize for $t
        where
            T: Serialize,
        {
            #[inline]
            fn json_write<W>(&self, writer: &mut W) -> Result
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
impl<'input, T> Deserialize<'input> for Vec<T>
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
                let mut v = Vec::with_capacity(size);
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

vec_like!([T]);
vec_like!(collections::VecDeque<T>);
impl<'input, T> Deserialize<'input> for collections::VecDeque<T>
where
    T: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Array(size, _)) = tape.next() {
            let mut v = collections::VecDeque::new();
            for _ in 0..size {
                v.push_back(T::from_tape(tape)?)
            }
            Ok(v)
        } else {
            Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedArray,
            ))
        }
    }
}
vec_like!(collections::BinaryHeap<T>);
impl<'input, T> Deserialize<'input> for collections::BinaryHeap<T>
where
    T: Deserialize<'input> + Ord,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Array(size, _)) = tape.next() {
            let mut v = collections::BinaryHeap::new();
            for _ in 0..size {
                v.push(T::from_tape(tape)?)
            }
            Ok(v)
        } else {
            Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedArray,
            ))
        }
    }
}
vec_like!(collections::BTreeSet<T>);
impl<'input, T> Deserialize<'input> for collections::BTreeSet<T>
where
    T: Deserialize<'input> + Ord,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Array(size, _)) = tape.next() {
            let mut v = collections::BTreeSet::new();
            for _ in 0..size {
                v.insert(T::from_tape(tape)?);
            }
            Ok(v)
        } else {
            Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedArray,
            ))
        }
    }
}
vec_like!(collections::LinkedList<T>);
impl<'input, T> Deserialize<'input> for collections::LinkedList<T>
where
    T: Deserialize<'input> + Ord,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Array(size, _)) = tape.next() {
            let mut v = collections::LinkedList::new();
            for _ in 0..size {
                v.push_back(T::from_tape(tape)?);
            }
            Ok(v)
        } else {
            Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedArray,
            ))
        }
    }
}
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
impl<'input, T, H> Deserialize<'input> for collections::HashSet<T, H>
where
    T: Deserialize<'input> + std::hash::Hash + Eq,
    H: std::hash::BuildHasher + Default,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Array(size, _)) = tape.next() {
            let mut v = collections::HashSet::with_capacity_and_hasher(size, H::default());
            for _ in 0..size {
                v.insert(T::from_tape(tape)?);
            }
            Ok(v)
        } else {
            Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedArray,
            ))
        }
    }
}

impl<K, V, H> Serialize for collections::HashMap<K, V, H>
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

impl<'input, K, V, H> Deserialize<'input> for collections::HashMap<K, V, H>
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
            let mut v = collections::HashMap::with_capacity_and_hasher(size, H::default());
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

impl<K, V> Serialize for collections::BTreeMap<K, V>
where
    K: SerializeAsKey,
    V: Serialize,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> Result
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
impl<'input, K, V> Deserialize<'input> for collections::BTreeMap<K, V>
where
    K: Deserialize<'input> + Ord,
    V: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Object(size, _)) = tape.next() {
            let mut v = collections::BTreeMap::new();
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

impl<T> Serialize for Range<T>
where
    T: Serialize,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> Result
    where
        W: Write,
    {
        if let Err(e) = writer.write_all(b"{\"start\":") {
            return Err(e);
        };
        if let Err(e) = self.start.json_write(writer) {
            return Err(e);
        }
        if let Err(e) = writer.write_all(b",\"end\":") {
            return Err(e);
        };
        if let Err(e) = self.end.json_write(writer) {
            return Err(e);
        }
        writer.write_all(b"}")
    }
}

impl<'input, T> Deserialize<'input> for Range<T>
where
    T: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        if let Some(simd_json::Node::Object(2, _)) = tape.next() {
            match tape.next() {
                Some(simd_json::Node::String("start")) => {
                    let start = Deserialize::from_tape(tape)?;
                    if let Some(simd_json::Node::String("end")) = tape.next() {
                        let end = Deserialize::from_tape(tape)?;
                        Ok(start..end)
                    } else {
                        // FIXME
                        Err(simd_json::Error::generic(
                            simd_json::ErrorType::ExpectedString,
                        ))
                    }
                }
                Some(simd_json::Node::String("end")) => {
                    let end = Deserialize::from_tape(tape)?;
                    if let Some(simd_json::Node::String("start")) = tape.next() {
                        let start = Deserialize::from_tape(tape)?;
                        Ok(start..end)
                    } else {
                        // FIXME
                        Err(simd_json::Error::generic(
                            simd_json::ErrorType::ExpectedString,
                        ))
                    }
                }
                _ => Err(simd_json::Error::generic(
                    simd_json::ErrorType::ExpectedString,
                )),
            }
        } else {
            Err(simd_json::Error::generic(simd_json::ErrorType::ExpectedMap))
        }
    }
}
#[cfg(test)]
mod test {
    use std::ops::Range;

    use crate::*;
    #[test]
    fn vec() {
        let mut v: Vec<u8> = Vec::new();
        assert_eq!(v.json_string().unwrap(), "[]");

        v.push(1);
        let mut s = v.json_string().unwrap();
        assert_eq!(s, "[1]");
        let s: Vec<u8> = Vec::from_str(s.as_mut_str()).unwrap();
        assert_eq!(s, v);

        v.push(2);
        let mut s = v.json_string().unwrap();
        assert_eq!(s, "[1,2]");
        let s: Vec<u8> = Vec::from_str(s.as_mut_str()).unwrap();
        assert_eq!(s, v);

        v.push(3);
        let mut s = v.json_string().unwrap();
        assert_eq!(s, "[1,2,3]");
        let s: Vec<u8> = Vec::from_str(s.as_mut_str()).unwrap();
        assert_eq!(s, v);
    }

    #[test]
    fn range() {
        let r = 1..42;
        let mut v = r.json_vec().unwrap();
        assert_eq!(br#"{"start":1,"end":42}"#, v.as_slice());
        let r1 = Range::from_slice(v.as_mut_slice()).unwrap();
        assert_eq!(r, r1);
    }
}
