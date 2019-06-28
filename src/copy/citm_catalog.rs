#[cfg(feature = "lib-serde")]
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap as Map;

use crate::empty;
use crate::prim_str::PrimStr;

#[cfg_attr(feature = "lib-serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "lib-serde",
    serde(deny_unknown_fields, rename_all = "camelCase")
)]
pub struct CitmCatalog {
    pub area_names: Map<IdStr, String>,
    pub audience_sub_category_names: Map<IdStr, String>,
    pub block_names: Map<IdStr, String>,
    pub events: Map<IdStr, Event>,
    pub performances: Vec<Performance>,
    pub seat_category_names: Map<IdStr, String>,
    pub sub_topic_names: Map<IdStr, String>,
    pub subject_names: Map<IdStr, String>,
    pub topic_names: Map<IdStr, String>,
    pub topic_sub_topics: Map<IdStr, Vec<Id>>,
    pub venue_names: Map<String, String>,
}

pub type Id = u32;
pub type IdStr = PrimStr<u32>;

#[cfg_attr(feature = "lib-serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "lib-serde",
    serde(deny_unknown_fields, rename_all = "camelCase")
)]
pub struct Event {
    pub description: (),
    pub id: Id,
    pub logo: Option<String>,
    pub name: String,
    pub sub_topic_ids: Vec<Id>,
    pub subject_code: (),
    pub subtitle: (),
    pub topic_ids: Vec<Id>,
}

#[cfg_attr(feature = "lib-serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "lib-serde",
    serde(deny_unknown_fields, rename_all = "camelCase")
)]
pub struct Performance {
    pub event_id: Id,
    pub id: Id,
    pub logo: Option<String>,
    pub name: (),
    pub prices: Vec<Price>,
    pub seat_categories: Vec<SeatCategory>,
    pub seat_map_image: (),
    pub start: u64,
    pub venue_code: String,
}

#[cfg_attr(feature = "lib-serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "lib-serde",
    serde(deny_unknown_fields, rename_all = "camelCase")
)]
pub struct Price {
    pub amount: u32,
    pub audience_sub_category_id: Id,
    pub seat_category_id: Id,
}

#[cfg_attr(feature = "lib-serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "lib-serde",
    serde(deny_unknown_fields, rename_all = "camelCase")
)]
pub struct SeatCategory {
    pub areas: Vec<Area>,
    pub seat_category_id: Id,
}

#[cfg_attr(feature = "lib-serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "lib-serde",
    serde(deny_unknown_fields, rename_all = "camelCase")
)]
pub struct Area {
    pub area_id: Id,
    pub block_ids: empty::Array,
}

