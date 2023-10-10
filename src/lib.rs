mod board;
mod loading;
mod ui;
mod world;
use crate::loading::LoadingPlugin;

use bevy::app::App;

use bevy::prelude::*;

use ui::UIPlugin;
use world::WorldPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, States, Default)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(LoadingPlugin)
            .add_plugins(UIPlugin)
            .add_plugins(WorldPlugin);
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
