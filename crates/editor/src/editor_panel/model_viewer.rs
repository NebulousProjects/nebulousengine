use bevy::{prelude::*, render::render_resource::*};
use nebulousengine_utils::{MainCamera, ViewportContainer};

pub struct ModelViewer {
    handle: Handle<Scene>,
    model: Option<Entity>,
    light: Option<Entity>,
    camera: Option<Entity>,
}

impl ModelViewer {
    pub fn new(asset_server: &AssetServer, path: &str) -> Self {
        Self {
            handle: asset_server.load(format!("{}#Scene0", path)), 
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
        self.model = Some(commands.spawn(SceneBundle {
            scene: handle,
            ..default()
        }).id());
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
        rendered_texture_id: Local<egui::TextureId>
    ) {
        viewport.enabled = true;
        let rect = ui.max_rect();
        viewport.size = Extent3d {
            width: rect.width() as u32,
            height: rect.height() as u32,
            ..Default::default()
        };
        ui.add(egui::Image::new(
            *rendered_texture_id,
            [rect.width(), rect.height()]
        ));
    }
}