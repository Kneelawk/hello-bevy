mod game;

#[macro_use]
extern crate log;

use crate::game::HelloBevyPlugin;
use bevy::{
    prelude::{App, ClearColor, Color, Msaa},
    DefaultPlugins,
};
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn run() {
    // Setup the WASM logger.
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(Default::default());

    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(feature = "panic_hook")]
    console_error_panic_hook::set_once();

    // Everything's setup, let's goooooo!!!
    info!("Hello Bevy!");

    // Initialize Bevy
    let mut app = App::build();

    app.insert_resource(ClearColor(Color::BLACK));
    app.insert_resource(Msaa { samples: 8 });
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // Lyon shape renderer
    app.add_plugin(ShapePlugin);

    // Rapier physics engine
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

    // Actual game logic
    app.add_plugin(HelloBevyPlugin);

    app.run();
}
