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

    let mut total_bytes = [0u8; 4];
    reader.read_exact(&mut total_bytes)?;
    let total_bytes = u32::from_le_bytes(total_bytes);

    let mut buffer = vec![0u8; total_bytes as usize];
    reader.read_exact(&mut buffer)?;

    let bitvec = BitVec::<u8, Msb0>::from_vec(buffer);
    let mut code = BitVec::<u8, Msb0>::new();

    for bit in bitvec.iter() {
        code.push(*bit);
        let s: String = code.iter().map(|b| if *b { '1' } else { '0' }).collect();

        if let Some(&ch) = map.get(&s) {
            result.push(ch);
            code.clear();
        }

        if code.len() > map.keys().map(|s| s.len()).max().unwrap_or(0) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid Huffman code",
            ));
        }
    }

    Ok(result)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::io::Cursor;

//     fn create_test_map() -> HashMap<String, char> {
//         let mut map = HashMap::new();
//         map.insert("0".to_string(), 'a');
//         map.insert("10".to_string(), 'b');
//         map.insert("110".to_string(), 'c');
//         map.insert("111".to_string(), 'd');
//         map
//     }

//     fn create_encoded_header(map: &HashMap<String, char>) -> Vec<u8> {
//         let mut header = Vec::new();
//         header.extend_from_slice(b"CCHF");
//         header.push(1); // version
//         header.extend_from_slice(&(map.len() as u16).to_le_bytes());

//         for (code, &ch) in map {
//             header.extend_from_slice(&(ch as u32).to_le_bytes());
//             header.extend_from_slice(&(code.len() as u16).to_le_bytes());

//             let mut bitvec = BitVec::<u8, Msb0>::new();
//             for bit in code.chars() {
//                 bitvec.push(bit == '1');
//             }
//             header.extend_from_slice(bitvec.as_raw_slice());
//         }

//         header
//     }

//     fn create_encoded_content(input: &str, map: &HashMap<String, char>) -> Vec<u8> {
//         let mut header = Vec::new();
//         header.extend_from_slice(b"CCHF");
//         header.push(1); // version
//         header.extend_from_slice(&(map.len() as u16).to_le_bytes());

//         for (code, &ch) in map {
//             header.extend_from_slice(&(ch as u32).to_le_bytes());
//             header.extend_from_slice(&(code.len() as u16).to_le_bytes());

//             let mut bitvec = BitVec::<u8, Msb0>::new();
//             for bit in code.chars() {
//                 bitvec.push(bit == '1');
//             }
//             header.extend_from_slice(bitvec.as_raw_slice());
//         }

//         header
//     }

//     #[test]
//     fn test_decode_header() {
//         let map = create_test_map();
//         let encoded_header = create_encoded_header(&map);
//         let mut reader = Cursor::new(encoded_header);

//         let decoded_map = decode_header(&mut reader).unwrap();
//         assert_eq!(decoded_map, map);
//     }

//     #[test]
//     fn test_decode_content() {
//         let map = create_test_map();
//         let input = "abcd";
//         let encoded_content = create_encoded_content(input, &map);
//         let mut reader = Cursor::new(encoded_content);

//         let decoded_content = decode_content(&mut reader, &map).unwrap();
//         assert_eq!(decoded_content, input);
//     }

//     #[test]
//     fn test_decode_complete() {
//         let map = create_test_map();
//         let input = "abcdabcd";
//         let mut encoded_data = create_encoded_header(&map);
//         encoded_data.extend_from_slice(&create_encoded_content(input, &map));

//         let decoded = decode(encoded_data).unwrap();
//         assert_eq!(decoded, input);
//     }

//     #[test]
//     fn test_decode_empty_content() {
//         let map = create_test_map();
//         let input = "";
//         let mut encoded_data = create_encoded_header(&map);
//         encoded_data.extend_from_slice(&create_encoded_content(input, &map));

//         let decoded = decode(encoded_data).unwrap();
//         assert_eq!(decoded, input);
//     }

//     #[test]
//     fn test_decode_single_char() {
//         let mut map = HashMap::new();
//         map.insert("0".to_string(), 'a');
//         let input = "a";
//         let mut encoded_data = create_encoded_header(&map);
//         encoded_data.extend_from_slice(&create_encoded_content(input, &map));

//         let decoded = decode(encoded_data).unwrap();
//         assert_eq!(decoded, input);
//     }

//     #[test]
//     fn test_decode_with_newline() {
//         let mut map = create_test_map();
//         map.insert("1111".to_string(), '\n');
//         let input = "ab\nc";
//         let mut encoded_data = create_encoded_header(&map);
//         encoded_data.extend_from_slice(&create_encoded_content(input, &map));

//         let decoded = decode(encoded_data).unwrap();
//         assert_eq!(decoded, input);
//     }

//     #[test]
//     fn test_decode_long_input() {
//         let map = create_test_map();
//         let input = "abcdabcdabcdabcd";
//         let mut encoded_data = create_encoded_header(&map);
//         encoded_data.extend_from_slice(&create_encoded_content(input, &map));

//         let decoded = decode(encoded_data).unwrap();
//         assert_eq!(decoded, input);
//     }

//     #[test]
//     #[should_panic(expected = "Invalid file signature")]
//     fn test_invalid_signature() {
//         let mut encoded_data = vec![0; 4]; // Invalid signature
//         encoded_data.extend_from_slice(&[1, 0, 0]); // Version and empty map
//         decode(encoded_data).unwrap();
//     }

//     #[test]
//     #[should_panic(expected = "Incomplete Huffman code at end of file")]
//     fn test_incomplete_code() {
//         let map = create_test_map();
//         let mut encoded_data = create_encoded_header(&map);
//         encoded_data.push(0b01011010); // Incomplete code

//         decode(encoded_data).unwrap();
//     }
// }
