use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use crate::globals::DEBUG_MODE;
use crate::ui::debug::{spawn_fps_text, update_fps_text};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if DEBUG_MODE {
            app
                .add_plugins(FrameTimeDiagnosticsPlugin::default())
                .add_systems(Startup, spawn_fps_text)
                .add_systems(Update, update_fps_text);
        }
    }
}