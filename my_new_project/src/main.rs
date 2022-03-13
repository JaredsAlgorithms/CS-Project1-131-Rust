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
    let file = File::open(filename).expect("no such file");
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let _foo = GroceryItem {
        name: "Hello".to_string(),
        quantity: 100,
        unit_price: 10.00,
        is_taxable: false,
    };

    let mut _invent = GroceryInventory {
        inventory: Vec::new(),
        tax_rate: 0.075,
    };
    let lines = lines_from_file("inputs/shipment.txt");
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
        _invent.add_item(_another);
    }
    println!("The size of the inventory is {}", _invent.size());
}
