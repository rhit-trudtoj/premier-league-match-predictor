use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

use super::{matches, predictions, teams};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Match endpoints
        .route("/matches", get(matches::get_matches))
        .route("/matches/upcoming", get(matches::get_upcoming_matches))
        .route("/matches/:id", get(matches::get_match_by_id))

        // Team endpoints
        .route("/teams", get(teams::get_teams))
        .route("/teams/:id", get(teams::get_team_by_id))
        .route("/teams/:id/stats", get(teams::get_team_stats))

        // Prediction endpoints
        .route("/predictions/:match_id", get(predictions::get_prediction))
        .route("/predictions", post(predictions::create_prediction))
}
