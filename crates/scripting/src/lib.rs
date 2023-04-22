use bevy::prelude::*;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start)
            .add_system(update);
    }
}

fn start() {}

fn update() {}