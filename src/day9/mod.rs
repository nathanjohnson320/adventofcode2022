use crate::GameState;
use bevy::prelude::*;
use std::collections::BTreeSet;

pub struct Day9Plugin;

impl Plugin for Day9Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day9).with_system(setup_day))
            .add_system_set(SystemSet::on_exit(GameState::Day9).with_system(cleanup_day));
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
                    "Advent Of Code: Day 9",
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
    let data = super::files::read_lines("src/day9/input.txt");
    let parsed: Vec<(&str, i32)> = data
        .iter()
        .map(|row| {
            let (dir, amount) = row.trim().split_once(' ').unwrap();
            (dir, amount.parse().unwrap())
        })
        .collect();

    let mut knots = vec![(0, 0); 2];
    let mut grid: BTreeSet<(i32, i32)> = BTreeSet::new();
    grid.insert(*knots.last().unwrap());

    for row in parsed {
        let (dir, amount) = row;

        for _i in 0..amount {
            match dir {
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => (),
            }

            for i in 1..2 {
                let curr = knots[i];
                let prev = knots[i - 1];
                if (prev.0 - curr.0).abs() >= 2 || (prev.1 - curr.1).abs() >= 2 {
                    knots[i].0 += (prev.0 - curr.0).signum();
                    knots[i].1 += (prev.1 - curr.1).signum();
                }
            }

            grid.insert(*knots.last().unwrap());
        }
    }

    grid.len()
}

fn part2() -> usize {
    let data = super::files::read_lines("src/day9/input.txt");
    let parsed: Vec<(&str, i32)> = data
        .iter()
        .map(|row| {
            let (dir, amount) = row.trim().split_once(' ').unwrap();
            (dir, amount.parse().unwrap())
        })
        .collect();

    // basically the same but knots is now 10
    let mut knots = vec![(0, 0); 10];
    let mut grid: BTreeSet<(i32, i32)> = BTreeSet::new();
    grid.insert(*knots.last().unwrap());

    for row in parsed {
        let (dir, amount) = row;

        for _i in 0..amount {
            match dir {
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => (),
            }

            for i in 1..10 {
                let curr = knots[i];
                let prev = knots[i - 1];
                if (prev.0 - curr.0).abs() >= 2 || (prev.1 - curr.1).abs() >= 2 {
                    knots[i].0 += (prev.0 - curr.0).signum();
                    knots[i].1 += (prev.1 - curr.1).signum();
                }
            }

            grid.insert(*knots.last().unwrap());
        }
    }

    grid.len()
}
