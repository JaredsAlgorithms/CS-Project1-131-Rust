use std::iter::{self, Sum};

pub struct GroceryItem {
    pub name: String,
    pub quantity: u32,
    pub unit_price: f32,
    pub is_taxable: bool,
}

impl Iterator for GroceryItem {
    type Item = f32;

    fn sum<S>(self) -> S
    where
        S: Sum<Self::Item>,
    {
        S::sum(iter::once(self.unit_price * self.quantity))
    }
}
