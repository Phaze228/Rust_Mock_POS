use diesel::prelude::*;
use crate::schema::employees;
use std::{sync::atomic::{self, AtomicI32}};

const EMPLOYEE_ID: AtomicI32 = AtomicI32::new(1);



#[derive(Debug, diesel_derive_enum::DbEnum, PartialEq, Default, Clone, Copy, Eq, Ord, PartialOrd)]
#[ExistingTypePath = "crate::schema::sql_types::Role"]
#[DbValueStyle = "UPPERCASE"]
pub enum Role {
    CEO,
    SUPERVISOR,
    ANALYST,
    HR,
    #[default]
    UNEMPLOYED,
}

impl Role {
    pub fn starting_pay(&self) -> Option<i32> {
        match &self {
            Role::CEO => Some(1_000_000),
            Role::SUPERVISOR => Some(125_000),
            Role::ANALYST => Some(150_000),
            Role::HR => Some(85_000),
            Role::UNEMPLOYED => Some(0),
            _ => Some(0)

        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let role_string = match self {
            Self::CEO => "CEO",
            Self::SUPERVISOR => "SUPERVISOR",
            Self::ANALYST => "ANALYST",
            Self::HR => "HR EMPLOYEE",
            Self::UNEMPLOYED => "UNEMPLOYED",

        };
        write!(f, "{}" , role_string,
    )
}
}


#[derive(Queryable, Selectable, Identifiable, Insertable, PartialEq, Default, Debug, Clone, Eq, Ord, PartialOrd)]
#[diesel(table_name = employees)]
pub struct Employee {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[diesel(deserialize_as = Role)]
    pub position: Role,
    #[diesel(deserialize_as = i32)]
    pub pay: Option<i32>,
    pub employed: bool,
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Employee ID: #{}\nName: {} {}\nEmail: {}\nPosition: {}\nCurrent Pay: ${}\nCurrently Employed: {}", 
                        self.id.unwrap_or(0), self.first_name, self.last_name, self.email, self.position, self.pay.unwrap_or(0), self.employed)
    }
}

impl Employee {
    pub fn new(first_name: String, last_name: String, email: String, position: Role, employed: bool ) -> Employee {
        let id = Option::Some(EMPLOYEE_ID.fetch_add(1, atomic::Ordering::SeqCst));
        let pay = position.starting_pay();
        Employee {id, first_name, last_name, email, position, pay, employed}
    }

    pub fn make_default(&mut self) -> &mut Employee {
        self.first_name = String::new();
        self.last_name = String::new();
        self.email = String::new();
        self.position = Role::UNEMPLOYED;
        self
    }
}
//___________________________________
// NEW EMPLOYEE FOR INSERTION // 
//____________________________________
#[derive(Selectable, Insertable, PartialEq, Default, Debug, Clone, Eq, Ord, PartialOrd)]
#[diesel(table_name = employees)]
pub struct NewEmployee {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub position: Role,
    pub pay: Option<i32>,
    pub employed: bool,
}

impl std::fmt::Display for NewEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nName: {} {}\nEmail: {}\nPosition: {}\nCurrent Pay: ${}\nCurrently Employed: {}\n", 
                        self.first_name, self.last_name, self.email, self.position, self.pay.unwrap_or(0), self.employed)
    }
}

impl NewEmployee {
    pub fn new(first_name: String, last_name: String, email: String, position: Role, employed: bool ) -> NewEmployee {
        let pay = position.starting_pay();
        let employed = true;
        NewEmployee {first_name, last_name, email, position, pay, employed}
    }

    pub fn make_default(&mut self) -> &mut NewEmployee {
        self.first_name = String::new();
        self.last_name = String::new();
        self.email = String::new();
        self.position = Role::UNEMPLOYED;
        self
    }
}


pub struct User {
   pub username: String,
   pub password: String,
}

impl Default for User {
    fn default() -> Self {
        let mut username = String::new();
        let mut password = String::new();
        User { username, password }
    }
}

impl User {
    pub fn default_username() -> String {
        String::new()
    }

    pub fn default_password() -> String {
        String::new()
    }
    }



