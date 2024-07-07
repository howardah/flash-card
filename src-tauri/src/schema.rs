// @generated automatically by Diesel CLI.

diesel::table! {
    set (id) {
        id -> Integer,
        #[max_length = 191]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    word (id) {
        id -> Integer,
        #[sql_name = "word"]
        word_value -> Varchar,
        #[max_length = 191]
        gender -> Varchar,
        #[max_length = 191]
        translation -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    word_set (word_id, set_id) {
        word_id -> Integer,
        set_id -> Integer,
    }
}

diesel::table! {
    _prisma_migrations (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 64]
        checksum -> Varchar,
        finished_at -> Nullable<Timestamp>,
        #[max_length = 255]
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamp>,
        started_at -> Timestamp,
        applied_steps_count -> Integer,
    }
}

diesel::joinable!(word_set -> set (set_id));
diesel::joinable!(word_set -> word (word_id));

diesel::allow_tables_to_appear_in_same_query!(set, word, word_set, _prisma_migrations,);
