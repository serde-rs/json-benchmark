#![cfg(test)]
#![feature(test)]
#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate test;

use test::Bencher;

////////////////////////////////////////////////////////////////////////////////

const SMALL: &str = include_str!("small.json");

#[derive(Serialize, Deserialize)]
struct SmallPayload {
    st: i64,
    sid: i64,
    tt: String,
    gr: i64,
    uuid: String,
    ip: String,
    ua: String,
    tz: i64,
    v: i64,
}

impl SmallPayload {
    fn new() -> Self {
        SmallPayload {
            st: 1,
            sid: 2,
            tt: "TestString".to_owned(),
            gr: 4,
            uuid: "8f9a65eb-4807-4d57-b6e0-bda5d62f1429".to_owned(),
            ip: "127.0.0.1".to_owned(),
            ua: "Mozilla".to_owned(),
            tz: 8,
            v: 6,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

const MEDIUM: &str = include_str!("medium.json");

#[derive(Serialize, Deserialize)]
struct MediumPayload {
    person: CBPerson,
    company: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CBPerson {
    name: CBName,
    github: CBGithub,
    gravatar: CBGravatar,
}

#[derive(Serialize, Deserialize)]
struct CBName {
    #[serde(rename = "fullName")]
    full_name: String,
}

#[derive(Serialize, Deserialize)]
struct CBGithub {
    followers: i64,
}

#[derive(Serialize, Deserialize)]
struct CBGravatar {
    avatars: Vec<CBAvatar>,
}

#[derive(Serialize, Deserialize)]
struct CBAvatar {
    url: String,
}

impl MediumPayload {
    fn new() -> Self {
        MediumPayload {
            company: Some("test".to_owned()),
            person: CBPerson {
                name: CBName {
                    full_name: "test".to_owned(),
                },
                github: CBGithub { followers: 100 },
                gravatar: CBGravatar {
                    avatars: vec![
                        CBAvatar {
                            url: "http://test.com".to_owned(),
                        },
                        CBAvatar {
                            url: "http://test.com".to_owned(),
                        },
                        CBAvatar {
                            url: "http://test.com".to_owned(),
                        },
                        CBAvatar {
                            url: "http://test.com".to_owned(),
                        },
                        CBAvatar {
                            url: "http://test.com".to_owned(),
                        },
                        CBAvatar {
                            url: "http://test.com".to_owned(),
                        },
                        CBAvatar {
                            url: "http://test.com".to_owned(),
                        },
                        CBAvatar {
                            url: "http://test.com".to_owned(),
                        },
                    ],
                },
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

const LARGE: &str = include_str!("large.json");

#[derive(Serialize, Deserialize)]
struct LargePayload {
    users: Vec<DSUser>,
    topics: DSTopicsList,
}

#[derive(Serialize, Deserialize)]
struct DSTopicsList {
    topics: Vec<DSTopic>,
    more_topics_url: String,
}

#[derive(Serialize, Deserialize)]
struct DSTopic {
    id: i64,
    slug: String,
}

#[derive(Serialize, Deserialize)]
struct DSUser {
    username: String,
}

impl LargePayload {
    fn new() -> Self {
        let mut ds_users = Vec::new();
        let mut ds_topics = Vec::new();
        for i in 0..100 {
            let s = format!("test{}", i);
            ds_users.push(DSUser {
                username: s.clone(),
            });
            ds_topics.push(DSTopic { id: i, slug: s });
        }
        LargePayload {
            users: ds_users,
            topics: DSTopicsList {
                topics: ds_topics,
                more_topics_url: "http://test.com".to_owned(),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[bench]
fn deserialize_small(b: &mut Bencher) {
    b.iter(|| serde_json::from_str::<SmallPayload>(SMALL).unwrap());
}

#[bench]
fn deserialize_medium(b: &mut Bencher) {
    b.iter(|| serde_json::from_str::<MediumPayload>(MEDIUM).unwrap());
}

#[bench]
fn deserialize_large(b: &mut Bencher) {
    b.iter(|| serde_json::from_str::<LargePayload>(LARGE).unwrap());
}

#[bench]
fn serialize_small(b: &mut Bencher) {
    let payload = SmallPayload::new();
    b.iter(|| serde_json::to_string(&payload).unwrap());
}

#[bench]
fn serialize_medium(b: &mut Bencher) {
    let payload = MediumPayload::new();
    b.iter(|| serde_json::to_string(&payload).unwrap());
}

#[bench]
fn serialize_large(b: &mut Bencher) {
    let payload = LargePayload::new();
    b.iter(|| serde_json::to_string(&payload).unwrap());
}
