use bevy::prelude::*;
use camera::BetterCameraPlugin;
use input::ConfigurableInputPlugin;
use models::ModelPlugins;
use ui::ConfigurableUiPlugin;
use utils::NebulousEngineUtils;

// reexport all internal modules
pub use nebulousengine_models as models;
pub use nebulousengine_utils as utils;
pub use nebulousengine_input as input;
pub use nebulousengine_ui as ui;
pub use nebulousengine_camera as camera;

// combination plugin for all plugins
pub struct NebulousEngine;
impl Plugin for NebulousEngine {
    fn build(&self, app: &mut App) {
        app.add_plugins((BetterCameraPlugin, ConfigurableInputPlugin, ModelPlugins, ConfigurableUiPlugin, NebulousEngineUtils));
    }
}