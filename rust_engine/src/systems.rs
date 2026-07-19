use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::*;

pub fn spawn_first_floor(
    mut commands: Commands,
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
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.5, 0.0)),
    ));

    // Пол
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(50.0, 50.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.8),
            ..default()
        })),
        Transform::from_xyz(0.0, -0.01, 0.0),
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
        ));
    }

    // Игрок
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Player {
            speed: 5.0,
            health: 100,
            inventory: Vec::new(),
        },
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
) {
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
        transform.translation.x += direction.x * player.speed * time.delta_secs();
        transform.translation.z += direction.z * player.speed * time.delta_secs();
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

pub fn enemy_attack(
    enemy_q: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut player_q: Query<(&Transform, &mut Player), (With<Player>, Without<Enemy>)>,
) {
    if let Ok((player_transform, mut player)) = player_q.single_mut() {
        for enemy_transform in enemy_q.iter() {
            if player_transform
                .translation
                .distance(enemy_transform.translation)
                < 1.5
            {
                player.health -= 1;
            }
        }
    }
}

pub fn pickup_items(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<
        (&Transform, &mut Player),
        (With<Player>, Without<Enemy>, Without<Item>, Without<Npc>),
    >,
    item_q: Query<
        (Entity, &Transform, &Item),
        (With<Item>, Without<Player>, Without<Enemy>, Without<Npc>),
    >,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        if let Ok((player_transform, mut player)) = player_q.single_mut() {
            for (item_entity, item_transform, item) in item_q.iter() {
                if player_transform
                    .translation
                    .distance(item_transform.translation)
                    < 1.5
                {
                    match item.kind {
                        ItemKind::Cookie => {
                            player.health += 5;
                            println!("+5 здоровья (текущее: {})", player.health);
                        }
                        ItemKind::Coffee => {
                            player.speed += 2.0;
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
