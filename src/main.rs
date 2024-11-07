use std::vec;

use rand::Error;

fn main() {
    match resultado() {
        Ok(s) => println!("Deu tudo certo: {}", s),
        Err(num) => println!("Deu tudo errado: {}", num)
    };
    
}

fn resultado() -> Result<String, u8> {
    // Ok(String::from("Deu certo"))
    Err(42)

}

