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

impl Tax for Item {
    fn get_price(&self) -> f64 {
        1.0
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
    fn test_chocolate_bar() {
        let chocolate_bar = Item::new(0.85, Imported::No, Category::Food).unwrap();
        let computed = chocolate_bar.get_price();
        let expected = 0.85;
        assert_relative_eq!(computed, expected, epsilon = f64::EPSILON);
    }

    // #[test]
    // fn test_imported_box_of_chocolates() {
    //     let box_of_choccolate = Item::new();
    // }
    // #[test]
    // fn test_imported_box_of_chocolates() {
    //     let box_of_choccolate = Item::new();
    // }
}
