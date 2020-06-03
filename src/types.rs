pub fn run(){
    // By default i32
    let x = 1;

    // By default f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4352452525;

    // Find max size

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max f32: {}", std::f32::MAX);
    println!("Max f32: {}", std::f64::MAX);

    // Boolean

    let is_active = true;

    println!("{:?}",(x, y, z, is_active, 2 > 4));

    // Strings: single quotes for char type and double quotes for str type

    let a1 = 'a';

    let face = '\u{1F600}';

    let text = "Hello";

    println!("{} {} {}", a1, face, text);
}