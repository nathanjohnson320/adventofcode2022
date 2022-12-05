use crate::GameState;
use bevy::prelude::*;
use regex::Regex;
use std::collections::VecDeque;

pub struct Day5Plugin;

impl Plugin for Day5Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day5).with_system(setup_day))
            .add_system_set(SystemSet::on_exit(GameState::Day5).with_system(cleanup_day));
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
                    "Advent Of Code: Day 5",
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

fn part1() -> String {
    // Test queues
    // VecDeque::from(['Z', 'N']),
    // VecDeque::from(['M', 'C', 'D']),
    // VecDeque::from(['P']),

    //         [G]         [D]     [Q]
    // [P]     [T]         [L] [M] [Z]
    // [Z] [Z] [C]         [Z] [G] [W]
    // [M] [B] [F]         [P] [C] [H] [N]
    // [T] [S] [R]     [H] [W] [R] [L] [W]
    // [R] [T] [Q] [Z] [R] [S] [Z] [F] [P]
    // [C] [N] [H] [R] [N] [H] [D] [J] [Q]
    // [N] [D] [M] [G] [Z] [F] [W] [S] [S]
    let mut crates: Vec<VecDeque<char>> = Vec::from([
        VecDeque::from(['N', 'C', 'R', 'T', 'M', 'Z', 'P']),
        VecDeque::from(['D', 'N', 'T', 'S', 'B', 'Z']),
        VecDeque::from(['M', 'H', 'Q', 'R', 'F', 'C', 'T', 'G']),
        VecDeque::from(['G', 'R', 'Z']),
        VecDeque::from(['Z', 'N', 'R', 'H']),
        VecDeque::from(['F', 'H', 'S', 'W', 'P', 'Z', 'L', 'D']),
        VecDeque::from(['W', 'D', 'Z', 'R', 'C', 'G', 'M']),
        VecDeque::from(['S', 'J', 'F', 'L', 'H', 'W', 'Z', 'Q']),
        VecDeque::from(['S', 'Q', 'P', 'W', 'N']),
    ]);

    let data: Vec<String> = super::files::read_lines("src/day5/input.txt");
    let re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for row in data {
        let caps = re.captures(row.as_str()).unwrap();
        let total: usize = caps
            .get(1)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let from: usize = caps
            .get(2)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let to: usize = caps
            .get(3)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());

        for _i in 0..total {
            let popped = crates.get_mut(from - 1).unwrap().pop_back().unwrap();
            crates.get_mut(to - 1).unwrap().push_back(popped);
        }
    }

    let code: String = crates.iter().map(|stack| stack.back().unwrap()).collect();

    code
}

fn part2() -> String {
    // Test queues
    // let mut crates: Vec<VecDeque<char>> = Vec::from([
    //     VecDeque::from(['Z', 'N']),
    //     VecDeque::from(['M', 'C', 'D']),
    //     VecDeque::from(['P']),
    // ]);

    //         [G]         [D]     [Q]
    // [P]     [T]         [L] [M] [Z]
    // [Z] [Z] [C]         [Z] [G] [W]
    // [M] [B] [F]         [P] [C] [H] [N]
    // [T] [S] [R]     [H] [W] [R] [L] [W]
    // [R] [T] [Q] [Z] [R] [S] [Z] [F] [P]
    // [C] [N] [H] [R] [N] [H] [D] [J] [Q]
    // [N] [D] [M] [G] [Z] [F] [W] [S] [S]
    let mut crates: Vec<VecDeque<char>> = Vec::from([
        VecDeque::from(['N', 'C', 'R', 'T', 'M', 'Z', 'P']),
        VecDeque::from(['D', 'N', 'T', 'S', 'B', 'Z']),
        VecDeque::from(['M', 'H', 'Q', 'R', 'F', 'C', 'T', 'G']),
        VecDeque::from(['G', 'R', 'Z']),
        VecDeque::from(['Z', 'N', 'R', 'H']),
        VecDeque::from(['F', 'H', 'S', 'W', 'P', 'Z', 'L', 'D']),
        VecDeque::from(['W', 'D', 'Z', 'R', 'C', 'G', 'M']),
        VecDeque::from(['S', 'J', 'F', 'L', 'H', 'W', 'Z', 'Q']),
        VecDeque::from(['S', 'Q', 'P', 'W', 'N']),
    ]);

    let data: Vec<String> = super::files::read_lines("src/day5/input.txt");
    let re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for row in data {
        let caps = re.captures(row.as_str()).unwrap();
        let total: usize = caps
            .get(1)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let from: usize = caps
            .get(2)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let to: usize = caps
            .get(3)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());

        let mut stack: Vec<char> = (0..total)
            .map(|_i| crates.get_mut(from - 1).unwrap().pop_back().unwrap())
            .collect();

        stack.reverse();

        for char in stack {
            crates.get_mut(to - 1).unwrap().push_back(char);
        }
    }

    let code: String = crates.iter().map(|stack| stack.back().unwrap()).collect();

    code
}
