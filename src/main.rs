use avian2d::{
    math::{PI, Vector},
    prelude::*,
};
use bevy::{prelude::*, window::PrimaryWindow};
use snake::fabrik::{Joint, Limb, LimbSegment};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, follow_mouse)
        .run();
}

#[derive(Component)]
struct FirstSquare;

#[derive(Resource,Deref,DerefMut)]
struct LimbResource(Limb);

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
    let limb = Limb::new(Vec2 { x: 200.0, y: 200.0 }, 8, Vec2 { x: 0.0, y: 0.0 });

    limb.display(&mut commands, mesh, material);

    commands.insert_resource(LimbResource(limb));
}

fn setup_2(mut commands: Commands) {
    commands.spawn(Camera2d);

    let square_sprite = Sprite {
        color: Color::srgb(0.2, 0.7, 0.9),
        custom_size: Some(Vec2 { x: 10.0, y: 50.0 }),
        ..default()
    };

    let square_sprite_2 = Sprite {
        color: Color::srgb(0.9, 0.7, 0.9),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    let anchor = commands
        .spawn((
            square_sprite.clone(),
            RigidBody::Kinematic,
            FirstSquare, // AngularVelocity(1.5),
        ))
        .id();

    let object = commands
        .spawn((
            square_sprite_2,
            Transform::from_xyz(0.0, -100.0, 0.0),
            RigidBody::Dynamic,
            MassPropertiesBundle::from_shape(&Rectangle::from_length(50.0), 1.0),
        ))
        .id();

    commands.spawn(
        RevoluteJoint::new(anchor, object), // .with_local_anchor1(Vec2 { x: 5.0, y: 0.0 })
                                            // .with_local_anchor2(Vec2 { x: -25., y: 0.0 }), // .with_local_basis1(PI / 2.0)
    );
}

fn rotate_by_degrees_2(
    mut square: Single<&mut Transform, With<FirstSquare>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        square.rotate_z(PI / 2.0);
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        square.translation.x += 5.0;
    }
}
fn follow_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    joint_query: Query<(&mut Transform, &Joint),Without<LimbSegment>>,
    limb_query: Query<&mut Transform, With<LimbSegment>>,
    mut limb_resource:ResMut<LimbResource>
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
