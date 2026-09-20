use crate::Coffee;
use reqwest::Error;
use scraper::{Html, Selector};

pub fn fetch_coffees() -> Result<Vec<Coffee>, Error> {
    let url = "https://www.kofio.cz/kava/filtr";
    println!("Stahuji data z Kofio.cz: {}\n", url);

    let response = reqwest::blocking::get(url)?;
    let html_content = response.text()?;

    let coffees = parse_coffees(&html_content);
    Ok(coffees)
}

fn parse_coffees(html: &str) -> Vec<Coffee> {
    let document = Html::parse_document(html);

    // Selektory z přiloženého HTML
    let item_selector = Selector::parse("div.category_item").unwrap();
    let name_selector = Selector::parse("div.category_item_footer h3 a").unwrap();
    let roaster_selector = Selector::parse("div.category_item_merchant a").unwrap();
    let price_selector = Selector::parse("div.price").unwrap();
    let small_selector = Selector::parse("div.price small").unwrap();
    let stock_selector = Selector::parse("div.stock_availability span").unwrap();
    let flavors_selector = Selector::parse("div.category_item_flavors").unwrap();

    let mut coffees = Vec::new();

    for item in document.select(&item_selector) {
        // 1. NÁZEV KÁVY (z <h3><a title="...">)
        let name = match item.select(&name_selector).next() {
            Some(el) => el.text().collect::<String>().trim().to_string(),
            None => continue, // Pokud káva nemá název, přeskočíme
        };

        if name.is_empty() {
            continue;
        }

        // 2. PRAŽÍRNA (z div.category_item_merchant a -> title atribut)
        let roaster = match item.select(&roaster_selector).next() {
            Some(el) => el
                .value()
                .attr("title")
                .unwrap_or("Neznámá pražírna")
                .trim()
                .to_string(),
            None => "Neznámá pražírna".to_string(),
        };

        // 3. GRAMÁŽ (z div.price small -> např. "/ 250g")
        let mut weight_g: u32 = 0;
        if let Some(small_el) = item.select(&small_selector).next() {
            let small_text = small_el.text().collect::<String>();
            weight_g = extract_weight(&small_text);
        }

        if weight_g == 0 {
            weight_g = extract_weight(&name);
        }
        if weight_g == 0 {
            weight_g = 250; // Fallback na 250g
        }

        // 4. CENA (z div.price bez podřazeného div.price_gram)
        let mut price_czk: f64 = 0.0;
        if let Some(price_el) = item.select(&price_selector).next() {
            let direct_text: String = price_el
                .children()
                .filter_map(|node| {
                    if node.value().is_element()
                        && node.value().as_element().unwrap().name() == "div"
                    {
                        None
                    } else {
                        node.value().as_text().map(|t| t.to_string())
                    }
                })
                .collect();

            price_czk = extract_price(&direct_text);
        }

        // 5. SKLAD (dostupnost)
        let stock = match item.select(&stock_selector).next() {
            Some(el) => el.text().collect::<String>().trim().to_string(),
            None => "Neuvedeno".to_string(),
        };

        // 6. CHUTĚ (chuťový profil)
        let flavors_raw = match item.select(&flavors_selector).next() {
            Some(el) => el.text().collect::<String>(),
            None => "".to_string(),
        };
        // Očistíme chuťové tóny od vícenásobných mezer a nových řádků
        let flavors = flavors_raw
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        if price_czk > 0.0 {
            coffees.push(Coffee {
                name,
                roaster,
                weight_g,
                price_czk,
                stock,
                flavors,
            });
        }
    }

    coffees
}

fn extract_weight(text: &str) -> u32 {
    let cleaned: String = text
        .chars()
        .map(|c| {
            if c.is_ascii_digit() || c == 'g' || c == 'G' {
                c
            } else {
                ' '
            }
        })
        .collect();

    for word in cleaned.split_whitespace() {
        if word.ends_with('g') || word.ends_with('G') {
            let num_str = &word[..word.len() - 1];
            if let Ok(num) = num_str.parse::<u32>() {
                if (50..=5000).contains(&num) {
                    return num;
                }
            }
        }
    }
    0
}

fn extract_price(text: &str) -> f64 {
    let text_before_kc = text.split("Kč").next().unwrap_or(text);

    let cleaned: String = text_before_kc
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ',' || *c == '.')
        .collect();

    let cleaned = cleaned.replace(',', ".");
    cleaned.parse::<f64>().unwrap_or(0.0)
}
