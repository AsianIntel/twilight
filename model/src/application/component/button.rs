use super::{ComponentEmoji, ComponentType};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Clickable interactive components that render on messages.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#buttons
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Button {
    pub style: ButtonStyle,
    pub emoji: Option<ComponentEmoji>,
    pub label: Option<String>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    #[serde(default)]
    pub disabled: bool,
}

/// Style of a [`Button`].
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#buttons-button-styles
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}

impl Serialize for Button {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let field_count = 1 + // type
            1 + // style
            if self.emoji.is_none() { 0 } else { 1 } + // emoji
            if self.label.is_none() { 0 } else  { 1 } + // label
            if self.custom_id.is_none() { 0 } else  { 1 } + // custom_id
            if self.url.is_none() { 0 } else { 1 } + // url
            1; // disabled
        let mut state = serializer.serialize_struct("Button", field_count)?;

        state.serialize_field("type", &ComponentType::Button)?;
        state.serialize_field("style", &self.style)?;
        if self.emoji.is_some() {
            state.serialize_field("emoji", &self.emoji)?;
        }
        if self.label.is_some() {
            state.serialize_field("label", &self.label)?;
        }
        if self.custom_id.is_some() {
            state.serialize_field("custom_id", &self.custom_id)?;
        }
        if self.url.is_some() {
            state.serialize_field("url", &self.url)?;
        }
        state.serialize_field("disabled", &self.disabled)?;

        state.end()
    }
}
