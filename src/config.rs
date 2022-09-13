use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

use crate::{diff_text, ExtraArgs, RequestProfile};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

impl DiffConfig {
    pub async fn load_yaml(path: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path).await?;
        Ok(serde_yaml::from_str(&content)?)
    }

    pub fn from_yaml(content: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }

    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    pub resp: ResponseProfile,
}

impl DiffProfile {
    pub async fn diff(&self, args: ExtraArgs) -> anyhow::Result<String> {
        let req1 = self.req1.send(&args).await?;
        let req2 = self.req2.send(&args).await?;

        let text1 = req1.get_text(&self.resp).await?;
        let text2 = req2.get_text(&self.resp).await?;

        diff_text(&text1, &text2)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}
