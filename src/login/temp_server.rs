use std::{env, path::Path};

use super::get_url;
use crate::success;
use axum::{self, extract::State, response::IntoResponse, routing::post, Json, Router};
use serde_json::Value;
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;

pub async fn service() -> anyhow::Result<()> {
    let control = CancellationToken::new();

    let app = Router::new()
        .route("/", post(get_token))
        .layer(CorsLayer::permissive())
        .with_state(control.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7743").await.unwrap();
    get_url().await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shuttle_signal(control))
        .await
        .unwrap();
    Ok(())
}

async fn get_token(State(arc): State<CancellationToken>, value: Json<Value>) -> impl IntoResponse {
    let dir = env!("HOME");
    let file_path = Path::new(dir).join(".cargo-actions/token.json");
    serde_json::to_writer_pretty(std::fs::File::create(file_path).unwrap(), &value.0).unwrap();
    arc.cancel();
    "ok"
}

async fn shuttle_signal(s: CancellationToken) {
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = s.cancelled() => {
            success!("登陆成功");
        },
        _ = terminate => {

        },

    }
}
