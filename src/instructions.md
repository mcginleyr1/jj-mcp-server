MCP server for Jujutsu (jj) version control operations.

IMPORTANT: Always use these tools instead of running jj commands directly via bash/shell.

## Key Concept: jj is NOT git

In jj, you work directly in a commit (@). Changes are automatically tracked.
There is no staging area. The working directory IS the commit.

- @ = your current working commit (where your changes live)
- @- = the parent of @ (the previous commit)
- 'describe' sets a message on @ but does NOT move anything
- 'new' creates a fresh empty commit; your previous @ becomes @-

## Available Tools

- status: See current state (changed files, current revision)
- log: View commit history and graph
- diff: See changes in a revision
- describe: Set the commit message for @ (does NOT create new commit!)
- new: Create new empty commit (use 'parents' param to specify base, e.g., "main")
- bookmark_create: Create a named bookmark pointing to a revision
- push: Push a bookmark to remote
- sync: Fetch from all remotes
- rebase: Move commits in the graph

## Correct Workflow

1. sync - fetch latest from remotes
2. new(parents="main") - start fresh working commit from main
3. [make file changes] - changes go directly into @
4. describe(message="what I did") - label the work (@ stays @, nothing moves!)
5. new - when ready for next logical change, creates new @ (old @ becomes @-)
6. [repeat 3-5 as needed for multiple commits]
7. bookmark_create(name="feature-name", revision="@") - name your work
8. push(bookmark="feature-name") - send to remote

## WRONG (git-brain mistakes to avoid)

- Do NOT use 'new' after every file change - only when starting a NEW logical commit
- Do NOT think 'describe' moves anything - it just sets a message
- Do NOT look for a 'commit' tool - 'describe' + 'new' is the pattern
- There is no staging. All file changes in the working directory are part of @.

## Common Patterns

Starting work: sync, then new(parents="main")
Save progress: describe(message="...") - that's it, work is already in @
Multiple commits: describe("first thing"), new, describe("second thing"), new, ...
Ship it: bookmark_create(name="x", revision="@"), push(bookmark="x")
