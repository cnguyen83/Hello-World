// Program to convert Fahrenheit to Celsius
// Example program to practice Rust

use std::io;

// TODO: let user quit the program w/o needing ctrl+c
fn main() {
    loop {    
	let fahrenheit: i32 = get_temp();

    	let celsius: f32 = convert_ftoc(fahrenheit);

    	println!("The temperature is {} in Celsius.", celsius)
    };
}

fn convert_ftoc(f: i32) -> f32 {
    let fahrenheit = f as f32;
    let celsius: f32 = (fahrenheit - 32.0) / 1.8;
    celsius
}

fn get_temp() -> i32 {
    loop {
        let mut degree = String::new();
        println!("Enter the temperature in Fahrenheit:");
	io::stdin().read_line(&mut degree)
            .expect("Failed to read input.");

        let degree: i32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        return degree;
    }
}
