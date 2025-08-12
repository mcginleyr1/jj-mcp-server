use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest,
    ListToolsResult, RpcError,
};
use rust_mcp_sdk::{mcp_server::ServerHandler, McpServer};

use crate::tools::JjTools;

pub struct JjServerHandler;

#[async_trait]
impl ServerHandler for JjServerHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: JjTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let tool_params: JjTools =
            JjTools::try_from(request.params).map_err(CallToolError::new)?;

        match tool_params {
            JjTools::StatusTool(tool) => tool.call_tool(),
            JjTools::LogTool(tool) => tool.call_tool(),
            JjTools::DiffTool(tool) => tool.call_tool(),
            JjTools::CommitTool(tool) => tool.call_tool(),
            JjTools::NewTool(tool) => tool.call_tool(),
            JjTools::RebaseTool(tool) => tool.call_tool(),
            JjTools::GitCloneTool(tool) => tool.call_tool(),
        }
    }
}