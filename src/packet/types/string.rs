use std::ops::Deref;

use winnow::{
    binary::{length_take, u8},
    IResult, Parser,
};

use crate::packet::PacketType;

pub struct TString(String);

impl TString {
    pub fn new(str: &str) -> Self {
        TString(str.to_string())
    }
}

impl<'a> PacketType<'a> for TString {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(std::mem::size_of::<u8>() + self.len());
        data.push(self.len() as u8);
        data.extend_from_slice(self.as_bytes());
        data
    }

    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (data, str_data) = length_take(u8).parse_peek(data)?;
        let str = String::from_utf8_lossy(str_data).to_string();
        Ok((data, TString(str)))
    }
}

impl Deref for TString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
