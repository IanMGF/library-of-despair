use std::path::PathBuf;

use log4rs::{
    Config,
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

pub fn setup_logging() {
    const LOG_LEVEL: log::Level = log::Level::Info;
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h([{d(%H:%M:%S)} - {l}])} {m} {n}",
        )))
        .build();

    let now_str = chrono::Local::now()
        .naive_local()
        .format("%Y_%m_%d-%H_%M_%S");
    let log_path = PathBuf::from(now_str.to_string());
    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h([{d(%H:%M:%S)} - {l}])} {m} {n}",
        )))
        .build(log_path);

    let log_out = match log_file {
        Ok(log_out) => log_out,
        Err(err) => panic!("Erro ao iniciar arquivo de log: {err}"),
    };

    let log_config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(log_out)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(LOG_LEVEL.to_level_filter()),
        )
        .unwrap();

    let _ = log4rs::init_config(log_config);
}
