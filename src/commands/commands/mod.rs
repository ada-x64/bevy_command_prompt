use crate::prelude::*;

mod clear;
mod show;

pub fn plugin(app: &mut App) {
    app.add_plugins((show::plugin, clear::plugin));
}
