mod parser;
mod grocery_inventory;
mod grocery_item;

use parser::Parser;

fn main() {
    let my_parser = Parser {};
    let contents = my_parser.read("inputs/shipment.txt");
    let inventory = my_parser.produce_inventory(contents);

    assert_eq!(inventory.size(), 166);
    assert_eq!(inventory.unit_revenue(), 1835852.4);
    assert_eq!(inventory.tax_revenue(), 11176.954);

    assert_eq!(inventory.total_revenue(), 1847029.4);

    assert_eq!(inventory.get_element("Soda_can").quantity, 8793);
}
