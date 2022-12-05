use std::collections::HashMap;

use crate::GameState;
use bevy::prelude::*;

pub struct Day1Plugin;

#[derive(Component)]
pub struct ButtonActive(bool);

impl Plugin for Day1Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day1).with_system(setup_day1))
            .add_system_set(SystemSet::on_update(GameState::Day1).with_system(day1))
            .add_system_set(SystemSet::on_exit(GameState::Day1).with_system(cleanup_day1));
    }
}

fn day1(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, _color, children) in &mut interaction_query {
        let _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Menu).unwrap();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_day1(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn cleanup_day1(mut commands: Commands, node_query: Query<Entity, With<Node>>) {
    for ent in node_query.iter() {
        commands.entity(ent).despawn_descendants();
    }
}

pub fn part1() -> i64 {
    let mut elf_index = 0;
    let mut elf_calories: HashMap<i64, Vec<i64>> = HashMap::new();
    elf_calories.insert(elf_index, Vec::new());

    let input = super::files::read_lines("src/day1/input.txt");

    // Could do this in one loop but the next one probably requires
    // some other transformation so I'm separating them
    for e in input.iter() {
        if e == "" {
            // New elf new vec
            elf_index += 1;
            elf_calories.insert(elf_index, Vec::new());

            continue;
        }

        let calories = e.parse::<i64>().unwrap();
        elf_calories.get_mut(&elf_index).unwrap().push(calories);
    }

    let sums = elf_calories
        .iter()
        .map(|(_elf, balances)| balances.iter().sum::<i64>());

    sums.into_iter().max().unwrap()
}

pub fn part2() -> i64 {
    let mut elf_index = 0;
    let mut elf_calories: HashMap<i64, Vec<i64>> = HashMap::new();
    elf_calories.insert(elf_index, Vec::new());

    let input = super::files::read_lines("src/day1/input.txt");

    for e in input.iter() {
        if e == "" {
            // New elf new vec
            elf_index += 1;
            elf_calories.insert(elf_index, Vec::new());

            continue;
        }

        let calories = e.parse::<i64>().unwrap();
        elf_calories.get_mut(&elf_index).unwrap().push(calories);
    }

    let mut sums = elf_calories
        .iter()
        .map(|(_elf, balances)| balances.iter().sum::<i64>())
        .collect::<Vec<i64>>();
    sums.sort();

    sums.iter().rev().take(3).sum()
}
