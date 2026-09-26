mod things;

use std::time::Duration;

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerConfig},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use serde_json::{Value, json};
use things::*;

const SCRIPT: &str = include_str!("things.js");

const INSTRUCTIONS: &str = "Reads and edits the Things task manager on macOS. \
Look items up with things-todos, things-search, or things-projects to get their IDs, then pass IDs to things-update or things-delete. \
Write tools return the item as it is after the change.";

#[derive(Clone)]
struct Server {
    tool_router: ToolRouter<Self>,
}

async fn run(req: Result<Value, String>) -> Result<String, String> {
    let output = tokio::process::Command::new("osascript")
        .args(["-l", "JavaScript", "-e", SCRIPT, &req?.to_string()])
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
            .trim_start_matches("execution error: ")
            .trim_start_matches("Error: ")
            .trim_start_matches("Error: ");
        let err = err.rsplit_once(" (-").map_or(err, |(msg, _)| msg);
        return Err(err.to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[tool_router]
impl Server {
    #[tool(
        name = "things-todos",
        description = "List to-dos in a built-in list, project, area, or tag",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn todos(&self, Parameters(p): Parameters<TodosInput>) -> Result<String, String> {
        run(p.request()).await
    }

    #[tool(
        name = "things-search",
        description = "Find to-dos whose title or notes contain the query",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn search(&self, Parameters(p): Parameters<SearchInput>) -> Result<String, String> {
        run(p.request()).await
    }

    #[tool(
        name = "things-get",
        description = "Get a to-do or project by ID. Projects include their to-dos",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn get(&self, Parameters(p): Parameters<IdInput>) -> Result<String, String> {
        run(Ok(request("get", &p))).await
    }

    #[tool(
        name = "things-projects",
        description = "List projects, optionally within one area",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn projects(&self, Parameters(p): Parameters<ProjectsInput>) -> Result<String, String> {
        run(Ok(request("projects", &p))).await
    }

    #[tool(
        name = "things-areas",
        description = "List areas",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn areas(&self) -> Result<String, String> {
        run(Ok(json!({ "op": "areas" }))).await
    }

    #[tool(
        name = "things-tags",
        description = "List tags",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn tags(&self) -> Result<String, String> {
        run(Ok(json!({ "op": "tags" }))).await
    }

    #[tool(
        name = "things-add",
        description = "Create a to-do",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            open_world_hint = false
        )
    )]
    async fn add(&self, Parameters(p): Parameters<AddInput>) -> Result<String, String> {
        run(Ok(request("add", &p))).await
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
        run(Ok(request("addProject", &p))).await
    }

    #[tool(
        name = "things-update",
        description = "Update a to-do or project. Only the fields you pass change. Set status to complete or cancel it",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            open_world_hint = false
        )
    )]
    async fn update(&self, Parameters(p): Parameters<UpdateInput>) -> Result<String, String> {
        run(Ok(request("update", &p))).await
    }

    #[tool(
        name = "things-delete",
        description = "Move a to-do or project to the Trash",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            open_world_hint = false
        )
    )]
    async fn delete(&self, Parameters(p): Parameters<IdInput>) -> Result<String, String> {
        run(Ok(request("remove", &p))).await
    }

    #[tool(
        name = "things-show",
        description = "Bring Things to the front showing an item or list",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn show(&self, Parameters(p): Parameters<ShowInput>) -> Result<String, String> {
        run(p.request()).await
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
    let server = Server {
        tool_router: Server::tool_router(),
    };
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}
