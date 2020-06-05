use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    // Add on to vector

    numbers.push(6);
    numbers.push(7);

    // Get single value

    println!("{}", numbers[2]);

    // Re-assign value

    numbers[4] = 10;

    println!("{:?}", numbers);

    // Pop off last value

    numbers.pop();

    println!("{:?}", numbers);

    // Vector length

    println!("Vector numbers length: {}", numbers.len());

    // Vector are stack allocated

    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice

    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);

    // Loop through vector values

    for x in numbers.iter() {
        println!("{}", x);
    }

    // Loop & mutate values

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec inter_mut: {:?}", numbers);
}