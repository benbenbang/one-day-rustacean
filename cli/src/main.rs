use std::io;

fn main() {
    println!("Hello, this is our first rust application");
    println!("It will calculate your weight on Mars");

    let mut input = String::new();
    println!("enter your weight (kg): ");
    io::stdin().read_line(&mut input).unwrap();

    let weight: f64 = input.trim().parse().unwrap();
    let mars_wright = calculate_weight_on_mars(weight);
    println!("Your weight on Mars is {}kg", mars_wright)
}

fn calculate_weight_on_mars(weight: f64) -> f64 {
    (weight * 3.711) / 9.81
}
