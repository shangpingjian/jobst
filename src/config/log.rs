use env_logger;
use log;
use log::{debug, error, info, trace, warn};
use serde_json;
use simple_log::{LogConfig};

pub fn logger_init(){
    let config = r#"
    {
        "path":"./logs/tmp.log",
        "level":"error",
        "size":10,
        "out_kind":["console","file"],
        "roll_count":10,
        "time_format":"%H:%M:%S.%f"
    }"#;
    let log_config: LogConfig = serde_json::from_str(config).unwrap();

    simple_log::new(log_config).unwrap();//init log

}
