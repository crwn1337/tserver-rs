pub mod packets;
pub mod types;

use winnow::IResult;

pub trait PacketType<'a> {
    #[must_use]
    #[allow(clippy::double_must_use)]
    fn serialize(&self) -> Vec<u8>;

    #[must_use]
    #[allow(clippy::double_must_use)]
    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized;
}

pub enum PacketDirection {
    ToClient,
    ToServer,
    Both,
}

pub trait Packet<'a> {
    const PACKET_TYPE: u8;
    const DIRECTION: PacketDirection;

    #[must_use]
    #[allow(clippy::double_must_use)]
    fn serialize(&self) -> Vec<u8>;

    #[must_use]
    #[allow(clippy::double_must_use)]
    fn deserialize(data: &'a [u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized;
}
