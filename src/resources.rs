use bevy::prelude::*;

/// Score tracking resource
#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

/// Level tracking resource
#[derive(Resource)]
pub struct Level {
    pub current: u32,
}

impl Default for Level {
    fn default() -> Self {
        Self { current: 1 }
    }
}

impl Level {
    /// Get the ball speed multiplier based on current level
    pub fn speed_multiplier(&self) -> f32 {
        1.0 + (self.current - 1) as f32 * crate::constants::SPEED_INCREASE_PER_LEVEL
    }
}

/// Font path constant
pub const GAME_FONT_PATH: &str = "fonts/DotGothic16-Regular.ttf";

/// Screen shake resource
#[derive(Resource, Default)]
pub struct ScreenShake {
    pub trauma: f32,
}

/// Combo tracking resource
#[derive(Resource)]
pub struct ComboTracker {
    pub count: u32,
    pub timer: Timer,
    pub last_score_gained: u32,
}

impl Default for ComboTracker {
    fn default() -> Self {
        Self {
            count: 0,
            timer: Timer::from_seconds(crate::constants::COMBO_WINDOW, TimerMode::Once),
            last_score_gained: 0,
        }
    }
}

/// Top-3 high score ranking (persisted across sessions)
#[derive(Resource)]
pub struct HighScores {
    pub scores: [u32; 3], // Descending order
}

impl HighScores {
    /// Load scores from persistent storage
    pub fn load() -> Self {
        Self {
            scores: crate::storage::load_scores(),
        }
    }

    /// Save scores to persistent storage
    pub fn save(&self) {
        crate::storage::save_scores(&self.scores);
    }

    /// Best (1st place) score
    pub fn best(&self) -> u32 {
        self.scores[0]
    }

    /// Try to insert a score into the ranking.
    /// Returns the rank index (0, 1, or 2) if the score qualifies, None otherwise.
    /// Automatically saves after insertion.
    pub fn try_insert(&mut self, score: u32) -> Option<usize> {
        if score == 0 {
            return None;
        }
        for i in 0..3 {
            if score > self.scores[i] {
                // Shift lower scores down
                for j in (i + 1..3).rev() {
                    self.scores[j] = self.scores[j - 1];
                }
                self.scores[i] = score;
                self.save();
                return Some(i);
            }
        }
        None
    }
}

impl Default for HighScores {
    fn default() -> Self {
        Self { scores: [0, 0, 0] }
    }
}

/// Per-level statistics resource
#[derive(Resource, Default)]
pub struct LevelStats {
    pub blocks_destroyed: u32,
    pub max_combo: u32,
    pub score_at_level_start: u32,
    pub time_elapsed: f32,
}

/// Sound handles resource
#[derive(Resource, Default)]
pub struct GameSounds {
    pub bounce: Option<Handle<AudioSource>>,
    pub break_block: Option<Handle<AudioSource>>,
    pub game_over: Option<Handle<AudioSource>>,
    pub level_up: Option<Handle<AudioSource>>,
    pub bgm: Option<Handle<AudioSource>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_default_zero() {
        let score = Score::default();
        assert_eq!(score.value, 0);
    }

    #[test]
    fn level_default_one() {
        let level = Level::default();
        assert_eq!(level.current, 1);
    }

    #[test]
    fn speed_multiplier_level_1() {
        let level = Level { current: 1 };
        assert!((level.speed_multiplier() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn speed_multiplier_level_2() {
        let level = Level { current: 2 };
        assert!((level.speed_multiplier() - 1.1).abs() < 0.001);
    }

    #[test]
    fn speed_multiplier_level_10() {
        let level = Level { current: 10 };
        assert!((level.speed_multiplier() - 1.9).abs() < 0.001);
    }

    #[test]
    fn high_scores_default_zero() {
        let hs = HighScores::default();
        assert_eq!(hs.scores, [0, 0, 0]);
        assert_eq!(hs.best(), 0);
    }

    #[test]
    fn high_scores_try_insert() {
        let mut hs = HighScores::default();
        assert_eq!(hs.try_insert(100), Some(0));
        assert_eq!(hs.scores, [100, 0, 0]);
        assert_eq!(hs.try_insert(50), Some(1));
        assert_eq!(hs.scores, [100, 50, 0]);
        assert_eq!(hs.try_insert(200), Some(0));
        assert_eq!(hs.scores, [200, 100, 50]);
    }

    #[test]
    fn high_scores_try_insert_no_rank() {
        let mut hs = HighScores { scores: [300, 200, 100] };
        assert_eq!(hs.try_insert(50), None);
        assert_eq!(hs.scores, [300, 200, 100]);
    }

    #[test]
    fn high_scores_try_insert_zero_ignored() {
        let mut hs = HighScores::default();
        assert_eq!(hs.try_insert(0), None);
        assert_eq!(hs.scores, [0, 0, 0]);
    }

    #[test]
    fn level_stats_default_zero() {
        let stats = LevelStats::default();
        assert_eq!(stats.blocks_destroyed, 0);
        assert_eq!(stats.max_combo, 0);
        assert_eq!(stats.score_at_level_start, 0);
        assert!((stats.time_elapsed - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn speed_multiplier_monotonic() {
        for i in 1..20 {
            let a = Level { current: i };
            let b = Level { current: i + 1 };
            assert!(b.speed_multiplier() > a.speed_multiplier());
        }
    }
}
