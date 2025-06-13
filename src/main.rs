use anyhow::Result;
use jj_mcp_server::*;
use mcp_sdk::server::Server;
use mcp_sdk::tools::Tools;
use mcp_sdk::transport::ServerStdioTransport;
use mcp_sdk::types::ServerCapabilities;
use serde_json::json;

fn create_tools() -> Tools {
    let mut tools = Tools::default();

    // Status tool
    tools.add_tool(JjTool {
        name: "status".to_string(),
        description: "Show the status of the working directory".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "repoPath": {
                    "type": "string",
                    "description": "Optional path to repo root"
                },
                "cwd": {
                    "type": "string",
                    "description": "Optional working directory"
                }
            }
        }),
    });

    // Rebase tool
    tools.add_tool(JjTool {
        name: "rebase".to_string(),
        description: "Rebase a revision onto another".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "source": {
                    "type": "string",
                    "description": "Source revision to rebase"
                },
                "destination": {
                    "type": "string",
                    "description": "Destination revision to rebase onto"
                },
                "repoPath": {
                    "type": "string",
                    "description": "Optional path to repo root"
                },
                "cwd": {
                    "type": "string",
                    "description": "Optional working directory"
                }
            }
        }),
    });

    // Commit tool
    tools.add_tool(JjTool {
        name: "commit".to_string(),
        description: "Create a new commit".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "Commit message"
                },
                "repoPath": {
                    "type": "string",
                    "description": "Optional path to repo root"
                },
                "cwd": {
                    "type": "string",
                    "description": "Optional working directory"
                }
            }
        }),
    });

    // New tool
    tools.add_tool(JjTool {
        name: "new".to_string(),
        description: "Create a new empty commit".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "parents": {
                    "type": "string",
                    "description": "Parent revisions for the new commit"
                },
                "repoPath": {
                    "type": "string",
                    "description": "Optional path to repo root"
                },
                "cwd": {
                    "type": "string",
                    "description": "Optional working directory"
                }
            }
        }),
    });

    // Log tool
    tools.add_tool(JjTool {
        name: "log".to_string(),
        description: "Show commit history".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "limit": {
                    "type": "number",
                    "description": "Maximum number of commits to show"
                },
                "template": {
                    "type": "string",
                    "description": "Template for formatting output"
                },
                "revisions": {
                    "type": "string",
                    "description": "Revisions to show"
                },
                "repoPath": {
                    "type": "string",
                    "description": "Optional path to repo root"
                },
                "cwd": {
                    "type": "string",
                    "description": "Optional working directory"
                }
            }
        }),
    });

    // Diff tool
    tools.add_tool(JjTool {
        name: "diff".to_string(),
        description: "Show differences between revisions".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "from": {
                    "type": "string",
                    "description": "Source revision"
                },
                "to": {
                    "type": "string",
                    "description": "Target revision"
                },
                "paths": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Specific paths to diff"
                },
                "context": {
                    "type": "number",
                    "description": "Number of context lines"
                },
                "summary": {
                    "type": "boolean",
                    "description": "Show summary only"
                },
                "stat": {
                    "type": "boolean",
                    "description": "Show file statistics"
                },
                "repoPath": {
                    "type": "string",
                    "description": "Optional path to repo root"
                },
                "cwd": {
                    "type": "string",
                    "description": "Optional working directory"
                }
            }
        }),
    });

    // Git clone tool
    tools.add_tool(JjTool {
        name: "git-clone".to_string(),
        description: "Clone a Git repository using jj".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "source": {
                    "type": "string",
                    "description": "Git repository URL to clone"
                },
                "destination": {
                    "type": "string",
                    "description": "Destination directory"
                },
                "colocate": {
                    "type": "boolean",
                    "description": "Create a colocated jj/git repository"
                },
                "remote": {
                    "type": "string",
                    "description": "Name for the remote"
                },
                "depth": {
                    "type": "number",
                    "description": "Depth for shallow clone"
                }
            }
        }),
    });

    tools
}

#[tokio::main]
async fn main() -> Result<()> {
    let transport = ServerStdioTransport::default();
    let tools = create_tools();

    let server = Server::builder(transport)
        .name("jj-mcp-server")
        .version("1.0.0")
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            prompts: None,
            resources: None,
            logging: None,
            experimental: None,
        })
        .tools(tools)
        .build();

    eprintln!("jj MCP Server starting...");
    server.listen().await?;
    Ok(())
}
