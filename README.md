# 光影管家 (Lensfolio) - MVP版本

> Copyright (c) 2025 Joe-rq
> Licensed under the MIT License

一个简单的本地图片管理工具，支持基础的图片浏览、标签管理和按日期重命名功能。

## 功能特点

✅ **已实现的功能**:
- 📁 文件夹扫描和图片导入
- 🖼️ 图片网格浏览
- 🏷️ 标签管理（添加、筛选）
- ⭐ 评分系统（1-5星）
- 📅 按日期重命名文件
- 🔍 基于标签和评分的筛选
- 💾 本地SQLite数据库存储

🚧 **计划中的功能**:
- 📸 EXIF信息读取和显示
- 🖼️ 缩略图生成和缓存
- 🎨 RAW格式支持
- 📱 响应式UI改进

## 技术栈

- **前端**: React + TypeScript + Vite
- **后端**: Rust + Tauri
- **数据库**: SQLite
- **图片处理**: image-rs

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Tauri CLI

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建应用

```bash
npm run tauri build
```

## 使用方法

1. **导入图片**: 点击侧边栏的"选择图片文件夹"按钮
2. **浏览图片**: 在网格视图中浏览，双击进入详细视图
3. **添加标签**: 选择图片后在侧边栏添加标签
4. **设置评分**: 选择图片后设置1-5星评分
5. **筛选图片**: 使用侧边栏的筛选功能
6. **重命名**: 选择图片后点击"按日期重命名"

## 支持格式

- JPG/JPEG
- PNG
- GIF
- BMP
- TIFF
- WebP

## 项目结构

```
├── src/                 # React前端代码
│   ├── components/      # React组件
│   └── App.tsx         # 主应用组件
├── src-tauri/          # Tauri后端代码
│   ├── src/            # Rust源代码
│   └── Cargo.toml      # Rust依赖配置
└── package.json        # Node.js配置
```

## 数据存储

- 图片元数据存储在当前目录的 `lensfolio.db` SQLite数据库中
- 不修改原始图片文件，所有标签和评分信息都存储在数据库中
- 支持数据库备份和迁移

## 注意事项

- 这是MVP版本，功能有限
- 目前不支持EXIF信息读取
- 重命名功能会直接修改文件名，请谨慎使用
- 建议在测试前备份重要图片

## 开发计划

- [ ] 添加EXIF信息读取
- [ ] 实现缩略图生成
- [ ] 添加更多图片格式支持
- [ ] 改进UI/UX
- [ ] 添加图片编辑功能
- [ ] 支持视频文件

## 许可证

MIT License © 2025 Joe-rq 