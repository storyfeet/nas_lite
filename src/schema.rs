// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Integer,
        user_id -> Integer,
        file_name -> Nullable<Text>,
        parent -> Nullable<Integer>,
        content -> Nullable<Binary>,
        file_hash -> Nullable<Text>,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions (rowid) {
        rowid -> Integer,
        token -> Text,
        token_pass -> Text,
        user_id -> Integer,
        created -> Timestamp,
        expires -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        user_name -> Nullable<Text>,
        password_hash -> Nullable<Binary>,
        created -> Timestamp,
        modified -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(files, sessions, users,);
