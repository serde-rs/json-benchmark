#[macro_export]
macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            fn as_str(self) -> &'static str {
                match self {
                    $( $name::$variant => $str, )*
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: ::serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer,
            {
                struct Visitor;

                impl ::serde::de::Visitor for Visitor {
                    type Value = $name;

                    fn visit_str<E>(&mut self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(
                                &format!("unknown {} variant: {}",
                                stringify!($name), value))),
                        }
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }

        impl ::rustc_serialize::Encodable for $name {
            fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
                where S: ::rustc_serialize::Encoder,
            {
                self.as_str().encode(s)
            }
        }

        impl ::rustc_serialize::Decodable for $name {
            fn decode<D>(d: &mut D) -> Result<$name, D::Error>
                where D: ::rustc_serialize::Decoder,
            {
                let string = try!(d.read_str());
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