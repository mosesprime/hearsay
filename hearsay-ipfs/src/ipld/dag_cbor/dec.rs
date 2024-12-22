use std::io::{self, Read};

use crate::ipld::{CodecError, Decode};

use super::{DagCbor, Header};

#[inline]
fn read_header<R: Read>(r: &mut R) -> io::Result<Header> {
    Ok(Header::from(read_u8(r)?))
}

#[inline]
fn read_u8<R: Read>(r: &mut R) -> io::Result<u8> {
    let mut buf = [0; 1];
    r.read_exact(&mut buf)?;
    Ok(u8::from_be_bytes(buf))
}

impl Decode<DagCbor> for f64 {
    fn decode<R: io::BufRead>(c: &DagCbor, r: &mut R) -> Result<Self, CodecError> {
        todo!()
    }
}
