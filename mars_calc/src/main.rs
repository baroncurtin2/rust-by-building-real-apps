/* Ownership Rules
1. Each value in Rust is owned by a variable.
2. When the owner goes out of scope, the value will be deallocated.
3. There can only be one owner at a time.

The ampersand '&' allows us to pass a reference to function
References are immutable by default

&mut allows us to pass a mutable reference to a function

Reference Rules
1. In the same scope, we can have as many immutable references as we want
2. We can have a single mutable reference
*/


use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}