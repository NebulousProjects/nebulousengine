use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Levels {
    pub(crate) current_state: String,
    pub(crate) next_state: Option<String>
}

impl Levels {
    pub fn current(&self) -> String { self.current_state.clone() }
    pub fn goto(&mut self, next_state: impl Into<String>) { self.next_state = Some(next_state.into()); }
}
