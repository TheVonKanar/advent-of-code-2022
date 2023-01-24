mod body;
mod header;
mod sidebar;

use crate::ui::body::draw_body;
use crate::ui::header::draw_header;
use crate::ui::sidebar::draw_sidebar;

use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(draw_header.before(draw_sidebar))
            .add_system(draw_sidebar.before(draw_body))
            .add_system(draw_body);
    }
}
