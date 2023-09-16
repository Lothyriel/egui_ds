use std::str::FromStr;

use scraper::{node::Text, Node};

use super::{ScrapingError, WeaponStat};

pub fn scrape_data() -> Result<Vec<WeaponStat>, ScrapingError> {
    let document = get_initial_stats_html()?;

    let table_selector = scraper::Selector::parse("#wiki-tab-0-0 > table > tbody")?;

    let initial_stats_table = document
        .select(&table_selector)
        .nth(2)
        .ok_or_else(|| ScrapingError::HtmlTableNotFound)?;

    let tr_tags = initial_stats_table.children().skip(1);

    let th_tags = tr_tags.filter(|n| n.has_children());

    let non_empty_th_tags = th_tags.map(|n| {
        n.children()
            .filter(|nr| matches!(nr.value(), Node::Element(_)))
            .flat_map(|nr| nr.first_child())
            .map(|nr| nr.value())
    });

    non_empty_th_tags
        .map(|c| parse_weapon_stats(c.cloned().collect()))
        .collect()
}

fn get_initial_stats_html() -> Result<scraper::Html, ScrapingError> {
    let request = ehttp::Request::get("http://darksouls.wikidot.com/reinforcement-formulas");

    let response = ehttp::fetch_blocking(&request).map_err(ScrapingError::Fetch)?;

    let text = response.text().ok_or(ScrapingError::HttpContent)?;

    let document = scraper::Html::parse_document(text);

    Ok(document)
}

fn parse_weapon_stats(e: Vec<Node>) -> Result<WeaponStat, ScrapingError> {
    let weapon = WeaponStat {
        name: get_text(0, &e)?.text.to_owned().into(),
        physical_ar: parse(1, &e)?,
        magic_ar: parse(2, &e)?,
        fire_ar: parse(3, &e)?,
        lightning_ar: parse(4, &e)?,
        str_scaling: parse(5, &e)?,
        dex_scaling: parse(6, &e)?,
        int_scaling: parse(7, &e)?,
        faith_scaling: parse(8, &e)?,
        physical_def: parse(9, &e)?,
        magic_def: parse(10, &e)?,
        fire_def: parse(11, &e)?,
        light_def: parse(12, &e)?,
        stability: parse(13, &e)?,
    };

    Ok(weapon)
}

fn parse<T: FromStr>(i: usize, e: &[Node]) -> Result<T, ScrapingError> {
    get_text(i, e).and_then(|t| {
        t.parse::<T>()
            .map_err(|_| ScrapingError::TextParse(i, std::any::type_name::<T>().to_owned()))
    })
}

fn get_text(i: usize, e: &[Node]) -> Result<&Text, ScrapingError> {
    e.get(i)
        .and_then(|n| n.as_text())
        .ok_or_else(|| ScrapingError::TextExtract(i))
}
