use crate::models::ImageFile;
use std::path::Path;
use walkdir::WalkDir;
use chrono::{DateTime, Utc};

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn scan_folder(&self, folder_path: &Path) -> Result<Vec<ImageFile>, Box<dyn std::error::Error>> {
        let mut images = Vec::new();
        
        // 支持的图片格式
        let supported_extensions = vec!["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"];

        for entry in WalkDir::new(folder_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(extension) = entry.path().extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if supported_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            if let Ok(image) = self.process_image_file(entry.path()) {
                                images.push(image);
                            }
                        }
                    }
                }
            }
        }

        Ok(images)
    }

    fn process_image_file(&self, path: &Path) -> Result<ImageFile, Box<dyn std::error::Error>> {
        let metadata = std::fs::metadata(path)?;
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut image = ImageFile::new(
            path.to_string_lossy().to_string(),
            file_name,
            metadata.len(),
        );

        // 读取图片尺寸
        if let Ok(img) = image::open(path) {
            image.width = Some(img.width());
            image.height = Some(img.height());
        }

        // 使用文件的修改时间作为拍摄时间的替代
        if let Ok(modified) = metadata.modified() {
            if let Ok(datetime) = modified.duration_since(std::time::UNIX_EPOCH) {
                let dt = DateTime::<Utc>::from_timestamp(datetime.as_secs() as i64, 0)
                    .unwrap_or_else(|| Utc::now());
                image.date_taken = Some(dt.to_rfc3339());
            }
        }

        Ok(image)
    }

    pub fn rename_with_date(&self, image: &ImageFile) -> Result<(), Box<dyn std::error::Error>> {
        let current_path = Path::new(&image.path);
        let parent_dir = current_path.parent().unwrap_or(Path::new("."));
        
        // 使用拍摄日期或创建日期
        let date_str = if let Some(ref date_taken) = image.date_taken {
            if let Ok(dt) = DateTime::parse_from_rfc3339(date_taken) {
                dt.format("%Y%m%d_%H%M%S").to_string()
            } else {
                "unknown_date".to_string()
            }
        } else {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&image.date_created) {
                dt.format("%Y%m%d_%H%M%S").to_string()
            } else {
                "unknown_date".to_string()
            }
        };

        // 获取文件扩展名
        let extension = current_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("jpg");

        // 构建新文件名
        let base_name = format!("{}_{}", date_str, image.name);
        let new_filename = if base_name.ends_with(&format!(".{}", extension)) {
            base_name
        } else {
            format!("{}.{}", base_name.strip_suffix(&format!(".{}", extension)).unwrap_or(&base_name), extension)
        };

        let new_path = parent_dir.join(&new_filename);

        // 如果新路径不同于当前路径，则重命名
        if new_path != current_path {
            std::fs::rename(current_path, &new_path)?;
            println!("重命名: {} -> {}", image.name, new_filename);
        }

        Ok(())
    }
} 