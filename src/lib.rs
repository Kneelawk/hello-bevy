#[macro_use]
extern crate log;

use bevy::{
    prelude::{
        App, Color, Commands, IntoSystem, Msaa, OrthographicCameraBundle, ResMut, Transform, Vec2,
        Vec3,
    },
    DefaultPlugins,
};
use bevy_prototype_lyon::prelude::{
    shapes, DrawMode, GeometryBuilder, ShapeColors, ShapePlugin, StrokeOptions,
};
use bevy_rapier2d::prelude::{
    ColliderBundle, ColliderMaterial, ColliderPositionSync, ColliderShape, NoUserData,
    RapierConfiguration, RapierPhysicsPlugin, RigidBodyBundle, RigidBodyType,
};
use wasm_bindgen::prelude::*;

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

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

    app.insert_resource(Msaa { samples: 8 });
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // Lyon shape renderer
    app.add_plugin(ShapePlugin);

    // Rapier physics engine
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

    app.add_startup_system(setup_world.system());

    app.run();
}

pub fn setup_world(mut commands: Commands, mut phys_config: ResMut<RapierConfiguration>) {
    phys_config.scale = 100.0;

    // Add camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Add floor
    let floor_shape = shapes::Rectangle {
        width: 1000.0,
        height: 10.0,
        origin: Default::default(),
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &floor_shape,
            ShapeColors::outlined(TRANSPARENT, Color::WHITE),
            DrawMode::Outlined {
                fill_options: Default::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
        ))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(0.0, -3.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(5.0, 0.05),
            ..Default::default()
        });

    // Add box
    let shape = shapes::Rectangle {
        width: 20.0,
        height: 20.0,
        origin: Default::default(),
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(TRANSPARENT, Color::WHITE),
            DrawMode::Outlined {
                fill_options: Default::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(0.0, 1.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.1, 0.1),
            material: ColliderMaterial {
                restitution: 0.7,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}
