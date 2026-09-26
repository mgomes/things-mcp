mod things;

use std::time::Duration;

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerConfig},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use serde_json::Value;
use things::*;

const READ_SCRIPT: &str = include_str!("read.js");

const INSTRUCTIONS: &str = "Controls the Things task manager on macOS. \
Use things-todos, things-get, things-projects, things-areas, and things-tags to read data and get IDs. \
things-show and things-search only change what the Things window displays and return no data. \
Writes go through the Things URL scheme and return the dispatched URL, not the created item.";

#[derive(Clone)]
struct Server {
    activate: bool,
    auth_token: Option<String>,
    tool_router: ToolRouter<Self>,
}

impl Server {
    async fn open(&self, url: Result<String, String>) -> Result<String, String> {
        let url = url?;
        let mut cmd = tokio::process::Command::new("open");
        if !self.activate {
            cmd.arg("-g");
        }
        let status = cmd
            .arg(&url)
            .status()
            .await
            .map_err(|e| format!("launch {url}: {e}"))?;
        if !status.success() {
            return Err(format!("launch {url}: {status}"));
        }
        Ok(format!("Dispatched {url}"))
    }

    async fn read(&self, req: Result<Value, String>) -> Result<String, String> {
        let output = tokio::process::Command::new("osascript")
            .args(["-l", "JavaScript", "-e", READ_SCRIPT, &req?.to_string()])
            .kill_on_drop(true)
            .output();
        let output = tokio::time::timeout(Duration::from_secs(60), output)
            .await
            .map_err(|_| "Things did not respond within 60s".to_string())?
            .map_err(|e| format!("run osascript: {e}"))?;
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            let err = err
                .trim()
                .trim_start_matches("execution error: Error: Error: ");
            let err = err.rsplit_once(" (-").map_or(err, |(msg, _)| msg);
            return Err(err.to_string());
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn token(&self, token: Option<String>) -> Option<String> {
        token.or_else(|| self.auth_token.clone())
    }
}

#[tool_router]
impl Server {
    #[tool(
        name = "things-todos",
        description = "List to-dos in a built-in list, project, area, or tag. Returns JSON with IDs, notes, dates, and tags",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn todos(&self, Parameters(p): Parameters<TodosInput>) -> Result<String, String> {
        self.read(p.request()).await
    }

    #[tool(
        name = "things-get",
        description = "Get a to-do or project by ID. Projects include their to-dos",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn get(&self, Parameters(p): Parameters<GetInput>) -> Result<String, String> {
        self.read(p.request()).await
    }

    #[tool(
        name = "things-projects",
        description = "List projects, optionally within one area",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn projects(&self, Parameters(p): Parameters<ProjectsInput>) -> Result<String, String> {
        self.read(Ok(p.request())).await
    }

    #[tool(
        name = "things-areas",
        description = "List areas",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn areas(&self) -> Result<String, String> {
        self.read(Ok(serde_json::json!({ "op": "areas" }))).await
    }

    #[tool(
        name = "things-tags",
        description = "List tags",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn tags(&self) -> Result<String, String> {
        self.read(Ok(serde_json::json!({ "op": "tags" }))).await
    }

    #[tool(
        name = "things-add",
        description = "Create to-dos",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            open_world_hint = false
        )
    )]
    async fn add(&self, Parameters(p): Parameters<AddInput>) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(
        name = "things-add-project",
        description = "Create a project, optionally with to-dos",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            open_world_hint = false
        )
    )]
    async fn add_project(
        &self,
        Parameters(p): Parameters<AddProjectInput>,
    ) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(
        name = "things-update",
        description = "Update a to-do. Only the fields you pass change",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            open_world_hint = false
        )
    )]
    async fn update(&self, Parameters(mut p): Parameters<UpdateInput>) -> Result<String, String> {
        p.auth_token = self.token(p.auth_token);
        self.open(p.url()).await
    }

    #[tool(
        name = "things-update-project",
        description = "Update a project. Only the fields you pass change",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            open_world_hint = false
        )
    )]
    async fn update_project(
        &self,
        Parameters(mut p): Parameters<UpdateProjectInput>,
    ) -> Result<String, String> {
        p.auth_token = self.token(p.auth_token);
        self.open(p.url()).await
    }

    #[tool(
        name = "things-show",
        description = "Open a list, project, area, tag, or to-do in the Things window. Returns no data; use things-todos to read",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn show(&self, Parameters(p): Parameters<ShowInput>) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(
        name = "things-search",
        description = "Open the Things search window. Returns no results; use things-todos to read",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn search(&self, Parameters(p): Parameters<SearchInput>) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(
        name = "things-version",
        description = "Open the Things version dialog",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn version(&self) -> Result<String, String> {
        self.open(Ok(version_url())).await
    }

    #[tool(
        name = "things-json",
        description = "Create or update many to-dos, projects, headings, and checklist items at once with the Things JSON command",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            open_world_hint = false
        )
    )]
    async fn json(&self, Parameters(mut p): Parameters<JsonInput>) -> Result<String, String> {
        p.auth_token = self.token(p.auth_token);
        self.open(p.url()).await
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(INSTRUCTIONS)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let activate = std::env::args()
        .skip(1)
        .any(|a| a == "-activate" || a == "--activate");
    let server = Server {
        activate,
        auth_token: std::env::var("THINGS_AUTH_TOKEN")
            .ok()
            .filter(|t| !t.is_empty()),
        tool_router: Server::tool_router(),
    };
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}
