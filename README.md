# jj MCP Server

A Model Context Protocol (MCP) server that provides tools for interacting with Jujutsu (jj) version control system. This server allows AI assistants and other MCP clients to perform common jj operations like checking status, rebasing, committing, and more.

## Features

The jj MCP server provides the following tools:

- **status** - Show the status of the working directory
- **rebase** - Rebase a revision onto another
- **commit** - Create a new commit with a message
- **new** - Create a new empty commit
- **log** - Show commit history with optional filtering
- **diff** - Show differences between revisions
- **git-clone** - Clone a Git repository using jj

All tools support optional parameters for repository path and working directory to operate on different repositories.

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Jujutsu (jj)](https://github.com/martinvonz/jj) installed and available in PATH
- A jj repository to work with (or use the git-clone tool to create one)

## Installation

### From Source

1. Clone this repository:
```bash
git clone <repository-url>
cd jj-mcp-server
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/jj-mcp-server`

## Usage

### As an MCP Server

The server communicates via stdio using the JSON-RPC protocol. Start the server:

```bash
./target/release/jj-mcp-server
```

### Tool Parameters

Most tools accept these common parameters:

- `repoPath` (optional): Path to the jj repository root
- `cwd` (optional): Working directory to run the command in

#### Status Tool
```json
{
  "repoPath": "/path/to/repo",
  "cwd": "/working/directory"
}
```

#### Rebase Tool
```json
{
  "source": "@",
  "destination": "main",
  "repoPath": "/path/to/repo",
  "cwd": "/working/directory"
}
```

#### Commit Tool
```json
{
  "message": "Your commit message",
  "repoPath": "/path/to/repo",
  "cwd": "/working/directory"
}
```

#### New Tool
```json
{
  "parents": "main",
  "repoPath": "/path/to/repo",
  "cwd": "/working/directory"
}
```

#### Log Tool
```json
{
  "limit": 10,
  "template": "commit_id \" \" description",
  "revisions": "main",
  "repoPath": "/path/to/repo",
  "cwd": "/working/directory"
}
```

#### Diff Tool
```json
{
  "from": "@-",
  "to": "@",
  "paths": ["src/", "README.md"],
  "context": 3,
  "summary": true,
  "stat": false,
  "repoPath": "/path/to/repo",
  "cwd": "/working/directory"
}
```

#### Git Clone Tool
```json
{
  "source": "https://github.com/user/repo.git",
  "destination": "local-repo",
  "colocate": true,
  "remote": "origin",
  "depth": 10
}
```

## Development

### Building

```bash
cargo build
```

### Running in Development

```bash
cargo run
```

### Testing

The project includes comprehensive unit and integration tests:

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run integration tests (requires jj to be installed)
cargo test --test integration_tests

# Run integration tests that require a real jj repository
cargo test --test integration_tests -- --ignored
```

#### Test Coverage

- **Unit Tests**: Parameter parsing, tool creation, command building, error handling
- **Integration Tests**: Real jj command execution with temporary repositories
- **Error Cases**: Invalid repositories, missing commands, malformed parameters

The integration tests create temporary jj repositories and test actual command execution, while unit tests focus on the internal logic and API structure.

### Code Structure

- `src/lib.rs` - Library crate with public API, tool implementations, and unit tests
- `src/main.rs` - Binary crate with MCP server setup and tool registration
- `tests/integration_tests.rs` - Integration tests with real jj repositories
- Parameter structs define the JSON schema for each tool's input
- All jj command functions are organized as separate, well-documented functions

## Configuration

The server uses the default MCP protocol configuration:

- **Protocol Version**: 2024-11-05
- **Transport**: stdio (JSON-RPC over stdin/stdout)
- **Capabilities**: Tools only (no prompts, resources, or logging)

## Error Handling

The server provides detailed error messages for:

- Invalid jj commands or parameters
- Repository access issues
- Missing dependencies (jj not installed)
- JSON parsing errors

All errors are returned as MCP tool responses with the `is_error` flag set to `true`.

## Jujutsu Primer

Jujutsu (jj) is a next-generation version control system. Here are some key concepts:

- **Revisions**: Every change is a revision, identified by a change ID
- **Working Copy**: Your current checkout, denoted by `@`
- **Bookmarks**: Similar to Git branches, but more flexible
- **No Staging Area**: Changes are automatically tracked
- **Immutable History**: Operations create new revisions rather than modifying existing ones

### Common Revision Syntax

- `@` - Current working copy revision
- `@-` - Parent of working copy
- `main` - Bookmark named "main"
- `abc123` - Revision by change ID prefix

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Related Projects

- [Jujutsu VCS](https://github.com/martinvonz/jj) - The version control system this server wraps
- [MCP SDK](https://github.com/modelcontextprotocol/rust-sdk) - Rust SDK for Model Context Protocol
- [Model Context Protocol](https://modelcontextprotocol.io/) - The protocol specification

## Troubleshooting

### "jj command not found"

Make sure Jujutsu is installed and available in your PATH:

```bash
# Install jj (example for macOS with Homebrew)
brew install jj

# Or install from source
cargo install --git https://github.com/martinvonz/jj jj-cli
```

### "Not a jj repository"

Initialize a jj repository or clone one:

```bash
# Initialize new repository
jj init

# Or clone from Git
jj git clone https://github.com/user/repo.git
```

### Permission Denied

Ensure the server binary has execute permissions:

```bash
chmod +x target/release/jj-mcp-server
```
