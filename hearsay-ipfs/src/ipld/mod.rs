use std::{any::TypeId, collections::BTreeMap, fmt, io::{self, Cursor, Read, Seek, Write}};

use bytes::Bytes;
use cid::Cid;
use dag_cbor::DagCbor;
use dag_json::DagJson;
use dag_pb::DagPb;
use thiserror::Error;

use crate::Block;

mod dag_cbor;
mod dag_json;
mod dag_pb;
mod raw;

pub trait Encode<C: Codec + ?Sized> {
    fn encode<W: Write>(&self, c: &C, w: &mut W) -> Result<(), CodecError>;
}

pub trait Decode<C: Codec>: Sized {
    fn decode<R: Read + Seek>(c: &C, r: &mut R) -> Result<Self, CodecError>;
}

pub trait Codec: Sized {
    /// See <https://github.com/multiformats/multicodec/blob/master/table.csv>.
    const CODE: u64;

    fn encode<T: Encode<Self>, W: Write>(&self, data: &T, w: &mut W) -> Result<(), CodecError> {
        data.encode(self, w)
    }

    fn encode_to_vec<T: Encode<Self>>(&self, data: &T) -> Result<Vec<u8>, CodecError> {
        let mut out = vec![];
        data.encode(self, &mut out)?;
        Ok(out)
    }

    fn decode<T: Decode<Self>, R: Read + Seek>(&self, r: &mut R) -> Result<T, CodecError> {
        T::decode(self, r)
    }

    fn decode_from_slice<T: Decode<Self>>(&self, bytes: &[u8]) -> Result<T, CodecError> {
        Self::decode(&self, &mut Cursor::new(bytes))
    }
}

/// Identify a specific multicodec format. See <https://github.com/multiformats/multicodec/blob/master/table.csv>
#[repr(u64)]
pub enum CodecKind {
    /// No codec 0x55
    Raw = 0x55,
    /// DAG-Protobuf codec 0x70
    DagPb = DagPb::CODE,
    /// DAG-CBOR codec 0x71
    DagCbor = DagCbor::CODE,
    /// DAG-JSON codec 0x0129
    DagJson = DagJson::CODE,
}

impl TryFrom<u64> for CodecKind {
    type Error = CodecError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            0x55 => CodecKind::Raw,
            DagPb::CODE => CodecKind::DagPb,
            DagCbor::CODE => CodecKind::DagCbor,
            DagJson::CODE => CodecKind::DagJson,
            _ => return Err(CodecError::UnsupportedCodec(value)),
        })
    }
}

#[derive(Debug, Error)]
pub enum CodecError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("number is not properly contained")]
    NumberOutOfBounds,
    #[error("codec id {:x?} is not supported", 0)]
    UnsupportedCodec(u64),
    #[error("malformed data: {}", 0)]
    MalformedData(&'static str),
}

#[derive(Debug)]
pub enum IpldError {
    BadConversion {
        expected: IpldKind,
        found: IpldKind,
    },
    InvalidKind {
        from: IpldKind,
        into: TypeId,
    },
}

/// IPLD data-model, see [reference](https://ipld.io/docs/data-model/kinds/).
#[derive(PartialEq)]
pub enum Ipld {
    Null,
    Bool(bool),
    /// IPLD DAG-CBOR supports -(2^64) to 2^64-1, so we use i128 to cover this space.
    Integer(i128),
    /// Floating point number. Should avoid using in IPLD. CBOR supports f16 and f32 but IPLD DAG-CBOR does not.
    Float(f64),
    /// UTF-8 encoded text string.
    String(String),
    /// Arbitrary byte string.
    Bytes(Bytes),
    List(Vec<Ipld>),
    Map(BTreeMap<String, Ipld>),
    Link(Cid),
}

impl fmt::Debug for Ipld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            match self {
                Self::Null => write!(f, "Null"),
                Self::Bool(b) => write!(f, "Bool({:?})", b),
                Self::Integer(i) => write!(f, "Integer({:?})", i),
                Self::Float(n) => write!(f, "Float({:?})", n),
                Self::String(s) => write!(f, "String({:?})", s),
                Self::Bytes(b) => write!(f, "Bytes({:?})", b),
                Self::List(l) => write!(f, "List({:#?})", l),
                Self::Map(m) => write!(f, "Map({:#?})", m),
                Self::Link(cid) => write!(f, "Link({})", cid),
            }
        } else {
            match self {
                Self::Null => write!(f, "null"),
                Self::Bool(b) => write!(f, "{:?}", b),
                Self::Integer(i) => write!(f, "{:?}", i),
                Self::Float(n) => write!(f, "{:?}", n),
                Self::String(s) => write!(f, "{:?}", s),
                Self::Bytes(b) => write!(f, "{:?}", b),
                Self::List(l) => write!(f, "{:?}", l),
                Self::Map(m) => write!(f, "{:?}", m),
                Self::Link(cid) => write!(f, "{}", cid),
            }
        }
    }
}

