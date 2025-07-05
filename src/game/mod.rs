use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod cuboid_uvcustom;
use cuboid_uvcustom::CuboidTiled;
mod camera;

const ALPHA_SPEED: f32 = 3.0;
const START_POS: Vec3 = Vec3::new(0.0, 3.0, 0.0);
const GOLEM_OFFSET: [Vec3; 5] = [
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, -2.0, 0.0),
    Vec3::new(1.0, -1.0, 0.0),
    Vec3::new(-1.0, -1.0, 0.0),
];

const IMPLUSE_ADDITION_POS_FROM: [Vec3; 5] = [
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 0.0, 0.0),
];
const IMPLUSE_ADDITION_POS_TO: [Vec3; 5] = [
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 0.0, 1.0),
];
// 0-1
#[derive(Component)]
struct GolemImpluseMovement {
    alpha: f32, // 0-1 具体偏移
    blend: f32, // 0-1 预表现应用偏移
    index: usize,
}
impl Default for GolemImpluseMovement {
    fn default() -> Self {
        Self {
            alpha: 0.0,
            blend: 0.0,
            index: 0,
        }
    }
}
impl GolemImpluseMovement {
    fn from_index(index: usize) -> Self {
        Self {
            alpha: 0.0,
            blend: 0.0,
            index,
        }
    }
}

#[derive(Component)]
struct Golem;

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
        Collider::cuboid(100.0, 0.1, 100.0),
        Transform::from_xyz(0.0, -2.0, 0.0),
    ));

    let texture_handle = asset_server.load::<Image>("golem_tex.png");

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
            GolemImpluseMovement::from_index(0),
        ))
        .id();

    for i in 1..5 {
        cmd.spawn((
            GolemImpluseMovement::from_index(i),
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

fn handle_on_reset(
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

fn handle_move_body_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut GolemImpluseMovement, &mut ImpulseJoint)>,
    time: Res<Time>,
) {
    for (mut mov, mut joint) in query.iter_mut() {
        let mut changed = false;
        if keyboard_input.pressed(KeyCode::KeyE) {
            changed = true;
            if mov.blend == 0f32 {
                mov.alpha = 0.0;
            }
            mov.alpha = f32::clamp(mov.alpha + time.delta_secs() * ALPHA_SPEED, 0.0, 1.0);
        } else if keyboard_input.pressed(KeyCode::KeyQ) {
            changed = true;
            if mov.blend == 0f32 {
                mov.alpha = 1.0;
            }
            mov.alpha = f32::clamp(mov.alpha - time.delta_secs() * ALPHA_SPEED, 0.0, 1.0);
        }
        if changed {
            mov.blend = f32::clamp(mov.blend + time.delta_secs(), 0.0, 1.0);
            joint.data = FixedJointBuilder::new()
                .local_anchor1(
                    GOLEM_OFFSET[mov.index]
                        + Vec3::lerp(
                            IMPLUSE_ADDITION_POS_FROM[mov.index],
                            IMPLUSE_ADDITION_POS_TO[mov.index],
                            mov.alpha,
                        ) * mov.blend,
                )
                .into();
        }
    }
}

pub struct GameModule;
impl Plugin for GameModule {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(camera::PlayerPlugin)
            .add_systems(Startup, setup_scene)
            .add_systems(PostUpdate, handle_on_reset)
            .add_systems(PostUpdate, handle_move_body_key);
    }
}
