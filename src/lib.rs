/// This lib allows to compute price after tax of an item
/// and a basket of items.

enum Imported {
    Yes,
    No,
}

enum Category {
    Book,
    Food,
    Medical,
    Other(String),
}

pub trait Tax {
    fn get_prices(&self) -> (f64, f64);
}

struct Item {
    clean_price: f64,
    imported: Imported,
    category: Category,
}

impl Item {
    fn new(clean_price: f64, imported: Imported, category: Category) -> Result<Self, &'static str> {
        if clean_price < 0.0 {
            return Err("clean_price must be positive");
        }
        Ok(Self {
            clean_price,
            imported,
            category,
        })
    }
}

fn round_numbers(number: f64) -> f64 {
    (number * 20.0).round() / 20.0
}

impl Tax for Item {
    fn get_prices(&self) -> (f64, f64) {
        //     match (&self.category, &self.imported) {
        //         (Category::Book | Category::Food | Category::Medical, Imported::No) => self.clean_price,
        //         (Category::Other(_), Imported::No) => {
        //             self.clean_price + round_numbers(self.clean_price * 0.10)
        //         }
        //         (Category::Book | Category::Food | Category::Medical, Imported::Yes) => {
        //             self.clean_price + round_numbers(self.clean_price * (0.05))
        //         }
        //         (Category::Other(_), Imported::Yes) => {
        //             self.clean_price + round_numbers(self.clean_price * (0.10 + 0.05))
        //         }
        //     }
        // }
        (1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_book() {
        let book = Item::new(12.49, Imported::No, Category::Book).unwrap();
        let (clean_price, tax) = book.get_prices();
        let expected = (12.49, 0.0);
        assert_relative_eq!(clean_price, expected.0, epsilon = f64::EPSILON);
        assert_relative_eq!(tax, expected.1, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_music_cd() {
        let music_cd = Item::new(14.99, Imported::No, Category::Other("CD".to_string())).unwrap();
        let (clean_price, tax) = music_cd.get_prices();
        let expected = (12.49, 1.5);
        assert_relative_eq!(clean_price, expected.0, epsilon = f64::EPSILON);
        assert_relative_eq!(tax, expected.1, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_imported_box_chocolates() {
        let box_chocolates = Item::new(10.00, Imported::Yes, Category::Food).unwrap();
        let (clean_price, tax) = box_chocolates.get_prices();
        let expected = (10.0, 0.50);
        assert_relative_eq!(clean_price, expected.0, epsilon = f64::EPSILON);
        assert_relative_eq!(tax, expected.1, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_imported_perfume() {
        let imported_perfume =
            Item::new(47.50, Imported::Yes, Category::Other("Perfume".to_string())).unwrap();
        let (clean_price, tax) = imported_perfume.get_prices();
        let expected = (47.50, 7.15);
        assert_relative_eq!(clean_price, expected.0, epsilon = f64::EPSILON);
        assert_relative_eq!(tax, expected.1, epsilon = f64::EPSILON);
    }
}

// #[cfg(test)]
// mod acceptance_tests {
//     use super::*;
//     use approx::assert_relative_eq;
//     #[test]
//     fn test_purchase_1() {
//         let book = Item::new(12.49, Imported::No, Category::Book).unwrap();
//                 let music_cd = Item::new(14.99, Imported::No, Category::Other("CD".to_string())).unwrap();
//         let bar_chocolates = Item::new(1.50, Imported::No, Category::Food).unwrap();
// let taxes = book.get_price() + music_cd.get_price() + bar_chocolates.get_price();

//     }
// }
