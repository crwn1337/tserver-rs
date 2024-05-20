use std::io::{Error, ErrorKind};

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::{
    bytes::BytesMut,
    codec::{Decoder, Framed, LengthDelimitedCodec},
};

use crate::packet::Packet;

pub struct Peer {
    frame: Framed<TcpStream, LengthDelimitedCodec>,
}

impl Peer {
    pub fn codec() -> LengthDelimitedCodec {
        LengthDelimitedCodec::builder()
            .little_endian()
            .length_field_type::<u16>()
            .length_adjustment(-(std::mem::size_of::<u16>() as isize))
            .new_codec()
    }

    pub fn new(stream: TcpStream) -> Self {
        Peer {
            frame: Self::codec().framed(stream),
        }
    }

    pub async fn next(&mut self) -> Result<BytesMut, Error> {
        match self.frame.next().await {
            Some(res) => res,
            None => Err(Error::from(ErrorKind::UnexpectedEof)),
        }
    }

    pub async fn send(&mut self, packet: impl Packet<'_>) -> Result<(), Error> {
        self.frame.send(packet.serialize().into()).await
    }
}
