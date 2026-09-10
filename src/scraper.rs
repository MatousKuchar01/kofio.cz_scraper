// src/scraper.rs

use reqwest::Error;

pub fn fetch_kofio_html() -> Result<String, Error> {
    let url = "https://www.kofio.cz/vyberova-kava/filtr";
    let response = reqwest::blocking::get(url)?;
    let html_content = response.text()?;

    Ok(html_content)
}
