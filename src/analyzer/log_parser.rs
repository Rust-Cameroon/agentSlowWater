use crate::errors::AppError;
use reqwest::Client;
use std::env;

pub async fn parse_logs(logs_uri: &str) -> Result<Vec<String>, AppError> {
    let client = Client::new();

    // Determine if GitHub or GitLab based on URL
    let is_github = logs_uri.contains("github.com");

    let mut req = client.get(logs_uri);

    if is_github {
        if let Ok(token) = env::var("GITHUB_TOKEN") {
            req = req.bearer_auth(token);
        }
    }

    let response = req.send().await.map_err(|e| {
        AppError::Http(anyhow::anyhow!("Failed to fetch logs: {}", e).into())
    })?;

    if !response.status().is_success() {
        return Err(AppError::BadRequest(format!(
            "Failed to fetch logs: HTTP {}",
            response.status()
        )));
    }

    let text = response.text().await.map_err(|e| {
        AppError::Http(anyhow::anyhow!("Failed to read logs: {}", e).into())
    })?;

    Ok(text.lines().map(str::to_string).collect())
}
