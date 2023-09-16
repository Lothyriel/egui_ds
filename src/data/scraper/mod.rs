mod web_scraper;

pub use web_scraper::scrape_data;

use super::model::{Defense, OffensiveStats};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{web_scraper::scrape_data, MergingError, ScrapingError};

    #[test]
    fn should_get_data() -> Result<(), ScrapingError> {
        let data = scrape_data()?;

        assert_eq!(data.len(), 655);

        Ok(())
    }
}

#[derive(serde::Serialize, Debug)]
pub struct ScrapedEntry {
    pub name: String,
    pub stats: OffensiveStats,
    pub def: Defense,
    pub stability: u8,
}

#[derive(thiserror::Error, Debug)]
pub enum GetDataError {
    #[error("{0}")]
    ScrapingError(#[from] ScrapingError),
    #[error("{0}")]
    MergingError(#[from] MergingError),
}

#[derive(thiserror::Error, Debug)]
pub enum MergingError {
    #[error("Weapon name was empty")]
    WeaponNameEmpty,
}

#[derive(thiserror::Error, Debug)]
pub enum ScrapingError {
    #[error("Error during fetch: {0}")]
    Fetch(String),
    #[error("Http request didn't return text content")]
    HttpContent,
    #[error("{0}")]
    HtmlParsing(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("Couldn't find the {0} tag")]
    HtmlTagNotFound(&'static str),
    #[error("Couldn't get text from {0} nth element")]
    TextExtract(usize),
    #[error("Could not parse {0} nth to {1}")]
    TextParse(usize, &'static str),
}
