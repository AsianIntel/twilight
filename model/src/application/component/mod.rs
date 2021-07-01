pub mod action_row;
pub mod button;
pub mod select_menu;

use crate::id::EmojiId;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter, Result as FmtResult};

use action_row::ActionRow;
use button::Button;
use select_menu::SelectMenu;

/// Interactive element of a message that an application uses.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#what-are-components
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Component {
    ActionRow(ActionRow),
    Button(Button),
    SelectMenu(SelectMenu),
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
    SelectMenu = 3,
}

/// Partial emoji used by components.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Hash)]
pub struct ComponentEmoji {
    pub id: Option<EmojiId>,
    pub name: String,
    #[serde(default)]
    pub animated: bool,
}

impl Component {
    pub const fn kind(&self) -> ComponentType {
        match self {
            Self::ActionRow(_) => ComponentType::ActionRow,
            Self::Button(_) => ComponentType::Button,
            Self::SelectMenu(_) => ComponentType::SelectMenu,
        }
    }
}

impl Display for ComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ActionRow => write!(f, "ActionRow"),
            Self::Button => write!(f, "Button"),
            Self::SelectMenu => write!(f, "SelectMenu"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        button::{Button, ButtonStyle},
        select_menu::{SelectMenu, SelectMenuOption},
        ActionRow, Component,
    };
    use serde_test::Token;

    #[test]
    fn test_component_full() {
        let component = Component::ActionRow(ActionRow {
            components: vec![
                Component::Button(Button {
                    style: ButtonStyle::Primary,
                    emoji: None,
                    label: Some("test label".into()),
                    custom_id: Some("test custom id".into()),
                    url: None,
                    disabled: false,
                }),
                Component::SelectMenu(SelectMenu {
                    custom_id: "test custom id 2".into(),
                    placeholder: Some("test placeholder".into()),
                    min_values: Some(5),
                    max_values: Some(25),
                    options: vec![SelectMenuOption {
                        label: "test option label".into(),
                        value: "test option value".into(),
                        description: Some("test description".into()),
                        emoji: None,
                        default: false,
                    }],
                }),
            ],
        });

        serde_test::assert_tokens(
            &component,
            &[
                Token::Struct {
                    name: "ActionRow",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(1),
                Token::Str("components"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "Button",
                    len: 5,
                },
                Token::Str("type"),
                Token::U8(2),
                Token::Str("style"),
                Token::U8(1),
                Token::Str("label"),
                Token::Some,
                Token::Str("test label"),
                Token::Str("custom_id"),
                Token::Some,
                Token::Str("test custom id"),
                Token::Str("disabled"),
                Token::Bool(false),
                Token::StructEnd,
                Token::Struct {
                    name: "SelectMenu",
                    len: 6,
                },
                Token::Str("type"),
                Token::U8(3),
                Token::Str("custom_id"),
                Token::Str("test custom id 2"),
                Token::Str("options"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "SelectMenuOption",
                    len: 4,
                },
                Token::Str("label"),
                Token::Str("test option label"),
                Token::Str("value"),
                Token::Str("test option value"),
                Token::Str("description"),
                Token::Some,
                Token::Str("test description"),
                Token::Str("default"),
                Token::Bool(false),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("placeholder"),
                Token::Some,
                Token::Str("test placeholder"),
                Token::Str("min_values"),
                Token::Some,
                Token::U8(5),
                Token::Str("max_values"),
                Token::Some,
                Token::U8(25),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
