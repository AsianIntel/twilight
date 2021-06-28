use super::{Component, ComponentType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActionRow {
    #[serde(rename = "type")]
    pub kind: ComponentType,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
}
