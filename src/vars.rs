pub fn run(){
    let name = "Ilgsson";

    let mut age = 25;

    println!("My name is {} and I am {}.", name, age);

    age = 26;

    println!("My name is {} and I am {}.", name, age);

    // Define constant

    const ID: i32 = 001; // By convention, use all letter with uppercase

    println!("ID: {}", ID);

    // Assign multiple variables

    let (num1, num2, num3) = (10,20,30);

    println!("{} {} {}", num1, num2, num3);
}