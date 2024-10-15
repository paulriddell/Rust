// The functions in this file are used to calculate the average price of a fruit    

// Create a function that finds out the average of several numbers and returns it
pub fn average_price(prices: Vec<f64>) -> f64 {
    if prices.is_empty() {
        return f64::NAN;
    }
    let sum: f64 = prices.iter().sum();
    sum / prices.len() as f64
}

fn main() {
    println!("Hello, world!");
    println!("I am a fruit vendor");
    let item = "mango";
    let price = 2.50;
    let quantity = 3;
    println!("{} costs ${:.2} for {} pieces", item, price, quantity);
}
