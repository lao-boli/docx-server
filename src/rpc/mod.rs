pub mod server;

use tonic::transport::Server;
use crate::docx_service_server;
use self::server::DocxService;

pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
     // 创建一个新的 gRPC 服务器实例
    let service = DocxService::new();

    // 启动服务器并绑定到指定地址
    Server::builder()
        .add_service(docx_service_server::DocxServiceServer::new(service))
        .serve(addr.parse()?)
        .await?;
    Ok(())
}
