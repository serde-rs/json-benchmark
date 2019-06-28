#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

enum_str!(Environment {
    Production("Production"),
    Staging("Stagig"),
    Development("Development"),
    Test("Test"),
});

enum_str!(LogLevel {
    Error("ERROR"),
    Warning("WARNING"),
    Info("INFO"),
    Debug("Debug"),
});

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Log<'a> {
    jk_host: &'a str,
    class_name: String, //&'a str,
    logger_name: &'a str,
    cgi_tte_ms: &'a str, //"3173.81" kin of is a float
    start_timestamp: u64,
    user_agent_device: &'a str,
    slush: String, //&'a str,
    and_an_ip4: &'a str,
    #[cfg_attr(feature = "lib-serde", serde(rename = "@version"))]
    version: &'a str, // Could be u64? "1"
    error_url_path: &'a str,
    logstash: &'a str,
    #[cfg_attr(feature = "lib-serde", serde(rename = "uuids->"))]
    uuid: &'a str,
    anotherfilename: &'a str,
    environment: Environment,
    floatasstr: &'a str, // Could be float "123.56",
    #[cfg_attr(feature = "lib-serde", serde(rename = "there_string:"))]
    there_string: &'a str,
    arry: Vec<&'a str>,
    message: &'a str,
    argh: &'a str,
    oh_my_files: &'a str,
    user_agent_os: &'a str,
    error_host: &'a str,
    application: &'a str,
    yam_message: &'a str,
    user_agent_browser: &'a str,
    error_url: &'a str,
    short_message: &'a str,
    action: &'a str,
    #[cfg_attr(feature = "lib-serde", serde(rename = "cakes!"))]
    cakes: &'a str,
    #[cfg_attr(feature = "lib-serde", serde(rename = "type"))]
    log_type: &'a str,
    log_level: LogLevel,
    too_many_ho: &'a str,
    controller: &'a str,
    key_keykeykey: &'a str,
    #[cfg_attr(feature = "lib-serde", serde(rename = "a proper_timestamp_ja"))]
    proper_timestamp: &'a str, //Coudl be date? "2018-07-23T12:19:16-04:00",
    and_yet_another: &'a str,
    #[cfg_attr(feature = "lib-serde", serde(rename = "@timestamp"))]
    timestamp: &'a str, //Could be date3 "2018-07-23T16:19:16.821Z",
    level: u8,
}
