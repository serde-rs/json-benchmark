#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::color::Color;
use crate::empty;
use crate::prim_str::PrimStr;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct Twitter {
    pub statuses: Vec<Status>,
    pub search_metadata: SearchMetadata,
}

pub type LongId = u64;
pub type ShortId = u32;
pub type LongIdStr = PrimStr<LongId>;
pub type ShortIdStr = PrimStr<ShortId>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct Status {
    pub metadata: Metadata,
    pub created_at: String,
    pub id: LongId,
    pub id_str: LongIdStr,
    pub text: String,
    pub source: String,
    pub truncated: bool,
    pub in_reply_to_status_id: Option<LongId>,
    pub in_reply_to_status_id_str: Option<LongIdStr>,
    pub in_reply_to_user_id: Option<ShortId>,
    pub in_reply_to_user_id_str: Option<ShortIdStr>,
    pub in_reply_to_screen_name: Option<String>,
    pub user: User,
    pub geo: (),
    pub coordinates: (),
    pub place: (),
    pub contributors: (),
    pub retweeted_status: Option<Box<Status>>,
    pub retweet_count: u32,
    pub favorite_count: u32,
    pub entities: StatusEntities,
    pub favorited: bool,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub lang: LanguageCode,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct Metadata {
    pub result_type: ResultType,
    pub iso_language_code: LanguageCode,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct User {
    pub id: ShortId,
    pub id_str: ShortIdStr,
    pub name: String,
    pub screen_name: String,
    pub location: String,
    pub description: String,
    pub url: Option<String>,
    pub entities: UserEntities,
    pub protected: bool,
    pub followers_count: u32,
    pub friends_count: u32,
    pub listed_count: u32,
    pub created_at: String,
    pub favourites_count: u32,
    pub utc_offset: Option<i32>,
    pub time_zone: Option<String>,
    pub geo_enabled: bool,
    pub verified: bool,
    pub statuses_count: u32,
    pub lang: LanguageCode,
    pub contributors_enabled: bool,
    pub is_translator: bool,
    pub is_translation_enabled: bool,
    pub profile_background_color: Color,
    pub profile_background_image_url: String,
    pub profile_background_image_url_https: String,
    pub profile_background_tile: bool,
    pub profile_image_url: String,
    pub profile_image_url_https: String,
    pub profile_banner_url: Option<String>,
    pub profile_link_color: Color,
    pub profile_sidebar_border_color: Color,
    pub profile_sidebar_fill_color: Color,
    pub profile_text_color: Color,
    pub profile_use_background_image: bool,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub following: bool,
    pub follow_request_sent: bool,
    pub notifications: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct UserEntities {
    pub url: Option<UserUrl>,
    pub description: UserEntitiesDescription,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct UserUrl {
    pub urls: Vec<Url>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct Url {
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
    pub indices: Indices,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct UserEntitiesDescription {
    pub urls: Vec<Url>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct StatusEntities {
    pub hashtags: Vec<Hashtag>,
    pub symbols: empty::Array,
    pub urls: Vec<Url>,
    pub user_mentions: Vec<UserMention>,
    pub media: Option<Vec<Media>>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct Hashtag {
    pub text: String,
    pub indices: Indices,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct UserMention {
    pub screen_name: String,
    pub name: String,
    pub id: ShortId,
    pub id_str: ShortIdStr,
    pub indices: Indices,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Media {
    pub id: LongId,
    pub id_str: LongIdStr,
    pub indices: Indices,
    pub media_url: String,
    pub media_url_https: String,
    pub url: String,
    pub display_url: String,
    pub expanded_url: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub media_type: String,
    pub sizes: Sizes,
    pub source_status_id: Option<LongId>,
    pub source_status_id_str: Option<LongIdStr>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct Sizes {
    pub medium: Size,
    pub small: Size,
    pub thumb: Size,
    pub large: Size,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct Size {
    pub w: u16,
    pub h: u16,
    pub resize: Resize,
}

pub type Indices = (u8, u8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct SearchMetadata {
    pub completed_in: f32,
    pub max_id: LongId,
    pub max_id_str: LongIdStr,
    pub next_results: String,
    pub query: String,
    pub refresh_url: String,
    pub count: u8,
    pub since_id: LongId,
    pub since_id_str: LongIdStr,
}

enum_str!(Resize {
    Fit("fit"),
    Crop("crop"),
});

enum_str!(LanguageCode {
    Cn("zh-cn"),
    En("en"),
    Es("es"),
    It("it"),
    Ja("ja"),
    Zh("zh"),
});

enum_str!(ResultType {
    Recent("recent"),
});

////////////////////////////////////////////////////////////////////////////////
// #[derive(RustcEncodable, RustcDecodable)] but with the field renames

#[cfg(feature = "lib-rustc-serialize")]
impl ::rustc_serialize::Decodable for Media {
    fn decode<__D: ::rustc_serialize::Decoder>(
        __arg_0: &mut __D,
    ) -> ::std::result::Result<Media, __D::Error> {
        __arg_0.read_struct("Media", 12usize, |_d| -> _ {
            ::std::result::Result::Ok(Media {
                id: match _d.read_struct_field("id", 0usize, ::rustc_serialize::Decodable::decode) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                id_str: match _d.read_struct_field(
                    "id_str",
                    1usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                indices: match _d.read_struct_field(
                    "indices",
                    2usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                media_url: match _d.read_struct_field(
                    "media_url",
                    3usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                media_url_https: match _d.read_struct_field(
                    "media_url_https",
                    4usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                url: match _d.read_struct_field("url", 5usize, ::rustc_serialize::Decodable::decode)
                {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                display_url: match _d.read_struct_field(
                    "display_url",
                    6usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                expanded_url: match _d.read_struct_field(
                    "expanded_url",
                    7usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                media_type: match _d.read_struct_field(
                    "type",
                    8usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                sizes: match _d.read_struct_field(
                    "sizes",
                    9usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                source_status_id: match _d.read_struct_field(
                    "source_status_id",
                    10usize,
                    ::rustc_serialize::Decodable::decode,
                ) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                },
                source_status_id_str: match _d.read_struct_field(
                    "source_status_id_str",
                    11usize,
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
impl ::rustc_serialize::Encodable for Media {
    fn encode<__S: ::rustc_serialize::Encoder>(
        &self,
        __arg_0: &mut __S,
    ) -> ::std::result::Result<(), __S::Error> {
        match *self {
            Media {
                id: ref __self_0_0,
                id_str: ref __self_0_1,
                indices: ref __self_0_2,
                media_url: ref __self_0_3,
                media_url_https: ref __self_0_4,
                url: ref __self_0_5,
                display_url: ref __self_0_6,
                expanded_url: ref __self_0_7,
                media_type: ref __self_0_8,
                sizes: ref __self_0_9,
                source_status_id: ref __self_0_10,
                source_status_id_str: ref __self_0_11,
            } => __arg_0.emit_struct("Media", 12usize, |_e| -> _ {
                match _e.emit_struct_field("id", 0usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_0), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("id_str", 1usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_1), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("indices", 2usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_2), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("media_url", 3usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_3), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("media_url_https", 4usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_4), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("url", 5usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_5), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("display_url", 6usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_6), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("expanded_url", 7usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_7), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("type", 8usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_8), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("sizes", 9usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_9), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                match _e.emit_struct_field("source_status_id", 10usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_10), _e)
                }) {
                    ::std::result::Result::Ok(__try_var) => __try_var,
                    ::std::result::Result::Err(__try_var) => {
                        return ::std::result::Result::Err(__try_var);
                    }
                };
                return _e.emit_struct_field("source_status_id_str", 11usize, |_e| -> _ {
                    ::rustc_serialize::Encodable::encode(&(*__self_0_11), _e)
                });
            }),
        }
    }
}
