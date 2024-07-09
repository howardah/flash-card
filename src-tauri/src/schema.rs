// @generated automatically by Diesel CLI.

diesel::table! {
    set (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    word (id) {
        id -> Integer,
        #[sql_name = "word"]
        word_value -> Text,
        gender -> Text,
        translation -> Nullable<Text>,
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

diesel::allow_tables_to_appear_in_same_query!(
    set,
    word,
    word_set,
);
