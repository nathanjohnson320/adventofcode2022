mod menu;
mod files;
mod day1;
mod day2;
mod day3;

use bevy::{prelude::*, winit::WinitSettings};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Advent of Code 2022".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_state(GameState::Menu)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(menu::MainMenuPlugin)
        .add_plugin(day1::Day1Plugin)
        .add_plugin(day2::Day2Plugin)
        .add_plugin(day3::Day3Plugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
