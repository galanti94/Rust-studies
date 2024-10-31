// static creates an address for my variable
static GLOBAL_VARIABLE:u8 = 1;

// I can create a global variable that is mutable, but it is unsafe
static mut UNSAFE_GLOBAL:u8 = 10;

fn main() {
    // In some cases I only need constant values that will not be manipulated dring execution

    const PI:f32 = 3.14;

    println!("PI = {}", PI); // Compiler only substitutes

    println!("global_variable = {}", GLOBAL_VARIABLE);

    unsafe {
        println!("global_variable = {}", UNSAFE_GLOBAL);
    }
}
