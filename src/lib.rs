use std::{error::Error, fs};
mod encoder;

pub fn run(input_path: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;

    encoder::encode(&input, out_path)?;

    Ok(())
}
