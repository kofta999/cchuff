use std::{error::Error, fs};
mod decoder;
mod encoder;

pub fn run(input_path: &str, out_path: Option<&str>) -> Result<(), Box<dyn Error>> {
    match out_path {
        Some(out) => {
            let input = fs::read_to_string(input_path)?;
            encoder::encode(&input, out)?;
        }
        None => {
            let input = fs::read(input_path)?;
            decoder::decode(input)?;
        }
    }

    Ok(())
}
