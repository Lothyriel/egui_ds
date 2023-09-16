use std::str::FromStr;

use scraper::{node::Text, Node};

use crate::data::model::{Damage, Defense, OffensiveStats, Scaling};

use super::{ScrapedEntry, ScrapingError};

pub fn scrape_data() -> Result<Vec<ScrapedEntry>, ScrapingError> {
    let document = get_initial_stats_html()?;

    let table_selector = scraper::Selector::parse("#wiki-tab-0-0 > table > tbody")?;

    let initial_stats_table = document
        .select(&table_selector)
        .nth(2)
        .ok_or_else(|| ScrapingError::HtmlTagNotFound("Third table"))?;

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

fn parse_weapon_stats(e: Vec<Node>) -> Result<ScrapedEntry, ScrapingError> {
    let weapon = ScrapedEntry {
        name: get_text(0, &e)?.text.to_owned().into(),
        stats: OffensiveStats {
            dmg: Damage {
                physical: parse(1, &e)?,
                magic: parse(2, &e)?,
                fire: parse(3, &e)?,
                lightning: parse(4, &e)?,
            },
            scaling: Scaling {
                str: parse(5, &e)?,
                dex: parse(6, &e)?,
                int: parse(7, &e)?,
                faith: parse(8, &e)?,
            },
        },
        def: Defense {
            physical: parse(9, &e)?,
            magic: parse(10, &e)?,
            fire: parse(11, &e)?,
            light: parse(12, &e)?,
        },
        stability: parse(13, &e)?,
    };

    Ok(weapon)
}

fn parse<T: FromStr>(i: usize, e: &[Node]) -> Result<T, ScrapingError> {
    get_text(i, e).and_then(|t| {
        t.parse::<T>()
            .map_err(|_| ScrapingError::TextParse(i, std::any::type_name::<T>()))
    })
}

fn get_text(i: usize, e: &[Node]) -> Result<&Text, ScrapingError> {
    e.get(i)
        .and_then(|n| n.as_text())
        .ok_or_else(|| ScrapingError::TextExtract(i))
}
