use nom::bytes::complete::take;
use nom::number::complete::u8;
use nom::IResult;

use crate::packet::PacketType;

pub struct TString(String);

impl TString {
    pub fn new(str: &str) -> Self {
        TString(str.to_string())
    }
}

impl<'a> PacketType<'a> for TString {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(std::mem::size_of::<u8>() + self.0.len());
        data.push(self.0.len() as u8);
        data.extend_from_slice(self.0.as_bytes());
        data
    }

    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (data, len) = u8(data)?;
        let (data, str_data) = take(len)(data)?;
        let str = String::from_utf8_lossy(str_data).to_string();
        Ok((data, TString(str)))
    }
}

impl AsRef<str> for TString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
