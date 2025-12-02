use std::time::Duration;

use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;
use snake::fabrik::{
    HeadOfSnake, Joint, JointFilter, Limb, LimbFilter, LimbSegment, SNAKE_HEAD_LENGTH,
    SNAKE_HEAD_THICKNESS,
};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin,
        ))
        .add_systems(Startup, (setup, draw_snake_head).chain())
        .add_systems(Startup, draw_boundaries)
        .add_systems(Update, follow_mouse)
        .add_systems(Update, move_snake)
        .add_systems(
            Update,
            (
                detect_start_collision_with_apple_field,
                detect_collision_with_apple,
                detect_start_collision_with_apple,
                trigger_tounge_and_eyes_animation,
                detect_start_collision_with_boundary,
            ),
        )
        .add_systems(Update, execute_animations)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct LimbResource(Limb);

#[derive(Resource, Deref, DerefMut)]
struct ToungeAndEyesAnimationTimer(Timer);

#[derive(Resource)]
struct CircleMeshAndMaterial {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct HitAnimationTextureAndAtlas {
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Deref, DerefMut)]
struct SnakeVelocity(Vec2);

#[derive(Resource, Deref, DerefMut)]
struct CrunchSound(Handle<AudioSource>);

#[derive(Resource, Deref, DerefMut)]
struct HitSound(Handle<AudioSource>);

#[derive(Component)]
pub struct Apple;

#[derive(Component)]
pub struct AppleField;

#[derive(Component)]
pub struct Mouth;

#[derive(Component)]
pub struct Tongue;

#[derive(Component)]
pub struct Eye;

#[derive(Component)]
pub struct Boundary;

#[derive(Component)]
struct AnimationTimer {
    frame_count: usize,
    timer: Timer,
    fps: u8,
}
impl AnimationTimer {
    fn new(frame_count: usize, fps: u8) -> Self {
        Self {
            frame_count,
            fps,
            timer: AnimationTimer::timer_from_fps(fps),
        }
    }
    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

const SNAKE_SPEED: f32 = 10.0;

const NO_OF_SNAKE_PARTS: usize = 10;
const WALL_HEIGHT: f32 = 600.0;
const WALL_THICKNESS: f32 = 20.0;

const FLOOR_THICKNESS: f32 = WALL_THICKNESS;
const WALL_RIGHT_POSITION: f32 = 600.0;
fn draw_snake_head(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    limb_resource: Res<LimbResource>,
    circle_mesh_and_material: Res<CircleMeshAndMaterial>,
) {
    let texture = asset_server.load("sprites/snake_mouth_sprite.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2 { x: 33, y: 53 },
        15,
        1,
        Some(UVec2 { x: 3, y: 0 }),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mouth_bundle = (
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            flip_x: true,
            ..default()
        },
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(-15.0, 0.0, 10.0)),
        AnimationTimer::new(15, 30),
        Mouth,
    );

    let texture = asset_server.load("sprites/snake_tounge.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2 { x: 47, y: 22 },
        21,
        1,
        Some(UVec2 { x: 2, y: 2 }),
        Some(UVec2 { x: 0, y: 3 }),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let tounge_bundle = (
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            flip_x: true,
            ..default()
        },
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(-40.0, 0.0, 10.0)),
        AnimationTimer::new(21, 20),
        Tongue,
    );

    let texture = asset_server.load("sprites/snake_eye_sprite.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2 { x: 26, y: 28 },
        9,
        1,
        Some(UVec2 { x: 3, y: 0 }),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let eye_bundle1 = (
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            flip_x: true,
            ..default()
        },
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(15.0, 10.0, 10.0)),
        AnimationTimer::new(9, 20),
        Eye,
    );

    let eye_bundle2 = (
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            flip_x: true,
            ..default()
        },
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(15.0, -10.0, 10.0)),
        AnimationTimer::new(9, 20),
        Eye,
    );

    let texture = asset_server.load("sprites/snake_hit.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2 { x: 64, y: 53 },
        36,
        1,
        Some(UVec2 { x: 2, y: 0 }),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.insert_resource(HitAnimationTextureAndAtlas {
        texture,
        texture_atlas_layout,
    });
    let line_sprite = Sprite {
        color: Color::srgb(0.2, 0.7, 0.9),
        custom_size: Some(Vec2 {
            x: SNAKE_HEAD_LENGTH,
            y: SNAKE_HEAD_THICKNESS,
        }),
        ..default()
    };
    let snake_bundle = (
        line_sprite,
        children![tounge_bundle, mouth_bundle, eye_bundle1, eye_bundle2,],
    );
    limb_resource.display(
        &mut commands,
        circle_mesh_and_material.mesh.clone(),
        circle_mesh_and_material.material.clone(),
        snake_bundle,
    );
}

