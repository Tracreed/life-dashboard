use axum::{extract::State, http::StatusCode, Json};
use chrono::NaiveDate;
use leptos::*;
use sqlx::PgPool;
use shuttle_runtime::SecretStore;

pub struct AppState {
    pub pool: PgPool,
    pub secrets: SecretStore,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct VacationRequest {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

pub async fn submit_vacation_request(
    State(state): State<AppState>,
    Json(request): Json<VacationRequest>,
) -> Result<Json<VacationRequest>, (StatusCode, String)> {
    // Validate date range
    if request.end_date < request.start_date {
        return Err((
            StatusCode::BAD_REQUEST, 
            "End date must be after start date".to_string()
        ));
    }

    // Insert vacation request
    let result = sqlx::query!(
        r#"
        INSERT INTO vacation_requests (start_date, end_date, status)
        VALUES ($1, $2, 'PENDING')
        RETURNING id
        "#,
        request.start_date,
        request.end_date
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Send email (we'll implement this in email.rs)
    crate::email::send_vacation_email(
        &state.secrets, 
        &request.start_date, 
        &request.end_date
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(request))
}