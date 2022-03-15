use crate::GroceryItem;

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

        let mut summation: f32 = 0.0;
        for element in &self.inventory {
            let value = element.unit_price * element.quantity as f32;
            summation += value;
        }
        summation
    }

    pub fn tax_revenue(&self) -> f32 {
        /*
         * Calculate only the revenue earned from taxation
        */

        let mut summation: f32 = 0.0;
        for element in &self.inventory {
            let value = element.unit_price * element.quantity as f32;
            let _t = if element.is_taxable { self.tax_rate } else {0 as f32};
            summation += value * _t;
        }
        summation

    }
    //pub fn total_revenue(&mut self) -> &f32 {
        //&mut self.tax_revenue() + &mut self.unit_revenue()
    //}
}
