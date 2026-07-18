use bevy::prelude::*;
mod components;
mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(AppState::MainMenu)
        .add_systems(Startup, setup_main_menu)
        .add_systems(
            Update,
            menu_control.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
        .add_systems(OnEnter(AppState::InGame), spawn_first_floor)
        .add_systems(
            Update,
            (
                player_movement,
                enemy_patrol,
                enemy_attack,
                pickup_items,
                interact_with_npc,
                camera_follow,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Component)]
struct MenuButton;

#[derive(Component)]
struct MainMenuUI;

fn setup_main_menu(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                    MenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Начать игру"),
                        TextFont {
                            font_size: FontSize::Px(28.0), // обратно FontSize
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn(); // просто despawn, потомки удалятся сами
    }
}

fn menu_control(
    mut next_state: ResMut<NextState<AppState>>,
    interaction: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
) {
    for inter in interaction.iter() {
        if *inter == Interaction::Pressed {
            next_state.set(AppState::InGame);
        }
    }
}

