use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};

use super::{ComponentEmoji, ComponentType};

/// Dropdown-style interactive components that render on messages.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#select-menus
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct SelectMenu {
    /// Developer defined identifier for the select menu.
    pub custom_id: String,
    /// [Choices][`SelectMenuOption`] for the select menu.
    pub options: Vec<SelectMenuOption>,
    /// Custom placeholder text if nothing is selected.
    pub placeholder: Option<String>,
    /// Minimum number of items that must be chosen.
    pub min_values: Option<u8>,
    /// Maximum number of items that can be chosen.
    pub max_values: Option<u8>,
    #[serde(default)]
    pub disabled: bool,
}

/// Dropdown options that are part of [`SelectMenu`].
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SelectMenuOption {
    /// The user-facing name of the option.
    pub label: String,
    /// The developer defined value of the option.
    pub value: String,
    /// Additional description of the option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ComponentEmoji>,
    /// Whether the option will be selected by default
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
            if self.max_values.is_none() { 0 } else { 1 } + // max_values
            1; // disabled
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
        state.serialize_field("disabled", &self.disabled)?;

        state.end()
    }
}
