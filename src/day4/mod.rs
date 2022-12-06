use crate::GameState;
use bevy::prelude::*;
use std::ops::Range;

pub struct Day4Plugin;

#[derive(Component)]
pub struct ControlledText;

impl Plugin for Day4Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Day4).with_system(keyboard))
            .add_system_set(SystemSet::on_enter(GameState::Day4).with_system(setup_day))
            .add_system_set(SystemSet::on_exit(GameState::Day4).with_system(cleanup_day));
    }
}

fn keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Text, With<ControlledText>>,
) {
    for mut text in &mut query {
        if keyboard_input.pressed(KeyCode::W) {
            let color = text.sections[0].style.color;
            text.sections[0].style.color =
                Color::rgb(color.r() + 0.1, color.g() + 0.1, color.b() + 0.1);
        }
        if keyboard_input.pressed(KeyCode::S) {
            let color = text.sections[0].style.color;
            text.sections[0].style.color =
                Color::rgb(color.r() - 0.1, color.g() - 0.1, color.b() - 0.1);
        }
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
                    "Advent Of Code: Day 4",
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

                content.spawn((
                    TextBundle::from_section(
                        format!("Part 2: {}", part2()),
                        TextStyle {
                            font: asset_server.load("fonts/runescape_uf.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ),
                    ControlledText,
                ));
            });
        });
}

fn cleanup_day(mut commands: Commands, node_query: Query<Entity, With<Node>>) {
    for ent in node_query.iter() {
        commands.entity(ent).despawn_descendants();
    }
}

fn part1() -> i32 {
    let data = super::files::read_lines("src/day4/input.txt");

    let mut pairs_containing_others = 0;

    for row in data {
        let split: Vec<&str> = row.split(',').collect();
        let first = to_range(split[0]);
        let second = to_range(split[1]);

        if (second.start >= first.start && second.end <= first.end)
            || (first.start >= second.start && first.end <= second.end)
        {
            pairs_containing_others += 1;
        }
    }

    pairs_containing_others
}

// Probably don't need to do this but it's interesting
fn to_range(sections: &str) -> Range<i32> {
    let split: Vec<&str> = sections.split('-').collect();
    let start = split[0].to_string().parse::<i32>().unwrap();
    let end = split[1].to_string().parse::<i32>().unwrap();

    start..end
}

fn part2() -> i32 {
    let data = super::files::read_lines("src/day4/input.txt");

    let mut pairs_containing_others = 0;

    for row in data {
        let split: Vec<&str> = row.split(',').collect();
        let first = to_range(split[0]);
        let second = to_range(split[1]);

        if first.start <= second.end && second.start <= first.end {
            pairs_containing_others += 1;
        }
    }

    pairs_containing_others
}
