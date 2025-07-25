// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod image_processor;
mod models;

use database::Database;
use image_processor::ImageProcessor;
use models::*;
use std::sync::{Arc, Mutex};
use tauri::{command, State};

type DatabaseState = Arc<Mutex<Database>>;

#[command]
async fn init_database(db: State<'_, DatabaseState>) -> Result<(), String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.init().map_err(|e| e.to_string())
}

#[command]
async fn scan_folder(
    db: State<'_, DatabaseState>,
    folder_path: String,
) -> Result<Vec<ImageFile>, String> {
    let processor = ImageProcessor::new();
    let path = std::path::Path::new(&folder_path);
    let images = processor.scan_folder(path).map_err(|e| e.to_string())?;
    
    let db = db.lock().map_err(|e| e.to_string())?;
    for image in &images {
        db.insert_image(image).map_err(|e| e.to_string())?;
    }
    
    Ok(images)
}

#[command]
async fn get_images(db: State<'_, DatabaseState>) -> Result<Vec<ImageFile>, String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    db.get_all_images().map_err(|e| e.to_string())
}

#[command]
async fn add_tag(
    db: State<'_, DatabaseState>,
    image_ids: Vec<String>,
    tag: String,
) -> Result<(), String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    for image_id in image_ids {
        db.add_tag(&image_id, &tag).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[command]
async fn set_rating(
    db: State<'_, DatabaseState>,
    image_ids: Vec<String>,
    rating: i32,
) -> Result<(), String> {
    let db = db.lock().map_err(|e| e.to_string())?;
    for image_id in image_ids {
        db.set_rating(&image_id, rating).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[command]
async fn rename_with_date(
    db: State<'_, DatabaseState>,
    image_ids: Vec<String>,
) -> Result<(), String> {
    let processor = ImageProcessor::new();
    let db = db.lock().map_err(|e| e.to_string())?;
    
    for image_id in image_ids {
        if let Ok(image) = db.get_image(&image_id) {
            processor.rename_with_date(&image).map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}

fn main() {
    let database = Arc::new(Mutex::new(Database::new()));

    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            init_database,
            scan_folder,
            get_images,
            add_tag,
            set_rating,
            rename_with_date
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 