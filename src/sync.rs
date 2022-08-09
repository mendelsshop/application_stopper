use crate::config;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use toml;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncToml {
    pub apps: Vec<Apps>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Apps {
    pub name: String,
    pub time_left: u64,
    pub last_sync: toml::Value,
    pub help_time: u64,
}

pub struct GistSync {
    pub client: reqwest::Client,
}

impl GistSync {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("app_stopper")
                .build()
                .unwrap(),
        }
    }
    #[tokio::main]
    pub async fn sync(
        &self,
        config: config::Gist,
        apps: Vec<Apps>,
    ) -> Result<(), Box<(dyn Error + 'static)>> {
        let request = format!("https://api.github.com/gists/{}", config.gist_id);
        let body = self
            .client
            .post(request.clone())
            .basic_auth(config.github_user.clone(), config.github_token.clone())
            .send()
            .await?
            .text()
            .await?;

        let body = serde_json::from_str::<serde_json::Value>(&body)?;
        let body = body["files"][&config.gist_file_name]["content"]
            .as_str()
            .unwrap();
        let mut body = toml::from_str::<SyncToml>(body)?;
        for mut i in &mut body.apps {
            for j in apps.iter() {
                if i.name == j.name {
                    i.time_left = j.time_left;
                }
            }
        }
        let body = toml::to_string(&body)?;
        let body = body.replace('\"', "\\\"");
        let body = format!(
            r#"{{ "files": {{ "{}": {{ "content": "{}" }} }} }}"#,
            config.gist_file_name, body
        );
        let body = body.replace('\n', "\\n");

        let body = match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(x) => x,
            Err(e) => {
                serde_json::Value::Null
            }
        };

        self.client
            .patch(request.clone())
            .basic_auth(config.github_user.clone(), config.github_token.clone())
            .json(&body)
            .send()
            .await?;

        Ok(())
    }
}
