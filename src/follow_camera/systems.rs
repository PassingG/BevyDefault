use bevy::prelude::*;
use wise_generic::components::MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 4.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..Default::default()
        },
        MainCamera,
        // PickRaycastSource,
    ));
}
