use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: i32,
    pub inventory: Vec<ItemKind>,
}

#[derive(Component)]
pub struct Enemy {
    pub bug_type: BugType,
    pub health: i32,
    pub damage: i32,
    pub speed: f32,
    pub direction: Vec3,
}

pub enum BugType {
    NullPointer,
    MemoryLeak,
    RaceCondition,
}

#[derive(Component)]
pub struct Item {
    pub kind: ItemKind,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ItemKind {
    Cookie,
    Coffee,
    USBKey,
}

#[derive(Component, Debug)]
pub struct Npc {
    pub role: NpcRole,
    pub dialog: Vec<String>,
    pub recruited: bool,
}

#[derive(Debug)]
pub enum NpcRole {
    Sysadmin,
    Tester,
    PM,
}

#[derive(Component)]
pub struct MainCamera;
