use bevy::{asset::embedded_asset, prelude::*};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    embedded_asset!(app, "splash.png");
    app.run();
}
