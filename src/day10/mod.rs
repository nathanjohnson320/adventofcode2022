use crate::GameState;
use bevy::prelude::*;

pub struct Day10Plugin;

impl Plugin for Day10Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day10).with_system(setup_day))
            .add_system_set(SystemSet::on_exit(GameState::Day10).with_system(cleanup_day));
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
                    "Advent Of Code: Day 10",
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

struct Registers {
    x: i32,
}

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            x: 1
        }
    }
}

fn part1() -> i32 {
    let mut registers = Registers::default();

    let data = super::files::read_lines("src/day10/input.txt");
    let mut commands: Vec<(&str, i32)> = Vec::new();
    for row in data {
        match row.trim() {
            "noop" => commands.push(("noop", 0)),
            trimmed => {
                match trimmed.split_once(' ').unwrap() {
                    ("addx", i)  => {
                        // addx takes 2 cycles so give it a blank one to process to make things simpler
                        commands.push(("noop", 0));
                        commands.push(("addx", i.parse::<i32>().unwrap()));
                    },
                    _ => ()
                }
            },
        }
    }

    let mut signal_strength = 0;
    for (cycle, command) in commands.iter().enumerate() {
        // the 20th cycle and every 40 cycles after that (that is, during the 20th, 60th, 100th, 140th, 180th, and 220th cycles).
        let c = cycle + 1;
        if [20, 60, 100, 140, 180, 220].contains(&c) {
            signal_strength += c as i32 * registers.x;
        }

        match command {
            ("addx", add) => {
                registers.x += add
            },
            _ => (),
        }
    }

    signal_strength
}

fn part2() -> String {
    let mut registers = Registers::default();

    let data = super::files::read_lines("src/day10/input.txt");
    let mut commands: Vec<(&str, i32)> = Vec::new();
    for row in data {
        match row.trim() {
            "noop" => commands.push(("noop", 0)),
            trimmed => {
                match trimmed.split_once(' ').unwrap() {
                    ("addx", i)  => {
                        // addx takes 2 cycles so give it a blank one to process to make things simpler
                        commands.push(("noop", 0));
                        commands.push(("addx", i.parse::<i32>().unwrap()));
                    },
                    _ => ()
                }
            },
        }
    }

    let mut lines = String::new();
    let mut line = String::new();
    let mut i = 0;
    for command in commands {
        let pixel = if [registers.x - 1, registers.x, registers.x + 1].contains(&i) {
            '#'
        } else {
            '.'
        };

        line.push(pixel);

        match command {
            ("addx", add) => {
                registers.x += add
            },
            _ => (),
        }

        // 40x6 lines
        i += 1;
        if i % 40 == 0 {
            lines.push('\n');
            lines += &line;
            println!("{}", line);
            line = String::new();
            i = 0;
        }
    }

    lines
}
