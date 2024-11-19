use std::io;
use colored::*;


const PROGRAM_TITLE: &str = "Simple Temperature Converter (v0.1.3)";
const CELSIUS: char = 'c';
const FAHRENHEIT: char = 'f';
const QUIT: char = 'q';


fn convert_celsius_to_fahrenheit(temperature: i32) -> i32 {
    // F = C * (9/5) + 32
    let fahrenheit_temp = ((temperature * 9) / 5) + 32;
    fahrenheit_temp
}

fn convert_fahrenheit_to_celsius(temperature: i32) -> i32 {
    // C = (F - 32) * 5/9
    let celsius_temp = ((temperature - 32) as f64 * (5.0 / 9.0)).round() as i32;
    celsius_temp
}

fn output_results(temperature: i32, input_unit: char) {
    match input_unit {
        CELSIUS => println!(
            "{}",
            format!("\nTemperature is: {}F", temperature).green().underline()
        ),
        FAHRENHEIT => println!(
            "{}",
            format!("\nTemperature is: {}C", temperature).green().underline()
        ),
        _ => println!("{}", "Unexpected error: invalid unit detected!".red()),
    }
}


fn main() {
    println!("{}", PROGRAM_TITLE);
    
    loop {
        println!("\nEnter temperature to convert (using format of 20c or 75f):");
        
        let mut user_input: String = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read temperature!");
        user_input = user_input.trim().to_string(); // remove "/n" and ensure value is string
        
        let input_temperature_unit = match user_input.pop().map(|character| character.to_ascii_lowercase()) {
            Some(CELSIUS) => CELSIUS,
            Some(FAHRENHEIT) => FAHRENHEIT,
            Some(QUIT) => {
                println!("Quitting {}... Cya nerd 🤓!", PROGRAM_TITLE);
                break
            }
            _ => {
                println!("{}", "Invalid temperature detected! Please use format of '20c' or '75f'".red());
                continue;
            }
        };
        let temperature = match user_input.parse::<i32>() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Invalid temperature input detected! Please use format [number][unit]");
                continue;
            }
        };
        
        let converted_temperature = match input_temperature_unit {
            CELSIUS => convert_celsius_to_fahrenheit(temperature),
            FAHRENHEIT => convert_fahrenheit_to_celsius(temperature),
            _ => unreachable!("This case should not occur due to earlier validation"),
        };

        output_results(converted_temperature, input_temperature_unit);
        println!("{}", "\nNote: enter 'q' to quit program.".yellow())
    }
}
