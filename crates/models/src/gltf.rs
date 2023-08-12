use bevy::{prelude::*, gltf::Gltf, utils::{HashMap, Duration}, scene::SceneInstance};

pub struct GLTFPlugin;
impl Plugin for GLTFPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (load_better_gltf, update_loaded_gltfs));
    }
}

#[derive(Component)]
pub struct GLTFModel {
    handle: Handle<Gltf>,
    animations: HashMap<String, Handle<AnimationClip>>,
    offset: Vec3,
    scale: Vec3,
    current_animation: Option<String>,
    next_animation: Option<String>,
    animation_started_time: f32,
    force_next_animation: bool
}

// functions for making gltf models easier
impl GLTFModel {
    pub fn from_str(asset_server: &Res<AssetServer>, model: impl Into<String>, offset: Vec3, scale: Vec3) -> Self {
        Self { 
            handle: asset_server.load(model.into()), 
            animations: HashMap::new(), 
            offset, scale,
            current_animation: Option::None, 
            next_animation: Option::None,
            animation_started_time: 0.,
            force_next_animation: false
        }
    }

    pub fn from_handle(handle: Handle<Gltf>, offset: Vec3, scale: Vec3) -> Self {
        Self { 
            handle, animations: HashMap::new(),
            offset, scale,
            current_animation: Option::None, 
            next_animation: Option::None,
            animation_started_time: 0.,
            force_next_animation: false
        }
    }

    // setters
    pub fn set_next_animation(&mut self, anim: impl Into<String>) {
        self.next_animation = Some(anim.into());
    }

    pub fn set_current_animation(&mut self, anim: impl Into<String>) {
        self.next_animation = Some(anim.into());
        self.force_next_animation = true;
    }

    // getters and checkers
    pub fn get_current_animation(&self) -> &Option<String> { &self.current_animation }
    pub fn get_current_animation_as_string(&self) -> String { self.current_animation.clone().unwrap_or("none".to_string()) }
    pub fn has_next_animation(&self) -> bool { self.next_animation.is_some() }
    pub fn has_animation(&self) -> bool { self.current_animation.is_some() }
    pub fn get_animations(&self) -> Vec<String> { self.animations.iter().map(|a| { a.0.clone() }).collect() }
}

// Component to mark gltf models as loaded
#[derive(Component)]
pub struct GLTFModelLoaded;

fn load_better_gltf(
    mut commands: Commands,
    mut query: Query<(Entity, &mut GLTFModel, Without<GLTFModelLoaded>)>,
    gltfs: Res<Assets<Gltf>>
) {
    // for all entities are that are waiting to be loaded, attempt to load them
    query.for_each_mut(|(entity, mut gltf_model, _)| {
        // attempt to get gltf, cancel if not found
        let gltf = gltfs.get(&gltf_model.handle);
        if gltf.is_none() { return }
        let gltf = gltf.unwrap();

        // mark gltf loaded
        let mut entity_commands = commands.entity(entity);
        entity_commands.insert(GLTFModelLoaded);

        // insert gltf into entity as children
        entity_commands.with_children(|builder| {
            gltf.scenes.iter().for_each(|scene| {
                builder.spawn(SceneBundle {
                    scene: scene.clone(),
                    transform: Transform::from_translation(gltf_model.offset).with_scale(gltf_model.scale),
                    ..Default::default()
                });
            });
        });

        // save animation map
        gltf.named_animations.iter().for_each(|(name, animation)| {
            gltf_model.animations.insert(name.clone(), animation.clone());
        })
    })
}

fn update_loaded_gltfs(
    mut query: Query<(Entity, &mut GLTFModel, &Children, With<GLTFModelLoaded>)>,
    scene_instances: Query<&SceneInstance>,
    mut players_query: Query<(Entity, &mut AnimationPlayer)>,
    scene_spawner: ResMut<SceneSpawner>,
    animation_clips: Res<Assets<AnimationClip>>,
    time: Res<Time>
) {
    // for each loaded gltfs
    query.for_each_mut(|(_, mut model, children, _)| {
        // if the current animation is over, clear current animation tracker
        if model.current_animation.is_some() {
            // get current animation clip
            let current_animation = model.current_animation.as_ref().unwrap();
            let current_animation_clip = model.animations.get(current_animation).unwrap();
            let current_animation_clip = animation_clips.get(current_animation_clip).unwrap();
            
            // if animation time has exceeded the animations clips duration, clear current animation
            let elapsed = time.elapsed_seconds() - model.animation_started_time;
            if elapsed >= current_animation_clip.duration() {
                model.current_animation = Option::None;
            }
        }

        // check if the model has no current animation but has animations in its queue
        if (model.current_animation.is_none() || model.force_next_animation) && model.next_animation.is_some() {
            // pop next animation
            let next_animation = model.next_animation.clone().unwrap();

            // make sure animation clip is found
            if model.animations.contains_key(&next_animation) {
                // get animation clip
                let next_animation_clip = model.animations.get(&next_animation).unwrap().clone();

                // play animation
                children.iter().for_each(|child| {
                    // get scene instance
                    let scene_instance = scene_instances.get(*child);
                    if scene_instance.is_err() { return }
                    let scene_instance = scene_instance.unwrap();

                    // get all players
                    scene_spawner.iter_instance_entities(**scene_instance).for_each(|player_entity| {
                        let player = players_query.get_mut(player_entity);
                        if player.is_err() { return }
                        let (_, mut player) = player.unwrap();
                        player.start_with_transition(next_animation_clip.clone(), Duration::from_millis(100));
                    });
                });
                
                // update start time
                model.animation_started_time = time.elapsed_seconds();

                // update force next animation
                model.force_next_animation = false;

                // update current animation tracker
                model.current_animation = Option::Some(next_animation);
            } else {
                error!("No animation named {}", next_animation);
            }
        }
    })
}