use anyhow::Result;
use mcp_sdk::server::Server;
use mcp_sdk::tools::{Tool, Tools};
use mcp_sdk::transport::ServerStdioTransport;
use mcp_sdk::types::{CallToolResponse, ServerCapabilities, ToolResponseContent};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const JJ_COMMAND: &str = "jj";

#[derive(Debug, Serialize, Deserialize, Default)]
struct StatusParams {
    #[serde(rename = "repoPath")]
    repo_path: Option<String>,
    cwd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct RebaseParams {
    source: Option<String>,
    destination: Option<String>,
    #[serde(rename = "repoPath")]
    repo_path: Option<String>,
    cwd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct CommitParams {
    message: Option<String>,
    #[serde(rename = "repoPath")]
    repo_path: Option<String>,
    cwd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct NewParams {
    parents: Option<String>,
    #[serde(rename = "repoPath")]
    repo_path: Option<String>,
    cwd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct LogParams {
    #[serde(rename = "repoPath")]
    repo_path: Option<String>,
    cwd: Option<String>,
    limit: Option<u32>,
    template: Option<String>,
    revisions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct DiffParams {
    #[serde(rename = "repoPath")]
    repo_path: Option<String>,
    cwd: Option<String>,
    from: Option<String>,
    to: Option<String>,
    paths: Option<Vec<String>>,
    summary: Option<bool>,
    stat: Option<bool>,
    context: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct GitCloneParams {
    source: Option<String>,
    destination: Option<String>,
    colocate: Option<bool>,
    remote: Option<String>,
    depth: Option<u32>,
}

struct JjTool {
    name: String,
    description: String,
    input_schema: Value,
}

impl Tool for JjTool {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn input_schema(&self) -> Value {
        self.input_schema.clone()
    }

    fn call(&self, arguments: Option<Value>) -> Result<CallToolResponse> {
        let args = arguments.unwrap_or_default();

        match self.name.as_str() {
            "status" => {
                let params: StatusParams = serde_json::from_value(args).unwrap_or_default();
                Ok(run_jj_status(params))
            }
            "rebase" => {
                let params: RebaseParams = serde_json::from_value(args).unwrap_or_default();
                Ok(run_jj_rebase(params))
            }
            "commit" => {
                let params: CommitParams = serde_json::from_value(args).unwrap_or_default();
                Ok(run_jj_commit(params))
            }
            "new" => {
                let params: NewParams = serde_json::from_value(args).unwrap_or_default();
                Ok(run_jj_new(params))
            }
            "log" => {
                let params: LogParams = serde_json::from_value(args).unwrap_or_default();
                Ok(run_jj_log(params))
            }
            "diff" => {
                let params: DiffParams = serde_json::from_value(args).unwrap_or_default();
                Ok(run_jj_diff(params))
            }
            "git-clone" => {
                let params: GitCloneParams = serde_json::from_value(args).unwrap_or_default();
                Ok(run_jj_git_clone(params))
            }
            _ => Ok(CallToolResponse {
                content: vec![ToolResponseContent::Text {
                    text: format!("Unknown tool: {}", self.name),
                }],
                is_error: Some(true),
                meta: None,
            }),
        }
    }
}

fn add_repo_args(args: &mut Vec<String>, repo_path: Option<String>) {
    if let Some(path) = repo_path {
        args.push("-R".to_string());
        args.push(path);
    }
}

fn run_jj_command_sync(args: Vec<String>, cwd: Option<String>) -> Result<String> {
    let mut cmd = std::process::Command::new(JJ_COMMAND);
    cmd.args(&args);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    if let Some(cwd_path) = cwd {
        cmd.current_dir(cwd_path);
    }

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stderr_trimmed = stderr.trim();
                Err(anyhow::anyhow!("Error: {}", stderr_trimmed))
            }
        }
        Err(e) => Err(anyhow::anyhow!("Error: {}", e)),
    }
}

fn run_jj_status(params: StatusParams) -> CallToolResponse {
    let mut args = vec!["status".to_string()];
    add_repo_args(&mut args, params.repo_path);

    match run_jj_command_sync(args, params.cwd) {
        Ok(output) => CallToolResponse {
            content: vec![ToolResponseContent::Text { text: output }],
            is_error: Some(false),
            meta: None,
        },
        Err(e) => CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: e.to_string(),
            }],
            is_error: Some(true),
            meta: None,
        },
    }
}

