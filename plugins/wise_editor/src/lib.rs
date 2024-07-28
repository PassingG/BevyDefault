use bevy::prelude::*;
use bevy_egui::EguiSet;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use systems::{set_camera_viewport, set_gizmo_mode, show_ui_system};

mod components;
mod systems;
mod tab_viewer;
mod ui_state;

pub struct WiseEditor;

impl Plugin for WiseEditor {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultInspectorConfigPlugin)
            .add_plugins(bevy_egui::EguiPlugin)
            .insert_resource(ui_state::UiState::new())
            .add_systems(
                PostUpdate,
                show_ui_system
                    .before(EguiSet::ProcessOutput)
                    .before(bevy::transform::TransformSystem::TransformPropagate),
            )
            .add_systems(PostUpdate, set_camera_viewport.after(show_ui_system))
            .add_systems(Update, set_gizmo_mode)
            .register_type::<components::MyComponent>()
            .register_type::<Option<Handle<Image>>>()
            .register_type::<AlphaMode>();
    }
}
