use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LevelSource {
    BaseLevels,
    BonusLevels,
    CommunityLevels,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub id: u32,
    pub size: u32,
    pub regions: Vec<Vec<u32>>,
    pub source: LevelSource,
}
