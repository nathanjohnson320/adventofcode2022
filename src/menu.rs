use crate::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct ButtonActive(bool);

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu));
    }
}

fn cleanup_menu(mut commands: Commands, node_query: Query<Entity, With<Node>>) {
    for ent in node_query.iter() {
        commands.entity(ent).despawn_descendants();
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

fn menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, _color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => match text.sections[0].value.as_str() {
                "Day 1" => state.set(GameState::Day1).unwrap(),
                "Day 2" => state.set(GameState::Day2).unwrap(),
                "Day 3" => state.set(GameState::Day3).unwrap(),
                "Day 4" => state.set(GameState::Day4).unwrap(),
                "Day 5" => state.set(GameState::Day5).unwrap(),
                "Day 6" => state.set(GameState::Day6).unwrap(),
                "Day 7" => state.set(GameState::Day7).unwrap(),
                "Day 8" => state.set(GameState::Day8).unwrap(),
                "Day 9" => state.set(GameState::Day9).unwrap(),
                "Day 10" => state.set(GameState::Day10).unwrap(),
                "Day 11" => state.set(GameState::Day11).unwrap(),
                "Day 12" => state.set(GameState::Day12).unwrap(),
                "Day 13" => state.set(GameState::Day13).unwrap(),
                "Day 14" => state.set(GameState::Day14).unwrap(),
                "Day 15" => state.set(GameState::Day15).unwrap(),
                "Day 16" => state.set(GameState::Day16).unwrap(),
                "Day 17" => state.set(GameState::Day17).unwrap(),
                "Day 18" => state.set(GameState::Day18).unwrap(),
                "Day 19" => state.set(GameState::Day19).unwrap(),
                "Day 20" => state.set(GameState::Day20).unwrap(),
                "Day 21" => state.set(GameState::Day21).unwrap(),
                "Day 22" => state.set(GameState::Day22).unwrap(),
                "Day 23" => state.set(GameState::Day23).unwrap(),
                "Day 24" => state.set(GameState::Day24).unwrap(),
                "Day 25" => state.set(GameState::Day25).unwrap(),
                a => println!("No match wtf did you click? {}", a),
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|body| {
            body.spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Auto, Val::Px(30.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|header| {
                header.spawn(TextBundle::from_section(
                    "Advent Of Code",
                    TextStyle {
                        font: asset_server.load("fonts/runescape_uf.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });

            body.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_wrap: FlexWrap::Wrap,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|grid| {
                for n in 0..25 {
                    grid.spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|cell| {
                        cell.spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                // center button
                                margin: UiRect::all(Val::Percent(20.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                format!("Day {}", n + 1),
                                TextStyle {
                                    font: asset_server.load("fonts/runescape_uf.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    });
                }
            });
        });
}
