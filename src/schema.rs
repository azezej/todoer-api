// @generated automatically by Diesel CLI.

diesel::table! {
    todolists (id) {
        id -> Int4,
        user_id -> Int4,
        shared_with -> Nullable<Text>,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

diesel::table! {
    todotasks (id) {
        id -> Int4,
        user_id -> Int4,
        todolist_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        parent_task_id -> Nullable<Int4>,
        due_date -> Nullable<Date>,
        done -> Bool,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todolists,
    todotasks,
    users,
);
