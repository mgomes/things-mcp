package things

import (
	"context"
	"errors"
	"strings"
	"testing"
)

type fakeLauncher struct {
	calls []string
	err   error
}

func (f *fakeLauncher) Launch(_ context.Context, target string) error {
	f.calls = append(f.calls, target)
	return f.err
}

func TestDispatchEncodesSpacesAsPercent20(t *testing.T) {
	launcher := &fakeLauncher{}
	client := NewClient(Config{Launcher: launcher})

	got, err := client.dispatch(context.Background(), "add", mapValues(map[string]string{
		"title": "Buy milk",
	}))
	if err != nil {
		t.Fatalf("dispatch returned error: %v", err)
	}

	want := "things:///add?title=Buy%20milk"
	if got != want {
		t.Fatalf("dispatch returned %q, want %q", got, want)
	}

	if len(launcher.calls) != 1 {
		t.Fatalf("expected 1 launch call, saw %d", len(launcher.calls))
	}
	if launcher.calls[0] != want {
		t.Fatalf("launcher called with %q, want %q", launcher.calls[0], want)
	}
	if strings.Contains(launcher.calls[0], "+") {
		t.Fatalf("launcher call %q contains '+'", launcher.calls[0])
	}
}

func TestDispatchPropagatesLauncherError(t *testing.T) {
	launcher := &fakeLauncher{err: errors.New("boom")}
	client := NewClient(Config{Launcher: launcher})

	_, err := client.dispatch(context.Background(), "add", mapValues(nil))
	if err == nil || !strings.Contains(err.Error(), "boom") {
		t.Fatalf("expected error containing boom, got %v", err)
	}
}

func TestDispatchRequiresCommand(t *testing.T) {
	client := NewClient(Config{Launcher: &fakeLauncher{}})
	if _, err := client.dispatch(context.Background(), "", mapValues(nil)); err == nil {
		t.Fatalf("expected error for empty command")
	}
}

func mapValues(values map[string]string) map[string][]string {
	if len(values) == 0 {
		return map[string][]string{}
	}
	res := make(map[string][]string, len(values))
	for k, v := range values {
		res[k] = []string{v}
	}
	return res
}
