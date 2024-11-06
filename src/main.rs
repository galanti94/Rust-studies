fn main() {
    let language = "Kotlin";
    let purpose = match language {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Inválido"
    };

    println!("O propósito de {} é {}", language, purpose);
}
