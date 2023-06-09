use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, bevy_egui::EguiPlugin, egui, egui::Context};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PanelType {
    Top,
    Bottom,
    Left,
    Right,
}

pub type PanelBuilder = Arc<dyn Fn(&mut egui::Ui) + Send + Sync + 'static>;
pub type PanelBuilderHash = HashMap<PanelType, PanelData>;

#[derive(Clone)]
pub struct PanelData {
    builder: PanelBuilder,
}

impl PanelData {
    pub fn new(builder: PanelBuilder) -> Self {
        PanelData { builder }
    }
}

#[derive(Debug, Default, Resource)]
struct OccupiedSpace {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

// Shared between toggle_dock and draw_dock, so it needs to be a resource
// and can't be a Local<T>
#[derive(Debug, Resource)]
struct IsVisible {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

// Set default values for IsVisible
impl Default for IsVisible {
    fn default() -> Self {
        IsVisible {
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }
}

/* ------ DockPlugin ------- */

// @note : Add a field to `DockPlugin` to store the closure:
pub struct DockPlugin {
    panels: PanelBuilderHash,
}
// @note :  Update the `DockPlugin` implementation to store the closure in the plugin
impl DockPlugin {
    pub fn new(panels: PanelBuilderHash) -> Self {
        DockPlugin { panels }
    }
}

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        let panels = self.panels.clone();

        app.add_plugin(EguiPlugin)
            .insert_resource(IsVisible::default())
            .insert_resource(OccupiedSpace::default())
            .insert_resource(DrawDockParams {
                panel_builders: panels,
            })
            .add_system(toggle_dock)
            .add_system(draw_dock);
    }
}

#[derive(Resource)]
struct DrawDockParams {
    panel_builders: PanelBuilderHash,
}

impl Default for DrawDockParams {
    fn default() -> Self {
        Self {
            panel_builders: HashMap::new(),
        }
    }
}

/* ------ Panel Draw Functions ------- */

/// toggles docking panels visibility
fn toggle_dock(mut is_visible: ResMut<IsVisible>, key_input: Res<Input<KeyCode>>) {
    let panel_type = if key_input.just_released(KeyCode::Up) {
        Some(PanelType::Top)
    } else if key_input.just_released(KeyCode::Down) {
        Some(PanelType::Bottom)
    } else if key_input.just_released(KeyCode::Left) {
        Some(PanelType::Left)
    } else if key_input.just_released(KeyCode::Right) {
        Some(PanelType::Right)
    } else {
        None
    };

    if let Some(panel_type) = panel_type {
        match panel_type {
            PanelType::Top => is_visible.top = !is_visible.top,
            PanelType::Bottom => is_visible.bottom = !is_visible.bottom,
            PanelType::Left => is_visible.left = !is_visible.left,
            PanelType::Right => is_visible.right = !is_visible.right,
        }
    }
}

/// draws docking panesl
/// - contexts: EguiContexts            // egui context
/// - is_visible: Res<IsVisible>        // is visible
/// - params: Res<DrawDockParams>       // draw dock params
fn draw_dock(
    mut contexts: EguiContexts,
    is_visible: Res<IsVisible>,
    params: Res<DrawDockParams>,
    mut o_space: ResMut<OccupiedSpace>,
) {
    let ctx = contexts.ctx_mut();

    for (panel_type, panel_data) in &params.panel_builders {
        let p_label = format!("{} Panel", format!("{:?}", panel_type).to_uppercase());

        if (*panel_type == PanelType::Top && is_visible.top)
            || (*panel_type == PanelType::Bottom && is_visible.bottom)
            || (*panel_type == PanelType::Left && is_visible.left)
            || (*panel_type == PanelType::Right && is_visible.right)
        {
            let size = panel_builder(ctx, &panel_data.builder, panel_type, p_label);
            match *panel_type {
                PanelType::Top => {
                    o_space.top = size.y;
                }
                PanelType::Bottom => {
                    o_space.bottom = size.y;
                }
                PanelType::Left => {
                    o_space.left = size.x;
                }
                PanelType::Right => {
                    o_space.right = size.x;
                }
            }
        }
    }
}

/* ------ Panel Utilitie Functions ------- */

/// Returns an egui::Vec2 representing the size of the panel
/// - ctx: &mut Context // the egui context to build the panel in
/// // the panel builder closure
/// - panel_builder: &Arc<dyn Fn(&mut egui::Ui) + Send + Sync + 'static>
/// - p_type: &str      // a string slice for the panel TYPE to build
/// - p_label: String   // a string for the panel LABEL to build
fn panel_builder(
    ctx: &mut Context,
    panel_builder: &PanelBuilder,
    p_type: &PanelType,
    p_label: String,
) -> egui::Vec2 {
    let ui = match *p_type {
        PanelType::Top => egui::TopBottomPanel::top(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                panel_builder(ui);
            }),
        PanelType::Bottom => {
            egui::TopBottomPanel::bottom(p_label)
                .resizable(true)
                .show(ctx, |ui| {
                    panel_builder(ui);
                })
        }
        PanelType::Left => egui::SidePanel::left(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                panel_builder(ui);
            }),
        PanelType::Right => egui::SidePanel::right(p_label)
            .resizable(true)
            .show(ctx, |ui| {
                panel_builder(ui);
            }),
    };

    ui.response.rect.size()
}
