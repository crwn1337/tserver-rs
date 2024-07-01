use crate::packet::{types::text::NetworkText, Packet, PacketDirection, PacketType};
use winnow::IResult;

pub struct Disconnect {
    pub reason: NetworkText,
}

impl Disconnect {
    pub fn new_literal(reason: &str) -> Self {
        Disconnect {
            reason: NetworkText::new_literal(reason),
        }
    }
}

impl<'a> Packet<'a> for Disconnect {
    const PACKET_TYPE: u8 = 0x02;
    const DIRECTION: PacketDirection = PacketDirection::ToClient;

    fn serialize(&self) -> Vec<u8> {
        self.reason.serialize()
    }

    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (data, reason) = NetworkText::deserialize(data)?;
        Ok((data, Self { reason }))
    }
}
