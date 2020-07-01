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

impl<K, V> Serialize for collections::BTreeMap<K, V>
where
    K: Serialize,
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

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn vec() {
        let mut v = Vec::new();
        assert_eq!(v.json_string().unwrap(), "[]");
        v.push(1);
        assert_eq!(v.json_string().unwrap(), "[1]");
        v.push(2);
        assert_eq!(v.json_string().unwrap(), "[1,2]");
        v.push(3);
        assert_eq!(v.json_string().unwrap(), "[1,2,3]");
    }
}
