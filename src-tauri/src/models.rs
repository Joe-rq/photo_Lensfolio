use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFile {
    pub id: String,
    pub path: String,
    pub name: String,
    pub size: u64,
    pub date_created: String,
    pub date_taken: Option<String>,
    pub tags: Vec<String>,
    pub rating: i32,
    pub thumbnail: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub iso: Option<u32>,
    pub aperture: Option<f32>,
    pub shutter_speed: Option<String>,
    pub focal_length: Option<f32>,
}

impl ImageFile {
    pub fn new(path: String, name: String, size: u64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            path,
            name,
            size,
            date_created: chrono::Utc::now().to_rfc3339(),
            date_taken: None,
            tags: Vec::new(),
            rating: 0,
            thumbnail: None,
            width: None,
            height: None,
            camera_make: None,
            camera_model: None,
            lens_model: None,
            iso: None,
            aperture: None,
            shutter_speed: None,
            focal_length: None,
        }
    }
} 