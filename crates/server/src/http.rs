use std::time::Duration;

use api::{EnsSchema, build_schema};
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use config::AppConfig;
use storage::Storage;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

#[derive(Clone)]
pub struct ServerState {
    schema: EnsSchema,
    storage: Storage,
    playground: bool,
}

pub async fn serve(config: AppConfig, storage: Storage) -> anyhow::Result<()> {
    let bind_address = config.bind_address;
    let app = build_router(config, storage);
    let listener = tokio::net::TcpListener::bind(bind_address).await?;

    tracing::info!(%bind_address, "starting ENS indexer HTTP server");
    axum::serve(listener, app).await?;
    Ok(())
}

pub fn build_router(config: AppConfig, storage: Storage) -> Router {
    let schema = build_schema(storage.clone());
    let state = ServerState {
        schema,
        storage,
        playground: config.graphql_playground,
    };

    Router::new()
        .route("/graphql", post(graphql_handler).get(graphql_playground))
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .with_state(state)
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(TraceLayer::new_for_http())
}

async fn graphql_handler(
    State(state): State<ServerState>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(request.into_inner()).await.into()
}

async fn graphql_playground(State(state): State<ServerState>) -> impl IntoResponse {
    if state.playground {
        Html(GraphiQLSource::build().endpoint("/graphql").finish()).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

async fn healthz() -> &'static str {
    "ok"
}

async fn readyz(State(state): State<ServerState>) -> impl IntoResponse {
    match sqlx::query("select 1").execute(state.storage.pool()).await {
        Ok(_) => axum::http::StatusCode::NO_CONTENT,
        Err(_) => axum::http::StatusCode::SERVICE_UNAVAILABLE,
    }
}
