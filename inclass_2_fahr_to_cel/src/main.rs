use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter temperature in Fahrenheit: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let fahr = input
        .trim()
        .parse::<f32>()
        .expect("User input is not a number");

    let celcius = (fahr - 32.0) * 5.0 / 9.0;
    println!("Celcius = {:0.2}", celcius)
}
