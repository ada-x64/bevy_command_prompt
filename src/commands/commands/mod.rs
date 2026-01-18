use crate::prelude::*;

mod echo;
mod show;

pub fn plugin(app: &mut App) {
    app.add_plugins((show::plugin, echo::plugin));
}
