mod scraper;

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
        Ok(coffees) => {
            for coffee in coffees.iter() {
                println!(
                    "-> {} | Pražírna: {}\n   {}g za {} Kč | Cena/100g: {:.2} Kč\n   Sklad: {} | Chuti: {}\n",
                    coffee.name,
                    coffee.roaster,
                    coffee.weight_g,
                    coffee.price_czk,
                    coffee.price_per_100g(),
                    coffee.stock,
                    coffee.flavors
                );
            }
        }
        Err(err) => {
            println!("Error fetching data: {}", err);
        }
    }
}
