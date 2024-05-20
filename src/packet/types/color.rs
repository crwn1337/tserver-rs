use crate::packet::PacketType;
use nom::number::complete::u8;
use nom::IResult;

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
        let (data, r) = u8(data)?;
        let (data, g) = u8(data)?;
        let (data, b) = u8(data)?;
        Ok((data, TColor { r, g, b }))
    }
}
