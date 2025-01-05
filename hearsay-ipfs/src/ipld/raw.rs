use std::io::{self, Read, Seek, Write};

use bytes::Bytes;

use super::{Codec, CodecError, Decode, Encode, Ipld};

/// IPLD raw data codec.
pub struct RawData;

impl Codec for RawData {
    /// See <https://github.com/multiformats/multicodec/blob/master/table.csv>
    const CODE: u64 = 0x55;
}

impl Encode<RawData> for Ipld {
    fn encode<W: Write>(&self, c: &RawData, w: &mut W) -> Result<(), CodecError> {
        if let Ipld::Bytes(b) = self {
            b.encode(c, w)
        } else {
            Err(CodecError::Io(io::ErrorKind::InvalidData.into()))
        }
    }
}

impl Encode<RawData> for Bytes {
    fn encode<W: Write>(&self, _c: &RawData, w: &mut W) -> Result<(), CodecError> {
        w.write_all(self).map_err(|e| CodecError::Io(e))
    }
}

impl Decode<RawData> for Ipld {
    fn decode<R: Read + Seek>(c: &RawData, r: &mut R) -> Result<Self, CodecError> {
        Ok(Ipld::Bytes(Decode::decode(c, r)?))
    }
}

impl Decode<RawData> for Bytes {
    fn decode<R: Read + Seek>(_c: &RawData, r: &mut R) -> Result<Self, CodecError> {
        let mut buf = vec![];
        r.read_to_end(&mut buf).map_err(|e| CodecError::Io(e))?;
        Ok(buf.into())
    }
}
