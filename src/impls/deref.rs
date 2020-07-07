use crate::*;

impl<T> Deserialize for Box<T>
where
    T: Deserialize,
{
    #[inline]
    fn from_tape<'input>(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: std::marker::Sized + 'input,
    {
        Ok(Box::new(T::from_tape(tape)?))
    }
}

// Taken from https://docs.serde.rs/src/serde/ser/impls.rs.html#378
macro_rules! deref_impl {
    (
        $(#[doc = $doc:tt])*
        <$($desc:tt)+
    ) => {
        $(#[doc = $doc])*
        impl <$($desc)+ {
            #[inline]
            fn json_write<W>(&self, writer: &mut W) -> Result
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
