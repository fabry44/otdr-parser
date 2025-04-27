use clap::Parser;
use otdrs::types::SORFile;
use serde_json;
use std::fs;

#[derive(Parser)]
#[clap(name = "wotdrs", version = "1.0", about = "Reconvertit un JSON SOR vers un fichier .sor en supprimant les blocs propriétaires")]
struct Opts {
    #[clap(short, long, help = "Chemin du fichier JSON généré par otdrs")]  
    input: String,
    #[clap(short, long, help = "Chemin du fichier .sor de sortie")]  
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    // Lire le JSON et désérialiser
    let json_str = fs::read_to_string(&opts.input)?;
    let mut sor: SORFile = serde_json::from_str(&json_str)?;
    // Supprimer tous les blocks propriétaires
    sor.proprietary_blocks.clear();
    // Générer le binaire SOR
    let bytes = sor.to_bytes()?;
    // Écrire dans le fichier de sortie
    fs::write(&opts.output, bytes)?;
    println!("Fichier écrit : {}", opts.output);
    Ok(())
}
