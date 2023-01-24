use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn draw_header(mut egui_ctx: ResMut<EguiContext>) {
    egui::TopBottomPanel::top("header").show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Advent of Code 2022");
    });
}
