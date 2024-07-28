use bevy::prelude::*;
use wise_editor::WiseEditor;

mod follow_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WiseEditor)
        .add_systems(Startup, follow_camera::systems::spawn_camera)
        .run();
}
