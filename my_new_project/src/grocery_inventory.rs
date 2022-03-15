use crate::grocery_item::GroceryItem;

pub struct GroceryInventory {
    pub inventory: Vec<GroceryItem>,
    pub tax_rate: f32,
}

impl GroceryInventory {
    pub fn add_item(&mut self, item: GroceryItem) {
        self.inventory.push(item);
    }
    pub fn size(&self) -> usize {
        self.inventory.len()
    }

    pub fn unit_revenue(&self) -> f32 {
        /*
         * Calculate the total revenue based on the quantity
         * and their unit price
         */

        self.inventory
            .iter()
            .map(|x| x.unit_price * x.quantity as f32)
            .sum()
    }

    pub fn tax_revenue(&self) -> f32 {
        /*
         * Calculate only the revenue earned from taxation
         */

        self.inventory
            .iter()
            .map(|x| {
                let value = x.unit_price * x.quantity as f32;
                let _t = if x.is_taxable {
                    self.tax_rate
                } else {
                    0 as f32
                };
                value * _t
            })
            .sum()
    }

    pub fn total_revenue(&self) -> f32 {
        self.unit_revenue() + self.tax_revenue()
    }

    pub fn get_element(&self, name: &str) -> &GroceryItem {
        self.inventory.iter().find(|&x| x.name == name).unwrap()
    }
}
