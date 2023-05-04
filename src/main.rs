use bevy::prelude::*;

use nebulousengine_editor::EditorPlugin;
use nebulousengine_input::*;
use nebulousengine_noneditor::*;
use nebulousengine_scenes::*;
use nebulousengine_ui::*;
use nebulousengine_utils::*;
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
        .insert_resource(RunningState::default());

    if cfg!(feature = "editor") {
        app.add_plugin(EditorPlugin);
    } else {
        app.add_plugin(NonEditorPlugin);
    }

    app.run();
}