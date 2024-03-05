use bevy::prelude::*;

mod components;
mod display;
mod movement;
mod scoring;
mod setup;

use components::{Player, Velocity};

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 1.;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1.;
        } else {
            velocity.0.y = 0.;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<scoring::Score>()
        .add_event::<scoring::Scored>()
        .add_systems(Startup, (
            setup::spawn_camera,
            setup::spawn_ball,
            setup::spawn_paddles,
            setup::spawn_gutters,
            setup::spawn_scoreboard
            ))
        .add_systems(Update, (
            handle_player_input,
            movement::move_ball,
            movement::move_ai.after(movement::move_ball),
            scoring::detect_scoring,
            scoring::reset_ball.after(scoring::detect_scoring),
            scoring::update_score.after(scoring::detect_scoring),
            movement::move_paddles.after(handle_player_input),
            movement::handle_collisions.after(movement::move_ball),
            scoring::update_scoreboard.after(scoring::detect_scoring),
            display::project_positions.after(movement::move_ball),
            bevy::window::close_on_esc,
            ))
        .run();
}
