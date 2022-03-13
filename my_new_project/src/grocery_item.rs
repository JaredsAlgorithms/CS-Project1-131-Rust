use std::iter::{self, Sum};

pub struct GroceryItem {
    pub name: String,
    pub quantity: u32,
    pub unit_price: f32,
    pub is_taxable: bool,
}

