use std::{collections::HashMap, error::Error, io::Write, time::Instant};

use bitvec::prelude::*;

fn write_header<W: Write>(
    writer: &mut W,
    code_map: &HashMap<char, String>,
) -> Result<(), Box<dyn Error>> {
    writer.write_all(b"CCHF")?;
    writer.write_all(&[1])?;
    writer.write_all(&(code_map.len() as u16).to_le_bytes())?;

    for (&ch, code) in code_map {
        writer.write_all(&(ch as u32).to_le_bytes())?;
        writer.write_all(&(code.len() as u16).to_le_bytes())?;

        let mut bitvec = BitVec::<u8, Msb0>::new();

        for bit in code.chars() {
            bitvec.push(bit == '1');
        }

        while bitvec.len() % 8 != 0 {
            bitvec.push(false);
        }

        writer.write_all(bitvec.as_raw_slice())?;
    }

    Ok(())
}

fn write_data<W: Write>(
    writer: &mut W,
    code_map: &HashMap<char, String>,
    input: &str,
) -> Result<(), Box<dyn Error>> {
    let mut bitvec = BitVec::<u8, Msb0>::new();

    for char in input.chars() {
        let code = code_map.get(&char).unwrap();

        for code_bit in code.chars() {
            bitvec.push(code_bit == '1');
        }
    }

    // Calculate and write total byte size
    let total_bytes = (bitvec.len() + 7) / 8; // Round up to nearest byte
    writer.write_all(&(total_bytes as u32).to_le_bytes())?;

    writer.write_all(bitvec.as_raw_slice())?;

    Ok(())
}

pub fn write<W: Write>(
    writer: &mut W,
    input: &str,
    code_map: &HashMap<char, String>,
) -> Result<(), Box<dyn Error>> {
    write_header(writer, code_map)?;
    write_data(writer, code_map, input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_test_map() -> HashMap<char, String> {
        let mut map = HashMap::new();
        map.insert('a', "0".to_string());
        map.insert('b', "10".to_string());
        map.insert('c', "110".to_string());
        map.insert('d', "111".to_string());
        map
    }

    #[test]
    fn test_write_header() {
        let mut buffer = Cursor::new(Vec::new());
        let map = create_test_map();
        write_header(&mut buffer, &map).unwrap();

        let written = buffer.into_inner();
        assert_eq!(&written[0..4], b"CCHF");
        assert_eq!(written[4], 1); // version
        assert_eq!(u16::from_le_bytes([written[5], written[6]]), 4); // map length
                                                                     // Further checks can be added for the map entries
    }

    #[test]
    fn test_write_data() {
        let mut buffer = Cursor::new(Vec::new());
        let map = create_test_map();
        let input = "abcd";
        write_data(&mut buffer, &map, input).unwrap();

        let written = buffer.into_inner();
        assert_eq!(written.len(), 6); // 4 bytes for length + 1 byte of data
        assert_eq!(
            u32::from_le_bytes([written[0], written[1], written[2], written[3]]),
            2
        ); // total bytes
        assert_eq!(written[4], 0b01011011); // encoded data
    }

    #[test]
    fn test_write_complete() {
        let mut buffer = Cursor::new(Vec::new());
        let map = create_test_map();
        let input = "abcd";
        write(&mut buffer, input, &map).unwrap();

        let written = buffer.into_inner();
        assert!(written.len() > 9); // Header + data length + data
        assert_eq!(&written[0..4], b"CCHF");
        // Further checks can be added
    }

    #[test]
    fn test_write_empty_input() {
        let mut buffer = Cursor::new(Vec::new());
        let map = HashMap::new();
        let input = "";
        write(&mut buffer, input, &map).unwrap();

        let written = buffer.into_inner();
        assert_eq!(written.len(), 11); // CCHF + version + map length (0) + data length (0)
    }

    #[test]
    fn test_write_single_char() {
        let mut buffer = Cursor::new(Vec::new());
        let mut map = HashMap::new();
        map.insert('a', "0".to_string());
        let input = "a";
        write(&mut buffer, input, &map).unwrap();

        let written = buffer.into_inner();
        assert!(written.len() > 11);
        assert_eq!(written[written.len() - 1], 0b10000000); // '0' padded to byte
    }

    #[test]
    fn test_write_with_newline() {
        let mut buffer = Cursor::new(Vec::new());
        let mut map = create_test_map();
        map.insert('\n', "1111".to_string());
        let input = "a\nb";
        write(&mut buffer, input, &map).unwrap();

        let written = buffer.into_inner();
        assert!(written.len() > 11);
        // Check that the newline is encoded correctly
    }

    #[test]
    fn test_write_long_input() {
        let mut buffer = Cursor::new(Vec::new());
        let map = create_test_map();
        let input = "abcdabcdabcd"; // 27 bits, should be 4 bytes
        write(&mut buffer, input, &map).unwrap();

        let written = buffer.into_inner();
        assert!(written.len() > 15); // Header + data length + 4 bytes of data
                                     // Check the data length is 4

        assert_eq!(
            u32::from_le_bytes([written[11], written[12], written[13], written[14]]),
            4
        );

        // Optionally, check the actual encoded data
        assert_eq!(written[15], 0b01011011);
        assert_eq!(written[16], 0b10101101);
        assert_eq!(written[17], 0b11010110);
        assert_eq!(written[18], 0b11100000); // Last byte with padding
    }
}
