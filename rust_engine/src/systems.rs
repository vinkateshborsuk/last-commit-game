use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::*;

pub fn spawn_first_floor(mut commands: Commands) {
    // Камера уже создана, новую не добавляем

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.8, 0.8),
            custom_size: Some(Vec2::new(1024.0, 1024.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let mut rng = thread_rng();

    for _ in 0..20 {
        let x = rng.gen_range(-500.0..500.0);
        let y = rng.gen_range(-500.0..500.0);
        commands.spawn((
            Sprite {
                color: Color::srgb(0.4, 0.4, 0.4),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.5),
        ));
    }

    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player {
            speed: 200.0,
            health: 100,
            inventory: Vec::new(),
        },
    ));

    for i in 0..3 {
        let x = rng.gen_range(-400.0..400.0);
        let y = rng.gen_range(-400.0..400.0);
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
            Enemy {
                bug_type: match i {
                    0 => BugType::NullPointer,
                    1 => BugType::MemoryLeak,
                    _ => BugType::RaceCondition,
                },
                health: 30,
                damage: 10,
                speed: 50.0,
                direction: Vec2::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
                .normalize_or_zero(),
            },
        ));
    }

    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(-100.0, -100.0, 1.0),
        Item {
            kind: ItemKind::Cookie,
        },
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 1.0),
        Item {
            kind: ItemKind::Coffee,
        },
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, 200.0, 1.0),
        Npc {
            role: NpcRole::Sysadmin,
            dialog: vec![
                "Ну и бардак... Серверная в подвале, ключ USB-шный потерял.".to_string(),
            ],
            recruited: false,
        },
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.647, 0.0),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(200.0, -200.0, 1.0),
        Npc {
            role: NpcRole::Tester,
            dialog: vec!["Каждый баг — это фича, если документацию правильно написать."
                .to_string()],
            recruited: false,
        },
    ));
}

// --- Системы ---

pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
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
        transform.translation += direction * player.speed * time.delta_secs();
    }
}

pub fn camera_follow(
    player_q: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_q: Query<&mut Transform, With<Camera>>,
) {
    // single() и single_mut() возвращают Result
    if let (Ok(player_transform), Ok(mut cam_transform)) = (player_q.single(), camera_q.single_mut()) {
        cam_transform.translation.x = player_transform.translation.x;
        cam_transform.translation.y = player_transform.translation.y;
    }
}

pub fn enemy_patrol(time: Res<Time>, mut enemy_q: Query<(&mut Transform, &mut Enemy)>) {
    let mut rng = thread_rng();
    for (mut transform, mut enemy) in enemy_q.iter_mut() {
        transform.translation.x += enemy.direction.x * enemy.speed * time.delta_secs();
        transform.translation.y += enemy.direction.y * enemy.speed * time.delta_secs();

        if rng.gen_bool(0.02) {
            enemy.direction = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                .normalize_or_zero();
        }

        if transform.translation.x.abs() > 500.0 || transform.translation.y.abs() > 500.0 {
            enemy.direction = -enemy.direction;
        }
    }
}

pub fn enemy_attack(
    mut player_q: Query<(&Transform, &mut Player)>,
    enemy_q: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    if let Ok((player_transform, mut player)) = player_q.single_mut() {
        for enemy_transform in enemy_q.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance < 30.0 {
                player.health -= 1;
            }
        }
    }
}

pub fn pickup_items(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&Transform, &mut Player)>,
    item_q: Query<(Entity, &Transform, &Item)>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        if let Ok((player_transform, mut player)) = player_q.single_mut() {
            for (item_entity, item_transform, item) in item_q.iter() {
                let distance = player_transform.translation.distance(item_transform.translation);
                if distance < 30.0 {
                    match item.kind {
                        ItemKind::Cookie => {
                            player.health += 5;
                            println!("+5 здоровья (текущее: {})", player.health);
                        }
                        ItemKind::Coffee => {
                            player.speed += 50.0;
                            println!("Скорость увеличена до {}", player.speed);
                        }
                        ItemKind::USBKey => {
                            player.inventory.push(ItemKind::USBKey);
                            println!("USB-ключ подобран!");
                        }
                    }
                    commands.entity(item_entity).despawn();
                }
            }
        }
    }
}

pub fn interact_with_npc(
    keys: Res<ButtonInput<KeyCode>>,
    player_q: Query<&Transform, With<Player>>,
    npc_q: Query<(&Transform, &Npc)>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        if let Ok(player_transform) = player_q.single() {
            for (npc_transform, npc) in npc_q.iter() {
                let distance = player_transform.translation.distance(npc_transform.translation);
                if distance < 40.0 {
                    println!("{:?}: {}", npc.role, npc.dialog[0]);
                }
            }
        }
    }
}

