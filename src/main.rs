fn main() {
    let sample_coffee = Coffee {
        name: String::from("Ethiopia CHIRE"),
        roaster: String::from("Fiftybeans"),
        weight_g: 200,
        price_czk: 380.0,
    };

    let price_100g = sample_coffee.price_per_100g();

    println!("Káva: {} od {}", sample_coffee.name, sample_coffee.roaster);
    println!(
        "Cena za balení ({}g): {} Kč",
        sample_coffee.weight_g, sample_coffee.price_czk
    );
    println!("Přepočtená cena za 100g: {:.2} Kč", price_100g);
    println!("{:#?}", sample_coffee);
}

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
