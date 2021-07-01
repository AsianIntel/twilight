use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};

use super::{ComponentEmoji, ComponentType};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct SelectMenu {
    pub custom_id: String,
    pub options: Vec<SelectMenuOption>,
    pub placeholder: Option<String>,
    pub min_values: Option<u8>,
    pub max_values: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SelectMenuOption {
    pub label: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ComponentEmoji>,
    #[serde(default)]
    pub default: bool,
}

impl Serialize for SelectMenu {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let field_count = 1 + //type
            1 + // custom_id
            1 + // options
            if self.placeholder.is_none() { 0 } else { 1 } + // placeholder
            if self.min_values.is_none() { 0 } else { 1 } + // min_values
            if self.max_values.is_none() { 0 } else { 1 }; // max_values
        let mut state = serializer.serialize_struct("SelectMenu", field_count)?;

        state.serialize_field("type", &ComponentType::SelectMenu)?;
        state.serialize_field("custom_id", &self.custom_id)?;
        state.serialize_field("options", &self.options)?;
        if self.placeholder.is_some() {
            state.serialize_field("placeholder", &self.placeholder)?;
        }
        if self.min_values.is_some() {
            state.serialize_field("min_values", &self.min_values)?;
        }
        if self.max_values.is_some() {
            state.serialize_field("max_values", &self.max_values)?;
        }

        state.end()
    }
}
