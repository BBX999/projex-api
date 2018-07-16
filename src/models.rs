use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female
}

pub trait Contact {
    fn uid(&self) -> u64;
    fn email(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Company {
    pub uid: u64,
    pub name: String,
    pub email: String,
}

impl Contact for Company {
    fn uid(&self) -> u64 {
        self.uid
    }

    fn email(&self) -> String {
        format!("{:?}", &self.email)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub uid: u64,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email: String,
    pub gender: Gender,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

impl Contact for Person {
    fn uid(&self) -> u64 {
        self.uid
    }

    fn email(&self) -> String {
        format!("{:?}", &self.email)
    }
}