use std::io::stdin;
pub fn menu() -> i32 {
    let mut option = String::new();
    println!("---- Menu ----");
    println!("[1] - Show Posts");
    println!("[2] - Create new Post");
    println!("[3] - Publish Post");
    println!("[4] - Show Draft Posts");
    println!("[5] - Delete Post");
    println!("[6] - Quit");

    stdin()
        .read_line(&mut option)
        .expect("algo de errado não ta certo");

    option
        .trim()
        .parse::<i32>()
        .expect("não foi possivel realizar o parse")
}
