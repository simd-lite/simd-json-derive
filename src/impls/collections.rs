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
impl<T> Deserialize for Vec<T>
where
    T: Deserialize,
{
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
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
impl<T> Deserialize for collections::VecDeque<T>
where
    T: Deserialize,
{
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
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
impl<T> Deserialize for collections::BinaryHeap<T>
where
    T: Deserialize + Ord,
{
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
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
impl<T> Deserialize for collections::BTreeSet<T>
where
    T: Deserialize + Ord,
{
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
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
impl<T> Deserialize for collections::LinkedList<T>
where
    T: Deserialize + Ord,
{
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
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
impl<T, H> Deserialize for collections::HashSet<T, H>
where
    T: Deserialize + std::hash::Hash + Eq,
    H: std::hash::BuildHasher + Default,
{
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
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

impl<K, V, H> Deserialize for collections::HashMap<K, V, H>
where
    K: Deserialize + std::hash::Hash + Eq,
    V: Deserialize,
    H: std::hash::BuildHasher + Default,
{
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
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
impl<K, V> Deserialize for collections::BTreeMap<K, V>
where
    K: Deserialize + Ord,
    V: Deserialize,
{
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
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

#[cfg(test)]
mod test {
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
}
