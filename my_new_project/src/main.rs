mod grocery_inventory;
mod grocery_item;

use grocery_inventory::GroceryInventory;
use grocery_item::GroceryItem;
use lenient_bool::LenientBool;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    // Read in a file from a given path and give a vector of strings

    let file = File::open(filename).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {

    let mut inventory = GroceryInventory {
        inventory: Vec::new(),
        tax_rate: 0.075,
    };
    let lines = lines_from_file("../inputs/shipment.txt");

    for line in lines {
        let mut split = line.split_whitespace();
        let tuple = (
            split.next().unwrap().to_string(),
            split.next().unwrap().parse::<u32>().unwrap(),
            split.next().unwrap().parse::<f32>().unwrap(),
            split.next().unwrap().parse::<LenientBool>().unwrap().into(),
        );
        let (name, quantity, unit_price, is_taxable) = tuple;
        let _another = GroceryItem {
            name,
            quantity,
            unit_price,
            is_taxable,
        };
        inventory.add_item(_another);
    }

    assert_eq!(inventory.size(), 166);
    assert_eq!(inventory.tax_revenue(), 11176.954);
    assert_eq!(inventory.unit_revenue(), 1835852.4);

    assert_eq!(inventory.total_revenue(), 1847029.4);

}
