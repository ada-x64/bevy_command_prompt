use crate::prelude::*;

mod show;

pub fn plugin(app: &mut App) {
    app.add_plugins(show::plugin);
}
