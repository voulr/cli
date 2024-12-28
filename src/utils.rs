use strum::{Display, EnumIter};

pub const DEFAULT_LOCATION: &str = "./";

#[derive(EnumIter, Display)]
pub enum Language {
    Rust,
}

#[derive(EnumIter, Display)]
pub enum Framework {
    Axum,
}

impl Language {
    pub fn frameworks(&self) -> Vec<Framework> {
        match self {
            Language::Rust => vec![Framework::Axum],
        }
    }
}
