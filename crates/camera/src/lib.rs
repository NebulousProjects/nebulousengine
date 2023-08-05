use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, camera::{RenderTarget, Viewport}}};

pub struct BetterCameraPlugin;
impl Plugin for BetterCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_image_cameras);
    }
}

// component to represent a camera that targets an image
#[derive(Component)]
pub struct ImageCamera {
    pub image_handle: Option<Handle<Image>>,
    pub size: Extent3d,
    pub last_size: Extent3d,
    pub force_update: bool
}

// add default options to editor camera
impl Default for ImageCamera {
    fn default() -> Self {
        Self {
            image_handle: None,
            size: Extent3d { width: 200, height: 200, ..Default::default() }, 
            last_size: Default::default(), 
            force_update: false
        }
    }
}

// update all image cameras
fn update_image_cameras(
    mut images: ResMut<Assets<Image>>,
    mut cameras: Query<(&mut Camera, &mut ImageCamera)>
) {
    cameras.for_each_mut(|(mut camera, mut editor_camera)| {
        // update camera size, skip if no size change or force update
        // if editor_camera.last_size == editor_camera.size && !editor_camera.force_update { return }
        editor_camera.force_update = false;
        editor_camera.last_size = editor_camera.size;

        // This is the texture that will be rendered to.
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: editor_camera.size,
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
        image.resize(editor_camera.size);

        // update image handle
        let image_handle = images.add(image);
        editor_camera.image_handle = Some(image_handle.clone());

        // update render target
        camera.target = RenderTarget::Image(editor_camera.image_handle.clone().expect("hi"));
        camera.viewport = Some(
            Viewport {
                physical_size: UVec2 { x: editor_camera.size.width, y: editor_camera.size.height },
                physical_position: UVec2 { x: 0, y: 0 },
                depth: 0.0..1.0
            }
        )
    });
}