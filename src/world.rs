use bevy::app::App;
use bevy::render::camera::Camera;
use bevy::{ecs::schedule::SystemSet, prelude::*};

use hex2d::{self, Coordinate, Spin};
use rand::prelude::IteratorRandom;

use crate::board::{Board, Hex};
use crate::loading::hex_models::HexImageAssets;
use crate::ui::UiState;
use crate::GameState;

pub struct WorldPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new(BSIZE, BSIZE))
            .insert_resource(Spacing(hex2d::Spacing::FlatTop(450f32)))
            .add_systems(OnEnter(GameState::Playing),setup)
            .add_systems(OnExit(GameState::Playing), teardown)
            .add_systems(OnExit(GameState::GameOver), teardown)
            .add_event::<BoardGenerateEvent>()
            .add_systems(Update, player_camera_control.run_if(in_state(GameState::Playing)))
            .add_systems(Update, generate_board.run_if(in_state(GameState::Playing)))
            .add_systems(Update, spawn_board.run_if(in_state(GameState::Playing)))
            .add_systems(Update, keyboard_react.run_if(in_state(GameState::Playing)));
    }
}


const BSIZE: usize = 1000;

#[derive(Default, Event)]
pub struct BoardGenerateEvent;

#[derive(Component)]
struct HexTag;

#[derive(Debug, Clone, Copy, Component)]
pub struct CellCoord(hex2d::Coordinate);

#[derive(Resource)]
struct Spacing(hex2d::Spacing);

fn setup(
    mut commands: Commands,
    hex_model_assets: Res<HexImageAssets>,
    hex_desc_assets: Res<Assets<Hex>>,
    spacing: Res<Spacing>,
    mut board: ResMut<Board>,
    ui_state: Res<UiState>,
) {
    for (_handle, hex) in hex_desc_assets.iter() {
        board.add_possible_hex(hex);
    }
    board.clear_board();
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    let center = Coordinate::new(1, 1);
    let center_pixel = center.to_pixel(spacing.0);

    //Spawn camera
    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(center_pixel.0, center_pixel.1, 0f32),
        ..Default::default()
    };
    camera.projection.scale = ui_state.board_size as f32;
    commands.spawn(camera);
}

fn spawn_board(
    mut commands: Commands,
    ui_state: Res<UiState>,
    mut board: ResMut<Board>,
    hex_model_assets: Res<HexImageAssets>,
    spacing: Res<Spacing>,
) {

    let center = Coordinate::new(1, 1).to_pixel(spacing.0);

    // spawn the game board
    if let Some(cell_coord) = board.get_minimal_entropy_coordinate() {
        dbg!(cell_coord);
        let cell_coord = Coordinate::new(cell_coord.0 as i32, cell_coord.1 as i32);
        let pixel = cell_coord.to_pixel(spacing.0);
        let posibility_space = board.get_possible_hexes_for_coordinate(cell_coord).clone();
        if let Some((hex, rotations)) = posibility_space.possible_hexes.iter().choose(&mut rand::thread_rng()) {
            let chosen_rotation = rotations.iter().choose(&mut rand::thread_rng()).unwrap();
            let mut hex = hex.clone();
            hex.rotation = *chosen_rotation;
            board.set(cell_coord, hex.clone());
            let model = hex_model_assets.get(hex.name.as_str());
            //We do some extra math for the y coordinate because hex2d has a coordinate system with y down
            let mut transform =
                Transform::from_xyz(pixel.0, 2f32 * center.1 - pixel.1, 0.0f32);
            transform.rotate(Quat::from_rotation_z(
                *chosen_rotation as f32 * -std::f32::consts::PI / 3f32,
            ));

            commands
                .spawn(SpriteBundle {
                    texture: model.clone(),
                    transform,
                    ..default()
                })
                .insert(HexTag)
                .insert(hex.clone());
        }
        else {
            let model = hex_model_assets.blank.clone();
            let transform =
                Transform::from_xyz(pixel.0, 2f32 * center.1 - pixel.1, 0.0f32);
            commands
                .spawn(SpriteBundle {
                    texture: model.clone(),
                    transform,
                    ..default()
                })
                .insert(HexTag);
        }
    }
}

fn generate_board(
    mut board: ResMut<Board>,
    hex_model_assets: Res<HexImageAssets>,
    hex_entities: Query<Entity, With<HexTag>>,
    spacing: Res<Spacing>,
    mut commands: Commands,
    ui_state: Res<UiState>,
    mut generate_board_events: EventReader<BoardGenerateEvent>,
) {
    if !generate_board_events.is_empty() {
        //trigger despawn of board
        for entity in hex_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        board.reset();
        board.clear_board();
        //then trigger spawn of board
    }
    generate_board_events.clear();
}

fn keyboard_react(
    keys: Res<Input<KeyCode>>,
    mut generate_board_events: EventWriter<BoardGenerateEvent>,
) {
    if keys.just_pressed(KeyCode::X) {
        generate_board_events.send_default();
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
