use crate::models::ImageFile;
use rusqlite::{Connection, Result, params};
use std::collections::HashMap;

pub struct Database {
    conn: Option<Connection>,
}

impl Database {
    pub fn new() -> Self {
        Self { conn: None }
    }

    pub fn init(&self) -> Result<()> {
        let conn = Connection::open("lensfolio.db")?;
        
        // 创建图片表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS images (
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                size INTEGER NOT NULL,
                date_created TEXT NOT NULL,
                date_taken TEXT,
                rating INTEGER DEFAULT 0,
                thumbnail TEXT,
                width INTEGER,
                height INTEGER,
                camera_make TEXT,
                camera_model TEXT,
                lens_model TEXT,
                iso INTEGER,
                aperture REAL,
                shutter_speed TEXT,
                focal_length REAL
            )",
            [],
        )?;

        // 创建标签表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                image_id TEXT NOT NULL,
                tag TEXT NOT NULL,
                FOREIGN KEY (image_id) REFERENCES images (id),
                UNIQUE(image_id, tag)
            )",
            [],
        )?;

        // 创建索引
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tags_image_id ON tags(image_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tags_tag ON tags(tag)",
            [],
        )?;

        Ok(())
    }

    fn get_connection(&self) -> Result<Connection> {
        Connection::open("lensfolio.db")
    }

    pub fn insert_image(&self, image: &ImageFile) -> Result<()> {
        let conn = self.get_connection()?;
        
        // 插入图片记录
        conn.execute(
            "INSERT OR REPLACE INTO images 
            (id, path, name, size, date_created, date_taken, rating, thumbnail, 
             width, height, camera_make, camera_model, lens_model, iso, aperture, shutter_speed, focal_length)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                image.id,
                image.path,
                image.name,
                image.size as i64,
                image.date_created,
                image.date_taken,
                image.rating,
                image.thumbnail,
                image.width.map(|w| w as i64),
                image.height.map(|h| h as i64),
                image.camera_make,
                image.camera_model,
                image.lens_model,
                image.iso.map(|i| i as i64),
                image.aperture,
                image.shutter_speed,
                image.focal_length
            ],
        )?;

        // 插入标签
        for tag in &image.tags {
            conn.execute(
                "INSERT OR IGNORE INTO tags (image_id, tag) VALUES (?1, ?2)",
                params![image.id, tag],
            )?;
        }

        Ok(())
    }

    pub fn get_all_images(&self) -> Result<Vec<ImageFile>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, path, name, size, date_created, date_taken, rating, thumbnail,
                    width, height, camera_make, camera_model, lens_model, iso, aperture, shutter_speed, focal_length
             FROM images ORDER BY date_created DESC"
        )?;

        let image_iter = stmt.query_map([], |row| {
            Ok(ImageFile {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                size: row.get::<_, i64>(3)? as u64,
                date_created: row.get(4)?,
                date_taken: row.get(5)?,
                rating: row.get(6)?,
                thumbnail: row.get(7)?,
                width: row.get::<_, Option<i64>>(8)?.map(|w| w as u32),
                height: row.get::<_, Option<i64>>(9)?.map(|h| h as u32),
                camera_make: row.get(10)?,
                camera_model: row.get(11)?,
                lens_model: row.get(12)?,
                iso: row.get::<_, Option<i64>>(13)?.map(|i| i as u32),
                aperture: row.get(14)?,
                shutter_speed: row.get(15)?,
                focal_length: row.get(16)?,
                tags: Vec::new(), // 稍后填充
            })
        })?;

        let mut images: Vec<ImageFile> = image_iter.collect::<Result<Vec<_>, _>>()?;

        // 获取所有图片的标签
        let mut tag_stmt = conn.prepare("SELECT image_id, tag FROM tags")?;
        let tag_iter = tag_stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut tags_map: HashMap<String, Vec<String>> = HashMap::new();
        for tag_result in tag_iter {
            let (image_id, tag) = tag_result?;
            tags_map.entry(image_id).or_default().push(tag);
        }

        // 为图片填充标签
        for image in &mut images {
            if let Some(tags) = tags_map.remove(&image.id) {
                image.tags = tags;
            }
        }

        Ok(images)
    }

    pub fn get_image(&self, image_id: &str) -> Result<ImageFile> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, path, name, size, date_created, date_taken, rating, thumbnail,
                    width, height, camera_make, camera_model, lens_model, iso, aperture, shutter_speed, focal_length
             FROM images WHERE id = ?1"
        )?;

        let mut image = stmt.query_row([image_id], |row| {
            Ok(ImageFile {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                size: row.get::<_, i64>(3)? as u64,
                date_created: row.get(4)?,
                date_taken: row.get(5)?,
                rating: row.get(6)?,
                thumbnail: row.get(7)?,
                width: row.get::<_, Option<i64>>(8)?.map(|w| w as u32),
                height: row.get::<_, Option<i64>>(9)?.map(|h| h as u32),
                camera_make: row.get(10)?,
                camera_model: row.get(11)?,
                lens_model: row.get(12)?,
                iso: row.get::<_, Option<i64>>(13)?.map(|i| i as u32),
                aperture: row.get(14)?,
                shutter_speed: row.get(15)?,
                focal_length: row.get(16)?,
                tags: Vec::new(),
            })
        })?;

        // 获取标签
        let mut tag_stmt = conn.prepare("SELECT tag FROM tags WHERE image_id = ?1")?;
        let tag_iter = tag_stmt.query_map([image_id], |row| row.get(0))?;
        image.tags = tag_iter.collect::<Result<Vec<String>, _>>()?;

        Ok(image)
    }

    pub fn add_tag(&self, image_id: &str, tag: &str) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute(
            "INSERT OR IGNORE INTO tags (image_id, tag) VALUES (?1, ?2)",
            params![image_id, tag],
        )?;
        Ok(())
    }

    pub fn set_rating(&self, image_id: &str, rating: i32) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute(
            "UPDATE images SET rating = ?1 WHERE id = ?2",
            params![rating, image_id],
        )?;
        Ok(())
    }
} 