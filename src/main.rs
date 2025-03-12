use docx_rs::{Docx, Table, TableRow, TableCell, Paragraph, Run};
use serde_json::{Value, Map};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use docx_server::{generate_docx, TableConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 示例JSON配置
    let json = r#"
    {
        "headers": [
            {"field": "id", "display_name": "编号", "enabled": true},
            {"field": "name", "display_name": "姓名", "enabled": true},
            {"field": "salary", "display_name": "薪资", "enabled": true}
        ],
        "data": [
            {"id": "1", "name": "张三", "salary": "10000"},
            {"id": "1", "name": "张三", "salary": "10000"},
            {"id": "2", "name": "李四", "salary": "15000"}
        ]
    }
    "#;

    // 解析配置
    let config: TableConfig = serde_json::from_str(json)?;

    // 生成文档
    let docx = generate_docx(&config)?;
    let path = std::path::Path::new("./hello.docx");
    let file = std::fs::File::create(path).unwrap();
    docx.build().pack(file)?;
    Ok(())
}
