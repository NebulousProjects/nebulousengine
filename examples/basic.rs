use bevy::prelude::*;

use nebulousengine_iced_ui::IcedUIPlugin;
use nebulousengine_input::*;
use nebulousengine_scenes::*;
use nebulousengine_ui::*;
use nebulousengine_entities::*;
 
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        // .add_plugin(UIPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(EntityPlugin)
        .add_plugin(IcedUIPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<SceneContainer> = asset_server.load("test.scene");
    commands.spawn_empty().insert(handle);
}