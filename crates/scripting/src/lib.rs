use bevy::prelude::*;

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

pub struct ScriptingEngine(Vec<ScriptInstance>);

pub struct ScriptInstance {
    path: String,
    script: js_sandbox::Script
}

impl FromWorld for ScriptingEngineWrapper {
    fn from_world(_world: &mut World) -> Self {
        Self(ScriptingEngine(Vec::new()))
    }
}

fn load_script_raw(engine: &mut ScriptingEngine, path: String, content: String) {
    let list = &mut engine.0;

    // get js runtime and execute the script
    let result = js_sandbox::Script::from_string(content.as_str());//runtime.execute_script(Box::leak(path.clone().into_boxed_str()), FastString::from(content));

    // if executing the script passed, add the resulting info to the scripts list
    if result.is_ok() {
        println!("Loaded script: {}", path);
        list.push(
            ScriptInstance { 
                path: path, 
                script: result.unwrap()
            }
        );
    } else { warn!("Could not load script at path {} with err {}", path, result.err().unwrap()) }
}

fn load_script_path(engine: &mut ScriptingEngine, path: String) {
    let content = std::fs::read_to_string(path.clone()).unwrap();
    load_script_raw(engine, path, content);
}

fn start_scripts(
    engine_wrapper: NonSendMut<ScriptingEngineWrapper>
) {
    load_script_path(&mut engine_wrapper.into_inner().0, "D:/nebulous/nebulousengine/assets/scripts/init.js".to_string());
}

fn update_scripts(
    engine_wrapper: NonSendMut<ScriptingEngineWrapper>
) {
    run_function(engine_wrapper, "update".to_string());
}

fn run_function(
    mut engine_wrapper: NonSendMut<ScriptingEngineWrapper>,
    route: String
) {
    // unwrap  the engine wrapper
    let engine = &mut engine_wrapper.0;

    for script in engine.0 {

    }

    // let runtime = &mut engine.0;
    // let handle = &mut runtime.handle_scope();

    // loop through each active script
    // for script in &mut engine.1 {
    //     // the scripts value as a local value
    //     let local_value = v8::Local::new(handle, &script.v8_value);
    //     script.v8_value.

    //     // attempt to convert the local value to an object, cancelling with a warning if fail
    //     let object = if let Ok(value) = v8::Local::<v8::Object>::try_from(local_value) {
    //         value
    //     } else {
    //         warn!("No export in script {}", script.path);
    //         return;
    //     };

    //     // get the route as a local string
    //     let local_route = v8::String::new_from_utf8(handle, route.as_bytes(), v8::NewStringType::Internalized).unwrap();

    //     // get the function from the object
    //     let function = if let Some(function) = object.get(handle, local_route.into()) { function } else { return; };

    //     // make sure function is a function
    //     let function = if let Ok(value) = v8::Local::<v8::Function>::try_from(function) { value }
    //     else { warn!("Object in export named {} was not function in script at path {}", route, script.path); return; };

    //     // create a try catch so that an error in the script is not a problem
    //     let tc_scope = &mut v8::TryCatch::new(handle);

    //     // call the function
    //     function.call(tc_scope, object.into(), &[]);

    //     // if any message came from the try catch, treat as error
    //     if let Some(message) = tc_scope.message() {
    //         // get the stack trace from the try catch
    //         let mut stack_trace_message = String::new();
    //         let stack_trace = message.get_stack_trace(tc_scope).unwrap();

    //         // loop through all frames of the stack trace
    //         for i in 0..stack_trace.get_frame_count() {
    //             // get frame and make sure it is valid
    //             let Some(frame) = stack_trace.get_frame(tc_scope, i) else { continue };

    //             // get the function name from the frame
    //             let function_name = frame
    //                 .get_function_name(tc_scope)
    //                 .map(|name| name.to_rust_string_lossy(tc_scope));

    //             // do the same for the script name
    //             let script_name = frame
    //                 .get_script_name(tc_scope)
    //                 .map(|name| name.to_rust_string_lossy(tc_scope));

    //             // push the frame information to the stack trace message
    //             stack_trace_message.push_str(&format!(
    //                 "\n    at {} ({}:{}:{})",
    //                 function_name.as_deref().unwrap_or("<unknown>"),
    //                 script_name.as_deref().unwrap_or("<unknown>"),
    //                 frame.get_line_number(),
    //                 frame.get_column()
    //             ));
    //         }

    //         // output the stack trace message as an error
    //         let message = message.get(tc_scope).to_rust_string_lossy(tc_scope);
    //         let message = message.trim_end_matches("Uncought ");
    //         error!("{message}{stack_trace_message}");
    //     }
    // }
}