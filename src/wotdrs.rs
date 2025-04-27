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

#[cfg(test)]
mod tests {
    use otdrs::parser::parse_file;
    use otdrs::types::SORFile;
    use serde_json;

    #[test]
    fn test_remove_proprietary_and_roundtrip() {
        // Charger un SOR example
        let data = include_bytes!("../data/example1-noyes-ofl280.sor");
        let sor: SORFile = parse_file(data).unwrap().1;
        // Le fichier d'origine contient des blocs propriétaires
        assert!(!sor.proprietary_blocks.is_empty());
        // Sérialiser en JSON et désérialiser
        let json = serde_json::to_string(&sor).unwrap();
        let mut sor2: SORFile = serde_json::from_str(&json).unwrap();
        // Appliquer la logique de wotdrs : suppression des blocs propriétaires
        sor2.proprietary_blocks.clear();
        // Générer le binaire SOR et reparser
        let bytes = sor2.to_bytes().unwrap();
        let new_sor: SORFile = parse_file(&bytes).unwrap().1;
        // Vérifier qu'il n'y a plus de blocs propriétaires
        assert!(new_sor.proprietary_blocks.is_empty());
        // Vérifier que la map ne contient plus que les blocs standards
        let ids: Vec<&str> = new_sor.map.block_info.iter().map(|bi| bi.identifier.as_str()).collect();
        assert_eq!(ids, vec!["GenParams", "SupParams", "FxdParams", "KeyEvents", "DataPts", "Cksum"]);
        // Vérifier que les GeneralParameters restent inchangés
        assert_eq!(new_sor.general_parameters, sor2.general_parameters);
    }
}
