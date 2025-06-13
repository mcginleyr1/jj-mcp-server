use jj_mcp_server::*;
use mcp_sdk::tools::Tool;
use serde_json::json;
use std::fs;
use tempfile::TempDir;

fn create_test_repo() -> Result<TempDir, Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize a jj repository
    let output = std::process::Command::new("jj")
        .args(&["init", "--git"])
        .current_dir(repo_path)
        .output();

    match output {
        Ok(output) if output.status.success() => Ok(temp_dir),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to init jj repo: {}", stderr).into())
        }
        Err(e) => Err(format!("jj command not found: {}", e).into()),
    }
}

fn create_test_file(
    repo_path: &std::path::Path,
    filename: &str,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = repo_path.join(filename);
    fs::write(file_path, content)?;
    Ok(())
}

#[test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
fn test_status_tool_with_real_repo() {
    let temp_repo = match create_test_repo() {
        Ok(repo) => repo,
        Err(_) => {
            println!("Skipping integration test: jj not available");
            return;
        }
    };

    let repo_path = temp_repo.path().to_string_lossy().to_string();

    // Create a test file
    create_test_file(temp_repo.path(), "test.txt", "Hello, world!").unwrap();

    let status_tool = JjTool {
        name: "status".to_string(),
        description: "Show status".to_string(),
        input_schema: json!({"type": "object"}),
    };

    let args = json!({
        "repoPath": repo_path
    });

    let result = status_tool.call(Some(args)).unwrap();
    assert_eq!(result.is_error, Some(false));

    if let ToolResponseContent::Text { text } = &result.content[0] {
        assert!(!text.is_empty());
        // Should show the new file
        assert!(text.contains("test.txt") || text.contains("Working copy"));
    } else {
        panic!("Expected text content");
    }
}

#[test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
fn test_log_tool_with_real_repo() {
    let temp_repo = match create_test_repo() {
        Ok(repo) => repo,
        Err(_) => {
            println!("Skipping integration test: jj not available");
            return;
        }
    };

    let repo_path = temp_repo.path().to_string_lossy().to_string();

    let log_tool = JjTool {
        name: "log".to_string(),
        description: "Show log".to_string(),
        input_schema: json!({"type": "object"}),
    };

    let args = json!({
        "repoPath": repo_path,
        "limit": 5
    });

    let result = log_tool.call(Some(args)).unwrap();
    assert_eq!(result.is_error, Some(false));

    if let ToolResponseContent::Text { text } = &result.content[0] {
        assert!(!text.is_empty());
        // Should show at least the root commit
        assert!(text.contains("zzzzzzzz") || text.contains("root"));
    } else {
        panic!("Expected text content");
    }
}

#[test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
fn test_new_tool_with_real_repo() {
    let temp_repo = match create_test_repo() {
        Ok(repo) => repo,
        Err(_) => {
            println!("Skipping integration test: jj not available");
            return;
        }
    };

    let repo_path = temp_repo.path().to_string_lossy().to_string();

    let new_tool = JjTool {
        name: "new".to_string(),
        description: "Create new commit".to_string(),
        input_schema: json!({"type": "object"}),
    };

    let args = json!({
        "repoPath": repo_path
    });

    let result = new_tool.call(Some(args)).unwrap();
    // This should succeed or give a reasonable error
    if let ToolResponseContent::Text { text } = &result.content[0] {
        assert!(!text.is_empty());
    } else {
        panic!("Expected text content");
    }
}

#[test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
fn test_diff_tool_with_real_repo() {
    let temp_repo = match create_test_repo() {
        Ok(repo) => repo,
        Err(_) => {
            println!("Skipping integration test: jj not available");
            return;
        }
    };

    let repo_path = temp_repo.path().to_string_lossy().to_string();

    // Create a test file
    create_test_file(
        temp_repo.path(),
        "diff_test.txt",
        "This is a test file\nfor diff testing",
    )
    .unwrap();

    let diff_tool = JjTool {
        name: "diff".to_string(),
        description: "Show diff".to_string(),
        input_schema: json!({"type": "object"}),
    };

    let args = json!({
        "repoPath": repo_path,
        "summary": true
    });

    let result = diff_tool.call(Some(args)).unwrap();

    if let ToolResponseContent::Text { text } = &result.content[0] {
        assert!(!text.is_empty());
        // Should show changes or indicate no changes
    } else {
        panic!("Expected text content");
    }
}

