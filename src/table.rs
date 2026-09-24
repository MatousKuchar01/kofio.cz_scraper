use crate::Coffee;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

/// Vytiskne přehlednou tabulku káv do standardního výstupu (STDOUT).
///
/// Funkce využívá formátování sady znaků UTF-8 (`UTF8_FULL`) pro vykreslení tabulky.
/// Automaticky počítá pořadí (rank) jednotlivých položek od jedničky.
///
/// # Vizuální formátování
///
/// * **Zarovnání:** Sloupce pro balení (hmotnost), celkovou cenu a cenu za 100g jsou zarovnány **doprava** pro lepší čitelnost číselných hodnot.
/// * **Zvýraznění TOP 3:** První **tři položky** (indexy 0, 1, 2) jsou v tabulce zvýrazněny **zelenou barvou a tučným písmem**, což je ideální pro zobrazení nejvýhodnějších nebo nejlépe hodnocených káv.
/// * **Výpočet ceny:** Pro sloupec "Cena / 100g" funkce interně volá metodu `.price_per_100g()` na struktuře `Coffee`.
///
/// # Arguments
///
/// * `coffees` - Slice (pohled na pole) struktur `Coffee`, které se mají v tabulce zobrazit. Pokud je předán prázdný slice, vytiskne se pouze hlavička tabulky.
pub fn print_coffee_table(coffees: &[Coffee]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    table.set_header(vec![
        Cell::new("#").add_attribute(Attribute::Bold),
        Cell::new("Název").add_attribute(Attribute::Bold),
        Cell::new("Pražírna").add_attribute(Attribute::Bold),
        Cell::new("Balení").add_attribute(Attribute::Bold),
        Cell::new("Cena").add_attribute(Attribute::Bold),
        Cell::new("Cena / 100g").add_attribute(Attribute::Bold),
        Cell::new("Skladem").add_attribute(Attribute::Bold),
    ]);

    if let Some(column) = table.column_mut(3) {
        column.set_cell_alignment(CellAlignment::Right)
    }

    if let Some(column) = table.column_mut(4) {
        column.set_cell_alignment(CellAlignment::Right)
    }

    if let Some(column) = table.column_mut(5) {
        column.set_cell_alignment(CellAlignment::Right)
    }

    for (index, coffee) in coffees.iter().enumerate() {
        let rank = index + 1;
        let is_top3 = rank <= 3;

        let mut row = vec![
            Cell::new(rank.to_string()),
            Cell::new(&coffee.name),
            Cell::new(&coffee.roaster),
            Cell::new(format!("{}g", coffee.weight_g)),
            Cell::new(format!("{:.0} Kč", coffee.price_czk)),
            Cell::new(format!("{:.2} Kč", coffee.price_per_100g())),
            Cell::new(&coffee.stock),
        ];

        if is_top3 {
            row = row
                .into_iter()
                .map(|cell| cell.fg(Color::Green).add_attribute(Attribute::Bold))
                .collect();
        }

        table.add_row(row);
    }

    println!("{table}");
}
