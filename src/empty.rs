#[cfg(feature = "lib-serde")]
use serde::{de, Serialize, Deserialize, Serializer, Deserializer};
#[cfg(feature = "lib-rustc-serialize")]
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};

#[derive(Clone, Copy)]
pub struct Array;

#[cfg(feature = "lib-serde")]
impl Serialize for Array {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        [(); 0].serialize(serializer)
    }
}

#[cfg(feature = "lib-serde")]
impl Deserialize for Array {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer,
    {
        struct Visitor;

        impl de::Visitor for Visitor {
            type Value = Array;

            fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Array, V::Error>
                where V: de::SeqVisitor,
            {
                try!(visitor.end());
                Ok(Array)
            }
        }

        deserializer.deserialize_seq_fixed_size(0, Visitor)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl Encodable for Array {
    fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
        where S: Encoder,
    {
        [(); 0].encode(s)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl Decodable for Array {
    fn decode<D>(d: &mut D) -> Result<Array, D::Error>
        where D: Decoder,
    {
        d.read_tuple(0, |_| Ok(Array))
    }
}
