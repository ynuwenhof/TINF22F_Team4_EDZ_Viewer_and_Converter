mod analyzer;
mod context;
mod db;
mod edz;
mod error;
mod model;
mod queue;
mod trace_layer;

use self::analyzer::Analyzer;
use self::context::Context;
use self::db::Database;
use self::trace_layer::{MakeSpan, OnFailure};
use axum::extract::{DefaultBodyLimit, Request};
use axum::response::Html;
use axum::{routing, Router, ServiceExt};
use clap::Parser;
use mongodb::Client;
use std::net::IpAddr;
use std::path::PathBuf;
use std::process::ExitCode;
use tokio::net::TcpListener;
use tokio::runtime;
use tower_http::compression::CompressionLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::trace::TraceLayer;
use tower_layer::Layer;
use tracing::error;
use url::Url;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, env = "RPLAN_ADDRESS", default_value_t = IpAddr::from([127, 0, 0, 1]))]
    addr: IpAddr,
    #[arg(short, long, env = "RPLAN_PORT", default_value_t = 80)]
    port: u16,
    #[arg(short, long, env = "RPLAN_BODY_LIMIT", default_value_t = 1024 * 1024 * 1024)]
    body_limit: usize,
    #[arg(long, env = "RPLAN_ANALYZER_TASKS", value_parser = clap::value_parser!(u8).range(1..), default_value_t = 3)]
    analyzer_tasks: u8,
    #[arg(short, long, env = "RPLAN_URL")]
    url: Url,
    #[arg(short, long, env = "RPLAN_MONGODB_URI", default_value_t = String::from("mongodb://localhost:27017"))]
    mongodb_uri: String,
    #[arg(short, long, env = "RPLAN_DATA_PATH", value_name = "DIRECTORY")]
    data_path: PathBuf,
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false))
        .with(EnvFilter::from_default_env())
        .with(ErrorLayer::default())
        .init();
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    install_tracing();

    if let Err(e) = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed building the runtime")
        .block_on(run(cli))
    {
        error!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

async fn run(cli: Cli) -> error::Result<()> {
    let body_limit = DefaultBodyLimit::max(cli.body_limit);
    let layer = TraceLayer::new_for_http()
        .make_span_with(MakeSpan)
        .on_failure(OnFailure);

    let client = Client::with_uri_str(&cli.mongodb_uri).await?;
    let db = Database::new(client).await?;
    let ctx = Context::new(cli, db);

    for i in 0..ctx.cli.analyzer_tasks {
        Analyzer::new(ctx.clone(), i);
    }

    let listener = TcpListener::bind((ctx.cli.addr, ctx.cli.port)).await?;

    #[rustfmt::skip]
        let app = Router::new()
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CompressionLayer::new())
        .layer(body_limit)
        .layer(layer)
        .with_state(ctx);

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);
    let app = ServiceExt::<Request>::into_make_service(app);

    axum::serve(listener, app).await?;

    Ok(())
}

