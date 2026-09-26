use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;

const COMPONENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

type Result<T> = std::result::Result<T, String>;

#[derive(Default)]
struct Query(Vec<(&'static str, String)>);

impl Query {
    fn set(&mut self, key: &'static str, value: impl Into<String>) {
        self.0.push((key, value.into()));
    }

    fn str(&mut self, key: &'static str, value: &Option<String>) {
        if let Some(v) = value {
            self.set(key, v.as_str());
        }
    }

    fn bool(&mut self, key: &'static str, value: Option<bool>) {
        if let Some(v) = value {
            self.set(key, v.to_string());
        }
    }

    fn list(&mut self, key: &'static str, value: &Option<Vec<String>>, sep: &str) {
        if let Some(v) = value.as_ref().filter(|v| !v.is_empty()) {
            self.set(key, v.join(sep));
        }
    }

    fn url(&self, command: &str) -> String {
        let mut url = format!("things:///{command}");
        for (i, (k, v)) in self.0.iter().enumerate() {
            url.push(if i == 0 { '?' } else { '&' });
            url.push_str(k);
            url.push('=');
            url.extend(utf8_percent_encode(v, COMPONENT));
        }
        url
    }
}

fn require_update(q: &Query, auth_token: &str, id: &str) -> Result<()> {
    if auth_token.is_empty() {
        return Err("authToken is required".into());
    }
    if id.is_empty() {
        return Err("id is required".into());
    }
    if q.0.is_empty() {
        return Err("provide at least one field to update".into());
    }
    Ok(())
}

fn with_auth(auth_token: &str, id: &str, q: Query) -> Query {
    let mut out = Query::default();
    out.set("auth-token", auth_token);
    out.set("id", id);
    out.0.extend(q.0);
    out
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddInput {
    pub title: Option<String>,
    pub titles: Option<Vec<String>>,
    pub notes: Option<String>,
    pub when: Option<String>,
    pub deadline: Option<String>,
    pub tags: Option<Vec<String>>,
    pub checklist_items: Option<Vec<String>>,
    pub use_clipboard: Option<String>,
    pub list: Option<String>,
    pub list_id: Option<String>,
    pub heading: Option<String>,
    pub heading_id: Option<String>,
    pub completed: Option<bool>,
    pub canceled: Option<bool>,
    pub show_quick_entry: Option<bool>,
    pub reveal: Option<bool>,
    pub creation_date: Option<String>,
    pub completion_date: Option<String>,
}

impl AddInput {
    pub fn url(&self) -> Result<String> {
        let has_titles = self.titles.as_ref().is_some_and(|t| !t.is_empty());
        if self.title.is_none()
            && !has_titles
            && self.use_clipboard.is_none()
            && self.show_quick_entry != Some(true)
        {
            return Err(
                "provide at least one of title, titles, useClipboard, or showQuickEntry".into(),
            );
        }
        let mut q = Query::default();
        q.str("title", &self.title);
        q.list("titles", &self.titles, "\n");
        q.str("notes", &self.notes);
        q.str("when", &self.when);
        q.str("deadline", &self.deadline);
        q.list("tags", &self.tags, ",");
        q.list("checklist-items", &self.checklist_items, "\n");
        q.str("use-clipboard", &self.use_clipboard);
        q.str("list", &self.list);
        q.str("list-id", &self.list_id);
        q.str("heading", &self.heading);
        q.str("heading-id", &self.heading_id);
        q.bool("completed", self.completed);
        q.bool("canceled", self.canceled);
        q.bool("show-quick-entry", self.show_quick_entry);
        q.bool("reveal", self.reveal);
        q.str("creation-date", &self.creation_date);
        q.str("completion-date", &self.completion_date);
        Ok(q.url("add"))
    }
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddProjectInput {
    pub title: Option<String>,
    pub notes: Option<String>,
    pub when: Option<String>,
    pub deadline: Option<String>,
    pub tags: Option<Vec<String>>,
    pub area: Option<String>,
    pub area_id: Option<String>,
    pub to_dos: Option<Vec<String>>,
    pub completed: Option<bool>,
    pub canceled: Option<bool>,
    pub reveal: Option<bool>,
    pub creation_date: Option<String>,
    pub completion_date: Option<String>,
}

impl AddProjectInput {
    pub fn url(&self) -> Result<String> {
        let mut q = Query::default();
        q.str("title", &self.title);
        q.str("notes", &self.notes);
        q.str("when", &self.when);
        q.str("deadline", &self.deadline);
        q.list("tags", &self.tags, ",");
        q.str("area", &self.area);
        q.str("area-id", &self.area_id);
        q.list("to-dos", &self.to_dos, "\n");
        q.bool("completed", self.completed);
        q.bool("canceled", self.canceled);
        q.bool("reveal", self.reveal);
        q.str("creation-date", &self.creation_date);
        q.str("completion-date", &self.completion_date);
        Ok(q.url("add-project"))
    }
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInput {
    pub auth_token: String,
    pub id: String,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub prepend_notes: Option<String>,
    pub append_notes: Option<String>,
    pub when: Option<String>,
    pub deadline: Option<String>,
    pub tags: Option<Vec<String>>,
    pub add_tags: Option<Vec<String>>,
    pub checklist_items: Option<Vec<String>>,
    pub prepend_checklist_items: Option<Vec<String>>,
    pub append_checklist_items: Option<Vec<String>>,
    pub list: Option<String>,
    pub list_id: Option<String>,
    pub heading: Option<String>,
    pub heading_id: Option<String>,
    pub completed: Option<bool>,
    pub canceled: Option<bool>,
    pub reveal: Option<bool>,
    pub duplicate: Option<bool>,
    pub creation_date: Option<String>,
    pub completion_date: Option<String>,
}

impl UpdateInput {
    pub fn url(&self) -> Result<String> {
        let mut q = Query::default();
        q.str("title", &self.title);
        q.str("notes", &self.notes);
        q.str("prepend-notes", &self.prepend_notes);
        q.str("append-notes", &self.append_notes);
        q.str("when", &self.when);
        q.str("deadline", &self.deadline);
        q.list("tags", &self.tags, ",");
        q.list("add-tags", &self.add_tags, ",");
        q.list("checklist-items", &self.checklist_items, "\n");
        q.list(
            "prepend-checklist-items",
            &self.prepend_checklist_items,
            "\n",
        );
        q.list("append-checklist-items", &self.append_checklist_items, "\n");
        q.str("list", &self.list);
        q.str("list-id", &self.list_id);
        q.str("heading", &self.heading);
        q.str("heading-id", &self.heading_id);
        q.bool("completed", self.completed);
        q.bool("canceled", self.canceled);
        q.bool("reveal", self.reveal);
        q.bool("duplicate", self.duplicate);
        q.str("creation-date", &self.creation_date);
        q.str("completion-date", &self.completion_date);
        require_update(&q, &self.auth_token, &self.id)?;
        Ok(with_auth(&self.auth_token, &self.id, q).url("update"))
    }
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectInput {
    pub auth_token: String,
    pub id: String,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub prepend_notes: Option<String>,
    pub append_notes: Option<String>,
    pub when: Option<String>,
    pub deadline: Option<String>,
    pub tags: Option<Vec<String>>,
    pub add_tags: Option<Vec<String>>,
    pub area: Option<String>,
    pub area_id: Option<String>,
    pub completed: Option<bool>,
    pub canceled: Option<bool>,
    pub reveal: Option<bool>,
    pub duplicate: Option<bool>,
    pub creation_date: Option<String>,
    pub completion_date: Option<String>,
}

impl UpdateProjectInput {
    pub fn url(&self) -> Result<String> {
        let mut q = Query::default();
        q.str("title", &self.title);
        q.str("notes", &self.notes);
        q.str("prepend-notes", &self.prepend_notes);
        q.str("append-notes", &self.append_notes);
        q.str("when", &self.when);
        q.str("deadline", &self.deadline);
        q.list("tags", &self.tags, ",");
        q.list("add-tags", &self.add_tags, ",");
        q.str("area", &self.area);
        q.str("area-id", &self.area_id);
        q.bool("completed", self.completed);
        q.bool("canceled", self.canceled);
        q.bool("reveal", self.reveal);
        q.bool("duplicate", self.duplicate);
        q.str("creation-date", &self.creation_date);
        q.str("completion-date", &self.completion_date);
        require_update(&q, &self.auth_token, &self.id)?;
        Ok(with_auth(&self.auth_token, &self.id, q).url("update-project"))
    }
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ShowInput {
    pub id: Option<String>,
    pub query: Option<String>,
    pub filter: Option<Vec<String>>,
}

impl ShowInput {
    pub fn url(&self) -> Result<String> {
        let mut q = Query::default();
        match (&self.id, &self.query) {
            (Some(_), _) => q.str("id", &self.id),
            (None, Some(_)) => q.str("query", &self.query),
            (None, None) => return Err("provide id or query".into()),
        }
        q.list("filter", &self.filter, ",");
        Ok(q.url("show"))
    }
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct SearchInput {
    pub query: Option<String>,
}

impl SearchInput {
    pub fn url(&self) -> Result<String> {
        let mut q = Query::default();
        q.str("query", &self.query);
        Ok(q.url("search"))
    }
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct JsonInput {
    pub auth_token: Option<String>,
    pub data: Value,
    pub reveal: Option<bool>,
}

impl JsonInput {
    pub fn url(&self) -> Result<String> {
        if !self.data.is_array() {
            return Err("data must be a JSON array of Things objects".into());
        }
        let mut q = Query::default();
        q.str("auth-token", &self.auth_token);
        q.set("data", self.data.to_string());
        q.bool("reveal", self.reveal);
        Ok(q.url("json"))
    }
}

pub fn version_url() -> String {
    Query::default().url("version")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn s(v: &str) -> Option<String> {
        Some(v.into())
    }

    #[test]
    fn add_requires_content() {
        assert!(AddInput::default().url().is_err());
        let quick = AddInput {
            show_quick_entry: Some(true),
            ..Default::default()
        };
        assert_eq!(quick.url().unwrap(), "things:///add?show-quick-entry=true");
    }

    #[test]
    fn add_encodes_params() {
        let input = AddInput {
            title: s("Plan trip"),
            tags: Some(vec!["Travel".into(), "Planning".into()]),
            checklist_items: Some(vec!["a".into(), "b+c".into()]),
            ..Default::default()
        };
        assert_eq!(
            input.url().unwrap(),
            "things:///add?title=Plan%20trip&tags=Travel%2CPlanning&checklist-items=a%0Ab%2Bc"
        );
    }

    #[test]
    fn update_requires_auth_id_and_field() {
        let err = UpdateInput {
            id: "x".into(),
            title: s("t"),
            ..Default::default()
        }
        .url();
        assert!(err.unwrap_err().contains("authToken"));
        let err = UpdateInput {
            auth_token: "t".into(),
            title: s("t"),
            ..Default::default()
        }
        .url();
        assert!(err.unwrap_err().contains("id"));
        let err = UpdateInput {
            auth_token: "t".into(),
            id: "x".into(),
            ..Default::default()
        }
        .url();
        assert!(err.unwrap_err().contains("at least one"));
    }

    #[test]
    fn update_allows_clearing_deadline() {
        let input = UpdateProjectInput {
            auth_token: "tok".into(),
            id: "p1".into(),
            deadline: s(""),
            ..Default::default()
        };
        assert_eq!(
            input.url().unwrap(),
            "things:///update-project?auth-token=tok&id=p1&deadline="
        );
    }

    #[test]
    fn show_prefers_id() {
        let input = ShowInput {
            id: s("today"),
            query: s("ignored"),
            filter: None,
        };
        assert_eq!(input.url().unwrap(), "things:///show?id=today");
        assert!(ShowInput::default().url().is_err());
    }

    #[test]
    fn json_compacts_payload() {
        let input = JsonInput {
            data: json!([{ "type": "to-do", "attributes": { "title": "Buy milk" } }]),
            ..Default::default()
        };
        let url = input.url().unwrap();
        assert!(url.starts_with("things:///json?data=%5B%7B%22"));
        assert!(!url.contains(' ') && !url.contains('+'));
        assert!(
            JsonInput {
                data: json!({}),
                ..Default::default()
            }
            .url()
            .is_err()
        );
    }

    #[test]
    fn version_has_no_query() {
        assert_eq!(version_url(), "things:///version");
    }
}
