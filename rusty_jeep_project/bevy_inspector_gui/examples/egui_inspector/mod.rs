use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui};

#[derive(Debug, Clone, PartialEq)]
pub struct UiState {
    slider_f32: f32,
}

impl Resource for UiState {}

#[derive(Debug, Clone)]
pub struct SliderChangedF32(f32);

impl SliderChangedF32{
    pub fn get_value(&self) -> f32{
        self.0
    }
}

impl Resource for SliderChangedF32{}

pub fn setup_ui_resources(mut commands: Commands) {
    commands.insert_resource(UiState { slider_f32: 1.0 });
    commands.insert_resource(Events::<SliderChangedF32>::default())
}

/// Minimal UI example : On update draw the UI and emit CircleRadiusChanged event
pub fn on_update_ui(
    mut context: EguiContexts,
    mut on_change: ResMut<Events<SliderChangedF32>>,
    mut ui_state: ResMut<UiState>,
) {
    egui::Window::new("UI-MTON").show(context.ctx_mut(), |ui| {
        ui.label("Eat Yo Avocado Skins!");
        if ui
            .add(egui::Slider::new(&mut ui_state.slider_f32, 0.0..=100.0).text("Slider F32"))
            .changed(){
                on_change.send(SliderChangedF32(ui_state.slider_f32));
            }
    });
}