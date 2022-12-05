use crate::GameState;
use bevy::prelude::*;

pub struct Day5Plugin;

#[derive(Component)]
pub struct ControlledText;

impl Plugin for Day5Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day5).with_system(setup_day))
            .add_system_set(SystemSet::on_update(GameState::Day5).with_system(day))
            .add_system_set(SystemSet::on_update(GameState::Day5).with_system(keyboard))
            .add_system_set(SystemSet::on_exit(GameState::Day5).with_system(cleanup_day));
    }
}

fn day(
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
    let crates = [
        Vec::from(['Z', 'N']),
        Vec::from(['M', 'C', 'D']),
        Vec::from(['P'])
    ];

    let data = super::files::read_lines("src/day5/test.txt");

    for row in data {
    }

    0
}

fn part2() -> i32 {
    0
}
