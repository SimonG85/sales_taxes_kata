/// This lib allows to compute price after tax of an item
/// and a basket of items.
use std::str::FromStr;

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

impl FromStr for Item {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split(" at ").collect();
        if components.len() != 2 {
            return Err("Invalid string: missing 'at'".to_string());
        }
        let descr = components[0];
        let price = components[1].parse().map_err(|_| "Price is not valid")?;
        let imported = if descr.contains("imported") {
            Imported::Yes
        } else {
            Imported::No
        };
        let category = if descr.contains("pills") {
            Category::Medical(descr.to_string())
        } else if descr.contains("chocolate") {
            Category::Food(descr.to_string())
        } else if descr.contains("perfume") {
            Category::Other(descr.to_string())
        } else {
            Category::Other(descr.to_string())
        };
        Item::new(price, imported, category).map_err(|e| e.to_string())
    }
}

struct Basket<T: Tax + ToString> {
    elements: Vec<T>,
}

impl<T> Basket<T>
where
    T: Tax + ToString,
{
    fn new(elements: Vec<T>) -> Self {
        Self { elements }
    }
    fn get_total(&self) -> f64 {
        self.elements
            .iter()
            .fold(0.0, |acc, x| acc + x.get_prices().0 + x.get_prices().1)
    }
    fn get_tax(&self) -> f64 {
        self.elements
            .iter()
            .fold(0.0, |acc, x| acc + x.get_prices().1)
    }
}

impl<T> ToString for Basket<T>
where
    T: Tax + ToString,
{
    fn to_string(&self) -> String {
        let mut string_element: Vec<String> = self.elements.iter().map(|s| s.to_string()).collect();
        string_element.push(format!("Sales Taxes: {}", self.get_tax()));
        string_element.push(format!("Total: {}", self.get_total()));
        string_element.join("\n")
    }
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
    #[test]
    fn test_parse_item_invalid_format() {
        let input = "1 bottle of perfume 18.99";
        assert!(Item::from_str(input).is_err());
    }
    #[test]
    fn test_parse_item_invalid_price() {
        let input = "1 bottle of perfume at invalid";
        assert!(Item::from_str(input).is_err());
    }
    #[test]
    fn test_parse_item_negative_price() {
        let input = "1 bottle of perfume at -18.99";
        assert!(Item::from_str(input).is_err());
    }
}

#[cfg(test)]
mod string_to_item_tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_parse_item_imported_perfume() {
        let input = "1 imported bottle of perfume at 27.99";
        let item = Item::from_str(input).unwrap();
        assert!(matches!(item.imported, Imported::Yes));
        assert!(matches!(item.category, Category::Other(_)));
        assert_relative_eq!(item.clean_price, 27.99, epsilon = f64::EPSILON);
    }
    #[test]
    fn test_parse_item_regular_perfume() {
        let input = "1 bottle of perfume at 18.99";
        let item = Item::from_str(input).unwrap();
        assert!(matches!(item.imported, Imported::No));
        assert!(matches!(item.category, Category::Other(_)));
        assert_relative_eq!(item.clean_price, 18.99, epsilon = f64::EPSILON);
    }
}

#[cfg(test)]
mod basket_tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_total() {
        let imported_perfume = Item::new(
            27.99,
            Imported::Yes,
            Category::Other("bottle of perfume".to_string()),
        )
        .unwrap();
        let perfume = Item::new(
            18.99,
            Imported::No,
            Category::Other("bottle of perfume".to_string()),
        )
        .unwrap();
        let headache_pills = Item::new(
            9.75,
            Imported::No,
            Category::Medical("packet of headache pills".to_string()),
        )
        .unwrap();
        let imported_chocolates = Item::new(
            11.25,
            Imported::Yes,
            Category::Food("box of chocolates".to_string()),
        )
        .unwrap();
        let basket = Basket::new(vec![
            imported_perfume,
            perfume,
            headache_pills,
            imported_chocolates,
        ]);
        assert_relative_eq!(basket.get_total(), 74.68, epsilon = f64::EPSILON);
        assert_relative_eq!(basket.get_tax(), 6.70, epsilon = f64::EPSILON);
        assert_eq!(
            basket.to_string(),
            "1 imported bottle of perfume: 32.19
1 bottle of perfume: 20.89
1 packet of headache pills: 9.75
1 imported box of chocolates: 11.85
Sales Taxes: 6.70
Total: 74.68
"
        );
    }
}
