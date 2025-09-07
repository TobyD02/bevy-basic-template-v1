use bevy::prelude::*;

pub const DEBUG_MODE: bool = true;

pub const GRAVITY: Vec2 = Vec2::new(0., -981.);
pub const MIN_SPEED: f32 = 1.;
pub const TEXTURE_ATLAS: &str = "monochrome_tilemap_transparent.png";

pub const TILE_SIZE: UVec2 = UVec2::splat(16);
pub const TILE_OFFSET: UVec2 = UVec2::splat(0);
pub const TILE_PADDING: UVec2= UVec2::splat(1);
pub const TILES_PER_ROW: u32 = 20;
pub const TILES_PER_COL: u32 = 20;
