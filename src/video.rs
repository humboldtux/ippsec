use anyhow::Result;
use serde::{Deserialize, Serialize};

const DATASET: &str =
    "https://raw.githubusercontent.com/IppSec/ippsec.github.io/master/dataset.json";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Academy {
    pub machine: String,
    pub academy: String,
    pub line: String,
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Youtube {
    pub machine: String,
    pub video_id: String,
    pub timestamp: Timestamp,
    pub tag: String,
    pub line: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Video {
    Academy(Academy),
    Youtube(Youtube),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timestamp {
    pub minutes: u32,
    pub seconds: u32,
}

impl Video {
    pub fn load() -> Result<Vec<Video>> {
        let videos = reqwest::blocking::get(DATASET)?.json::<Vec<Video>>()?;

        Ok(videos)
    }

    pub fn url(&self) -> String {
        match self {
            Video::Academy(a) => {
                return format!("https://academy.hackthebox.eu/module/details/{}", a.academy)
            }
            Video::Youtube(y) => {
                let time = 60 * y.timestamp.minutes + y.timestamp.seconds;
                return format!("https://www.youtube.com/watch?v={}&t={}", y.video_id, time);
            }
        }
    }
}