fn trigger_tounge_and_eyes_animation(
    mut tounge_and_eyes_animation_timer: ResMut<ToungeAndEyesAnimationTimer>,
    tounge_and_eyes_query: Query<&mut AnimationTimer, Or<(With<Tongue>, With<Eye>)>>,
    time: Res<Time>,
) {
    tounge_and_eyes_animation_timer.tick(time.delta());
    if tounge_and_eyes_animation_timer.just_finished() {
        for mut animation in tounge_and_eyes_query {
            animation.timer = AnimationTimer::timer_from_fps(animation.fps)
        }
    }
}

fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationTimer, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.timer.tick(time.delta());
        if config.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.frame_count - 1 {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = 0;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.timer = AnimationTimer::timer_from_fps(config.fps);
            }
        }
    }
}
fn draw_boundaries(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(WALL_THICKNESS, WALL_HEIGHT);
    let mesh = meshes.add(shape);
    let color = Color::Srgba(Srgba::rgb(1.0, 0.647, 0.0));
    let material = materials.add(color);
    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(-WALL_RIGHT_POSITION, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, WALL_HEIGHT),
        Boundary,
    ));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(WALL_RIGHT_POSITION, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, WALL_HEIGHT),
        Boundary,
    ));

    let shape = Rectangle::new(WALL_RIGHT_POSITION * 2.0, FLOOR_THICKNESS);
    let mesh = meshes.add(shape);

    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, WALL_HEIGHT / 2.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(WALL_RIGHT_POSITION * 2.0, FLOOR_THICKNESS),
        Boundary,
    ));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, -WALL_HEIGHT / 2.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(WALL_RIGHT_POSITION * 2.0, FLOOR_THICKNESS),
        Boundary,
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let shape = Circle::new(5.0);
    let mesh = meshes.add(shape);
    let color = Color::Srgba(Srgba::rgb(1.0, 0.647, 0.0));
    let material = materials.add(color);
    commands.insert_resource(CircleMeshAndMaterial {
        mesh: mesh.clone(),
        material: material.clone(),
    });
    commands.spawn(Camera2d);
    let limb = Limb::new(
        Vec2 { x: 200.0, y: 200.0 },
        NO_OF_SNAKE_PARTS,
        Vec2 {
            x: 200.0,
            y: -100.0,
        },
    );
    commands.insert_resource(LimbResource(limb));

    commands.insert_resource(SnakeVelocity(Vec2 {
        x: 0.0,
        y: SNAKE_SPEED - 10.0,
    }));

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/apple.png"),
            ..default()
        },
        Transform::from_xyz(20.0, 50.0, 0.0),
        RigidBody::Kinematic,
        Collider::circle(15.0),
        Sensor,
        Apple,
        children![(
            RigidBody::Kinematic,
            Collider::circle(150.0),
            Sensor,
            AppleField
        )],
    ));

    let apple_crunch_sound = asset_server.load("sounds/crunch.wav");
    commands.insert_resource(CrunchSound(apple_crunch_sound));

    let hit_sound = asset_server.load("sounds/hit.wav");
    commands.insert_resource(HitSound(hit_sound));

    commands.insert_resource(ToungeAndEyesAnimationTimer(Timer::from_seconds(
        5.0,
        TimerMode::Repeating,
    )));
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
    let right_bound_x = WALL_RIGHT_POSITION - WALL_THICKNESS / 2.0;
    let upper_bound_y = WALL_HEIGHT / 2.0 - FLOOR_THICKNESS / 2.0;
    let target = limb_resource.get_last_segment_position() + snake_velocity.0;
    if target.x >= right_bound_x
        || target.x <= -right_bound_x
        || target.y >= upper_bound_y
        || target.y <= -upper_bound_y
    {
        snake_velocity.0 = Vec2::ZERO;
    }
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
    mut apple: Single<(Entity, &mut Transform), With<Apple>>,
    mut joints_query: Query<&mut Joint>,
    mut limb_query: Query<&mut LimbSegment>,
    mut limb_resource: ResMut<LimbResource>,
    mut commands: Commands,
    circle_mesh_and_material: Res<CircleMeshAndMaterial>,
) {
    let no_of_snake_parts_to_add = 1;
    for event in collision_reader.read() {
        if event.collider1 != apple.0 && event.collider2 != apple.0 {
            continue;
        }
        for mut joint in joints_query.iter_mut() {
            joint.0 += no_of_snake_parts_to_add;
        }
        for mut limb in limb_query.iter_mut() {
            limb.0 += no_of_snake_parts_to_add;
        }
        limb_resource.add_multiple_snake_parts(
            no_of_snake_parts_to_add,
            &mut commands,
            circle_mesh_and_material.mesh.clone(),
            circle_mesh_and_material.material.clone(),
        );

        let mut rng = rand::rng();
        let right_bound_x = WALL_RIGHT_POSITION - WALL_THICKNESS / 2.0 - 20.0;
        let upper_bound_y = WALL_HEIGHT / 2.0 - FLOOR_THICKNESS / 2.0 - 20.0;
        let x: f32 = rng.random_range(-right_bound_x..=right_bound_x);
        let y: f32 = rng.random_range(-upper_bound_y..=upper_bound_y);
        apple.1.translation.x = x;
        apple.1.translation.y = y;
    }
}

