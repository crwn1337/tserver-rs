use crate::packet::PacketType;

use nom::{multi::count, number::complete::u8, IResult};

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

    fn deserialize_subtitutions(mut self, data: &'a [u8]) -> IResult<&'a [u8], Self> {
        if self.mode == TextMode::Literal {
            return Ok((data, self));
        }
        let (data, len) = u8(data)?;
        let (data, subtitutions) = count(TText::deserialize, len as usize)(data)?;
        self.subtitutions = Some(subtitutions);
        Ok((data, self))
    }
}

impl<'a> PacketType<'a> for TText {
    fn serialize(&self) -> Vec<u8> {
        // no idea if this is correct, i just let copilot generate it :)
        let mut data = vec![self.to_u8()];
        data.extend(TString::new(&self.text).serialize());
        if let Some(subtitutions) = &self.subtitutions {
            data.push(subtitutions.len() as u8);
            for subtitution in subtitutions {
                data.extend(subtitution.serialize());
            }
        }
        data
    }

    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (data, mode) = u8(data)?;
        let (data, text) = TString::deserialize(data)?;

        // good lord heavens
        let mode = TText::from_u8(mode).ok_or(nom::Err::Error(nom::error::make_error(
            data,
            nom::error::ErrorKind::Verify,
        )))?;

        let networktext = TText {
            mode,
            text: text.as_ref().to_string(),
            subtitutions: None,
        };
        let (data, networktext) = networktext.deserialize_subtitutions(data)?;

        Ok((data, networktext))
    }
}
