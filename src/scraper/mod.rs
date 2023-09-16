mod web_scraper;

pub use web_scraper::scrape_data;

#[cfg(test)]
mod tests {
    use super::{web_scraper::scrape_data, ScrapingError};

    #[test]
    fn should_get_data() -> Result<(), ScrapingError> {
        let data = scrape_data()?;

        assert_eq!(data.len(), 655);

        Ok(())
    }
}

#[derive(serde::Serialize, Debug)]
pub struct WeaponStat {
    name: String,
    stability: u8,

    physical_ar: u16,
    magic_ar: u16,
    fire_ar: u16,
    lightning_ar: u16,

    str_scaling: u8,
    dex_scaling: u8,
    int_scaling: u8,
    faith_scaling: u8,

    physical_def: u8,
    magic_def: u8,
    fire_def: u8,
    light_def: u8,
}

#[derive(thiserror::Error, Debug)]
pub enum ScrapingError {
    #[error("{0}")]
    Fetch(String),
    #[error("Http request didn't return content")]
    HttpContent,
    #[error("{0}")]
    HtmlParse(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("Couldn't find the table tag")]
    HtmlTableNotFound,
    #[error("Couldn't get text from {0} nth element")]
    TextExtract(usize),
    #[error("Could not parse {0} nth to {1}")]
    TextParse(usize, String),
}
