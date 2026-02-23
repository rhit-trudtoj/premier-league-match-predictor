use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::match_model::MatchResult;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Prediction {
    pub id: Uuid,
    pub match_id: Uuid,
    pub model_version: String,

    // Prediction probabilities
    pub prob_home_win: f64,
    pub prob_draw: f64,
    pub prob_away_win: f64,

    // Predicted outcome
    pub predicted_result: i32, // 0=Draw, 1=Home, 2=Away
    pub confidence: f64,

    // Actual result (filled after match)
    pub actual_result: Option<i32>,
    pub was_correct: Option<bool>,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Prediction {
    pub fn new(
        match_id: Uuid,
        model_version: String,
        probabilities: [f64; 3],
    ) -> Self {
        let (predicted_result, confidence) = Self::calculate_prediction(&probabilities);

        Self {
            id: Uuid::new_v4(),
            match_id,
            model_version,
            prob_draw: probabilities[0],
            prob_home_win: probabilities[1],
            prob_away_win: probabilities[2],
            predicted_result,
            confidence,
            actual_result: None,
            was_correct: None,
            created_at: chrono::Utc::now(),
        }
    }

    fn calculate_prediction(probs: &[f64; 3]) -> (i32, f64) {
        let max_idx = probs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        let confidence = probs[max_idx];
        (max_idx as i32, confidence)
    }

    pub fn update_actual_result(&mut self, actual_result: MatchResult) {
        let actual = actual_result.to_class_label();
        self.actual_result = Some(actual);
        self.was_correct = Some(actual == self.predicted_result);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionRequest {
    pub match_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionResponse {
    pub prediction: Prediction,
    pub feature_importance: Option<Vec<FeatureImportance>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureImportance {
    pub feature_name: String,
    pub importance: f64,
}
