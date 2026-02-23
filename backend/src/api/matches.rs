use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    models::{Match, MatchWithTeams, UpcomingMatchesResponse},
    AppState,
};

/// GET /api/v1/matches
/// Returns all matches
pub async fn get_matches(
    State(state): State<AppState>,
) -> Result<Json<Vec<Match>>, StatusCode> {
    // TODO: Implement database query
    // For now, return empty array
    tracing::info!("Fetching all matches");

    // Example query:
    // let matches = sqlx::query_as::<_, Match>("SELECT * FROM matches ORDER BY match_date DESC LIMIT 100")
    //     .fetch_all(&state.db_pool)
    //     .await
    //     .map_err(|e| {
    //         tracing::error!("Database error: {}", e);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?;

    Ok(Json(vec![]))
}

/// GET /api/v1/matches/upcoming
/// Returns upcoming matches with predictions
pub async fn get_upcoming_matches(
    State(state): State<AppState>,
) -> Result<Json<UpcomingMatchesResponse>, StatusCode> {
    tracing::info!("Fetching upcoming matches");

    // TODO: Query upcoming matches from database
    // TODO: Generate predictions for each match
    // TODO: Join with team names

    // Example implementation:
    // let matches = sqlx::query_as::<_, Match>(
    //     "SELECT * FROM matches WHERE match_date > NOW() AND status = 'scheduled' ORDER BY match_date LIMIT 20"
    // )
    // .fetch_all(&state.db_pool)
    // .await
    // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UpcomingMatchesResponse { matches: vec![] }))
}

/// GET /api/v1/matches/:id
/// Returns a specific match with team details
pub async fn get_match_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MatchWithTeams>, StatusCode> {
    tracing::info!("Fetching match: {}", id);

    // TODO: Implement database query with JOIN to get team names
    // Example:
    // let match_data = sqlx::query_as::<_, MatchWithTeams>(
    //     r#"
    //     SELECT m.*, ht.name as home_team_name, at.name as away_team_name
    //     FROM matches m
    //     JOIN teams ht ON m.home_team_id = ht.id
    //     JOIN teams at ON m.away_team_id = at.id
    //     WHERE m.id = $1
    //     "#
    // )
    // .bind(id)
    // .fetch_one(&state.db_pool)
    // .await
    // .map_err(|_| StatusCode::NOT_FOUND)?;

    Err(StatusCode::NOT_IMPLEMENTED)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // Tests will be added once database is set up
        assert!(true);
    }
}
