use super::Options;
use error::Result;
use std::io::Write;
use super::IntEncoding;
use de::read::BincodeRead;
use super::int::cast_u64_to_usize;

pub trait LenWidth {
    fn serialize_len<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        len: usize,
    ) -> Result<()>;

    fn serialize_str_len<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        len: usize,
    ) -> Result<()>;

    fn deserialize_len<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<usize>;

    fn deserialize_str_len<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<usize>;

}

/// Doc: Later
#[derive(Copy, Clone)]
pub struct DefaultLenWidth;

/// Doc: later
#[derive(Copy, Clone)]
pub struct YYPLenWidth;

impl LenWidth for DefaultLenWidth {
    #[inline(always)]
    fn serialize_len<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        len: usize,
    ) -> Result<()> {
        O::IntEncoding::serialize_len(ser, len)
    }

    #[inline(always)]
    fn serialize_str_len<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        len: usize,
    ) -> Result<()> {
        O::IntEncoding::serialize_len(ser, len)
    }

    #[inline(always)]
    fn deserialize_len<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<usize> {
        O::IntEncoding::deserialize_u64(de).and_then(cast_u64_to_usize)
    }

    #[inline(always)]
    fn deserialize_str_len<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<usize> {
        O::IntEncoding::deserialize_u64(de).and_then(cast_u64_to_usize)
    }

}

impl LenWidth for YYPLenWidth {
    #[inline(always)]
    fn serialize_len<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        len: usize,
    ) -> Result<()> {
        O::IntEncoding::serialize_u32(ser, len as u32)
    }

    #[inline(always)]
    fn serialize_str_len<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        len: usize,
    ) -> Result<()> {
        O::IntEncoding::serialize_u16(ser, len as u16)
    }

    #[inline(always)]
    fn deserialize_len<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<usize> {
        O::IntEncoding::deserialize_u32(de).map(|n| n as usize)
    }

    #[inline(always)]
    fn deserialize_str_len<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<usize> {
        O::IntEncoding::deserialize_u16(de).map(|n| n as usize)
    }
}
