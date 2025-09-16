use log::info;
use multi_log;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::logs::{BatchLogProcessor, SdkLoggerProvider};
use opentelemetry_sdk::Resource;
use std::env;

const LOGGING: &str = "logging";

pub async fn setup_logging() -> anyhow::Result<()> {
    // Read service name from environment variable (fall back to "neutron-strategist")
    let service_name =
        env::var("SERVICE_NAME").unwrap_or_else(|_| "neutron-strategist".to_string());
    let service_name: &'static str = Box::leak(service_name.into_boxed_str());

    match env::var("OTLP_ENDPOINT") {
        Ok(otlp_endpoint) => {
            let otlp_exporter = opentelemetry_otlp::LogExporter::builder()
                .with_http()
                .with_protocol(Protocol::HttpBinary)
                .with_endpoint(otlp_endpoint)
                .build()?;

            let otlp_logger_provider = SdkLoggerProvider::builder()
                .with_resource(Resource::builder().with_service_name(service_name).build())
                .with_log_processor(BatchLogProcessor::builder(otlp_exporter).build())
                .build();

            let otlp_logger = Box::new(OpenTelemetryLogBridge::new(&otlp_logger_provider));
            let std_logger = Box::new(env_logger::Builder::from_default_env().build());

            multi_log::MultiLogger::init(vec![otlp_logger, std_logger], log::Level::Trace)?;
        }
        Err(_) => {
            env_logger::init();
            info!(target: LOGGING, "OTLP_ENDPOINT not set, skipping OpenTelemetry logging");
        }
    };

    Ok(())
}
