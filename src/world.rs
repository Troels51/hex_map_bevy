use std::ops::Index;

use bevy::app::App;
use bevy::render::camera::{Camera, PerspectiveProjection};
use bevy::{ecs::schedule::SystemSet, prelude::*};
use bevy_4x_camera::CameraRigBundle;
use bevy_mod_picking::*;
use hex2d::{self, Coordinate, Spacing, Spin};
use rand::prelude::IteratorRandom;

use crate::board::{Board, Hex};
use crate::loading::hexes::{HexAssets, HexImageAssets};
use crate::GameState;

pub struct WorldPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneInstance::default())
            .insert_resource(Board::new(BSIZE, BSIZE))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(scene_update))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(rotate_hex));
    }
}

//Constants
const SPACING: Spacing = Spacing::PointyTop(1.00f32);

const BOARD_SIZE: i32 = 5;
const BSIZE: usize = 20;

#[derive(Debug, Clone, Copy, Component)]
pub struct CellCoord(hex2d::Coordinate);




fn setup(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
    hex_model_assets: Res<HexAssets>,
    hex_desc_assets: Res<Assets<Hex>>,
    mut board: ResMut<Board>,
) {
    for (_handle, hex) in hex_desc_assets.iter() {
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

    let center = Coordinate::new(5, 5);
    let center_pixel = center.to_pixel(SPACING);

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
                transform: Transform::from_translation(Vec3::new(-30.0, 30., -0.0))
                    .looking_at(Vec3::new(center_pixel.0, 0.0, center_pixel.1), Vec3::Y),
                ..Default::default()
            })
            .insert_bundle(PickingCameraBundle::default());
        });

    // spawn the game board
    for ring_radius in 0..BOARD_SIZE {
        let ring = center.ring_iter(ring_radius, Spin::CCW(hex2d::Direction::XY));
        for cell_coord in ring {
            let pixel = cell_coord.to_pixel(SPACING);

            let posibility_space = board.get_possible_hexes_for_coordinate(cell_coord);
            if let Some(hex) = posibility_space.iter().choose(&mut rand::thread_rng()) {
                board.set(cell_coord, hex.clone());
                let model = hex_model_assets.get(hex.name.as_str());
                let mut transform = Transform::from_xyz(pixel.0, 0 as f32, pixel.1);
                transform.rotate(Quat::from_axis_angle(
                    Vec3::Y,
                    hex.rotation as f32 * -std::f32::consts::PI / 3.0,
                ));

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



fn get_top_parent<'a>(parent_query: &'a Query<'a, 'a, &Parent>, child: &'a Entity) -> &'a Parent {
    let parent = parent_query.get(*child).unwrap();
    let parent = parent_query.get(parent.0).unwrap();
    let parent = parent_query.get(parent.0).unwrap();
    parent
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
