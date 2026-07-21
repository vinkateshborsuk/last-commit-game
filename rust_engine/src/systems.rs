use bevy::prelude::*;
use bevy::scene::SceneRoot;
use rand::{thread_rng, Rng};

use crate::components::*;
use crate::AppState;

// ---------- Компоненты для HUD ----------
#[derive(Component)]
pub struct HudRoot;
#[derive(Component)]
pub struct HealthText;
#[derive(Component)]
pub struct SpeedText;
#[derive(Component)]
pub struct InventoryText;

// ---------- Система создания первого этажа ----------
pub fn spawn_first_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Освещение
    commands.spawn((AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.4),
        brightness: 0.4,
        affects_lightmapped_meshes: true,
    },));

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.8),
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.5, 0.0)),
    ));

    let mut rng = thread_rng();

    // Стены
    let wall_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.4, 0.4),
        ..default()
    });
    for _ in 0..30 {
        let x = rng.gen_range(-20.0..20.0);
        let z = rng.gen_range(-20.0..20.0);
        commands.spawn((
            Mesh3d(wall_mesh.clone()),
            MeshMaterial3d(wall_material.clone()),
            Transform::from_xyz(x, 0.5, z),
            GameEntity,
            Wall,
        ));
    }

    // 1. Загрузка текстуры с режимом повторения (Repeat)
    let floor_texture: Handle<Image> = asset_server.load_with_settings(
        "textures/floor.jpg",
        |settings: &mut bevy::image::ImageLoaderSettings| {
            settings.sampler =
                bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
                    address_mode_u: bevy::image::ImageAddressMode::Repeat,
                    address_mode_v: bevy::image::ImageAddressMode::Repeat,
                    ..default()
                });
        },
    );

    // 2. Создаем маленькую плоскость, но масштабируем её через Transform для тайлинга
    commands.spawn((
        // Размер плоскости 1x1 метр. UV-координаты привязаны к этому размеру.
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            // В новых версиях Bevy цвета берутся из модуля `color` (например, Srgb или палитр)
            base_color: bevy::color::Color::from(bevy::color::palettes::css::WHITE),
            base_color_texture: Some(floor_texture),
            ..default()
        })),
        // Масштабируем до 50x50 метров.
        // Поскольку текстура зациклена, она повторится 50 раз по осям X и Z!
        Transform::from_xyz(0.0, -0.01, 0.0).with_scale(Vec3::new(50.0, 1.0, 50.0)),
        GameEntity,
    ));

    // ----- Игрок (GLB-модель) -----
    commands.spawn((
        SceneRoot(asset_server.load("models/player.glb#Scene0")),
        Transform::from_xyz(0.0, 2.5, 0.0).with_scale(Vec3::splat(1.5)),
        Player {
            speed: 5.0,
            health: 100,
            inventory: Vec::new(),
        },
        GameEntity,
    ));

    // Враги
    let enemy_mesh = meshes.add(Sphere::new(0.4));
    for i in 0..5 {
        let x = rng.gen_range(-15.0..15.0);
        let z = rng.gen_range(-15.0..15.0);
        commands.spawn((
            Mesh3d(enemy_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                ..default()
            })),
            Transform::from_xyz(x, 0.5, z),
            Enemy {
                bug_type: match i % 3 {
                    0 => BugType::NullPointer,
                    1 => BugType::MemoryLeak,
                    _ => BugType::RaceCondition,
                },
                health: 30,
                damage: 10,
                speed: 2.0,
                direction: Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0))
                    .normalize_or_zero(),
            },
            GameEntity,
        ));
    }

    // Предметы
    let item_mesh = meshes.add(Cylinder::new(0.3, 0.1));
    commands.spawn((
        Mesh3d(item_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            ..default()
        })),
        Transform::from_xyz(-3.0, 0.5, -3.0),
        Item {
            kind: ItemKind::Cookie,
        },
        GameEntity,
    ));
    commands.spawn((
        Mesh3d(item_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 0.0),
            ..default()
        })),
        Transform::from_xyz(3.0, 0.5, 3.0),
        Item {
            kind: ItemKind::Coffee,
        },
        GameEntity,
    ));

    // NPC
    let npc_mesh = meshes.add(Cylinder::new(0.4, 0.8));
    commands.spawn((
        Mesh3d(npc_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(-5.0, 0.5, 5.0),
        Npc {
            role: NpcRole::Sysadmin,
            dialog: vec!["Ну и бардак... Серверная в подвале, ключ USB-шный потерял.".into()],
            recruited: false,
        },
        GameEntity,
    ));
    commands.spawn((
        Mesh3d(npc_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.647, 0.0),
            ..default()
        })),
        Transform::from_xyz(5.0, 0.5, -5.0),
        Npc {
            role: NpcRole::Tester,
            dialog: vec!["Каждый баг — это фича, если документацию правильно написать.".into()],
            recruited: false,
        },
        GameEntity,
    ));
}

