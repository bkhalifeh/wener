// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_status"))]
    pub struct UserStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserStatus;

    users (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 254]
        email -> Varchar,
        #[max_length = 93]
        password -> Bpchar,
        #[max_length = 40]
        profile -> Bpchar,
        status -> UserStatus,
        is_admin -> Bool,
    }
}
