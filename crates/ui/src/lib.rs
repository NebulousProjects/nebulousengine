use bevy::{prelude::*, asset::{AssetLoader, HandleId, LoadedAsset}, reflect::TypeUuid, ecs::system::EntityCommands};
use json::JsonValue;
use loader::*;

mod loader;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<UIContainer>()
            .init_asset_loader::<UILoader>()
            .add_event::<UIInteractEvent>()
            .add_system(button_listener)
            .add_system(load_ui)
            .add_system(reload);
    }
}

#[derive(Default)]
pub struct UILoader;
impl AssetLoader for UILoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // get json from bytes
            let str = std::str::from_utf8(bytes);
            if str.is_err() { return Err(bevy::asset::Error::msg("Could not convert bytes to json in input asset loader")) }
            let root_json = json::parse(str.unwrap());
            if root_json.is_err() { return Err(bevy::asset::Error::msg("Could not parse json in input asset loader")) }
            let root_json = root_json.unwrap();

            // save container
            load_context.set_default_asset(LoadedAsset::new(UIContainer { json: root_json }));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ui"]
    }
}

#[derive(Component, TypeUuid)]
#[uuid = "8510c296-d2ca-4353-8710-ce996aee18cb"]
pub struct UIContainer {
    json: JsonValue
}

#[derive(Component)]
pub struct UIContainerLoaded {
    path: HandleId
}

fn load_ui(
    mut commands: Commands,
    query: Query<(Entity, &Handle<UIContainer>), Without<UIContainerLoaded>>,
    assets: Res<Assets<UIContainer>>,
    asset_server: Res<AssetServer>
) {
    // loop through query and load each handle
    for (entity, handle) in query.iter() {
        let container = assets.get(handle);
        if container.is_some() {
            // get id
            let id = handle.id();
            let mut entity = commands.entity(entity);
            let json = &container.unwrap().json;

            // assemble ui
            add_ui_json_to_commands(json, &mut entity, &asset_server);
            entity.insert(UIContainerLoaded { path: id });
        }
    }
}

fn reload(
    mut commands: Commands,
    mut ev_assets: EventReader<AssetEvent<UIContainer>>,
    query: Query<(Entity, &UIContainerLoaded), With<UIContainerLoaded>>,
) {
    for event in &mut ev_assets {
        match event {
            AssetEvent::Modified { handle } => {
                let id = handle.id();
                query.iter().filter(|(_, loaded)| {
                    id == loaded.path
                }).for_each(|(entity, _)| {
                    let mut entity_cmds = commands.entity(entity);
                    entity_cmds.remove::<UIContainerLoaded>();
                    entity_cmds.despawn_descendants();
                });
            }
            _ => {}
        }
    }
}

pub struct UIInteractEvent {
    pub id: String,
    pub interaction: Interaction
}
fn button_listener(
    mut button_query: Query<
        (&Interaction, &ButtonID),
        (Changed<Interaction>, With<Button>)
    >,
    mut events: EventWriter<UIInteractEvent>
) {
    for (interaction, tag) in &mut button_query {
        events.send(UIInteractEvent {
            id: tag.id.clone(),
            interaction: interaction.clone()
        })
    }
}

pub fn add_ui_json_to_commands(input_json: &JsonValue, entity: &mut EntityCommands, asset_server: &Res<AssetServer>) {
    // add ui and children
    insert_json_ui_bundle(input_json, entity, asset_server);
    insert_children(input_json, entity, asset_server);
}
