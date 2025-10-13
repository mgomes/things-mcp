package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"github.com/moonbase/things-mcp/internal/things"
)

type invocationOutput struct {
	URL string `json:"url"`
}

func main() {
	var activate bool
	flag.BoolVar(&activate, "activate", false, "bring Things to the foreground when launching URLs")
	flag.Parse()

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	server := mcp.NewServer(&mcp.Implementation{
		Name:    "things-mcp",
		Version: "0.1.0",
	}, nil)

	client := things.NewClient(things.Config{
		Activate: activate,
	})

	registerTools(server, client)

	if err := server.Run(ctx, &mcp.StdioTransport{}); err != nil {
		log.Fatalf("run server: %v", err)
	}
}

func registerTools(server *mcp.Server, client *things.Client) {
	mcp.AddTool(server, &mcp.Tool{
		Name:        "things-add",
		Description: "Create new to-dos in Things using the URL scheme",
	}, func(ctx context.Context, req *mcp.CallToolRequest, input things.AddInput) (*mcp.CallToolResult, invocationOutput, error) {
		url, err := client.Add(ctx, input)
		if err != nil {
			return nil, invocationOutput{}, err
		}
		res, out := success(url)
		return res, out, nil
	})

	mcp.AddTool(server, &mcp.Tool{
		Name:        "things-add-project",
		Description: "Create new projects in Things",
	}, func(ctx context.Context, req *mcp.CallToolRequest, input things.AddProjectInput) (*mcp.CallToolResult, invocationOutput, error) {
		url, err := client.AddProject(ctx, input)
		if err != nil {
			return nil, invocationOutput{}, err
		}
		res, out := success(url)
		return res, out, nil
	})

	mcp.AddTool(server, &mcp.Tool{
		Name:        "things-update",
		Description: "Update existing to-dos in Things",
	}, func(ctx context.Context, req *mcp.CallToolRequest, input things.UpdateInput) (*mcp.CallToolResult, invocationOutput, error) {
		url, err := client.Update(ctx, input)
		if err != nil {
			return nil, invocationOutput{}, err
		}
		res, out := success(url)
		return res, out, nil
	})

	mcp.AddTool(server, &mcp.Tool{
		Name:        "things-update-project",
		Description: "Update existing projects in Things",
	}, func(ctx context.Context, req *mcp.CallToolRequest, input things.UpdateProjectInput) (*mcp.CallToolResult, invocationOutput, error) {
		url, err := client.UpdateProject(ctx, input)
		if err != nil {
			return nil, invocationOutput{}, err
		}
		res, out := success(url)
		return res, out, nil
	})

	mcp.AddTool(server, &mcp.Tool{
		Name:        "things-show",
		Description: "Open Things lists, projects, or tags",
	}, func(ctx context.Context, req *mcp.CallToolRequest, input things.ShowInput) (*mcp.CallToolResult, invocationOutput, error) {
		url, err := client.Show(ctx, input)
		if err != nil {
			return nil, invocationOutput{}, err
		}
		res, out := success(url)
		return res, out, nil
	})

	mcp.AddTool(server, &mcp.Tool{
		Name:        "things-search",
		Description: "Open the Things search UI",
	}, func(ctx context.Context, req *mcp.CallToolRequest, input things.SearchInput) (*mcp.CallToolResult, invocationOutput, error) {
		url, err := client.Search(ctx, input)
		if err != nil {
			return nil, invocationOutput{}, err
		}
		res, out := success(url)
		return res, out, nil
	})

	mcp.AddTool(server, &mcp.Tool{
		Name:        "things-version",
		Description: "Reveal the Things app and URL scheme version dialog",
	}, func(ctx context.Context, req *mcp.CallToolRequest, input things.VersionInput) (*mcp.CallToolResult, invocationOutput, error) {
		url, err := client.Version(ctx, input)
		if err != nil {
			return nil, invocationOutput{}, err
		}
		res, out := success(url)
		return res, out, nil
	})

	mcp.AddTool(server, &mcp.Tool{
		Name:        "things-json",
		Description: "Invoke the Things JSON command for complex imports",
	}, func(ctx context.Context, req *mcp.CallToolRequest, input things.JSONInput) (*mcp.CallToolResult, invocationOutput, error) {
		url, err := client.JSON(ctx, input)
		if err != nil {
			return nil, invocationOutput{}, err
		}
		res, out := success(url)
		return res, out, nil
	})
}

func success(url string) (*mcp.CallToolResult, invocationOutput) {
	result := &mcp.CallToolResult{
		Content: []mcp.Content{
			&mcp.TextContent{
				Text: fmt.Sprintf("Dispatched %s", url),
			},
		},
	}
	return result, invocationOutput{URL: url}
}
