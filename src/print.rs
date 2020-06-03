pub fn run(){
    println!("Hello from the print.rs file!!");
    println!("{}", 1);

    // Basic formatting
    println!("{} is from {}!!", "Ilgsson", "Brazil");

    // Positional arguments
    println!("{0} is from {1} and {0} likes {2}!!", "Ilgsson", "Brazil", "Rust");

    // Named arguments
    println!("{name} is from {country} and {name} likes {language}!!", name = "Ilgsson", country = "Brazil", language = "Rust");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (10,true, "Igsson"));

    // Basic math
    println!("13 + 7 = {}", 13 + 7);
} 