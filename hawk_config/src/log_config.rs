use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn logger_main(level: LevelFilter, log_path: &str) {
    let stdout = ConsoleAppender::builder().build();

    if let Ok(requests) = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build(log_path)
    {
        if let Ok(config) = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("requests", Box::new(requests)))
            .logger(Logger::builder().build("app::backend::db", level))
            .logger(
                Logger::builder()
                    .appender("requests")
                    .additive(false)
                    .build("app::requests", level),
            )
            .build(
                Root::builder()
                    .appender("stdout")
                    .appender("requests")
                    .build(level),
            )
        {
            let handle = log4rs::init_config(config);
            if let Err(e) = handle {
                println!("init log error : {:?}", e);
            }
        };
    };

    // use handle to change logger configuration at runtime
}
