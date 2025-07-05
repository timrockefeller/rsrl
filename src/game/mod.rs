use bevy::{image::ImageSampler, prelude::*, render::render_resource::Texture};
use bevy_rapier3d::prelude::*;

mod cuboid_uvcustom;
use cuboid_uvcustom::CuboidTiled;

#[derive(Component)]
struct Golem {}

fn make_golem_mesh_head() -> CuboidTiled {
    CuboidTiled {
        half_size: Vec3::new(0.5, 0.5, 0.5),
        tile_count: [2, 2],
        face: [[1, 0], [1, 1], [1, 1], [1, 1], [0, 1], [0, 1]],
    }
}
fn make_golem_mesh_body() -> CuboidTiled {
    CuboidTiled {
        half_size: Vec3::new(0.5, 0.5, 0.5),
        tile_count: [2, 2],
        face: [[0, 0]; 6],
    }
}

fn setup_scene(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    cmd.spawn((
        Collider::cuboid(100.0, 0.1, 100.0),
        Transform::from_xyz(0.0, -2.0, 0.0),
    ));

    let texture_handle = asset_server.load::<Image>("golem_tex.png");

    const START_POS: Vec3 = Vec3::new(0.0, 5.0, 0.0);
    const GOLEM_OFFSET: [Vec3; 5] = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -2.0, 0.0),
        Vec3::new(1.0, -1.0, 0.0),
        Vec3::new(-1.0, -1.0, 0.0),
    ];

    let parent = cmd
        .spawn((
            Golem {},
            RigidBody::Dynamic,
            Collider::cuboid(0.49, 0.49, 0.49),
            Transform::from_translation(START_POS + GOLEM_OFFSET[0]),
            Mesh3d(meshes.add(make_golem_mesh_head())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                unlit: true,
                ..Default::default()
            })),
        ))
        .id();

    for i in 1..5 {
        cmd.spawn((
            RigidBody::Dynamic,
            Collider::cuboid(0.49, 0.49, 0.49),
            Transform::from_translation(START_POS + GOLEM_OFFSET[i]),
            ImpulseJoint::new(
                parent,
                FixedJointBuilder::new().local_anchor1(GOLEM_OFFSET[i]),
            ),
            // Transform::from_xyz(0.0, -1.0, 0.0),
            Mesh3d(meshes.add(make_golem_mesh_body())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                unlit: true,
                ..Default::default()
            })),
        ));
    }
}

fn on_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,

    mut query: Query<&mut Transform, With<Golem>>,
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
