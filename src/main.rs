use std::io;


fn main() {
    println!("A simple temp converter!");
    
    println!("Please enter temperature (celsius): ");

    let mut temperature: String = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature!");

    println!("User temperature {}", temperature);
}
