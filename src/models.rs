use std::collections::HashSet;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female
}

pub enum MaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed
}

pub enum MaritalProperty {
    Separate,
    Community
}

pub struct Email {
    pub address: String,
    pub primary: bool,
    pub active: bool,
    pub notes: String
}

pub enum Contact {
    Company(Company),
    Person(Person),
    Trust(Trust)
}

impl Contact {
    fn uid(&self) -> u64 {
        match self {
            Company(c) => c.uid,
            Person(p) => p.uid,
            Trust(t) => t.uid
        }
    }

    fn email(&self) -> Vec<String> {
        match self {
            Company(_) | Trust(_) => {
                fn email<T>(entity: &[T]) -> <Vec<String>> {
                  for &contact_person in entity.
                }

            },
            Person(p) => p.uid,
            Trust(t) => t.uid
        };
        self.contact_persons
            .into_iter()
            .map(|person| person.email)
            .collect::<Vec<String>>()
    }
}



#[derive(Debug, Deserialize, Serialize)]
pub struct Company {
    pub uid: u64,
    pub name: String,
    pub contact_persons: HashSet<Person>,
    // add more fields here
}



#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub uid: u64,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email: String, // many-to-many, so not sure if string is the way to go?
    pub gender: Gender,
    pub start_date: DateTime<Utc>,
    pub inactive: bool,
    pub end_date: Option<DateTime<Utc>>, // I understand that this is likely empty, but we won't have a birthday for everyone either...
    pub marital_status: Marital_Status,
    pub marital_property: Marital_Property,
    pub marital_start_date: DateTime<Utc>, // should this be Option? If not option, then does that mean required value?
    pub marital_end_date: Option<DateTime<Utc>>, // should this be Option?
    pub tax_class: '', // enum within trait? or two enums? different for companies / trusts and people
    // (Non-Resident Alien, Resident Alien, U.S. Citizen) -- how easy is it to change enum names later? need to change everywhere else?\
    // or would you have tax_class of type NRA, and NRA in enum has a string value which you can change without needing to change code elsewhere?
    pub us_tax_id: '', // there are different types, EIN, ITIN, SSN, do we care? or just take the number?
    pub notes: String,
    pub addresses: '', // many to many...
    pub phone_numbers: '', // many-to-many./..
    pub real: bool, // if Person is real or fictitious
    pub created_on: DateTime<Utc>,
    pub created_by: '', // should the users of the system be Persons in the database, or some other class like User?
    // how do we deal with related parties, such as who their spouse is? or many-to-many relations such as many e-mail addresses?
}

impl Contact for Person {
    fn uid(&self) -> u64 {
        self.uid
    }

    fn email(&self) -> String {
        format!("{:?}", &self.email) // why {:?} is not just {} enough? I thought :? was for arrays/objects etc.
    }
}

fn test_function(x: Option<String>) -> String {
    match x {
        Some(t) => format!("Yay: {:?}", t),
        None => format!("noooooo")

    }

}