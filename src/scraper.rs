use crate::Coffee;
use reqwest::Error;
use scraper::{Html, Selector};

pub fn fetch_coffees() -> Result<Vec<Coffee>, Error> {
    let url = "https://www.kofio.cz/kava/filtr";
    println!("Stahuji: {}\n", url);

    let response = reqwest::blocking::get(url)?;
    let html_content = response.text()?;

    let coffees = parse_coffees(&html_content);
    Ok(coffees)
}

fn parse_coffees(html: &str) -> Vec<Coffee> {
    let document = Html::parse_document(html);

    let item_selector = Selector::parse("div.category_item").unwrap();
    let link_selector = Selector::parse("h3 a").unwrap();
    let price_selector = Selector::parse("div.price").unwrap();
    let small_selector = Selector::parse("small").unwrap();
    let merchant_selector = Selector::parse(".category_item_merchant").unwrap();

    let mut coffees = Vec::new();

    for item in document.select(&item_selector) {
        let link_element = match item.select(&link_selector).next() {
            Some(el) => el,
            None => continue,
        };

        let link_text = link_element.text().collect::<String>();
        let title_attr = link_element.value().attr("title").unwrap_or("").trim();

        if link_text.trim().is_empty() && title_attr.is_empty() {
            continue;
        }

        // 1. PRAŽÍRNA A NÁZEV
        let mut roaster = String::new();
        let mut name = link_text.trim().to_string();

        // A) Zkusíme najít pražírnu přímo v HTML prvku .category_item_merchant
        if let Some(merchant_el) = item.select(&merchant_selector).next() {
            roaster = merchant_el.text().collect::<String>().trim().to_string();
        }

        // B) Pokud prvek v HTML nebyl, vytáhneme pražírnu z title="Název - Gramáž - Pražírna"
        if (roaster.is_empty() || roaster == "Neznámá pražírna") && title_attr.contains('-') {
            let parts: Vec<&str> = title_attr.split('-').map(|s| s.trim()).collect();
            if parts.len() >= 2 {
                // Pražírna bývá na Kofiu vždy úplně na konci title atributu
                roaster = parts[parts.len() - 1].to_string();

                // Pokud název obsahoval i pražírnu, očistíme ho
                if name.ends_with(&roaster) {
                    name = name
                        .trim_end_matches(&roaster)
                        .trim()
                        .trim_end_matches('-')
                        .trim()
                        .to_string();
                }
            }
        }

        if roaster.is_empty() {
            roaster = "Neznámá pražírna".to_string();
        }

        // 2. GRAMÁŽ
        let mut weight_g: u32 = 0;

        // A) Zkusíme <small> tag uvnitř ceny (např. "/ 200g")
        if let Some(small_el) = item.select(&small_selector).next() {
            let small_text = small_el.text().collect::<String>();
            weight_g = extract_weight(&small_text);
        }

        // B) Zkusíme z title atributu
        if weight_g == 0 {
            weight_g = extract_weight(title_attr);
        }

        // C) Zkusíme přímo z názvu kávy
        if weight_g == 0 {
            weight_g = extract_weight(&name);
        }

        // D) Pokud gramáž stále nemáme, ale máme název i cenu, fallback na standardních 250g
        if weight_g == 0 {
            weight_g = 250;
        }

        // 3. CENA
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

        if price_czk > 0.0 {
            coffees.push(Coffee {
                name,
                roaster,
                weight_g,
                price_czk,
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
