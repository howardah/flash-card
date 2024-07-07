use std::{fs::File, io::{Read, Write}};

use tauri::{AppHandle, Manager};

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