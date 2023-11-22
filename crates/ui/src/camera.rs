use bevy::{prelude::*, render::{render_resource::{TextureDescriptor, TextureFormat, TextureDimension, TextureUsages, Extent3d}, camera::RenderTarget}};

use crate::{node::UINode, UIID};

// component represents a camera that should be attached to a ui node with the given id
#[derive(Component, Default, Debug, Clone)]
pub struct UICamera {
    pub id: String,
    pub size: Vec2
}

// create new camera
impl UICamera {
    pub fn new(id: String) -> Self {
        Self { id, ..Default::default() }
    }
}

// setup a plugin to control everything
pub struct UICameraPlugin;
impl Plugin for UICameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ui_cameras);
    }
}

fn update_ui_cameras(
    mut images: ResMut<Assets<Image>>,
    mut ui: ResMut<UINode>,
    mut query: Query<(&mut Camera, &mut UICamera)>,
    nodes: Query<(&Node, &UIID)>
) {
    // loop through all ui cameras to update them
    query.for_each_mut(|(mut camera, mut ui_camera)| {
        // get ui
        let ui = ui.get_mut(&ui_camera.id);
        let ui = if ui.is_some() { ui.unwrap() } else { return };

        let node = nodes.iter().find(|(_, id)| id.0 == ui_camera.id);
        let (node, _) = if node.is_some() { node.unwrap() } else { return };

        // attempt to get image from ui
        let image_handle = ui.image.as_ref();
        let image = if image_handle.is_some() { Some(images.get(image_handle.unwrap()).unwrap()) } else { None };

        // should rebuild if is no image or image size is different
        let should_rebuild = image.is_none() || image.unwrap().size_f32() != node.size();

        // if should NOT rebuild, stop here
        if !should_rebuild { return }

        // create new image
        let mut new_image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d { width: node.size().x as u32, height: node.size().y as u32, depth_or_array_layers: 1 },
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                view_formats: &[],
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
            },
            ..default()
        };
        new_image.resize(new_image.texture_descriptor.size);

        // update image size tracker
        ui_camera.size = node.size();

        // replace image in assets if possible
        if image_handle.is_some() {
            images.insert(image_handle.unwrap(), new_image);
            return
        }

        // add image to assets
        let handle = images.add(new_image);

        // insert image into camera render target
        camera.target = RenderTarget::Image(handle.clone());

        // insert image into ui
        ui.image(handle);
    });
}
