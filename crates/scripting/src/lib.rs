use std::{rc::Rc, borrow::BorrowMut};

use bevy::{prelude::*, utils::{HashMap}};
use deno_core::*;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_non_send_resource::<ScriptingEngineWrapper>()
            .add_startup_system(start_scripts)
            .add_system(update_scripts);
    }
}

#[derive(Deref, DerefMut)]
pub struct ScriptingEngineWrapper(ScriptingEngine);

pub struct ScriptingEngine(JsRuntime, Vec<ScriptInstance>);

pub struct ScriptInstance {
    path: String,
    v8_value: v8::Global<v8::Value>
}

impl FromWorld for ScriptingEngineWrapper {
    fn from_world(_world: &mut World) -> Self {
        Self(ScriptingEngine(
            JsRuntime::new(RuntimeOptions {
                module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
                ..Default::default()
            }), Vec::new()
        ))
    }
}

fn load_script_raw(engine: &mut ScriptingEngine, path: String, content: String) {
    let runtime = &mut engine.0;
    let list = &mut engine.1;

    // get js runtime and execute the script
    let result = runtime.execute_script(Box::leak(path.clone().into_boxed_str()), FastString::from(content));

    // if executing the script passed, add the resulting info to the scripts list
    if result.is_ok() {
        list.push(
            ScriptInstance { 
                path: path, 
                v8_value: result.unwrap()
            }
        );
    } else { warn!("Could not load script at path {}", path) }
}

fn load_script_path(engine: &mut ScriptingEngine, path: String) {
    let content = std::fs::read_to_string(path.clone()).unwrap();
    load_script_raw(engine, path, content);
}

fn start_scripts(
    engine_wrapper: NonSendMut<ScriptingEngineWrapper>
) {
    load_script_path(&mut engine_wrapper.into_inner().0, "./assets/scripts/init.js".to_string());
}

fn update_scripts(
    engine_wrapper: NonSend<ScriptingEngineWrapper>
) {
    let engine = &engine_wrapper.1;
    // println!("Found {} scripts!", engine.len());
}