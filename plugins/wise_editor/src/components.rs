use bevy::prelude::*;

use bevy::asset::UntypedAssetId;

use std::any::TypeId;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MyComponent {
    pub value: i32,
}

#[derive(Debug)]
pub enum EguiWindow {
    GameView,
    Hierarchy,
    Resources,
    Assets,
    Inspector,
}


#[derive(Eq, PartialEq)]
pub enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, UntypedAssetId),
}
