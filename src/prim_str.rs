#[cfg(feature = "serde")]
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[cfg(feature = "lib-rustc-serialize")]
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
#[cfg(feature = "serde")]
use serde::de::{self, Deserialize, Deserializer, Unexpected};
#[cfg(feature = "serde")]
use serde::ser::{Serialize, Serializer};

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct PrimStr<T>(T)
where
    T: Copy + Ord + Display + FromStr;

#[cfg(feature = "serde")]
impl<T> Serialize for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::marker::PhantomData;
        struct Visitor<T>(PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for Visitor<T>
        where
            T: Copy + Ord + Display + FromStr,
        {
            type Value = PrimStr<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number represented as string")
            }

            fn visit_str<E>(self, value: &str) -> Result<PrimStr<T>, E>
            where
                E: de::Error,
            {
                match T::from_str(value) {
                    Ok(id) => Ok(PrimStr(id)),
                    Err(_) => Err(E::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl<T> Encodable for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
    where
        S: Encoder,
    {
        self.0.to_string().encode(s)
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl<T> Decodable for PrimStr<T>
where
    T: Copy + Ord + Display + FromStr,
{
    fn decode<D>(d: &mut D) -> Result<PrimStr<T>, D::Error>
    where
        D: Decoder,
    {
        let string = d.read_str()?;
        match T::from_str(&string) {
            Ok(id) => Ok(PrimStr(id)),
            Err(_) => Err(d.error(&format!("failed to parse id: {}", string))),
        }
    }
}
