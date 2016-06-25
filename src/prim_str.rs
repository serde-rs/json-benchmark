use std::marker::PhantomData;
use std::str::FromStr;

use serde::{de, Serialize, Serializer, Deserialize, Deserializer};
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct PrimStr<T>(T) where T: Copy + Ord + ToString + FromStr;

impl<T> Serialize for PrimStr<T>
    where T: Copy + Ord + ToString + FromStr,
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<T> Deserialize for PrimStr<T>
    where T: Copy + Ord + ToString + FromStr,
{
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer,
    {
        struct Visitor<T>(PhantomData<T>);

        impl<T> de::Visitor for Visitor<T>
            where T: Copy + Ord + ToString + FromStr,
        {
            type Value = PrimStr<T>;

            fn visit_str<E>(&mut self, value: &str) -> Result<PrimStr<T>, E>
                where E: de::Error,
            {
                match T::from_str(value) {
                    Ok(id) => Ok(PrimStr(id)),
                    Err(_) => Err(E::invalid_value(
                        &format!("failed to parse id: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

impl<T> Encodable for PrimStr<T>
    where T: Copy + Ord + ToString + FromStr,
{
    fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
        where S: Encoder,
    {
        self.0.to_string().encode(s)
    }
}

impl<T> Decodable for PrimStr<T>
    where T: Copy + Ord + ToString + FromStr,
{
    fn decode<D>(d: &mut D) -> Result<PrimStr<T>, D::Error>
        where D: Decoder,
    {
        let string = try!(d.read_str());
        match T::from_str(&string) {
            Ok(id) => Ok(PrimStr(id)),
            Err(_) => Err(d.error(&format!("failed to parse id: {}", string))),
        }
    }
}
