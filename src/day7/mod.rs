use crate::GameState;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct Day7Plugin;

impl Plugin for Day7Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day7).with_system(setup_day))
            .add_system_set(SystemSet::on_exit(GameState::Day7).with_system(cleanup_day));
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
                    "Advent Of Code: Day 7",
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
    let data: Vec<String> = super::files::read_lines("src/day7/input.txt");
    let mut folder_sizes: HashMap<String, usize> = HashMap::new();
    let mut folder_path: Vec<&str> = Vec::new();

    for i in 0..data.len() {
        let commands: Vec<&str> = data[i].split(' ').collect();
        let p1 = commands[0];
        let sub_command = commands[1];

        if p1 == "$" {
            if sub_command == "cd" {
                let path = commands[2];
                if path == ".." {
                    folder_path.pop();
                } else if path == "/" {
                    folder_path.clear();
                    folder_path.push("/");
                } else {
                    folder_path.push(path);
                }
            }
        } else {
            if p1 != "dir" {
                let mut current_path = String::new();

                for j in 0..folder_path.len() {
                    if current_path != "/" && folder_path[j] != "/" {
                        current_path = format!("{}{}", current_path, "/");
                    }

                    current_path = format!("{}{}", current_path, folder_path[j]);
                    let size = folder_sizes.get(&current_path).cloned().unwrap_or(0) + p1.parse::<usize>().unwrap();
                    folder_sizes.insert(current_path.to_owned(), size);
                }
            }
        }
    }

    folder_sizes.values().into_iter().filter(|size| **size < 100000).sum()
}

fn part2() -> usize {
    let data: Vec<String> = super::files::read_lines("src/day7/input.txt");
    let mut folder_sizes: HashMap<String, usize> = HashMap::new();
    let mut folder_path: Vec<&str> = Vec::new();

    for i in 0..data.len() {
        let commands: Vec<&str> = data[i].split(' ').collect();
        let p1 = commands[0];
        let sub_command = commands[1];

        if p1 == "$" {
            if sub_command == "cd" {
                let path = commands[2];
                if path == ".." {
                    folder_path.pop();
                } else if path == "/" {
                    folder_path.clear();
                    folder_path.push("/");
                } else {
                    folder_path.push(path);
                }
            }
        } else {
            if p1 != "dir" {
                let mut current_path = String::new();

                for j in 0..folder_path.len() {
                    if current_path != "/" && folder_path[j] != "/" {
                        current_path = format!("{}{}", current_path, "/");
                    }

                    current_path = format!("{}{}", current_path, folder_path[j]);
                    let size = folder_sizes.get(&current_path).cloned().unwrap_or(0) + p1.parse::<usize>().unwrap();
                    folder_sizes.insert(current_path.to_owned(), size);
                }
            }
        }
    }

    let needed_space = 30000000 - (70000000 - folder_sizes.get("/").unwrap());

    *folder_sizes.values().into_iter().filter(|size| **size >= needed_space).min().unwrap()
}
