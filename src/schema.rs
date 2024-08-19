// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "roles"))]
    pub struct Roles;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Roles;

    user_roles (user_id, role) {
        user_id -> Uuid,
        role -> Roles,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 256]
        email -> Varchar,
        #[max_length = 2048]
        password -> Varchar,
        #[max_length = 128]
        first_name -> Varchar,
        #[max_length = 128]
        last_name -> Varchar,
        #[max_length = 128]
        bio -> Nullable<Varchar>,
        #[max_length = 2048]
        dp_url -> Nullable<Varchar>,
    }
}

diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_roles,
    users,
);
