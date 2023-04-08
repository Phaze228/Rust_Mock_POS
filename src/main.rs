use eframe::{NativeOptions, run_native};
use employee_db::{models::{Employee,Role::*}, establish_connection, insert_employee};
use diesel::prelude::*;
use employee_db::gui::POS;


fn main() {
    let mut win_options = NativeOptions::default();
    run_native("POS_SYSTEM", win_options, Box::new(|cc| Box::new(POS::new(cc))));

}
