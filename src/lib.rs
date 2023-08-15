extern crate core;

use simd_json::AlignedBuf;
pub use simd_json_derive_int::*;
use std::collections;
use std::io::{self, Write};
use std::iter::Peekable;
use std::vec::IntoIter;
use value_trait::generator::BaseGenerator;
mod impls;
pub type Result = io::Result<()>;

pub type Tape<'input> = Peekable<IntoIter<simd_json::Node<'input>>>;

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

pub trait Deserialize<'input> {
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: Sized + 'input;

    #[inline]
    fn from_slice(json: &'input mut [u8]) -> simd_json::Result<Self>
    where
        Self: Sized + 'input,
    {
        let tape = simd_json::to_tape(json)?;
        let mut itr = tape.into_iter().peekable();
        itr.next();
        Self::from_tape(&mut itr)
    }

    #[inline]
    fn from_slice_with_buffer(
        json: &'input mut [u8],
        string_buffer: &mut [u8],
    ) -> simd_json::Result<Self>
    where
        Self: Sized + 'input,
    {
        let tape =
            simd_json::Deserializer::from_slice_with_buffer(json, string_buffer)?.into_tape();
        let mut itr = tape.into_iter().peekable();
        itr.next();
        Self::from_tape(&mut itr)
    }

    #[inline]
    fn from_slice_with_buffers(
        json: &'input mut [u8],
        input_buffer: &mut AlignedBuf,
        string_buffer: &mut [u8],
    ) -> simd_json::Result<Self>
    where
        Self: Sized + 'input,
    {
        let tape =
            simd_json::Deserializer::from_slice_with_buffers(json, input_buffer, string_buffer)?
                .into_tape();
        let mut itr = tape.into_iter().peekable();
        itr.next();
        Self::from_tape(&mut itr)
    }


    #[allow(clippy::missing_safety_doc)]// it's literally right below this idk what it's mad about
    #[inline]
    /// # Safety:
    ///
    /// user must not use the string afterwards
    /// as it most likely will no longer contain valid utf-8
    unsafe fn from_str(json: &'input mut str) -> simd_json::Result<Self>
    where
        Self: Sized + 'input,
    {
        Self::from_slice(json.as_bytes_mut())
    }

    fn from_string(mut json: String) -> simd_json::Result<Self>
    where
        Self: Sized + 'input + for<'de> Deserialize<'de>,
    { unsafe { Self::from_slice(json.as_bytes_mut()) } }
}

pub(crate) struct DummyGenerator<W: Write>(W);
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
