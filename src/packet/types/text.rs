use crate::packet::PacketType;
use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};
use std::mem::size_of;
use winnow::{
    binary::{length_repeat, u8},
    unpeek, IResult, Parser,
};

#[repr(u8)]
#[derive(PartialEq, FromPrimitive, ToPrimitive)]
pub enum TextMode {
    Literal = 0,
    Formattable = 1,
    LocalizationKey = 2,
}

pub struct NetworkText {
    pub mode: TextMode,
    pub text: String,
    pub subtitutions: Vec<NetworkText>,
}

impl NetworkText {
    pub fn new_literal(text: &str) -> Self {
        NetworkText {
            mode: TextMode::Literal,
            text: text.to_string(),
            subtitutions: Vec::new(),
        }
    }
}

impl<'a> NetworkText {
    // TODO: test if this is correct
    fn deserialize_subtitutions(mut self, data: &'a [u8]) -> IResult<&'a [u8], Self> {
        if self.mode == TextMode::Literal {
            return Ok((data, self));
        }
        let (data, subtitutions) =
            length_repeat(u8, unpeek(NetworkText::deserialize)).parse_peek(data)?;
        self.subtitutions = subtitutions;
        Ok((data, self))
    }

    // TODO: test if this is correct
    fn serialize_subtitutions(&self) -> Vec<u8> {
        if self.mode == TextMode::Literal || self.subtitutions.is_empty() {
            return Vec::new();
        }
        let mut data = Vec::new();
        data.push(self.subtitutions.len() as u8);
        self.subtitutions
            .iter()
            .map(|sub| sub.serialize())
            .for_each(|sub| data.extend(sub));
        data
    }
}

impl<'a> PacketType<'a> for NetworkText {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(size_of::<u8>() + self.text.len());
        // Should be safe?
        data.push(self.mode.to_u8().unwrap());
        data.extend(self.text.serialize());
        data.extend(self.serialize_subtitutions());
        data
    }

    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (data, mode) = u8.verify_map(TextMode::from_u8).parse_peek(data)?;
        let (data, text) = String::deserialize(data)?;

        let networktext = NetworkText {
            mode,
            text: text.to_string(),
            subtitutions: Vec::new(),
        };
        let (data, networktext) = networktext.deserialize_subtitutions(data)?;

        Ok((data, networktext))
    }
}
