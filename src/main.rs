use avian2d::prelude::*;
use bevy::{app::Animation, prelude::*, window::PrimaryWindow};
use rand::Rng;
use snake::fabrik::{Joint, JointFilter, Limb, LimbFilter, LimbSegment};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, draw_snake_head)
        .add_systems(Update, follow_mouse)
        .add_systems(Update, move_snake)
        .add_systems(Update, detect_collision_with_apple)
        .add_systems(Update, execute_animations)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct LimbResource(Limb);

#[derive(Resource)]
struct CircleMeshAndMaterial{
    mesh:Handle<Mesh>,
    material:Handle<ColorMaterial>
}


#[derive(Resource, Deref, DerefMut)]
struct SnakeVelocity(Vec2);

#[derive(Component)]
pub struct Apple;


#[derive(Component)]
struct AnimationTimer{
    frame_count:usize,
    timer:Timer

}

const SNAKE_SPEED: f32 = 10.0;

const NO_OF_SNAKE_PARTS: usize = 10;

fn draw_snake_head(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
){
    let shape = Rectangle::new(10.0,5.0);
    let mesh = meshes.add(shape);
    let color = Color::Srgba(Srgba::rgb(1.0, 0.647, 0.0));
    let material = materials.add(color);

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(20.0, -50.0, 0.0),
        // RigidBody::Kinematic,
        // Collider::circle(15.0),
        // Sensor,
        // Apple,
    ));

    let texture = asset_server.load("sprites/snake_mouth_sprite.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2{
        x:33,
        y:53
    }, 15, 1, Some(UVec2{
        x:3,
        y:0
    }), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(-70.0, 0.0, 0.0)),
        AnimationTimer{
            frame_count:15,
            timer:Timer::from_seconds(0.125, TimerMode::Repeating)
        }
    ));

}


fn execute_animations(time: Res<Time>, mut query: Query<( &mut AnimationTimer,&mut Sprite)>) {

    for (mut config, mut sprite) in &mut query {
        config.timer.tick(time.delta());
        if config.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index+=1;
            if atlas.index == config.frame_count{
                atlas.index=0;
            }
        }
    }


}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Circle::new(5.0);
    let mesh = meshes.add(shape);
    let color = Color::Srgba(Srgba::rgb(1.0, 0.647, 0.0));
    let material = materials.add(color);
    commands.insert_resource(CircleMeshAndMaterial{
        mesh:mesh.clone(),
        material:material.clone()
    });
    commands.spawn(Camera2d);
    let limb = Limb::new(
        Vec2 { x: 200.0, y: 200.0 },
        NO_OF_SNAKE_PARTS,
        Vec2 { x: 200.0, y: -200.0 },
    );

    limb.display(&mut commands, mesh, material);

    commands.insert_resource(LimbResource(limb));

    commands.insert_resource(SnakeVelocity(Vec2 {
        x: 0.0,
        y: SNAKE_SPEED - 10.0,
    }));

    let apple_shape = Circle::new(15.0);
    let apple_mesh = meshes.add(apple_shape);
    let apple_color = Color::Srgba(Srgba::rgb(0.0, 0.647, 0.0));

    let apple_material = materials.add(apple_color);
    commands.spawn((
        Mesh2d(apple_mesh),
        MeshMaterial2d(apple_material),
        Transform::from_xyz(20.0, 50.0, 0.0),
        RigidBody::Kinematic,
        Collider::circle(15.0),
        Sensor,
        Apple,
    ));
}

fn move_snake(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    joint_query: Query<(&mut Transform, &Joint), JointFilter>,
    limb_query: Query<(&mut Transform, &LimbSegment), LimbFilter>,
    mut limb_resource: ResMut<LimbResource>,
    mut snake_velocity: ResMut<SnakeVelocity>,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        snake_velocity.0 = Vec2 {
            x: -SNAKE_SPEED,
            y: 0.0,
        };
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        snake_velocity.0 = Vec2 {
            x: SNAKE_SPEED,
            y: 0.0,
        };
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        snake_velocity.0 = Vec2 {
            x: 0.0,
            y: SNAKE_SPEED,
        };
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        snake_velocity.0 = Vec2 {
            x: 0.0,
            y: -SNAKE_SPEED,
        };
    }
    if snake_velocity.0.length() == 0.0 {
        return;
    }
    let target = limb_resource.get_last_segment_position() + snake_velocity.0;
    limb_resource.set_target(target);
    limb_resource.forward_fabrik();
    limb_resource.update_visuals(joint_query, limb_query);
}
fn follow_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    joint_query: Query<(&mut Transform, &Joint), JointFilter>,
    limb_query: Query<(&mut Transform, &LimbSegment), LimbFilter>,
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

fn detect_collision_with_apple(
    mut collision_reader: MessageReader<CollisionEnd>,
    mut apple: Single<&mut Transform, With<Apple>>,
    mut joints_query:Query<&mut Joint>,
    mut limb_query:Query<&mut LimbSegment>,
    mut limb_resource: ResMut<LimbResource>,
    mut commands: Commands,
    circle_mesh_and_material:Res<CircleMeshAndMaterial>
) {
    let no_of_snake_parts_to_add=10;
    for event in collision_reader.read() {
        for mut joint in joints_query.iter_mut(){
            joint.0+=no_of_snake_parts_to_add;

        }
        for mut limb in limb_query.iter_mut(){
            limb.0+=no_of_snake_parts_to_add;
        }
        limb_resource.add_multiple_snake_parts(no_of_snake_parts_to_add, &mut commands, circle_mesh_and_material.mesh.clone(), circle_mesh_and_material.material.clone());

        let mut rng = rand::rng();
        let x: f32 = rng.random_range(-300.0..=300.0);
        let y: f32 = rng.random_range(-300.0..=300.0);
        apple.translation.x = x;
        apple.translation.y = y;
    }
}
