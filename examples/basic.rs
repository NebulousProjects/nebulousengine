use bevy::prelude::*;

use nebulousengine_input::*;
use nebulousengine_scenes::*;
use nebulousengine_ui::*;
use nebulousengine_entities::*;
 
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        // .add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(EntityPlugin)
        .add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<SceneContainer> = asset_server.load("test.scene");
    commands.spawn_empty().insert(handle);
}