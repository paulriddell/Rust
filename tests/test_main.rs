// main.rs

// The functions in this file are used to calculate the average price of a fruit    

// Create a function that finds out the average of several numbers and returns it

// update this file with your own tests

#[cfg(test)]
mod tests {
    //use super::*;
    use main::average_price;

    #[test]
    fn test_average_price() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(average_price(prices), 3.0);
    }

    #[test]
    fn test_average_price_empty() {
        let prices: Vec<f64> = vec![];
        assert!(average_price(prices).is_nan());
    }

    #[test]
    fn test_average_price_single() {
        let prices = vec![10.0];
        assert_eq!(average_price(prices), 10.0);
    }

    #[test]
    fn test_average_price_negative() {
        let prices = vec![-1.0, -2.0, -3.0, -4.0, -5.0];
        assert_eq!(average_price(prices), -3.0);
    }

    #[test]
    fn test_average_price_mixed() {
        let prices = vec![1.0, -1.0, 2.0, -2.0, 3.0, -3.0];
        assert_eq!(average_price(prices), 0.0);
    }
}