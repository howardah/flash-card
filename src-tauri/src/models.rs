use diesel::prelude::*;
// use diesel::sql_types::{Integer, Varchar, Timestamp, Nullable};
// use diesel::{Identifiable, Associations, Queryable, Selectable};
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = crate::schema::word)]
pub struct Word {
    pub id: i32,
    pub word_value: String,
    pub gender: String,
    pub translation: Option<String>,
    pub created_at: NaiveDateTime, // Explicitly specify the type
    pub updated_at: NaiveDateTime, // Explicitly specify the type
}

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = crate::schema::set)]
pub struct Set {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Debug)]
#[primary_key(word_id, set_id)]
#[belongs_to(Word, foreign_key = "word_id")]
#[belongs_to(Set, foreign_key = "set_id")]
#[diesel(table_name = crate::schema::word_set)]
pub struct WordSet {
    pub word_id: i32,
    pub set_id: i32,
}