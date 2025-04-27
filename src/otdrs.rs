use otdrs::parser::parse_file;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use otdrs::types::SORFile;

#[derive(Parser)]
#[clap(version = "0.4.2", author = "James Harrison <james@talkunafraid.co.uk>", about = "otdrs is a conversion utility to convert Telcordia SOR files, used by optical time-domain reflectometry testers, into open formats such as JSON")]
struct Opts {
    #[clap(index=1, required=true)]
    input_filename: String,
    #[clap(short, long, default_value="json")]
    format: String,
    #[clap(short, long, default_value="stdout")]
    output_filename: String,
    #[clap(short, long)]
    modify_script: Option<String>, // Nouvelle option pour spécifier le script de modification
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    // Lire le fichier d'entrée SOR
    let mut file = File::open(&opts.input_filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    // Parse the file, ignoring the remaining slice
    let mut sor = parse_file(&buffer).unwrap().1;

    // Exécuter le script de modification, s'il est spécifié
    if let Some(script_path) = &opts.modify_script {
        execute_modification_script(&mut sor, script_path)?;
    }

    // Traiter le fichier SOR et sérialiser en format JSON ou CBOR
    let output_bytes = if opts.format == "json" {
        serde_json::to_vec(&sor)?
    } else if opts.format == "cbor" {
        serde_cbor::to_vec(&sor)?
    } else {
        panic!("Unimplemented output format");
    };

    // Écrire le fichier de sortie
    if opts.output_filename == "stdout" {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(&output_bytes)?;
    } else {
        let mut output_file = File::create(&opts.output_filename)?;
        output_file.write_all(&output_bytes)?;
    }
    
    Ok(())
}

fn execute_modification_script(_sor: &mut SORFile, script_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Charger et exécuter le script de modification ici
    // Vous devrez adapter cette fonction pour votre cas d'utilisation spécifique
    let mut script_file = File::open(script_path)?;
    let mut script_content = String::new();
    script_file.read_to_string(&mut script_content)?;

    // Vous pouvez maintenant exécuter le script de modification sur votre structure `sor`

    Ok(())
}
