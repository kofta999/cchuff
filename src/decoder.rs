use std::{
    collections::HashMap,
    error::Error,
    io::{Cursor, Read},
};

pub fn decode(input: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let mut reader = Cursor::new(input);
    let map = decode_header(&mut reader)?;

    dbg!(&map);

    Ok(String::new())
}

fn decode_header(reader: &mut impl Read) -> Result<HashMap<String, char>, std::io::Error> {
    let mut map: HashMap<String, char> = HashMap::new();

    let mut signature = [0u8; 4];
    reader.read_exact(&mut signature)?;

    // Read signature
    if &signature != b"CCHF" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid file signature",
        ));
    }

    // Read version
    let mut version = [0u8];
    reader.read_exact(&mut version)?;

    // Read map length
    let mut map_length = [0u8; 2];
    reader.read_exact(&mut map_length)?;
    let map_length = u16::from_le_bytes(map_length);

    for _ in 0..map_length {
        let mut char = [0u8; 4];
        reader.read_exact(&mut char)?;
        let char = char::from_u32(u32::from_le_bytes(char)).expect("Invalid char");

        let mut code_len = [0u8; 2];
        reader.read_exact(&mut code_len)?;
        let code_len = u16::from_le_bytes(code_len);

        let mut code = vec![0u8; (code_len as usize + 7) / 8];
        reader.read_exact(&mut code)?;

        let mut s = String::new();

        for b in code {
            s.push_str(&format!("{:08b}", b));
        }

        map.insert(s, char);
    }

    Ok(map)
}
