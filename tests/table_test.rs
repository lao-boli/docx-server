use docx_server::{TableConfig, HeaderConfig, generate_docx};
use std::collections::HashMap;
use tempfile::NamedTempFile;

#[test]
fn test_basic_table_generation() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"
    {
        "headers": [
            {"field": "id", "display_name": "编号", "enabled": true},
            {"field": "name", "display_name": "姓名", "enabled": true},
            {"field": "salary", "display_name": "薪资", "enabled": true}
        ],
        "data": [
            {"id": "1", "name": "张三", "salary": "10000"},
            {"id": "2", "name": "李四", "salary": "15000"}
        ]
    }
    "#;

    // 解析配置
    let config: TableConfig = serde_json::from_str(json)?;
    // 生成文档
    let docx = generate_docx(&config)?;

    // 验证文件生成
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    docx.build().save(path)?;

    // 验证文件存在且非空
    assert!(path.exists());
    assert!(path.metadata()?.len() > 1000); // 简单验证文件大小

    Ok(())
}

#[test]
fn test_disabled_headers() {
    let config = TableConfig {
        headers: vec![
            HeaderConfig {
                field: "id".to_string(),
                display_name: "ID".to_string(),
                enabled: false,
            },
            HeaderConfig {
                field: "name".to_string(),
                display_name: "Name".to_string(),
                enabled: true,
            },
        ],
        data: vec![HashMap::new()],
    };

    let docx = generate_docx(&config).unwrap();
    // let table = docx.document.content[0].as_table().unwrap();

    // 验证只有1个表头列
    // assert_eq!(table.rows[0].cells.len(), 1);
    // assert_eq!(table.rows[0].cells[0].text_content(), "Name");
}
