import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import ImageGrid from "./components/ImageGrid";
import Sidebar from "./components/Sidebar";
import "./App.css";

interface ImageFile {
  id: string;
  path: string;
  name: string;
  size: number;
  dateCreated: string;
  dateTaken?: string;
  tags: string[];
  rating: number;
  thumbnail?: string;
}

function App() {
  const [images, setImages] = useState<ImageFile[]>([]);
  const [selectedImages, setSelectedImages] = useState<string[]>([]);
  const [filterTags, setFilterTags] = useState<string[]>([]);
  const [filterRating, setFilterRating] = useState<number>(0);

  useEffect(() => {
    // 初始化数据库
    invoke("init_database");
  }, []);

  const handleSelectFolder = async () => {
    try {
      // 使用Tauri的对话框API选择文件夹
      const { open } = await import("@tauri-apps/api/dialog");
      const folderPath = await open({
        directory: true,
        title: "选择图片文件夹",
      });
      
      if (folderPath && typeof folderPath === "string") {
        const result = await invoke<ImageFile[]>("scan_folder", { 
          folder_path: folderPath 
        });
        setImages(result);
      }
    } catch (error) {
      console.error("选择文件夹失败:", error);
    }
  };

  const handleAddTag = async (imageIds: string[], tag: string) => {
    try {
      await invoke("add_tag", { imageIds, tag });
      // 重新加载图片列表
      const result = await invoke<ImageFile[]>("get_images");
      setImages(result);
    } catch (error) {
      console.error("添加标签失败:", error);
    }
  };

  const handleSetRating = async (imageIds: string[], rating: number) => {
    try {
      await invoke("set_rating", { imageIds, rating });
      // 重新加载图片列表
      const result = await invoke<ImageFile[]>("get_images");
      setImages(result);
    } catch (error) {
      console.error("设置评分失败:", error);
    }
  };

  const handleRenameWithDate = async (imageIds: string[]) => {
    try {
      await invoke("rename_with_date", { imageIds });
      // 重新加载图片列表
      const result = await invoke<ImageFile[]>("get_images");
      setImages(result);
    } catch (error) {
      console.error("重命名失败:", error);
    }
  };

  const filteredImages = images.filter(image => {
    // 评分筛选
    if (filterRating > 0 && image.rating < filterRating) {
      return false;
    }
    
    // 标签筛选
    if (filterTags.length > 0) {
      return filterTags.every(tag => image.tags.includes(tag));
    }
    
    return true;
  });

  return (
    <div className="app">
      <Sidebar
        onSelectFolder={handleSelectFolder}
        onFilterTags={setFilterTags}
        onFilterRating={setFilterRating}
        onAddTag={(tag) => selectedImages.length > 0 && handleAddTag(selectedImages, tag)}
        onSetRating={(rating) => selectedImages.length > 0 && handleSetRating(selectedImages, rating)}
        onRenameWithDate={() => selectedImages.length > 0 && handleRenameWithDate(selectedImages)}
        selectedCount={selectedImages.length}
        filterTags={filterTags}
        filterRating={filterRating}
      />
      <main className="main-content">
        <ImageGrid
          images={filteredImages}
          selectedImages={selectedImages}
          onSelectionChange={setSelectedImages}
        />
      </main>
    </div>
  );
}

export default App; 