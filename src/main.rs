fn main() {
    let x = 1000;

    println!("The number {} has {} bytes", x, std::mem::size_of_val(&x));

    let decimal: f32 = 2.5;

    println!("The number {} has {} bytes", decimal, std::mem::size_of_val(&decimal));

    // I can't have false as 0 in Rust (what is common in C++)
    let check = false; // True boolean
    let check_zero = 0; // Not a bollean

    // Rust uses UTF-8 thats why char has 4 bytes
    let letter = 'A';

    println!("The number {} has {} bytes", letter, std::mem::size_of_val(&letter));
}
