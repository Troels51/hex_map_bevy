use bevy::{prelude::{Plugin, App, SystemSet, Commands, ResMut, AssetServer, UiCameraBundle, Res, NodeBundle, Color, TextBundle, ImageBundle, Query, With, Component, DetectChanges}, ui::{Style, Val, JustifyContent, AlignItems, FlexDirection, PositionType, UiImage}, math::{Size, Rect}, hierarchy::BuildChildren, text::{Text, TextStyle}};

use crate::{GameState, loading::hexes::HexImageAssets};

pub struct UIPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_ui));
    }
}


#[derive(Component)]
struct HexChooserUI;


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
                });
        });
}
