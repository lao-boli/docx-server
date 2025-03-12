use std::io::Cursor;
use tonic::{Request, Response, Status};
use crate::core::{TableConfig, HeaderConfig, generate_docx};
use crate::pb::{TableConfigReq as ProtoTableConfig, DocxResponse};

pub struct DocxService;

impl DocxService {
    pub fn new() -> Self {
        Self
    }

    fn convert_config(&self, proto: ProtoTableConfig) -> Result<TableConfig, Status> {
        let headers = proto.headers.into_iter().map(|h| {
        HeaderConfig {
            field: h.field,
            display_name: h.display_name,
            enabled: h.enabled,
        }
    }).collect();

    let data = proto.data.into_iter().map(|item| {
        item.cells.into_iter()
            .map(|(k, v)| (k, v))
            .collect()
    }).collect();

    Ok(TableConfig { headers, data })
    }
}

#[tonic::async_trait]
impl crate::pb::docx_service_server::DocxService for DocxService {
    async fn generate_table(
        &self,
        request: Request<ProtoTableConfig>,
    ) -> Result<Response<DocxResponse>, Status> {
        // 处理逻辑保持不变
        // ...
        // 5. 转换 Protobuf 请求到领域模型
        let proto_config = request.into_inner();
        let config = self.convert_config(proto_config)?;

        // 6. 调用核心业务逻辑
        let docx = generate_docx(&config)
            .map_err(|e| Status::internal(e.to_string()))?;

        // 7. 序列化 DOCX 文件
        let mut buf = Cursor::new(Vec::new());
        docx.build().pack(&mut buf)
            .map_err(|e| Status::internal(e.to_string()))?;

        // 8. 返回响应
        Ok(Response::new(DocxResponse {
            docx_content: buf.into_inner(),
            file_name: format!("document_{}.docx", chrono::Utc::now().timestamp()),
        }))
    }
}
