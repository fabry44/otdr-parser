use otdrs::parser::parse_file;
use std::fs::File;
use std::io::prelude::*;
fn main() -> std::io::Result<()>  {
    // Read the file into memory
    let mut in_file = File::open("input.sor")?;
    let mut in_data = Vec::new();
    in_file.read_to_end(&mut in_data)?;
    // Parse the file
    let mut sor = parse_file(&in_data).unwrap().1;
    // Most blocks are Options because the parser can't guarantee them
    // This is true even of "required" blocks in the spec, because otdrs is permissive.
    match sor.general_parameters.as_mut() {
        Some(gp) => { 
            // If we've got a block we can read and modify it!
            println!("Nominal wavelength of this SOR record: {:?}", gp.nominal_wavelength);
            gp.fiber_id = "Hello world".to_string();
            gp.cable_id = "Foo bar".to_string();
            // Note that we don't check the values you write a valid, outside of type enforcement
            // Even then you need to be careful to adhere to the spec. e.g. no UTF-8 glyphs
            // No emojis in your comments field 😭😭😭
        }
        None => {
            println!("Your SOR file has no General Parameters block!")
        }
    }
    // Encode the SORFile structure to a binary SOR file again...
    // Note you'd normally want to handle errors properly here, but we're an example, so...
    let bytes_to_write = sor.to_bytes().unwrap();
    // Write the file out to disk!
    let mut out_file = File::create("output.sor").unwrap();
    out_file.write_all(&bytes_to_write)?;
    return Ok(());
}
