mod scraper;
mod table;

#[derive(Debug)]
pub struct Coffee {
    pub name: String,
    pub roaster: String,
    pub weight_g: u32,
    pub price_czk: f64,
    pub stock: String,
    pub flavors: String,
}

impl Coffee {
    fn price_per_100g(&self) -> f64 {
        (self.price_czk / self.weight_g as f64) * 100.0
    }
}

fn main() {
    match scraper::fetch_coffees() {
        Ok(mut coffees) => {
            coffees.sort_by(|a, b| {
                a.price_per_100g()
                    .partial_cmp(&b.price_per_100g())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            table::print_coffee_table(&coffees);
        }
        Err(err) => {
            println!("Error fetching data: {}", err);
        }
    }
}
