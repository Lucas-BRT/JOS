use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_logging() {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_target(true)
        .init();
}
