/// This lib allows to compute price after tax of an item
/// and a basket of items.

enum Imported {
    Yes,
    No,
}

enum Category {
    Book(String),
    Food(String),
    Medical(String),
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

impl ToString for Item {
    fn to_string(&self) -> String {
        let name = match &self.category {
            Category::Book(x) | Category::Food(x) | Category::Medical(x) | Category::Other(x) => x,
        };

        let prefix = if matches!(self.imported, Imported::Yes) {
            "1 imported "
        } else {
            "1 "
        };

        format!("{}{}: {}", prefix, name, self.get_prices().0)
    }
}

fn round_numbers(number: f64) -> f64 {
    (number * 20.0).round() / 20.0
}

impl Tax for Item {
    fn get_prices(&self) -> (f64, f64) {
        match (&self.category, &self.imported) {
            (Category::Book(_) | Category::Food(_) | Category::Medical(_), Imported::No) => {
                (self.clean_price, 0.0)
            }
            (Category::Other(_), Imported::No) => {
                (self.clean_price, round_numbers(self.clean_price * 0.10))
            }
            (Category::Book(_) | Category::Food(_) | Category::Medical(_), Imported::Yes) => {
                (self.clean_price, round_numbers(self.clean_price * (0.05)))
            }
            (Category::Other(_), Imported::Yes) => (
                self.clean_price,
                round_numbers(self.clean_price * (0.10 + 0.05)),
            ),
        }
    }
}

struct Basket<T: Tax> {
    elements: [T],
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_book() {
        let book = Item::new(12.49, Imported::No, Category::Book("book".to_string())).unwrap();
        let (clean_price, tax) = book.get_prices();
        let expected = (12.49, 0.0);
        assert_relative_eq!(clean_price, expected.0, epsilon = f64::EPSILON);
        assert_relative_eq!(tax, expected.1, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_music_cd() {
        let music_cd =
            Item::new(14.99, Imported::No, Category::Other("music CD".to_string())).unwrap();
        let (clean_price, tax) = music_cd.get_prices();
        let expected = (14.99, 1.5);
        assert_relative_eq!(clean_price, expected.0, epsilon = f64::EPSILON);
        assert_relative_eq!(tax, expected.1, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_imported_box_chocolates() {
        let box_chocolates =
            Item::new(10.00, Imported::Yes, Category::Food("".to_string())).unwrap();
        let (clean_price, tax) = box_chocolates.get_prices();
        let expected = (10.0, 0.50);
        assert_relative_eq!(clean_price, expected.0, epsilon = f64::EPSILON);
        assert_relative_eq!(tax, expected.1, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_imported_perfume() {
        let imported_perfume = Item::new(
            47.50,
            Imported::Yes,
            Category::Other("bottle of perfume".to_string()),
        )
        .unwrap();
        let (clean_price, tax) = imported_perfume.get_prices();
        let expected = (47.50, 7.15);
        assert_relative_eq!(clean_price, expected.0, epsilon = f64::EPSILON);
        assert_relative_eq!(tax, expected.1, epsilon = f64::EPSILON);
    }
}

#[cfg(test)]
mod multiple_item_tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_purchase_1() {
        let book = Item::new(12.49, Imported::No, Category::Book("".to_string())).unwrap();
        let book_prices = book.get_prices();
        let music_cd = Item::new(14.99, Imported::No, Category::Other("CD".to_string())).unwrap();
        let music_cd_prices = music_cd.get_prices();
        let bar_chocolates = Item::new(0.85, Imported::No, Category::Food("".to_string())).unwrap();
        let bar_chocolates_prices = bar_chocolates.get_prices();
        let clean_price = book_prices.0 + music_cd_prices.0 + bar_chocolates_prices.0;
        let taxes = book_prices.1 + music_cd_prices.1 + bar_chocolates_prices.1;
        assert_relative_eq!(clean_price, 28.33, epsilon = f64::EPSILON);
        assert_relative_eq!(taxes, 1.50, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_purchase_2() {
        let chocolates_box =
            Item::new(10.00, Imported::Yes, Category::Food("".to_string())).unwrap();
        let choc_box_prices = chocolates_box.get_prices();
        let imported_perfume = Item::new(
            47.50,
            Imported::Yes,
            Category::Other("bottle of perfume".to_string()),
        )
        .unwrap();
        let imported_perf_prices = imported_perfume.get_prices();
        let clean_price = choc_box_prices.0 + imported_perf_prices.0;
        let taxes = choc_box_prices.1 + imported_perf_prices.1;
        assert_relative_eq!(clean_price, 57.50, epsilon = f64::EPSILON);
        assert_relative_eq!(taxes, 7.65, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_purchase_3() {
        let imported_perfume = Item::new(
            27.99,
            Imported::Yes,
            Category::Other("bottle of perfume".to_string()),
        )
        .unwrap();
        let imported_perf_prices = imported_perfume.get_prices();
        let perfume = Item::new(
            18.99,
            Imported::No,
            Category::Other("bottle of perfume".to_string()),
        )
        .unwrap();
        let perf_prices = perfume.get_prices();
        let headache_pills =
            Item::new(9.75, Imported::No, Category::Medical("".to_string())).unwrap();
        let pills_prices = headache_pills.get_prices();
        let imported_chocolates =
            Item::new(11.25, Imported::Yes, Category::Food("".to_string())).unwrap();
        let imported_choc_prices = imported_chocolates.get_prices();

        let clean_price =
            imported_perf_prices.0 + perf_prices.0 + pills_prices.0 + imported_choc_prices.0;
        let taxes =
            imported_perf_prices.1 + perf_prices.1 + pills_prices.1 + imported_choc_prices.1;
        assert_relative_eq!(clean_price, 67.98, epsilon = f64::EPSILON);
        assert_relative_eq!(taxes, 6.70, epsilon = f64::EPSILON);
    }
}

#[cfg(test)]
mod item_to_string_tests {
    use super::*;
    #[test]
    fn test_book() {
        let book = Item::new(12.49, Imported::No, Category::Book("book".to_string())).unwrap();
        let book_to_string = "1 book: 12.49".to_string();
        assert_eq!(book.to_string(), book_to_string);
    }
    #[test]
    fn test_music_cd() {
        let music_cd =
            Item::new(16.49, Imported::No, Category::Other("music CD".to_string())).unwrap();
        let music_cd_to_string = "1 music CD: 16.49".to_string();
        assert_eq!(music_cd.to_string(), music_cd_to_string);
    }
}
