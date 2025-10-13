package things

import (
	"context"
	"errors"
	"fmt"
	"net/url"
	"os/exec"
	"strings"
)

// Launcher abstracts how Things URLs get dispatched. Useful for testing.
type Launcher interface {
	Launch(ctx context.Context, url string) error
}

type openLauncher struct {
	activate bool
}

func (o openLauncher) Launch(ctx context.Context, target string) error {
	args := []string{"open"}
	if !o.activate {
		args = append(args, "-g")
	}
	args = append(args, target)

	cmd := exec.CommandContext(ctx, args[0], args[1:]...)
	return cmd.Run()
}

// Client handles invoking the Things URL scheme.
type Client struct {
	launcher Launcher
}

// Config controls client behaviour.
type Config struct {
	Activate bool
	Launcher Launcher
}

// NewClient builds a new Client using the supplied config.
func NewClient(cfg Config) *Client {
	launcher := cfg.Launcher
	if launcher == nil {
		launcher = openLauncher{activate: cfg.Activate}
	}

	return &Client{launcher: launcher}
}

func (c *Client) dispatch(ctx context.Context, command string, params url.Values) (string, error) {
	if command == "" {
		return "", errors.New("command required")
	}

	target := "things:///" + command
	if encoded := encodeQuery(params); encoded != "" {
		target += "?" + encoded
	}
	if err := c.launcher.Launch(ctx, target); err != nil {
		return "", fmt.Errorf("launch %q: %w", target, err)
	}

	return target, nil
}

func encodeQuery(values url.Values) string {
	if len(values) == 0 {
		return ""
	}

	encoded := values.Encode()
	if !strings.Contains(encoded, "+") {
		return encoded
	}
	return strings.ReplaceAll(encoded, "+", "%20")
}
