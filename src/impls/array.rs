use crate::*;
use std::mem::MaybeUninit;
use std::ptr;
// Taken from https://docs.serde.rs/src/serde/ser/impls.rs.html#378

struct Guard<'a, T, const N: usize> {
    pub array: &'a mut [MaybeUninit<T>; N], // we include size for a small optimization of pointer size
    pub initialized: usize,
}

impl<'a, T, const N: usize> Guard<'a, T, N> {
    #[inline]
    pub unsafe fn push_unchecked(&mut self, item: T) {
        // SAFETY: If `initialized` was correct before and the caller does not
        // invoke this method more than N times then writes will be in-bounds
        // and slots will not be initialized more than once.
        unsafe {
            self.array.get_unchecked_mut(self.initialized).write(item);
            self.initialized = self.initialized.wrapping_add(1); // unchecked_add is unstable
        }
    }
}

impl<'a, T, const N: usize> Drop for Guard<'a, T, N> {
    fn drop(&mut self) {
        debug_assert!(self.initialized <= N);

        // SAFETY: this slice will contain only initialized objects.
        unsafe {
            let slice = core::ptr::slice_from_raw_parts_mut(
                self.array.as_mut_ptr() as *mut T,
                self.initialized,
            );
            core::ptr::drop_in_place(slice);
        }
    }
}

impl<'input, T, const N: usize> Deserialize<'input> for [T; N]
where
    T: Deserialize<'input>,
{
    #[inline]
    fn from_tape(tape: &mut Tape<'input>) -> simd_json::Result<Self>
    where
        Self: Sized + 'input,
    {
        if let Some(simd_json::Node::Array { len, .. }) = tape.next() {
            if len != N {
                return Err(simd_json::Error::generic(simd_json::ErrorType::Serde(
                    format!("expected array of len {N} found array of len {len}"),
                )));
            }

            if N == 0 {
                // Safety: N is 0, and so *const [T; N] is *const [T; 0]
                return Ok(unsafe { ptr::read((&[]) as *const [T; N]) });
            }

            // Safety: elements are still MaybeUninit
            let mut array: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

            // Guard is here to make sure we drop
            let mut guard = Guard {
                array: &mut array,
                initialized: 0,
            };
            while guard.initialized < N {
                let item = T::from_tape(tape)?;

                // SAFETY: The loop condition ensures we have space to push the item
                unsafe { guard.push_unchecked(item) };
            }
            core::mem::forget(guard);

            // all elements initialized
            Ok(unsafe { array.map(|x| x.assume_init()) })
        } else {
            Err(simd_json::Error::generic(
                simd_json::ErrorType::ExpectedArray,
            ))
        }
    }
}

impl<T, const N: usize> Serialize for [T; N]
where
    T: Serialize,
{
    #[inline]
    fn json_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        // N is a compile time constant, this wont be checked at runtime
        if N == 0 {
            return writer.write_all(b"[]");
        }

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
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn arr() {
        let s: [u8; 0] = [];
        assert_eq!(s.json_string().unwrap(), "[]");
        assert_eq!([1].json_string().unwrap(), "[1]");
        assert_eq!([1, 2].json_string().unwrap(), "[1,2]");
        assert_eq!([1, 2, 3].json_string().unwrap(), "[1,2,3]");
    }
    #[test]
    fn arr2() {
        assert_eq!(
            <[u8; 0] as Deserialize<'_>>::from_slice(&mut b"[]".to_vec()),
            Ok([])
        );
        assert_eq!(
            <[u8; 1] as Deserialize<'_>>::from_slice(&mut b"[1]".to_vec()),
            Ok([1])
        );
        assert_eq!(
            <[u8; 2] as Deserialize<'_>>::from_slice(&mut b"[1, 2]".to_vec()),
            Ok([1, 2])
        );
        assert_eq!(
            <[u8; 3] as Deserialize<'_>>::from_slice(&mut b"[1, 2, 3]".to_vec()),
            Ok([1, 2, 3])
        );
    }
    #[test]
    fn slice() {
        let s: [u8; 0] = [];
        assert_eq!(s.json_string().unwrap(), "[]");
        assert_eq!([1].json_string().unwrap(), "[1]");
        assert_eq!([1, 2].json_string().unwrap(), "[1,2]");
        assert_eq!([1, 2, 3].json_string().unwrap(), "[1,2,3]");
    }
}
