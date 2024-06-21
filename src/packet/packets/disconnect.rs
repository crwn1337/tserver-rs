use winnow::IResult;

use crate::packet::{types::text::TText, Packet, PacketDirection, PacketType};

pub struct Disconnect {
    pub reason: TText,
}

impl Disconnect {
    pub fn new_literal(reason: &str) -> Self {
        Disconnect {
            reason: TText::new_literal(reason),
        }
    }
}

impl<'a> Packet<'a> for Disconnect {
    const PACKET_TYPE: u8 = 0x02;
    const DIRECTION: PacketDirection = PacketDirection::ToClient;

    fn serialize(&self) -> Vec<u8> {
        let mut data = vec![Self::PACKET_TYPE];
        data.extend(self.reason.serialize());
        data
    }

    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (data, reason) = TText::deserialize(data)?;
        Ok((data, Self { reason }))
    }
}
