use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::Write,
};

mod bit_writer;
mod huffman_tree;

pub fn run(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(input_path)?;
    let tree = huffman_tree::build(&input);
    let code_map = huffman_tree::generate_coding_map(tree);
    let out_file = write_header(output_path, &code_map)?;
    write_data(out_file, &code_map, &input)?;

    Ok(())
}

fn write_header(file_path: &str, code_map: &HashMap<char, String>) -> Result<File, Box<dyn Error>> {
    let mut file = fs::File::create(file_path)?;
    code_map.iter().for_each(|(k, v)| {
        file.write_all(format!("{v}({k})").as_bytes()).unwrap();
    });

    file.write_all("\n".as_bytes())?;

    Ok(file)
}

fn write_data(
    mut file: File,
    code_map: &HashMap<char, String>,
    input: &str,
) -> Result<(), Box<dyn Error>> {
    let mut bit_len: u8 = 0;
    let mut curr_byte: u8 = 0;

    for char in input.chars() {
        let buf = code_map.get(&char).unwrap();
        for bit in buf.chars() {
            if bit_len == 8 {
                file.write_all(&[curr_byte])?;

                curr_byte = 0;
                bit_len = 0;
            }

            curr_byte = curr_byte << 1 | (bit == '1') as u8;
            bit_len += 1
        }
    }

    if bit_len > 0 {
        curr_byte <<= 8 - bit_len;
        file.write_all(&[curr_byte])?;
    }

    Ok(())
}
