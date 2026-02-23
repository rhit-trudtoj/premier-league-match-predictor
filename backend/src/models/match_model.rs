use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchStatus {
    Scheduled,
    Live,
    Finished,
    Postponed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchResult {
    HomeWin,
    Draw,
    AwayWin,
}

impl MatchResult {
    pub fn to_class_label(&self) -> i32 {
        match self {
            MatchResult::Draw => 0,
            MatchResult::HomeWin => 1,
            MatchResult::AwayWin => 2,
        }
    }

    pub fn from_class_label(label: i32) -> Option<Self> {
        match label {
            0 => Some(MatchResult::Draw),
            1 => Some(MatchResult::HomeWin),
            2 => Some(MatchResult::AwayWin),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Match {
    pub id: Uuid,
    pub home_team_id: Uuid,
    pub away_team_id: Uuid,
    pub match_date: chrono::DateTime<chrono::Utc>,
    pub status: String, // Will be converted to/from MatchStatus
    pub season: String, // e.g., "2023-24"
    pub gameweek: i32,

    // Score (null if not finished)
    pub home_score: Option<i32>,
    pub away_score: Option<i32>,

    // Match statistics (populated during/after match)
    pub home_xg: Option<f64>,
    pub away_xg: Option<f64>,
    pub home_possession: Option<f64>,
    pub away_possession: Option<f64>,
    pub home_shots: Option<i32>,
    pub away_shots: Option<i32>,
    pub home_shots_on_target: Option<i32>,
    pub away_shots_on_target: Option<i32>,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchWithTeams {
    #[serde(flatten)]
    pub match_info: Match,
    pub home_team_name: String,
    pub away_team_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingMatchesResponse {
    pub matches: Vec<MatchWithPrediction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchWithPrediction {
    pub match_id: Uuid,
    pub home_team: String,
    pub away_team: String,
    pub match_date: chrono::DateTime<chrono::Utc>,
    pub prediction: Option<PredictionProbabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionProbabilities {
    pub home_win: f64,
    pub draw: f64,
    pub away_win: f64,
    pub predicted_result: MatchResult,
    pub confidence: f64,
}
