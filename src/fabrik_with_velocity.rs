use avian2d::prelude::LinearVelocity;
use bevy::{ecs::system::Query, math::Vec2};

pub fn forward_fabrik(limb_query:Query<&mut LinearVelocity>,target:Vec2) {
        // Set the last segment to target
        

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