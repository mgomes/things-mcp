package things

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/url"
	"strings"
)

type AddInput struct {
	Title          string   `json:"title,omitempty"`
	Titles         []string `json:"titles,omitempty"`
	Notes          string   `json:"notes,omitempty"`
	When           string   `json:"when,omitempty"`
	Deadline       string   `json:"deadline,omitempty"`
	Tags           []string `json:"tags,omitempty"`
	ChecklistItems []string `json:"checklistItems,omitempty"`
	UseClipboard   string   `json:"useClipboard,omitempty"`
	List           string   `json:"list,omitempty"`
	ListID         string   `json:"listId,omitempty"`
	Heading        string   `json:"heading,omitempty"`
	HeadingID      string   `json:"headingId,omitempty"`
	Completed      *bool    `json:"completed,omitempty"`
	Canceled       *bool    `json:"canceled,omitempty"`
	ShowQuickEntry *bool    `json:"showQuickEntry,omitempty"`
	Reveal         *bool    `json:"reveal,omitempty"`
	CreationDate   string   `json:"creationDate,omitempty"`
	CompletionDate string   `json:"completionDate,omitempty"`
}

func (c *Client) Add(ctx context.Context, input AddInput) (string, error) {
	if len(input.Titles) == 0 && input.Title == "" && input.UseClipboard == "" && !boolValue(input.ShowQuickEntry) {
		return "", errors.New("provide at least one of title, titles, useClipboard, or showQuickEntry")
	}

	params := url.Values{}
	setString(params, "title", input.Title)
	if len(input.Titles) > 0 {
		params.Set("titles", joinLines(input.Titles))
	}
	setString(params, "notes", input.Notes)
	setString(params, "when", input.When)
	setString(params, "deadline", input.Deadline)
	if len(input.Tags) > 0 {
		params.Set("tags", strings.Join(input.Tags, ","))
	}
	if len(input.ChecklistItems) > 0 {
		params.Set("checklist-items", joinLines(input.ChecklistItems))
	}
	setString(params, "use-clipboard", input.UseClipboard)
	setString(params, "list", input.List)
	setString(params, "list-id", input.ListID)
	setString(params, "heading", input.Heading)
	setString(params, "heading-id", input.HeadingID)
	setBool(params, "completed", input.Completed)
	setBool(params, "canceled", input.Canceled)
	setBool(params, "show-quick-entry", input.ShowQuickEntry)
	setBool(params, "reveal", input.Reveal)
	setString(params, "creation-date", input.CreationDate)
	setString(params, "completion-date", input.CompletionDate)

	return c.dispatch(ctx, "add", params)
}

type AddProjectInput struct {
	Title          string   `json:"title,omitempty"`
	Notes          string   `json:"notes,omitempty"`
	When           string   `json:"when,omitempty"`
	Deadline       string   `json:"deadline,omitempty"`
	Tags           []string `json:"tags,omitempty"`
	Area           string   `json:"area,omitempty"`
	AreaID         string   `json:"areaId,omitempty"`
	ToDos          []string `json:"toDos,omitempty"`
	Completed      *bool    `json:"completed,omitempty"`
	Canceled       *bool    `json:"canceled,omitempty"`
	Reveal         *bool    `json:"reveal,omitempty"`
	CreationDate   string   `json:"creationDate,omitempty"`
	CompletionDate string   `json:"completionDate,omitempty"`
}

func (c *Client) AddProject(ctx context.Context, input AddProjectInput) (string, error) {
	params := url.Values{}
	setString(params, "title", input.Title)
	setString(params, "notes", input.Notes)
	setString(params, "when", input.When)
	setString(params, "deadline", input.Deadline)
	if len(input.Tags) > 0 {
		params.Set("tags", strings.Join(input.Tags, ","))
	}
	setString(params, "area", input.Area)
	setString(params, "area-id", input.AreaID)
	if len(input.ToDos) > 0 {
		params.Set("to-dos", joinLines(input.ToDos))
	}
	setBool(params, "completed", input.Completed)
	setBool(params, "canceled", input.Canceled)
	setBool(params, "reveal", input.Reveal)
	setString(params, "creation-date", input.CreationDate)
	setString(params, "completion-date", input.CompletionDate)

	return c.dispatch(ctx, "add-project", params)
}

