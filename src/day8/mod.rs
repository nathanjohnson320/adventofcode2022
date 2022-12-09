use crate::GameState;
use bevy::prelude::*;

pub struct Day8Plugin;

impl Plugin for Day8Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day8).with_system(setup_day))
            .add_system_set(SystemSet::on_exit(GameState::Day8).with_system(cleanup_day));
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
                    "Advent Of Code: Day 8",
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
    let data: Vec<String> = super::files::read_lines("src/day8/input.txt");
    let trees: Vec<Vec<u32>> = data
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut visible = 0;
    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            // Trees on the edge are automatically visible
            if row == 0 || col == 0 || row == trees.len() - 1 || col == trees[row].len() - 1 {
                visible += 1;
                continue;
            }

            let height = trees[row][col];

            let top = (0..row).all(|k| height > trees[k][col]);
            let bottom = (row + 1..trees.len()).all(|k| height > trees[k][col]);
            let left = (0..col).all(|k| height > trees[row][k]);
            let right = (col + 1..trees.len()).all(|k| height > trees[row][k]);

            if top || bottom || left || right {
                visible += 1;
            }
        }
    }

    visible
}

fn part2() -> usize {
    let data: Vec<String> = super::files::read_lines("src/day8/input.txt");
    let trees: Vec<Vec<u32>> = data
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut highest_score = 0;
    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            if row == 0 || col == 0 || row == trees.len() - 1 || col == trees[row].len() - 1 {
                continue;
            }

            let height = trees[row][col];

            let mut top = 0;
            for k in (0..row).rev() {
                top += 1;
                if height <= trees[k][col] {
                    break;
                }
            }

            let mut bottom = 0;
            for k in row + 1..trees.len() {
                bottom += 1;
                if height <= trees[k][col] {
                    break;
                }
            }

            let mut left = 0;
            for k in (0..col).rev() {
                left += 1;
                if height <= trees[row][k] {
                    break;
                }
            }

            let mut right = 0;
            for k in col + 1..trees.len() {
                right += 1;
                if height <= trees[row][k] {
                    break;
                }
            }

            let scenic_score = top * bottom * left * right;
            if scenic_score > highest_score {
                highest_score = scenic_score;
            }
        }
    }

    highest_score
}
