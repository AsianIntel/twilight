use super::ComponentEmoji;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Clickable interactive components that render on messages.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#buttons
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
