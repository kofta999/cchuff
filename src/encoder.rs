use std::{error::Error, fs, time::Instant};

mod huffman;
mod writer;

pub fn encode(input: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let (freq_map, bitvec) = huffman::build(input);
    let mut file = fs::File::create(out_path)?;
    writer::write(&mut file, input, &freq_map, bitvec)?;
    println!("Total encoding took: {:?}", start.elapsed());

    Ok(())
}
