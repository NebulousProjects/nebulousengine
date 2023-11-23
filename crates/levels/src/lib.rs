use bevy::prelude::*;
use levels::Levels;

pub mod levels;

/**
 * The levels system is designed to make the seperation of cabailities and entities across levels.
 * 
 * TODO add global option to UI elements, if not set, give current level marker
 * TODO builder
 * TODO add on start, exit, and update systems for levels
 * TODO auto spawn and despawn in those systems
 */

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct CurrentLevel;

// create plugin for loading/unloading/updating levels
pub struct LevelsPlugin;
impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Levels>()
            .add_systems(Update, update);
    }
}

fn update(
    mut commands: Commands,
    mut levels: ResMut<Levels>,
    in_level: Query<Entity, With<CurrentLevel>>
) {
    // check if should change level
    if levels.next_state.is_some() {
        // despawn all in level objects
        in_level.for_each(|entity| commands.entity(entity).despawn_recursive());

        // change saved level ID
        levels.current_state = levels.next_state.clone().unwrap();
        levels.next_state = None;
    }
}
