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
#[cfg_attr(
    feature = "lib-rustc-serialize",
    derive(RustcEncodable, RustcDecodable)
)]
pub struct Log {
    jk_host: String,
    class_name: String,
    logger_name: String,
    cgi_tte_ms: String, //"3173.81" kin of is a float
    start_timestamp: u64,
    user_agent_device: String,
    slush: String,
    and_an_ip4: String,
    version: String, // Could be u64? "1"
    error_url_path: String,
    logstash: String,
    uuids: String,
    anotherfilename: String,
    environment: Environment,
    floatasstr: String, // Could be float "123.56",
    there_string: String,
    arry: Vec<String>,
    message: String,
    argh: String,
    oh_my_files: String,
    user_agent_os: String,
    error_host: String,
    application: String,
    yam_message: String,
    user_agent_browser: String,
    error_url: String,
    short_message: String,
    action: String,
    cakes: String,
    log_type: String,
    log_level: LogLevel,
    too_many_ho: String,
    controller: String,
    key_keykeykey: String,
    proper_timestamp: String, //Coudl be date? "2018-07-23T12:19:16-04:00",
    and_yet_another: String,
    timestamp: String, //Could be date3 "2018-07-23T16:19:16.821Z",
    level: u8,
}
