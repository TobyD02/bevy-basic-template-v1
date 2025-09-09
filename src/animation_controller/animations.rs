use crate::animation_controller::animation_controller::Animation;
pub const PLAYER_IDLE: Animation = Animation {
    frames: &[240],
    name: "player_idle",
    current_pos: 0,
};

pub const PLAYER_RUN: Animation = Animation {
    frames: &[241, 242, 243, 244],
    name: "player_run",
    current_pos: 0,
};

pub const PLAYER_JUMP: Animation = Animation {
    frames: &[245],
    name: "player_jump",
    current_pos: 0,
};

pub const PLAYER_DEAD: Animation = Animation {
    frames: &[246],
    name: "player_dead",
    current_pos: 0,
};

pub const GROUND_CORNER: Animation = Animation {
    frames: &[110],
    name: "ground_corner",
    current_pos: 0,
};
