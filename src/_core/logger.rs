use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::_core::data_dir;

pub fn init_tracing() -> WorkerGuard {
    let log_dir = data_dir().join("logs");

    let file_appender = tracing_appender::rolling::daily(log_dir, "app.log");

    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let std_layer = fmt::layer().with_ansi(true).with_target(true);

    let file_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false)
        .with_target(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(std_layer)
        .with(file_layer)
        .init();

    guard
}
