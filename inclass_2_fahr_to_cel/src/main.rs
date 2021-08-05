use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter temperature in Fahrenheit: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let fahr = parse_string(&input);

    let celcius = fahr_to_cel(fahr);
    println!("Celcius = {:0.2}", celcius);
}

fn fahr_to_cel(fahr: f32) -> f32 {
    (fahr - 32.0) * 5.0 / 9.0
}

fn parse_string(input: &str) -> f32 {
    input.trim()
        .parse::<f32>()
        .expect("User input is not a number")
}