impl Ipld {
    pub fn kind(&self) -> IpldKind {
        match self {
            Self::Null => IpldKind::Null,
            Self::Bool(_) => IpldKind::Bool,
            Self::Integer(_) => IpldKind::Integer,
            Self::Float(_) => IpldKind::Float,
            Self::String(_) => IpldKind::String,
            Self::Bytes(_) => IpldKind::Bytes,
            Self::List(_) => IpldKind::List,
            Self::Map(_) => IpldKind::Map,
            Self::Link(_) => IpldKind::Link,
        }
    }

}

impl TryFrom<Block> for Ipld {
    type Error = CodecError;
    fn try_from(value: Block) -> Result<Self, Self::Error> {
        let kind = CodecKind::try_from(value.cid().codec())?;
        Ok(match kind {
            CodecKind::Raw => Ipld::Bytes(value.inner().clone()), // PERF: remove clone?
            CodecKind::DagPb => {
                let pb = DagPb;
                pb.decode_from_slice(value.data())?
            },
            CodecKind::DagCbor => {
                let cbor = DagCbor;
                cbor.decode_from_slice(value.data())?
            },
            CodecKind::DagJson => {
                let json = DagJson;
                json.decode_from_slice(value.data())?
            },
        })
    }
}

impl From<Option<Ipld>> for Ipld {
    fn from(value: Option<Ipld>) -> Self {
        match value {
            Some(inner) => Self::from(inner),
            None => Self::Null,
        }
    }
}

impl From<String> for Ipld {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for Ipld {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<u8> for Ipld {
    fn from(value: u8) -> Self {
        Self::Integer(i128::from(value))
    }
}
impl From<u16> for Ipld {
    fn from(value: u16) -> Self {
        Self::Integer(i128::from(value))
    }
}
impl From<u32> for Ipld {
    fn from(value: u32) -> Self {
        Self::Integer(i128::from(value))
    }
}
impl From<u64> for Ipld {
    fn from(value: u64) -> Self {
        Self::Integer(i128::from(value))
    }
}

impl From<i8> for Ipld {
    fn from(value: i8) -> Self {
        Self::Integer(i128::from(value))
    }
}

impl From<i16> for Ipld {
    fn from(value: i16) -> Self {
        Self::Integer(i128::from(value))
    }
}

impl From<i32> for Ipld {
    fn from(value: i32) -> Self {
        Self::Integer(i128::from(value))
    }
}

impl From<i64> for Ipld {
    fn from(value: i64) -> Self {
        Self::Integer(i128::from(value))
    }
}

impl From<i128> for Ipld {
    fn from(value: i128) -> Self {
        Self::Integer(value)
    }
}

impl From<f32> for Ipld {
    fn from(value: f32) -> Self {
        Self::Float(f64::from(value))
    }
}

impl From<f64> for Ipld {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<Bytes> for Ipld {
    fn from(value: Bytes) -> Self {
        Self::Bytes(value)
    }
}

impl From<Cid> for Ipld {
    fn from(value: Cid) -> Self {
        Self::Link(value)
    }
}


impl From<Vec<Ipld>> for Ipld {
    fn from(value: Vec<Ipld>) -> Self {
        Self::List(value)
    }
}

impl From<BTreeMap<String, Ipld>> for Ipld {
    fn from(value: BTreeMap<String, Ipld>) -> Self {
        Self::Map(value)
    }
}

#[derive(Clone, Debug)]
pub enum IpldKind {
    Null,
    Bool,
    Integer,
    Float,
    String,
    Bytes,
    List,
    Map,
    Link,
}

macro_rules! ipld {
    (null) => {
        $crate::ipld::Ipld::Null
    };

    (true) => {
        $crate::ipld::Ipld::Bool(true)
    };

    (false) => {
        $crate::ipld::Ipld::Bool(false)
    };

    // empty map
    ({}) => {
        $crate::ipld::Ipld::Map(BTreeMap::new())
    };

    // filled map
    ({ $($tt:tt)+ }) => {
        $crate::ipld::Ipld::Map({
            let mut object = BTreeMap::new();
            ipld!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    // 
    ($other:expr) => {
        { $crate::ipld::Ipld::from($other) }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipld_macro() {
        assert_eq!(ipld!(null), Ipld::Null);
        assert_eq!(ipld!(true), Ipld::Bool(true));
        assert_eq!(ipld!(i64::MIN), Ipld::Integer(i64::MIN.into()));
        assert_eq!(ipld!(3.14f64), Ipld::Float(3.14f64));
        assert_eq!(ipld!(String::from("banana")), Ipld::String(String::from("banana")));
        assert_eq!(ipld!(Bytes::from_static(b"banana")), Ipld::Bytes(Bytes::from_static(b"banana")));
        // TODO: add ipld list, map, link tests
    }
    
    #[test]
    fn test_ipld_conversions() {
        assert_eq!(Ipld::from(true), Ipld::Bool(true));
        assert_eq!(Ipld::from(u32::MAX), Ipld::Integer(u32::MAX as i128));
        assert_eq!(Ipld::from(i64::MIN), Ipld::Integer(i64::MIN as i128));
        // TODO: finish ipld conversion tests
    }
}
