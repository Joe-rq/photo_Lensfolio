import { useState } from "react";
import "./ImageGrid.css";

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

interface ImageGridProps {
  images: ImageFile[];
  selectedImages: string[];
  onSelectionChange: (selectedIds: string[]) => void;
}

function ImageGrid({ images, selectedImages, onSelectionChange }: ImageGridProps) {
  const [viewMode, setViewMode] = useState<'grid' | 'detail'>('grid');
  const [selectedImage, setSelectedImage] = useState<ImageFile | null>(null);

  const handleImageClick = (image: ImageFile, event: React.MouseEvent) => {
    if (event.ctrlKey || event.metaKey) {
      // 多选模式
      const newSelection = selectedImages.includes(image.id)
        ? selectedImages.filter(id => id !== image.id)
        : [...selectedImages, image.id];
      onSelectionChange(newSelection);
    } else if (event.shiftKey && selectedImages.length > 0) {
      // 范围选择模式（简化版）
      const lastSelectedIndex = images.findIndex(img => img.id === selectedImages[selectedImages.length - 1]);
      const currentIndex = images.findIndex(img => img.id === image.id);
      const start = Math.min(lastSelectedIndex, currentIndex);
      const end = Math.max(lastSelectedIndex, currentIndex);
      const rangeIds = images.slice(start, end + 1).map(img => img.id);
      onSelectionChange([...new Set([...selectedImages, ...rangeIds])]);
    } else {
      // 单选模式
      onSelectionChange([image.id]);
    }
  };

  const handleImageDoubleClick = (image: ImageFile) => {
    setSelectedImage(image);
    setViewMode('detail');
  };

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const renderStars = (rating: number) => {
    return Array.from({ length: 5 }, (_, i) => (
      <span key={i} className={i < rating ? 'star filled' : 'star'}>
        ★
      </span>
    ));
  };

  if (viewMode === 'detail' && selectedImage) {
    return (
      <div className="image-detail">
        <div className="detail-header">
          <button onClick={() => setViewMode('grid')}>← 返回网格视图</button>
          <h2>{selectedImage.name}</h2>
        </div>
        <div className="detail-content">
          <div className="detail-image">
            <img 
              src={`file://${selectedImage.path}`} 
              alt={selectedImage.name}
              onError={(e) => {
                const target = e.target as HTMLImageElement;
                target.src = '/placeholder.jpg';
              }}
            />
          </div>
          <div className="detail-info">
            <div className="info-section">
              <h3>基本信息</h3>
              <p><strong>文件名:</strong> {selectedImage.name}</p>
              <p><strong>大小:</strong> {formatFileSize(selectedImage.size)}</p>
              <p><strong>创建时间:</strong> {selectedImage.dateCreated}</p>
              {selectedImage.dateTaken && (
                <p><strong>拍摄时间:</strong> {selectedImage.dateTaken}</p>
              )}
              <p><strong>评分:</strong> {renderStars(selectedImage.rating)}</p>
            </div>
            <div className="info-section">
              <h3>标签</h3>
              <div className="tags">
                {selectedImage.tags.map(tag => (
                  <span key={tag} className="tag">{tag}</span>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="image-grid">
      <div className="grid-header">
        <div className="view-controls">
          <span>共 {images.length} 张图片</span>
          {selectedImages.length > 0 && (
            <span>已选择 {selectedImages.length} 张</span>
          )}
        </div>
      </div>
      <div className="grid-container">
        {images.map(image => (
          <div
            key={image.id}
            className={`image-item ${selectedImages.includes(image.id) ? 'selected' : ''}`}
            onClick={(e) => handleImageClick(image, e)}
            onDoubleClick={() => handleImageDoubleClick(image)}
          >
            <div className="image-thumbnail">
              <img 
                src={image.thumbnail || `file://${image.path}`}
                alt={image.name}
                loading="lazy"
                onError={(e) => {
                  const target = e.target as HTMLImageElement;
                  target.src = '/placeholder.jpg';
                }}
              />
            </div>
            <div className="image-info">
              <div className="image-name" title={image.name}>
                {image.name}
              </div>
              <div className="image-rating">
                {renderStars(image.rating)}
              </div>
              {image.tags.length > 0 && (
                <div className="image-tags">
                  {image.tags.slice(0, 3).map(tag => (
                    <span key={tag} className="tag-mini">{tag}</span>
                  ))}
                  {image.tags.length > 3 && <span className="tag-mini">+{image.tags.length - 3}</span>}
                </div>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default ImageGrid; 