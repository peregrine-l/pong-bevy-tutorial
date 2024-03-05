use bevy::prelude::*;
use crate::components::Position;

pub fn project_positions(
    mut ball: Query<(&mut Transform, &Position)>
) {
    for (mut transform, position) in &mut ball {
        transform.translation = position.0.extend(0.);
    }
}