////////////////////////////////////////////////////////////////////////////////
// #[derive(RustcEncodable, RustcDecodable)] but with the field renames

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for CitmCatalog {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<CitmCatalog, __D::Error> {
        __arg_0.read_struct("CitmCatalog", 11usize, |_d| -> _ {
            ::std::result::Result::Ok(CitmCatalog {
                area_names: match _d.read_struct_field(
                    "areaNames",
                    0usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                audience_sub_category_names: match _d.read_struct_field(
                    "audienceSubCategoryNames",
                    1usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                block_names: match _d.read_struct_field(
                    "blockNames",
                    2usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                events: match _d.read_struct_field(
                    "events",
                    3usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                performances: match _d.read_struct_field(
                    "performances",
                    4usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                seat_category_names: match _d.read_struct_field(
                    "seatCategoryNames",
                    5usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                sub_topic_names: match _d.read_struct_field(
                    "subTopicNames",
                    6usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                subject_names: match _d.read_struct_field(
                    "subjectNames",
                    7usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                topic_names: match _d.read_struct_field(
                    "topicNames",
                    8usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                topic_sub_topics: match _d.read_struct_field(
                    "topicSubTopics",
                    9usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                venue_names: match _d.read_struct_field(
                    "venueNames",
                    10usize,
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
impl ::rustc_serialize::Encodable for CitmCatalog {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            CitmCatalog {
                area_names: ref __self_0_0,
                audience_sub_category_names: ref __self_0_1,
                block_names: ref __self_0_2,
                events: ref __self_0_3,
                performances: ref __self_0_4,
                seat_category_names: ref __self_0_5,
                sub_topic_names: ref __self_0_6,
                subject_names: ref __self_0_7,
                topic_names: ref __self_0_8,
                topic_sub_topics: ref __self_0_9,
                venue_names: ref __self_0_10,
            } => __arg_0.emit_struct("CitmCatalog", 11usize, |_e| -> _ {
                match _e.emit_struct_field("areaNames", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("audienceSubCategoryNames", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("blockNames", 2usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_2), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("events", 3usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_3), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("performances", 4usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_4), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("seatCategoryNames", 5usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_5), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("subTopicNames", 6usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_6), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("subjectNames", 7usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_7), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("topicNames", 8usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_8), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("topicSubTopics", 9usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_9), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("venueNames", 10usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_10), _e)
                });
            }),
        }
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for Event {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<Event, __D::Error> {
        __arg_0.read_struct("Event", 8usize, |_d| -> _ {
            ::std::result::Result::Ok(Event {
                description: match _d.read_struct_field(
                    "description",
                    0usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                id: match _d.read_struct_field("id", 1usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                logo: match _d.read_struct_field(
                    "logo",
                    2usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                name: match _d.read_struct_field(
                    "name",
                    3usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                sub_topic_ids: match _d.read_struct_field(
                    "subTopicIds",
                    4usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                subject_code: match _d.read_struct_field(
                    "subjectCode",
                    5usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                subtitle: match _d.read_struct_field(
                    "subtitle",
                    6usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                topic_ids: match _d.read_struct_field(
                    "topicIds",
                    7usize,
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
impl ::rustc_serialize::Encodable for Event {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            Event {
                description: ref __self_0_0,
                id: ref __self_0_1,
                logo: ref __self_0_2,
                name: ref __self_0_3,
                sub_topic_ids: ref __self_0_4,
                subject_code: ref __self_0_5,
                subtitle: ref __self_0_6,
                topic_ids: ref __self_0_7,
            } => __arg_0.emit_struct("Event", 8usize, |_e| -> _ {
                match _e.emit_struct_field("description", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("id", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("logo", 2usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_2), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("name", 3usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_3), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("subTopicIds", 4usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_4), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("subjectCode", 5usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_5), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("subtitle", 6usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_6), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("topicIds", 7usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_7), _e)
                });
            }),
        }
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for Performance {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<Performance, __D::Error> {
        __arg_0.read_struct("Performance", 9usize, |_d| -> _ {
            ::std::result::Result::Ok(Performance {
                event_id: match _d.read_struct_field(
                    "eventId",
                    0usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                id: match _d.read_struct_field("id", 1usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                logo: match _d.read_struct_field(
                    "logo",
                    2usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                name: match _d.read_struct_field(
                    "name",
                    3usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                prices: match _d.read_struct_field(
                    "prices",
                    4usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                seat_categories: match _d.read_struct_field(
                    "seatCategories",
                    5usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                seat_map_image: match _d.read_struct_field(
                    "seatMapImage",
                    6usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                start: match _d.read_struct_field(
                    "start",
                    7usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                venue_code: match _d.read_struct_field(
                    "venueCode",
                    8usize,
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
impl ::rustc_serialize::Encodable for Performance {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            Performance {
                event_id: ref __self_0_0,
                id: ref __self_0_1,
                logo: ref __self_0_2,
                name: ref __self_0_3,
                prices: ref __self_0_4,
                seat_categories: ref __self_0_5,
                seat_map_image: ref __self_0_6,
                start: ref __self_0_7,
                venue_code: ref __self_0_8,
            } => __arg_0.emit_struct("Performance", 9usize, |_e| -> _ {
                match _e.emit_struct_field("eventId", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("id", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("logo", 2usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_2), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("name", 3usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_3), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("prices", 4usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_4), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("seatCategories", 5usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_5), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("seatMapImage", 6usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_6), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("start", 7usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_7), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("venueCode", 8usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_8), _e)
                });
            }),
        }
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for Price {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<Price, __D::Error> {
        __arg_0.read_struct("Price", 3usize, |_d| -> _ {
            ::std::result::Result::Ok(Price {
                amount: match _d.read_struct_field(
                    "amount",
                    0usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                audience_sub_category_id: match _d.read_struct_field(
                    "audienceSubCategoryId",
                    1usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                seat_category_id: match _d.read_struct_field(
                    "seatCategoryId",
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
impl ::rustc_serialize::Encodable for Price {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            Price {
                amount: ref __self_0_0,
                audience_sub_category_id: ref __self_0_1,
                seat_category_id: ref __self_0_2,
            } => __arg_0.emit_struct("Price", 3usize, |_e| -> _ {
                match _e.emit_struct_field("amount", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("audienceSubCategoryId", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("seatCategoryId", 2usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_2), _e)
                });
            }),
        }
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for SeatCategory {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<SeatCategory, __D::Error> {
        __arg_0.read_struct("SeatCategory", 2usize, |_d| -> _ {
            ::std::result::Result::Ok(SeatCategory {
                areas: match _d.read_struct_field(
                    "areas",
                    0usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                seat_category_id: match _d.read_struct_field(
                    "seatCategoryId",
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
impl ::rustc_serialize::Encodable for SeatCategory {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            SeatCategory {
                areas: ref __self_0_0,
                seat_category_id: ref __self_0_1,
            } => __arg_0.emit_struct("SeatCategory", 2usize, |_e| -> _ {
                match _e.emit_struct_field("areas", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("seatCategoryId", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                });
            }),
        }
    }
}

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for Area {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<Area, __D::Error> {
        __arg_0.read_struct("Area", 2usize, |_d| -> _ {
            ::std::result::Result::Ok(Area {
                area_id: match _d.read_struct_field(
                    "areaId",
                    0usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                block_ids: match _d.read_struct_field(
                    "blockIds",
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
impl ::rustc_serialize::Encodable for Area {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            Area {
                area_id: ref __self_0_0,
                block_ids: ref __self_0_1,
            } => __arg_0.emit_struct("Area", 2usize, |_e| -> _ {
                match _e.emit_struct_field("areaId", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("blockIds", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                });
            }),
        }
    }
}
