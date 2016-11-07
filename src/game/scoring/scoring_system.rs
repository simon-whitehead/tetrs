use super::Score;

pub trait ScoringSystem {
    fn update_score(&self, score: &mut Score, metadata: ScoreMetaData);
}

pub struct ScoreMetaData {
    pub lines_cleared: u32,
}