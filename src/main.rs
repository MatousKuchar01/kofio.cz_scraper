mod scraper;

/*#[derive(Debug)]
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
}*/

fn main() {
    match scraper::fetch_kofio_html() {
        Ok(html) => {
            println!("Success! Length of HTML is: {}", html.len());
        }
        Err(err) => {
            println!("Error fetching data: {}", err);
        }
    }
}
