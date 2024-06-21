use crate::packet::PacketType;
use winnow::{
    binary::{length_repeat, u8},
    unpeek, IResult, Parser,
};

use super::string::TString;

#[derive(PartialEq)]
pub enum TextMode {
    Literal,
    Formattable,
    LocalizationKey,
}

pub struct TText {
    pub mode: TextMode,
    pub text: String,
    pub subtitutions: Option<Vec<TText>>,
}

impl TText {
    pub fn new_literal(text: &str) -> Self {
        TText {
            mode: TextMode::Literal,
            text: text.to_string(),
            subtitutions: None,
        }
    }
}

impl<'a> TText {
    fn from_u8(t: u8) -> Option<TextMode> {
        match t {
            0 => Some(TextMode::Literal),
            1 => Some(TextMode::Formattable),
            2 => Some(TextMode::LocalizationKey),
            _ => None,
        }
    }

    fn to_u8(&self) -> u8 {
        match self.mode {
            TextMode::Literal => 0,
            TextMode::Formattable => 1,
            TextMode::LocalizationKey => 2,
        }
    }

    // TODO: test if this is correct
    fn deserialize_subtitutions(mut self, data: &'a [u8]) -> IResult<&'a [u8], Self> {
        if self.mode == TextMode::Literal {
            return Ok((data, self));
        }
        let (data, subtitutions) =
            length_repeat(u8, unpeek(TText::deserialize)).parse_peek(data)?;
        self.subtitutions = Some(subtitutions);
        Ok((data, self))
    }
}

impl<'a> PacketType<'a> for TText {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(std::mem::size_of::<u8>() + self.text.len());
        data.push(self.to_u8());
        data.extend(TString::new(&self.text).serialize());
        // TODO: test if this is correct
        if let Some(subtitutions) = &self.subtitutions {
            data.push(subtitutions.len() as u8);
            for substitution in subtitutions {
                data.extend(substitution.serialize());
            }
        }
        data
    }

    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (data, mode) = u8.verify_map(TText::from_u8).parse_peek(data)?;
        let (data, text) = TString::deserialize(data)?;

        let networktext = TText {
            mode,
            text: text.to_string(),
            subtitutions: None,
        };
        let (data, networktext) = networktext.deserialize_subtitutions(data)?;

        Ok((data, networktext))
    }
}
