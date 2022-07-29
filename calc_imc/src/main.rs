use std::io;
fn main() {
    println!("Escreva sua altura: ");
    let mut height = String::new();

    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");

    let height: f32 = height
        .trim()
        .parse()
        .expect("é necessario informa um valor");

    println!("Escreva seu peso: ");

    let mut weight = String::new();

    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line");

    let weight: f32 = weight
        .trim()
        .parse()
        .expect("é necessario informa um valor");

    let imc = weight / height.powi(2);
    println!("seu imc: {:.2} peso: {} altura: {}", &imc, weight, height)
}
