use std::io::{Error, ErrorKind};

use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::{
    bytes::BytesMut,
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
}
