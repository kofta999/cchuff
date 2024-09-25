use bitvec::prelude::*;
use std::{error::Error, io::Read, time::Instant};

#[derive(Clone, Debug)]
struct Node {
    value: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub fn decode(input: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let start = Instant::now();
    let reader = BitVec::<u8, Msb0>::from_vec(input);
    let (tree, reader) = decode_header(&reader)?;
    let header_duration = start.elapsed();
    println!("Header decoding took: {:?}", header_duration);

    let content_start = Instant::now();
    let content = decode_content(reader, &tree)?;
    let content_duration = content_start.elapsed();

    println!("Content decoding took: {:?}", content_duration);
    println!("Total decoding took: {:?}", start.elapsed());

    Ok(content)
}

fn build_tree<'a>(iter: &mut impl Iterator<Item = &'a bool>) -> Option<Box<Node>> {
    match iter.next() {
        Some(true) => {
            // Leaf node
            let mut char_bytes = [0u8; 4];
            for byte in char_bytes.iter_mut() {
                *byte = iter.take(8).fold(0, |acc, b| (acc << 1) | u8::from(*b));
            }

            let ch = char::from_u32(u32::from_be_bytes(char_bytes)).unwrap();
            Some(Box::new(Node {
                value: Some(ch),
                left: None,
                right: None,
            }))
        }
        Some(false) => {
            // Internal node
            let left = build_tree(iter);
            let right = build_tree(iter);
            Some(Box::new(Node {
                value: None,
                left,
                right,
            }))
        }
        None => None,
    }
}

type DecodeResult<'a> = (Node, &'a BitSlice<u8, Msb0>);

fn decode_header(mut reader: &BitSlice<u8, Msb0>) -> Result<DecodeResult, std::io::Error> {
    // Read signature
    let mut signature = [0u8; 4];
    reader.read_exact(&mut signature)?;
    if &signature != b"CCHF" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid file signature",
        ));
    }

    // Read version
    let mut version = [0u8];
    reader.read_exact(&mut version)?;

    // Read bitvec length
    let mut bitvec_length = [0u8; 4];
    reader.read_exact(&mut bitvec_length)?;
    let bitvec_length = u32::from_le_bytes(bitvec_length);

    // Build the tree
    let mut iter = reader.iter().by_refs();
    let tree = build_tree(&mut iter).expect("Failed to build Huffman tree");

    // Advance the reader past the tree data
    let tree_bits = (bitvec_length as usize + 7) / 8;
    reader = &reader[tree_bits * 8..];

    Ok((*tree, reader))
}

fn decode_content(mut reader: &BitSlice<u8, Msb0>, head: &Node) -> Result<String, std::io::Error> {
    let mut result = String::new();
    let mut current_node = head;

    let mut total_bits = [0u8; 4];
    reader.read_exact(&mut total_bits)?;
    let total_bits = u32::from_le_bytes(total_bits);

    let mut iter = reader.iter();

    for _ in 0..total_bits {
        let bit = iter.next().expect("Unexpected end of input");

        current_node = if *bit {
            current_node.right.as_ref().expect("Invalid Huffman tree")
        } else {
            current_node.left.as_ref().expect("Invalid Huffman tree")
        };

        if let Some(ch) = current_node.value {
            result.push(ch);
            current_node = head; // Reset to the root
        }
    }

    // TODO: fix that check (total_bytes is the compressed size)
    // if result.bytes().len() != total_bytes as usize {
    //     return Err(std::io::Error::new(
    //         std::io::ErrorKind::InvalidData,
    //         "Input length does not match output length",
    //     ));
    // }

    Ok(result)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::io::Cursor;

//     fn create_test_map() -> BTreeMap<String, char> {
//         let mut map = BTreeMap::new();
//         map.insert("0".to_string(), 'a');
//         map.insert("10".to_string(), 'b');
//         map.insert("110".to_string(), 'c');
//         map.insert("111".to_string(), 'd');
//         map
//     }

//     fn create_encoded_header(map: &BTreeMap<String, char>) -> Vec<u8> {
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

//     fn create_encoded_content(input: &str, map: &BTreeMap<String, char>) -> Vec<u8> {
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
//         let mut map = BTreeMap::new();
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
