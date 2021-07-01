use crate::application::component::ComponentType;
use serde::{Deserialize, Serialize};

/// Data received when an [`MessageComponent`] interaction is executed.
///
/// Refer to [the discord docs] for more information.
///
/// [`MessageComponent`]: crate::application::interaction::Interaction::MessageComponent
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageComponentInteractionData {
    pub custom_id: String,
    pub component_type: ComponentType,
    #[serde(default)]
    pub values: Vec<String>,
}
