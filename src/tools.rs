use rmcp::{
    ErrorData as McpError, ServerHandler, handler::server::router::tool::ToolRouter, model::*,
    schemars, tool, tool_handler, tool_router,
};
use std::process::Command;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct StatusParams {
    /// Optional path to repo root
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    /// Optional working directory
    pub cwd: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LogParams {
    /// Maximum number of commits to show
    pub limit: Option<u32>,
    /// Template for formatting output
    pub template: Option<String>,
    /// Revisions to show
    pub revisions: Option<String>,
    /// Optional path to repo root
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    /// Optional working directory
    pub cwd: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DiffParams {
    /// Source revision
    pub from: Option<String>,
    /// Target revision
    pub to: Option<String>,
    /// Number of context lines
    pub context: Option<u32>,
    /// Show summary only
    pub summary: Option<bool>,
    /// Show file statistics
    pub stat: Option<bool>,
    /// Optional path to repo root
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    /// Optional working directory
    pub cwd: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DescribeParams {
    /// The description/commit message to set
    pub message: String,
    /// Revision to describe (defaults to @, the current working commit)
    pub revision: Option<String>,
    /// Optional path to repo root
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    /// Optional working directory
    pub cwd: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BookmarkCreateParams {
    /// Name of the bookmark to create
    pub name: String,
    /// Revision to point the bookmark at (defaults to @)
    pub revision: Option<String>,
    /// Optional path to repo root
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    /// Optional working directory
    pub cwd: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PushParams {
    /// Name of the bookmark to push
    pub bookmark: String,
    /// Optional path to repo root
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    /// Optional working directory
    pub cwd: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SyncParams {
    /// Optional path to repo root
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    /// Optional working directory
    pub cwd: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct NewParams {
    /// Parent revisions for the new commit (space-separated)
    pub parents: Option<String>,
    /// Optional path to repo root
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    /// Optional working directory
    pub cwd: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RebaseParams {
    /// Source revision to rebase
    pub source: String,
    /// Destination revision to rebase onto
    pub destination: String,
    /// Optional path to repo root
    #[serde(rename = "repoPath")]
    pub repo_path: Option<String>,
    /// Optional working directory
    pub cwd: Option<String>,
}

#[derive(Clone)]
pub struct JjService {
    tool_router: ToolRouter<JjService>,
}

#[tool_router]
impl JjService {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Show the status of the working directory in jujutsu, including changed files and current revision"
    )]
    fn status(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<StatusParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut cmd = Command::new("jj");
        cmd.arg("status");
        if let Some(ref repo_path) = params.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = params.cwd {
            cmd.current_dir(cwd);
        }
        run_command(cmd)
    }

    #[tool(description = "Show commit history with jujutsu's revision graph")]
    fn log(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<LogParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut cmd = Command::new("jj");
        cmd.arg("log");
        if let Some(limit) = params.limit {
            cmd.arg("-n").arg(limit.to_string());
        }
        if let Some(ref template) = params.template {
            cmd.arg("-T").arg(template);
        }
        if let Some(ref revisions) = params.revisions {
            cmd.arg("-r").arg(revisions);
        }
        if let Some(ref repo_path) = params.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = params.cwd {
            cmd.current_dir(cwd);
        }
        run_command(cmd)
    }

    #[tool(description = "Show differences between revisions or the working directory")]
    fn diff(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<DiffParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut cmd = Command::new("jj");
        cmd.arg("diff");
        if let Some(ref from) = params.from {
            if let Some(ref to) = params.to {
                cmd.arg("-r").arg(format!("{}..{}", from, to));
            } else {
                cmd.arg("-r").arg(from);
            }
        } else if let Some(ref to) = params.to {
            cmd.arg("-r").arg(format!("..{}", to));
        }
        if let Some(context) = params.context {
            cmd.arg("-U").arg(context.to_string());
        }
        if params.summary == Some(true) {
            cmd.arg("--summary");
        }
        if params.stat == Some(true) {
            cmd.arg("--stat");
        }
        if let Some(ref repo_path) = params.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = params.cwd {
            cmd.current_dir(cwd);
        }
        run_command(cmd)
    }

    #[tool(
        description = "Set or update the description (commit message) of a revision. Does NOT create a new commit."
    )]
    fn describe(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<DescribeParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut cmd = Command::new("jj");
        cmd.arg("describe").arg("-m").arg(&params.message);
        if let Some(ref revision) = params.revision {
            cmd.arg("-r").arg(revision);
        }
        if let Some(ref repo_path) = params.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = params.cwd {
            cmd.current_dir(cwd);
        }
        run_command(cmd)
    }

    #[tool(description = "Create a bookmark (like a git branch) pointing to a revision")]
    fn bookmark_create(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<BookmarkCreateParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut cmd = Command::new("jj");
        cmd.arg("bookmark").arg("create").arg(&params.name);
        if let Some(ref revision) = params.revision {
            cmd.arg("-r").arg(revision);
        }
        if let Some(ref repo_path) = params.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = params.cwd {
            cmd.current_dir(cwd);
        }
        run_command(cmd)
    }

    #[tool(description = "Push a bookmark to the remote")]
    fn push(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<PushParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut cmd = Command::new("jj");
        cmd.arg("git")
            .arg("push")
            .arg("--bookmark")
            .arg(&params.bookmark);
        if let Some(ref repo_path) = params.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = params.cwd {
            cmd.current_dir(cwd);
        }
        run_command(cmd)
    }

    #[tool(description = "Fetch updates from all remotes")]
    fn sync(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<SyncParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut cmd = Command::new("jj");
        cmd.arg("git").arg("fetch").arg("--all-remotes");
        if let Some(ref repo_path) = params.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = params.cwd {
            cmd.current_dir(cwd);
        }
        run_command(cmd)
    }

    #[tool(
        description = "Create a new empty working directory commit, optionally with specified parents"
    )]
    fn jj_new(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<NewParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut cmd = Command::new("jj");
        cmd.arg("new");
        if let Some(ref parents) = params.parents {
            for parent in parents.split_whitespace() {
                cmd.arg(parent);
            }
        }
        if let Some(ref repo_path) = params.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = params.cwd {
            cmd.current_dir(cwd);
        }
        run_command(cmd)
    }

    #[tool(description = "Move a revision and its descendants to another location in the graph")]
    fn rebase(
        &self,
        rmcp::handler::server::wrapper::Parameters(params): rmcp::handler::server::wrapper::Parameters<RebaseParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut cmd = Command::new("jj");
        cmd.arg("rebase")
            .arg("-s")
            .arg(&params.source)
            .arg("-d")
            .arg(&params.destination);
        if let Some(ref repo_path) = params.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = params.cwd {
            cmd.current_dir(cwd);
        }
        run_command(cmd)
    }
}

fn run_command(mut cmd: Command) -> Result<CallToolResult, McpError> {
    let output = cmd
        .output()
        .map_err(|e| McpError::internal_error(format!("Failed to run command: {}", e), None))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(CallToolResult::success(vec![Content::text(stdout)]))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(McpError::internal_error(stderr, None))
    }
}

#[tool_handler]
impl ServerHandler for JjService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "jj-mcp-server".into(),
                version: "1.0.0".into(),
                title: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(include_str!("instructions.md").to_string()),
        }
    }
}
