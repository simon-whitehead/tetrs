
use super::{Score, ScoreMetaData, ScoringSystem};

pub struct DefaultScoringSystem;

impl ScoringSystem for DefaultScoringSystem {
    fn update_score(&self, score: &mut Score, metadata: ScoreMetaData) {
        // 100 times the amount of lines that were cleared
        score.add(metadata.lines_cleared * 100);
    }
}