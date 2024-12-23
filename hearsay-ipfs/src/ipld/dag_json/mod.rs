use std::io::{Read, Seek, Write};

use super::{Codec, CodecError, Decode, Encode, Ipld};

/// IPLD DAG-JSON. See [IPLD spec](https://ipld.io/specs/codecs/dag-json/spec/) and [DAG-JSON spec](https://datatracker.ietf.org/doc/html/rfc8259).
pub struct DagJson;

impl Codec for DagJson {
    /// See <https://github.com/multiformats/multicodec/blob/master/table.csv>
    const CODE: u64 = 0x0129;
}

impl Encode<DagJson> for Ipld {
    fn encode<W: Write>(&self, c: &DagJson, w: &mut W) -> Result<(), CodecError> {
        todo!()
    }
}

impl Decode<DagJson> for Ipld {
    fn decode<R: Read + Seek>(c: &DagJson, r: &mut R) -> Result<Self, CodecError> {
        todo!()
    }
}
