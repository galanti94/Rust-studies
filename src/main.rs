fn main() {
    let language = "Kotlin";

    let purpose = match language {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Inválido"
    };

    println!("O propósito de {} é {}", language, purpose);

    patter_matching();
}

fn patter_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouco",
            _ => "Muito"
        });
    }
}
