.PHONY: test build run

test:
	go test ./...

build:
	go build -o bin/things-mcp ./cmd/things-mcp

run:
	go run ./cmd/things-mcp $(ARGS)
