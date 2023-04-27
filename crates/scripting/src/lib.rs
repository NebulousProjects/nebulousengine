use bevy::prelude::*;
use nebulousengine_utils::RunningState;
use rhai::*;

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_non_send_resource::<ScriptEngineWrapper>()
            .add_system(update);
    }
}

#[derive(Deref, DerefMut)]
pub struct ScriptEngineWrapper<'a> {
    pub engine: ScriptEngine<'a>
}

pub struct ScriptEngine<'a> {
    pub engine: Engine,
    pub scripts: Vec<ScriptInstance<'a>>
}

pub struct ScriptInstance<'a> {
    pub ast: AST,
    pub scope: Scope<'a>
}

impl Default for ScriptEngineWrapper<'_> {
    fn default() -> Self {
        Self { engine: ScriptEngine { 
            engine: Engine::new(), 
            scripts: Vec::new()
        }}
    }
}

pub fn load_script_path(
    wrapper: &mut NonSendMut<ScriptEngineWrapper>,
    path: String
) {
    load_script_raw(wrapper, std::fs::read_to_string(path).unwrap())
}

pub fn load_script_raw(
    wrapper: &mut NonSendMut<ScriptEngineWrapper>,
    content: String
) {
    // compile the script, cancel if any errors
    let compiled = wrapper.engine.engine.compile(content);
    if compiled.is_err() {
        error!("Script load failed with error: {}", compiled.err().unwrap());
        return;
    }
    let compiled = compiled.unwrap();

    // create a new score
    let mut scope = Scope::new();
    let run_result = wrapper.engine.engine.run_ast_with_scope(&mut scope, &compiled);
    if run_result.is_err() {
        error!("Run result failed with error: {}", run_result.err().unwrap());
        return;
    }

    // save the script instance
    wrapper.engine.scripts.push(
        ScriptInstance { 
            ast: compiled, 
            scope: scope
        }
    )
}

pub fn execute_functions(
    wrapper: &mut NonSendMut<ScriptEngineWrapper>,
    route: String
) {
    // unpack wrapper
    let engine = &mut wrapper.engine;
    let scripts = &mut engine.scripts;

    // loop through each instance
    for instance in scripts.iter_mut() {
        // call the function
        let result = engine.engine.call_fn::<String>(&mut instance.scope, &instance.ast, route.clone(), ());

        // if the function errored, print that error
        if result.is_err() {
            error!("Executed script with error: {}", result.err().unwrap());
        }
    }
}

fn update(
    mut wrapper: NonSendMut<ScriptEngineWrapper>,
    running_state: ResMut<RunningState>
) {
    // update active script "systems"
    if running_state.running {
        execute_functions(&mut wrapper, "update".to_string());
    }
}