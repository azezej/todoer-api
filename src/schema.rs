// @generated automatically by Diesel CLI.

diesel::table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
    }
}

diesel::table! {
    todolists (id) {
        id -> Int4,
        user_id -> Int4,
        shared_with -> Nullable<Text>,
        parent_list_id -> Nullable<Int4>,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
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
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        login_session -> Text,
    }
}

diesel::joinable!(login_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    login_history,
    todolists,
    todotasks,
    users,
);
