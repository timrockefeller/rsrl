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

const IMPLUSE_ADDITION_POS_FROM: [f32; 5] = [0f32, 1f32, 1f32, 0f32, 0f32];
const IMPLUSE_ADDITION_POS_TO: [f32; 5] = [0f32, 0f32, 0f32, 1f32, 1f32];
// 0-1
#[derive(Component)]
struct GolemImpluseMovement {
    alpha: f32, // 0-1 具体偏移
    blend: f32, // 0-1 预表现应用偏移
    index: usize,
    stiffness: f32,
}
impl Default for GolemImpluseMovement {
    fn default() -> Self {
        Self {
            alpha: 0.0,
            blend: 0.0,
            index: 0,
            stiffness: 1000.0,
        }
    }
}
impl GolemImpluseMovement {
    fn from_index(index: usize) -> Self {
        Self {
            index,
            ..Default::default()
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

    cmd.spawn((
        GolemImpluseMovement::from_index(1),
        RigidBody::Dynamic,
        Collider::cuboid(0.49, 0.49, 0.49),
        Transform::from_translation(START_POS + GOLEM_OFFSET[1]),
        ImpulseJoint::new(
            parent,
            PrismaticJointBuilder::new(Vec3::Z)
                .local_anchor1(Vec3::ZERO)
                .local_anchor2(-GOLEM_OFFSET[1])
                .limits([0.0, 1.0])
                .motor_position(0.0, 512.0, 0.0),
        ),
        Mesh3d(meshes.add(make_golem_mesh_body())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            unlit: true,
            ..Default::default()
        })),
        children![(
            Collider::cuboid(0.49, 0.49, 0.49),
            Transform::from_translation(GOLEM_OFFSET[2] - GOLEM_OFFSET[1]),
            Mesh3d(meshes.add(make_golem_mesh_body())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                unlit: true,
                ..Default::default()
            })),
        )],
    ));
    cmd.spawn((
        GolemImpluseMovement::from_index(3),
        RigidBody::Dynamic,
        Collider::cuboid(0.49, 0.49, 0.49),
        Transform::from_translation(START_POS + GOLEM_OFFSET[3]),
        ImpulseJoint::new(
            parent,
            PrismaticJointBuilder::new(Vec3::Z)
                .local_anchor1(Vec3::ZERO)
                .local_anchor2(-GOLEM_OFFSET[3])
                .limits([0.0, 1.0])
                .motor_position(0.0, 512.0, 0.0),
        ),
        Mesh3d(meshes.add(make_golem_mesh_body())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            unlit: true,
            ..Default::default()
        })),
        children![(
            Collider::cuboid(0.49, 0.49, 0.49),
            Transform::from_translation(GOLEM_OFFSET[4] - GOLEM_OFFSET[3]),
            Mesh3d(meshes.add(make_golem_mesh_body())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                unlit: true,
                ..Default::default()
            })),
        )],
    ));
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

#[derive(Event)]
struct MovementDirty(Entity);

fn handle_move_body_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut GolemImpluseMovement)>,
    time: Res<Time>,
    mut event: EventWriter<MovementDirty>,
) {
    for (e, mut mov) in query.iter_mut() {
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
        // emit changed event
        if changed {
            mov.blend = f32::clamp(mov.blend + time.delta_secs() * ALPHA_SPEED, 0.0, 1.0);
            event.write(MovementDirty(e));
        }
    }
}

fn handle_movement(
    mut query: Query<(&GolemImpluseMovement, &mut ImpulseJoint)>,
    mut events: EventReader<MovementDirty>,
) {
    for event in events.read() {
        let result = query.get_mut(event.0);
        if let Ok((mov, mut joint)) = result {
            match &mut joint.data {
                TypedJoint::PrismaticJoint(prism) => {
                    prism.set_motor_position(
                        f32::lerp(
                            IMPLUSE_ADDITION_POS_FROM[mov.index],
                            IMPLUSE_ADDITION_POS_TO[mov.index],
                            mov.alpha,
                        ) * mov.blend,
                        mov.stiffness,
                        0.0,
                    );
                    println!("alpha: {}, blend: {}", mov.alpha, mov.blend);
                }
                _ => {}
            }
        }
    }
}

pub struct GameModule;
impl Plugin for GameModule {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(camera::PlayerPlugin)
            .add_event::<MovementDirty>()
            .add_systems(Startup, setup_scene)
            .add_systems(PostUpdate, handle_on_reset)
            .add_systems(PostUpdate, (handle_move_body_key, handle_movement).chain());
    }
}
