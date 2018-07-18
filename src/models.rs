use std::collections::HashSet;
use chrono::{DateTime, Utc};

//#[derive(Debug, Deserialize, Serialize)]

// ENUMS ------------------------------------------- (alphabetical) ------------------------------------------- //

pub enum AuditEvent {
    Creation {
        created_on: DateTime<Utc>,
        created_by: User
    },
    Edit {
        last_edited: DateTime<Utc>,
        edited_by: User
    },
    Activity (DateTime<Utc>)
}

pub enum CompanyType {
    Bank,
    Gov,
    GP,
    Inc,
    LLC,
    LLLP,
    LLP,
    LP,
    NPInc,
    P,
    PA,
    PC,
    PLLC,
    SDERL
}

pub enum Contact {
    Company(Company),
    Person(Person),
    Trust(Trust)
}

pub enum Gender {
    Male,
    Female
}

pub enum MaritalProperty {
    Separate,
    Community
}

pub enum MaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed
}

pub enum Places {
    Country(Country),
    State(State)
}

pub enum TaxClass {
    FC,
    FGTrust,
    FNGTrust,
    FSMDE,
    NRA,
    RA,
    SCorp,
    USCCorp,
    USCitizen,
    USGTrust,
    USNGTrust,
    USSMDE
}

pub enum TaxID {
    EIN(u32),
    FOREIGN(String),
    ITIN(u32),
    SSN(u32)
}
// STRUCTS ------------------------------------------- (alphabetical) ------------------------------------------- //

pub struct Address {
    pub street: String,
    pub city_state: String,
    pub country: HashSet<Country>,
    pub postal_code: Option<String>,
    pub active: bool,
    pub notes: Option<String>,
    pub audit_trail: HashSet<AuditEvent>
}

pub struct Asset {
    // will continue from here
}

pub struct Company {
    pub uid: u64,
    pub name: String,
    pub jurisdiction: Country,
    pub company_type: CompanyType,
    pub formation_date: Option<DateTime<Utc>>,
    pub tax_profile: Option<TaxProfile>,
    pub contact_persons: HashSet<Person>,
    pub owners: Option<HashSet<Box<Contact>>>,
    pub subsidiaries: Option<HashSet<Box<Company>>>,
    //pub assets:
    pub audit_trail: HashSet<AuditEvent>

}

pub struct Country {
    pub name: String
}

pub struct Email {
    pub address: String,
    pub primary: bool,
    pub active: bool,
    pub notes: Option<String>,
    pub audit_trail: HashSet<AuditEvent>
}

pub struct Person {
    pub uid: u64,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub gender: Gender,
    pub start_date: Option<DateTime<Utc>>,
    pub inactive: bool,
    pub end_date: Option<DateTime<Utc>>,
    pub marital_status: Option<MaritalStatus>,
    pub marital_property: Option<MaritalProperty>,
    pub marital_start_date: Option<DateTime<Utc>>,
    pub marital_end_date: Option<DateTime<Utc>>,
    pub spouse: Option<Box<Person>>,
    pub parents: Option<HashSet<Box<Person>>>,
    pub children: Option<HashSet<Box<Person>>>,
    pub tax_profile: Option<TaxProfile>,
    pub citizenships: Option<HashSet<Country>>,
    pub addresses: Option<HashSet<Address>>,
    pub emails: Option<HashSet<Email>>,
    pub phone_numbers: Option<HashSet<Phone>>,
    pub notes: String,
    pub real: bool,
    pub audit_trail: HashSet<AuditEvent>
}

pub struct Phone {
    pub number: u64,
    pub primary: bool,
    pub active: bool,
    pub notes: Option<String>,
    pub audit_trail: HashSet<AuditEvent>
}

pub struct State {
    pub name: String
}

pub struct TaxProfile {
    pub tax_class: Option<TaxClass>,
    pub us_tax_id: Option<TaxID>,
    pub foreign_tax_id: Option<TaxID>,
    pub fatca_status: String,
    pub taxable_jurisdictions: HashSet<Places>
}

pub struct Trust {
    pub uid: u64,
    pub name: String,
    pub contact_persons: HashSet<Person>,
    pub tax_profile: Option<TaxProfile>,
    // add more fields here,
    pub audit_trail: HashSet<AuditEvent>
}

pub struct User {
    pub uid: u64,
    pub username: String,
    pub email: Email,
    pub person: Option<Box<Person>>,
    pub active: bool,
    pub audit_trail: HashSet<AuditEvent>
}








//
//impl Contact {
//    fn uid(&self) -> u64 {
//        match self {
//            Contact::Company(c) => c.uid,
//            Contact::Person(p) => p.uid,
//            Contact::Trust(t) => t.uid
//        }
//    }
//
//    fn email(&self) -> Vec<String> {
//        let get_emails = |entity| {
//
//        };

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
//}











