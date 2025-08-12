use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult, TextContent};
use rust_mcp_sdk::{
    macros::{mcp_tool, JsonSchema},
    tool_box,
};
use std::process::Command;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct ToolError(String);

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ToolError {}

#[mcp_tool(
    name = "status",
    description = "Show the status of the working directory"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct StatusTool {
    /// Optional path to repo root
    #[serde(rename = "repoPath", skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
    /// Optional working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
}

impl StatusTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("jj");
        cmd.arg("status");

        if let Some(ref repo_path) = self.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = self.cwd {
            cmd.current_dir(cwd);
        }

        let output = cmd.output().map_err(|e| CallToolError::new(e))?;
        
        let result = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            return Err(CallToolError::new(ToolError(String::from_utf8_lossy(&output.stderr).to_string())));
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(result)]))
    }
}

#[mcp_tool(
    name = "log",
    description = "Show commit history"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct LogTool {
    /// Maximum number of commits to show
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    /// Template for formatting output
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<String>,
    /// Revisions to show
    #[serde(skip_serializing_if = "Option::is_none")]
    revisions: Option<String>,
    /// Optional path to repo root
    #[serde(rename = "repoPath", skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
    /// Optional working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
}

impl LogTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("jj");
        cmd.arg("log");

        if let Some(limit) = self.limit {
            cmd.arg("-n").arg(limit.to_string());
        }
        if let Some(ref template) = self.template {
            cmd.arg("-T").arg(template);
        }
        if let Some(ref revisions) = self.revisions {
            cmd.arg("-r").arg(revisions);
        }
        if let Some(ref repo_path) = self.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = self.cwd {
            cmd.current_dir(cwd);
        }

        let output = cmd.output().map_err(|e| CallToolError::new(e))?;
        
        let result = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            return Err(CallToolError::new(ToolError(String::from_utf8_lossy(&output.stderr).to_string())));
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(result)]))
    }
}

#[mcp_tool(
    name = "diff",
    description = "Show differences between revisions"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct DiffTool {
    /// Source revision
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    /// Target revision
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
    /// Specific paths to diff
    #[serde(skip_serializing_if = "Option::is_none")]
    paths: Option<Vec<String>>,
    /// Number of context lines
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<u32>,
    /// Show summary only
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<bool>,
    /// Show file statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    stat: Option<bool>,
    /// Optional path to repo root
    #[serde(rename = "repoPath", skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
    /// Optional working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
}

impl DiffTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("jj");
        cmd.arg("diff");

        if let Some(ref from) = self.from {
            if let Some(ref to) = self.to {
                cmd.arg("-r").arg(format!("{}..{}", from, to));
            } else {
                cmd.arg("-r").arg(from);
            }
        } else if let Some(ref to) = self.to {
            cmd.arg("-r").arg(format!("..{}", to));
        }

        if let Some(context) = self.context {
            cmd.arg("-U").arg(context.to_string());
        }
        if let Some(true) = self.summary {
            cmd.arg("--summary");
        }
        if let Some(true) = self.stat {
            cmd.arg("--stat");
        }
        if let Some(ref repo_path) = self.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = self.cwd {
            cmd.current_dir(cwd);
        }
        if let Some(ref paths) = self.paths {
            for path in paths {
                cmd.arg(path);
            }
        }

        let output = cmd.output().map_err(|e| CallToolError::new(e))?;
        
        let result = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            return Err(CallToolError::new(ToolError(String::from_utf8_lossy(&output.stderr).to_string())));
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(result)]))
    }
}

#[mcp_tool(
    name = "commit",
    description = "Create a new commit"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct CommitTool {
    /// Commit message
    message: String,
    /// Optional path to repo root
    #[serde(rename = "repoPath", skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
    /// Optional working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
}

impl CommitTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("jj");
        cmd.arg("commit").arg("-m").arg(&self.message);

        if let Some(ref repo_path) = self.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = self.cwd {
            cmd.current_dir(cwd);
        }

        let output = cmd.output().map_err(|e| CallToolError::new(e))?;
        
        let result = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            return Err(CallToolError::new(ToolError(String::from_utf8_lossy(&output.stderr).to_string())));
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(result)]))
    }
}

#[mcp_tool(
    name = "new",
    description = "Create a new empty commit"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct NewTool {
    /// Parent revisions for the new commit
    #[serde(skip_serializing_if = "Option::is_none")]
    parents: Option<String>,
    /// Optional path to repo root
    #[serde(rename = "repoPath", skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
    /// Optional working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
}

impl NewTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("jj");
        cmd.arg("new");

        if let Some(ref parents) = self.parents {
            for parent in parents.split_whitespace() {
                cmd.arg(parent);
            }
        }
        if let Some(ref repo_path) = self.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = self.cwd {
            cmd.current_dir(cwd);
        }

        let output = cmd.output().map_err(|e| CallToolError::new(e))?;
        
        let result = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            return Err(CallToolError::new(ToolError(String::from_utf8_lossy(&output.stderr).to_string())));
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(result)]))
    }
}

#[mcp_tool(
    name = "rebase",
    description = "Rebase a revision onto another"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct RebaseTool {
    /// Source revision to rebase
    source: String,
    /// Destination revision to rebase onto
    destination: String,
    /// Optional path to repo root
    #[serde(rename = "repoPath", skip_serializing_if = "Option::is_none")]
    repo_path: Option<String>,
    /// Optional working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
}

impl RebaseTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("jj");
        cmd.arg("rebase")
            .arg("-s").arg(&self.source)
            .arg("-d").arg(&self.destination);

        if let Some(ref repo_path) = self.repo_path {
            cmd.arg("-R").arg(repo_path);
        }
        if let Some(ref cwd) = self.cwd {
            cmd.current_dir(cwd);
        }

        let output = cmd.output().map_err(|e| CallToolError::new(e))?;
        
        let result = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            return Err(CallToolError::new(ToolError(String::from_utf8_lossy(&output.stderr).to_string())));
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(result)]))
    }
}

#[mcp_tool(
    name = "git-clone",
    description = "Clone a Git repository using jj"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct GitCloneTool {
    /// Git repository URL to clone
    source: String,
    /// Destination directory
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<String>,
    /// Create a colocated jj/git repository
    #[serde(skip_serializing_if = "Option::is_none")]
    colocate: Option<bool>,
    /// Name for the remote
    #[serde(skip_serializing_if = "Option::is_none")]
    remote: Option<String>,
    /// Depth for shallow clone
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<u32>,
}

impl GitCloneTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let mut cmd = Command::new("jj");
        cmd.arg("git").arg("clone").arg(&self.source);

        if let Some(ref dest) = self.destination {
            cmd.arg(dest);
        }
        if let Some(true) = self.colocate {
            cmd.arg("--colocate");
        }
        if let Some(ref remote) = self.remote {
            cmd.arg("--remote").arg(remote);
        }
        if let Some(depth) = self.depth {
            cmd.arg("--depth").arg(depth.to_string());
        }

        let output = cmd.output().map_err(|e| CallToolError::new(e))?;
        
        let result = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            return Err(CallToolError::new(ToolError(String::from_utf8_lossy(&output.stderr).to_string())));
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(result)]))
    }
}

// Generate the JjTools enum with all tool variants
tool_box!(JjTools, [
    StatusTool,
    LogTool,
    DiffTool,
    CommitTool,
    NewTool,
    RebaseTool,
    GitCloneTool
]);