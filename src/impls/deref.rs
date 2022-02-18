use crate::*;

impl<'input, T> Deserialize<'input> for Box<T>
where
    T: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
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
#[cfg(feature = "impl-abi_stable")]
// TODO: make RBox `T: ?Sized`, upstream it
deref_impl!(<T> Serialize for abi_stable::std_types::RBox<T> where T: Serialize);
deref_impl!(<'a, T: ?Sized> Serialize for std::borrow::Cow<'a, T> where T: Serialize + ToOwned);
#[cfg(feature = "impl-abi_stable")]
deref_impl!(<'a, T> Serialize for abi_stable::std_types::RCow<&'a T> where T: Serialize + abi_stable::std_types::cow::IntoOwned);
#[cfg(feature = "impl-abi_stable")]
deref_impl!(<'a> Serialize for abi_stable::std_types::RCowStr<'a>);
#[cfg(feature = "impl-abi_stable")]
deref_impl!(<'a, T> Serialize for abi_stable::std_types::RCowSlice<'a, T> where T: Serialize + abi_stable::std_types::cow::IntoOwned);
