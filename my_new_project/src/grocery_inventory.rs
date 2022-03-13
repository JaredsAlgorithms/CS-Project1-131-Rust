use crate::GroceryItem;

pub struct GroceryInventory {
    pub inventory: Vec<GroceryItem>,
    pub tax_rate: f32
}

impl GroceryInventory {
    pub fn add_item(&mut self, item: GroceryItem) {
        self.inventory.push(item);
    }
    pub fn size(&self) -> usize {
        self.inventory.len()
    }


}
