use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

/// Update score text display
pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            *text = Text::new(format!("スコア {}", score.value));
        }
    }
}

/// Update level text display
pub fn update_level_text(level: Res<Level>, mut query: Query<&mut Text, With<LevelText>>) {
    if level.is_changed() {
        for mut text in &mut query {
            *text = Text::new(format!("レベル {}", level.current));
        }
    }
}

/// Update high score text display
pub fn update_high_score_text(
    high_scores: Res<HighScores>,
    mut query: Query<&mut Text, With<HighScoreText>>,
) {
    if high_scores.is_changed() {
        for mut text in &mut query {
            *text = Text::new(format!("ハイスコア {}", high_scores.best()));
        }
    }
}

/// Track level elapsed time
pub fn update_level_time(time: Res<Time>, mut level_stats: ResMut<LevelStats>) {
    level_stats.time_elapsed += time.delta_secs();
}
