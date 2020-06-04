pub fn run(){
  //  let message = "hello"; // str type (immutable, fixed-length)
    let mut message2 = String::from("Hello ");

    // Get length

    println!("Length: {}", message2.len());

    message2.push('W'); // add one character to one string
    message2.push_str("orld!"); // add more than one characters to one string

    println!("{}", message2);

    // Capacity in bytes

    println!("Capacity: {}", message2.capacity());

    // Verify if a string is empty

    println!("Is empty: {}", message2.is_empty());

    // Verify if a string contains substring

    println!("Contains 'orl' {}", message2.contains("orl"));

    // Replace

    println!("Replace 'orl' for 'jjj': {}", message2.replace("orl", "jjj"));

    // Loop through string by whitespace

    for word in message2.split_whitespace(){
        println!("{}", word);
    }
}