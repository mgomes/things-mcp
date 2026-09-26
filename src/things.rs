use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

type Result<T> = std::result::Result<T, String>;

const DEFAULT_LIMIT: u32 = 50;

/// Builds the JSON request the Things script expects, dropping unset fields.
pub fn request(op: &str, input: &impl Serialize) -> Value {
    let mut v = serde_json::to_value(input).unwrap_or_else(|_| json!({}));
    if let Some(obj) = v.as_object_mut() {
        obj.retain(|_, v| !v.is_null());
        obj.insert("op".into(), op.into());
    }
    v
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum List {
    Inbox,
    Today,
    Tomorrow,
    Anytime,
    Upcoming,
    Someday,
    Logbook,
    Trash,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Open,
    Completed,
    Canceled,
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct TodosInput {
    pub list: Option<List>,
    /// Project name or ID
    pub project: Option<String>,
    /// Area name or ID
    pub area: Option<String>,
    /// Tag name or ID
    pub tag: Option<String>,
    /// Max items to return. Defaults to 50
    pub limit: Option<u32>,
}

impl TodosInput {
    pub fn request(mut self) -> Result<Value> {
        let sources = [
            self.list.is_some(),
            self.project.is_some(),
            self.area.is_some(),
            self.tag.is_some(),
        ];
        if sources.iter().filter(|s| **s).count() != 1 {
            return Err("provide exactly one of list, project, area, or tag".into());
        }
        self.limit.get_or_insert(DEFAULT_LIMIT);
        Ok(request("todos", &self))
    }
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchInput {
    /// Text to find in titles and notes
    pub query: String,
    /// Also search the Logbook
    pub include_completed: Option<bool>,
    /// Max items to return. Defaults to 50
    pub limit: Option<u32>,
}

impl SearchInput {
    pub fn request(mut self) -> Result<Value> {
        if self.query.is_empty() {
            return Err("query is required".into());
        }
        self.limit.get_or_insert(DEFAULT_LIMIT);
        Ok(request("search", &self))
    }
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct IdInput {
    /// To-do or project ID
    pub id: String,
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct ProjectsInput {
    /// Only projects in this area (name or ID)
    pub area: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct AddInput {
    pub title: String,
    pub notes: Option<String>,
    /// today, tomorrow, anytime, someday, or yyyy-mm-dd. Omit to put it in the Inbox
    pub when: Option<String>,
    /// yyyy-mm-dd
    pub deadline: Option<String>,
    /// Tag names
    pub tags: Option<Vec<String>>,
    /// Project name or ID
    pub project: Option<String>,
    /// Area name or ID
    pub area: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddProjectInput {
    pub title: String,
    pub notes: Option<String>,
    /// today, tomorrow, anytime, someday, or yyyy-mm-dd
    pub when: Option<String>,
    /// yyyy-mm-dd
    pub deadline: Option<String>,
    /// Tag names
    pub tags: Option<Vec<String>>,
    /// Area name or ID
    pub area: Option<String>,
    /// Titles of to-dos to create in the project
    pub to_dos: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInput {
    /// To-do or project ID
    pub id: String,
    pub title: Option<String>,
    /// Replaces the notes
    pub notes: Option<String>,
    /// Appended to the notes on a new line
    pub append_notes: Option<String>,
    /// today, tomorrow, anytime, someday, or yyyy-mm-dd
    pub when: Option<String>,
    /// yyyy-mm-dd, or an empty string to clear it
    pub deadline: Option<String>,
    /// Replaces all tags
    pub tags: Option<Vec<String>>,
    /// Added to the existing tags
    pub add_tags: Option<Vec<String>>,
    /// Project name or ID to move a to-do into, or an empty string to remove it from its project
    pub project: Option<String>,
    /// Area name or ID, or an empty string to remove it from its area
    pub area: Option<String>,
    pub status: Option<Status>,
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct ShowInput {
    /// To-do, project, area, or tag ID
    pub id: Option<String>,
    pub list: Option<List>,
}

impl ShowInput {
    pub fn request(self) -> Result<Value> {
        if self.id.is_some() == self.list.is_some() {
            return Err("provide either id or list".into());
        }
        Ok(request("show", &self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &str) -> Option<String> {
        Some(v.into())
    }

    #[test]
    fn request_drops_unset_fields() {
        let req = request(
            "update",
            &UpdateInput {
                id: "x".into(),
                deadline: s(""),
                status: Some(Status::Completed),
                add_tags: Some(vec!["a".into()]),
                ..Default::default()
            },
        );
        assert_eq!(
            req,
            json!({"op": "update", "id": "x", "deadline": "", "status": "completed", "addTags": ["a"]})
        );
    }

    #[test]
    fn todos_requires_one_source() {
        assert!(TodosInput::default().request().is_err());
        let both = TodosInput {
            list: Some(List::Today),
            tag: s("x"),
            ..Default::default()
        };
        assert!(both.request().is_err());
        let req = TodosInput {
            list: Some(List::Logbook),
            ..Default::default()
        }
        .request()
        .unwrap();
        assert_eq!(req, json!({"op": "todos", "list": "logbook", "limit": 50}));
    }

    #[test]
    fn search_requires_query() {
        assert!(SearchInput::default().request().is_err());
    }

    #[test]
    fn show_requires_id_or_list() {
        assert!(ShowInput::default().request().is_err());
        let both = ShowInput {
            id: s("x"),
            list: Some(List::Today),
        };
        assert!(both.request().is_err());
    }
}
