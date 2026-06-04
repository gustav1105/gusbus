use axum::{Router, middleware, routing::get};
use std::net::SocketAddr;

use tower_http::trace::DefaultOnResponse;
use tower_http::{services::ServeDir, trace::TraceLayer};

use tracing::{Level, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tracing_opentelemetry::OpenTelemetryLayer;

use opentelemetry::trace::TracerProvider;
use opentelemetry::{KeyValue, global};

use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace as sdktrace;
use opentelemetry_sdk::{Resource, runtime};

use domain_router::handlers::{dump, location_page};
use domain_router::middleware::context_middleware;

use axum::http::Request;
use domain_router::context::Context;
use tower_http::trace::MakeSpan;
use tracing::Span;

#[derive(Clone)]
struct MakeSpanWithContext;

impl<B> MakeSpan<B> for MakeSpanWithContext {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let ctx = request.extensions().get::<Context>();

        let identity = ctx
            .and_then(|c| c.identity.username.clone())
            .unwrap_or_else(|| "anonymous".to_string());

        let ip = ctx
            .and_then(|c| c.network.client_ip)
            .map(|ip: std::net::IpAddr| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let country = ctx
            .and_then(|c| c.network.country.clone())
            .unwrap_or_else(|| "unknown".to_string());

        tracing::info_span!(
            "request",
            method = %request.method(),
            uri = %request.uri(),
            user = %identity,
            ip = %ip,
            country = %country,
        )
    }
}

fn init_tracer() -> opentelemetry_sdk::trace::Tracer {
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://otel-collector:4317");

    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "domain-router",
            )])),
        )
        .install_batch(runtime::Tokio)
        .expect("failed to init tracer provider");

    let tracer = provider.tracer("domain-router");

    global::set_tracer_provider(provider);

    tracer
}

#[tokio::main]
async fn main() {
    let tracer = init_tracer();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    info!("domain-router starting");

    let app = Router::new()
        .route("/", get(dump))
        .route("/location", get(location_page))
        .nest_service("/pkg", ServeDir::new("pkg"))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(MakeSpanWithContext)
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(middleware::from_fn(context_middleware));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
