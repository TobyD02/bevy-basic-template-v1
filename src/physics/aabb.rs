use bevy::color::palettes::basic::RED;
use bevy::prelude::*;
use crate::physics::body::Body;

#[derive(Component)]
pub struct AABB {
    pub width: f32,
    pub height: f32,
}

impl AABB {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Check collision using center-based positions
    pub fn is_colliding(&self, center: Vec2, other: &AABB, other_center: Vec2) -> bool {
        let dx = (center.x - other_center.x).abs();
        let dy = (center.y - other_center.y).abs();
        dx < (self.width / 2.0 + other.width / 2.0) &&
            dy < (self.height / 2.0 + other.height / 2.0)
    }

    /// Returns the minimum translation vector to resolve a collision
    pub fn get_penetration_vector(&self, center: Vec2, other: &AABB, other_center: Vec2) -> Option<Vec2> {
        let dx = other_center.x - center.x;
        let dy = other_center.y - center.y;

        let px = (self.width / 2.0 + other.width / 2.0) - dx.abs();
        let py = (self.height / 2.0 + other.height / 2.0) - dy.abs();

        if px <= 0.0 || py <= 0.0 {
            return None;
        }

        if px < py {
            Some(Vec2::new(if dx > 0.0 { -px } else { px }, 0.0))
        } else {
            Some(Vec2::new(0.0, if dy > 0.0 { -py } else { py }))
        }
    }
}

pub fn resolve_bodies(mut query: Query<(&AABB, &Transform, &mut Body)>) {
    const MIN_TRANSLATION: f32 = 0.1; // small sticky buffer

    let mut iter = query.iter_combinations_mut();
    while let Some([
                   (col_a, transform_a, mut body_a),
                   (col_b, transform_b, mut body_b)
                   ]) = iter.fetch_next() {
        // Use predicted (next) positions for dynamic bodies, current transform for static
        let pos_a = if body_a.dynamic {
            // next_translation was set by update_bodies (semi-implicit Euler)
            body_a.next_translation.truncate()
        } else {
            transform_a.translation.truncate()
        };

        let pos_b = if body_b.dynamic {
            body_b.next_translation.truncate()
        } else {
            transform_b.translation.truncate()
        };

        // Compute penetration based on predicted/current positions
        if let Some(penetration) = col_a.get_penetration_vector(pos_a, col_b, pos_b) {
            // Always set flags based on the original penetration (sign indicates direction).
            // We'll then apply translation only along the axis of minimum penetration.
            let set_flags_single = |p: Vec2, body: &mut Body, sign_mult: f32| {
                // sign_mult = 1.0 if p is the delta to move this body, -1.0 if it's the opposite body
                let px = p.x * sign_mult;
                let py = p.y * sign_mult;
                if px > 0.0 { body.colliding_left = true; }
                else if px < 0.0 { body.colliding_right = true; }
                if py > 0.0 { body.colliding_down = true; }
                else if py < 0.0 { body.colliding_up = true; }
            };

            // decide which axis to resolve
            if penetration.x.abs() < penetration.y.abs() {
                // resolve X only
                let desired_move = penetration.x; // amount to move A to resolve against B
                // compute applied move that leaves MIN_TRANSLATION remaining (if desired)
                let applied = if desired_move.abs() > MIN_TRANSLATION {
                    // move so remaining penetration == MIN_TRANSLATION (preserve sign)
                    desired_move - desired_move.signum() * MIN_TRANSLATION
                } else {
                    0.0
                };

                match (body_a.dynamic, body_b.dynamic) {
                    (true, false) => {
                        // move A by 'applied'
                        body_a.next_translation.x += applied;
                        set_flags_single(penetration, &mut body_a, 1.0);
                    }
                    (false, true) => {
                        // move B by -applied
                        body_b.next_translation.x -= applied;
                        set_flags_single(penetration, &mut body_b, -1.0);
                    }
                    (true, true) => {
                        // split resolution
                        body_a.next_translation.x += applied * 0.5;
                        body_b.next_translation.x -= applied * 0.5;
                        set_flags_single(penetration * 0.5, &mut body_a, 1.0);
                        set_flags_single(penetration * 0.5, &mut body_b, -1.0);
                    }
                    (false, false) => {}
                }
            } else {
                // resolve Y only
                let desired_move = penetration.y;
                let applied = if desired_move.abs() > MIN_TRANSLATION {
                    desired_move - desired_move.signum() * MIN_TRANSLATION
                } else {
                    0.0
                };

                match (body_a.dynamic, body_b.dynamic) {
                    (true, false) => {
                        body_a.next_translation.y += applied;
                        set_flags_single(penetration, &mut body_a, 1.0);
                    }
                    (false, true) => {
                        body_b.next_translation.y -= applied;
                        set_flags_single(penetration, &mut body_b, -1.0);
                    }
                    (true, true) => {
                        body_a.next_translation.y += applied * 0.5;
                        body_b.next_translation.y -= applied * 0.5;
                        set_flags_single(penetration * 0.5, &mut body_a, 1.0);
                        set_flags_single(penetration * 0.5, &mut body_b, -1.0);
                    }
                    (false, false) => {}
                }
            }
        }
    }
}

pub fn reset_collision_flags(mut query: Query<&mut Body>) {
    for mut body in &mut query {
        body.colliding_left = false;
        body.colliding_right = false;
        body.colliding_up = false;
        body.colliding_down = false;
    }
}

#[cfg(feature="debug")]
pub fn draw_debug_colliders(mut gizmos: Gizmos, query: Query<(&AABB, &Transform)>) {
    for (aabb, transform) in &query {
        let center = transform.translation.truncate();
        gizmos.rect_2d(center, Vec2::new(aabb.width, aabb.height), RED);
    }
}
