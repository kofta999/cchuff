use std::{error::Error, fs};

mod huffman_tree;

pub fn run(path: String) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    let tree = huffman_tree::build(&input);
    let coding_map = huffman_tree::generate_coding_map(tree);

    dbg!(coding_map);

    Ok(())
}
