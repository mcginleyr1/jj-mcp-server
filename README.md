# jj MCP Server

A Model Context Protocol (MCP) server for [Jujutsu (jj)](https://github.com/martinvonz/jj) version control. Enables AI assistants to work with jj repositories using the correct workflow.

## Tools

| Tool              | Description                                               |
|-------------------|-----------------------------------------------------------|
| `status`          | Show working directory state                              |
| `log`             | View commit history and graph                             |
| `diff`            | Show changes in a revision                                |
| `describe`        | Set commit message for @ (does NOT create new commit)     |
| `new`             | Create new empty commit (optionally from specific parent) |
| `bookmark_create` | Create a named bookmark at a revision                     |
| `push`            | Push a bookmark to remote                                 |
| `sync`            | Fetch from all remotes                                    |
| `rebase`          | Move commits in the graph                                 |

## jj Workflow (for AI assistants)

jj is NOT git. Key differences:

- `@` is your current working commit - changes go directly into it
- There is no staging area
- `describe` sets a message but doesn't move anything
- `new` creates a fresh empty commit; previous `@` becomes `@-`

**Correct workflow:**
```
sync                              # fetch latest
new(parents="main")               # start from main
[make changes]                    # changes are in @
describe(message="what I did")    # label the work
new                               # ready for next change
[repeat as needed]
bookmark_create(name="feature")   # name the work
push(bookmark="feature")          # ship it
```

## Installation

```bash
cargo install --path .
```

## Configuration

Add to `~/.claude/mcp.json`:

```json
{
  "mcpServers": {
    "jj": {
      "command": "jj-mcp-server"
    }
  }
}
```

Alternatively 

``` json
claude mcp add --transport stdio jj-mcp-server jj-mcp-server
```

## Prerequisites

- [Jujutsu (jj)](https://github.com/martinvonz/jj) installed and in PATH
- Rust toolchain (for building)

## License

MIT
