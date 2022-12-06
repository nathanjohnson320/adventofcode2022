use crate::GameState;
use bevy::prelude::*;
use std::collections::HashSet;

pub struct Day6Plugin;

impl Plugin for Day6Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day6).with_system(setup_day))
            .add_system_set(SystemSet::on_exit(GameState::Day6).with_system(cleanup_day));
    }
}

fn setup_day(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    "Advent Of Code: Day 6",
                    TextStyle {
                        font: asset_server.load("fonts/runescape_uf.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });

            body.spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Auto),
                    size: Size::new(Val::Auto, Val::Auto),
                    ..default()
                },
                ..default()
            })
            .with_children(|content| {
                content.spawn(TextBundle::from_section(
                    format!("Part 1: {}", part1()),
                    TextStyle {
                        font: asset_server.load("fonts/runescape_uf.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));

                content.spawn(TextBundle::from_section(
                    format!("Part 2: {}", part2()),
                    TextStyle {
                        font: asset_server.load("fonts/runescape_uf.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
        });
}

fn cleanup_day(mut commands: Commands, node_query: Query<Entity, With<Node>>) {
    for ent in node_query.iter() {
        commands.entity(ent).despawn_descendants();
    }
}

fn part1() -> usize {
    let data: Vec<String> = super::files::read_lines("src/day6/input.txt");
    let input = data.first().unwrap();

    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        if i > 3 && i + 4 < input.len() {
            let unique_chars = HashSet::from([chars[i], chars[i - 1], chars[i - 2], chars[i - 3]]);
            if unique_chars.len() == 4 {
                return i + 1;
            }
        }
    }

    0
}

fn part2() -> usize {
    let data: Vec<String> = super::files::read_lines("src/day6/input.txt");
    let input = data.first().unwrap();

    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        if i > 13 && i + 14 < input.len() {
            let chars = &chars[i-13..i];
            let unique_chars: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
            println!("{:?}", unique_chars);
            if unique_chars.len() == 14 {
                return i + 1;
            }
        }
    }

    0
}
