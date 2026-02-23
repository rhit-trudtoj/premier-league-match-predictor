use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub short_name: Option<String>,
    pub logo_url: Option<String>,

    // Current season statistics
    pub matches_played: i32,
    pub wins: i32,
    pub draws: i32,
    pub losses: i32,
    pub goals_for: i32,
    pub goals_against: i32,
    pub points: i32,

    // Advanced metrics (nullable for historical teams)
    pub avg_xg: Option<f64>,
    pub avg_xg_against: Option<f64>,
    pub avg_possession: Option<f64>,
    pub avg_shots: Option<f64>,
    pub avg_shots_on_target: Option<f64>,

    // Metadata
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub short_name: Option<String>,
    pub logo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamStats {
    pub team_id: Uuid,
    pub team_name: String,
    pub form: Vec<String>, // Last 5 results: ["W", "L", "D", "W", "W"]
    pub home_record: Record,
    pub away_record: Record,
    pub recent_xg_avg: f64,
    pub recent_goals_avg: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub played: i32,
    pub won: i32,
    pub drawn: i32,
    pub lost: i32,
}
