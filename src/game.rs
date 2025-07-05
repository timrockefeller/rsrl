use bevy::{prelude::*, render::primitives::Sphere};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Golem {}

fn setup_scene(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    /* Create the ground. */
    cmd.spawn((
        Collider::cuboid(100.0, 0.1, 100.0),
        Transform::from_xyz(0.0, -2.0, 0.0),
    ));

    /* Create the bouncing ball. */
    cmd.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Transform {translation: Vec3::new(0.0, 10.0, 0.0), rotation: Quat::from_rotation_x(3.0), ..Default::default()},
        Mesh3d(meshes.add(Mesh::from(Cuboid::from_size(Vec3::new(1.0, 1.0, 1.0))))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.0, 1.0, 0.0)))
    ));
}

fn on_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,

    mut query: Query<&mut Transform, With<RigidBody>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            transform.translation.y += 1.0;
            transform.translation.x = 0.0;
            transform.translation.z = 0.0;
        }
    }
}

pub struct GameModule;
impl Plugin for GameModule {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup_scene)
            .add_systems(PostUpdate, on_keyboard_input);
    }
}
