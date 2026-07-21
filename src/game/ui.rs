use bevy::prelude::*;

use crate::game::{input_mode::InputMode, scene::SceneRoot};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spwan_root)
            .add_systems(PostUpdate, (toggle_menu, menu_action));
    }
}

#[derive(Component)]
pub struct RootNode;

#[derive(Component)]
enum MenuAction {
    Resume,
    Restart,
    Quit,
}

#[derive(Component)]
pub struct Menu;

fn spwan_root(mut commands: Commands) {
    commands.spawn((
        RootNode,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                Menu,
                Visibility::Hidden,
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (MenuAction::Resume, button("Resume")),
                    (MenuAction::Restart, button("Restart")),
                    (MenuAction::Quit, button("Quit")),
                ],
            ),
            (
                Text::new("WASD: Move / Mouse: Camera Move / Space: Jump / Esc: Menu / P: Toggle Free Camera "),
                Node {
                    left: px(10),
                    bottom: px(10),
                    position_type: PositionType::Absolute,
                    ..default()
                },
            )
        ],
    ));
}

fn menu_action(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut q_menu: Query<&mut Visibility, With<Menu>>,
    q_scene: Query<Entity, With<SceneRoot>>,
    mut input_mode: ResMut<InputMode>,
) {
    for (interaction, menu_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_action {
                MenuAction::Restart => {
                    for entity in &q_scene {
                        commands.entity(entity).despawn();
                        commands.run_system_cached(crate::setup_scene);
                    }
                    for mut visibility in &mut q_menu {
                        *visibility = Visibility::Hidden;
                    }
                    *input_mode = InputMode::Game;
                }
                MenuAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuAction::Resume => {
                    for mut visibility in &mut q_menu {
                        *visibility = Visibility::Hidden;
                    }
                    *input_mode = InputMode::Game;
                }
            }
        }
    }
}

fn toggle_menu(
    button_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<Menu>>,
    mut input_mode: ResMut<InputMode>,
) {
    if button_input.just_pressed(KeyCode::Escape) {
        match *input_mode {
            InputMode::Game => {
                *input_mode = InputMode::Menu;
                for mut visibility in &mut query {
                    *visibility = Visibility::Visible;
                }
            }
            InputMode::Menu => {
                *input_mode = InputMode::Game;
                for mut visibility in &mut query {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

fn button(text: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: px(150),
            height: px(50),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::BLACK),
        children![(
            Text::new(text),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}
