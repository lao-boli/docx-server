//! DOCX生成核心逻辑模块
//!
//! 包含表格配置定义、文档生成主逻辑

mod generator;

// 公开导出接口
pub use generator::{generate_docx, TableConfig, HeaderConfig};

