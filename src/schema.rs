// @generated automatically by Diesel CLI.
// Allows postgres to see my Role type.
pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    employees (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        position -> Role,
        pay -> Int4,
        employed -> Bool,
    }
}
