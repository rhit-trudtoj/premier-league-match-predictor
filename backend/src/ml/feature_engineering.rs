use crate::models::{Match, Team};

/// Calculates features for a match prediction
pub fn calculate_match_features(
    home_team: &Team,
    away_team: &Team,
    home_form: &[String],
    away_form: &[String],
) -> Vec<f64> {
    let mut features = Vec::new();

    // xG metrics
    features.push(home_team.avg_xg.unwrap_or(1.0));
    features.push(away_team.avg_xg.unwrap_or(1.0));
    features.push(
        home_team.avg_xg.unwrap_or(1.0) - away_team.avg_xg.unwrap_or(1.0),
    );

    // Possession metrics
    features.push(home_team.avg_possession.unwrap_or(50.0));
    features.push(away_team.avg_possession.unwrap_or(50.0));
    features.push(
        home_team.avg_possession.unwrap_or(50.0) - away_team.avg_possession.unwrap_or(50.0),
    );

    // Shots on target
    features.push(home_team.avg_shots_on_target.unwrap_or(4.0));
    features.push(away_team.avg_shots_on_target.unwrap_or(4.0));

    // Goals metrics
    features.push(home_team.goals_for as f64 / home_team.matches_played.max(1) as f64);
    features.push(away_team.goals_for as f64 / away_team.matches_played.max(1) as f64);
    features.push(home_team.goals_against as f64 / home_team.matches_played.max(1) as f64);
    features.push(away_team.goals_against as f64 / away_team.matches_played.max(1) as f64);

    // Form (points in last 5 matches)
    let home_form_points = calculate_form_points(home_form);
    let away_form_points = calculate_form_points(away_form);
    features.push(home_form_points);
    features.push(away_form_points);
    features.push(home_form_points - away_form_points);

    // Head-to-head ratio (placeholder - would need historical data)
    features.push(0.5); // TODO: Calculate from database

    features
}

/// Converts form string to points
/// "W" = 3, "D" = 1, "L" = 0
fn calculate_form_points(form: &[String]) -> f64 {
    form.iter()
        .map(|result| match result.as_str() {
            "W" => 3.0,
            "D" => 1.0,
            _ => 0.0,
        })
        .sum::<f64>()
}

/// Normalizes features to 0-1 range
pub fn normalize_features(features: &[f64]) -> Vec<f64> {
    // Simple min-max normalization
    // In production, use saved scaler parameters from training
    features
        .iter()
        .map(|&f| {
            // Placeholder: actual normalization should use training set statistics
            f / 100.0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_calculation() {
        let form = vec!["W".to_string(), "W".to_string(), "D".to_string(), "L".to_string(), "W".to_string()];
        let points = calculate_form_points(&form);
        assert_eq!(points, 10.0); // 3+3+1+0+3 = 10
    }

    #[test]
    fn test_feature_count() {
        use uuid::Uuid;
        use chrono::Utc;

        let home_team = Team {
            id: Uuid::new_v4(),
            name: "Arsenal".to_string(),
            short_name: Some("ARS".to_string()),
            logo_url: None,
            matches_played: 10,
            wins: 7,
            draws: 2,
            losses: 1,
            goals_for: 25,
            goals_against: 10,
            points: 23,
            avg_xg: Some(2.1),
            avg_xg_against: Some(1.0),
            avg_possession: Some(58.0),
            avg_shots: Some(15.0),
            avg_shots_on_target: Some(6.0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let away_team = Team {
            id: Uuid::new_v4(),
            name: "Chelsea".to_string(),
            short_name: Some("CHE".to_string()),
            logo_url: None,
            matches_played: 10,
            wins: 5,
            draws: 3,
            losses: 2,
            goals_for: 18,
            goals_against: 12,
            points: 18,
            avg_xg: Some(1.7),
            avg_xg_against: Some(1.2),
            avg_possession: Some(52.0),
            avg_shots: Some(12.0),
            avg_shots_on_target: Some(5.0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let home_form = vec!["W".to_string(); 5];
        let away_form = vec!["D".to_string(); 5];

        let features = calculate_match_features(&home_team, &away_team, &home_form, &away_form);
        assert_eq!(features.len(), 16); // Should match model's expected feature count
    }
}
