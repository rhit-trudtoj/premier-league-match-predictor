use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub model_path: String,
    pub jwt_secret: String,
    pub football_api_key: String,
    pub api_rate_limit: u32,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let config = Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/premier_league".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            model_path: std::env::var("MODEL_PATH")
                .unwrap_or_else(|_| "./models/predictor.onnx".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            football_api_key: std::env::var("FOOTBALL_API_KEY")
                .unwrap_or_else(|_| String::new()),
            api_rate_limit: std::env::var("API_RATE_LIMIT")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
        };

        Ok(config)
    }
}
