use crate::packet::Packet;
use futures::{SinkExt, StreamExt};
use std::{
    io::{Error, ErrorKind},
    mem::size_of,
};
use tokio::net::TcpStream;
use tokio_util::{
    bytes::{Bytes, BytesMut},
    codec::{Decoder, Framed, LengthDelimitedCodec},
};

pub struct Peer {
    frame: Framed<TcpStream, LengthDelimitedCodec>,
}

impl Peer {
    pub fn codec() -> LengthDelimitedCodec {
        LengthDelimitedCodec::builder()
            .little_endian()
            .length_field_type::<u16>()
            .length_adjustment(-(size_of::<u16>() as isize))
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

    pub async fn send<'a, P>(&mut self, packet: P) -> Result<(), Error>
    where
        P: Packet<'a>,
    {
        let mut buf = vec![P::PACKET_TYPE];
        buf.extend(packet.serialize());
        self.frame.send(Bytes::from(buf)).await
    }
}
