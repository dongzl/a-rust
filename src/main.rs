use std::env;
use std::error::Error;

mod analyzer;
mod backend;
mod config;
mod frontend;
mod monitor;
mod mysql;
mod proxy;
mod router;

lazy_static::lazy_static! {
        //1 init global config
        static ref  GLOBAL_CONFIG: config::Config = {
            config::load_config().unwrap()
        };

        static ref SHOTCUT_GLOBAL_CONFIG:config::ConfigShortcut = {
            config::build_config_shortcut().unwrap()
        };
}

include!(concat!(env!("OUT_DIR"), "/commit_id.rs"));
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logger();
    log::info!(
        "Start arana-rust MySQL proxy, run commit_id: {} compile_time: {}",
        COMMIT_ID,
        COMPILE_TIME
    );
    proxy::ProxyServer::new().run().await?;
    Ok(())
}

fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || /* get the task id here */ 0)
        .start(
            GLOBAL_CONFIG
                .query_log_level()
                .unwrap_or(log::LevelFilter::Trace),
        )
        .unwrap();
}
