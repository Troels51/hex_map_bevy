use core::time;
use std::ops::Index;

use bevy::app::App;
use bevy::render::camera::{Camera, PerspectiveProjection};
use bevy::{ecs::schedule::SystemSet, prelude::*};
use bevy_4x_camera::CameraRigBundle;
use bevy_mod_picking::*;
use hex2d::{self, Coordinate, Spacing, Spin};
use rand::prelude::IteratorRandom;

use crate::board::{Hex, Board};
use crate::loading::hexes::{HexAssets, HexImageAssets};
use crate::{loading, GameState};

pub struct WorldPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneInstance::default())
            .insert_resource(ChoosenHex(1))
            .insert_resource(Board::new(BSIZE, BSIZE))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_ui))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(scene_update))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(click_events))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(rotate_hex))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(choose_hex))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(update_chosen_hex_ui),
            );
    }
}

//Constants
const SPACING: Spacing = Spacing::PointyTop(1.05f32);

const BOARD_SIZE: i32 = 5;
const BSIZE: usize = 20;


#[derive(Debug, Clone, Copy, Component)]
pub enum CellType {
    Bland,
    Trees,
}
#[derive(Debug, Clone, Copy, Component)]
pub struct CellCoord(hex2d::Coordinate);

struct ChoosenHex(usize);

#[derive(Component)]
struct HexChooserUI;

fn setup(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
    hex_model_assets: Res<HexAssets>,
    hex_desc_assets: Res<Assets<Hex>>,
    mut board: ResMut<Board>,
) {
    
    for (handle, hex) in hex_desc_assets.iter() {
        dbg!(&hex.sides);
        dbg!(&hex.name);
        board.add_possible_hex(hex);
    }
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    //Add Point light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        point_light: PointLight {
            intensity: 2500.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());

    //Spawn camera
    commands
        .spawn_bundle(CameraRigBundle::default())
        // camera
        .with_children(|cb| {
            cb.spawn_bundle(PerspectiveCameraBundle {
                perspective_projection: PerspectiveProjection {
                    fov: 0.3,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-30.0, 30., 0.0))
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            })
            .insert_bundle(PickingCameraBundle::default());
        });

    // spawn the game board
    let center = Coordinate::new(5, 5);
    for ring_radius in 0..BOARD_SIZE {
        let ring = center.ring_iter(ring_radius, Spin::CCW(hex2d::Direction::XY));
        for cell_coord in ring {
            let pixel = cell_coord.to_pixel(SPACING);
            
            let posibility_space = board.get_possible_hexes_for_coordinate(cell_coord);
            if let Some(hex) = posibility_space.iter().choose(&mut rand::thread_rng()) {
                board.set(cell_coord, hex.clone());
                let model = hex_model_assets.get(hex.name.as_str());
                let mut transform = Transform::from_xyz(pixel.0, 0 as f32, pixel.1);
                transform.rotate(Quat::from_axis_angle(Vec3::Y, hex.rotation as f32 * -std::f32::consts::PI/3.0));
    
                commands
                    .spawn_bundle((
                        transform,
                        GlobalTransform::identity(),
                        CellCoord(cell_coord),
                    ))
                    .with_children(|parent| {
                        let instance_id =
                            scene_spawner.spawn_as_child(model.clone(), parent.parent_entity());
                        scene_instance.0.push(instance_id);
                    });
            }

        }
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    hex_image_assets: Res<HexImageAssets>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
            // right vertical fill
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            });
            // absolute positioning
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(350.0), Val::Px(350.0)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(210.0),
                            bottom: Val::Px(10.0),
                            ..Default::default()
                        },
                        border: Rect::all(Val::Px(20.0)),
                        ..Default::default()
                    },
                    color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(500.0), Val::Auto),
                                ..Default::default()
                            },
                            image: hex_image_assets.bland.clone().into(),
                            ..Default::default()
                        })
                        .insert(HexChooserUI);
                });
        });
}

// Resource to hold the scene `instance_id` until it is loaded
#[derive(Default)]
struct SceneInstance(Vec<bevy::scene::InstanceId>);

fn scene_update(
    mut commands: Commands,
    scene_spawner: Res<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
) {
    let mut counter = 0;
    while let Some(instance_id) = scene_instance.0.pop() {
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
            entity_iter.for_each(|entity| {
                commands
                    .entity(entity)
                    .insert_bundle(PickableBundle::default());
            });
        } else {
            counter += 1;
            if counter > 1000 {
                return;
            };

            scene_instance.0.push(instance_id); //I know this is dumb
        }
    }
}

fn click_events(
    mut events: EventReader<PickingEvent>,
    transfrom_query: Query<&Transform>,
    cellcord_query: Query<&CellCoord>,
    parent_query: Query<&Parent>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
    mut commands: Commands,
    hex_assets: Res<HexAssets>,
    chosen_hex: Res<ChoosenHex>,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(tile) = event {
            let parent = get_top_parent(&parent_query, tile);
            let cellcoord: &CellCoord = cellcord_query.get_component(parent.0).unwrap();
            let position: &Transform = transfrom_query.get_component(parent.0).unwrap();
            commands
                .spawn_bundle((
                    position.clone(),
                    GlobalTransform::identity(),
                    cellcoord.clone(),
                    CellType::Trees,
                ))
                .with_children(|parent_1| {
                    let instance_id = scene_spawner.spawn_as_child(
                        hex_assets.index(chosen_hex.0).clone(),
                        parent_1.parent_entity(),
                    );
                    scene_instance.0.push(instance_id);
                });
            commands.entity(parent.0).despawn_recursive();
        }
    }
}

fn rotate_hex(
    keys: Res<Input<KeyCode>>,
    parent_query: Query<&Parent>,
    hover_query: Query<(Entity, &Hover)>,
    mut transform: Query<&mut Transform>,
) {
    if keys.just_pressed(KeyCode::R) {
        if let Some(child) = hover_query.iter().find(|(_, h)| h.hovered()) {
            let parent = get_top_parent(&parent_query, &child.0);
            let mut trans = transform.get_mut(parent.0).unwrap();
            trans.rotate(Quat::from_axis_angle(Vec3::Y, -std::f32::consts::PI / 3.0))
        }
    }
}
fn choose_hex(keys: Res<Input<KeyCode>>, mut chosen_hex: ResMut<ChoosenHex>) {
    if keys.just_pressed(KeyCode::F) {
        chosen_hex.0 = (chosen_hex.0 + 1) % 58;
    }
}

fn update_chosen_hex_ui(
    mut ui: Query<&mut UiImage, With<HexChooserUI>>,
    chosen_hex: ResMut<ChoosenHex>,
    hex_assets: Res<HexImageAssets>,
) {
    if chosen_hex.is_changed() {
        ui.single_mut().0 = hex_assets[chosen_hex.0].clone();
    }
}

fn get_top_parent<'a>(parent_query: &'a Query<'a, 'a, &Parent>, child: &'a Entity) -> &'a Parent {
    let parent = parent_query.get(*child).unwrap();
    let parent = parent_query.get(parent.0).unwrap();
    let parent = parent_query.get(parent.0).unwrap();
    return parent;
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
