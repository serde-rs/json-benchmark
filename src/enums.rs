#[macro_export]
macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy)]
        pub enum $name {
            $($variant,)*
        }

        #[cfg(any(feature = "serde", feature = "lib-rustc-serialize"))]
        impl $name {
            fn as_str(self) -> &'static str {
                match self {
                    $( $name::$variant => $str, )*
                }
            }
        }

        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str("unit variant")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(::serde::de::Unexpected::Str(value), &self)),
                        }
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }

        #[cfg(feature = "lib-rustc-serialize")]
        impl ::rustc_serialize::Encodable for $name {
            fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
                where S: ::rustc_serialize::Encoder,
            {
                self.as_str().encode(s)
            }
        }

        #[cfg(feature = "lib-rustc-serialize")]
        impl ::rustc_serialize::Decodable for $name {
            fn decode<D>(d: &mut D) -> Result<$name, D::Error>
                where D: ::rustc_serialize::Decoder,
            {
                let string = d.read_str()?;
                match &string as &str {
                    $( $str => Ok($name::$variant), )*
                    _ => Err(d.error(
                        &format!("unknown {} variant: {}",
                        stringify!($name), string))),
                }
            }
        }
    }
}
