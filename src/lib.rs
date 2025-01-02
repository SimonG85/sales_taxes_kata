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
    fn get_price(&self) -> f64;
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
    fn get_price(&self) -> f64 {
        match (&self.category, &self.imported) {
            (Category::Book | Category::Food | Category::Medical, Imported::No) => self.clean_price,
            (Category::Other(_), Imported::No) => {
                self.clean_price + round_numbers(self.clean_price * 0.10)
            }
            (Category::Book | Category::Food | Category::Medical, Imported::Yes) => {
                self.clean_price + round_numbers(self.clean_price * (0.05))
            }
            (Category::Other(_), Imported::Yes) => {
                self.clean_price + round_numbers(self.clean_price * (0.10 + 0.05))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_book() {
        let book = Item::new(12.49, Imported::No, Category::Book).unwrap();
        let computed = book.get_price();
        let expected = 12.49;
        assert_relative_eq!(computed, expected, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_music_cd() {
        let music_cd = Item::new(14.99, Imported::No, Category::Other("CD".to_string())).unwrap();
        let computed = music_cd.get_price();
        let expected = 16.49;
        assert_relative_eq!(computed, expected, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_imported_box_chocolates() {
        let box_chocolates = Item::new(10.00, Imported::Yes, Category::Food).unwrap();
        let computed = box_chocolates.get_price();
        let expected = 10.50;
        assert_relative_eq!(computed, expected, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_imported_perfume() {
        let imported_perfume =
            Item::new(47.50, Imported::Yes, Category::Other("Perfume".to_string())).unwrap();
        let computed = imported_perfume.get_price();
        let expected = 54.65;
        assert_relative_eq!(computed, expected, epsilon = f64::EPSILON);
    }
}
