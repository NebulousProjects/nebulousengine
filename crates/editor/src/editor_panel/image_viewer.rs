use bevy::prelude::{Handle, Image, Assets, Res};
use egui::Vec2;

pub struct ImageRenderer {
    pub handle: Handle<Image>,
    pub texture: egui::TextureId,
    pub texture_size: Option<Vec2>,
}

impl ImageRenderer {
    pub fn ui(&mut self, ui: &mut egui::Ui, images: &Res<Assets<Image>>) {
        if self.texture_size.is_some() {
            ui.image(self.texture, self.texture_size.unwrap());
        } else {
            let image = images.get(&self.handle);
            if image.is_some() {
                let dimensions = image.unwrap().texture_descriptor.size;
                self.texture_size = Some(Vec2 { x: dimensions.width as f32, y: dimensions.height as f32 });
            }
        }
    }
}
/*
let image: Handle<Image> = asset_server.load(tab.path.to_str().expect("Could not convert path for image load"));
self.texture = Some(contexts.add_image(image));
*/