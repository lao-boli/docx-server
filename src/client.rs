// use tonic::transport::Channel;
// use crate::core::{HeaderConfig, TableConfig};
// use crate::docx_generator_client::DocxGeneratorClient;
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 连接到 gRPC 服务器
//     let channel = Channel::from_static("http://[::1]:50051").connect().await?;
//     let mut client = DocxGeneratorClient::new(channel);
//     let json = r#"
//     {
//         "headers": [
//             {"field": "id", "display_name": "编号", "enabled": true},
//             {"field": "name", "display_name": "姓名", "enabled": true},
//             {"field": "salary", "display_name": "薪资", "enabled": true}
//         ],
//         "data": [
//             {"id": "1", "name": "张三", "salary": "10000"},
//             {"id": "2", "name": "李四", "salary": "15000"}
//         ]
//     }
//     "#;
//
//     // 解析配置
//     let config: TableConfig = serde_json::from_str(json)?;
//
//     // 创建 TableConfig 请求
//     let request = tonic::IntoRequest::new(config);
//     //  let request = <dyn tonic::IntoRequest<TableConfig>>::new(config);
//
//     // 调用 gRPC 服务
//     let response = client.generate_table(request).await?;
//
//     // 获取响应
//     let docx_response = response.into_inner();
//
//     // 打印文件名
//     println!("Generated file name: {}", docx_response.file_name);
//
//     // 保存 DOCX 文件到本地
//     std::fs::write(&docx_response.file_name, &docx_response.docx_content)?;
//
//     Ok(())
// }
