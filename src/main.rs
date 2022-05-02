use bevy::prelude::{App, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use hex_map_bevy::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Map of hexes".to_string(),
            width: 3000.,
            height: 1200.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run()
}
