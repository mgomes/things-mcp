# Things MCP

An MCP server for [Things](https://culturedcode.com/things/) on macOS. It wraps the [Things URL scheme](https://culturedcode.com/things/support/articles/2803573/) so your coding agent can create, update, and open to-dos and projects.

Any client connected to this server can change your tasks. Only give it to agents you trust.

## Requirements

- macOS with Things 3
- **Things → Settings → General → Enable Things URLs** turned on
- Rust 1.85+

## Install

```bash
cargo install --git https://github.com/mgomes/things-mcp
```

This puts `things-mcp` in `~/.cargo/bin`. GUI apps may not see that path, so use the absolute path (`which things-mcp`) in JSON configs.

Things stays in the background by default. Add `-activate` to the server's args to bring it to the front on every call.

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
      "command": "/Users/you/.cargo/bin/things-mcp",
      "args": []
    }
  }
}
```

## Tools

| Tool | Does |
| --- | --- |
| `things-add` | Create to-dos |
| `things-add-project` | Create a project, optionally with to-dos |
| `things-update` | Update a to-do |
| `things-update-project` | Update a project |
| `things-show` | Open a list, project, area, tag, or to-do |
| `things-search` | Open search |
| `things-version` | Show the Things and URL scheme versions |
| `things-json` | Bulk create or update with the JSON command |

Updates need your auth token from **Things → Settings → General → Enable Things URLs → Manage**. To get an item's ID, right-click it and choose **Share → Copy Link**.

## Limitations

The URL scheme is write-only. The server can't read or list your tasks.

## Development

```bash
just test
just build
just run -activate
```
