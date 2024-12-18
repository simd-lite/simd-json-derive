use crate::{de, Deserialize, Serialize};
use simd_json::{BorrowedValue, Node, OwnedValue};
use value_trait::{base::Writable, ValueBuilder};

impl Serialize for OwnedValue {
    fn json_write<W>(&self, writer: &mut W) -> crate::Result
    where
        W: std::io::Write,
    {
        self.write(writer)
    }
}
impl Serialize for BorrowedValue<'_> {
    fn json_write<W>(&self, writer: &mut W) -> crate::Result
    where
        W: std::io::Write,
    {
        self.write(writer)
    }
}

struct OwnedDeser<'input, 'tape>(&'tape mut crate::Tape<'input>);

impl OwnedDeser<'_, '_> {
    #[inline(always)]
    fn parse(&mut self) -> OwnedValue {
        match self.0.next() {
            Some(Node::Static(s)) => OwnedValue::Static(s),
            Some(Node::String(s)) => OwnedValue::from(s),
            Some(Node::Array { len, .. }) => self.parse_array(len),
            Some(Node::Object { len, .. }) => self.parse_map(len),
            None => unreachable!("We have validated the tape in the second stage of parsing, this should never happen"),
        }
    }
    #[inline(always)]
    fn parse_array(&mut self, len: usize) -> OwnedValue {
        let mut res: Vec<OwnedValue> = Vec::with_capacity(len);
        // Rust doesn't optimize the normal loop away here
        // so we write our own avoiding the length
        // checks during push
        for _ in 0..len {
            res.push(self.parse())
        }
        OwnedValue::Array(Box::new(res))
    }

    #[inline(always)]
    fn parse_map(&mut self, len: usize) -> OwnedValue {
        let mut res = OwnedValue::object_with_capacity(len);

        // Since we checked if it's empty we know that we at least have one
        // element so we eat this
        if let OwnedValue::Object(ref mut res) = res {
            for _ in 0..len {
                if let Some(Node::String(key)) = self.0.next() {
                    res.insert_nocheck(key.into(), self.parse());
                } else {
                    unreachable!("We have validated the tape in the second stage of parsing, this should never happen")
                }
            }
        } else {
            unreachable!("We have generated this object and know it is nothing else")
        }
        res
    }
}
impl<'input> Deserialize<'input> for OwnedValue {
    fn from_tape(tape: &mut crate::Tape<'input>) -> de::Result<Self>
    where
        Self: Sized + 'input,
    {
        Ok(OwnedDeser(tape).parse())
    }
}

struct BorrowedDeser<'input, 'tape>(&'tape mut crate::Tape<'input>);

impl<'input> BorrowedDeser<'input, '_> {
    #[inline(always)]
    fn parse(&mut self) -> BorrowedValue<'input> {
        match self.0.next() {
            Some(Node::Static(s)) => BorrowedValue::Static(s),
            Some(Node::String(s)) => BorrowedValue::from(s),
            Some(Node::Array { len, .. }) => self.parse_array(len),
            Some(Node::Object { len, .. }) => self.parse_map(len),
            None => unreachable!("We have validated the tape in the second stage of parsing, this should never happen"),
        }
    }
    #[inline(always)]
    fn parse_array(&mut self, len: usize) -> BorrowedValue<'input> {
        let mut res: Vec<BorrowedValue<'input>> = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(self.parse());
        }
        BorrowedValue::Array(Box::new(res))
    }

    #[inline(always)]
    fn parse_map(&mut self, len: usize) -> BorrowedValue<'input> {
        let mut res = BorrowedValue::object_with_capacity(len);

        // Since we checked if it's empty we know that we at least have one
        // element so we eat this
        if let BorrowedValue::Object(ref mut res) = res {
            for _ in 0..len {
                if let Some(Node::String(key)) = self.0.next() {
                    res.insert_nocheck(key.into(), self.parse());
                } else {
                    unreachable!("We have validated the tape in the second stage of parsing, this should never happen")
                }
            }
        } else {
            unreachable!("We have generated this object and know it is nothing else")
        }

        res
    }
}
impl<'input> Deserialize<'input> for BorrowedValue<'input> {
    fn from_tape(tape: &mut crate::Tape<'input>) -> de::Result<Self>
    where
        Self: Sized + 'input,
    {
        Ok(BorrowedDeser(tape).parse())
    }
}
