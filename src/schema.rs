table! {
    expenses (id) {
        id -> Int4,
        paid_for_id -> Nullable<Int4>,
        paid_by_id -> Nullable<Int4>,
        author_id -> Nullable<Int4>,
        project_id -> Nullable<Int4>,
        date -> Date,
        amount -> Float8,
        description -> Nullable<Varchar>,
        name -> Varchar,
        expense_type -> Nullable<Varchar>,
    }
}

table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Date,
        total_expenses -> Float8,
        currency -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        balance -> Nullable<Float8>,
    }
}

joinable!(expenses -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    expenses,
    projects,
    users,
);
