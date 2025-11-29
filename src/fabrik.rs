use bevy::prelude::*;

#[derive(Component)]
pub struct Joint(usize);

#[derive(Component)]
pub struct LimbSegment;
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
    segments: Vec<Segment>,
    target: Vec2,
}

impl Limb {
    pub fn new(target: Vec2, no_of_segments: usize, starting_position: Vec2) -> Self {
        let mut segments = Vec::<Segment>::new();
        let mut sum = 0.0;
        let length = 50.0;

        for _ in 0..no_of_segments {
            segments.push(Segment::new(
                Vec2 {
                    x: starting_position.x,
                    y: starting_position.y + sum,
                },
                length,
            ));
            sum += length;
        }
        Self { segments, target }
    }
    pub fn display(
        &self,
        commands: &mut Commands,
        circle_mesh: Handle<Mesh>,
        circle_material: Handle<ColorMaterial>,
    ) {
        let first_position = self.segments[0].position;
        let line_sprite = Sprite {
            color: Color::srgb(0.2, 0.7, 0.9),
            custom_size: Some(Vec2 { x: 50.0, y: 5.0 }),
            ..default()
        };

        commands.spawn((
            Mesh2d(circle_mesh.clone()),
            MeshMaterial2d(circle_material.clone()),
            Transform::from_xyz(first_position.x, first_position.y, 0.0),
            Joint(0),
        ));

        for i in 0..self.segments.len() - 1 {
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
                LimbSegment,
            ));
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
        mut joint_query:Query<(&mut Transform, &Joint),Without<LimbSegment>>,
        mut limb_query:Query<&mut Transform, With<LimbSegment>>,
    ) {
        for (mut transform, segment_id) in joint_query.iter_mut() {
            let segment = &self.segments[segment_id.0];
            transform.translation = segment.position.extend(0.0);
        }

        for (i,mut transform) in limb_query.iter_mut().enumerate() {
            let start_point = self.segments[i].position;
            let end_point = self.segments[i + 1].position;

            let direction = start_point - end_point;
            let angle = direction.y.atan2(direction.x);
            let midpoint = (start_point + end_point) / 2.0;
            transform.translation=midpoint.extend(0.0);
            transform.rotation=Quat::from_rotation_z(angle);
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
            direction = direction.normalize() * current_length;

            self.segments[i].set_position(next_pos - direction);
        }
    }
    
    pub fn set_target(&mut self, target: Vec2) {
        self.target = target;
    }
}
