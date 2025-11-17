use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::levels::LevelData;

pub struct LevelLoader;
impl LevelLoader {
    pub fn load_all() -> Vec<LevelData> {
        let mut all = Vec::new();

        let mut base = Self::load_base_levels();
        let mut bonus = Self::load_bonus_levels();
        let mut community = Self::load_community_levels();

        all.append(&mut base);
        all.append(&mut bonus);
        all.append(&mut community);

        all.sort_by_key(|lvl| lvl.id);

        all
    }

    pub fn load_base_levels() -> Vec<LevelData> {
        Self::load_jsonl_file("data/base-levels.jsonl")
    }

    pub fn load_bonus_levels() -> Vec<LevelData> {
        Self::load_jsonl_file("data/bonus-levels.jsonl")
    }

    pub fn load_community_levels() -> Vec<LevelData> {
        Self::load_jsonl_file("data/community-levels.jsonl")
    }

    fn load_jsonl_file(path: &str) -> Vec<LevelData> {
        let file = File::open(path).expect(&format!("Failed to open {}", path));

        let reader = BufReader::new(file);

        let mut result = Vec::new();

        for line in reader.lines() {
            if let Ok(text) = line {
                if !text.trim().is_empty() {
                    let parsed: LevelData =
                        serde_json::from_str(&text).expect("Invalid JSONL record");

                    result.push(parsed);
                }
            }
        }

        result.sort_by_key(|lvl| lvl.id);

        result
    }
}
