use axum::Router;
use leptos::*;
use leptos_router::*;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

mod components;
mod email;
mod routes;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::SecretStore] secrets: SecretStore,
) -> shuttle_axum::Service {
    // Initialize routes and database
    let pool = pool.clone();
    
    let app = Router::new()
        .route("/vacation-request", axum::routing::post(routes::submit_vacation_request))
        .with_state(AppState { pool, secrets });

    Ok(app.into())
}