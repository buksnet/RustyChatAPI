use rand::seq::IndexedRandom;
use rand::RngExt;

pub struct TagGenerator {
    adjectives: Vec<String>,
    nouns: Vec<String>,
}

impl TagGenerator {
    /// Генерирует ники для пользователей по формату
    /// прилагательное + существительное + 0 < число < 100
    pub fn new() -> Self {
        Self {
            adjectives: include_str!("../../assets/english-adjectives.txt")
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            nouns: include_str!("../../assets/english-nouns.txt")
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        }
    }

    pub fn get_nick(&self) -> String {
        let mut rng = rand::rng();
        let adj = self.adjectives.choose(&mut rng).unwrap();
        let noun = self.nouns.choose(&mut rng).unwrap();
        let num = rng.random_range(1..99);
        format!("{}_{}_{}", adj, noun, num)
    }

    pub fn clear(&mut self) {
        self.adjectives.clear();
        self.adjectives.shrink_to_fit();
        self.nouns.clear();
        self.nouns.shrink_to_fit();
    }
}
