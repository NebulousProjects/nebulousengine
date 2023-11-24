use std::{fmt::Debug, hash::Hash};

use bevy::{prelude::*, ecs::system::EntityCommands};

use crate::CurrentLevel;

#[derive(Resource, Debug, Default)]
pub struct Level<T: States + Default + Debug + Eq + PartialEq + Hash> {
    pub(crate) state: T,
    pub(crate) next_state: Option<T>
}

impl <T: States + Default + Debug + Eq + PartialEq + Hash> Level<T> {
    pub fn current(&self) -> T { self.state.clone() }
    pub fn goto(&mut self, next_state: T) { self.next_state = Some(next_state); }

    pub fn add(&self, commands: &mut EntityCommands) {
        commands.insert(CurrentLevel);
    }

    pub fn spawn<B: Bundle>(&self, commands: &mut Commands, bundle: B) -> Entity     {
        commands.spawn((bundle, CurrentLevel)).id()
    }
}
