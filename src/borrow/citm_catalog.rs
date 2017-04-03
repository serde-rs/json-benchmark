use std::collections::BTreeMap as Map;

use empty;
use prim_str::PrimStr;

use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CitmCatalog<'a> {
    #[serde(rename = "areaNames", borrow)]
    pub area_names: Map<IdStr, &'a str>,
    #[serde(rename = "audienceSubCategoryNames", borrow)]
    pub audience_sub_category_names: Map<IdStr, &'a str>,
    #[serde(rename = "blockNames", borrow)]
    pub block_names: Map<IdStr, &'a str>,
    #[serde(borrow)]
    pub events: Map<IdStr, Event<'a>>,
    #[serde(borrow)]
    pub performances: Vec<Performance<'a>>,
    #[serde(rename = "seatCategoryNames", borrow)]
    pub seat_category_names: Map<IdStr, &'a str>,
    #[serde(rename = "subTopicNames", borrow)]
    pub sub_topic_names: Map<IdStr, &'a str>,
    #[serde(rename = "subjectNames", borrow)]
    pub subject_names: Map<IdStr, &'a str>,
    #[serde(rename = "topicNames", borrow)]
    pub topic_names: Map<IdStr, &'a str>,
    #[serde(rename = "topicSubTopics")]
    pub topic_sub_topics: Map<IdStr, Vec<Id>>,
    #[serde(rename = "venueNames", borrow)]
    pub venue_names: Map<&'a str, &'a str>,
}

pub type Id = u32;
pub type IdStr = PrimStr<u32>;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event<'a> {
    pub description: (),
    pub id: Id,
    #[serde(borrow)]
    pub logo: Option<&'a str>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(rename = "subTopicIds")]
    pub sub_topic_ids: Vec<Id>,
    #[serde(rename = "subjectCode")]
    pub subject_code: (),
    pub subtitle: (),
    #[serde(rename = "topicIds")]
    pub topic_ids: Vec<Id>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Performance<'a> {
    #[serde(rename = "eventId")]
    pub event_id: Id,
    pub id: Id,
    #[serde(borrow)]
    pub logo: Option<&'a str>,
    pub name: (),
    pub prices: Vec<Price>,
    #[serde(rename = "seatCategories")]
    pub seat_categories: Vec<SeatCategory>,
    #[serde(rename = "seatMapImage")]
    pub seat_map_image: (),
    pub start: u64,
    #[serde(rename = "venueCode", borrow)]
    pub venue_code: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Price {
    pub amount: u32,
    #[serde(rename = "audienceSubCategoryId")]
    pub audience_sub_category_id: Id,
    #[serde(rename = "seatCategoryId")]
    pub seat_category_id: Id,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeatCategory {
    pub areas: Vec<Area>,
    #[serde(rename = "seatCategoryId")]
    pub seat_category_id: Id,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Area {
    #[serde(rename = "areaId")]
    pub area_id: Id,
    #[serde(rename = "blockIds")]
    pub block_ids: empty::Array,
}
