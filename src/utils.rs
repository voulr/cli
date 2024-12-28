use strum::{Display, EnumIter};

pub const DEFAULT_LOCATION: &str = "./";

#[derive(EnumIter, Display)]
pub enum Language {
    Rust,
}
