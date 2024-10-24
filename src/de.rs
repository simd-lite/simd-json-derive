use simd_json::Buffers;

use crate::Tape;

fn expected(fields: &[&str]) -> String {
    fields
        .iter()
        .map(|f| format!("`{f}`"))
        .collect::<Vec<_>>()
        .join(", ")
}
/// Deserialisation error
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    /// Error from simd-json
    #[error(transparent)]
    Simd(#[from] simd_json::Error),
    /// Missing field
    #[error("missing field: `{0}`")]
    MissingField(&'static str),
    /// Unexpected field
    #[error("unknown field `{0}`, expected one of {}",expected(.1))]
    UnknownField(String, &'static [&'static str]),
    #[error("unnamed enum field `{0}` is not an array")]
    FieldNotAnArray(&'static str),
    #[error("unknwon enum variant `{0}`")]
    UnknownEnumVariant(String),
    #[error("invalid enum representation, needs to be either a string or an object")]
    InvalidEnumRepresentation,
    #[error("invalid struct representation, needs to be an object")]
    InvalidStructRepresentation,
    #[error("Unexpected e,nd of input")]
    EOF,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Deserialize<'input> {
    fn from_tape(tape: &mut Tape<'input>) -> Result<Self>
    where
        Self: Sized + 'input;

    #[inline]
    fn from_slice(json: &'input mut [u8]) -> Result<Self>
    where
        Self: Sized + 'input,
    {
        let tape = simd_json::to_tape(json)?;
        let mut itr = tape.0.into_iter().peekable();
        Self::from_tape(&mut itr)
    }

    #[inline]
    fn from_slice_with_buffers(json: &'input mut [u8], buffers: &mut Buffers) -> Result<Self>
    where
        Self: Sized + 'input,
    {
        let tape = simd_json::Deserializer::from_slice_with_buffers(json, buffers)?.into_tape();
        let mut itr = tape.0.into_iter().peekable();
        Self::from_tape(&mut itr)
    }

    #[inline]
    /// # Safety
    ///
    /// user must not use the string afterwards
    /// as it most likely will no longer contain valid utf-8
    unsafe fn from_str(json: &'input mut str) -> Result<Self>
    where
        Self: Sized + 'input,
    {
        Self::from_slice(json.as_bytes_mut())
    }
}
