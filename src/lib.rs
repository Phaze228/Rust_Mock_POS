use std::env;

use diesel::{PgConnection, Connection, debug_query, query_dsl::select_dsl::SelectDsl};
use models::{Employee, NewEmployee};

pub mod models;
pub mod schema;
pub mod gui;
use diesel::RunQueryDsl;
use dotenvy::dotenv;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE MUST BE SET");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error with DB"))
}

pub fn insert_employee(emp: NewEmployee) -> Employee {
    let mut conn =  establish_connection();
    let query =  diesel::insert_into(schema::employees::table).values(&emp);
    let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);
    query.get_result::<Employee>(&mut conn).expect("FAILED")
}

pub fn get_employees() -> Vec<Employee> {
    use self::schema::employees::dsl::*;
    let connection = &mut establish_connection();
    let results = employees.load::<Employee>(connection).expect("Error Loading Database");
    results
}