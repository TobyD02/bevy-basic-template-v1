use bevy::prelude::*;
use crate::globals::{TEXTURE_ATLAS, TILES_PER_COL, TILES_PER_ROW, TILE_SIZE, TILE_PADDING, TILE_OFFSET};

#[derive(Bundle)]
pub struct AnimationControllerBundle {
    animation_timer: AnimationTimer,
    sprite: Sprite,
    animation_controller: AnimationController,
}

#[derive(Component)]
pub struct AnimationController {
    pub flip_x: bool,
    pub animations: Vec<Animation>,
    pub current_animation: usize,
}

pub struct Animation{
    pub frames: &'static [usize],
    pub current_pos: usize, // track which frame we're on
    pub name: &'static str,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationController {
    pub(crate) fn play_animation(&mut self, name: &str) {
        if let Some((i, _)) = self.animations.iter().enumerate().find(|(_, a)| a.name == name) {
            if self.current_animation == i {
                return
            }

            self.current_animation = i;
            self.animations[i].current_pos = 0;
        }
    }
}

impl AnimationControllerBundle {
    pub fn new(
        animations: Vec<Animation>,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {

        let texture_atlas: Handle<Image> = asset_server.load(TEXTURE_ATLAS);
        let layout = TextureAtlasLayout::from_grid(
            TILE_SIZE, TILES_PER_ROW, TILES_PER_COL, Some(TILE_PADDING), Some(TILE_OFFSET)
        );

        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let first_frame = animations
            .get(0)
            .map(|a| a.frames[0])
            .unwrap_or(0);

        return Self {
            sprite: Sprite::from_atlas_image(
                    texture_atlas,
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: first_frame
                }
            ),
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            animation_controller: AnimationController {
                flip_x: false,
                current_animation: 0,
                animations,
            },
        };
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationController, &mut AnimationTimer, &mut Sprite)>,
) {
    for (mut controller, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        // Mirror sprite if flip_x is set
        sprite.flip_x = controller.flip_x;

        let current_animation = controller.current_animation;
        if timer.just_finished() {
            // Get the currently active animation
            if let Some(animation) = controller.animations.get_mut(current_animation) {
                // Advance to the next frame, wrapping around
                animation.current_pos = (animation.current_pos + 1) % animation.frames.len();

                // Update the sprite atlas index
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = animation.frames[animation.current_pos];
                }
            }
        }
    }
}
