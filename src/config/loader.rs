use crate::config::Config;
use crate::errors::AppError;
use crate::perceiver::event::{NormalizedEvent, Platform};
use serde_yaml;
use serde_json;

pub async fn load_for_event(event: &NormalizedEvent) -> Result<Config, AppError> {
    match event.platform {
        Platform::GitHub => load_from_github(event).await,
        Platform::GitLab => load_from_gitlab(event).await,
    }
}

async fn load_from_github(event: &NormalizedEvent) -> Result<Config, AppError> {
    use octocrab::Octocrab;

    let repo = event
        .metadata
        .get("repository")
        .ok_or_else(|| AppError::BadRequest("Missing repository metadata".into()))?;

    let sha = event
        .metadata
        .get("commit_sha")
        .ok_or_else(|| AppError::BadRequest("Missing commit_sha metadata".into()))?;

    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        return Err(AppError::BadRequest("Invalid repository format".into()));
    }

    let (owner, repo_name) = (parts[0], parts[1]);
    let octo = Octocrab::builder().build()?;

    for path in [".optimizer.yml", ".optimizer.json"] {
        let result = octo
            .repos(owner, repo_name)
            .get_content()
            .path(path)
            .r#ref(sha)
            .send()
            .await;

        if let Ok(response) = result {
            if let Some(file) = response.items.into_iter().next() {
                let decoded = base64::decode(file.content.replace('\n', ""))
                    .map_err(|e| AppError::ConfigError(e.to_string()))?;
                return parse_config_bytes(&decoded, path);
            }
        }
    }

    Ok(Config::default())
}

async fn load_from_gitlab(event: &NormalizedEvent) -> Result<Config, AppError> {
    use gitlab::Gitlab;

    let project = event
        .metadata
        .get("project")
        .ok_or_else(|| AppError::BadRequest("Missing GitLab project metadata".into()))?;

    let sha = event
        .metadata
        .get("commit_sha")
        .ok_or_else(|| AppError::BadRequest("Missing commit_sha metadata".into()))?;

    let token = std::env::var("GITLAB_TOKEN")
        .map_err(|_| AppError::ConfigError("Missing GITLAB_TOKEN".into()))?;
    let client = Gitlab::new("gitlab.com", token)?;

    for path in [".optimizer.yml", ".optimizer.json"] {
        let file = client
            .projects()
            .repository(project)
            .get_file(path, sha);

        if let Ok(file_info) = file {
            let content = base64::decode(file_info.content.replace('\n', ""))
                .map_err(|e| AppError::ConfigError(e.to_string()))?;
            return parse_config_bytes(&content, path);
        }
    }

    Ok(Config::default())
}

fn parse_config_bytes(bytes: &[u8], path: &str) -> Result<Config, AppError> {
    if path.ends_with(".yml") {
        serde_yaml::from_slice(bytes).map_err(|e| AppError::ConfigError(e.to_string()))
    } else {
        serde_json::from_slice(bytes).map_err(|e| AppError::ConfigError(e.to_string()))
    }
}
