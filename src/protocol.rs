extern crate tokio_proto;
extern crate tokio_io;
extern crate tokio_codec;


use self::tokio_proto::pipeline::ServerProto;
use self::tokio_io::{ AsyncRead, AsyncWrite };
use self::tokio_codec::{ Framed, Decoder };
use std::io;
use super::codec::LineCodec;

pub struct LineProto;

impl <T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        return Ok(Decoder::framed(LineCodec, io));
    }
}