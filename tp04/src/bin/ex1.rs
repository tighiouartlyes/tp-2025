use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn copier_fichier(fichier_source: &str, fichier_destination: &str) -> io::Result<()> {
    // Ouvrir le fichier source en mode lecture
    let mut source = File::open(fichier_source)?;

    // Ouvrir le fichier de destination en mode écriture (création ou remplacement)
    let mut destination = File::create(fichier_destination)?;

    // Buffer pour stocker temporairement les données lues
    let mut buffer = Vec::new();

    // Lire tout le contenu du fichier source dans le buffer
    source.read_to_end(&mut buffer)?;

    // Écrire le contenu du buffer dans le fichier de destination
    destination.write_all(&buffer)?;

    Ok(())
}
fn main() {
    // Récupérer les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();

    // Vérifier que deux arguments sont fournis
    if args.len() != 3 {
        eprintln!("Usage: {} <fichier_source> <fichier_destination>", args[0]);
        return;
    }

    let fichier_source = &args[1];
    let fichier_destination = &args[2];

    // Appeler la fonction pour copier le fichier
    match copier_fichier(fichier_source, fichier_destination) {
        Ok(()) => println!("Le fichier a été copié avec succès."),
        Err(e) => eprintln!("Erreur lors de la copie du fichier : {}", e),
    }
}
