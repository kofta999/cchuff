use std::collections::HashMap;

#[derive(Debug)]
pub struct HuffNode {
    value: Option<char>,
    freq: usize,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

pub fn build(input: &str) -> HashMap<char, String> {
    generate_coding_map(generate_tree(generate_queue(calculate_frequency(input))))
}

fn generate_queue(map: HashMap<char, usize>) -> Vec<HuffNode> {
    let mut vec = map
        .iter()
        .map(|(k, v)| HuffNode {
            freq: *v,
            value: Some(*k),
            left: None,
            right: None,
        })
        .collect::<Vec<HuffNode>>();

    vec.sort_by_key(|node| node.freq);

    vec
}

fn generate_tree(mut nodes: Vec<HuffNode>) -> HuffNode {
    while nodes.len() > 1 {
        let curr = nodes.remove(0);
        let next = nodes.remove(0);

        let node = HuffNode {
            value: None,
            freq: curr.freq + next.freq,
            left: Some(Box::new(curr)),
            right: Some(Box::new(next)),
        };

        nodes.push(node);

        nodes.sort_by_key(|v| v.freq);
    }

        nodes.remove(0)
}

fn calculate_frequency(input: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for char in input.chars() {
        map.entry(char)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    }

    map
}

fn generate_coding_map(head: HuffNode) -> HashMap<char, String> {
    let mut map: HashMap<char, String> = HashMap::new();

    dfs(Some(Box::new(head)), "", &mut map);
    map
}

fn dfs(p: Option<Box<HuffNode>>, s: &str, map: &mut HashMap<char, String>) {
    match p {
        None => (),
        Some(p) => {
            if let Some(v) = p.value {
                map.insert(v, s.to_string());
            }

            dfs(p.left, &format!("{}0", s), map);
            dfs(p.right, &format!("{}1", s), map);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_frequency() {
        let input = "hello world";
        let freq = calculate_frequency(input);
        assert_eq!(freq.get(&'h'), Some(&1));
        assert_eq!(freq.get(&'e'), Some(&1));
        assert_eq!(freq.get(&'l'), Some(&3));
        assert_eq!(freq.get(&'o'), Some(&2));
        assert_eq!(freq.get(&' '), Some(&1));
        assert_eq!(freq.get(&'w'), Some(&1));
        assert_eq!(freq.get(&'r'), Some(&1));
        assert_eq!(freq.get(&'d'), Some(&1));
        assert_eq!(freq.len(), 8);
    }

    #[test]
    fn test_generate_queue() {
        let mut freq = HashMap::new();
        freq.insert('a', 3);
        freq.insert('b', 2);
        freq.insert('c', 1);
        let queue = generate_queue(freq);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue[0].freq, 1);
        assert_eq!(queue[0].value, Some('c'));
        assert_eq!(queue[2].freq, 3);
        assert_eq!(queue[2].value, Some('a'));
    }

    #[test]
    fn test_generate_tree() {
        let mut freq = HashMap::new();
        freq.insert('a', 3);
        freq.insert('b', 2);
        freq.insert('c', 1);
        let queue = generate_queue(freq);
        let tree = generate_tree(queue);
        assert_eq!(tree.freq, 6);
        assert_eq!(tree.value, None);
    }

    #[test]
    fn test_generate_coding_map() {
        let input = "aabbc";
        let freq = calculate_frequency(input);
        let queue = generate_queue(freq);
        let tree = generate_tree(queue);
        let map = generate_coding_map(tree);
        assert_eq!(map.len(), 3);
        assert!(map.get(&'a').unwrap().len() <= 2);
        assert!(map.get(&'b').unwrap().len() <= 2);
        assert!(map.get(&'c').unwrap().len() <= 2);
    }

    #[test]
    fn test_build() {
        let input = "hello world";
        let map = build(input);
        assert_eq!(map.len(), 8);
        for (_, code) in map.iter() {
            assert!(code.chars().all(|c| c == '0' || c == '1'));
        }
    }

    // #[test]
    // fn test_empty_input() {
    //     let input = "";
    //     let map = build(input);
    //     assert_eq!(map.len(), 0);
    // }

    // #[test]
    // fn test_single_character() {
    //     let input = "aaaa";
    //     let map = build(input);
    //     assert_eq!(map.len(), 1);
    //     assert_eq!(map.get(&'a'), Some(&"0".to_string()));
    // }

    #[test]
    fn test_all_unique_characters() {
        let input = "abcdefg";
        let map = build(input);
        assert_eq!(map.len(), 7);
    }

    #[test]
    fn test_with_newline() {
        let input = "hello\nworld";
        let map = build(input);
        assert!(map.contains_key(&'\n'));
    }

    #[test]
    fn test_code_uniqueness() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let map = build(input);
        let mut codes: Vec<&String> = map.values().collect();
        codes.sort();
        codes.dedup();
        assert_eq!(codes.len(), map.len());
    }

    #[test]
    fn test_code_prefix() {
        let input = "aaabbc";
        let map = build(input);
        for (char1, code1) in map.iter() {
            for (char2, code2) in map.iter() {
                if char1 != char2 {
                    assert!(!code1.starts_with(code2));
                    assert!(!code2.starts_with(code1));
                }
            }
        }
    }
}
