use docx_rs::{Docx, Table, TableRow, TableCell, Paragraph, Run};
use serde_json::{Value, Map};
use std::collections::HashMap;

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
    let json_data = r#"
        [
            {"user_name": "张三", "dept": "技术部", "join_date": "2020-03-12", "salary": "****"},
            {"user_name": "李四", "dept": "市场部", "join_date": "2021-07-01"}
        ]
    "#;

    let mut header_map = HashMap::new();
    header_map.insert("user_name", "员工姓名");
    header_map.insert("dept", "部门");

    let allowed_fields = vec!["user_name", "dept", "join_date"];

    let docx = generate_table_from_json(json_data, &header_map, &allowed_fields)?;
    let path = std::path::Path::new("./hello.docx");
    let file = std::fs::File::create(path).unwrap();
    docx.build().pack(file)?;
    Ok(())
}
