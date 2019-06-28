#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

enum_str!(Environment {
    Production("Production"),
    Staging("Staging"),
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
    class_name: String,
    logger_name: &'a str,
    cgi_tte_ms: &'a str,
    start_timestamp: u64,
    user_agent_device: &'a str,
    slush: String,
    and_an_ip4: &'a str,
    #[cfg_attr(feature = "lib-serde", serde(rename = "@version"))]
    version: &'a str,
    error_url_path: &'a str,
    logstash: &'a str,
    #[cfg_attr(feature = "lib-serde", serde(rename = "uuids->"))]
    uuid: &'a str,
    anotherfilename: &'a str,
    environment: Environment,
    floatasstr: &'a str,
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
    proper_timestamp: &'a str,
    and_yet_another: &'a str,
    #[cfg_attr(feature = "lib-serde", serde(rename = "@timestamp"))]
    timestamp: &'a str,
    level: u8,
}
