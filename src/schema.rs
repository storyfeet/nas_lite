// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> BigInt,
        user_id -> BigInt,
        file_name -> Text,
        file_type -> Text,
        file_size -> BigInt,
        chunk_size -> BigInt,
        chunks_loaded -> Integer,
        file_hash -> Text,
        completed -> Nullable<Timestamp>,
        created -> Timestamp,
        modified -> Timestamp,
        deleted -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions (token) {
        token -> Text,
        token_pass -> Text,
        user_id -> BigInt,
        created -> Timestamp,
        expires -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> BigInt,
        created -> Timestamp,
        modified -> Timestamp,
        user_name -> Text,
        password_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(files, sessions, users,);
diesel::joinable!(sessions -> users(user_id));
