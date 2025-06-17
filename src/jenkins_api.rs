use chrono::{DateTime, TimeZone, Utc};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct BuildInfo {
    result: Option<String>,
    duration: u64,
    timestamp: u64,
    building: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuildSummary {
    pub(crate) name: String,
    pub(crate) status: String,
    pub(crate) start_time: DateTime<Utc>,
    pub(crate) duration: f64,
}

pub async fn get_job_data(job_name: &str) -> Result<BuildSummary, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let url = format!("{}/job/{}/lastBuild/api/json", std::env::var("JENKINS_URL")?, job_name);
    let user = std::env::var("JENKINS_USER")?;
    let token = std::env::var("JENKINS_TOKEN")?;

    let client = Client::new();
    let response = client
        .get(&url)
        .basic_auth(user, Some(token))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Jenkins HTTP error: {}", response.status()).into());
    }

    let build_info: BuildInfo = response.json().await?;
    let start_time = Utc.timestamp_millis_opt(build_info.timestamp as i64)
        .single()
        .ok_or("Invalid timestamp")?;
    let duration = build_info.duration as f64 / 1000.0;

    let status = match (build_info.result, build_info.building) {
        (None, Some(true)) => "INPROGRESS".to_string(),
        (None, _) => "UNKNOWN".to_string(),
        (Some(result), _) => result,
    };

    Ok(BuildSummary {
        name: job_name.to_string(),
        status,
        start_time,
        duration,
    })
}