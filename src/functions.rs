pub fn run(){
    greeting("Hello", "Ilgsson");

    // Bind functions values to variables

    let get_sum = add(3,2);
    println!("Sum: {}", get_sum);

    // Closures
    let n3: i32 = 10;
    let add_numbers = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_numbers(4,4,));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}", greet, name);
}

fn add(num1: i32, num2: i32) -> i32{
    return num1 + num2;
}