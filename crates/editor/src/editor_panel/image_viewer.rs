use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::{Vec2, Rect};

pub struct ImageRenderer {
    pub handle: Handle<Image>,
    pub texture: egui::TextureId,
    pub texture_size: Option<Vec2>
}

impl ImageRenderer {
    pub fn new(asset_server: &AssetServer, contexts: &mut EguiContexts, path: &str) -> Self {
        let image: Handle<Image> = asset_server.load(path);
        let image_id = contexts.add_image(image.clone());
        Self { handle: image, texture: image_id, texture_size: None }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, rect: &Rect, images: &Res<Assets<Image>>) {
        if self.texture_size.is_some() {
            ui.image(self.texture, self.texture_size.unwrap());
        } else {
            // attempt to get image, skip if not loaded yet
            let image = images.get(&self.handle);
            if image.is_some() {
                // get and unpack width and height
                let dimensions = image.unwrap().texture_descriptor.size;
                let width = dimensions.width as f32;
                let height = dimensions.height as f32;

                // get scale
                let scale = rect.width() / width;
                let scale = scale.min(rect.height() / height).min(1.0);

                // save texture size with scale
                self.texture_size = Some(Vec2 { x: width * scale, y: height * scale });
            }
        }
    }
}
/*
let image: Handle<Image> = asset_server.load(tab.path.to_str().expect("Could not convert path for image load"));
self.texture = Some(contexts.add_image(image));
*/