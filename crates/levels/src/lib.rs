use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::prelude::*;
use levels::Level;

pub mod levels;

/**
 * The levels system is designed to make the seperation of cabailities and entities across levels.
 * 
 * TODO add on start, exit, and update systems for levels
 * TODO auto spawn and despawn in those systems
 */

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct CurrentLevel;

// create plugin for loading/unloading/updating levels
#[derive(Default)]
pub struct Levels<T: States + Default + Debug + Eq + PartialEq + Hash>(pub PhantomData<T>);
impl <T: States + Default + Debug + Eq + PartialEq + Hash> Plugin for Levels<T> {
    fn build(&self, app: &mut App) {
        app
            .add_state::<T>()            
            .init_resource::<Level::<T>>()
            .add_systems(Update, update::<T>);
    }
}

fn update<T: States + Default + Debug + Eq + PartialEq + Hash>(
    mut commands: Commands,
    mut levels: ResMut<Level<T>>,
    mut next_state: ResMut<NextState<T>>,
    in_level: Query<Entity, With<CurrentLevel>>,
) {
    // check if should change level     
    if levels.next_state.is_some() {
        // change saved level ID if next is different
        let next = levels.next_state.clone().unwrap();
        if next != levels.current() {
            // perform state swap
            levels.state = next.clone();
            levels.next_state = None;
            next_state.set(next);
            
            // despawn all in level objects
            in_level.for_each(|entity| commands.entity(entity).despawn_recursive());
        }
    }
}
