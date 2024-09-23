#[derive(Debug)]
pub struct HuffNode {
    value: Option<char>,
    freq: usize,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}
use std::collections::HashMap;

pub fn build(input: &str) -> HuffNode {
    generate_tree(generate_queue(calculate_frequency(input)))
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

pub fn generate_coding_map(head: HuffNode) -> HashMap<char, String> {
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
    fn test_calc_freq() {
        let string = "hello world";
        let mut map = HashMap::new();

        map.insert('h', 1);
        map.insert('e', 1);
        map.insert('l', 3);
        map.insert('o', 2);
        map.insert('w', 1);
        map.insert('d', 1);
        map.insert('r', 1);
        map.insert(' ', 1);

        assert_eq!(calculate_frequency(string), map)
    }

    #[test]
    fn test_code_map() {
        let mut vec = vec![
            HuffNode {
                freq: 32,
                value: Some('C'),
                left: None,
                right: None,
            },
            HuffNode {
                freq: 42,
                value: Some('D'),
                left: None,
                right: None,
            },
            HuffNode {
                freq: 120,
                value: Some('E'),
                left: None,
                right: None,
            },
            HuffNode {
                freq: 7,
                value: Some('K'),
                left: None,
                right: None,
            },
            HuffNode {
                freq: 42,
                value: Some('L'),
                left: None,
                right: None,
            },
            HuffNode {
                freq: 24,
                value: Some('M'),
                left: None,
                right: None,
            },
            HuffNode {
                freq: 37,
                value: Some('U'),
                left: None,
                right: None,
            },
            HuffNode {
                freq: 2,
                value: Some('Z'),
                left: None,
                right: None,
            },
        ];
        vec.sort_by_key(|node| node.freq);

        let res = generate_tree(vec);
        let map = generate_coding_map(res);

        panic!()
    }
}
