use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Client for external football data API
/// This is a template - you'll need to adapt it to your chosen API
pub struct FootballApiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl FootballApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.football-data.org/v4".to_string(),
        }
    }

    /// Fetch upcoming fixtures for Premier League
    pub async fn fetch_fixtures(&self, season: &str) -> anyhow::Result<Vec<ApiMatch>> {
        let url = format!("{}/competitions/PL/matches", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("X-Auth-Token", &self.api_key)
            .query(&[("season", season)])
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("API request failed: {}", response.status());
        }

        let data: FixturesResponse = response.json().await?;
        Ok(data.matches)
    }

    /// Fetch team statistics
    pub async fn fetch_team_stats(&self, team_id: i32) -> anyhow::Result<ApiTeamStats> {
        let url = format!("{}/teams/{}", self.base_url, team_id);

        let response = self
            .client
            .get(&url)
            .header("X-Auth-Token", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("API request failed: {}", response.status());
        }

        let data: ApiTeamStats = response.json().await?;
        Ok(data)
    }
}

// API response structures
// These are templates - adjust based on actual API

#[derive(Debug, Deserialize)]
pub struct FixturesResponse {
    pub matches: Vec<ApiMatch>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiMatch {
    pub id: i32,
    #[serde(rename = "utcDate")]
    pub utc_date: String,
    pub status: String,
    pub matchday: i32,
    #[serde(rename = "homeTeam")]
    pub home_team: ApiTeamInfo,
    #[serde(rename = "awayTeam")]
    pub away_team: ApiTeamInfo,
    pub score: Option<ApiScore>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiTeamInfo {
    pub id: i32,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub crest: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiScore {
    pub winner: Option<String>,
    #[serde(rename = "fullTime")]
    pub full_time: ApiScoreDetail,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiScoreDetail {
    pub home: Option<i32>,
    pub away: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ApiTeamStats {
    pub id: i32,
    pub name: String,
    // Add more fields based on API response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = FootballApiClient::new("test_key".to_string());
        assert_eq!(client.api_key, "test_key");
    }
}
