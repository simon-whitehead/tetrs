
use game::leveling::{LevelingSystem, LevelMetaData};

pub struct DefaultLevelingSystem;

impl LevelingSystem for DefaultLevelingSystem {
    fn process(&self, metadata: &mut LevelMetaData) {
        // Increase the level every 10 lines cleared
        if metadata.lines_cleared > 0 && metadata.total_lines_cleared > 0 &&
           metadata.total_lines_cleared % 10 == 0 {
            metadata.level.increase();
        }
    }
}