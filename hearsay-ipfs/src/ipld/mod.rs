use std::io::{BufRead, Write};

use cid::Cid;

pub trait Codec<T>: Link {
    const CODE: u64;
    type CodecError;

    fn encode<W: Write>(writer: W, data: &T) -> Result<(), Self::CodecError>;

    fn encode_to_vec(data: &T) -> Result<Vec<u8>, Self::CodecError> {
        let mut out = vec![];
        Self::encode(&mut out, data)?;
        Ok(out)
    }

    fn decode<R: BufRead>(reader: R) -> Result<T, Self::CodecError>;

    fn decode_from_slice(bytes: &[u8]) -> Result<T, Self::CodecError> {
        Self::decode(bytes)
    }
}

pub trait Link {
    type LinkError;

    fn links(bytes: &[u8]) -> Result<impl Iterator<Item = Cid>, Self::LinkError>;
}
