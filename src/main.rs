mod handler;
mod tools;

use handler::JjServerHandler;
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
    LATEST_PROTOCOL_VERSION,
};

use rust_mcp_sdk::{
    error::SdkResult,
    mcp_server::{server_runtime, ServerRuntime},
    McpServer, StdioTransport, TransportOptions,
};

#[tokio::main]
async fn main() -> SdkResult<()> {
    // Define server details and capabilities
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "jj MCP Server".to_string(),
            version: "1.0.0".to_string(),
            title: Some("Jujutsu MCP Server".to_string()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some("MCP server for Jujutsu (jj) version control system".to_string()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    // Create stdio transport
    let transport = StdioTransport::new(TransportOptions::default())?;

    // Instantiate our handler
    let handler = JjServerHandler {};

    // Create and start the MCP server
    let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);

    eprintln!("jj MCP Server starting...");
    
    if let Err(start_error) = server.start().await {
        eprintln!(
            "{}",
            start_error
                .rpc_error_message()
                .unwrap_or(&start_error.to_string())
        );
    };
    Ok(())
}