pub mod button;

use crate::id::EmojiId;
use button::{Button, ButtonStyle};
use serde::{ser::Serializer, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Interactive element of a message that an application uses.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#what-are-components
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]

pub enum Component {
    ActionRow(Vec<Component>),
    Button(Button),
}

/// Type of Component.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#component-types
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
}

/// Partial emoji used by components.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Hash)]
pub struct ComponentEmoji {
    pub id: Option<EmojiId>,
    pub name: String,
    #[serde(default)]
    pub animated: bool,
}

impl Display for ComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ActionRow => write!(f, "ActionRow"),
            Self::Button => write!(f, "Button"),
        }
    }
}

#[derive(Serialize)]
struct ComponentEnvelope<'ser> {
    #[serde(rename = "type")]
    pub kind: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'ser str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<&'ser ComponentEmoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<&'ser str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'ser str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<&'ser [Component]>,
}

impl Serialize for Component {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let envelope = match self {
            Self::ActionRow(components) => ComponentEnvelope {
                kind: ComponentType::ActionRow,
                style: None,
                label: None,
                emoji: None,
                custom_id: None,
                url: None,
                disabled: None,
                components: Some(components.as_ref()),
            },
            Self::Button(button) => ComponentEnvelope {
                kind: ComponentType::Button,
                style: Some(button.style),
                label: button.label.as_ref().map(|s| s.as_str()),
                emoji: button.emoji.as_ref(),
                custom_id: button.custom_id.as_ref().map(|s| s.as_str()),
                url: button.url.as_ref().map(|s| s.as_str()),
                disabled: Some(button.disabled),
                components: None,
            },
        };

        envelope.serialize(serializer)
    }
}
