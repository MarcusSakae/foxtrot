use bevy::prelude::*;
use bevy_rapier3d::na::{Matrix3, Vector3};

use crate::actions::Actions;
use crate::player::Player;
use crate::GameState;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            // Enables the system that synchronizes your `Transform`s and `LookTransform`s.
            .add_plugin(LookTransformPlugin)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_camera));
    }
}

fn setup_camera(mut commands: Commands) {
    let eye = Vec3::default();
    let target = Vec3::default();
    commands.spawn((
        LookTransformBundle {
            transform: LookTransform::new(eye, target),
            smoother: Smoother::new(0.9), // Value between 0.0 and 1.0, higher is smoother.
        },
        Camera3dBundle::default(),
        Name::new("Camera"),
    ));
}

fn handle_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut LookTransform>,
    actions: Res<Actions>,
) {
    let max_distance = 550.0;
    let mouse_sensitivity = 0.01;
    let player = match player_query.iter().next() {
        Some(transform) => transform,
        None => return,
    };
    for mut camera in &mut camera_query {
        camera.target = player.translation;
        let mut direction = camera.look_direction().unwrap_or(Vect::Z);
        if let Some(camera_movement) = actions.camera_movement {
            // See https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
            let x_angle = mouse_sensitivity * camera_movement.x;
            let y_angle = mouse_sensitivity * camera_movement.y;

            let y_axis_rotation_matrix = Matrix3::from_row_iterator(
                #[cfg_attr(rustfmt, rustfmt::skip)]
                [
                    x_angle.cos(), 0., -x_angle.sin(),
                    0., 1., 0.,
                    x_angle.sin(), 0., x_angle.cos(),
                ].into_iter(),
            );

            let x_axis_rotation_matrix = Matrix3::from_row_iterator(
                #[cfg_attr(rustfmt, rustfmt::skip)]
                [
                    1., 0., 0.,
                    0., y_angle.cos(), -y_angle.sin(),
                    0., y_angle.sin(), y_angle.cos(),
                ].into_iter(),
            );

            direction =
                (y_axis_rotation_matrix * x_axis_rotation_matrix * Vector3::from(direction)).into();
        }

        camera.eye = camera.target - direction * max_distance;
    }
}
