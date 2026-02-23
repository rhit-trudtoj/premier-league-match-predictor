use ndarray::{Array1, Array2};
use ort::{Session, SessionOutputs};
use std::sync::Arc;

pub struct Model {
    session: Session,
    feature_names: Vec<String>,
}

impl Model {
    pub fn from_file(model_path: &str) -> anyhow::Result<Self> {
        tracing::info!("Loading ONNX model from: {}", model_path);

        let session = Session::builder()?
            .with_model_from_file(model_path)?;

        // TODO: Load feature names from a config file or model metadata
        let feature_names = vec![
            "home_avg_xg".to_string(),
            "away_avg_xg".to_string(),
            "xg_differential".to_string(),
            "home_possession".to_string(),
            "away_possession".to_string(),
            "possession_differential".to_string(),
            "home_shots_on_target".to_string(),
            "away_shots_on_target".to_string(),
            "home_goals_for".to_string(),
            "away_goals_for".to_string(),
            "home_goals_against".to_string(),
            "away_goals_against".to_string(),
            "home_form_points".to_string(),
            "away_form_points".to_string(),
            "form_differential".to_string(),
            "head_to_head_ratio".to_string(),
        ];

        Ok(Self {
            session,
            feature_names,
        })
    }

    pub fn predict(&self, features: &[f64]) -> anyhow::Result<[f64; 3]> {
        // Convert features to ndarray
        let input_array = Array2::from_shape_vec((1, features.len()), features.to_vec())?;

        // Run inference
        let outputs: SessionOutputs = self.session.run(ort::inputs!["input" => input_array.view()]?)?;

        // Extract probabilities
        let output_tensor = outputs["output"].try_extract_tensor::<f32>()?;
        let probs = output_tensor.view().to_slice().unwrap();

        // Convert to f64 and return as array
        Ok([
            probs[0] as f64,
            probs[1] as f64,
            probs[2] as f64,
        ])
    }

    pub fn feature_names(&self) -> &[String] {
        &self.feature_names
    }
}

pub fn load_model(model_path: &str) -> anyhow::Result<Arc<Model>> {
    let model = Model::from_file(model_path)?;
    Ok(Arc::new(model))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_prediction_format() {
        // This test will fail until you have an actual ONNX model
        // It's here as a template for when the model is ready
        // let model = Model::from_file("./models/predictor.onnx").unwrap();
        // let features = vec![0.5; 16]; // Mock features
        // let probs = model.predict(&features).unwrap();
        // assert_eq!(probs.len(), 3);
        // assert!((probs.iter().sum::<f64>() - 1.0).abs() < 0.01);
    }
}
