use bevy::prelude::{Handle, Image};
use egui::Vec2;

pub struct ImageRenderer {
    // pub handle: Handle<Image>,
    pub texture: egui::TextureId,
    pub texture_size: Vec2,
}

impl ImageRenderer {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // println!("Rendering image");
        ui.image(self.texture, self.texture_size);
    }
}
/*
let image: Handle<Image> = asset_server.load(tab.path.to_str().expect("Could not convert path for image load"));
self.texture = Some(contexts.add_image(image));
*/