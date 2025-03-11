// src/models/vacation.rs
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VacationRequest {
    pub id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VacationRequest {
    pub fn duration_days(&self) -> i64 {
        (self.end_date - self.start_date).num_days() + 1 // Include both start and end days
    }
    
    pub fn is_pending(&self) -> bool {
        self.status == "PENDING"
    }
    
    pub fn is_approved(&self) -> bool {
        self.status == "APPROVED"
    }
    
    pub fn is_rejected(&self) -> bool {
        self.status == "REJECTED"
    }
    
    pub fn formatted_start_date(&self) -> String {
        self.start_date.format("%Y-%m-%d").to_string()
    }
    
    pub fn formatted_end_date(&self) -> String {
        self.end_date.format("%Y-%m-%d").to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VacationRequestForm {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VacationStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

impl From<&str> for VacationStatus {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "APPROVED" => VacationStatus::Approved,
            "REJECTED" => VacationStatus::Rejected,
            "CANCELLED" => VacationStatus::Cancelled,
            _ => VacationStatus::Pending,
        }
    }
}

impl ToString for VacationStatus {
    fn to_string(&self) -> String {
        match self {
            VacationStatus::Pending => "PENDING".to_string(),
            VacationStatus::Approved => "APPROVED".to_string(),
            VacationStatus::Rejected => "REJECTED".to_string(),
            VacationStatus::Cancelled => "CANCELLED".to_string(),
        }
    }
}