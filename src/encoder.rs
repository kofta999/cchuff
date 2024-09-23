use std::error::Error;

mod huffman;
mod writer;

pub fn encode(input: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let code_map = huffman::build(input);
    writer::write(input, &code_map, out_path)?;

    Ok(())
}
