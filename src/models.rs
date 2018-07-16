use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub email: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>
}