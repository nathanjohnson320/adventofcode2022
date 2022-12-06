use crate::GameState;
use bevy::prelude::*;
use std::collections::HashSet;

pub struct Day3Plugin;

impl Plugin for Day3Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day3).with_system(setup_day))
            .add_system_set(SystemSet::on_exit(GameState::Day3).with_system(cleanup_day));
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
                    "Advent Of Code: Day 3",
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
                    },
                ));

                content.spawn(TextBundle::from_section(
                    format!("Part 2: {}", part2()),
                    TextStyle {
                        font: asset_server.load("fonts/runescape_uf.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
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

fn part1() -> i64 {
    let data = super::files::read_lines("src/day3/input.txt");

    let mut sum = 0;
    for r in data {
        let ruck: Vec<_> = r.chars().collect();
        let ruck_len = ruck.len() / 2;
        let start = &ruck[..ruck_len];
        let end = &ruck[ruck_len..];

        let front: HashSet<&char> = HashSet::from_iter(start);
        let rear: HashSet<&char> = HashSet::from_iter(end);

        for item in front.intersection(&rear) {
            sum += priority(**item);
        }
    }

    sum
}

fn part2() -> i64 {
    let data = super::files::read_lines("src/day3/input.txt");

    let mut sum = 0;
    for i in (0..data.len()).step_by(3) {
        let a: HashSet<char> = HashSet::from_iter(data[i].chars());
        let b: HashSet<char> = HashSet::from_iter(data[i + 1].chars());
        let c: HashSet<char> = HashSet::from_iter(data[i + 2].chars());

        let ab: HashSet<char> = a.intersection(&b).map(|x| x.clone()).collect();
        let ab_intersection = ab.intersection(&c);

        for item in ab_intersection {
            sum += priority(*item);
        }
    }

    sum
}

fn priority(c: char) -> i64 {
    let offset = if c.is_lowercase() { 96 } else { 38 };
    (c as u32 - offset).into()
}
