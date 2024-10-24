use simd_json::Node;
pub use simd_json_derive_int::*;
use std::io::{self, Write};
use std::iter::Peekable;
use std::vec::IntoIter;
use value_trait::generator::BaseGenerator;
mod impls;
pub type Result = io::Result<()>;

pub type Tape<'input> = Peekable<IntoIter<Node<'input>>>;

pub mod de;

pub use de::Deserialize;

pub fn __skip(n: usize, tape: &mut Tape) {
    for _ in 0..n {
        match tape.next() {
            Some(Node::Array { count, .. }) => {
                for _ in 0..count {
                    if tape.next().is_none() {
                        return;
                    }
                }
            }
            Some(Node::Object { count, .. }) => {
                for _ in 0..count {
                    if tape.next().is_none() {
                        return;
                    }
                }
            }
            Some(_) => {}
            None => return,
        }
    }
}

pub trait Serialize {
    fn json_write<W>(&self, writer: &mut W) -> Result
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

pub trait SerializeAsKey {
    fn json_write<W>(&self, writer: &mut W) -> Result
    where
        W: Write;
}
impl<T: AsRef<str>> SerializeAsKey for T {
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> Result
    where
        W: Write,
    {
        let s: &str = self.as_ref();
        s.json_write(writer)
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
        unimplemented!("write min is not supported")
    }
}