type UpdateInput struct {
	AuthToken             string   `json:"authToken"`
	ID                    string   `json:"id"`
	Title                 *string  `json:"title,omitempty"`
	Notes                 *string  `json:"notes,omitempty"`
	PrependNotes          *string  `json:"prependNotes,omitempty"`
	AppendNotes           *string  `json:"appendNotes,omitempty"`
	When                  *string  `json:"when,omitempty"`
	Deadline              *string  `json:"deadline,omitempty"`
	Tags                  []string `json:"tags,omitempty"`
	AddTags               []string `json:"addTags,omitempty"`
	ChecklistItems        []string `json:"checklistItems,omitempty"`
	PrependChecklistItems []string `json:"prependChecklistItems,omitempty"`
	AppendChecklistItems  []string `json:"appendChecklistItems,omitempty"`
	List                  *string  `json:"list,omitempty"`
	ListID                *string  `json:"listId,omitempty"`
	Heading               *string  `json:"heading,omitempty"`
	HeadingID             *string  `json:"headingId,omitempty"`
	Completed             *bool    `json:"completed,omitempty"`
	Canceled              *bool    `json:"canceled,omitempty"`
	Reveal                *bool    `json:"reveal,omitempty"`
	Duplicate             *bool    `json:"duplicate,omitempty"`
	CreationDate          *string  `json:"creationDate,omitempty"`
	CompletionDate        *string  `json:"completionDate,omitempty"`
}

func (c *Client) Update(ctx context.Context, input UpdateInput) (string, error) {
	if input.AuthToken == "" {
		return "", errors.New("authToken is required")
	}
	if input.ID == "" {
		return "", errors.New("id is required")
	}

	params := url.Values{}
	params.Set("auth-token", input.AuthToken)
	params.Set("id", input.ID)

	setOptionalString(params, "title", input.Title)
	setOptionalString(params, "notes", input.Notes)
	setOptionalString(params, "prepend-notes", input.PrependNotes)
	setOptionalString(params, "append-notes", input.AppendNotes)
	setOptionalString(params, "when", input.When)
	setOptionalString(params, "deadline", input.Deadline)
	if len(input.Tags) > 0 {
		params.Set("tags", strings.Join(input.Tags, ","))
	}
	if len(input.AddTags) > 0 {
		params.Set("add-tags", strings.Join(input.AddTags, ","))
	}
	if len(input.ChecklistItems) > 0 {
		params.Set("checklist-items", joinLines(input.ChecklistItems))
	}
	if len(input.PrependChecklistItems) > 0 {
		params.Set("prepend-checklist-items", joinLines(input.PrependChecklistItems))
	}
	if len(input.AppendChecklistItems) > 0 {
		params.Set("append-checklist-items", joinLines(input.AppendChecklistItems))
	}
	setOptionalString(params, "list", input.List)
	setOptionalString(params, "list-id", input.ListID)
	setOptionalString(params, "heading", input.Heading)
	setOptionalString(params, "heading-id", input.HeadingID)
	setBool(params, "completed", input.Completed)
	setBool(params, "canceled", input.Canceled)
	setBool(params, "reveal", input.Reveal)
	setBool(params, "duplicate", input.Duplicate)
	setOptionalString(params, "creation-date", input.CreationDate)
	setOptionalString(params, "completion-date", input.CompletionDate)

	if len(params) <= 2 {
		return "", errors.New("provide at least one field to update")
	}

	return c.dispatch(ctx, "update", params)
}

type UpdateProjectInput struct {
	AuthToken      string   `json:"authToken"`
	ID             string   `json:"id"`
	Title          *string  `json:"title,omitempty"`
	Notes          *string  `json:"notes,omitempty"`
	PrependNotes   *string  `json:"prependNotes,omitempty"`
	AppendNotes    *string  `json:"appendNotes,omitempty"`
	When           *string  `json:"when,omitempty"`
	Deadline       *string  `json:"deadline,omitempty"`
	Tags           []string `json:"tags,omitempty"`
	AddTags        []string `json:"addTags,omitempty"`
	Area           *string  `json:"area,omitempty"`
	AreaID         *string  `json:"areaId,omitempty"`
	Completed      *bool    `json:"completed,omitempty"`
	Canceled       *bool    `json:"canceled,omitempty"`
	Reveal         *bool    `json:"reveal,omitempty"`
	Duplicate      *bool    `json:"duplicate,omitempty"`
	CreationDate   *string  `json:"creationDate,omitempty"`
	CompletionDate *string  `json:"completionDate,omitempty"`
}

