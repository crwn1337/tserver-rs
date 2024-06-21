use crate::packet::PacketType;
use winnow::binary::u8;
use winnow::{IResult, Parser};

pub struct TColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl<'a> PacketType<'a> for TColor {
    fn serialize(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }

    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (data, r) = u8.parse_peek(data)?;
        let (data, g) = u8.parse_peek(data)?;
        let (data, b) = u8.parse_peek(data)?;
        Ok((data, TColor { r, g, b }))
    }
}
