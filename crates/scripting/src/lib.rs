use std::rc::Rc;

use bevy::prelude::*;
use deno_core::*;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_non_send_resource::<JsRuntimeWrapper>()
            .add_startup_system(start_scripts)
            .add_system(update_scripts);
    }
}

#[derive(Deref, DerefMut)]
pub struct JsRuntimeWrapper(JsRuntime);

impl FromWorld for JsRuntimeWrapper {
    fn from_world(_world: &mut World) -> Self {
        Self(
            JsRuntime::new(RuntimeOptions {
                module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
                ..Default::default()
            })
        )
    }
}

fn start_scripts() {
}

fn update_scripts() {
}