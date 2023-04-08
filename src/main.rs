use eframe::{NativeOptions, run_native};
use employee_db::{models::{Employee,Role::*}, establish_connection, insert_employee};
use diesel::prelude::*;
use employee_db::gui::POS;


fn main() {
    let emp = Employee::new("Alex".to_owned(), "Chinny".to_owned(), "Something@company.com".to_owned(), CEO, true);
    let mut connection = establish_connection();
    let mut win_options = NativeOptions::default();
    run_native("POS_SYSTEM", win_options, Box::new(|cc| Box::new(POS::new(cc))));

    //insert_employee(&mut connection, emp)
}
