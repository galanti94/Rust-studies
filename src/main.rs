fn main() {
    let idade:u8 = 15;
    let autorizado:bool = true;

    if idade > 18 || autorizado {
        println!("Pode beber cerveja!");
        // return;
    } else {println!("Não pode beber certeja!");}    

    // Posso passar tudo como expressão
    let condicao = if idade > 18 { "maior" } else { "menor" };

    println!("{}", condicao);
}
