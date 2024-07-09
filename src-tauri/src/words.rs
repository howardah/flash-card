use once_cell::sync::Lazy; // Add this import

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub gender: String,
    pub translation: String,
    pub created_at: String,
    pub updated_at: String,
}

pub static WORDS: Lazy<Vec<Word>> = Lazy::new(|| {
    let data = include_str!("./words.json");
    serde_json::from_str(data).expect("Error parsing words.json")
});