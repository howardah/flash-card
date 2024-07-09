use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    thread
};

// use diesel::{prelude::*, sqlite::Sqlite};
// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

use tauri::{AppHandle, Manager};

use crate::words::get_words;

// fn run_migrations(
//     connection: &mut impl MigrationHarness<Sqlite>,
// ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//     // This will run the necessary migrations.
//     //
//     // See the documentation for `MigrationHarness` for
//     // all available methods.
//     connection.run_pending_migrations(MIGRATIONS)?;

//     Ok(())
// }

pub fn init_db(app_handle: &AppHandle) {
    let path = app_handle.path();
    let data_dir = path.app_data_dir().unwrap().clone();

    thread::spawn(move || {
        // Print message to console
        println!("Initializing database...");
        let database_path = data_dir.join("flash_card.db");
        // let mut connection = SqliteConnection::establish(database_path.to_str().unwrap()).unwrap();

        // run_migrations(&mut connection).unwrap();
    });
}

pub async fn add_words_from_cloud(app_handle: &AppHandle) {
    let path = app_handle.path();
    let data_dir = path.app_data_dir().unwrap();

    println!("Adding words from cloud...");
    
    // Fetch contents of "https://gender-flash-card.netlify.app/api/words/all" 
    // in a new thread

    thread::spawn(move || {
        // Use an async block within the thread
        tauri::async_runtime::block_on(async {
            let words = get_words();

            println!("Fetched words from cloud: {:?}", words);
        });
    });
}

#[tauri::command]
pub fn save_word(word: &str, app_handle: AppHandle) -> String {
    let path = app_handle.path();
    let data_dir = path.app_data_dir().unwrap();

    // Save word to word.txt
    let word_path = data_dir.join("word.txt");
    let mut word_file = File::create(word_path).unwrap();
    word_file.write_all(word.as_bytes()).unwrap();

    String::from(word)
}

#[tauri::command]
pub fn get_word(app_handle: AppHandle) -> String {
    let path = app_handle.path();
    let data_dir = path.app_data_dir().unwrap();
    let word_path = data_dir.join("word.txt");

    let mut word = String::new();
    if let Ok(mut word_file) = File::open(word_path) {
        word_file.read_to_string(&mut word).unwrap();
    }

    word
}