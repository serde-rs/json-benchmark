#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap as Map;

pub type Canada = FeatureCollection;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct FeatureCollection {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub obj_type: ObjType,
    pub features: Vec<Feature>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Feature {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub obj_type: ObjType,
    pub properties: Map<String, String>,
    pub geometry: Geometry,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Geometry {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub obj_type: ObjType,
    pub coordinates: Vec<Vec<(Latitude, Longitude)>>,
}

pub type Latitude = f32;
pub type Longitude = f32;

enum_str!(ObjType {
    FeatureCollection("FeatureCollection"),
    Feature("Feature"),
    Polygon("Polygon"),
});

////////////////////////////////////////////////////////////////////////////////
// #[derive(RustcEncodable, RustcDecodable)] but with the field renames

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for FeatureCollection {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<FeatureCollection, __D::Error> {
        __arg_0.read_struct("FeatureCollection", 2usize, |_d| -> _ {
            ::std::result::Result::Ok(FeatureCollection {
                obj_type: match _d.read_struct_field(
                    "type",
                    0usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                features: match _d.read_struct_field(
                    "features",
                    1usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
            })
        })
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Encodable for FeatureCollection {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            FeatureCollection {
                obj_type: ref __self_0_0,
                features: ref __self_0_1,
            } => __arg_0.emit_struct("FeatureCollection", 2usize, |_e| -> _ {
                match _e.emit_struct_field("type", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("features", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                });
            }),
        }
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for Feature {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<Feature, __D::Error> {
        __arg_0.read_struct("Feature", 3usize, |_d| -> _ {
            ::std::result::Result::Ok(Feature {
                obj_type: match _d.read_struct_field(
                    "type",
                    0usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                properties: match _d.read_struct_field(
                    "properties",
                    1usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                geometry: match _d.read_struct_field(
                    "geometry",
                    2usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
            })
        })
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Encodable for Feature {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            Feature {
                obj_type: ref __self_0_0,
                properties: ref __self_0_1,
                geometry: ref __self_0_2,
            } => __arg_0.emit_struct("Feature", 3usize, |_e| -> _ {
                match _e.emit_struct_field("type", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("properties", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("geometry", 2usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_2), _e)
                });
            }),
        }
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for Geometry {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<Geometry, __D::Error> {
        __arg_0.read_struct("Geometry", 2usize, |_d| -> _ {
            ::std::result::Result::Ok(Geometry {
                obj_type: match _d.read_struct_field(
                    "type",
                    0usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                coordinates: match _d.read_struct_field(
                    "coordinates",
                    1usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
            })
        })
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Encodable for Geometry {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            Geometry {
                obj_type: ref __self_0_0,
                coordinates: ref __self_0_1,
            } => __arg_0.emit_struct("Geometry", 2usize, |_e| -> _ {
                match _e.emit_struct_field("type", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("coordinates", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                });
            }),
        }
    }
}
