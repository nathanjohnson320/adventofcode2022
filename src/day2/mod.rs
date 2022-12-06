use self::Play::{PAPER, ROCK, SCISSORS};
use std::str::FromStr;

use crate::GameState;
use bevy::prelude::*;

pub struct Day2Plugin;

impl Plugin for Day2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day2).with_system(setup_day))
            .add_system_set(SystemSet::on_exit(GameState::Day2).with_system(cleanup_day));
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
                    "Advent Of Code: Day 2",
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

#[derive(Debug, PartialEq)]
enum Play {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Play {
    fn to_score(&self) -> i32 {
        match *self {
            ROCK => 1,
            PAPER => 2,
            SCISSORS => 3,
        }
    }
}

impl FromStr for Play {
    type Err = ();
    fn from_str(input: &str) -> Result<Play, Self::Err> {
        match input {
            "X" => Ok(ROCK),
            "Y" => Ok(PAPER),
            "Z" => Ok(SCISSORS),
            "A" => Ok(ROCK),
            "B" => Ok(PAPER),
            "C" => Ok(SCISSORS),
            "rock" => Ok(ROCK),
            "paper" => Ok(PAPER),
            "scissors" => Ok(SCISSORS),
            _ => Err(()),
        }
    }
}

fn part1() -> i32 {
    let input = super::files::read_lines("src/day2/input.txt");
    let rows: Vec<Vec<Play>> = input
        .iter()
        .map(|row| row.split(' ').map(|s| Play::from_str(s).unwrap()).collect())
        .collect();

    rows.iter().map(|row| calculate_score(row)).sum()
}

fn calculate_score(row: &Vec<Play>) -> i32 {
    let them = &row[0];
    let you = &row[1];

    // draws are 3
    if you == them {
        return 3 + you.to_score();
    }

    let choices = [ROCK, PAPER, SCISSORS];
    let them_index = choices.iter().position(|x| x == them).unwrap();
    let you_index = choices.iter().position(|x| x == you).unwrap();

    if them_index == choices.len() - 1 && you_index == 0 {
        return 6 + you.to_score();
    }

    if you_index == choices.len() - 1 && them_index == 0 {
        return you.to_score();
    }
    if them_index > you_index {
        return you.to_score();
    } else {
        return 6 + you.to_score();
    }
}

#[derive(Debug, PartialEq)]
enum Play2 {
    ROCK,
    PAPER,
    SCISSORS,
    LOSE,
    DRAW,
    WIN,
}

impl Play2 {
    fn to_score(&self) -> i32 {
        match *self {
            Play2::ROCK => 1,
            Play2::PAPER => 2,
            Play2::SCISSORS => 3,
            _ => 0,
        }
    }
}

impl FromStr for Play2 {
    type Err = ();
    fn from_str(input: &str) -> Result<Play2, Self::Err> {
        match input {
            "X" => Ok(Play2::LOSE),
            "Y" => Ok(Play2::DRAW),
            "Z" => Ok(Play2::WIN),
            "A" => Ok(Play2::ROCK),
            "B" => Ok(Play2::PAPER),
            "C" => Ok(Play2::SCISSORS),
            "rock" => Ok(Play2::ROCK),
            "paper" => Ok(Play2::PAPER),
            "scissors" => Ok(Play2::SCISSORS),
            _ => Err(()),
        }
    }
}

pub fn part2() -> i32 {
    let input = super::files::read_lines("src/day2/input.txt");
    let rows: Vec<Vec<Play2>> = input
        .iter()
        .map(|row| {
            row.split(' ')
                .map(|s| Play2::from_str(s).unwrap())
                .collect()
        })
        .collect();

    rows.iter().map(|row| calculate_score_2(row)).sum()
}

fn find_play<'a>(their_play: &'a Play2, desired_result: &'a Play2) -> &'a Play2 {
    match desired_result {
        Play2::WIN => {
            if *their_play == Play2::ROCK {
                &Play2::PAPER
            } else if *their_play == Play2::PAPER {
                &Play2::SCISSORS
            } else {
                &Play2::ROCK
            }
        }
        Play2::LOSE => {
            if *their_play == Play2::ROCK {
                &Play2::SCISSORS
            } else if *their_play == Play2::PAPER {
                &Play2::ROCK
            } else {
                &Play2::PAPER
            }
        }
        Play2::DRAW => their_play,
        _ => &Play2::PAPER,
    }
}

// total score =
// shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// +  score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
fn calculate_score_2(row: &Vec<Play2>) -> i32 {
    let them = &row[0];
    let result = &row[1];
    let you = find_play(them, result);

    if you == them {
        return 3 + you.to_score();
    }

    let choices = [Play2::ROCK, Play2::PAPER, Play2::SCISSORS];
    let them_index = choices.iter().position(|x| x == them).unwrap();
    let you_index = choices.iter().position(|x| x == you).unwrap();

    if them_index == choices.len() - 1 && you_index == 0 {
        return 6 + you.to_score();
    }

    if you_index == choices.len() - 1 && them_index == 0 {
        return you.to_score();
    }
    if them_index > you_index {
        return you.to_score();
    } else {
        return 6 + you.to_score();
    }
}
