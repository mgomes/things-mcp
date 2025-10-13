package things

import (
	"context"
	"encoding/json"
	"strings"
	"testing"
)

func TestAddRequiresContent(t *testing.T) {
	client := NewClient(Config{Launcher: &fakeLauncher{}})

	if _, err := client.Add(context.Background(), AddInput{}); err == nil {
		t.Fatalf("expected error when no title, titles, clipboard, or quick entry flag provided")
	}
}

func TestAddBuildsExpectedURL(t *testing.T) {
	launcher := &fakeLauncher{}
	client := NewClient(Config{Launcher: launcher})

	_, err := client.Add(context.Background(), AddInput{
		Title: "Plan trip",
		Tags:  []string{"Travel", "Planning"},
	})
	if err != nil {
		t.Fatalf("Add returned error: %v", err)
	}

	got := launcher.calls[0]
	want := "things:///add?tags=Travel%2CPlanning&title=Plan%20trip"
	if got != want {
		t.Fatalf("Add dispatched %q, want %q", got, want)
	}
}

func TestUpdateRequiresAuthAndID(t *testing.T) {
	client := NewClient(Config{Launcher: &fakeLauncher{}})

	_, err := client.Update(context.Background(), UpdateInput{
		ID: "todo-id",
	})
	if err == nil || !strings.Contains(err.Error(), "authToken") {
		t.Fatalf("expected authToken error, got %v", err)
	}

	_, err = client.Update(context.Background(), UpdateInput{
		AuthToken: "token",
	})
	if err == nil || !strings.Contains(err.Error(), "id") {
		t.Fatalf("expected id error, got %v", err)
	}
}

func TestJSONCompactsPayload(t *testing.T) {
	launcher := &fakeLauncher{}
	client := NewClient(Config{Launcher: launcher})

	raw := json.RawMessage(`[
		{
			"type": "to-do",
			"attributes": {
				"title": "Buy milk"
			}
		}
	]`)

	if _, err := client.JSON(context.Background(), JSONInput{Data: raw}); err != nil {
		t.Fatalf("JSON returned error: %v", err)
	}

	got := launcher.calls[0]
	if strings.Contains(got, " ") {
		t.Fatalf("expected compact JSON, got %q", got)
	}
	if !strings.Contains(got, "%7B%22type%22%3A%22to-do%22") {
		t.Fatalf("expected URL-encoded JSON payload, got %q", got)
	}
}