// ---------- Системы обновления ----------

pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (&mut Transform, &Player),
        (
            With<Player>,
            Without<Enemy>,
            Without<Item>,
            Without<Npc>,
            Without<MainCamera>,
        ),
    >,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
) {
    let player_radius = 0.5;
    let half_extent = 0.5;
    let boundary = 24.5;

    for (mut transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            direction.z -= 1.0;
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            direction.z += 1.0;
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let delta = time.delta_secs();
        let speed = player.speed;

        // Движение по X
        let mut new_x = transform.translation.x + direction.x * speed * delta;
        if new_x > boundary {
            new_x = boundary;
        } else if new_x < -boundary {
            new_x = -boundary;
        }

        let mut collides = false;
        let test_pos_x = Vec3::new(new_x, transform.translation.y, transform.translation.z);
        for wall_transform in wall_query.iter() {
            let wall_pos = wall_transform.translation;
            let dx = (test_pos_x.x - wall_pos.x).abs();
            let dz = (test_pos_x.z - wall_pos.z).abs();
            if dx < (player_radius + half_extent) && dz < (player_radius + half_extent) {
                collides = true;
                break;
            }
        }
        if !collides {
            transform.translation.x = new_x;
        }

        // Движение по Z
        let mut new_z = transform.translation.z + direction.z * speed * delta;
        if new_z > boundary {
            new_z = boundary;
        } else if new_z < -boundary {
            new_z = -boundary;
        }

        collides = false;
        let test_pos_z = Vec3::new(transform.translation.x, transform.translation.y, new_z);
        for wall_transform in wall_query.iter() {
            let wall_pos = wall_transform.translation;
            let dx = (test_pos_z.x - wall_pos.x).abs();
            let dz = (test_pos_z.z - wall_pos.z).abs();
            if dx < (player_radius + half_extent) && dz < (player_radius + half_extent) {
                collides = true;
                break;
            }
        }
        if !collides {
            transform.translation.z = new_z;
        }
    }
}

pub fn enemy_patrol(
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &mut Enemy),
        (
            With<Enemy>,
            Without<Player>,
            Without<Item>,
            Without<Npc>,
            Without<MainCamera>,
        ),
    >,
) {
    let mut rng = thread_rng();
    for (mut transform, mut enemy) in query.iter_mut() {
        transform.translation.x += enemy.direction.x * enemy.speed * time.delta_secs();
        transform.translation.z += enemy.direction.z * enemy.speed * time.delta_secs();

        if rng.gen_bool(0.02) {
            enemy.direction = Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0))
                .normalize_or_zero();
        }

        if transform.translation.x.abs() > 20.0 || transform.translation.z.abs() > 20.0 {
            enemy.direction = -enemy.direction;
        }
    }
}

