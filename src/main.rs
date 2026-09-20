mod scraper;

#[derive(Debug)]
struct Coffee {
    name: String,
    roaster: String,
    weight_g: u32,
    price_czk: f64,
}

impl Coffee {
    fn price_per_100g(&self) -> f64 {
        (self.price_czk / self.weight_g as f64) * 100.0
    }
}

fn main() {
    match scraper::fetch_coffees() {
        Ok(coffees) => {
            for coffee in coffees.iter() {
                println!(
                    "-> {} ({}) | {}g za {} Kč | Cena/100g: {:.2} Kč",
                    coffee.name,
                    coffee.roaster,
                    coffee.weight_g,
                    coffee.price_czk,
                    coffee.price_per_100g()
                );
            }
        }
        Err(err) => {
            println!("Error fetching data: {}", err);
        }
    }
}
