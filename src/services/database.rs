use anyhow::{Result, Context};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::OnceLock;
use crate::models::vacation::VacationRequest;

// Global connection pool
static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn initialize_pool() -> Result<&'static PgPool> {
    let pool = DB_POOL.get_or_init(|| async {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
            
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create database pool")
    }).await;
    
    Ok(pool)
}

pub struct VacationRequestService {
    pool: &'static PgPool,
}

impl VacationRequestService {
    pub fn new() -> Self {
        let pool = DB_POOL.get()
            .expect("Database pool not initialized");
        
        Self { pool }
    }
    
    pub async fn create_request(&self, start_date: &str, end_date: &str) -> Result<i32> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO vacation_requests (start_date, end_date, status)
            VALUES ($1, $2, 'PENDING')
            RETURNING id
            "#,
            start_date.parse::<chrono::NaiveDate>().context("Invalid start date format")?,
            end_date.parse::<chrono::NaiveDate>().context("Invalid end date format")?
        )
        .fetch_one(self.pool)
        .await
        .context("Failed to insert vacation request")?;
        
        Ok(id)
    }
    
    pub async fn get_request(&self, id: i32) -> Result<VacationRequest> {
        let request = sqlx::query_as!(
            VacationRequest,
            r#"
            SELECT id, start_date, end_date, status,
                   created_at as "created_at: chrono::DateTime<chrono::Utc>",
                   updated_at as "updated_at: chrono::DateTime<chrono::Utc>"
            FROM vacation_requests
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.pool)
        .await
        .context("Failed to fetch vacation request")?;
        
        Ok(request)
    }
    
    pub async fn list_requests(&self) -> Result<Vec<VacationRequest>> {
        let requests = sqlx::query_as!(
            VacationRequest,
            r#"
            SELECT id, start_date, end_date, status,
                   created_at as "created_at: chrono::DateTime<chrono::Utc>",
                   updated_at as "updated_at: chrono::DateTime<chrono::Utc>"
            FROM vacation_requests
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(self.pool)
        .await
        .context("Failed to fetch vacation requests")?;
        
        Ok(requests)
    }
    
    pub async fn update_status(&self, id: i32, status: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE vacation_requests
            SET status = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            "#,
            status,
            id
        )
        .execute(self.pool)
        .await
        .context("Failed to update vacation request status")?;
        
        Ok(())
    }
    
    pub async fn delete_request(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM vacation_requests
            WHERE id = $1
            "#,
            id
        )
        .execute(self.pool)
        .await
        .context("Failed to delete vacation request")?;
        
        Ok(())
    }
}