func (c *Client) UpdateProject(ctx context.Context, input UpdateProjectInput) (string, error) {
	if input.AuthToken == "" {
		return "", errors.New("authToken is required")
	}
	if input.ID == "" {
		return "", errors.New("id is required")
	}

	params := url.Values{}
	params.Set("auth-token", input.AuthToken)
	params.Set("id", input.ID)

	setOptionalString(params, "title", input.Title)
	setOptionalString(params, "notes", input.Notes)
	setOptionalString(params, "prepend-notes", input.PrependNotes)
	setOptionalString(params, "append-notes", input.AppendNotes)
	setOptionalString(params, "when", input.When)
	setOptionalString(params, "deadline", input.Deadline)
	if len(input.Tags) > 0 {
		params.Set("tags", strings.Join(input.Tags, ","))
	}
	if len(input.AddTags) > 0 {
		params.Set("add-tags", strings.Join(input.AddTags, ","))
	}
	setOptionalString(params, "area", input.Area)
	setOptionalString(params, "area-id", input.AreaID)
	setBool(params, "completed", input.Completed)
	setBool(params, "canceled", input.Canceled)
	setBool(params, "reveal", input.Reveal)
	setBool(params, "duplicate", input.Duplicate)
	setOptionalString(params, "creation-date", input.CreationDate)
	setOptionalString(params, "completion-date", input.CompletionDate)

	if len(params) <= 2 {
		return "", errors.New("provide at least one field to update")
	}

	return c.dispatch(ctx, "update-project", params)
}

type ShowInput struct {
	ID     string   `json:"id,omitempty"`
	Query  string   `json:"query,omitempty"`
	Filter []string `json:"filter,omitempty"`
}

func (c *Client) Show(ctx context.Context, input ShowInput) (string, error) {
	if input.ID == "" && input.Query == "" {
		return "", errors.New("provide id or query")
	}

	params := url.Values{}
	setString(params, "id", input.ID)
	if input.ID == "" {
		setString(params, "query", input.Query)
	}
	if len(input.Filter) > 0 {
		params.Set("filter", strings.Join(input.Filter, ","))
	}

	return c.dispatch(ctx, "show", params)
}

type SearchInput struct {
	Query string `json:"query,omitempty"`
}

func (c *Client) Search(ctx context.Context, input SearchInput) (string, error) {
	params := url.Values{}
	setString(params, "query", input.Query)
	return c.dispatch(ctx, "search", params)
}

type VersionInput struct{}

func (c *Client) Version(ctx context.Context, _ VersionInput) (string, error) {
	return c.dispatch(ctx, "version", url.Values{})
}

type JSONInput struct {
	AuthToken string          `json:"authToken,omitempty"`
	Data      json.RawMessage `json:"data"`
	Reveal    *bool           `json:"reveal,omitempty"`
}

func (c *Client) JSON(ctx context.Context, input JSONInput) (string, error) {
	if len(input.Data) == 0 {
		return "", errors.New("data is required")
	}
	if !json.Valid(input.Data) {
		return "", errors.New("data must be valid JSON")
	}

	var compact bytes.Buffer
	if err := json.Compact(&compact, input.Data); err != nil {
		return "", fmt.Errorf("compact data: %w", err)
	}

	params := url.Values{}
	if input.AuthToken != "" {
		params.Set("auth-token", input.AuthToken)
	}
	params.Set("data", compact.String())
	setBool(params, "reveal", input.Reveal)

	return c.dispatch(ctx, "json", params)
}

func setString(params url.Values, key, value string) {
	if value == "" {
		return
	}
	params.Set(key, value)
}

func setOptionalString(params url.Values, key string, value *string) {
	if value == nil {
		return
	}
	params.Set(key, *value)
}

func setBool(params url.Values, key string, value *bool) {
	if value == nil {
		return
	}
	params.Set(key, fmt.Sprintf("%t", *value))
}

func joinLines(values []string) string {
	if len(values) == 0 {
		return ""
	}
	return strings.Join(values, "\n")
}

func boolValue(value *bool) bool {
	if value == nil {
		return false
	}
	return *value
}
