//! This module contains the actual hello-bevy game content.

use bevy::{
    app::AppBuilder,
    prelude::{
        Color, Commands, IntoSystem, OrthographicCameraBundle, Plugin, ResMut, Transform, Vec2,
        Vec3,
    },
};
use bevy_prototype_lyon::prelude::{shapes, DrawMode, GeometryBuilder, ShapeColors, StrokeOptions};
use bevy_rapier2d::prelude::{
    ColliderBundle, ColliderMaterial, ColliderPositionSync, ColliderShape, RapierConfiguration,
    RigidBodyBundle, RigidBodyType,
};

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

pub struct HelloBevyPlugin;

impl Plugin for HelloBevyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_world.system());
    }
}

fn setup_world(mut commands: Commands, mut phys_config: ResMut<RapierConfiguration>) {
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
