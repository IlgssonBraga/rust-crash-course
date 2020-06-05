use std::mem;

pub fn run(){
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    // Get single value

    println!("{}", numbers[2]);

    // Re-assign value

    let mut letters: [char; 5] = ['a','b','c','d','e'];

    letters[4] = 'f';

    println!("{:?}", letters);

    // Array length

    println!("Array numbers length: {}", numbers.len());

    // Arrays are stack allocated

    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice

    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);
}