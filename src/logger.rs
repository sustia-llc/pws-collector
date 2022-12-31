use crate::settings::get_settings;
use std::env;
// requires tracing 0.2, see Cargo.toml
use tracing_subscriber::{
    fmt::{self, writer::MakeWriterExt},
    subscribe::CollectExt,
    EnvFilter,
};

pub fn setup() {
    let settings = get_settings();
    if env::var_os("RUST_LOG").is_none() {
        let level = settings.logger.level.as_str();
        let env = format!("pws_collector={},mongodb={}", level, level);
        env::set_var("RUST_LOG", env);
    }
    // Log warnings and errors to a file.
    let warn_file = tracing_appender::rolling::daily(
        settings.logger.log_path.as_str(),
        settings.logger.log_file_prefix.as_str(),
    )
    .with_max_level(tracing::Level::WARN);

    let collector = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Subscriber::new().with_writer(std::io::stdout))
        .with(fmt::Subscriber::new().with_writer(warn_file));
    tracing::collect::set_global_default(collector).expect("Unable to set a global collector");

    if settings.environment == "test" {
        tracing::info!("info level logging enabled");
        tracing::debug!("debug level logging enabled");
        tracing::trace!("trace level logging enabled");
        tracing::warn!("warn level logging enabled");
        tracing::error!("error level logging enabled");
    }
}
