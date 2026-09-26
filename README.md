# Things MCP

An MCP server for [Things](https://culturedcode.com/things/) on macOS. Agents read your tasks through AppleScript and write through the [Things URL scheme](https://culturedcode.com/things/support/articles/2803573/).

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

To let agents update items without asking you for a token, set `THINGS_AUTH_TOKEN`. Find it in **Things → Settings → General → Enable Things URLs → Manage**.

The first read asks macOS to let your agent's app control Things. Allow it.

## Add to your agent

**Claude Code**

```bash
claude mcp add -s user -e THINGS_AUTH_TOKEN=your-token things -- things-mcp
```

**Codex**

```bash
codex mcp add things --env THINGS_AUTH_TOKEN=your-token -- things-mcp
```

**Gemini CLI**

```bash
gemini mcp add -s user -e THINGS_AUTH_TOKEN=your-token things things-mcp
```

**VS Code**

```bash
code --add-mcp '{"name":"things","command":"things-mcp","env":{"THINGS_AUTH_TOKEN":"your-token"}}'
```

**Cursor, Claude Desktop, and other JSON configs**

Add to `~/.cursor/mcp.json`, `~/Library/Application Support/Claude/claude_desktop_config.json`, or your client's equivalent:

```json
{
  "mcpServers": {
    "things": {
      "command": "/Users/you/.cargo/bin/things-mcp",
      "args": [],
      "env": { "THINGS_AUTH_TOKEN": "your-token" }
    }
  }
}
```

## Tools

| Tool | Does |
| --- | --- |
| `things-todos` | List to-dos in a built-in list, project, area, or tag |
| `things-get` | Get a to-do or project by ID |
| `things-projects` | List projects |
| `things-areas` | List areas |
| `things-tags` | List tags |
| `things-add` | Create to-dos |
| `things-add-project` | Create a project, optionally with to-dos |
| `things-update` | Update a to-do |
| `things-update-project` | Update a project |
| `things-json` | Bulk create or update with the JSON command |
| `things-show` | Open an item or list in the Things window |
| `things-search` | Open Things search |
| `things-version` | Show the Things and URL scheme versions |

Read tools return JSON with IDs that the update tools accept. Tools are annotated read-only or destructive so agents can auto-approve reads.

## Development

```bash
just test
just build
just run -activate
```
