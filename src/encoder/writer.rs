use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::Write,
};

fn write_header(file_path: &str, code_map: &HashMap<char, String>) -> Result<File, Box<dyn Error>> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(b"CCHF")?;
    file.write_all(&[1])?;
    file.write_all(&(code_map.len() as u16).to_le_bytes())?;

    for (&ch, code) in code_map {
        file.write_all(&(ch as u32).to_le_bytes())?;
        file.write_all(&(code.len() as u16).to_le_bytes())?;

        let mut remaining_bits = code.len();
        let code_bits = code
            .chars()
            .fold(0u64, |acc, bit| acc << 1 | (bit == '1') as u64);

        while remaining_bits > 0 {
            let bytes_to_write = std::cmp::min(remaining_bits, 8);
            file.write_all(&[(code_bits >> (remaining_bits - bytes_to_write)) as u8])?;
            remaining_bits -= bytes_to_write;
        }
    }

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

pub fn write(
    input: &str,
    code_map: &HashMap<char, String>,
    out_path: &str,
) -> Result<(), Box<dyn Error>> {
    let out_file = write_header(out_path, code_map)?;
    write_data(out_file, code_map, input)?;

    Ok(())
}
