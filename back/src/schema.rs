// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "expense_type"))]
    pub struct ExpenseType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ExpenseType;

    expenses (id) {
        id -> Int4,
        author_id -> Int4,
        project_id -> Int4,
        date -> Date,
        amount -> Float8,
        description -> Nullable<Varchar>,
        name -> Varchar,
        expense_type -> ExpenseType,
    }
}

diesel::table! {
    payments (id) {
        id -> Int4,
        expense_id -> Int4,
        user_id -> Int4,
        is_debt -> Bool,
        amount -> Float8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        users -> Array<Nullable<Int4>>,
        created_at -> Nullable<Timestamp>,
        total_expenses -> Float8,
        currency -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        balance -> Nullable<Float8>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(expenses -> projects (project_id));
diesel::joinable!(expenses -> users (author_id));
diesel::joinable!(payments -> expenses (expense_id));
diesel::joinable!(payments -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    expenses,
    payments,
    projects,
    users,
);
