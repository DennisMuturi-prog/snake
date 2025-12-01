use std::collections::VecDeque;

use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Joint(pub usize);

pub type JointFilter = (With<Joint>, Without<LimbSegment>);

pub type LimbFilter = (With<LimbSegment>, Without<Joint>);

const SNAKE_PART_LENGTH: f32 = 20.0;

const SNAKE_PART_THICKNESS: f32 = 5.0;
pub const SNAKE_HEAD_THICKNESS: f32 = 50.0;
pub const SNAKE_HEAD_LENGTH: f32 = 50.0;

#[derive(Component)]
pub struct LimbSegment(pub usize);

#[derive(Component)]
pub struct HeadOfSnake;
pub struct Segment {
    position: Vec2,
    length: f32,
}

impl Segment {
    pub fn new(position: Vec2, length: f32) -> Self {
        Self { position, length }
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
}

pub struct Limb {
    segments: VecDeque<Segment>,
    target: Vec2,
}

impl Limb {
    pub fn new(target: Vec2, no_of_segments: usize, starting_position: Vec2) -> Self {
        let mut segments = VecDeque::<Segment>::new();
        let mut sum = 0.0;

        for i in 0..no_of_segments {
            if i == no_of_segments - 2 {
                segments.push_back(Segment::new(
                    Vec2 {
                        x: starting_position.x + sum,
                        y: starting_position.y,
                    },
                    SNAKE_HEAD_LENGTH,
                ));
                sum -= SNAKE_HEAD_LENGTH;
            } else {
                segments.push_back(Segment::new(
                    Vec2 {
                        x: starting_position.x + sum,
                        y: starting_position.y,
                    },
                    SNAKE_PART_LENGTH,
                ));
                sum -= SNAKE_PART_LENGTH;
            }
        }

        Self { segments, target }
    }
    pub fn display<S: Bundle>(
        &self,
        commands: &mut Commands,
        circle_mesh: Handle<Mesh>,
        circle_material: Handle<ColorMaterial>,
        snake_bundle: S,
    ) {
        let first_position = self.segments[0].position;
        let line_sprite = Sprite {
            color: Color::srgb(0.2, 0.7, 0.9),
            custom_size: Some(Vec2 {
                x: SNAKE_PART_LENGTH,
                y: SNAKE_PART_THICKNESS,
            }),
            ..default()
        };

        let second_last_index = self.segments.len() - 2;

        commands.spawn((
            Mesh2d(circle_mesh.clone()),
            MeshMaterial2d(circle_material.clone()),
            Transform::from_xyz(first_position.x, first_position.y, 0.0),
            Joint(0),
        ));

        let start_point = self.segments[second_last_index].position;
        let end_point = self.segments[second_last_index + 1].position;
        let midpoint = (start_point + end_point) / 2.0;

        commands.spawn((
            Transform {
                translation: midpoint.extend(0.0),
                ..default()
            },
            LimbSegment(second_last_index),
            HeadOfSnake,
            RigidBody::Kinematic,
            Collider::rectangle(SNAKE_HEAD_LENGTH, SNAKE_HEAD_THICKNESS),
            CollisionEventsEnabled,
            snake_bundle,
        ));

        for i in 0..self.segments.len() - 1 {
            let start_point = self.segments[i].position;
            let end_point = self.segments[i + 1].position;
            let midpoint = (start_point + end_point) / 2.0;
            if i != second_last_index {
                commands.spawn((
                    line_sprite.clone(),
                    Transform {
                        translation: midpoint.extend(0.0),
                        ..default()
                    },
                    LimbSegment(i),
                ));
            }

            commands.spawn((
                Mesh2d(circle_mesh.clone()),
                MeshMaterial2d(circle_material.clone()),
                Transform::from_xyz(end_point.x, end_point.y, 0.0),
                Joint(i + 1),
            ));
        }
    }
    pub fn update_visuals(
        &self,
        mut joint_query: Query<(&mut Transform, &Joint), JointFilter>,
        mut limb_query: Query<(&mut Transform, &LimbSegment), LimbFilter>,
    ) {
        for (mut transform, joint) in joint_query.iter_mut() {
            if let Some(segment) = self.segments.get(joint.0) {
                transform.translation = segment.position.extend(0.0);
            }
        }

        for (mut transform, limb_segment) in limb_query.iter_mut() {
            let i = limb_segment.0;
            let start_point = self.segments[i].position;
            let end_point = self.segments[i + 1].position;

            let direction = start_point - end_point;

            let angle = direction.y.atan2(direction.x);
            let midpoint = (start_point + end_point) / 2.0;

            transform.translation = midpoint.extend(0.0);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }

    pub fn forward_fabrik(&mut self) {
        let len = self.segments.len();
        if len == 0 {
            return;
        }

        // Set the last segment to target
        self.segments[len - 1].set_position(self.target);

        // Work backwards from second-to-last to first
        for i in (0..len - 1).rev() {
            let next_pos = self.segments[i + 1].position;
            let current_pos = self.segments[i].position;
            let current_length = self.segments[i].length();

            let mut direction = next_pos - current_pos;
            direction = direction.normalize_or_zero() * current_length;

            self.segments[i].set_position(next_pos - direction);
        }
    }
    pub fn get_last_segment_position(&self) -> Vec2 {
        let last_index = self.segments.len() - 1;
        self.segments[last_index].position
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.target = target;
    }
    pub fn add_multiple_snake_parts(
        &mut self,
        no_of_parts: usize,
        commands: &mut Commands,
        circle_mesh: Handle<Mesh>,
        circle_material: Handle<ColorMaterial>,
    ) {
        let line_sprite = Sprite {
            color: Color::srgb(0.2, 0.7, 0.9),
            custom_size: Some(Vec2 {
                x: SNAKE_PART_LENGTH,
                y: SNAKE_PART_THICKNESS,
            }),
            ..default()
        };

        for _ in 0..no_of_parts {
            self.add_snake_part();
        }
        let first_position = self.segments[0].position;
        commands.spawn((
            Mesh2d(circle_mesh.clone()),
            MeshMaterial2d(circle_material.clone()),
            Transform::from_xyz(first_position.x, first_position.y, 0.0),
            Joint(0),
        ));

        for i in 0..no_of_parts {
            let start_point = self.segments[i].position;
            let end_point = self.segments[i + 1].position;

            let direction = start_point - end_point;
            let angle = direction.y.atan2(direction.x);
            let midpoint = (start_point + end_point) / 2.0;

            commands.spawn((
                line_sprite.clone(),
                Transform {
                    translation: midpoint.extend(0.0),
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                },
                LimbSegment(i),
            ));

            if i < no_of_parts - 1 {
                commands.spawn((
                    Mesh2d(circle_mesh.clone()),
                    MeshMaterial2d(circle_material.clone()),
                    Transform::from_xyz(end_point.x, end_point.y, 0.0),
                    Joint(i + 1),
                ));
            }
        }
    }

    pub fn add_snake_part(&mut self) {
        let start_point = self.segments[0].position;
        let end_point = self.segments[1].position;
        let direction = start_point - end_point;
        let new_point = start_point + direction;
        self.segments
            .push_front(Segment::new(new_point, SNAKE_PART_LENGTH));
    }
}
