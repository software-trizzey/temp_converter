use std::io;

fn convert_celsius_to_fahrenheit(temperature: u32) -> u32 {
    // F = C * (9/5) + 32
    let fahrenheit_temp = ((temperature * 9) / 5) + 32;
    fahrenheit_temp
}

fn convert_fahrenheit_to_celsius(temperature: u32) -> u32 {
    // C = (F - 32) * 5/9
    let celsius_temp = ((temperature - 32) as f64 * (5.0 / 9.0)).round() as u32;
    celsius_temp
}

fn output_results(temperature: u32, input_unit: Option<char>) {
    match input_unit {
        Some('c') => println!("Your temperature is: {}F", temperature),
        Some('f') => println!("Your temperature is: {}C", temperature),
        _ => todo!() // probably a better way to handle this
    }
}


fn main() {
    println!("Simple Temperature Converter (v0.1.0)");
    
    println!("Please enter temperature followed by unit (ex. 20c or 75f): ");

    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read temperature!");
    user_input = user_input.trim().to_string(); // remove "/n" and ensure value is string
    
    let input_temperature_unit = user_input.pop();
    let temperature = user_input.parse::<u32>().unwrap();
    
    let converted_temperature = match input_temperature_unit {
        Some('c') => convert_celsius_to_fahrenheit(temperature),
        Some('f') => convert_fahrenheit_to_celsius(temperature),
        None => {
            println!("No temperature unit provided! Defaulting to celsius.");
            convert_celsius_to_fahrenheit(temperature)
        }
        _ => todo!(),
    };

    output_results(converted_temperature, input_temperature_unit);
}
