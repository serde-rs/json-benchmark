#[cfg(feature = "lib-rustc-serialize")]
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "serde")]
use std::fmt;

#[derive(Clone, Copy)]
pub struct Array;

#[cfg(feature = "serde")]
impl Serialize for Array {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [(); 0].serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Array {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Array;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("empty array")
            }

            fn visit_seq<V>(self, _: V) -> Result<Array, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                Ok(Array)
            }
        }

        deserializer.deserialize_tuple(0, Visitor)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl Encodable for Array {
    fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
    where
        S: Encoder,
    {
        [(); 0].encode(s)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl Decodable for Array {
    fn decode<D>(d: &mut D) -> Result<Array, D::Error>
    where
        D: Decoder,
    {
        d.read_tuple(0, |_| Ok(Array))
    }
}
