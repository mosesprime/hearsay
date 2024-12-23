use std::{collections::BTreeMap, io::{self, Read, Seek, SeekFrom}};

use bytes::Bytes;
use cid::Cid;

use crate::ipld::{CodecError, Decode};

use super::{DagCbor, Header, MajorType};

#[inline]
pub(super) fn read_header<R: Read>(r: &mut R) -> io::Result<Header> {
    Ok(Header::from(read_u8(r)?))
}

#[inline]
fn read_u8<R: Read>(r: &mut R) -> io::Result<u8> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf)?;
    Ok(u8::from_be_bytes(buf))
}

#[inline]
fn read_u16<R: Read>(r: &mut R) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

#[inline]
fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

#[inline]
fn read_u64<R: Read>(r: &mut R) -> io::Result<u64> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf)?;
    Ok(u64::from_be_bytes(buf))
}

#[inline]
pub(super) fn read_f64<R: Read>(r: &mut R) -> io::Result<f64> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf)?;
    Ok(f64::from_be_bytes(buf))
}

#[inline]
pub(super) fn read_list<R, T>(r: &mut R, len: u64) -> Result<Vec<T>, CodecError>
where 
    R: Read + Seek,
    T: Decode<DagCbor>,
{
    let len = usize::try_from(len).map_err(|_| CodecError::NumberOutOfBounds)?;
    let mut list = Vec::with_capacity(len);
    for _ in 0..len {
        list.push(T::decode(&DagCbor, r)?);
    }
    Ok(list)
}

#[inline]
pub(super) fn read_bytes<R: Read>(r: &mut R, len: u64) -> Result<Bytes, CodecError> {
    let len = usize::try_from(len).map_err(|_| CodecError::NumberOutOfBounds)?;
    let mut buf = Vec::with_capacity(len); // TODO: cap size?
    r.take(len as u64).read_exact(&mut buf).map_err(|e| CodecError::Io(e))?;
    Ok(buf.into())
}

#[inline]
pub(super) fn read_string<R: Read>(r: &mut R, len: u64) -> Result<String, CodecError> {
    let buf = read_bytes(r, len)?;
    Ok(String::from_utf8(buf.into()).map_err(|_| CodecError::MalformedData("bytes are not in utf-8 string format"))?)
}

#[inline]
pub(super) fn read_map<R, K, V>(r: &mut R, len: u64) -> Result<BTreeMap<K, V>, CodecError>
where 
    R: Read + Seek,
    K: Decode<DagCbor> + Ord,
    V: Decode<DagCbor>,
{
    let len = usize::try_from(len).map_err(|_| CodecError::NumberOutOfBounds)?;
    let c = DagCbor;
    let mut map = BTreeMap::new();
    for _ in 0..len {
        let k = K::decode(&c, r)?;
        let v = V::decode(&c, r)?;
        if map.insert(k, v).is_some() {
            return Err(CodecError::MalformedData("duplicate map keys"));
        }
    }
    Ok(map)
}

#[inline]
pub(super) fn read_link<R: Read>(r: &mut R, header: &Header) -> Result<Cid, CodecError> {
    let len = match header.major_type {
        MajorType::ByteString => read_u8(r).map_err(|e| CodecError::Io(e))?,
        _ => return Err(CodecError::MalformedData("unexpected major type")),
    };
    let mut r = r.take(len as u64);
    if read_u8(&mut r).map_err(|e| CodecError::Io(e))? != 0 {
        return Err(CodecError::MalformedData("invalid cid prefix"));
    }
    Cid::read_bytes(&mut r).map_err(|_| CodecError::MalformedData("invalid cid data"))
}

#[inline]
pub(super) fn read_uint<R: Read>(r: &mut R, header: &Header) -> Result<u64, CodecError> {
    debug_assert!(header.major_type != MajorType::Other);
    let v = header.short_count;
    Ok(match v {
        0..=23 => return Ok(v as u64),
        24 => read_u8(r).map_err(|e| CodecError::Io(e))? as u64,
        25 => read_u16(r).map_err(|e| CodecError::Io(e))? as u64,
        26 => read_u32(r).map_err(|e| CodecError::Io(e))? as u64,
        27 => read_u64(r).map_err(|e| CodecError::Io(e))?,
        _ => return Err(CodecError::MalformedData("unexpected short count"))
    })
}

impl Decode<DagCbor> for bool {
    fn decode<R: Read + Seek>(_c: &DagCbor, r: &mut R) -> Result<Self, CodecError> {
        match read_header(r).map_err(|e| CodecError::Io(e))? {
            Header::TRUE => Ok(true),
            Header::FALSE => Ok(false),
            _ => Err(CodecError::MalformedData("unexpected header type")),
        }
    }
}

impl<T: Decode<DagCbor>> Decode<DagCbor> for Option<T> {
    fn decode<R: Read + Seek>(c: &DagCbor, r: &mut R) -> Result<Self, CodecError> {
        match read_header(r).map_err(|e| CodecError::Io(e))? {
            Header::NULL => Ok(None),
            _ => {
                r.seek(SeekFrom::Current(-1)).map_err(|e| CodecError::Io(e))?;
                Ok(Some(T::decode(c, r)?))
            }
        }
    }
}

impl<T: Decode<DagCbor>> Decode<DagCbor> for Vec<T> {
    fn decode<R: Read + Seek>(_c: &DagCbor, r: &mut R) -> Result<Self, CodecError> {
        let header = read_header(r).map_err(|e| CodecError::Io(e))?;
        if header.major_type != MajorType::Array {
            return Err(CodecError::MalformedData("unexpected major type"));
        }
        let len = read_uint(r, &header)?;
        read_list(r, len)
    }
}

impl Decode<DagCbor> for String {
    fn decode<R: Read + Seek>(_c: &DagCbor, r: &mut R) -> Result<Self, CodecError> {
        let header = read_header(r).map_err(|e| CodecError::Io(e))?;
        if header.major_type != MajorType::TextString {
            return Err(CodecError::MalformedData("unexpected major type"));
        }
        let len = read_uint(r, &header)?;
        read_string(r, len)
    }
}

impl Decode<DagCbor> for f64 {
    fn decode<R: Read + Seek>(_c: &DagCbor, r: &mut R) -> Result<Self, CodecError> {
        let f = match read_header(r).map_err(|e| CodecError::Io(e))? {
            Header::_F16 => todo!(), // TODO: decode f16?
            Header::_F32 => todo!(), // TODO: decode f32?
            Header::F64 => read_f64(r)?,
            _ => return Err(CodecError::MalformedData("unexpected header type")),
        };
        if !f.is_finite() {
            return Err(CodecError::NumberOutOfBounds);
        }
        Ok(f)
    }
}
