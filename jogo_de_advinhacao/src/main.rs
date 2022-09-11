


use std::io;
fn main() {
    println!("Advinhe o número");

    println!("Digite seu palpite");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

        println!("Você disse:{}", palpite);
}
