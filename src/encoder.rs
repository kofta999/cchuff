use std::{error::Error, fs};

mod huffman;
mod writer;

pub fn encode(input: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let code_map = huffman::build(input);
    let mut file = fs::File::create(out_path)?;
    writer::write(&mut file, input, &code_map)?;

    Ok(())
}
