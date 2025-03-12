use docx_rs::{Docx, Table, TableRow, TableCell, Paragraph, Run};
use serde_json::{Value, Map};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use docx_server::{generate_docx, run_server, TableConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 示例JSON配置
    let addr = "[::1]:50051"; // 例如，监听本地的 50051 端口
    run_server(addr).await?;
    Ok(())
}
