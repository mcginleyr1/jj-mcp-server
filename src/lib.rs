//! jj MCP Server Library
//!
//! A Model Context Protocol (MCP) server that provides tools for interacting with
//! Jujutsu (jj) version control system.

use anyhow::Result;
use mcp_sdk::tools::Tool;
pub use mcp_sdk::types::{CallToolResponse, ServerCapabilities, ToolResponseContent};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const JJ_COMMAND: &str = "jj";

/// Parameters for the status tool
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StatusParams {
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    pub cwd: Option<String>,
}

/// Parameters for the rebase tool
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RebaseParams {
    pub source: Option<String>,
    pub destination: Option<String>,
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    pub cwd: Option<String>,
}

/// Parameters for the commit tool
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CommitParams {
    pub message: Option<String>,
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    pub cwd: Option<String>,
}

/// Parameters for the new tool
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NewParams {
    pub parents: Option<String>,
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    pub cwd: Option<String>,
}

/// Parameters for the log tool
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LogParams {
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    pub cwd: Option<String>,
    pub limit: Option<u32>,
    pub template: Option<String>,
    pub revisions: Option<String>,
}

/// Parameters for the diff tool
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DiffParams {
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    pub cwd: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub paths: Option<Vec<String>>,
    pub summary: Option<bool>,
    pub stat: Option<bool>,
    pub context: Option<u32>,
}

/// Parameters for the git-clone tool
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GitCloneParams {
    pub source: Option<String>,
    pub destination: Option<String>,
    pub colocate: Option<bool>,
    pub remote: Option<String>,
    pub depth: Option<u32>,
}

/// A jj tool that implements the MCP Tool trait
pub struct JjTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
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

/// Add repository arguments to a command
pub fn add_repo_args(args: &mut Vec<String>, repo_path: Option<String>) {
    if let Some(path) = repo_path {
        args.push("-R".to_string());
        args.push(path);
    }
}

/// Run a jj command synchronously
pub fn run_jj_command_sync(args: Vec<String>, cwd: Option<String>) -> Result<String> {
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

/// Execute jj status command
pub fn run_jj_status(params: StatusParams) -> CallToolResponse {
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

/// Execute jj rebase command
pub fn run_jj_rebase(params: RebaseParams) -> CallToolResponse {
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

/// Execute jj commit command
pub fn run_jj_commit(params: CommitParams) -> CallToolResponse {
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

/// Execute jj new command
pub fn run_jj_new(params: NewParams) -> CallToolResponse {
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

/// Execute jj log command
pub fn run_jj_log(params: LogParams) -> CallToolResponse {
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

/// Execute jj diff command
pub fn run_jj_diff(params: DiffParams) -> CallToolResponse {
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

/// Execute jj git clone command
pub fn run_jj_git_clone(params: GitCloneParams) -> CallToolResponse {
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


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_jj_tool_creation() {
        let tool = JjTool {
            name: "test-tool".to_string(),
            description: "Test tool description".to_string(),
            input_schema: json!({"type": "object"}),
        };

        assert_eq!(tool.name(), "test-tool");
        assert_eq!(tool.description(), "Test tool description");
        assert_eq!(tool.input_schema(), json!({"type": "object"}));
    }

    #[test]
    fn test_add_repo_args() {
        let mut args = vec!["status".to_string()];
        add_repo_args(&mut args, Some("/path/to/repo".to_string()));

        assert_eq!(args, vec!["status", "-R", "/path/to/repo"]);
    }

    #[test]
    fn test_add_repo_args_none() {
        let mut args = vec!["status".to_string()];
        add_repo_args(&mut args, None);

        assert_eq!(args, vec!["status"]);
    }

    #[test]
    fn test_status_params_default() {
        let params = StatusParams::default();
        assert!(params.repo_path.is_none());
        assert!(params.cwd.is_none());
    }

    #[test]
    fn test_status_params_deserialization() {
        let json_val = json!({
            "repoPath": "/test/repo",
            "cwd": "/test/dir"
        });

        let params: StatusParams = serde_json::from_value(json_val).unwrap();
        assert_eq!(params.repo_path, Some("/test/repo".to_string()));
        assert_eq!(params.cwd, Some("/test/dir".to_string()));
    }

    #[test]
    fn test_rebase_params_deserialization() {
        let json_val = json!({
            "source": "@",
            "destination": "main",
            "repoPath": "/test/repo"
        });

        let params: RebaseParams = serde_json::from_value(json_val).unwrap();
        assert_eq!(params.source, Some("@".to_string()));
        assert_eq!(params.destination, Some("main".to_string()));
        assert_eq!(params.repo_path, Some("/test/repo".to_string()));
    }

    #[test]
    fn test_tool_call_unknown_tool() {
        let tool = JjTool {
            name: "unknown".to_string(),
            description: "Unknown tool".to_string(),
            input_schema: json!({}),
        };

        let result = tool.call(None).unwrap();
        assert_eq!(result.is_error, Some(true));

        if let ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Unknown tool: unknown"));
        } else {
            panic!("Expected text content");
        }
    }

    #[test]
    fn test_run_jj_command_sync_invalid_command() {
        // Test with an invalid jj command
        let result = run_jj_command_sync(vec!["invalid-command".to_string()], None);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_response_format() {
        let params = StatusParams {
            repo_path: Some("/nonexistent/path".to_string()),
            cwd: None,
        };

        let result = run_jj_status(params);
        assert_eq!(result.is_error, Some(true));
        assert_eq!(result.content.len(), 1);

        if let ToolResponseContent::Text { text } = &result.content[0] {
            assert!(text.contains("Error:"));
        } else {
            panic!("Expected text content");
        }
    }
}
