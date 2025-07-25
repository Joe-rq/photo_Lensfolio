import { useState } from "react";
import "./Sidebar.css";

interface SidebarProps {
  onSelectFolder: () => void;
  onFilterTags: (tags: string[]) => void;
  onFilterRating: (rating: number) => void;
  onAddTag: (tag: string) => void;
  onSetRating: (rating: number) => void;
  onRenameWithDate: () => void;
  selectedCount: number;
  filterTags: string[];
  filterRating: number;
}

function Sidebar({
  onSelectFolder,
  onFilterTags,
  onFilterRating,
  onAddTag,
  onSetRating,
  onRenameWithDate,
  selectedCount,
  filterTags,
  filterRating
}: SidebarProps) {
  const [newTag, setNewTag] = useState("");
  const [newRating, setNewRating] = useState(0);

  const handleAddTag = () => {
    if (newTag.trim() && selectedCount > 0) {
      onAddTag(newTag.trim());
      setNewTag("");
    }
  };

  const handleSetRating = (rating: number) => {
    if (selectedCount > 0) {
      onSetRating(rating);
      setNewRating(rating);
    }
  };

  const handleFilterTagToggle = (tag: string) => {
    const newFilterTags = filterTags.includes(tag)
      ? filterTags.filter(t => t !== tag)
      : [...filterTags, tag];
    onFilterTags(newFilterTags);
  };

  const commonTags = ["风景", "人像", "街拍", "夜景", "动物", "建筑", "美食", "旅行"];

  return (
    <div className="sidebar">
      <div className="sidebar-section">
        <h3>图库管理</h3>
        <button 
          className="primary-button"
          onClick={onSelectFolder}
        >
          选择图片文件夹
        </button>
      </div>

      <div className="sidebar-section">
        <h3>筛选</h3>
        
        <div className="filter-group">
          <label>评分筛选</label>
          <div className="rating-filter">
            {[0, 1, 2, 3, 4, 5].map(rating => (
              <button
                key={rating}
                className={`rating-button ${filterRating === rating ? 'active' : ''}`}
                onClick={() => onFilterRating(rating)}
              >
                {rating === 0 ? '全部' : `${rating}★`}
              </button>
            ))}
          </div>
        </div>

        <div className="filter-group">
          <label>标签筛选</label>
          <div className="tag-filters">
            {commonTags.map(tag => (
              <button
                key={tag}
                className={`tag-filter ${filterTags.includes(tag) ? 'active' : ''}`}
                onClick={() => handleFilterTagToggle(tag)}
              >
                {tag}
              </button>
            ))}
          </div>
          {filterTags.length > 0 && (
            <button 
              className="clear-filters"
              onClick={() => onFilterTags([])}
            >
              清除筛选
            </button>
          )}
        </div>
      </div>

      <div className="sidebar-section">
        <h3>批量操作</h3>
        <div className="selection-info">
          已选择 {selectedCount} 张图片
        </div>

        {selectedCount > 0 && (
          <>
            <div className="operation-group">
              <label>添加标签</label>
              <div className="tag-input">
                <input
                  type="text"
                  value={newTag}
                  onChange={(e) => setNewTag(e.target.value)}
                  placeholder="输入标签"
                  onKeyPress={(e) => e.key === 'Enter' && handleAddTag()}
                />
                <button onClick={handleAddTag}>添加</button>
              </div>
              <div className="quick-tags">
                {commonTags.map(tag => (
                  <button
                    key={tag}
                    className="quick-tag"
                    onClick={() => onAddTag(tag)}
                  >
                    {tag}
                  </button>
                ))}
              </div>
            </div>

            <div className="operation-group">
              <label>设置评分</label>
              <div className="rating-buttons">
                {[1, 2, 3, 4, 5].map(rating => (
                  <button
                    key={rating}
                    className={`rating-button ${newRating === rating ? 'active' : ''}`}
                    onClick={() => handleSetRating(rating)}
                  >
                    {rating}★
                  </button>
                ))}
              </div>
            </div>

            <div className="operation-group">
              <label>文件操作</label>
              <button 
                className="action-button"
                onClick={onRenameWithDate}
              >
                按日期重命名
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
}

export default Sidebar; 