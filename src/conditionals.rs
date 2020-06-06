pub fn run(){

    // IF-ELSE
    let age: u8 = 25;
    let check_id: bool = false;
    let knows_person_age = true;

    if age >= 21 && check_id || knows_person_age {
        println!("Bartender: What would you like to drink?");
    }else if age < 21 && check_id {
        println!("Sorry, you must to leave!");
    }else {
        println!("Let me see your id!")
    }

    // Shorthand IF

    let is_of_age = if age >= 21 { true } else {false};

    println!("Is of Age: {}", is_of_age);
}