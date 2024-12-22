use std::{cmp::Ordering, collections::BTreeMap, i128, io::{self, Write}};

use bytes::Bytes;
use cid::Cid;

use crate::ipld::{CodecError, Encode};

use super::{DagCbor, Header, MajorType};

#[inline]
pub(super) fn write_null<W: Write>(w: &mut W) -> Result<(), CodecError> {
    w.write_all(&[Header::NULL.into()]).map_err(|e| CodecError::Io(e))
}

/// Use the minimal encoding to convey the data item. See <https://datatracker.ietf.org/doc/html/rfc8949#section-4.2.1>.
#[inline]
fn write_uint<W: Write>(w: &mut W, major_type: MajorType, data: u64) -> io::Result<()> {
    match data {
        0..=23 => w.write_all(&[Header::new(major_type, data as u8).into()]),
        24..=255 => {
            let buf = [Header::new(major_type, 24).into(), data as u8];
            w.write_all(&buf)
        },
        256..=65_535 => {
            let mut buf = [Header::new(major_type, 25).into(), 0, 0];
            buf[1..].copy_from_slice(&data.to_be_bytes());
            w.write_all(&buf)
        },
        65_536..=4_294_967_295 => {
            let mut buf = [Header::new(major_type, 26).into(), 0, 0, 0, 0];
            buf[1..].copy_from_slice(&data.to_be_bytes());
            w.write_all(&buf)
        },
        ..=u64::MAX => {
            let mut buf = [Header::new(major_type, 27).into(), 0, 0, 0, 0, 0, 0, 0, 0];
            buf[1..].copy_from_slice(&data.to_be_bytes());
            w.write_all(&buf)
        },
    }
}

impl Encode<DagCbor> for bool {
    fn encode<W: Write>(&self, _: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        let buf = match self {
            true => [Header::TRUE.into()],
            false => [Header::FALSE.into()],
        };
        w.write_all(&buf).map_err(|e| CodecError::Io(e))
    }
}

impl<T: Encode<DagCbor>> Encode<DagCbor> for Option<T> {
    fn encode<W: Write>(&self, c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        if let Some(t) = self {
            return Ok(t.encode(c, w)?);
        }
        w.write_all(&[Header::NULL.into()]).map_err(|e| CodecError::Io(e))
    }
}

impl<T: Encode<DagCbor>> Encode<DagCbor> for Vec<T> {
    fn encode<W: Write>(&self, c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        write_uint(w, MajorType::Array, self.len() as u64).map_err(|e| CodecError::Io(e))?;
        for inner in self {
            inner.encode(c, w)?;
        }
        Ok(())
    }
}

/*impl Encode<DagCbor> for f32 { // TODO: idk if nessesary to support
    fn encode<W: Write>(&self, _c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        if !self.is_finite() {
            return Err(CodecError::NumberOutOfBounds);
        }
        let mut buf = [Header::F32.into(), 0, 0, 0, 0];
        buf[1..].copy_from_slice(&self.to_be_bytes());
        w.write_all(&buf).map_err(|e| CodecError::Io(e))
    }
}*/

impl Encode<DagCbor> for f64 {
    fn encode<W: Write>(&self, _c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        if !self.is_finite() {
            return Err(CodecError::NumberOutOfBounds);
        }
        let mut buf = [Header::F64.into(), 0, 0, 0, 0, 0, 0, 0, 0];
        buf[1..].copy_from_slice(&self.to_be_bytes());
        w.write_all(&buf).map_err(|e| CodecError::Io(e))
    }
}

impl Encode<DagCbor> for i128 {
    fn encode<W: Write>(&self, _c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        const MAX: i128 = 2i128.pow(64) - 1;
        const MIN: i128 = -(2i128.pow(64));
        let major_type = match self {
            MIN..0 => MajorType::NegativeInt,
            ..=MAX => MajorType::PositiveInt,
            _ => return Err(CodecError::NumberOutOfBounds),
        };
        write_uint(w, major_type, self.abs() as u64).map_err(|e| CodecError::Io(e))
    }
}

impl Encode<DagCbor> for String {
    fn encode<W: Write>(&self, _c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        write_uint(w, MajorType::TextString, self.len() as u64).map_err(|e| CodecError::Io(e))?;
        w.write_all(self.as_bytes()).map_err(|e| CodecError::Io(e))
    }
}

impl Encode<DagCbor> for Bytes {
    fn encode<W: Write>(&self, _c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        write_uint(w, MajorType::ByteString, self.len() as u64).map_err(|e| CodecError::Io(e))?;
        w.write_all(&self).map_err(|e| CodecError::Io(e))
    }
}

impl Encode<DagCbor> for Cid {
    fn encode<W: Write>(&self, _c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        // tag 42 to identify byte string as a CID
        write_uint(w, MajorType::Tag, 42u64).map_err(|e| CodecError::Io(e))?;
        let buf = self.to_bytes();
        let len = buf.len();
        // 0x00 prefix to denote multibase CID
        write_uint(w, MajorType::ByteString, (len + 1) as u64).map_err(|e| CodecError::Io(e))?;
        w.write_all(&[0u8]).map_err(|e| CodecError::Io(e))?;
        w.write_all(&buf[..len]).map_err(|e| CodecError::Io(e))
    }
}

impl<T: Encode<DagCbor> + 'static> Encode<DagCbor> for BTreeMap<String, T> { // TODO: idk if the 'static is nessesary
    fn encode<W: Write>(&self, c: &DagCbor, w: &mut W) -> Result<(), CodecError> {
        write_uint(w, MajorType::Map, self.len() as u64).map_err(|e| CodecError::Io(e))?;
        // Ordering for [RFC 8049](https://datatracker.ietf.org/doc/html/rfc8949#section-4.2.3).
        // Use old ordering for compatability [RFC 7049](https://datatracker.ietf.org/doc/html/rfc7049#section-3.9).
        let mut order = Vec::from_iter(self);
        order.sort_unstable_by(|&(a, _), &(b, _)| match a.len().cmp(&b.len()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.cmp(b),
        });
        for (k, v) in order {
            k.encode(c, w)?;
            v.encode(c, w)?;
        }
        Ok(())
    }
}