#[test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
fn test_commit_tool_with_real_repo() {
    let temp_repo = match create_test_repo() {
        Ok(repo) => repo,
        Err(_) => {
            println!("Skipping integration test: jj not available");
            return;
        }
    };

    let repo_path = temp_repo.path().to_string_lossy().to_string();

    // Create a test file to commit
    create_test_file(
        temp_repo.path(),
        "commit_test.txt",
        "This file will be committed",
    )
    .unwrap();

    let commit_tool = JjTool {
        name: "commit".to_string(),
        description: "Create commit".to_string(),
        input_schema: json!({"type": "object"}),
    };

    let args = json!({
        "repoPath": repo_path,
        "message": "Test commit from integration test"
    });

    let result = commit_tool.call(Some(args)).unwrap();

    if let ToolResponseContent::Text { text } = &result.content[0] {
        assert!(!text.is_empty());
        // The result should indicate success or provide information about the commit
    } else {
        panic!("Expected text content");
    }
}

#[test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
fn test_rebase_tool_error_handling() {
    let temp_repo = match create_test_repo() {
        Ok(repo) => repo,
        Err(_) => {
            println!("Skipping integration test: jj not available");
            return;
        }
    };

    let repo_path = temp_repo.path().to_string_lossy().to_string();

    let rebase_tool = JjTool {
        name: "rebase".to_string(),
        description: "Rebase commits".to_string(),
        input_schema: json!({"type": "object"}),
    };

    // Try to rebase with invalid revisions
    let args = json!({
        "repoPath": repo_path,
        "source": "nonexistent",
        "destination": "alsononexistent"
    });

    let result = rebase_tool.call(Some(args)).unwrap();

    // This should result in an error
    if let ToolResponseContent::Text { text } = &result.content[0] {
        assert!(!text.is_empty());
        // Should contain error information
        assert!(
            text.contains("Error") || text.contains("error") || text.contains("No such revision")
        );
    } else {
        panic!("Expected text content");
    }
}

#[test]
fn test_invalid_repository_path() {
    let status_tool = JjTool {
        name: "status".to_string(),
        description: "Show status".to_string(),
        input_schema: json!({"type": "object"}),
    };

    let args = json!({
        "repoPath": "/completely/nonexistent/path/that/should/not/exist"
    });

    let result = status_tool.call(Some(args)).unwrap();
    assert_eq!(result.is_error, Some(true));

    if let ToolResponseContent::Text { text } = &result.content[0] {
        assert!(text.contains("Error"));
    } else {
        panic!("Expected text content");
    }
}

#[test]
fn test_tool_with_empty_args() {
    let status_tool = JjTool {
        name: "status".to_string(),
        description: "Show status".to_string(),
        input_schema: json!({"type": "object"}),
    };

    // Test with None arguments
    let result = status_tool.call(None).unwrap();

    // Should work with current directory if it's a jj repo, or error if not
    if let ToolResponseContent::Text { text } = &result.content[0] {
        assert!(!text.is_empty());
    } else {
        panic!("Expected text content");
    }
}

#[test]
fn test_git_clone_tool_invalid_source() {
    let clone_tool = JjTool {
        name: "git-clone".to_string(),
        description: "Clone repository".to_string(),
        input_schema: json!({"type": "object"}),
    };

    let args = json!({
        "source": "https://this-should-not-exist.invalid/repo.git",
        "destination": "test-repo"
    });

    let result = clone_tool.call(Some(args)).unwrap();

    // Should result in an error
    if let ToolResponseContent::Text { text } = &result.content[0] {
        assert!(!text.is_empty());
        // Should contain error information about the invalid URL
    } else {
        panic!("Expected text content");
    }
}
