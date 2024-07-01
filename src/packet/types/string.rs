use crate::packet::PacketType;
use std::mem::size_of;
use winnow::{
    binary::{length_take, u8},
    IResult, Parser,
};

impl<'a> PacketType<'a> for String {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<u8>() + self.len());
        buf.push(self.len() as u8);
        buf.extend_from_slice(self.as_bytes());
        buf
    }

    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (data, str_data) = length_take(u8).parse_peek(data)?;
        let str = String::from_utf8_lossy(str_data).to_string();
        Ok((data, str))
    }
}
