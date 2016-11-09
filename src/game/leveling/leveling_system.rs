use game::leveling::Level;

pub trait LevelingSystem {
    fn process(&self, metadata: &mut LevelMetaData);
}

pub struct LevelMetaData<'a> {
    pub level: &'a mut Level,
    pub total_lines_cleared: u32,
    pub lines_cleared: u32,
}