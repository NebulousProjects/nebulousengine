use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Test;

fn main() {
    let system = IntoSystem::into_system(test);
    let mut world = World::default();
    world.init_resource::<Test>();
    let id = world.register_system(system);
    let _ = world.run_system(id);
}

fn test(
    test: Res<Test>
) {
    println!("{:?}", test);
}