// ------------- ИСПРАВЛЕННАЯ СИСТЕМА АТАКИ ВРАГА (горизонтальное расстояние) -------------
pub fn enemy_attack(
    enemy_q: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut player_q: Query<(&Transform, &mut Player), (With<Player>, Without<Enemy>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((player_transform, mut player)) = player_q.single_mut() {
        let player_pos = player_transform.translation;
        for enemy_transform in enemy_q.iter() {
            // Расстояние только по XZ
            let dx = player_pos.x - enemy_transform.translation.x;
            let dz = player_pos.z - enemy_transform.translation.z;
            let dist_sq = dx * dx + dz * dz;
            if dist_sq < 1.5 * 1.5 {
                player.health -= 1;
                if player.health <= 0 {
                    next_state.set(AppState::GameOver);
                }
            }
        }
    }
}

// ------------- ИСПРАВЛЕННАЯ СИСТЕМА ПОДБОРА ПРЕДМЕТОВ (горизонтальное расстояние) -------------
pub fn pickup_items(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<
        (&Transform, &mut Player),
        (With<Player>, Without<Enemy>, Without<Item>, Without<Npc>),
    >,
    mut item_q: Query<
        (Entity, &mut Transform, &Item),
        (With<Item>, Without<Player>, Without<Enemy>, Without<Npc>),
    >,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        if let Ok((player_transform, mut player)) = player_q.single_mut() {
            let player_pos = player_transform.translation;
            for (item_entity, mut item_transform, item) in item_q.iter_mut() {
                let dx = player_pos.x - item_transform.translation.x;
                let dz = player_pos.z - item_transform.translation.z;
                let dist_sq = dx * dx + dz * dz;
                if dist_sq < 1.5 * 1.5 {
                    match item.kind {
                        ItemKind::Cookie => {
                            player.health += 5;
                            println!("+5 здоровья (текущее: {})", player.health);
                            // Оттолкнуть печеньку
                            let mut rng = thread_rng();
                            let random_dir =
                                Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0))
                                    .normalize_or_zero();
                            let distance = rng.gen_range(3.0..8.0);
                            let new_pos = item_transform.translation + random_dir * distance;
                            item_transform.translation = new_pos;
                        }
                        ItemKind::Coffee => {
                            player.speed += 2.0;
                            println!("Скорость увеличена до {}", player.speed);
                            commands.entity(item_entity).despawn();
                        }
                        ItemKind::USBKey => {
                            player.inventory.push(ItemKind::USBKey);
                            println!("USB-ключ подобран!");
                            commands.entity(item_entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

// Остальные системы (interact_with_npc, camera_follow, HUD, cleanup) без изменений.
// Они приведены ниже для полноты.

pub fn interact_with_npc(
    keys: Res<ButtonInput<KeyCode>>,
    player_q: Query<&Transform, (With<Player>, Without<Npc>)>,
    npc_q: Query<(&Transform, &Npc), (With<Npc>, Without<Player>)>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        if let Ok(player_transform) = player_q.single() {
            for (npc_transform, npc) in npc_q.iter() {
                if player_transform
                    .translation
                    .distance(npc_transform.translation)
                    < 2.0
                {
                    println!("{:?}: {}", npc.role, npc.dialog[0]);
                }
            }
        }
    }
}

pub fn camera_follow(
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_q: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let (Ok(player_transform), Ok(mut cam_transform)) =
        (player_q.single(), camera_q.single_mut())
    {
        cam_transform.translation.x = player_transform.translation.x;
        cam_transform.translation.z = player_transform.translation.z + 20.0;
    }
}

// ---------- Системы HUD ----------
pub fn setup_hud(mut commands: Commands) {
    println!("setup_hud called");
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            HudRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Health: 100"),
                TextFont::default(),
                TextColor(Color::WHITE),
                HealthText,
            ));
            parent.spawn((
                Text::new("Speed: 5.0"),
                TextFont::default(),
                TextColor(Color::WHITE),
                SpeedText,
            ));
            parent.spawn((
                Text::new("Inventory: "),
                TextFont::default(),
                TextColor(Color::WHITE),
                InventoryText,
            ));
        });
}

pub fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_hud(
    player_query: Query<&Player, (With<Player>, Without<HudRoot>)>,
    mut health_text: Query<
        &mut Text,
        (With<HealthText>, Without<SpeedText>, Without<InventoryText>),
    >,
    mut speed_text: Query<
        &mut Text,
        (With<SpeedText>, Without<HealthText>, Without<InventoryText>),
    >,
    mut inventory_text: Query<
        &mut Text,
        (With<InventoryText>, Without<HealthText>, Without<SpeedText>),
    >,
) {
    if let Ok(player) = player_query.single() {
        if let Ok(mut text) = health_text.single_mut() {
            text.0 = format!("Health: {}", player.health);
        }
        if let Ok(mut text) = speed_text.single_mut() {
            text.0 = format!("Speed: {:.1}", player.speed);
        }
        if let Ok(mut text) = inventory_text.single_mut() {
            let inv_str = if player.inventory.is_empty() {
                "empty".to_string()
            } else {
                player
                    .inventory
                    .iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            text.0 = format!("Inventory: {}", inv_str);
        }
    }
}

// ---------- Очистка игрового мира ----------
pub fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
