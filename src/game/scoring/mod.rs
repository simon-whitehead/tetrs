mod default;
mod score;
mod scoring_system;

pub use game::scoring::score::Score;
pub use game::scoring::scoring_system::{ScoreMetaData, ScoringSystem};
pub use game::scoring::default::DefaultScoringSystem;