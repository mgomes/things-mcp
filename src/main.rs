mod things;

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerConfig},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use things::*;

#[derive(Clone)]
struct Server {
    activate: bool,
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
}

#[tool_router]
impl Server {
    #[tool(name = "things-add", description = "Create new to-dos in Things")]
    async fn add(&self, Parameters(p): Parameters<AddInput>) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(
        name = "things-add-project",
        description = "Create new projects in Things"
    )]
    async fn add_project(
        &self,
        Parameters(p): Parameters<AddProjectInput>,
    ) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(
        name = "things-update",
        description = "Update existing to-dos in Things"
    )]
    async fn update(&self, Parameters(p): Parameters<UpdateInput>) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(
        name = "things-update-project",
        description = "Update existing projects in Things"
    )]
    async fn update_project(
        &self,
        Parameters(p): Parameters<UpdateProjectInput>,
    ) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(
        name = "things-show",
        description = "Open Things lists, projects, areas, tags, or to-dos"
    )]
    async fn show(&self, Parameters(p): Parameters<ShowInput>) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(name = "things-search", description = "Open the Things search UI")]
    async fn search(&self, Parameters(p): Parameters<SearchInput>) -> Result<String, String> {
        self.open(p.url()).await
    }

    #[tool(
        name = "things-version",
        description = "Reveal the Things app and URL scheme version"
    )]
    async fn version(&self) -> Result<String, String> {
        self.open(Ok(version_url())).await
    }

    #[tool(
        name = "things-json",
        description = "Invoke the Things JSON command for complex imports and updates"
    )]
    async fn json(&self, Parameters(p): Parameters<JsonInput>) -> Result<String, String> {
        self.open(p.url()).await
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(
            Implementation::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        )
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let activate = std::env::args()
        .skip(1)
        .any(|a| a == "-activate" || a == "--activate");
    let server = Server {
        activate,
        tool_router: Server::tool_router(),
    };
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}
