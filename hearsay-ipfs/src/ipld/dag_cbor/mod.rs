//! IPLD DAG-CBOR Implimentation
//! Note: For use in IPLD DAG-CBOR only. Not nessisarily compatable otherwise.

use std::io::{self, BufRead, Write};
use cid::Cid;
use dec::*;
use enc::*;
use thiserror::Error;
use super::{Codec, CodecError, Decode, Encode, Ipld, Links};

mod dec;
mod enc;

/// Codec for [CBOR](https://datatracker.ietf.org/doc/html/rfc8949).
/// See IPLD DAG-CBOR [Spec](https://ipld.io/specs/codecs/dag-cbor/spec/).
pub struct DagCbor;

impl Codec for DagCbor {
    const CODE: u64 = 0x71;
}

impl Links for DagCbor {
    type LinkError = CodecError;

    fn links(bytes: &[u8]) -> Result<impl Iterator<Item = Cid>, Self::LinkError> {
        todo!()
    }
}

impl Encode<DagCbor> for Ipld {
    fn encode<W: Write>(&self, c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        match self {
            Self::Null => write_null(w),
            Self::Bool(b) => b.encode(c, w),
            Self::Integer(i) => i.encode(c, w),
            Self::Float(f) => f.encode(c, w),
            Self::String(s) => s.encode(c, w),
            Self::Bytes(b) => b.encode(c, w),
            Self::List(l) => l.encode(c, w),
            Self::Map(m) => m.encode(c, w),
            Self::Link(l) => l.encode(c, w),
        }
    }
}

impl Decode<DagCbor> for Ipld {
    fn decode<R: BufRead>(c: &DagCbor, r: &mut R) -> Result<Self, CodecError> {
        todo!("decode ipld from dag-cbor")
    }
}

/// 3-bit [Major Type](https://datatracker.ietf.org/doc/html/rfc8949#section-3.1).
#[repr(u8)]
pub(super) enum MajorType {
    PositiveInt = 0,
    NegativeInt = 1,
    ByteString = 2,
    TextString = 3,
    Array = 4,
    Map = 5,
    Tag = 6,
    Other = 7,
}

impl From<u8> for MajorType {
    fn from(value: u8) -> Self {
        debug_assert!(value <= 7);
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<u8> for MajorType {
    fn into(self) -> u8 {
        self as u8
    }
}

/// 5-bit additonal header data
pub(crate) type ShortCount = u8;

/// 1-byte data item header
pub(crate) struct Header {
    major_type: MajorType,
    short_count: u8,
}

impl Header {
    /// True boolean [Header]. See [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949#section-3.3).
    pub(crate) const TRUE: Header = Header::new(MajorType::Other, 20);
    /// False boolean [Header]. See [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949#section-3.3).
    pub(crate) const FALSE: Header = Header::new(MajorType::Other, 21);
    /// Null value [Header]. See [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949#section-3.3).
    pub(crate) const NULL: Header = Header::new(MajorType::Other, 22);
    /// Unused by IPLD: 16-bit float [Header]. See [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949#section-3.3).
    pub(crate) const _F16: Header = Header::new(MajorType::Other, 25);
    /// Unused by IPLD: 32-bit float [Header]. See [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949#section-3.3).);
    pub(crate) const _F32: Header = Header::new(MajorType::Other, 26);
    /// 64-bit float [Header]. See [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949#section-3.3).;
    pub(crate) const F64: Header = Header::new(MajorType::Other, 27);
    /// [Header] to close indefinite-length items. See [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949#section-3.2.1).;
    pub(crate) const BREAK: Header = Header::new(MajorType::Other, 31);

    pub(crate) const fn new(major_type: MajorType, short_count: ShortCount) -> Self {
        debug_assert!(short_count <= 31);
        Self { 
            major_type,
            short_count,
        }
    }
}

impl From<u8> for Header {
    fn from(value: u8) -> Self {
        Self { 
            major_type: (value << 5).into(),
            short_count: value & 0x0001_1111,
        }
    }
}

impl Into<u8> for Header {
    fn into(self) -> u8 {
        (self.major_type as u8) << 5 | self.short_count
    }
}

#[derive(Debug, Error)]
pub enum DagCborError {
    #[error("{}", 0)]
    IoError(#[from] io::Error),
    #[error("number not properly confined")]
    NumberOutOfBounds,
}

#[cfg(test)]
mod tests {

    #[test]
    fn dag_cbor_roundtrips(){
        todo!()
    }
}
