use bevy::prelude::*;
use crate::components::{Ball, Position, Velocity};

enum Scorer {
    AI,
    Player
}

#[derive(Event)]
pub struct Scored(Scorer);

#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

#[derive(Component)]
pub struct PlayerScoreboard;

#[derive(Component)]
pub struct AIScoreboard;

pub fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.get_single_mut() {
            // Here we write the events using our EventWriter
            if ball.0.x > window_width / 2. {
                events.send(Scored(Scorer::AI));
            } else if ball.0.x < -window_width / 2. {
                events.send(Scored(Scorer::Player));
            }
        }
    }
}

pub fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((
            mut position,
            mut velocity
        )) = ball.get_single_mut() {
            match event.0 {
                Scorer::AI => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(-1., 1.);
                }
                Scorer::Player => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(1., 1.);
                }
            }
        }
    }
}

pub fn update_score(
    mut score: ResMut<Score>,
    mut events: EventReader<Scored>
) {
    for event in events.read() {
        match event.0 {
            Scorer::AI => score.ai += 1,
            Scorer::Player => score.player += 1,
        }
    }

    println!("Score: {} - {}", score.player, score.ai);
}

pub fn update_scoreboard(
    mut player_score: Query<&mut Text, With<PlayerScoreboard>>,
    mut ai_score: Query<&mut Text, (With<AIScoreboard>, Without<PlayerScoreboard>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.sections[0].value = score.player.to_string();
        }

        if let Ok(mut ai_score) = ai_score.get_single_mut() {
            ai_score.sections[0].value = score.ai.to_string();
        }
    }
}