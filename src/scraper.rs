// src/scraper.rs

use crate::Coffee;

use reqwest::Error;
use scraper::{Html, Selector};

pub fn fetch_kofio_html() -> Result<String, Error> {
    let url = "https://www.kofio.cz/kava/filtr";

    let response = reqwest::blocking::get(url)?;
    let html_content = response.text()?;

    Ok(html_content)
}

pub fn parse_coffees(html: &str) -> Vec<Coffee> {
    let document = Html::parse_document(html);

    let item_selector = Selector::parse("div.category_item").unwrap();
    let link_selector = Selector::parse("h3 a").unwrap();
    let price_selector = Selector::parse("div.price").unwrap();

    let mut coffees = Vec::new();

    for item in document.select(&item_selector) {
        let title_attr = match item.select(&link_selector).next() {
            Some(link) => link.value().attr("title").unwrap_or(""),
            None => continue,
        };

        if title_attr.is_empty() {
            continue;
        }

        let parts: Vec<&str> = title_attr.split(" - ").collect();

        if parts.len() < 3 {
            continue;
        }

        //todo
    }

    coffees
}
