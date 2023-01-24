mod constants;
mod solvers;
mod ui;

use crate::ui::MenuPlugin;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_pkv::PkvStore;
use solvers::SolversPlugin;

fn main() {
    App::new()
        .insert_resource(PkvStore::new("TheVonKanar", "AoC2022"))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(SolversPlugin)
        .run();
}
