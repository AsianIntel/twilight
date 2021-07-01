use super::{Component, ComponentType};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct ActionRow {
    pub components: Vec<Component>,
}

impl Serialize for ActionRow {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let field_count = 1 + // type
            if self.components.is_empty() { 0 } else { 1 }; // components
        let mut state = serializer.serialize_struct("ActionRow", field_count)?;

        state.serialize_field("type", &ComponentType::ActionRow)?;
        if !self.components.is_empty() {
            state.serialize_field("components", &self.components)?;
        }

        state.end()
    }
}