fn run_jj_rebase(params: RebaseParams) -> CallToolResponse {
    let mut args = vec!["rebase".to_string()];

    if let Some(source) = params.source {
        args.push("-s".to_string());
        args.push(source);
    }

    if let Some(destination) = params.destination {
        args.push("-d".to_string());
        args.push(destination);
    }

    add_repo_args(&mut args, params.repo_path);

    match run_jj_command_sync(args, params.cwd) {
        Ok(output) => CallToolResponse {
            content: vec![ToolResponseContent::Text { text: output }],
            is_error: Some(false),
            meta: None,
        },
        Err(e) => CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: e.to_string(),
            }],
            is_error: Some(true),
            meta: None,
        },
    }
}

fn run_jj_commit(params: CommitParams) -> CallToolResponse {
    let mut args = vec!["commit".to_string()];

    if let Some(message) = params.message {
        args.push("-m".to_string());
        args.push(message);
    }

    add_repo_args(&mut args, params.repo_path);

    match run_jj_command_sync(args, params.cwd) {
        Ok(output) => CallToolResponse {
            content: vec![ToolResponseContent::Text { text: output }],
            is_error: Some(false),
            meta: None,
        },
        Err(e) => CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: e.to_string(),
            }],
            is_error: Some(true),
            meta: None,
        },
    }
}

fn run_jj_new(params: NewParams) -> CallToolResponse {
    let mut args = vec!["new".to_string()];

    if let Some(parents) = params.parents {
        args.push(parents);
    }

    add_repo_args(&mut args, params.repo_path);

    match run_jj_command_sync(args, params.cwd) {
        Ok(output) => CallToolResponse {
            content: vec![ToolResponseContent::Text { text: output }],
            is_error: Some(false),
            meta: None,
        },
        Err(e) => CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: e.to_string(),
            }],
            is_error: Some(true),
            meta: None,
        },
    }
}

fn run_jj_log(params: LogParams) -> CallToolResponse {
    let mut args = vec!["log".to_string()];

    if let Some(limit) = params.limit {
        args.push("-n".to_string());
        args.push(limit.to_string());
    }

    if let Some(template) = params.template {
        args.push("-T".to_string());
        args.push(template);
    }

    if let Some(revisions) = params.revisions {
        args.push(revisions);
    }

    add_repo_args(&mut args, params.repo_path);

    match run_jj_command_sync(args, params.cwd) {
        Ok(output) => CallToolResponse {
            content: vec![ToolResponseContent::Text { text: output }],
            is_error: Some(false),
            meta: None,
        },
        Err(e) => CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: e.to_string(),
            }],
            is_error: Some(true),
            meta: None,
        },
    }
}

fn run_jj_diff(params: DiffParams) -> CallToolResponse {
    let mut args = vec!["diff".to_string()];

    if let Some(from) = params.from {
        args.push("--from".to_string());
        args.push(from);
    }

    if let Some(to) = params.to {
        args.push("--to".to_string());
        args.push(to);
    }

    if let Some(context) = params.context {
        args.push("--context".to_string());
        args.push(context.to_string());
    }

    if let Some(true) = params.summary {
        args.push("--summary".to_string());
    }

    if let Some(true) = params.stat {
        args.push("--stat".to_string());
    }

    if let Some(paths) = params.paths {
        args.extend(paths);
    }

    add_repo_args(&mut args, params.repo_path);

    match run_jj_command_sync(args, params.cwd) {
        Ok(output) => CallToolResponse {
            content: vec![ToolResponseContent::Text { text: output }],
            is_error: Some(false),
            meta: None,
        },
        Err(e) => CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: e.to_string(),
            }],
            is_error: Some(true),
            meta: None,
        },
    }
}

fn run_jj_git_clone(params: GitCloneParams) -> CallToolResponse {
    let mut args = vec!["git".to_string(), "clone".to_string()];

    if let Some(source) = params.source {
        args.push(source);
    }

    if let Some(destination) = params.destination {
        args.push(destination);
    }

    if let Some(true) = params.colocate {
        args.push("--colocate".to_string());
    }

    if let Some(remote) = params.remote {
        args.push("--remote".to_string());
        args.push(remote);
    }

    if let Some(depth) = params.depth {
        args.push("--depth".to_string());
        args.push(depth.to_string());
    }

    match run_jj_command_sync(args, None) {
        Ok(output) => CallToolResponse {
            content: vec![ToolResponseContent::Text { text: output }],
            is_error: Some(false),
            meta: None,
        },
        Err(e) => CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: e.to_string(),
            }],
            is_error: Some(true),
            meta: None,
        },
    }
}

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
