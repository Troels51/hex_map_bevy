use bevy::app::App;
use bevy::render::camera::{Camera};
use bevy::{ecs::schedule::SystemSet, prelude::*};


use hex2d::{self, Coordinate, Spacing, Spin};
use rand::prelude::IteratorRandom;

use crate::board::{Board, Hex};
use crate::loading::hex_models::{HexImageAssets};
use crate::GameState;

pub struct WorldPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new(BSIZE, BSIZE))
            .insert_resource(Spacing::FlatTop(450f32))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(player_camera_control),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(spawn_board_on_press),
            );
    }
}

// //Constants
// const SPACING: Spacing = Spacing::FlatTop(451.00f32);

const BOARD_SIZE: i32 = 10;
const BSIZE: usize = 100;

#[derive(Component)]
struct HexTag;

#[derive(Debug, Clone, Copy, Component)]
pub struct CellCoord(hex2d::Coordinate);

fn setup(
    mut commands: Commands,
    hex_model_assets: Res<HexImageAssets>,
    hex_desc_assets: Res<Assets<Hex>>,
    spacing: Res<Spacing<f32>>,
    mut board: ResMut<Board>,
) {
    for (_handle, hex) in hex_desc_assets.iter() {
        board.add_possible_hex(hex);
    }
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    let center = Coordinate::new(10, 10);
    let center_pixel = center.to_pixel(*spacing);

    //Spawn camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_xyz(center_pixel.0, center_pixel.1, 0f32);
    camera.orthographic_projection.scale = BOARD_SIZE as f32;
    commands.spawn_bundle(camera);

    spawn_board(center, board, hex_model_assets, spacing, commands);
}

fn spawn_board(
    center: Coordinate,
    mut board: ResMut<Board>,
    hex_model_assets: Res<HexImageAssets>,
    spacing: Res<Spacing<f32>>,
    mut commands: Commands,
) {
    let center_pixel = center.to_pixel(*spacing);
    // spawn the game board
    for ring_radius in 0..BOARD_SIZE {
        let ring = center.ring_iter(ring_radius, Spin::CW(hex2d::Direction::YZ));
        for cell_coord in ring {
            let pixel = cell_coord.to_pixel(*spacing);
            let posibility_space = board.get_possible_hexes_for_coordinate(cell_coord);
            if let Some(hex) = posibility_space.iter().choose(&mut rand::thread_rng()) {
                board.set(cell_coord, hex.clone());
                let model = hex_model_assets.get(hex.name.as_str());
                //We do some extra math for the y coordinate because hex2d has a coordinate system with y down
                let mut transform =
                    Transform::from_xyz(pixel.0, 2f32 * center_pixel.1 - pixel.1, 0.0f32);
                transform.rotate(Quat::from_rotation_z(
                    hex.rotation as f32 * -std::f32::consts::PI / 3f32,
                ));

                commands
                    .spawn_bundle(SpriteBundle {
                        texture: model.clone(),
                        transform,
                        ..default()
                    })
                    .insert(HexTag)
                    .insert(hex.clone());
            }
        }
    }
}

fn spawn_board_on_press(
    keys: Res<Input<KeyCode>>,
    mut board: ResMut<Board>,
    hex_model_assets: Res<HexImageAssets>,
    hex_entities: Query<Entity, With<HexTag>>,
    spacing: Res<Spacing<f32>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::X) {
        //trigger despawn of board
        for entity in hex_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        board.reset();
        //then trigger spawn of board
        let center = Coordinate::new(10, 10);
        spawn_board(center, board, hex_model_assets, spacing, commands);
    }
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn player_camera_control(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut projection_query: Query<&mut OrthographicProjection>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let dist = 1f32 * time.delta().as_secs_f32();
    let mut log_scale = 1.0;

    for mut projection in projection_query.iter_mut() {
        log_scale = projection.scale.ln();
        // Scale in or out
        if kb.pressed(KeyCode::PageUp) {
            log_scale -= dist;
        }
        if kb.pressed(KeyCode::PageDown) {
            log_scale += dist;
        }
        projection.scale = log_scale.exp();
    }
    let move_speed = 1000.0 * log_scale;

    for mut transform in camera_query.iter_mut() {
        // Move camera all about
        if kb.pressed(KeyCode::W) {
            transform.translation.y += move_speed * dist;
        }
        if kb.pressed(KeyCode::A) {
            transform.translation.x -= move_speed * dist;
        }
        if kb.pressed(KeyCode::S) {
            transform.translation.y -= move_speed * dist;
        }
        if kb.pressed(KeyCode::D) {
            transform.translation.x += move_speed * dist;
        }
    }
}
