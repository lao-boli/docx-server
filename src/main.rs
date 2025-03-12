use docx_rs::{Docx, Table, TableRow, TableCell, Paragraph, Run};
use serde_json::{Value, Map};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TableConfig {
    headers: Vec<HeaderConfig>,
    data: Vec<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct HeaderConfig {
    field: String,
    display_name: String,
    #[serde(default = "default_enabled")]
    enabled: bool,
}

fn default_enabled() -> bool {
    true
}

// 表格生成主函数
fn generate_table(config: &TableConfig) -> Result<Docx, Box<dyn std::error::Error>> {
    let mut docx = Docx::default();

    // 过滤并排序启用的表头
    let enabled_headers: Vec<&HeaderConfig> = config.headers
        .iter()
        .filter(|h| h.enabled)
        .collect();

    // 构建表头行
    let header_cells: Vec<TableCell> = enabled_headers.iter()
        .map(|h| {
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&h.display_name)))
        })
        .collect();
    let header_row = TableRow::new(header_cells);

    // 构建数据行
    let mut table = Table::new(vec![header_row]);
    for item in &config.data {
        let data_cells: Vec<TableCell> = enabled_headers.iter()
            .map(|h| {
                let value = item.get(&h.field)
                    .map(|s| s.as_str())
                    .unwrap_or("");

                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text(value)))
            })
            .collect();

       table =  table.add_row(TableRow::new(data_cells));
    }

    docx = docx.add_table(table);
    Ok(docx)
}

fn generate_table_from_json(
    json_data: &str,
    header_map: &HashMap<&str, &str>,
    allowed_fields: &[&str]
) -> Result<Docx, Box<dyn std::error::Error>> {
    let mut docx = Docx::default();
    let data: Value = serde_json::from_str(json_data)?;
    let records = data.as_array().unwrap();

    // 构建表头行（正确方法）
    let header_cells: Vec<TableCell> = allowed_fields.iter()
        .map(|&field| {
            let header_name = header_map.get(field)
                .map(|s| *s) // 解包双重引用
                .unwrap_or(field);
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(header_name.to_string())))
        })
        .collect();

    let header_row = TableRow::new(header_cells); // ✅ 直接传入单元格集合

    // 构建表格
    let mut table = Table::new(vec![header_row]);

    // 添加数据行
    for record in records {
        let data_cells: Vec<TableCell> = allowed_fields.iter()
            .map(|field| {
                let cell_value = record.get(*field)
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text(cell_value)))
            })
            .collect();

       table =  table.add_row(TableRow::new(data_cells)); // ✅ 正确添加行
    }

    docx = docx.add_table(table);
    Ok(docx)
}
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
            {"id": "2", "name": "李四", "salary": "15000"}
        ]
    }
    "#;

    // 解析配置
    let config: TableConfig = serde_json::from_str(json)?;

    // 生成文档
    let docx = generate_table(&config)?;
    let path = std::path::Path::new("./hello.docx");
    let file = std::fs::File::create(path).unwrap();
    docx.build().pack(file)?;
    Ok(())
}
