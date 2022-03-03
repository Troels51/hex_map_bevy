
mod loading;
mod world;
use crate::loading::LoadingPlugin;
use std::ops::Index;

use bevy::{ecs::schedule::SystemSet, prelude::*};
use bevy::render::{camera::{Camera, PerspectiveProjection}};
use bevy_4x_camera::{CameraRigBundle, FourXCameraPlugin};
use bevy_asset_loader::{AssetLoader};
use hex2d::{self, Coordinate, Spacing, Spin};
use bevy_mod_picking::*;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_mod_picking::{DebugCursorPickingPlugin, InteractablePickingPlugin};
use world::WorldPlugin;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Playing,
    GameOver,
}


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(DebugCursorPickingPlugin)
            .add_plugin(DebugCursorPickingPlugin)
            .add_plugin(FourXCameraPlugin)
            .add_plugin(WorldPlugin);
        #[cfg(debug_assertions)]
        {
             app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                 .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
