use std::io::{Read, Seek, Write};

use super::{Codec, CodecError, Decode, Encode, Ipld};

/// IPLD DAG-Protobuf. See {IPLD spec}(https://ipld.io/specs/codecs/dag-pb/spec/)
pub struct DagPb;

impl Codec for DagPb {
    /// See <https://github.com/multiformats/multicodec/blob/master/table.csv>
    const CODE: u64 = 0x70;
}

impl Encode<DagPb> for Ipld {
    fn encode<W: Write>(&self, c: &DagPb, w: &mut W) -> Result<(), CodecError> {
        todo!()
    }
}

impl Decode<DagPb> for Ipld {
    fn decode<R: Read + Seek>(c: &DagPb, r: &mut R) -> Result<Self, CodecError> {
        todo!()
    }
}
