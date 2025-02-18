// Declare a constant for the freezing point of water in fahrenheit
const FREEZING_POINT_FAHRENHEIT: f32 = 32.0;

// Converts fahrenheit to celsius
fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

// Converts celsius to fahrenheit
fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    // aWe start by declaring our variable making it mutable and setting it to the global
    let mut temperature_f = FREEZING_POINT_FAHRENHEIT;

    // Create a new variable to hlod the converted temperature
    let celsius = fahrenheit_to_celsius(temperature_f);
    //print
    println!("{temperature_f} °F = {celsius:.2} °C");

    // Use a loop to convert and print the next 5 integer temperatures
    for increment in 1..=5 {
        let next_temp_f = temperature_f + increment as f32;
        let next_temp_c = fahrenheit_to_celsius(next_temp_f);
        println!("{next_temp_f} °F = {next_temp_c:.2} °C");
    }
}
