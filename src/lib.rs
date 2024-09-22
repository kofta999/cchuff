use std::{error::Error, fs};

mod huffman_tree;

pub fn run(path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    let tree = huffman_tree::build(&input);

    Ok(())
}
