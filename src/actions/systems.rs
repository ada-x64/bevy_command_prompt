use bevy::{input::keyboard::keyboard_input_system, ui::ui_layout_system};

use crate::prelude::*;

fn clear_action_queue(queues: Query<&mut ConsoleActionQueue>, mut commands: Commands) {
    for mut queue in queues {
        queue
            .drain(..)
            .for_each(|(i, s)| commands.run_system_with(s, i));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        clear_action_queue
            .after(keyboard_input_system)
            .before(ui_layout_system),
    );
}