fn detect_start_collision_with_apple(
    mut collision_reader: MessageReader<CollisionEnd>,
    apple: Single<Entity, With<Apple>>,
    mut commands: Commands,
    crunch_sound: Res<CrunchSound>,
) {
    let apple = apple.entity();
    for event in collision_reader.read() {
        if event.collider1 != apple && event.collider2 != apple {
            continue;
        }
        commands.spawn((AudioPlayer(crunch_sound.clone()), PlaybackSettings::DESPAWN));
    }
}

fn detect_start_collision_with_boundary(
    mut collision_reader: MessageReader<CollisionStart>,
    boundary: Query<Entity, With<Boundary>>,
    mut snake_head: Single<(Entity,&mut Sprite), With<HeadOfSnake>>,
    mut commands: Commands,
    hit_sound: Res<HitSound>,
    hit_animation: Res<HitAnimationTextureAndAtlas>,
) {
    for event in collision_reader.read() {
        if boundary.get(event.collider1).is_err() && boundary.get(event.collider2).is_err() {
            continue;
        }
        commands.spawn((AudioPlayer(hit_sound.clone()), PlaybackSettings::DESPAWN));
        snake_head.1.image = hit_animation.texture.clone();
        snake_head.1.texture_atlas = Some(TextureAtlas {
            layout: hit_animation.texture_atlas_layout.clone(),
            index: 0,
        });
        snake_head.1.flip_x=true;
        commands.entity(snake_head.0).insert(AnimationTimer::new(36, 10)).despawn_children();
    }
}

fn detect_start_collision_with_apple_field(
    mut collision_reader: MessageReader<CollisionStart>,
    mut mouth: Single<&mut AnimationTimer, With<Mouth>>,
    apple_field: Single<Entity, With<AppleField>>,
) {
    let apple_field = apple_field.entity();
    for event in collision_reader.read() {
        if event.collider1 != apple_field && event.collider2 != apple_field {
            continue;
        }
        mouth.timer = AnimationTimer::timer_from_fps(mouth.fps)
    }
}
