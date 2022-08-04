mod board;
mod loading;
mod ui;
mod world;
use crate::loading::LoadingPlugin;

use bevy::app::App;

use bevy::prelude::*;

use ui::UIPlugin;
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
            .add_plugin(UIPlugin)
            .add_plugin(WorldPlugin);
        #[cfg(debug_assertions)]
        {
            // app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            //     .add_plugin(LogDiagnosticsPlugin::default())
            //     .add_plugin(EditorPlugin);
        }
    }
}

// .add_plugin(PickingPlugin)
// .add_plugin(InteractablePickingPlugin)
// .add_plugin(DebugCursorPickingPlugin)
// .add_plugin(DebugCursorPickingPlugin)
// .add_plugin(FourXCameraPlugin)
