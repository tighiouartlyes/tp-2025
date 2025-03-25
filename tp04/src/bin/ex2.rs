use std::io::{self, Write};

fn main() {

    let mut texte: Box<String> = Box::new(String::new());
    print!("Entrez un texte : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    texte.push_str(&input.trim()); 
    println!("Le texte saisi est : {}", texte);
}
