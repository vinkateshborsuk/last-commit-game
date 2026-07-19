use bevy::audio::{AudioSource, PlaybackMode};
use bevy::prelude::*;

mod components;
mod systems;

use components::MainCamera;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .insert_state(AppState::Splash)
        // Заставка
        .add_systems(OnEnter(AppState::Splash), setup_splash)
        .add_systems(Update, splash_countdown.run_if(in_state(AppState::Splash)))
        .add_systems(OnExit(AppState::Splash), cleanup_splash)
        // Главное меню
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(Update, menu_control.run_if(in_state(AppState::MainMenu)))
        .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
        // Игра
        .add_systems(
            OnEnter(AppState::InGame),
            (spawn_first_floor, start_game_music).chain(),
        )
        .add_systems(OnExit(AppState::InGame), stop_game_music)
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
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    #[default]
    Splash,
    MainMenu,
    InGame,
}

#[derive(Component)]
struct SplashUI;
#[derive(Component)]
struct MainMenuUI;
#[derive(Component)]
struct MenuButton;

#[derive(Resource)]
struct SplashTimer(Timer);
#[derive(Resource, Default)]
struct GameMusic(Option<Entity>);

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        MainCamera,
        Transform::from_xyz(0.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// ---------- Заставка ----------
fn setup_splash(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            SplashUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("The Last Commit"),
                TextFont {
                    font_size: FontSize::Px(64.0),
                    ..default()
                },
                TextColor(Color::srgba(0.9, 0.9, 0.2, 0.0)),
            ));
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn splash_countdown(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut text_query: Query<&mut TextColor, With<SplashUI>>,
) {
    timer.0.tick(time.delta());
    if let Ok(mut text_color) = text_query.single_mut() {
        let progress = (timer.0.elapsed_secs() / timer.0.duration().as_secs_f32()).clamp(0.0, 1.0);
        text_color.0.set_alpha(progress);
    }
    if timer.0.just_finished() {
        next_state.set(AppState::MainMenu);
    }
}

fn cleanup_splash(mut commands: Commands, query: Query<Entity, With<SplashUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// ---------- Главное меню ----------
fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("The Last Commit"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(220.0),
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
                        Text::new("Start Game"),
                        TextFont {
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
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

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// ---------- Музыка ----------
fn start_game_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music_entity = commands
        .spawn((
            AudioPlayer::<AudioSource>(asset_server.load("audio/background.ogg")),
            PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        ))
        .id();
    commands.insert_resource(GameMusic(Some(music_entity)));
}

fn stop_game_music(mut commands: Commands, music: Res<GameMusic>) {
    if let Some(entity) = music.0 {
        commands.entity(entity).despawn();
    }
}
