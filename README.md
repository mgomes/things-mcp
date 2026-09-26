# Things MCP

An MCP server for [Things](https://culturedcode.com/things/) on macOS. Agents read and edit your tasks through Things' AppleScript interface.

Any client connected to this server can change your tasks. Only give it to agents you trust.

## Requirements

- macOS with Things 3
- Rust 1.85+

## Install

```bash
cargo install --git https://github.com/mgomes/things-mcp
```

This puts `things-mcp` in `~/.cargo/bin`. GUI apps may not see that path, so use the absolute path (`which things-mcp`) in JSON configs.

The first call asks macOS to let your agent's app control Things. Allow it. The server never brings Things to the front.

## Add to your agent

**Claude Code**

```bash
claude mcp add -s user things -- things-mcp
```

**Codex**

```bash
codex mcp add things -- things-mcp
```

**Gemini CLI**

```bash
gemini mcp add -s user things things-mcp
```

**VS Code**

```bash
code --add-mcp '{"name":"things","command":"things-mcp"}'
```

**Cursor, Claude Desktop, and other JSON configs**

Add to `~/.cursor/mcp.json`, `~/Library/Application Support/Claude/claude_desktop_config.json`, or your client's equivalent:

```json
{
  "mcpServers": {
    "things": {
      "command": "/Users/you/.cargo/bin/things-mcp"
    }
  }
}
```

## Tools

| Tool | Does |
| --- | --- |
| `things-todos` | List to-dos in a built-in list, project, area, or tag |
| `things-search` | Find to-dos by title or notes |
| `things-get` | Get a to-do or project by ID |
| `things-projects` | List projects |
| `things-areas` | List areas |
| `things-tags` | List tags |
| `things-add` | Create a to-do |
| `things-add-project` | Create a project, optionally with to-dos |
| `things-update` | Edit, move, schedule, complete, or cancel a to-do or project |
| `things-delete` | Move a to-do or project to the Trash |
| `things-show` | Navigate the Things window to an item or list |

Everything returns JSON with IDs. Tools are annotated read-only or destructive so agents can auto-approve reads.

## Limitations

Things' AppleScript interface doesn't expose checklist items, headings, This Evening, or reminder times.

## Development

```bash
just test
just build
just install
```
