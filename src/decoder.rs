use bitvec::prelude::*;
use std::{
    collections::HashMap,
    error::Error,
    io::{Cursor, Read},
};

pub fn decode(input: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let mut reader = Cursor::new(input);
    let map = decode_header(&mut reader)?;
    let content = decode_content(&mut reader, &map)?;

    dbg!(&content);

    Ok(content)
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

    // Read map
    for _ in 0..map_length {
        let mut char = [0u8; 4];
        reader.read_exact(&mut char)?;
        let char = char::from_u32(u32::from_le_bytes(char)).expect("Invalid char");

        let mut code_len = [0u8; 2];
        reader.read_exact(&mut code_len)?;
        let code_len = u16::from_le_bytes(code_len);

        let mut code = BitVec::<u8, Msb0>::with_capacity(code_len as usize);

        let mut buffer = vec![0u8; (code_len as usize + 7) / 8];
        reader.read_exact(&mut buffer)?;

        // Convert the buffer into a BitVec
        for byte in buffer {
            code.extend_from_bitslice(byte.view_bits::<Msb0>());
        }

        // Truncate to the exact length
        code.truncate(code_len as usize);

        // Convert BitVec to String
        let s: String = code
            .iter()
            .map(|bit| if *bit { '1' } else { '0' })
            .collect();

        map.insert(s, char);
    }

    Ok(map)
}

fn decode_content(
    reader: &mut impl Read,
    map: &HashMap<String, char>,
) -> Result<String, std::io::Error> {
    let mut result = String::new();
    let mut bitvec = BitVec::<u8, Msb0>::new();
    let mut buffer = [0u8; 1];

    loop {
        match reader.read_exact(&mut buffer) {
            Ok(_) => {
                
            }
            Err(e) => return Err(e),
        }
    }

    // let mut code = String::new();

    // loop {
    //     match reader.read_exact(&mut buffer) {
    //         Ok(_) => {
    //             for bit_pos in (0..8).rev() {
    //                 let bit = (buffer[0] >> bit_pos) & 1;
    //                 code.push(if bit == 0 { '0' } else { '1' });

    //                 if let Some(&ch) = map.get(&code) {
    //                     result.push(ch);
    //                     code.clear();
    //                 }
    //             }
    //         }
    //         Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
    //         Err(e) => return Err(e),
    //     }
    // }

    // dbg!(&result);

    // if !code.is_empty() {
    //     return Err(std::io::Error::new(
    //         std::io::ErrorKind::InvalidData,
    //         "Incomplete Huffman code at end of file",
    //     ));
    // }

    // Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_test_map() -> HashMap<String, char> {
        let mut map = HashMap::new();
        map.insert("0".to_string(), 'a');
        map.insert("10".to_string(), 'b');
        map.insert("110".to_string(), 'c');
        map.insert("111".to_string(), 'd');
        map
    }

    fn create_encoded_header(map: &HashMap<String, char>) -> Vec<u8> {
        let mut header = Vec::new();
        header.extend_from_slice(b"CCHF");
        header.push(1); // version
        header.extend_from_slice(&(map.len() as u16).to_le_bytes());

        for (code, &ch) in map {
            header.extend_from_slice(&(ch as u32).to_le_bytes());
            header.extend_from_slice(&(code.len() as u16).to_le_bytes());
            let code_bits = code
                .chars()
                .fold(0u8, |acc, bit| (acc << 1) | (bit == '1') as u8);
            header.push(code_bits << (8 - code.len()));
        }

        header
    }

    #[test]
    fn test_decode_header() {
        let map = create_test_map();
        let encoded_header = create_encoded_header(&map);
        let mut reader = Cursor::new(encoded_header);

        let decoded_map = decode_header(&mut reader).unwrap();
        assert_eq!(decoded_map, map);
    }

    #[test]
    fn test_decode_content() {
        let map = create_test_map();
        let encoded_content = vec![0b01011011, 0b10000000]; // "abcd"
        let mut reader = Cursor::new(encoded_content);

        let decoded_content = decode_content(&mut reader, &map).unwrap();
        assert_eq!(decoded_content, "abcd");
    }

    #[test]
    fn test_decode_complete() {
        let map = create_test_map();
        let mut encoded_data = create_encoded_header(&map);
        encoded_data.extend_from_slice(&[0b01011011, 0b10000000]); // "abcd"

        let decoded = decode(encoded_data).unwrap();
        assert_eq!(decoded, "abcd");
    }

    #[test]
    fn test_decode_empty_content() {
        let map = HashMap::new();
        let encoded_data = create_encoded_header(&map);

        let decoded = decode(encoded_data).unwrap();
        assert_eq!(decoded, "");
    }

    #[test]
    fn test_decode_single_char() {
        let mut map = HashMap::new();
        map.insert("0".to_string(), 'a');

        let mut encoded_data = create_encoded_header(&map);
        encoded_data.push(0b10000000); // 'a'

        let decoded = decode(encoded_data).unwrap();
        assert_eq!(decoded, "a");
    }

    #[test]
    fn test_decode_with_newline() {
        let mut map = create_test_map();
        map.insert("1111".to_string(), '\n');

        let mut encoded_data = create_encoded_header(&map);
        encoded_data.extend_from_slice(&[0b01011011, 0b11110000]); // "ab\n"

        let decoded = decode(encoded_data).unwrap();
        assert_eq!(decoded, "ab\n");
    }

    #[test]
    fn test_decode_long_input() {
        let map = create_test_map();
        let mut encoded_data = create_encoded_header(&map);
        encoded_data.extend_from_slice(&[0b01011011, 0b10110111, 0b01101111]); // "abcdabcdab"

        let decoded = decode(encoded_data).unwrap();
        assert_eq!(decoded, "abcdabcdab");
    }

    #[test]
    #[should_panic(expected = "Invalid file signature")]
    fn test_invalid_signature() {
        let mut encoded_data = vec![0; 7]; // Invalid signature
        encoded_data.extend_from_slice(&[0b01011011, 0b10000000]);

        decode(encoded_data).unwrap();
    }

    #[test]
    #[should_panic(expected = "Incomplete Huffman code at end of file")]
    fn test_incomplete_code() {
        let map = create_test_map();
        let mut encoded_data = create_encoded_header(&map);
        encoded_data.push(0b01011010); // Incomplete code

        decode(encoded_data).unwrap();
    }
}
