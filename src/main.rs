use avian2d::
    prelude::*
;
use bevy::{prelude::*, window::PrimaryWindow};
use snake::fabrik::{JointFilter, Limb, LimbFilter};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, follow_mouse)
        .add_systems(Update, move_snake)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct LimbResource(Limb);

#[derive(Resource, Deref, DerefMut)]
struct SnakeVelocity(Vec2);

const SNAKE_SPEED:f32=10.0;

const NO_OF_SNAKE_PARTS:usize=8;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Circle::new(5.0);
    let mesh = meshes.add(shape);
    let color = Color::Srgba(Srgba::rgb(1.0, 0.647, 0.0));
    let material = materials.add(color);
    commands.spawn(Camera2d);
    let limb = Limb::new(Vec2 { x: 200.0, y: 200.0 }, NO_OF_SNAKE_PARTS, Vec2 { x: 0.0, y: 0.0 });

    limb.display(&mut commands, mesh, material);

    commands.insert_resource(LimbResource(limb));

    commands.insert_resource(SnakeVelocity(Vec2 { x: 0.0, y: SNAKE_SPEED-5.0}));


    let apple_shape = Circle::new(10.0);
    let apple_mesh = meshes.add(shape);
    let apple_color = Color::Srgba(Srgba::rgb(0.0, 0.647, 0.0));

    let apple_material = materials.add(color);
    commands.spawn((
            Mesh2d(apple_mesh),
            MeshMaterial2d(apple_material),
            Transform::from_xyz(20.0, 50.0, 0.0),
        ));
}





fn move_snake(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    joint_query: Query<&mut Transform, JointFilter>,
    limb_query: Query<&mut Transform, LimbFilter>,
    mut limb_resource: ResMut<LimbResource>,
    mut snake_velocity: ResMut<SnakeVelocity>,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        snake_velocity.0 = Vec2 { x: -SNAKE_SPEED, y: 0.0 };
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        snake_velocity.0 = Vec2 { x: SNAKE_SPEED, y: 0.0 };
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        snake_velocity.0 = Vec2 { x: 0.0, y: SNAKE_SPEED };
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        snake_velocity.0 = Vec2 { x: 0.0, y: -SNAKE_SPEED };
    }    
    let target = limb_resource.get_last_segment_position()+snake_velocity.0;
    limb_resource.set_target(target);
    limb_resource.forward_fabrik();
    limb_resource.update_visuals(joint_query, limb_query);
}
fn follow_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    joint_query: Query<&mut Transform, JointFilter>,
    limb_query: Query<&mut Transform, LimbFilter>,
    mut limb_resource: ResMut<LimbResource>,
) -> Result {
    if buttons.pressed(MouseButton::Left) {
        let window = windows.single()?;
        let (camera, camera_transform) = camera.single()?;

        if let Some(cursor_world_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            limb_resource.set_target(cursor_world_pos);

            limb_resource.forward_fabrik();
            limb_resource.update_visuals(joint_query, limb_query);
        }
    }

    Ok(())
}
