use tonic::transport::Channel;
use docx_server::docx_service_client::DocxServiceClient;
use docx_server::{ProtoHeaderConfig, RowData, TableConfig, TableConfigReq};

#[tokio::test]
async fn test_grpc_table_generation() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = DocxServiceClient::new(channel);
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

    // 创建 TableConfig 请求
    let request = tonic::Request::new(config);

    // 调用 gRPC 服务
    let response = client.generate_table(TableConfigReq{
        headers: vec![
            ProtoHeaderConfig {
                field: "id".to_string(),
                display_name: "编号".to_string(),
                enabled: true,
            },
            ProtoHeaderConfig {
                field: "name".to_string(),
                display_name: "姓名".to_string(),
                enabled: true,
            },
            ProtoHeaderConfig {
                field: "salary".to_string(),
                display_name: "薪资".to_string(),
                enabled: true,
            },
        ],
        data: vec![
           RowData {
               cells: vec![
                   ("id".to_string(), "1".to_string()),
                   ("name".to_string(), "张三".to_string()),
                   ("salary".to_string(), "10000".to_string()),
               ].into_iter().collect(),
           },
           RowData {
               cells: vec![
                   ("id".to_string(), "2".to_string()),
                   ("name".to_string(), "李四".to_string()),
                   ("salary".to_string(), "15000".to_string()),
               ].into_iter().collect(),
           },
        ],
    }).await?;

    // 获取响应
    let docx_response = response.into_inner();

    // 打印文件名
    println!("Generated file name: {}", docx_response.file_name);

    // 定义保存文件的目录
let save_directory = "output";

// 确保目录存在，如果不存在则创建
std::fs::create_dir_all(save_directory)?;

// 构建完整的文件路径
let file_path = format!("{}/{}", save_directory, docx_response.file_name);


    // 保存 DOCX 文件到本地
    std::fs::write(&file_path, &docx_response.docx_content)?;


    Ok(())
}
