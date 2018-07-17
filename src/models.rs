use std::collections::HashSet;
use chrono::{DateTime, Utc};

//#[derive(Debug, Deserialize, Serialize)]
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
            Contact::Company(c) => c.uid,
            Contact::Person(p) => p.uid,
            Contact::Trust(t) => t.uid
        }
    }

    fn email(&self) -> Vec<String> {
        let get_emails = |entity| {

        };

//        match self {
//            Contact::Company(_) | Contact::Trust(_) => {
//                fn email<T>(entity: &T) -> <Vec<String>> {
//            },
//            Person(p) => p.uid,
//            Trust(t) => t.uid
//        };
//        self.contact_persons
//            .into_iter()
//            .map(|person| person.email)
//            .collect::<Vec<String>>()
//    }
}



//#[derive(Debug, Deserialize, Serialize)]
pub struct Company {
    pub uid: u64,
    pub name: String,
    pub contact_persons: HashSet<Person>,
    // add more fields here
}



//#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub uid: u64,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub emails: Vec<String>,
    pub gender: Gender,
    pub start_date: DateTime<Utc>,
    pub inactive: bool,
    pub end_date: Option<DateTime<Utc>>, // I understand that this is likely empty, but we won't have a birthday for everyone either...
    pub marital_status: MaritalStatus,
    pub marital_property: MaritalProperty,
    pub marital_start_date: DateTime<Utc>,
    pub marital_end_date: Option<DateTime<Utc>>,
    //pub tax_class: '', // will make one large enum for all classes for entities and people
    //pub us_tax_id: '', // there are different types, EIN, ITIN, SSN, do we care? or just take the number?m yes with ENUM
    pub notes: String,
    //pub addresses: '', // many to many...
    //pub phone_numbers: '', // many-to-many./..
    pub real: bool, // if Person is real or fictitious
    pub created_on: DateTime<Utc>,
    //pub created_by: '', // make STRUCT of USERS
    // how do we deal with related parties, such as who their spouse is? or many-to-many relations such as many e-mail addresses? - structs and vects
}


//#[derive(Debug, Deserialize, Serialize)]
pub struct Trust {
    pub uid: u64,
    pub name: String,
    pub contact_persons: HashSet<Person>,
    // add more fields here
}