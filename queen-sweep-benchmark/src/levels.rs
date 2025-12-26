use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
    path::Path,
};

use serde::Deserialize;

const BASE_LEVEL_JSONL: &'static str = "data/base-levels.jsonl";
const BONUS_LEVEL_JSONL: &'static str = "data/bonus-levels.jsonl";
const COMMUNITY_LEVEL_JSONL: &'static str = "data/community-levels.jsonl";

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub id: u32,
    pub size: u32,
    pub regions: Vec<Vec<u8>>,
    pub source: LevelSource,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LevelSource {
    BaseLevels,
    BonusLevels,
    CommunityLevels,
}

#[inline]
pub fn load_base_levels() -> Vec<LevelData> {
    load_level_data(BASE_LEVEL_JSONL)
}

#[inline]
pub fn load_bonus_levels() -> Vec<LevelData> {
    load_level_data(BONUS_LEVEL_JSONL)
}

#[inline]
pub fn load_community_levels() -> Vec<LevelData> {
    load_level_data(COMMUNITY_LEVEL_JSONL)
}

fn load_level_data<P: AsRef<Path>>(path: P) -> Vec<LevelData> {
    let file = load_file(path);

    let reader = BufReader::new(file);

    let mut result: Vec<LevelData> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| match serde_json::from_str::<LevelData>(&line) {
            Ok(parsed) => Some(parsed),
            Err(err) => {
                eprintln!("Skipping invalid JSONL record: {err}");
                None
            }
        })
        .collect();

    result.sort_by_key(|lvl| lvl.id);
    result
}

fn load_file<P: AsRef<Path>>(path: P) -> File {
    let path = path.as_ref();
    match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Level file not found: {}", path.display());
                }
                _ => {
                    eprintln!("Failed to open level file {}: {}", path.display(), err);
                }
            }

            std::process::exit(1);
        }
    }
}
