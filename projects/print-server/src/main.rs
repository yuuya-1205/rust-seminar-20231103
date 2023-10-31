//! Run with
//!
//! ```not_rust
//! cargo run -p example-print-request-response
//! ```
//!
//!
use axum::{
    body::{Body, Bytes},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use http::Request;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    setup_tracing();

    let app = Router::new()
        .route("/", post(|| async move { "Hello from `POST /`" }))
        .layer(middleware::from_fn(print_request_response));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tracing::instrument(skip_all)]
async fn print_request_response<B>(
    req: Request<B>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let (parts, body) = req.into_parts();

    let bytes = buffer_and_print("request", body).await?;
    tracing::info!("headers: {:?}", &parts.headers);

    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::info!("{direction} body = {body:?}");
    }

    Ok(bytes)
}

#[cfg(feature = "tracer")]
fn setup_tracing() {
    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("axum-tracing-test")
        .with_endpoint("jaeger:6831")
        .install_simple()
        .unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let format = tracing_subscriber::fmt::format().with_target(false);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(telemetry)
        .with(tracing_subscriber::fmt::layer().event_format(format))
        .init();
}

#[cfg(not(feature = "tracer"))]
fn setup_tracing() {
    let format = tracing_subscriber::fmt::format().with_target(false);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().event_format(format))
        .init();
}
