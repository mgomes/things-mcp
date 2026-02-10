# Things MCP

`things-mcp` lets your coding agent (Claude, Cursor, Gemini, Copilot, etc.) drive the [Things](https://culturedcode.com/things/) task manager on macOS via its documented URL scheme. The server wraps each Things command as a Model Context Protocol (MCP) tool so your assistant can create, update, and reveal todos and projects without private APIs.

## Key Features

- **First-class Things commands** – Exposes `add`, `add-project`, `update`, `update-project`, `show`, `search`, `version`, and `json` as MCP tools.
- **Safe URL dispatch** – Normalizes outgoing URLs (e.g. spaces as `%20`) and supports optional foreground activation.
- **Composable toolkit** – Each tool returns the invoked Things URL, making it easy to log or retry actions in agents.

## Disclaimers

`things-mcp` launches Things through its URL scheme. Any MCP client with access to the server can navigate your task lists or create/update items. Only enable the server for trusted assistants and users.

## Requirements

- macOS with [Things](https://culturedcode.com/things/mac/) installed and “Things URLs” enabled in Things → Settings → General.
- Go `1.25.2` (the repo uses the Go toolchain manager).

## Getting Started

Clone the repo, then build and test:

```bash
make test
make build   # outputs bin/things-mcp
```

Start the server. By default Things stays in the background; pass `ARGS="-activate"` to tell the binary to bring Things to the front after each command:

```bash
make run                     # launch with background URLs
make run ARGS="-activate"    # launch and foreground Things each time
```

Add the following MCP server config to your client (adjust the binary path if needed):

```json
{
  "mcpServers": {
    "things-mcp": {
      "command": "/your/local/path/things-mcp/bin/things-mcp",
      "args": []
    }
  }
}
```

Pass `"-activate"` or other flags in the `args` array when you want to foreground Things:

```json
{
  "mcpServers": {
    "things-mcp": {
      "command": "/your/local/path/things-mcp/bin/things-mcp",
      "args": ["-activate"]
    }
  }
}
```

### MCP Client Configuration

<details>
  <summary>Codex CLI</summary>
  Run:

```bash
codex mcp add things-mcp -- /your/local/path/things-mcp/bin/things-mcp
```

Add `-activate` after the binary path if you want Things to pop to the foreground:

```bash
codex mcp add things-mcp -- /your/local/path/things-mcp/bin/things-mcp -activate
```

</details>

<details>
  <summary>Claude Desktop</summary>
  Edit `~/Library/Application Support/Claude/claude_desktop_config.json` and add the snippet above under `mcpServers`. Restart Claude Desktop afterwards.
</details>

<details>
  <summary>Claude Code CLI</summary>
  Run:

```bash
claude mcp add things-mcp /your/local/path/things-mcp/bin/things-mcp
```

Add `-activate` after the binary path if you want Things to pop to the foreground.

</details>

<details>
  <summary>Cursor</summary>
  Go to **Settings → MCP → New MCP Server**, choose “Stdio”, set the command to the built binary path, and optionally add `-activate` in arguments. Alternatively use the deeplink builder inside Cursor with the JSON above.
</details>

<details>
  <summary>Gemini CLI</summary>

```bash
gemini mcp add things-mcp /your/local/path/things-mcp/bin/things-mcp
```

Supply `--args -activate` if you want foreground launches.

</details>

<details>
  <summary>GitHub Copilot CLI</summary>
  Inside the Copilot prompt run `/mcp add`, choose “Local” server type, set command to the binary path, and leave arguments blank (or `-activate` as desired).
</details>

<details>
  <summary>JetBrains AI Assistant / Junie</summary>
  Navigate to **Settings → Tools → AI Assistant → Model Context Protocol**, click **Add**, set the command field to the built binary, and specify any arguments. Repeat the same flow for Junie under **Settings → Tools → Junie → MCP Settings**.
</details>

<details>
  <summary>VS Code / Copilot Chat</summary>
  Run:

```bash
code --add-mcp '{"name":"things-mcp","command":"/your/local/path/things-mcp/bin/things-mcp","args":[]}'
```

Reopen VS Code so Copilot Chat loads the server.

</details>

<details>
  <summary>Warp</summary>
  Open **Settings → AI → Manage MCP Servers → + Add**, select “Local”, and use the standard command/args snippet.
</details>

## Tools

- `things-add` – create todos (supports multi-title batches, tags, deadlines, etc.)
- `things-add-project` – create projects with optional child todos and metadata
- `things-update` – update existing todos (requires Things auth token)
- `things-update-project` – update existing projects (requires auth token)
- `things-show` – reveal a list/project/todo or quick find query
- `things-search` – open the search UI with optional query text
- `things-version` – show the Things build/scheme version dialog
- `things-json` – invoke the JSON batch command for complex imports

Each tool returns structured output with the dispatched URL so clients can display or reuse it.

## Testing

Run the suite with:

```bash
make test
```

The tests cover URL encoding, validation, and JSON compaction logic.

## Known Limitations

- The Things URL scheme is write- and navigation-focused; it does **not** provide endpoints to list existing todos or projects. Use Things directly (or another integration) when you need to read structured data.
