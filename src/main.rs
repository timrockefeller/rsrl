use bevy::prelude::*;
mod game;
fn main() {
    let mut app = App::new();
    app
    .add_plugins(DefaultPlugins)
    .add_plugins(game::GameModule)
    .run();
}
