/*
--- Egui Inspector Module ---

Initializes Resources
 - UiState
 - SliderChangedF32

Implements
 -`on_update_ui` function to display a UI with an adjustable slider.
 -`SliderChangedF32` event is sent when the slider is changed

*/

use bevy::prelude::*;
// from bevy_inspector_gui we are re-exporting sub-crates :
// - bevy_egui::EguiContexts : draw surface for the UI, pub access to Window::new
// - egui : the UI library
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui, egui::Color32, egui::RichText};

/// UiState struct to hold the UI state
// Resource trait allows insertion into Bevy world resource map
#[derive(Debug, Resource)]
pub struct UiState {
    slider_f32: f32,
}

/// SliderChangedF32 event
// Resource trait allows insertion into Bevy world resource map
#[derive(Debug, Resource)]
pub struct SliderChangedF32 {
    value: f32,
}

impl SliderChangedF32 {
    // read access to the value when the event is received
    pub fn get_value(&self) -> f32 {
        self.value
    }
}

/// Insert inspector resources, queue up Commands to insert resources:
/// - UiState : holds the UI state
/// - Events<SliderChangedF32> : event channel for the slider
pub fn insert_inspector_resources(mut commands: Commands) {
    commands.insert_resource(UiState { slider_f32: 1.0 });
    commands.insert_resource(Events::<SliderChangedF32>::default())
}

/// Draw the inspector UI
pub fn draw_inspector(
    mut context: EguiContexts,
    mut on_change: ResMut<Events<SliderChangedF32>>,
    mut ui_state: ResMut<UiState>,
    label_inspector: &str,
    label_slider: &str,
) {
    // create a new egui Window on function call
    egui::Window::new(label_inspector)
        // call show() method on egui::Window
        .show(
            // pass a mutable reference to the EguiContexts
            context.ctx_mut(),
            // 2nd argument is a closure that with mutabe ref to egui::Ui struct
            // - Ui struct allows us to build a UI within the closure
            |ui| {
                // add a label widget to the UI
                ui.label(RichText::new("Egui Inspector Module").color(Color32::GOLD));
                // add a slider widget to the UI
                if ui
                    .add(
                        egui::Slider::new(&mut ui_state.slider_f32, 0.0..=100.0).text(label_slider),
                    )
                    // check if the slider widget's value has changed
                    .changed()
                {
                    // if the value has changed, send the SliderChangedF32 event
                    // with the updated value
                    on_change.send(SliderChangedF32 {
                        value: ui_state.slider_f32,
                    });
                }
                if ui.button("SetSliderChanged=69").clicked() {
                    on_change.send(SliderChangedF32 {
                        value: 69.0, // very mature
                    })
                }
            },
        );
}
