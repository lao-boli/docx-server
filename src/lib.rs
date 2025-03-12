mod core;
mod rpc;
mod pb;
mod client;

pub use core::*;
pub use rpc::*;
pub use pb::*;

use docx_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct TableConfig {
//     pub headers: Vec<HeaderConfig>,
//     pub data: Vec<HashMap<String, String>>,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct HeaderConfig {
//     pub field: String,
//     pub display_name: String,
//     #[serde(default = "default_enabled")]
//     pub enabled: bool,
// }
//
// fn default_enabled() -> bool {
//     true
// }
//
// pub fn generate_docx(config: &TableConfig) -> Result<Docx, Box<dyn std::error::Error>> {
//     let mut docx = Docx::default();
//     let enabled_headers = get_enabled_headers(config);
//
//     let header_row = build_header_row(&enabled_headers);
//     let mut table = Table::new(vec![header_row]);
//
//     for data_item in &config.data {
//        table =  table.add_row(build_data_row(data_item, &enabled_headers));
//     }
//
//     docx = docx.add_table(table);
//     Ok(docx)
// }
//
// // 内部工具函数（可测试）
// fn get_enabled_headers(config: &TableConfig) -> Vec<&HeaderConfig> {
//     config.headers.iter()
//         .filter(|h| h.enabled)
//         .collect()
// }
//
// fn build_header_row(headers: &[&HeaderConfig]) -> TableRow {
//     let cells = headers.iter()
//         .map(|h| TableCell::new().add_paragraph(para_with_text(&h.display_name)))
//         .collect();
//     TableRow::new(cells)
// }
//
// fn build_data_row(item: &HashMap<String, String>, headers: &[&HeaderConfig]) -> TableRow {
//     let cells = headers.iter()
//         .map(|h| {
//             let text = item.get(&h.field).map(|s| s.as_str()).unwrap_or("");
//             TableCell::new().add_paragraph(para_with_text(text))
//         })
//         .collect();
//     TableRow::new(cells)
// }
//
// fn para_with_text(text: &str) -> Paragraph {
//     Paragraph::new().add_run(Run::new().add_text(text))
// }
