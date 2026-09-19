// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Integer,
        user_id -> Integer,
        file_name -> Text,
        file_type -> Text,
        file_size -> Integer,
        chunk_size -> Integer,
        chunks_loaded -> Integer,
        file_hash -> Text,
        completed -> Nullable<Timestamp>,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Integer,
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
        created -> Timestamp,
        modified -> Timestamp,
        user_name -> Text,
        password_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(files, sessions, users,);
