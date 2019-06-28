#[cfg(feature = "serde")]
use std::fmt;
#[cfg(any(feature = "serde", feature = "lib-rustc-serialize"))]
use std::{ptr, slice, str};

#[cfg(feature = "lib-rustc-serialize")]
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
#[cfg(feature = "serde")]
use serde::de::{self, Deserialize, Deserializer, Unexpected};
#[cfg(feature = "serde")]
use serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy)]
pub struct Color(u32);

#[cfg(any(feature = "serde", feature = "lib-rustc-serialize"))]
const HEX_LUT: &'static [u8] = b"\
      000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F\
      202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F\
      404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F\
      606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F\
      808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9F\
      A0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBF\
      C0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDF\
      E0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF";

#[cfg(any(feature = "serde", feature = "lib-rustc-serialize"))]
impl Color {
    fn as_str<'a>(self, buf: &'a mut [u8; 6]) -> &'a str {
        let buf_ptr = buf.as_mut_ptr();
        let lut_ptr = HEX_LUT.as_ptr();

        let r = ((self.0 & 0xFF0000) >> 15) as isize;
        let g = ((self.0 & 0x00FF00) >> 7) as isize;
        let b = ((self.0 & 0x0000FF) << 1) as isize;

        unsafe {
            ptr::copy_nonoverlapping(lut_ptr.offset(r), buf_ptr, 2);
            ptr::copy_nonoverlapping(lut_ptr.offset(g), buf_ptr.offset(2), 2);
            ptr::copy_nonoverlapping(lut_ptr.offset(b), buf_ptr.offset(4), 2);

            str::from_utf8(slice::from_raw_parts(buf_ptr, buf.len())).unwrap()
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf: [u8; 6] = unsafe { ::std::mem::uninitialized() };
        serializer.serialize_str(self.as_str(&mut buf))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("color string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Color, E>
            where
                E: de::Error,
            {
                match u32::from_str_radix(value, 16) {
                    Ok(hex) => Ok(Color(hex)),
                    Err(_) => Err(E::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl Encodable for Color {
    fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
    where
        S: Encoder,
    {
        let mut buf: [u8; 6] = unsafe { ::std::mem::uninitialized() };
        self.as_str(&mut buf).encode(s)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl Decodable for Color {
    fn decode<D>(d: &mut D) -> Result<Color, D::Error>
    where
        D: Decoder,
    {
        let string = d.read_str()?;
        match u32::from_str_radix(&string, 16) {
            Ok(hex) => Ok(Color(hex)),
            Err(_) => Err(d.error(&format!("failed to parse color: {}", string))),
        }
    }
}

#[test]
fn test_color() {
    let mut buf: [u8; 6] = unsafe { ::std::mem::uninitialized() };
    let string = Color(0xA0A0A0).as_str(&mut buf);
    assert_eq!(string, "A0A0A0");
}
