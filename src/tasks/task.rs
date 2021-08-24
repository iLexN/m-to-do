use chrono::{DateTime, Utc, Local, serde::ts_seconds};
use std::fmt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at = Utc::now();
        Task {
            text,
            created_at,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        //{:<50}: a left-aligned string padded with 50 spaces.
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}
