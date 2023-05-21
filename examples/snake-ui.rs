use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

/// This example illustrates the various features of Bevy UI.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        //.add_system(mouse_scroll)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(400.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                text: Text::with_section(
                                    "Score window",
                                    TextStyle {
                                        font: asset_server.load("dejavu-sans-mono/DejaVuSansMono.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..default()
                            });
                        });
                });
            // right vertical fill
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        //flex_direction: FlexDirection::ColumnReverse,
                        //justify_content: JustifyContent::FlexStart,
                        size: Size::new(Val::Px(700.0), Val::Percent(100.0)),
                        ..default()
                    },
                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            size: Size::new(Val::Undefined, Val::Px(25.)),
                            margin: Rect {
                                left: Val::Auto,
                                right: Val::Auto,
                                ..default()
                            },
                            ..default()
                        },
                        text: Text::with_section(
                            "Snake game window",
                            TextStyle {
                                font: asset_server.load("dejavu-sans-mono/DejaVuSansMono.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..default()
                    });

                });
            // absolute positioning
            // parent
            //     .spawn_bundle(NodeBundle {
            //         style: Style {
            //             size: Size::new(Val::Px(200.0), Val::Px(200.0)),
            //             position_type: PositionType::Absolute,
            //             position: Rect {
            //                 left: Val::Px(210.0),
            //                 bottom: Val::Px(10.0),
            //                 ..default()
            //             },
            //             border: Rect::all(Val::Px(20.0)),
            //             ..default()
            //         },
            //         color: Color::rgb(0.4, 0.4, 1.0).into(),
            //         ..default()
            //     })
            //     .with_children(|parent| {
            //         parent.spawn_bundle(NodeBundle {
            //             style: Style {
            //                 size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            //                 ..default()
            //             },
            //             color: Color::rgb(0.8, 0.8, 1.0).into(),
            //             ..default()
            //         });
            //     });

        });
}

