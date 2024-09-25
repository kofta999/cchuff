use bitvec::prelude::*;

use crate::shared::{generate_queue, generate_tree, HuffNode};
use std::collections::BTreeMap;

pub fn build(input: &str) -> (BTreeMap<char, String>, BitVec<u8, Msb0>) {
    let freq_table = calculate_frequency(input);
    let tree = generate_tree(generate_queue(&freq_table));
    let p = tree.clone();
    let (codemap, bitvec) = generate_codemap_and_bittree(p);

    (codemap, bitvec)
}

fn calculate_frequency(input: &str) -> BTreeMap<char, u32> {
    let mut map = BTreeMap::new();

    for char in input.chars() {
        map.entry(char).and_modify(|freq| *freq += 1).or_insert(1);
    }

    map
}

fn generate_codemap_and_bittree(head: HuffNode) -> (BTreeMap<char, String>, BitVec<u8, Msb0>) {
    let mut map: BTreeMap<char, String> = BTreeMap::new();
    let mut bitvec = BitVec::<u8, Msb0>::new();
    
    dbg!(&head);

    dfs(Some(Box::new(head)), "", &mut map, &mut bitvec);
    

    (map, bitvec)
}

fn dfs(
    p: Option<Box<HuffNode>>,
    s: &str,
    map: &mut BTreeMap<char, String>,
    vec: &mut BitVec<u8, Msb0>,
) {
    match p {
        None => (),
        Some(p) => {
            if let Some(v) = p.value {
                map.insert(v, s.to_string());
                vec.push(true);
                let char_code = v as u32;

                // Push each bit of the char_code into the BitVec
                for i in (0..32).rev() {
                    vec.push(((char_code >> i) & 1) == 1);
                }
            } else {
                vec.push(false);
            }

            dfs(p.left, &format!("{}0", s), map, vec);
            dfs(p.right, &format!("{}1", s), map, vec);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_calculate_frequency() {
//         let input = "hello world";
//         let freq = calculate_frequency(input);
//         assert_eq!(freq.get(&'h'), Some(&1));
//         assert_eq!(freq.get(&'e'), Some(&1));
//         assert_eq!(freq.get(&'l'), Some(&3));
//         assert_eq!(freq.get(&'o'), Some(&2));
//         assert_eq!(freq.get(&' '), Some(&1));
//         assert_eq!(freq.get(&'w'), Some(&1));
//         assert_eq!(freq.get(&'r'), Some(&1));
//         assert_eq!(freq.get(&'d'), Some(&1));
//         assert_eq!(freq.len(), 8);
//     }

//     #[test]
//     fn test_generate_queue() {
//         let mut freq = BTreeMap::new();
//         freq.insert('a', 3);
//         freq.insert('b', 2);
//         freq.insert('c', 1);
//         let queue = generate_queue(&freq);
//         assert_eq!(queue.len(), 3);
//         assert_eq!(queue[0].freq, 1);
//         assert_eq!(queue[0].value, Some('c'));
//         assert_eq!(queue[2].freq, 3);
//         assert_eq!(queue[2].value, Some('a'));
//     }

//     #[test]
//     fn test_generate_tree() {
//         let mut freq = BTreeMap::new();
//         freq.insert('a', 3);
//         freq.insert('b', 2);
//         freq.insert('c', 1);
//         let queue = generate_queue(&freq);
//         let tree = generate_tree(queue);
//         assert_eq!(tree.freq, 6);
//         assert_eq!(tree.value, None);
//     }

//     #[test]
//     fn test_generate_coding_map() {
//         let input = "aabbc";
//         let freq = calculate_frequency(input);
//         let queue = generate_queue(&freq);
//         let tree = generate_tree(queue);
//         let map = generate_coding_map(tree);
//         assert_eq!(map.len(), 3);
//         assert!(map.get(&'a').unwrap().len() <= 2);
//         assert!(map.get(&'b').unwrap().len() <= 2);
//         assert!(map.get(&'c').unwrap().len() <= 2);
//     }

//     #[test]
//     fn test_build() {
//         let input = "hello world";
//         let map = build(input);
//         assert_eq!(map.len(), 8);
//         for (_, code) in map.iter() {
//             assert!(code.chars().all(|c| c == '0' || c == '1'));
//         }
//     }

//     // #[test]
//     // fn test_empty_input() {
//     //     let input = "";
//     //     let map = build(input);
//     //     assert_eq!(map.len(), 0);
//     // }

//     // #[test]
//     // fn test_single_character() {
//     //     let input = "aaaa";
//     //     let map = build(input);
//     //     assert_eq!(map.len(), 1);
//     //     assert_eq!(map.get(&'a'), Some(&"0".to_string()));
//     // }

//     #[test]
//     fn test_all_unique_characters() {
//         let input = "abcdefg";
//         let map = build(input);
//         assert_eq!(map.len(), 7);
//     }

//     #[test]
//     fn test_with_newline() {
//         let input = "hello\nworld";
//         let map = build(input);
//         assert!(map.contains_key(&'\n'));
//     }

//     #[test]
//     fn test_code_uniqueness() {
//         let input = "abcdefghijklmnopqrstuvwxyz";
//         let map = build(input);
//         let mut codes: Vec<&String> = map.values().collect();
//         codes.sort();
//         codes.dedup();
//         assert_eq!(codes.len(), map.len());
//     }

//     #[test]
//     fn test_code_prefix() {
//         let input = "aaabbc";
//         let map = build(input);
//         for (char1, code1) in map.iter() {
//             for (char2, code2) in map.iter() {
//                 if char1 != char2 {
//                     assert!(!code1.starts_with(code2));
//                     assert!(!code2.starts_with(code1));
//                 }
//             }
//         }
//     }
// }
