use std::str::FromStr;

use sales_taxes_kata::{Basket, Item};

fn main() {
    let input_1 = "1 imported bottle of perfume at 27.99
1 bottle of perfume at 18.99
1 packet of headache pills at 9.75
1 box of imported chocolates at 11.25";
    let basket_1 = Basket::<Item>::from_str(input_1).unwrap();
    // println!("{:?}", basket_1);
    println!("{}", basket_1.to_string());
}
