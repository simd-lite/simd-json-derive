use std::mem::MaybeUninit;

use crate::{Deserialize, Serialize};
use simd_json::{BorrowedValue, Node, OwnedValue};
use value_trait::Writable;

impl Serialize for OwnedValue {
    fn json_write<W>(&self, writer: &mut W) -> crate::Result
    where
        W: std::io::Write,
    {
        self.write(writer)
    }
}
impl<'value> Serialize for BorrowedValue<'value> {
    fn json_write<W>(&self, writer: &mut W) -> crate::Result
    where
        W: std::io::Write,
    {
        self.write(writer)
    }
}

struct OwnedDeser<'input, 'tape>(&'tape mut crate::Tape<'input>);

impl<'input, 'tape> OwnedDeser<'input, 'tape> {
    #[inline(always)]
    fn parse(&mut self) -> simd_json::Result<OwnedValue> {
        match self.0.next() {
            Some(Node::Static(s)) => Ok(OwnedValue::Static(s)),
            Some(Node::String(s)) => Ok(OwnedValue::from(s)),
            Some(Node::Array(len, _)) => Ok(self.parse_array(len)),
            Some(Node::Object(len, _)) => Ok(self.parse_map(len)),
            None => Err(simd_json::Error::generic(simd_json::ErrorType::Eof)),
        }
    }
    #[inline(always)]
    #[allow(clippy::uninit_vec)]
    fn parse_array(&mut self, len: usize) -> OwnedValue {
        // Rust doens't optimize the normal loop away here
        // so we write our own avoiding the lenght
        // checks during push
        let res: Vec<OwnedValue> = unsafe {
            let mut res: Vec<MaybeUninit<OwnedValue>> = Vec::with_capacity(len);
            for i in 0..len {
                res.get_unchecked_mut(i).write(self.parse().unwrap());
            }
            res.set_len(len);
            std::mem::transmute(res)
        };
        OwnedValue::Array(res)
    }

    #[inline(always)]
    fn parse_map(&mut self, len: usize) -> OwnedValue {
        let mut res = simd_json::value::owned::Object::with_capacity(len);

        // Since we checked if it's empty we know that we at least have one
        // element so we eat this
        for _ in 0..len {
            if let Node::String(key) = self.0.next().unwrap() {
                res.insert_nocheck(key.into(), self.parse().unwrap());
            } else {
                unreachable!()
            }
        }
        OwnedValue::from(res)
    }
}
impl<'input> Deserialize<'input> for OwnedValue {
    fn from_tape(tape: &mut crate::Tape<'input>) -> simd_json::Result<Self>
    where
        Self: Sized + 'input,
    {
        OwnedDeser(tape).parse()
    }
}

struct BorrowedDeser<'input, 'tape>(&'tape mut crate::Tape<'input>);

impl<'input, 'tape> BorrowedDeser<'input, 'tape> {
    #[inline(always)]
    fn parse(&mut self) -> simd_json::Result<BorrowedValue<'input>> {
        match self.0.next() {
            Some(Node::Static(s)) => Ok(BorrowedValue::Static(s)),
            Some(Node::String(s)) => Ok(BorrowedValue::from(s)),
            Some(Node::Array(len, _)) => Ok(self.parse_array(len)),
            Some(Node::Object(len, _)) => Ok(self.parse_map(len)),
            None => Err(simd_json::Error::generic(simd_json::ErrorType::Eof)),
        }
    }
    #[inline(always)]
    #[allow(clippy::uninit_vec)]
    fn parse_array(&mut self, len: usize) -> BorrowedValue<'input> {
        // Rust doens't optimize the normal loop away here
        // so we write our own avoiding the lenght
        // checks during push
        let res: Vec<BorrowedValue> = unsafe {
            let mut res: Vec<MaybeUninit<BorrowedValue>> = Vec::with_capacity(len);
            for i in 0..len {
                res.get_unchecked_mut(i).write(self.parse().unwrap());
            }
            res.set_len(len);
            std::mem::transmute(res)
        };
        BorrowedValue::Array(res)
    }

    #[inline(always)]
    fn parse_map(&mut self, len: usize) -> BorrowedValue<'input> {
        let mut res = simd_json::value::borrowed::Object::with_capacity(len);

        // Since we checked if it's empty we know that we at least have one
        // element so we eat this
        for _ in 0..len {
            if let Node::String(key) = self.0.next().unwrap() {
                res.insert_nocheck(key.into(), self.parse().unwrap());
            } else {
                unreachable!()
            }
        }
        BorrowedValue::from(res)
    }
}
impl<'input> Deserialize<'input> for BorrowedValue<'input> {
    fn from_tape(tape: &mut crate::Tape<'input>) -> simd_json::Result<Self>
    where
        Self: Sized + 'input,
    {
        BorrowedDeser(tape).parse()
    }
}
