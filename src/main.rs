fn main() {
    let multiplicator:u8 = 5;

    let mut counter:u8 = 0;

    while counter < 10 {
        counter += 1;

        if counter == 5 { continue };

        println!("{} x {} = {}", multiplicator, counter, multiplicator * counter);
    }

    counter = 0;

    // Equivalente ao while infinito while true
    loop {
        counter +=1;

        println!("{} x {} = {}", multiplicator, counter, multiplicator * counter);

        if counter == 10 {
            break;
        }
    }

    for index in 1..=10 {
        println!("{} x {} = {}", index, multiplicator, index * multiplicator);
    }
}
