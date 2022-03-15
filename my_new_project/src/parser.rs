use crate::grocery_inventory::GroceryInventory;
use crate::grocery_item::GroceryItem;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

//use grocery_inventory::GroceryInventory;
//use grocery_item::GroceryItem;
use lenient_bool::LenientBool;

pub struct Parser {}

impl Parser {
    pub fn read(&self, path: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(path).expect("no such file");
        let buffer = BufReader::new(file);

        buffer
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }
    pub fn produce_inventory(&self, contents: Vec<String>) -> GroceryInventory {
        let mut inventory = GroceryInventory {
            inventory: Vec::new(),
            tax_rate: 0.075,
        };

        for line in contents {
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
        inventory
    }
}
