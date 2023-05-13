use bevy::{prelude::*, render::render_resource::Extent3d, ecs::{archetype::Archetypes, component::{ComponentId, Components}}};
use nebulousengine_entities::EntityContainer;
use nebulousengine_utils::{ViewportContainer, MainCamera};

pub struct EntityEditor {
    handle: Handle<EntityContainer>,
    model: Option<Entity>,
    light: Option<Entity>,
    camera: Option<Entity>,
}

impl EntityEditor {
    pub fn new(asset_server: &AssetServer, path: &str) -> Self {
        Self {
            handle: asset_server.load(path), 
            model: None, camera: None, light: None
        }
    }

    pub fn select(&mut self, commands: &mut Commands, viewport: &mut ResMut<ViewportContainer>) {
        // setup basic scene
        let handle = self.handle.clone();
        self.camera = Some(commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }).insert(MainCamera).id());
        self.model = Some(commands.spawn(handle).id());
        self.light = Some(commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 8000.0,
                ..Default::default()
            },
            ..Default::default()
        }).id());

        // enable viewport
        viewport.force_update = true;
    }

    pub fn deselect(&mut self, commands: &mut Commands, viewport: &mut ResMut<ViewportContainer>) {
        commands.entity(self.model.unwrap()).despawn_recursive();
        commands.entity(self.camera.unwrap()).despawn_recursive();
        commands.entity(self.light.unwrap()).despawn_recursive();
        self.model = None;
        self.light = None;
        self.camera = None;

        // disable viewport
        viewport.enabled = false;
    }

    pub fn close(&mut self) {}

    pub fn ui(
        &mut self, ui: &mut egui::Ui,
        viewport: &mut ResMut<ViewportContainer>,
        rendered_texture_id: Local<egui::TextureId>,
        archetypes: &Archetypes,
        components: &Components
    ) {
        egui::SidePanel::right("components").resizable(true).min_width(200.0).show_inside(ui, |ui| {
            ui.vertical(|ui| {
                let entity = self.model;
                if entity.is_some() {
                    let entity = entity.unwrap();
                    let component_ids = get_components_for_entity(&entity, archetypes);
                    // commands.entity(entity).log_components();
                    if component_ids.is_some() {
                        for comp_id in component_ids.unwrap() {
                            if let Some(comp_info) = components.get_info(comp_id) {
                                ui.collapsing(comp_info.name(), |ui| {
                                    
                                });
                            }
                        }
                    } else {
                        error!("Component IDs not found");
                    }
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // mark viewport enabled
            viewport.enabled = true;
    
            // get ui rectangle
            let rect = ui.max_rect();
    
            // update viewport rect
            viewport.size = Extent3d {
                width: rect.width() as u32,
                height: rect.height() as u32,
                ..Default::default()
            };
    
            // add image to ui
            ui.add(egui::Image::new(
                *rendered_texture_id,
                [rect.width(), rect.height()]
            ));
        });
    }
}

fn get_components_for_entity<'a>(
    entity: &Entity,
    archetypes: &'a Archetypes,
) -> Option<impl Iterator<Item = ComponentId> + 'a> {
    for archetype in archetypes.iter() {
        if archetype.entities().iter().any(|e| e.entity() == *entity) {
            return Some(archetype.components());
        }
    }
    None
}