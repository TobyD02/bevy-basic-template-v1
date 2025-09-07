use bevy::prelude::*;

#[cfg(feature="debug")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(feature="debug")]
use crate::ui::debug::{spawn_fps_text, update_fps_text};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature="debug")]
        app
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, spawn_fps_text)
            .add_systems(Update, update_fps_text);

    }
}